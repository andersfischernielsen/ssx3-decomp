//! Staged lobby MD5 compression port.

use crate::abi::{r32, set_word};
use crate::call::{sp, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003f_2220, crypt_md5_transform, "CryptMD5_Transform")];

const MD5_STATE_A: u32 = 0x04;
const MD5_STATE_B: u32 = 0x08;
const MD5_STATE_C: u32 = 0x0c;
const MD5_STATE_D: u32 = 0x10;
const MD5_BLOCK: u32 = 0x14;
const MD5_BLOCK_BYTES: u32 = 64;
const MD5_WORD_BYTES: u32 = size_of::<u32>() as u32;
const MD5_WORDS: u32 = MD5_BLOCK_BYTES / MD5_WORD_BYTES;
const MD5_DECODE_HEAD: u32 = 0x003f_2258;

// The first 64 bytes of the frame hold the decoded block. The next 19
// words hold constants and one pre-addition that the EE compiler spilled.
const MD5_K30_SLOT: u32 = 0x40;
const MD5_SUM39_SLOT: u32 = 0x44;
const MD5_K39_SLOT: u32 = 0x48;
const MD5_K41_SLOT: u32 = 0x4c;
const MD5_K42_SLOT: u32 = 0x50;
const MD5_K44_SLOT: u32 = 0x54;
const MD5_K46_SLOT: u32 = 0x58;
const MD5_K47_SLOT: u32 = 0x5c;
const MD5_K49_SLOT: u32 = 0x60;
const MD5_K50_SLOT: u32 = 0x64;
const MD5_K52_SLOT: u32 = 0x68;
const MD5_K53_SLOT: u32 = 0x6c;
const MD5_K55_SLOT: u32 = 0x70;
const MD5_K56_SLOT: u32 = 0x74;
const MD5_K58_SLOT: u32 = 0x78;
const MD5_K59_SLOT: u32 = 0x7c;
const MD5_K61_SLOT: u32 = 0x80;
const MD5_K62_SLOT: u32 = 0x84;
const MD5_K64_SLOT: u32 = 0x88;
const MD5_SAVE_FP: u32 = 0x90;
const MD5_SAVE_S7: u32 = 0xa0;
const MD5_SAVE_S6: u32 = 0xb0;
const MD5_SAVE_S5: u32 = 0xc0;
const MD5_SAVE_S4: u32 = 0xd0;
const MD5_SAVE_S3: u32 = 0xe0;
const MD5_SAVE_S2: u32 = 0xf0;
const MD5_SAVE_S1: u32 = 0x100;
const MD5_SAVE_S0: u32 = 0x110;
const MD5_FRAME_BYTES: u32 = 0x120;

#[rustfmt::skip]
const MD5_CONSTANTS: [u32; 64] = [
    0xd76a_a478, 0xe8c7_b756, 0x2420_70db, 0xc1bd_ceee,
    0xf57c_0faf, 0x4787_c62a, 0xa830_4613, 0xfd46_9501,
    0x6980_98d8, 0x8b44_f7af, 0xffff_5bb1, 0x895c_d7be,
    0x6b90_1122, 0xfd98_7193, 0xa679_438e, 0x49b4_0821,
    0xf61e_2562, 0xc040_b340, 0x265e_5a51, 0xe9b6_c7aa,
    0xd62f_105d, 0x0244_1453, 0xd8a1_e681, 0xe7d3_fbc8,
    0x21e1_cde6, 0xc337_07d6, 0xf4d5_0d87, 0x455a_14ed,
    0xa9e3_e905, 0xfcef_a3f8, 0x676f_02d9, 0x8d2a_4c8a,
    0xfffa_3942, 0x8771_f681, 0x6d9d_6122, 0xfde5_380c,
    0xa4be_ea44, 0x4bde_cfa9, 0xf6bb_4b60, 0xbebf_bc70,
    0x289b_7ec6, 0xeaa1_27fa, 0xd4ef_3085, 0x0488_1d05,
    0xd9d4_d039, 0xe6db_99e5, 0x1fa2_7cf8, 0xc4ac_5665,
    0xf429_2244, 0x432a_ff97, 0xab94_23a7, 0xfc93_a039,
    0x655b_59c3, 0x8f0c_cc92, 0xffef_f47d, 0x8584_5dd1,
    0x6fa8_7e4f, 0xfe2c_e6e0, 0xa301_4314, 0x4e08_11a1,
    0xf753_7e82, 0xbd3a_f235, 0x2ad7_d2bb, 0xeb86_d391,
];

const MD5_ROTATIONS: [[u32; 4]; 4] = [
    [7, 12, 17, 22],
    [5, 9, 14, 20],
    [4, 11, 16, 23],
    [6, 10, 15, 21],
];

struct Md5State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Md5State {
    fn step(&mut self, step: usize, word: u32) {
        self.step_with_sum(step, word, self.a.wrapping_add(MD5_CONSTANTS[step - 1]));
    }

    // Each round cycles the roles of the same four digest words. Some EE
    // steps add their constant early and pass that sum through a spill slot.
    fn step_with_sum(&mut self, step: usize, word: u32, a_plus_constant: u32) {
        let round = (step - 1) / MD5_WORDS as usize;
        let mixed = match round {
            0 => self.d ^ ((self.c ^ self.d) & self.b),
            1 => self.c ^ ((self.b ^ self.c) & self.d),
            2 => self.b ^ self.c ^ self.d,
            _ => self.last_round_mix(),
        };
        let rotation = MD5_ROTATIONS[round][(step - 1) % 4];
        self.add_rotated(
            a_plus_constant.wrapping_add(mixed.wrapping_add(word)),
            rotation,
        );
    }

    fn last_round_mix(&self) -> u32 {
        self.c ^ (self.b | !self.d)
    }

    fn add_rotated(&mut self, sum: u32, rotation: u32) {
        let next = self.b.wrapping_add(sum.rotate_left(rotation));
        (self.a, self.b, self.c, self.d) = (self.d, next, self.b, self.c);
    }
}

fn md5_block_word(c: &mut Cpu, stack: Addr, index: u32) -> u32 {
    stack.off(index * MD5_WORD_BYTES).lw(c) as u32
}

// These splits preserve the compiler's two stores and intervening reload
// when it assembles a round constant out of its two halfwords.
fn md5_finish_spill(c: &mut Cpu, slot: Addr, constant: u32) {
    let high_half = slot.lw(c) as u32;
    slot.sw(c, u64::from(high_half | (constant & u16::MAX as u32)));
}

fn md5_spill_constant(c: &mut Cpu, stack: Addr, offset: u32, step: usize) {
    let slot = stack.off(offset);
    let constant = MD5_CONSTANTS[step - 1];
    slot.sw(c, u64::from(constant & !(u16::MAX as u32)));
    md5_finish_spill(c, slot, constant);
}

fn md5_word_during_spill(c: &mut Cpu, stack: Addr, index: u32, offset: u32, step: usize) -> u32 {
    let slot = stack.off(offset);
    let constant = MD5_CONSTANTS[step - 1];
    slot.sw(c, u64::from(constant & !(u16::MAX as u32)));
    let word = md5_block_word(c, stack, index);
    md5_finish_spill(c, slot, constant);
    word
}

/// Compress one buffered 64-byte block into the lobby's MD5 digest state.
pub fn crypt_md5_transform(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, MD5_FRAME_BYTES);
    frame.save(c, S0, MD5_SAVE_S0);
    let decode_end = frame.at(c, MD5_BLOCK_BYTES);
    set_word(c, A1, decode_end);
    frame.save(c, S7, MD5_SAVE_S7);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, MD5_SAVE_S1);
    set_word(c, S7, MD5_WORDS);
    frame.save(c, S2, MD5_SAVE_S2);
    let block_end = r32(c, S0).wrapping_add(MD5_BLOCK + MD5_BLOCK_BYTES);
    set_word(c, T2, block_end);
    frame.save(c, S3, MD5_SAVE_S3);
    frame.save(c, S4, MD5_SAVE_S4);
    frame.save(c, S5, MD5_SAVE_S5);
    frame.save(c, S6, MD5_SAVE_S6);
    frame.save(c, FP, MD5_SAVE_FP);

    // Decode from high addresses to low. The back edge exposes the pointers,
    // counter and partial-byte registers to the guest event handler.
    loop {
        c.cycles += 18;
        let source = Addr(r32(c, T2).wrapping_sub(MD5_WORD_BYTES));
        let destination = Addr(r32(c, A1).wrapping_sub(MD5_WORD_BYTES));
        let byte3 = source.off(3).lbu(c) as u8;
        let byte2 = source.off(2).lbu(c) as u8;
        let byte1 = source.off(1).lbu(c) as u8;
        let byte0 = source.lbu(c) as u8;
        let word = u32::from_le_bytes([byte0, byte1, byte2, byte3]);
        let remaining = r32(c, S7).wrapping_sub(1);
        set_word(c, T2, source.0);
        set_word(c, A1, destination.0);
        set_word(c, S7, remaining);
        set_word(c, A0, u32::from(byte1));
        set_word(c, V1, u32::from(byte0));
        set_word(c, V0, word & !(u8::MAX as u32));
        set_word(c, FP, word);
        destination.sw(c, u64::from(word));
        if remaining == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, MD5_DECODE_HEAD);
        }
    }

    c.cycles += 915;
    let stack = Addr(sp(c));
    let context = Addr(r32(c, S0));
    let digest_c = context.off(MD5_STATE_C).lw(c) as u32;
    let digest_d = context.off(MD5_STATE_D).lw(c) as u32;
    let digest_b = context.off(MD5_STATE_B).lw(c) as u32;
    let digest_a = context.off(MD5_STATE_A).lw(c) as u32;
    let mut state = Md5State {
        a: digest_a,
        b: digest_b,
        c: digest_c,
        d: digest_d,
    };

    for index in 0..MD5_WORDS {
        state.step(index as usize + 1, md5_block_word(c, stack, index));
    }

    // Round 2 constructs the stack constants used by later rounds.
    state.step(17, md5_block_word(c, stack, 1));
    let word18 = md5_block_word(c, stack, 6);
    md5_spill_constant(c, stack, MD5_K30_SLOT, 30);
    state.step(18, word18);
    state.step(19, md5_block_word(c, stack, 11));
    state.step(20, md5_block_word(c, stack, 0));
    let word21 = md5_block_word(c, stack, 5);
    md5_spill_constant(c, stack, MD5_K39_SLOT, 39);
    state.step(21, word21);
    let word22 = md5_word_during_spill(c, stack, 10, MD5_K41_SLOT, 41);
    stack
        .off(MD5_K42_SLOT)
        .sw(c, u64::from(MD5_CONSTANTS[42 - 1]));
    state.step(22, word22);
    state.step(23, md5_word_during_spill(c, stack, 15, MD5_K44_SLOT, 44));
    md5_spill_constant(c, stack, MD5_K46_SLOT, 46);
    state.step(24, md5_word_during_spill(c, stack, 4, MD5_K47_SLOT, 47));
    md5_spill_constant(c, stack, MD5_K49_SLOT, 49);
    state.step(25, md5_word_during_spill(c, stack, 9, MD5_K50_SLOT, 50));
    md5_spill_constant(c, stack, MD5_K52_SLOT, 52);
    let word26 = md5_word_during_spill(c, stack, 14, MD5_K53_SLOT, 53);
    let k30 = stack.off(MD5_K30_SLOT).lw(c) as u32;
    state.step(26, word26);
    md5_spill_constant(c, stack, MD5_K55_SLOT, 55);
    state.step(27, md5_word_during_spill(c, stack, 3, MD5_K56_SLOT, 56));
    md5_spill_constant(c, stack, MD5_K58_SLOT, 58);
    state.step(28, md5_word_during_spill(c, stack, 8, MD5_K59_SLOT, 59));
    md5_spill_constant(c, stack, MD5_K61_SLOT, 61);
    state.step(29, md5_word_during_spill(c, stack, 13, MD5_K62_SLOT, 62));
    md5_spill_constant(c, stack, MD5_K64_SLOT, 64);
    let word30 = md5_block_word(c, stack, 2);
    state.step_with_sum(30, word30, state.a.wrapping_add(k30));
    state.step(31, md5_block_word(c, stack, 7));
    state.step(32, md5_block_word(c, stack, 12));

    // Round 3 reloads several constants a few steps before consuming them.
    state.step(33, md5_block_word(c, stack, 5));
    state.step(34, md5_block_word(c, stack, 8));
    let word35 = md5_block_word(c, stack, 11);
    let k39 = stack.off(MD5_K39_SLOT).lw(c) as u32;
    state.step(35, word35);
    stack
        .off(MD5_SUM39_SLOT)
        .sw(c, u64::from(state.b.wrapping_add(k39)));
    state.step(36, md5_block_word(c, stack, 14));
    let word37 = md5_block_word(c, stack, 1);
    let k41 = stack.off(MD5_K41_SLOT).lw(c) as u32;
    state.step(37, word37);
    let sum41 = state.b.wrapping_add(k41);
    let word38 = md5_block_word(c, stack, 4);
    let k42 = stack.off(MD5_K42_SLOT).lw(c) as u32;
    state.step(38, word38);
    let sum42 = state.b.wrapping_add(k42);
    let word39 = md5_block_word(c, stack, 7);
    let sum39 = stack.off(MD5_SUM39_SLOT).lw(c) as u32;
    state.step_with_sum(39, word39, sum39);
    let word40 = md5_block_word(c, stack, 10);
    let k44 = stack.off(MD5_K44_SLOT).lw(c) as u32;
    state.step(40, word40);
    let sum44 = state.b.wrapping_add(k44);
    state.step_with_sum(41, md5_block_word(c, stack, 13), sum41);
    let word42 = md5_block_word(c, stack, 0);
    let k46 = stack.off(MD5_K46_SLOT).lw(c) as u32;
    state.step_with_sum(42, word42, sum42);
    let sum46 = state.b.wrapping_add(k46);
    let word43 = md5_block_word(c, stack, 3);
    let k47 = stack.off(MD5_K47_SLOT).lw(c) as u32;
    state.step(43, word43);
    let sum47 = state.b.wrapping_add(k47);
    state.step_with_sum(44, md5_block_word(c, stack, 6), sum44);
    let sum48 = state.b.wrapping_add(MD5_CONSTANTS[48 - 1]);
    let word45 = md5_block_word(c, stack, 9);
    let k49 = stack.off(MD5_K49_SLOT).lw(c) as u32;
    state.step(45, word45);
    let sum49 = state.b.wrapping_add(k49);
    let word46 = md5_block_word(c, stack, 12);
    let k50 = stack.off(MD5_K50_SLOT).lw(c) as u32;
    state.step_with_sum(46, word46, sum46);
    let sum50 = state.b.wrapping_add(k50);
    let word47 = md5_block_word(c, stack, 15);
    let word48 = md5_block_word(c, stack, 2);
    state.step_with_sum(47, word47, sum47);
    let k52 = stack.off(MD5_K52_SLOT).lw(c) as u32;
    state.step_with_sum(48, word48, sum48);
    let sum52 = state.b.wrapping_add(k52);

    // Round 4 keeps the compiler's late pre-additions for the EE registers
    // left live at return.
    let word49 = md5_block_word(c, stack, 0);
    let k53 = stack.off(MD5_K53_SLOT).lw(c) as u32;
    state.step_with_sum(49, word49, sum49);
    let sum53 = state.b.wrapping_add(k53);
    state.step_with_sum(50, md5_block_word(c, stack, 7), sum50);
    let sum54 = state.b.wrapping_add(MD5_CONSTANTS[54 - 1]);
    let word51 = md5_block_word(c, stack, 14);
    let k55 = stack.off(MD5_K55_SLOT).lw(c) as u32;
    state.step(51, word51);
    let sum55 = state.b.wrapping_add(k55);
    let word52 = md5_block_word(c, stack, 5);
    let k56 = stack.off(MD5_K56_SLOT).lw(c) as u32;
    state.step_with_sum(52, word52, sum52);
    let sum56 = state.b.wrapping_add(k56);
    state.step_with_sum(53, md5_block_word(c, stack, 12), sum53);
    let sum57 = state.b.wrapping_add(MD5_CONSTANTS[57 - 1]);
    let word54 = md5_block_word(c, stack, 3);
    let k58 = stack.off(MD5_K58_SLOT).lw(c) as u32;
    state.step_with_sum(54, word54, sum54);
    let sum58 = state.b.wrapping_add(k58);
    let word55 = md5_block_word(c, stack, 10);
    let k59 = stack.off(MD5_K59_SLOT).lw(c) as u32;
    state.step_with_sum(55, word55, sum55);
    let sum59 = state.b.wrapping_add(k59);
    state.step_with_sum(56, md5_block_word(c, stack, 1), sum56);
    let sum60 = state.b.wrapping_add(MD5_CONSTANTS[60 - 1]);
    let word57 = md5_block_word(c, stack, 8);
    let k61 = stack.off(MD5_K61_SLOT).lw(c) as u32;
    state.step_with_sum(57, word57, sum57);
    let sum61 = state.b.wrapping_add(k61);
    let word58 = md5_block_word(c, stack, 15);
    let k62 = stack.off(MD5_K62_SLOT).lw(c) as u32;
    state.step_with_sum(58, word58, sum58);
    let sum62 = state.b.wrapping_add(k62);
    let word59 = md5_block_word(c, stack, 6);
    let word60 = md5_block_word(c, stack, 13);
    state.step_with_sum(59, word59, sum59);
    let sum63 = state.b.wrapping_add(MD5_CONSTANTS[63 - 1]);
    state.step_with_sum(60, word60, sum60);

    // The final four additions read and update each digest word at its own
    // step. A context address can overlap the frame, including saved GPRs.
    let word61 = md5_block_word(c, stack, 4);
    let k64 = stack.off(MD5_K64_SLOT).lw(c) as u32;
    let sum64 = state.b.wrapping_add(k64);
    let digest_a = context.off(MD5_STATE_A).lw(c) as u32;
    state.step_with_sum(61, word61, sum61);
    let word62 = md5_block_word(c, stack, 11);
    let mix62_and_word = state.last_round_mix().wrapping_add(word62);
    context
        .off(MD5_STATE_A)
        .sw(c, u64::from(digest_a.wrapping_add(state.b)));
    frame.restore(c, S1, MD5_SAVE_S1);
    frame.restore(c, S2, MD5_SAVE_S2);
    frame.restore(c, S3, MD5_SAVE_S3);

    let digest_d = context.off(MD5_STATE_D).lw(c) as u32;
    state.add_rotated(sum62.wrapping_add(mix62_and_word), MD5_ROTATIONS[3][1]);
    let round_d = state.b;
    frame.restore(c, S4, MD5_SAVE_S4);
    context
        .off(MD5_STATE_D)
        .sw(c, u64::from(digest_d.wrapping_add(round_d)));
    frame.restore(c, S5, MD5_SAVE_S5);

    let word63 = md5_block_word(c, stack, 2);
    frame.restore(c, S6, MD5_SAVE_S6);
    frame.restore(c, S7, MD5_SAVE_S7);
    let digest_c = context.off(MD5_STATE_C).lw(c) as u32;
    state.step_with_sum(63, word63, sum63);
    let round_c = state.b;
    context
        .off(MD5_STATE_C)
        .sw(c, u64::from(digest_c.wrapping_add(round_c)));

    let word64 = md5_block_word(c, stack, 9);
    let mix64_and_word = state.last_round_mix().wrapping_add(word64);
    let digest_b = context.off(MD5_STATE_B).lw(c) as u32;
    state.add_rotated(sum64.wrapping_add(mix64_and_word), MD5_ROTATIONS[3][3]);
    let updated_b = digest_b.wrapping_add(state.b);
    context.off(MD5_STATE_B).sw(c, u64::from(updated_b));
    frame.restore(c, S0, MD5_SAVE_S0);
    frame.restore(c, FP, MD5_SAVE_FP);

    set_word(c, V0, updated_b);
    set_word(c, V1, digest_b);
    set_word(c, A0, mix64_and_word);
    set_word(c, A1, digest_c);
    set_word(c, A2, mix62_and_word);
    set_word(c, A3, sum64);
    set_word(c, T0, sum61);
    set_word(c, T1, sum60);
    set_word(c, T2, sum62);
    set_word(c, T3, sum53);
    set_word(c, T4, sum48);
    set_word(c, T5, sum54);
    set_word(c, T6, sum57);
    set_word(c, T7, sum63);
    set_word(c, T8, round_c);
    set_word(c, T9, round_d);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::{check_paths, fill, fresh, put_words, scramble, Rng, DATA};

    const MD5_TRANSFORM_ENTRY: u32 = 0x003f_2220;

    fn md5_transform_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut rng = Rng(0x6d64_3500 ^ u64::from(seed));
        scramble(c, &mut rng);
        let frame_start = crate::difftest::SP - MD5_FRAME_BYTES;
        fill(c, &mut rng, frame_start, MD5_FRAME_BYTES / MD5_WORD_BYTES);
        let context = match seed % 5 {
            0 => DATA,
            // Feed-forward writes alias words that a later step still reads.
            1 => frame_start + 2 * MD5_WORD_BYTES - MD5_STATE_A,
            2 => frame_start + 9 * MD5_WORD_BYTES - MD5_STATE_C,
            // The context aliases a register save or the constant spill area.
            3 => frame_start + MD5_SAVE_S3 - MD5_STATE_A,
            _ => frame_start + MD5_K44_SLOT - MD5_STATE_B,
        };
        fill(
            c,
            &mut rng,
            context,
            (MD5_BLOCK + MD5_BLOCK_BYTES) / MD5_WORD_BYTES,
        );
        set_word(c, A0, context);
        c.pc = MD5_TRANSFORM_ENTRY;
        rng
    }

    #[test]
    fn crypt_md5_transform_matches() {
        check_paths(
            ssx3_ee::s14::f_003f2220__generated,
            crypt_md5_transform,
            80,
            &[1217],
            &|c, seed| {
                md5_transform_fixture(c, seed);
            },
        );
    }

    fn md5_transform_with_event_check(c: &mut Cpu) {
        let due = c.next_event;
        crypt_md5_transform(c);
        if due <= c.cycles {
            assert_ne!(c.next_event, due, "the block copy should deliver the event");
        }
    }

    #[test]
    fn crypt_md5_transform_events_matches() {
        check_events(
            ssx3_ee::s14::f_003f2220__generated,
            md5_transform_with_event_check,
            45,
            &|c, seed| {
                let mut rng = md5_transform_fixture(c, seed / 3);
                let mode = schedule(c, &mut rng, seed, MD5_TRANSFORM_ENTRY);
                if mode != Events::Off {
                    // The first event falls inside one of the 15 back-edge intervals.
                    let word = 1 + u64::from(seed / 3);
                    c.next_event = c.cycles + 14 + word * 18 - 1;
                }
                mode
            },
        );
    }

    #[test]
    fn crypt_md5_transform_known_answers() {
        const INITIAL_DIGEST: [u32; 4] = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];
        let vectors: [(&[u8], [u32; 4]); 3] = [
            (b"", [0xd98c_1dd4, 0x04b2_008f, 0x9809_80e9, 0x7e42_f8ec]),
            (b"abc", [0x9850_0190, 0xb04f_d23c, 0x7d3f_96d6, 0x727f_e128]),
            (
                b"12345678901234567890123456789012345678901234567890123456789012345678901234567890",
                [0xa2f4_ed57, 0x55c9_e32b, 0x2eda_49ac, 0x7ab6_0721],
            ),
        ];
        for (message, expected) in vectors {
            let mut c = fresh();
            put_words(&mut c, DATA + MD5_STATE_A, &INITIAL_DIGEST);

            // Pad with a one bit, then zeros, then the original bit length.
            let block_bytes = MD5_BLOCK_BYTES as usize;
            let bit_length = ((message.len() as u64) * 8).to_le_bytes();
            let mut padded = message.to_vec();
            padded.push(0x80);
            let length_offset = (padded.len() + bit_length.len()).div_ceil(block_bytes)
                * block_bytes
                - bit_length.len();
            padded.resize(length_offset, 0);
            padded.extend_from_slice(&bit_length);

            for block in padded.chunks_exact(block_bytes) {
                for (index, &byte) in block.iter().enumerate() {
                    c.bus.write8(DATA + MD5_BLOCK + index as u32, byte);
                }
                set_word(&mut c, A0, DATA);
                c.pc = MD5_TRANSFORM_ENTRY;
                crypt_md5_transform(&mut c);
            }
            let digest = [MD5_STATE_A, MD5_STATE_B, MD5_STATE_C, MD5_STATE_D]
                .map(|offset| c.bus.read32(DATA + offset));
            assert_eq!(digest, expected, "message {message:?}");
        }
    }
}
