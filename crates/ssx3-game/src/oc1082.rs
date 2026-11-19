//! Lobby cipher, message digest, text conversion, and sort helpers.

use crate::abi::{addu, r32, ret};
use crate::call::{call_ptr, guest, Callee, Flow, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::game::lib::libc::callee::MEMCPY;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

mod callee {
    use super::*;

    pub const LOBBY_MEM_ALLOC: u32 = 0x0025_91e8;
    pub const LOBBY_MEM_FREE: u32 = 0x0025_9210;

    pub const LOBBY_CIPHER_INIT: Callee = Callee {
        addr: 0x003f_33b0,
        f: ee::s14::Lobby_CipherInit,
    };
    pub const CRYPT_MD5_TRANSFORM: Callee = Callee {
        addr: 0x003f_2220,
        f: ee::s14::CryptMD5_Transform,
    };
    pub const CRYPT_MD5_INIT: Callee = Callee {
        addr: 0x003f_30f0,
        f: ee::s14::CryptMD5_Init,
    };
    pub const CRYPT_MD5_UPDATE: Callee = Callee {
        addr: 0x003f_3128,
        f: ee::s14::CryptMD5_Update,
    };
    pub const CRYPT_MD5_FINAL: Callee = Callee {
        addr: 0x003f_31f8,
        f: ee::s14::CryptMD5_Final,
    };
    pub const LOBBY_PARSE_HEX: Callee = Callee {
        addr: 0x003e_fc68,
        f: ee::s14::Lobby_ParseHex,
    };
    pub const LOBBY_PARSE_HEX_RANGE: Callee = Callee {
        addr: 0x003e_fcd8,
        f: ee::s14::Lobby_ParseHexRange,
    };
    pub const UTF8_DECODE_CODE_PT: Callee = Callee {
        addr: 0x003e_f640,
        f: ee::s14::Utf8_DecodeCodePt,
    };
    pub const UTF8_TRANSLATE_CODE_PT: Callee = Callee {
        addr: 0x003e_f770,
        f: ee::s14::Utf8_TranslateCodePt,
    };
    pub const UTF8_ENCODE_FROM_UCS2_CODE_PT: Callee = Callee {
        addr: 0x003e_f6f8,
        f: ee::s14::Utf8EncodeFromUCS2CodePt,
    };
    pub const UTF8_GET_SEQ_LEN: Callee = Callee {
        addr: 0x003e_f600,
        f: ee::s14::Utf8_GetSeqLen,
    };
    pub const UTF8_REPLACE: Callee = Callee {
        addr: 0x003e_f828,
        f: ee::s14::Utf8Replace,
    };
}

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_3530, lobby_cipher_apply, "Lobby_CipherApply"),
    (0x003f_33b0, lobby_cipher_init, "Lobby_CipherInit"),
    (0x003f_31f8, crypt_md5_final, "CryptMD5_Final"),
    (0x003f_3128, crypt_md5_update, "CryptMD5_Update"),
    (0x003f_30f0, crypt_md5_init, "CryptMD5_Init"),
    (
        0x003e_fd30,
        lobby_hash_memory_ranges,
        "Lobby_HashMemoryRanges",
    ),
    (0x003e_fcd8, lobby_parse_hex_range, "Lobby_ParseHexRange"),
    (0x003e_fc68, lobby_parse_hex, "Lobby_ParseHex"),
    (0x003e_fba0, utf8_translate_to_8_bit, "Utf8TranslateTo8Bit"),
    (0x003e_faf8, utf8_encode_from_8_bit, "Utf8EncodeFrom8Bit"),
    (0x003e_fa48, utf8_decode_to_ucs2, "Utf8DecodeToUCS2"),
    (0x003e_f9b8, utf8_encode_from_ucs2, "Utf8EncodeFromUCS2"),
    (0x003e_f940, utf8_str_len, "Utf8StrLen"),
    (0x003e_f828, utf8_replace, "Utf8Replace"),
    (0x003e_f808, utf8_strip, "Utf8Strip"),
    (0x003e_f770, utf8_translate_code_pt, "Utf8_TranslateCodePt"),
    (
        0x003e_f6f8,
        utf8_encode_from_ucs2_code_pt,
        "Utf8EncodeFromUCS2CodePt",
    ),
    (0x003e_f640, utf8_decode_code_pt, "Utf8_DecodeCodePt"),
    (0x003e_f600, utf8_get_seq_len, "Utf8_GetSeqLen"),
    (0x003e_f210, lobby_merge_sort, "Lobby_MergeSort"),
];

const LOBBY_CIPHER_BYTES: u32 = 256;
const LOBBY_CIPHER_MASK: u64 = (LOBBY_CIPHER_BYTES - 1) as u64;
const LOBBY_CIPHER_CURSOR: u32 = LOBBY_CIPHER_BYTES;
const LOBBY_CIPHER_ACCUMULATOR: u32 = 0x104;
const LOBBY_CIPHER_CRC_TABLE: u32 = 0x0049_6c08;
const LOBBY_CIPHER_STATIC_STATE: u32 = 0x0051_eef8;
const LOBBY_CIPHER_STATIC_KEY: u32 = 0x0049_7008;
const LOBBY_CIPHER_STATIC_ROUNDS: u64 = 10;
const LOBBY_CIPHER_INITIALIZED_GP: u32 = 0x1aac;
const LOBBY_CIPHER_APPLY_BYTE: u32 = 0x003f_3550;
const LOBBY_CIPHER_INIT_FILL: u32 = 0x003f_3418;
const LOBBY_CIPHER_INIT_KEY_LENGTH: u32 = 0x003f_3458;
const LOBBY_CIPHER_INIT_SHUFFLE: u32 = 0x003f_3498;

/// XOR a lobby buffer with the cipher stream and save the updated cursor and accumulator.
pub fn lobby_cipher_apply(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T0][0] = c.g[A0][0];
    c.g[T2][0] = c.g[A1][0];
    c.g[T3][0] = c.g[A2][0];
    let state = Addr(r32(c, T0));
    c.g[T1][0] = state.off(LOBBY_CIPHER_CURSOR).lbu(c);
    c.g[A3][0] = state.off(LOBBY_CIPHER_ACCUMULATOR).lw(c);
    if c.g[T3][0] as i64 > 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(LOBBY_CIPHER_CRC_TABLE & 0xffff_0000);
        c.g[T4][0] = sx(LOBBY_CIPHER_CRC_TABLE);
        loop {
            c.cycles += 29;
            c.g[V0][0] = c.g[T1][0];
            c.g[V1][0] = c.g[A3][0] & LOBBY_CIPHER_MASK;
            c.g[A0][0] = addu(c.g[T0][0], c.g[V0][0]);
            c.g[A3][0] = sx(r32(c, A3) >> 8);
            c.g[A1][0] = Addr(r32(c, A0)).lbu(c);
            c.g[V0][0] = addu(c.g[V0][0], 1);
            c.g[T1][0] = c.g[V0][0] & LOBBY_CIPHER_MASK;
            c.g[T3][0] = sx(r32(c, T3).wrapping_sub(1));
            c.g[V1][0] ^= c.g[A1][0];
            c.g[A2][0] = addu(c.g[T0][0], c.g[T1][0]);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A1][0] = Addr(r32(c, A2)).lbu(c);
            c.g[V1][0] = addu(c.g[V1][0], c.g[T4][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            c.g[A3][0] ^= c.g[V0][0];
            c.g[A0][0] = c.g[A3][0] & LOBBY_CIPHER_MASK;
            c.g[A0][0] = addu(c.g[T0][0], c.g[A0][0]);
            c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
            Addr(r32(c, A2)).sb(c, c.g[V0][0]);
            c.g[V1][0] = sx(r32(c, A1).wrapping_sub(r32(c, V0)));
            Addr(r32(c, A0)).sb(c, c.g[A1][0]);
            c.g[V1][0] &= LOBBY_CIPHER_MASK;
            c.g[V1][0] = addu(c.g[T0][0], c.g[V1][0]);
            c.g[V0][0] = Addr(r32(c, T2)).lbu(c);
            c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
            c.g[V0][0] ^= c.g[A0][0];
            Addr(r32(c, T2)).sb(c, c.g[V0][0]);
            let more = c.g[T3][0] as i64 > 0;
            c.g[T2][0] = addu(c.g[T2][0], 1);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LOBBY_CIPHER_APPLY_BYTE);
            }
        }
    }
    c.cycles += 3;
    let state = Addr(r32(c, T0));
    state.off(LOBBY_CIPHER_ACCUMULATOR).sw(c, c.g[A3][0]);
    state.off(LOBBY_CIPHER_CURSOR).sb(c, c.g[T1][0]);
    ret(c);
}

/// Initialize or reshuffle a lobby cipher permutation from a key.
pub fn lobby_cipher_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = Addr(r32(c, GP)).off(LOBBY_CIPHER_INITIALIZED_GP).lw(c);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        let initialized = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A1][0];
        if !initialized {
            c.cycles += 9;
            c.g[V0][0] = 1;
            Addr(r32(c, GP))
                .off(LOBBY_CIPHER_INITIALIZED_GP)
                .sw(c, c.g[V0][0]);
            c.g[A0][0] = sx(LOBBY_CIPHER_STATIC_STATE);
            c.g[A1][0] = sx(LOBBY_CIPHER_STATIC_KEY);
            c.g[A2][0] = u64::MAX;
            c.g[A3][0] = LOBBY_CIPHER_STATIC_ROUNDS;
            callee::LOBBY_CIPHER_INIT.call(c, 0x003f_3404)?;
        }

        c.cycles += 2;
        let reset = c.g[S2][0] as i64 >= 0;
        c.g[T1][0] = LOBBY_CIPHER_MASK;
        if reset {
            c.cycles += 1;
            c.g[V0][0] = addu(c.g[S1][0], LOBBY_CIPHER_MASK);
            loop {
                c.cycles += 7;
                Addr(r32(c, V0)).sb(c, c.g[T1][0]);
                c.g[T1][0] = sx(r32(c, T1).wrapping_sub(1));
                let more = c.g[T1][0] as i64 >= 0;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOBBY_CIPHER_INIT_FILL);
                }
            }
            c.cycles += 2;
            let state = Addr(r32(c, S1));
            state.off(LOBBY_CIPHER_CURSOR).sb(c, 0);
            state.off(LOBBY_CIPHER_ACCUMULATOR).sw(c, 0);
        } else {
            c.cycles += 2;
            c.g[S2][0] = sx(r32(c, S2).wrapping_neg());
        }

        c.cycles += 2;
        if (c.g[S0][0] as i64) < 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S3)).lbu(c);
            c.g[S0][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S0][0] = 1;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = addu(c.g[S3][0], c.g[S0][0]);
                    c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    c.g[S0][0] = addu(c.g[S0][0], 1);
                    if event_due(c) {
                        deliver_event_here(c, LOBBY_CIPHER_INIT_KEY_LENGTH);
                    }
                }
            }
        }

        c.cycles += 2;
        if c.g[S0][0] as i64 > 0 {
            c.cycles += 4;
            c.g[T2][0] = sx(r32(c, S2).wrapping_mul(LOBBY_CIPHER_BYTES));
            c.g[A1][0] = 0;
            c.g[T1][0] = 0;
            if c.g[T2][0] as i64 > 0 {
                c.cycles += 3;
                c.g[V0][0] = sx(LOBBY_CIPHER_CRC_TABLE & 0xffff_0000);
                c.g[T4][0] = 0;
                c.g[T3][0] = sx(LOBBY_CIPHER_CRC_TABLE);
                loop {
                    c.cycles += 3;
                    c.div(0, T1, S0);
                    c.cycles += 28;
                    c.g[A0][0] = c.g[A1][0] & LOBBY_CIPHER_MASK;
                    c.g[A2][0] = c.g[T1][0] & LOBBY_CIPHER_MASK;
                    c.g[A2][0] = addu(c.g[S1][0], c.g[A2][0]);
                    c.g[A1][0] = sx(r32(c, A1) >> 8);
                    c.g[A3][0] = Addr(r32(c, A2)).lbu(c);
                    c.g[T1][0] = addu(c.g[T1][0], 1);
                    c.g[T0][0] = ((c.g[T1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                    c.g[V1][0] = c.hi[0];
                    c.g[V1][0] = addu(c.g[S3][0], c.g[V1][0]);
                    c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                    c.g[A0][0] ^= c.g[V0][0];
                    c.g[A0][0] = sx(r32(c, A0) << 2);
                    c.g[A0][0] = addu(c.g[A0][0], c.g[T3][0]);
                    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
                    c.g[A1][0] ^= c.g[V1][0];
                    c.g[V0][0] = c.g[A1][0] & LOBBY_CIPHER_MASK;
                    c.g[V0][0] ^= c.g[A3][0];
                    c.g[A1][0] = sx(r32(c, A1) >> 8);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = addu(c.g[V0][0], c.g[T3][0]);
                    c.g[V1][0] = Addr(r32(c, V0)).lw(c);
                    c.g[A1][0] ^= c.g[V1][0];
                    c.g[A0][0] = c.g[A1][0] & LOBBY_CIPHER_MASK;
                    c.g[A0][0] = addu(c.g[S1][0], c.g[A0][0]);
                    c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
                    Addr(r32(c, A2)).sb(c, c.g[V0][0]);
                    let more = c.g[T0][0] != 0;
                    Addr(r32(c, A0)).sb(c, c.g[A3][0]);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, LOBBY_CIPHER_INIT_SHUFFLE);
                    }
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

const MD5_COUNT: u32 = 0;
const MD5_STATE: u32 = 4;
const MD5_BLOCK: u32 = 0x14;
const MD5_BLOCK_BYTES: u64 = 64;
const MD5_DIGEST_BYTES: u64 = 16;
const MD5_LENGTH_FIELD: u32 = MD5_BLOCK + MD5_BLOCK_BYTES as u32 - 8;
const MD5_LENGTH_INDEX: u64 = MD5_BLOCK_BYTES - 8;
const MD5_INITIAL_STATE: [u32; 4] = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];
const MD5_HEX_TABLE_GP: u32 = 0x1aa8;
const MD5_FINAL_PAD: u32 = 0x003f_3250;
const MD5_FINAL_DIGEST: u32 = 0x003f_32d0;
const MD5_FINAL_CLEAR: u32 = 0x003f_3368;
const MD5_UPDATE_STRING: u32 = 0x003f_3168;
const MD5_UPDATE_COPY: u32 = 0x003f_31a0;

/// Finalize the digest and write 16 binary bytes or a NUL-terminated hexadecimal string.
pub fn crypt_md5_final(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = 0x80;
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x30);
        c.g[S4][0] = 0;
        frame.save(c, S6, 0x20);
        c.g[S5][0] = addu(c.g[S0][0], MD5_BLOCK as u64);
        frame.save(c, S7, 0x10);
        c.g[S6][0] = MD5_LENGTH_INDEX;
        frame.save_ra(c, 0);
        c.g[S7][0] = MD5_BLOCK_BYTES;
        c.g[V0][0] = Addr(r32(c, S0)).off(MD5_COUNT).lw(c);
        c.g[A2][0] = c.g[V0][0] & (MD5_BLOCK_BYTES - 1);
        c.g[V0][0] = addu(c.g[S5][0], c.g[A2][0]);
        loop {
            c.cycles += 4;
            Addr(r32(c, V0)).sb(c, c.g[S2][0]);
            c.g[A2][0] = addu(c.g[A2][0], 1);
            let full = c.g[A2][0] == c.g[S7][0];
            c.g[S2][0] = 0;
            if full {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::CRYPT_MD5_TRANSFORM.call(c, 0x003f_3268)?;
                c.g[A2][0] = 0;
            }
            c.cycles += 2;
            let padding_done = c.g[A2][0] == c.g[S6][0];
            c.g[V0][0] = addu(c.g[S5][0], c.g[A2][0]);
            if padding_done {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, MD5_FINAL_PAD);
            }
        }

        c.cycles += 23;
        let context = Addr(r32(c, S0));
        c.g[V0][0] = context.off(MD5_COUNT).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = context.off(MD5_COUNT).lbu(c);
        c.g[A3][0] = sx(r32(c, V0) >> 29);
        c.g[A2][0] = sx(r32(c, V0) >> 5);
        c.g[A1][0] = sx(r32(c, V0) >> 13);
        c.g[V1][0] = sx(r32(c, V1) << 3);
        c.g[V0][0] = sx(r32(c, V0) >> 21);
        let bit_length = context.off(MD5_LENGTH_FIELD);
        bit_length.off(5).sb(c, 0);
        bit_length.off(6).sb(c, 0);
        bit_length.off(7).sb(c, 0);
        bit_length.off(1).sb(c, c.g[A2][0]);
        bit_length.off(4).sb(c, c.g[A3][0]);
        bit_length.sb(c, c.g[V1][0]);
        bit_length.off(2).sb(c, c.g[A1][0]);
        bit_length.off(3).sb(c, c.g[V0][0]);
        callee::CRYPT_MD5_TRANSFORM.call(c, 0x003f_32b8)?;
        c.g[T1][0] = ((c.g[S3][0] as i64) < (MD5_DIGEST_BYTES * 2 + 1) as i64) as u64;
        c.g[T0][0] = addu(c.g[S0][0], MD5_STATE as u64);
        c.g[T2][0] = MD5_DIGEST_BYTES;
        c.g[A3][0] = c.g[T1][0];
        c.g[A2][0] = 0;

        loop {
            c.cycles += 3;
            c.g[V0][0] = c.g[A2][0] & 3;
            let first_byte = c.g[V0][0] == 0;
            c.g[V0][0] = sx((r32(c, A2) as i32 >> 2) as u32);
            if first_byte {
                c.cycles += 3;
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[V0][0] = addu(c.g[T0][0], c.g[V0][0]);
                c.g[S4][0] = Addr(r32(c, V0)).lw(c);
            }

            c.cycles += 2;
            if c.g[S3][0] == c.g[T2][0] {
                c.cycles += 2;
                Addr(r32(c, S1)).sb(c, c.g[S4][0]);
                c.cycles += 2;
                c.g[S1][0] = addu(c.g[S1][0], 1);
                c.g[A2][0] = addu(c.g[A2][0], 1);
            } else {
                c.cycles += 2;
                if c.g[A3][0] != 0 {
                    c.g[A2][0] = addu(c.g[A2][0], 1);
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = Addr(r32(c, GP)).off(MD5_HEX_TABLE_GP).lw(c);
                    c.g[V0][0] = sx(r32(c, S4) >> 4);
                    c.g[V0][0] &= 0xf;
                    c.g[A1][0] = c.g[S4][0] & 0xf;
                    c.g[A0][0] = addu(c.g[A0][0], c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, A0)).lbu(c);
                    Addr(r32(c, S1)).sb(c, c.g[V1][0]);
                    c.g[S1][0] = addu(c.g[S1][0], 1);
                    c.g[V0][0] = Addr(r32(c, GP)).off(MD5_HEX_TABLE_GP).lw(c);
                    c.g[V0][0] = addu(c.g[V0][0], c.g[A1][0]);
                    c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                    Addr(r32(c, S1)).sb(c, c.g[V1][0]);
                    c.cycles += 2;
                    c.g[S1][0] = addu(c.g[S1][0], 1);
                    c.g[A2][0] = addu(c.g[A2][0], 1);
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < MD5_DIGEST_BYTES as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S4][0] = sx(r32(c, S4) >> 8);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, MD5_FINAL_DIGEST);
            }
        }

        c.cycles += 2;
        let hex_output = c.g[T1][0] == 0;
        c.g[V0][0] = MD5_DIGEST_BYTES;
        if hex_output {
            c.cycles += 2;
            Addr(r32(c, S1)).sb(c, 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
            c.cycles += 1;
            frame.restore(c, S1, 0x70);
        } else {
            c.cycles += 2;
            let binary_output = c.g[S3][0] == c.g[V0][0];
            frame.restore(c, S0, 0x80);
            if binary_output {
                c.cycles += 1;
                frame.restore(c, S1, 0x70);
            } else {
                c.cycles += 2;
                if c.g[S3][0] as i64 <= 0 {
                    frame.restore(c, S1, 0x70);
                } else {
                    // The guest's short-buffer loop keeps the requested size in S3.
                    c.cycles += 1;
                    loop {
                        c.cycles += 7;
                        Addr(r32(c, S1)).sb(c, 0);
                        let more = c.g[S3][0] as i64 > 0;
                        c.g[S1][0] = addu(c.g[S1][0], 1);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, MD5_FINAL_CLEAR);
                        }
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x80);
                    c.cycles += 1;
                    frame.restore(c, S1, 0x70);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Feed bytes or a NUL-terminated string into the lobby's MD5 context.
pub fn crypt_md5_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        let scan = (c.g[S0][0] as i64) < 0;
        frame.save_ra(c, 0);
        if scan {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S2)).lbu(c);
            c.g[S0][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S0][0] = 1;
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = addu(c.g[A1][0], c.g[S0][0]);
                    c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    c.g[S0][0] = addu(c.g[S0][0], 1);
                    if event_due(c) {
                        deliver_event_here(c, MD5_UPDATE_STRING);
                    }
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S1)).off(MD5_COUNT).lw(c);
        c.g[A1][0] = c.g[V0][0] & (MD5_BLOCK_BYTES - 1);
        if c.g[S0][0] as i64 > 0 {
            c.cycles += 4;
            c.g[S3][0] = addu(c.g[S1][0], MD5_BLOCK as u64);
            c.g[S4][0] = MD5_BLOCK_BYTES;
            c.g[A0][0] = Addr(r32(c, S2)).lbu(c);
            loop {
                c.cycles += 8;
                c.g[V1][0] = addu(c.g[S3][0], c.g[A1][0]);
                c.g[S2][0] = addu(c.g[S2][0], 1);
                c.g[A1][0] = addu(c.g[A1][0], 1);
                Addr(r32(c, V1)).sb(c, c.g[A0][0]);
                let count = Addr(r32(c, S1)).off(MD5_COUNT);
                c.g[V0][0] = count.lw(c);
                c.g[V0][0] = addu(c.g[V0][0], 1);
                let full = c.g[A1][0] == c.g[S4][0];
                count.sw(c, c.g[V0][0]);
                if full {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    callee::CRYPT_MD5_TRANSFORM.call(c, 0x003f_31c8)?;
                    c.g[A1][0] = 0;
                }
                c.cycles += 3;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                if c.g[S0][0] as i64 <= 0 {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S2)).lbu(c);
                if event_due(c) {
                    deliver_event_here(c, MD5_UPDATE_COPY);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Seed the digest state and clear the byte count.
pub fn crypt_md5_init(c: &mut Cpu) {
    c.cycles += 14;
    c.g[V1][0] = sx(MD5_INITIAL_STATE[0]);
    c.g[A1][0] = sx(MD5_INITIAL_STATE[1]);
    c.g[A2][0] = sx(MD5_INITIAL_STATE[2]);
    c.g[V0][0] = sx(MD5_INITIAL_STATE[3]);
    let context = Addr(r32(c, A0));
    context.off(MD5_STATE).sw(c, c.g[V1][0]);
    context.off(MD5_STATE + 12).sw(c, c.g[V0][0]);
    context.off(MD5_STATE + 4).sw(c, c.g[A1][0]);
    context.off(MD5_STATE + 8).sw(c, c.g[A2][0]);
    context.off(MD5_COUNT).sw(c, 0);
    ret(c);
}

const LOBBY_HASH_MINIMUM_GP: u32 = 0x19a8;
const LOBBY_HASH_LIMIT_GP: u32 = 0x19ac;
const LOBBY_HASH_START: u32 = 0x60;
const LOBBY_HASH_END: u32 = 0x64;
const LOBBY_HASH_NEXT_RANGE: u32 = 0x003e_fd88;

/// Hash the range description around each parsed span that falls within the game's permitted memory.
pub fn lobby_hash_memory_ranges(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S2, 0xa0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x80);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0xb0);
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x70);
        c.g[S4][0] = c.g[A3][0];
        callee::CRYPT_MD5_INIT.call(c, 0x003e_fd64)?;
        c.g[S1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        callee::CRYPT_MD5_UPDATE.call(c, 0x003e_fd78)?;
        let has_ranges = c.g[S1][0] != 0;
        c.g[A1][0] = c.g[S1][0];
        if has_ranges {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 6;
                c.g[A1][0] = sx(frame.at(c, LOBBY_HASH_START));
                c.g[A2][0] = sx(frame.at(c, LOBBY_HASH_END));
                callee::LOBBY_PARSE_HEX_RANGE.call(c, 0x003e_fd94)?;
                c.g[A1][0] = Addr(frame.at(c, LOBBY_HASH_START)).lw(c);
                let missing_start = c.g[A1][0] == 0;
                c.g[S0][0] = c.g[V0][0];
                if missing_start {
                    break;
                }
                c.cycles += 3;
                c.g[A2][0] = Addr(frame.at(c, LOBBY_HASH_END)).lw(c);
                let missing_end = c.g[A2][0] == 0;
                c.g[V0][0] = Addr(r32(c, GP)).off(LOBBY_HASH_MINIMUM_GP).lw(c);
                if missing_end {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                let below_minimum = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(r32(c, GP)).off(LOBBY_HASH_LIMIT_GP).lw(c);
                if !below_minimum {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                    let below_limit = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
                    if below_limit {
                        c.cycles += 2;
                        let ascending = c.g[V0][0] != 0;
                        c.g[A0][0] = c.g[SP][0];
                        if ascending {
                            c.cycles += 2;
                            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, A1)));
                            callee::CRYPT_MD5_UPDATE.call(c, 0x003e_fdd4)?;
                        }
                    }
                }
                c.cycles += 2;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, LOBBY_HASH_NEXT_RANGE);
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[S1][0];
        }
        c.cycles += 15;
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[SP][0];
        callee::CRYPT_MD5_UPDATE.call(c, 0x003e_fdec)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[A0][0] = c.g[SP][0];
        callee::CRYPT_MD5_FINAL.call(c, 0x003e_fdfc)?;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    });
}

const LOBBY_HEX_RANGE_SEPARATOR: u8 = b':';
const LOBBY_HEX_LIST_SEPARATOR: u8 = b',';
const LOBBY_HEX_NEXT_BYTE: u32 = 0x003e_fc70;

/// Decode a hexadecimal start and end and return the next range's text position.
pub fn lobby_parse_hex_range(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::LOBBY_PARSE_HEX.call(c, 0x003e_fcec)?;
        c.g[V1][0] = Addr(r32(c, V0)).lb(c);
        c.g[A0][0] = LOBBY_HEX_RANGE_SEPARATOR as u64;
        let has_end = c.g[V1][0] == c.g[A0][0];
        c.g[A1][0] = addu(c.g[V0][0], 1);
        if has_end {
            c.cycles += 8;
            c.g[A0][0] = c.g[A1][0];
            c.g[A1][0] = c.g[S0][0];
            callee::LOBBY_PARSE_HEX.call(c, 0x003e_fd08)?;
            c.g[V1][0] = Addr(r32(c, V0)).lb(c);
            c.g[A0][0] = addu(c.g[V0][0], 1);
            c.g[V1][0] ^= LOBBY_HEX_LIST_SEPARATOR as u64;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.g[A0][0];
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Read hexadecimal digits into a word and return the first unread character.
pub fn lobby_parse_hex(c: &mut Cpu) {
    match c.pc {
        0x003e_fc68 => {
            c.cycles += 2;
            c.g[V1][0] = 0;
        }
        LOBBY_HEX_NEXT_BYTE => {
            c.cycles += 1;
            c.g[A0][0] = addu(c.g[A0][0], 1);
        }
        pc => {
            c.jump_out(pc, 0x003e_fc68, 0x003e_fcd8);
            return;
        }
    }
    loop {
        c.cycles += 5;
        let cursor = Addr(r32(c, A0));
        c.g[A2][0] = cursor.lbu(c);
        c.g[V0][0] = (sx(r32(c, A2).wrapping_sub(b'0' as u32)) < 10) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = (sx(r32(c, A2).wrapping_sub(b'a' as u32)) < 6) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[V0][0] = (sx(r32(c, A2).wrapping_sub(b'A' as u32)) < 6) as u64;
                if c.g[V0][0] == 0 {
                    Addr(r32(c, A1)).sw(c, c.g[V1][0]);
                    break;
                }
                c.cycles += 1;
            }
        }
        c.g[A2][0] = cursor.lb(c);
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < LOBBY_HEX_RANGE_SEPARATOR as i64) as u64;
        let decimal = c.g[V0][0] != 0;
        c.g[V1][0] = sx(r32(c, V1) << 4);
        if !decimal {
            c.cycles += 3;
            c.g[V0][0] = addu(c.g[A2][0], 9);
            c.g[V0][0] = sx(r32(c, V0) << 24);
            c.g[A2][0] = sx((r32(c, V0) as i32 >> 24) as u32);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A2][0] & 0xf;
        c.g[V1][0] |= c.g[V0][0];
        if back_edge_unwinds(c, LOBBY_HEX_NEXT_BYTE) {
            return;
        }
        c.cycles += 1;
        c.g[A0][0] = addu(c.g[A0][0], 1);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

const UTF8_HIGH_BIT: u64 = 0x80;
const UTF8_TWO_BYTE_PREFIX: u64 = 0xc0;
const UTF8_THREE_BYTE_PREFIX: u64 = 0xe0;
const UTF8_CONTINUATION_MASK: u64 = 0x3f;
const UTF8_CONTINUATION_BITS: u32 = 6;
const UTF8_TRANSLATION_END: u32 = 4;
const UTF8_TRANSLATION_BYTES: u32 = 8;
const UTF8_TRANSLATION_ROW: u64 = 12;
const UTF8_UNMAPPED_BYTE: u64 = 0xff;
const UCS2_BYTES: u64 = 2;
const UTF8_TERMINATOR_BYTES: u32 = 1;
const UTF8_MAX_UCS2_BYTES: u64 = 3;

/// Convert UTF-8 into the lobby's single-byte encoding, with an optional replacement byte.
pub fn utf8_translate_to_8_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        c.g[A3][0] = sx(r32(c, A3) << 24);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x60);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        c.g[S3][0] = sx(r32(c, A1).wrapping_sub(UTF8_TERMINATOR_BYTES));
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[T0][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = Addr(r32(c, S2)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S6][0] = sx((r32(c, A3) as i32 >> 24) as u32);
        let mut need_terminator_pointer = true;
        if !empty {
            c.cycles += 2;
            let no_room = c.g[S3][0] as i64 <= 0;
            c.g[V1][0] = addu(c.g[S4][0], c.g[S0][0]);
            if no_room {
                need_terminator_pointer = false;
            } else {
                c.cycles += 1;
                c.g[A0][0] = c.g[SP][0];
                loop {
                    c.cycles += 13;
                    c.g[A1][0] = addu(c.g[S2][0], c.g[S1][0]);
                    callee::UTF8_DECODE_CODE_PT.call(c, 0x003e_fc00)?;
                    c.g[A2][0] = Addr(r32(c, SP)).lhu(c);
                    c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                    c.g[A0][0] = addu(c.g[S4][0], c.g[S0][0]);
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A3][0] = c.g[S6][0];
                    callee::UTF8_TRANSLATE_CODE_PT.call(c, 0x003e_fc18)?;
                    c.g[V1][0] = addu(c.g[S2][0], c.g[S1][0]);
                    c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                    c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                    let end = c.g[V0][0] == 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < c.g[S3][0] as i64) as u64;
                    if end {
                        break;
                    }
                    c.cycles += 2;
                    let room = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[SP][0];
                    if !room {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_fbf8);
                    }
                }
            }
        }
        if need_terminator_pointer {
            c.cycles += 1;
            c.g[V1][0] = addu(c.g[S4][0], c.g[S0][0]);
        }
        c.cycles += 12;
        c.g[V0][0] = addu(c.g[S0][0], 1);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        Addr(r32(c, V1)).sb(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Convert a single-byte string to UTF-8 through a byte-to-UCS-2 table.
pub fn utf8_encode_from_8_bit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = sx(r32(c, A1).wrapping_sub(UTF8_MAX_UCS2_BYTES as u32 - 1));
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S4][0] = c.g[A3][0];
        let mut need_terminator_pointer = true;
        if !empty {
            c.cycles += 2;
            let no_room = c.g[S2][0] as i64 <= 0;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
            if no_room {
                need_terminator_pointer = false;
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = addu(c.g[S3][0], c.g[S1][0]);
                    c.g[S0][0] = addu(c.g[S0][0], 1);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_mul(UCS2_BYTES as u32));
                    c.g[V0][0] = addu(c.g[S4][0], c.g[V0][0]);
                    c.g[A1][0] = Addr(r32(c, V0)).lhu(c);
                    callee::UTF8_ENCODE_FROM_UCS2_CODE_PT.call(c, 0x003e_fb58)?;
                    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
                    let end = c.g[V1][0] == 0;
                    c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                    if end {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < c.g[S2][0] as i64) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_fb40);
                    }
                }
            }
        }
        if need_terminator_pointer {
            c.cycles += 1;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = addu(c.g[S1][0], 1);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V1)).sb(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Decode a UTF-8 string into a terminated UCS-2 buffer.
pub fn utf8_decode_to_ucs2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = sx(r32(c, A1).wrapping_sub(1));
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S4][0] = c.g[A0][0];
        if !empty {
            c.cycles += 2;
            c.g[S5][0] = UTF8_MAX_UCS2_BYTES;
            if c.g[S3][0] as i64 > 0 {
                c.cycles += 1;
                c.g[A0][0] = sx(r32(c, S0).wrapping_mul(UCS2_BYTES as u32));
                loop {
                    c.cycles += 11;
                    c.g[A1][0] = addu(c.g[S2][0], c.g[S1][0]);
                    c.g[A0][0] = addu(c.g[S4][0], c.g[A0][0]);
                    callee::UTF8_DECODE_CODE_PT.call(c, 0x003e_fa9c)?;
                    c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                    c.g[A0][0] = addu(c.g[S0][0], 1);
                    c.g[V0][0] = ((c.g[S5][0] as i64) < c.g[V0][0] as i64) as u64;
                    c.g[V1][0] = addu(c.g[S2][0], c.g[S1][0]);
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[A0][0];
                    }
                    c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                    let end = c.g[V0][0] == 0;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < c.g[S3][0] as i64) as u64;
                    if end {
                        break;
                    }
                    c.cycles += 2;
                    let room = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(r32(c, S0).wrapping_mul(UCS2_BYTES as u32));
                    if !room {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_fa90);
                    }
                }
            }
        }
        c.cycles += 13;
        c.g[V1][0] = sx(r32(c, S0).wrapping_mul(UCS2_BYTES as u32));
        c.g[V0][0] = addu(c.g[S0][0], 1);
        c.g[V1][0] = addu(c.g[V1][0], c.g[S4][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V1)).sh(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Encode a terminated UCS-2 string into a bounded UTF-8 buffer.
pub fn utf8_encode_from_ucs2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx(r32(c, A1).wrapping_sub(UTF8_MAX_UCS2_BYTES as u32 - 1));
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lhu(c);
        let empty = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A0][0];
        let mut need_terminator_pointer = true;
        if !empty {
            c.cycles += 2;
            let no_room = c.g[S2][0] as i64 <= 0;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
            if no_room {
                need_terminator_pointer = false;
            } else {
                c.cycles += 2;
                c.g[A1][0] = Addr(r32(c, S0)).lhu(c);
                loop {
                    c.cycles += 6;
                    c.g[A0][0] = addu(c.g[S3][0], c.g[S1][0]);
                    c.g[S0][0] = addu(c.g[S0][0], UCS2_BYTES);
                    callee::UTF8_ENCODE_FROM_UCS2_CODE_PT.call(c, 0x003e_fa04)?;
                    c.g[V1][0] = Addr(r32(c, S0)).lhu(c);
                    let end = c.g[V1][0] == 0;
                    c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                    if end {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < c.g[S2][0] as i64) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = Addr(r32(c, S0)).lhu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_f9f8);
                    }
                }
            }
        }
        if need_terminator_pointer {
            c.cycles += 1;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
        }
        c.cycles += 9;
        c.g[V0][0] = addu(c.g[S1][0], 1);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V1)).sb(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Count sequences in a NUL-terminated UTF-8 string.
pub fn utf8_str_len(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = addu(c.g[S1][0], c.g[S0][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lb(c);
            loop {
                c.cycles += 3;
                c.g[V0][0] &= UTF8_HIGH_BIT;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = addu(c.g[S0][0], 1);
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
                    callee::UTF8_GET_SEQ_LEN.call(c, 0x003e_f984)?;
                    c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                }
                c.cycles += 4;
                c.g[V1][0] = addu(c.g[S1][0], c.g[S0][0]);
                c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                let end = c.g[V0][0] == 0;
                c.g[S2][0] = addu(c.g[S2][0], 1);
                if end {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003e_f970);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Replace non-ASCII sequences after the leading ASCII prefix.
pub fn utf8_replace(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        c.g[A3][0] = sx(r32(c, A3) << 24);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = sx((r32(c, A3) as i32 >> 24) as u32);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S3)).lb(c);
        c.g[V0][0] = c.g[V1][0] & UTF8_HIGH_BIT;
        let initial_ascii = c.g[V0][0] == 0;
        c.g[S5][0] = c.g[A1][0];

        let mut nonempty = true;
        if initial_ascii {
            c.cycles += 2;
            let empty = c.g[V1][0] == 0;
            c.g[A0][0] = c.g[S3][0];
            if empty {
                nonempty = false;
            } else {
                c.cycles += 2;
                c.g[A0][0] = addu(c.g[A0][0], 1);
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, A0)).lb(c);
                    c.g[V0][0] = c.g[V1][0] & UTF8_HIGH_BIT;
                    let non_ascii = c.g[V0][0] != 0;
                    c.g[S1][0] = addu(c.g[S1][0], 1);
                    if non_ascii {
                        break;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    c.g[A0][0] = addu(c.g[A0][0], 1);
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_f880);
                    }
                }
            }
        }

        if nonempty {
            c.cycles += 4;
            c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lb(c);
            let end = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[S1][0] as i64) < c.g[S5][0] as i64) as u64;
            if !end {
                c.cycles += 3;
                c.g[S0][0] = c.g[S1][0];
                let room = c.g[V0][0] != 0;
                c.g[V0][0] = addu(c.g[S2][0], c.g[S0][0]);
                if room {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                    loop {
                        c.cycles += 3;
                        c.g[V0][0] &= UTF8_HIGH_BIT;
                        let ascii = c.g[V0][0] == 0;
                        c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
                        let wrote_byte;
                        if ascii {
                            c.cycles += 3;
                            c.g[V0][0] = addu(c.g[S2][0], c.g[S0][0]);
                            c.g[S1][0] = addu(c.g[S1][0], 1);
                            Addr(r32(c, V0)).sb(c, c.g[A0][0]);
                            wrote_byte = true;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] &= 0xff;
                            callee::UTF8_GET_SEQ_LEN.call(c, 0x003e_f8cc)?;
                            wrote_byte = c.g[S4][0] != 0;
                            c.g[S1][0] = addu(c.g[S1][0], c.g[V0][0]);
                            if wrote_byte {
                                c.cycles += 3;
                                c.g[V0][0] = addu(c.g[S2][0], c.g[S0][0]);
                                Addr(r32(c, V0)).sb(c, c.g[S4][0]);
                            }
                        }
                        if wrote_byte {
                            c.cycles += 1;
                            c.g[S0][0] = addu(c.g[S0][0], 1);
                        }
                        c.cycles += 4;
                        c.g[V1][0] = addu(c.g[S3][0], c.g[S1][0]);
                        c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                        let end = c.g[V0][0] == 0;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < c.g[S5][0] as i64) as u64;
                        if end {
                            break;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_f8b8);
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = addu(c.g[S2][0], c.g[S0][0]);
                }
                c.cycles += 2;
                Addr(r32(c, V0)).sb(c, 0);
                c.g[S0][0] = addu(c.g[S0][0], 1);
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Remove non-ASCII sequences using the replacement routine's zero-byte mode.
pub fn utf8_strip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        callee::UTF8_REPLACE.call(c, 0x003e_f818)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Map one UCS-2 code point through the translation ranges.
pub fn utf8_translate_code_pt(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = sx(r32(c, A3) << 24);
    c.g[T2][0] = c.g[A0][0];
    c.g[A3][0] = sx((r32(c, A3) as i32 >> 24) as u32);
    c.g[A2][0] &= 0xffff;
    c.g[T0][0] = 0;
    c.g[T1][0] = UTF8_UNMAPPED_BYTE;
    loop {
        c.cycles += 3;
        c.g[V1][0] = Addr(r32(c, A1)).off(UTF8_TRANSLATION_END).lw(c);
        if c.g[V1][0] == 0 {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
        let below = c.g[V0][0] != 0;
        if !below {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 8;
                c.g[V0][0] = Addr(r32(c, A1)).lhu(c);
                c.g[V1][0] = Addr(r32(c, A1)).off(UTF8_TRANSLATION_BYTES).lw(c);
                c.g[V0][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
                c.g[A2][0] = c.g[V0][0] & 0xffff;
                c.g[V1][0] = addu(c.g[V1][0], c.g[A2][0]);
                c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                let mut byte = c.g[V0][0];
                if byte == c.g[T1][0] {
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        c.cycles += 1;
                        byte = c.g[A3][0];
                    }
                }
                Addr(r32(c, A0)).sb(c, byte);
                c.cycles += 2;
                c.g[A0][0] = addu(c.g[A0][0], 1);
                c.g[T0][0] = 1;
                break;
            }
        }
        c.g[A1][0] = addu(c.g[A1][0], UTF8_TRANSLATION_ROW);
        if event_due(c) {
            deliver_event_here(c, 0x003e_f788);
        }
    }
    c.cycles += 2;
    if c.g[T0][0] == 0 {
        c.cycles += 2;
        if c.g[A3][0] != 0 {
            c.cycles += 2;
            Addr(r32(c, A0)).sb(c, c.g[A3][0]);
            c.g[A0][0] = addu(c.g[A0][0], 1);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, T2)));
    ret(c);
}

/// Encode one UCS-2 code point and return the number of UTF-8 bytes written.
pub fn utf8_encode_from_ucs2_code_pt(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.g[A1][0] & 0xffff;
    c.g[V0][0] = (c.g[V1][0] < UTF8_HIGH_BIT) as u64;
    let ascii = c.g[V0][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    let out = Addr(r32(c, A2));
    if ascii {
        c.cycles += 3;
        out.sb(c, c.g[V1][0]);
        c.g[A3][0] = 1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V1][0] < 0x800) as u64;
        let two_bytes = c.g[V0][0] != 0;
        c.g[A3][0] = 2;
        if two_bytes {
            c.cycles += 7;
            c.g[V0][0] = c.g[V1][0] & UTF8_CONTINUATION_MASK;
            c.g[V1][0] = sx(r32(c, V1) >> UTF8_CONTINUATION_BITS);
            c.g[V0][0] |= UTF8_HIGH_BIT;
            c.g[V1][0] |= UTF8_TWO_BYTE_PREFIX;
            out.off(1).sb(c, c.g[V0][0]);
            out.sb(c, c.g[V1][0]);
        } else {
            c.cycles += 11;
            c.g[V0][0] = sx(r32(c, V1) >> UTF8_CONTINUATION_BITS);
            c.g[A0][0] = c.g[V1][0] & UTF8_CONTINUATION_MASK;
            c.g[V0][0] &= UTF8_CONTINUATION_MASK;
            c.g[V1][0] = sx(r32(c, V1) >> (UTF8_CONTINUATION_BITS * 2));
            c.g[V1][0] |= UTF8_THREE_BYTE_PREFIX;
            c.g[V0][0] |= UTF8_HIGH_BIT;
            c.g[A0][0] |= UTF8_HIGH_BIT;
            out.sb(c, c.g[V1][0]);
            out.off(2).sb(c, c.g[A0][0]);
            c.g[A3][0] = UTF8_MAX_UCS2_BYTES;
            out.off(1).sb(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

/// Decode one UTF-8 sequence into UCS-2, returning its input width.
pub fn utf8_decode_code_pt(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[A2][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        let input = Addr(r32(c, A1));
        c.g[A0][0] = input.lbu(c);
        c.g[V0][0] = c.g[A0][0] & UTF8_HIGH_BIT;
        let ascii = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if ascii {
            c.cycles += 4;
            c.g[V0][0] = c.g[A0][0] & 0xff;
            c.g[A3][0] = 1;
            Addr(r32(c, A2)).sh(c, c.g[V0][0]);
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.g[A0][0] & 0xe0;
            c.g[V0][0] = UTF8_TWO_BYTE_PREFIX;
            let two_bytes = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = c.g[A0][0] & 0xf0;
            if two_bytes {
                c.cycles += 9;
                c.g[V0][0] = input.lbu(c);
                c.g[A3][0] = 2;
                c.g[V1][0] = input.off(1).lbu(c);
                c.g[V0][0] &= 0x1f;
                c.g[V0][0] = sx(r32(c, V0) << UTF8_CONTINUATION_BITS);
                c.g[V1][0] &= UTF8_CONTINUATION_MASK;
                c.g[V0][0] |= c.g[V1][0];
                Addr(r32(c, A2)).sh(c, c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.g[V0][0] = UTF8_THREE_BYTE_PREFIX;
                let three_bytes = c.g[V1][0] == c.g[V0][0];
                c.g[A3][0] = 3;
                if three_bytes {
                    c.cycles += 12;
                    c.g[V0][0] = input.lbu(c);
                    c.g[V1][0] = input.off(1).lbu(c);
                    c.g[A0][0] = input.off(2).lbu(c);
                    c.g[V0][0] &= 0x0f;
                    c.g[V1][0] &= UTF8_CONTINUATION_MASK;
                    c.g[V0][0] = sx(r32(c, V0) << (UTF8_CONTINUATION_BITS * 2));
                    c.g[V1][0] = sx(r32(c, V1) << UTF8_CONTINUATION_BITS);
                    c.g[A0][0] &= UTF8_CONTINUATION_MASK;
                    c.g[V0][0] |= c.g[V1][0];
                    c.g[V0][0] |= c.g[A0][0];
                    Addr(r32(c, A2)).sh(c, c.g[V0][0]);
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = input.lbu(c);
                    callee::UTF8_GET_SEQ_LEN.call(c, 0x003e_f6e0)?;
                    c.g[A3][0] = c.g[V0][0];
                }
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[A3][0];
        frame.close(c);
        Ok(())
    });
}

/// Count a UTF-8 lead byte's sequence width, up to the guest's limit of seven.
pub fn utf8_get_seq_len(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.g[A0][0] & 0xff;
    c.g[V0][0] = c.g[V1][0] & UTF8_HIGH_BIT;
    let ascii = c.g[V0][0] == 0;
    c.g[A0][0] = 1;
    if !ascii {
        c.cycles += 2;
        c.g[V1][0] = sx(r32(c, V1) << 1);
        loop {
            c.cycles += 4;
            c.g[A0][0] = addu(c.g[A0][0], 1);
            c.g[V0][0] = ((c.g[A0][0] as i64) < 7) as u64;
            let at_limit = c.g[V0][0] == 0;
            c.g[V1][0] &= 0xff;
            if at_limit {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & UTF8_HIGH_BIT;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1) << 1);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_f618);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

const LOBBY_SORT_FRAME: u32 = 0x1d00;
const LOBBY_SORT_STACK_COUNT_LIMIT: i64 = 512;
const LOBBY_SORT_STACK_ITEM_BYTES: i64 = 1024;
const LOBBY_SORT_INDEX_BYTES: u64 = 4;
const LOBBY_SORT_POINTERS: u32 = 0x800;
const LOBBY_SORT_POINTERS_ALT: u32 = LOBBY_SORT_POINTERS + LOBBY_SORT_INDEX_BYTES as u32;
const LOBBY_SORT_STACK_POINTERS: u32 = 0x810;
const LOBBY_SORT_STACK_POINTERS_ALT: u32 = 0x1010;
const LOBBY_SORT_STACK_ITEM: u32 = 0x1810;
const LOBBY_SORT_USER: u32 = 0x1c10;
const LOBBY_SORT_ITEMS: u32 = 0x1c14;
const LOBBY_SORT_COUNT: u32 = 0x1c18;
const LOBBY_SORT_COMPARE: u32 = 0x1c1c;
const LOBBY_SORT_RUN_LENGTHS: u32 = 0x1c20;
const LOBBY_SORT_ITEM_SCRATCH: u32 = 0x1c24;
const LOBBY_SORT_PASS: u32 = 0x1c28;
const LOBBY_SORT_STACK_SCRATCH: u32 = 0x1c2c;
const LOBBY_SORT_SAVE_RUN_SUM: u32 = 0x1c30;
const LOBBY_SORT_SAVE_NEXT_PAIR: u32 = 0x1c40;
const LOBBY_SORT_SAVE_POINTERS: u32 = 0x1c50;
const LOBBY_SORT_SAVE_RA: u32 = 0x1c60;
const LOBBY_SORT_SAVE_FP: u32 = 0x1c70;
const LOBBY_SORT_SAVE_S7: u32 = 0x1c80;
const LOBBY_SORT_SAVE_S6: u32 = 0x1c90;
const LOBBY_SORT_SAVE_S5: u32 = 0x1ca0;
const LOBBY_SORT_SAVE_S4: u32 = 0x1cb0;
const LOBBY_SORT_SAVE_S3: u32 = 0x1cc0;
const LOBBY_SORT_SAVE_S2: u32 = 0x1cd0;
const LOBBY_SORT_SAVE_S1: u32 = 0x1ce0;
const LOBBY_SORT_SAVE_S0: u32 = 0x1cf0;

const LOBBY_SORT_SCAN_RUN: u32 = 0x003e_f308;
const LOBBY_SORT_MERGE_PASS: u32 = 0x003e_f370;
const LOBBY_SORT_MERGE_PAIR: u32 = 0x003e_f3c8;
const LOBBY_SORT_PICK_ENTRY: u32 = 0x003e_f3e8;
const LOBBY_SORT_ADVANCE_RIGHT: u32 = 0x003e_f438;
const LOBBY_SORT_MOVE_ENTRY: u32 = 0x003e_f4d0;
const LOBBY_SORT_ROTATE_ENTRY: u32 = 0x003e_f518;

#[derive(Clone, Copy, PartialEq, Eq)]
enum LobbySortMergeStage {
    Pass,
    Pair,
    PickEntry,
    AdvanceRight,
}

/// Stably merge the lobby list's sorted runs, then move each record into its new slot.
pub fn lobby_merge_sort(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame {
            size: LOBBY_SORT_FRAME,
        };
        match c.pc {
            0x003e_f210 => lobby_sort_start(c),
            LOBBY_SORT_SCAN_RUN => lobby_sort_scan(c, frame),
            LOBBY_SORT_MERGE_PASS => lobby_sort_merge(c, frame, LobbySortMergeStage::Pass),
            LOBBY_SORT_MERGE_PAIR => lobby_sort_merge(c, frame, LobbySortMergeStage::Pair),
            LOBBY_SORT_PICK_ENTRY => lobby_sort_merge(c, frame, LobbySortMergeStage::PickEntry),
            LOBBY_SORT_ADVANCE_RIGHT => {
                lobby_sort_merge(c, frame, LobbySortMergeStage::AdvanceRight)
            }
            LOBBY_SORT_MOVE_ENTRY => lobby_sort_permute(c, frame, false),
            LOBBY_SORT_ROTATE_ENTRY => lobby_sort_permute(c, frame, true),
            pc => {
                c.jump_out(pc, 0x003e_f210, 0x003e_f5fc);
                Ok(())
            }
        }
    });
}

fn lobby_sort_start(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    let frame = Frame::open(c, LOBBY_SORT_FRAME);
    Addr(frame.at(c, LOBBY_SORT_COUNT)).sw(c, c.g[A2][0]);
    frame.save(c, FP, LOBBY_SORT_SAVE_FP);
    frame.save(c, S0, LOBBY_SORT_SAVE_S0);
    c.g[FP][0] = c.g[A3][0];
    frame.save(c, S1, LOBBY_SORT_SAVE_S1);
    c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
    frame.save(c, S2, LOBBY_SORT_SAVE_S2);
    frame.save(c, S3, LOBBY_SORT_SAVE_S3);
    frame.save(c, S4, LOBBY_SORT_SAVE_S4);
    frame.save(c, S5, LOBBY_SORT_SAVE_S5);
    frame.save(c, S6, LOBBY_SORT_SAVE_S6);
    frame.save(c, S7, LOBBY_SORT_SAVE_S7);
    frame.save_ra(c, LOBBY_SORT_SAVE_RA);
    Addr(frame.at(c, LOBBY_SORT_USER)).sw(c, c.g[A0][0]);
    Addr(frame.at(c, LOBBY_SORT_ITEMS)).sw(c, c.g[A1][0]);
    let trivial = c.g[V0][0] != 0;
    Addr(frame.at(c, LOBBY_SORT_COMPARE)).sw(c, c.g[T0][0]);
    if trivial {
        c.cycles += 1;
        frame.restore(c, S0, LOBBY_SORT_SAVE_S0);
        lobby_sort_return(c, frame);
        return Ok(());
    }

    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < LOBBY_SORT_STACK_COUNT_LIMIT) as u64;
    let stack_runs = c.g[V0][0] != 0;
    c.g[V0][0] = sx(frame.at(c, LOBBY_SORT_STACK_POINTERS));
    if stack_runs {
        c.cycles += 5;
        c.g[V1][0] = sx(frame.at(c, LOBBY_SORT_STACK_POINTERS_ALT));
        Addr(frame.at(c, LOBBY_SORT_POINTERS)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).sw(c, c.g[SP][0]);
        Addr(frame.at(c, LOBBY_SORT_POINTERS_ALT)).sw(c, c.g[V1][0]);
    } else {
        c.cycles += 11;
        c.g[V0][0] = Addr(frame.at(c, LOBBY_SORT_COUNT)).lw(c);
        c.g[S0][0] = sx(r32(c, V0).wrapping_mul(LOBBY_SORT_INDEX_BYTES as u32));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_MEM_ALLOC, 0x003e_f288)?;
        Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_MEM_ALLOC, 0x003e_f294)?;
        Addr(frame.at(c, LOBBY_SORT_POINTERS)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_MEM_ALLOC, 0x003e_f2a0)?;
        Addr(frame.at(c, LOBBY_SORT_POINTERS_ALT)).sw(c, c.g[V0][0]);
    }

    c.cycles += 3;
    c.g[V0][0] = ((c.g[FP][0] as i64) <= LOBBY_SORT_STACK_ITEM_BYTES) as u64;
    let stack_item = c.g[V0][0] != 0;
    c.g[V1][0] = sx(frame.at(c, LOBBY_SORT_STACK_ITEM));
    if stack_item {
        c.cycles += 3;
        Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).sw(c, c.g[V1][0]);
        Addr(frame.at(c, LOBBY_SORT_STACK_SCRATCH)).sw(c, c.g[V1][0]);
    } else {
        c.cycles += 5;
        c.g[A0][0] = c.g[FP][0];
        call_ptr(c, callee::LOBBY_MEM_ALLOC, 0x003e_f2c4)?;
        c.g[A0][0] = sx(frame.at(c, LOBBY_SORT_STACK_ITEM));
        Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, LOBBY_SORT_STACK_SCRATCH)).sw(c, c.g[A0][0]);
    }

    c.cycles += 12;
    c.g[V0][0] = Addr(frame.at(c, LOBBY_SORT_ITEMS)).lw(c);
    c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_ITEMS)).lw(c);
    c.g[S3][0] = Addr(frame.at(c, LOBBY_SORT_POINTERS)).lw(c);
    c.lo[0] = c.g[V0][0];
    c.g[V0][0] = Addr(frame.at(c, LOBBY_SORT_COUNT)).lw(c);
    c.g[S0][0] = addu(c.g[V1][0], c.g[FP][0]);
    Addr(r32(c, S3)).sw(c, c.g[V1][0]);
    c.madd(0, S1, V0, FP);
    c.g[S6][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
    c.g[V0][0] = 1;
    let has_next_item = c.g[S0][0] != c.g[S1][0];
    Addr(r32(c, S6)).sw(c, c.g[V0][0]);
    if has_next_item {
        c.cycles += 2;
        c.g[S2][0] = 1;
        c.g[A1][0] = Addr(r32(c, S3)).lw(c);
        lobby_sort_scan(c, frame)
    } else {
        lobby_sort_after_scan(c, frame)
    }
}

fn lobby_sort_scan(c: &mut Cpu, frame: Frame) -> Flow {
    loop {
        c.cycles += 6;
        c.g[A2][0] = c.g[S0][0];
        c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_COMPARE)).lw(c);
        let compare = r32(c, T1);
        c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_USER)).lw(c);
        call_ptr(c, compare, 0x003e_f318)?;
        if c.g[V0][0] as i64 > 0 {
            c.g[S6][0] = addu(c.g[S6][0], LOBBY_SORT_INDEX_BYTES);
            c.cycles += 1;
            Addr(r32(c, S6)).sw(c, c.g[S2][0]);
        } else {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S6)).lw(c);
            c.g[V0][0] = addu(c.g[V0][0], 1);
            Addr(r32(c, S6)).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[S3][0] = addu(c.g[S3][0], LOBBY_SORT_INDEX_BYTES);
        Addr(r32(c, S3)).sw(c, c.g[S0][0]);
        c.g[S0][0] = addu(c.g[S0][0], c.g[FP][0]);
        if c.g[S0][0] == c.g[S1][0] {
            break;
        }
        c.g[A1][0] = Addr(r32(c, S3)).lw(c);
        if back_edge_unwinds(c, LOBBY_SORT_SCAN_RUN) {
            return Err(Unwound);
        }
    }
    lobby_sort_after_scan(c, frame)
}

fn lobby_sort_after_scan(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[S6][0] = addu(c.g[S6][0], LOBBY_SORT_INDEX_BYTES);
    Addr(r32(c, S6)).off(LOBBY_SORT_INDEX_BYTES as u32).sw(c, 0);
    Addr(r32(c, S6)).sw(c, 0);
    c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(LOBBY_SORT_INDEX_BYTES as u32).lw(c);
    let multiple_runs = c.g[V0][0] != 0;
    c.g[A3][0] = 0;
    if multiple_runs {
        c.cycles += 3;
        c.g[T0][0] = sx(frame.at(c, LOBBY_SORT_POINTERS));
        c.g[V0][0] = c.g[A3][0] & 1;
        lobby_sort_merge(c, frame, LobbySortMergeStage::Pass)
    } else {
        lobby_sort_begin_permutation(c, frame)
    }
}

fn lobby_sort_merge(c: &mut Cpu, frame: Frame, mut stage: LobbySortMergeStage) -> Flow {
    loop {
        let mut has_pair = true;
        if stage == LobbySortMergeStage::Pass {
            c.cycles += 21;
            c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
            c.g[V1][0] = sx(r32(c, V0) << 2);
            c.g[S6][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
            c.g[A1][0] = Addr(r32(c, A0)).lw(c);
            c.g[V1][0] = addu(c.g[T0][0], c.g[V1][0]);
            c.g[V0][0] ^= 1;
            c.g[S3][0] = Addr(r32(c, V1)).lw(c);
            c.g[A2][0] = Addr(r32(c, A0)).off(LOBBY_SORT_INDEX_BYTES as u32).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] = addu(c.g[T0][0], c.g[V0][0]);
            c.g[S0][0] = addu(c.g[S3][0], c.g[V1][0]);
            c.g[A0][0] = sx(r32(c, A2) << 2);
            c.g[S1][0] = Addr(r32(c, V0)).lw(c);
            c.g[A3][0] = addu(c.g[A3][0], 1);
            c.g[S4][0] = addu(c.g[S0][0], c.g[A0][0]);
            Addr(frame.at(c, LOBBY_SORT_PASS)).sw(c, c.g[A3][0]);
            c.g[S5][0] = addu(c.g[S6][0], 2 * LOBBY_SORT_INDEX_BYTES);
            c.g[S2][0] = c.g[S3][0];
            has_pair = c.g[A1][0] != 0;
            c.g[S7][0] = c.g[S0][0];
            if has_pair {
                c.cycles += 1;
                stage = LobbySortMergeStage::Pair;
            }
        }

        while has_pair {
            lobby_sort_merge_pair(c, frame, stage)?;
            c.cycles += 13;
            c.g[S3][0] = c.g[S4][0];
            c.g[S6][0] = addu(c.g[S6][0], LOBBY_SORT_INDEX_BYTES);
            c.g[S2][0] = c.g[S3][0];
            c.g[A1][0] = Addr(r32(c, S5)).lw(c);
            c.g[S5][0] = c.g[A3][0];
            c.g[A2][0] = Addr(r32(c, S5)).lw(c);
            c.g[V0][0] = sx(r32(c, A1) << 2);
            c.g[S0][0] = addu(c.g[S3][0], c.g[V0][0]);
            c.g[S5][0] = addu(c.g[S5][0], LOBBY_SORT_INDEX_BYTES);
            c.g[V0][0] = sx(r32(c, A2) << 2);
            c.g[S7][0] = c.g[S0][0];
            has_pair = c.g[A1][0] != 0;
            c.g[S4][0] = addu(c.g[S0][0], c.g[V0][0]);
            if has_pair && back_edge_unwinds(c, LOBBY_SORT_MERGE_PAIR) {
                return Err(Unwound);
            }
            stage = LobbySortMergeStage::Pair;
        }

        c.cycles += 7;
        Addr(r32(c, S6)).off(LOBBY_SORT_INDEX_BYTES as u32).sw(c, 0);
        c.g[A3][0] = Addr(frame.at(c, LOBBY_SORT_PASS)).lw(c);
        Addr(r32(c, S6)).sw(c, 0);
        c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(LOBBY_SORT_INDEX_BYTES as u32).lw(c);
        let more_passes = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[A3][0] & 1;
        if !more_passes {
            break;
        }
        if back_edge_unwinds(c, LOBBY_SORT_MERGE_PASS) {
            return Err(Unwound);
        }
        stage = LobbySortMergeStage::Pass;
    }
    lobby_sort_begin_permutation(c, frame)
}

fn lobby_sort_merge_pair(c: &mut Cpu, frame: Frame, stage: LobbySortMergeStage) -> Flow {
    let mut advance_right = stage == LobbySortMergeStage::AdvanceRight;
    if stage == LobbySortMergeStage::Pair {
        c.cycles += 3;
        c.g[V1][0] = addu(c.g[A1][0], c.g[A2][0]);
        let left_empty = c.g[S3][0] == c.g[S0][0];
        c.g[A3][0] = addu(c.g[S5][0], LOBBY_SORT_INDEX_BYTES);
        if left_empty {
            c.cycles += 2;
            if c.g[S3][0] == c.g[S4][0] {
                Addr(r32(c, S6)).sw(c, c.g[V1][0]);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            advance_right = true;
        }
    }

    loop {
        if !advance_right {
            c.cycles += 2;
            let right_nonempty = c.g[S0][0] != c.g[S4][0];
            if right_nonempty {
                c.g[A1][0] = Addr(r32(c, S2)).lw(c);
                c.cycles += 11;
                c.g[A2][0] = Addr(r32(c, S0)).lw(c);
                c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_USER)).lw(c);
                c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_COMPARE)).lw(c);
                frame.save(c, V1, LOBBY_SORT_SAVE_RUN_SUM);
                frame.save(c, A3, LOBBY_SORT_SAVE_NEXT_PAIR);
                let compare = r32(c, T1);
                frame.save(c, T0, LOBBY_SORT_SAVE_POINTERS);
                call_ptr(c, compare, 0x003e_f418)?;
                frame.restore(c, V1, LOBBY_SORT_SAVE_RUN_SUM);
                frame.restore(c, A3, LOBBY_SORT_SAVE_NEXT_PAIR);
                advance_right = c.g[V0][0] as i64 > 0;
                frame.restore(c, T0, LOBBY_SORT_SAVE_POINTERS);
            }
            if advance_right {
                c.cycles += 1;
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S2)).lw(c);
                c.g[S2][0] = addu(c.g[S2][0], LOBBY_SORT_INDEX_BYTES);
            }
        }
        if advance_right {
            c.cycles += 1;
            c.g[S0][0] = addu(c.g[S0][0], LOBBY_SORT_INDEX_BYTES);
        }

        c.cycles += 3;
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        let left_nonempty = c.g[S2][0] != c.g[S7][0];
        c.g[S1][0] = addu(c.g[S1][0], LOBBY_SORT_INDEX_BYTES);
        if left_nonempty {
            advance_right = false;
            if back_edge_unwinds(c, LOBBY_SORT_PICK_ENTRY) {
                return Err(Unwound);
            }
        } else {
            c.cycles += 2;
            if c.g[S0][0] == c.g[S4][0] {
                break;
            }
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            advance_right = true;
            if back_edge_unwinds(c, LOBBY_SORT_ADVANCE_RIGHT) {
                return Err(Unwound);
            }
        }
    }
    c.cycles += 1;
    Addr(r32(c, S6)).sw(c, c.g[V1][0]);
    Ok(())
}

fn lobby_sort_begin_permutation(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let already_ordered = c.g[A3][0] == 0;
    c.g[V0][0] = c.g[A3][0] & 1;
    if !already_ordered {
        c.cycles += 7;
        c.g[V1][0] = sx(frame.at(c, LOBBY_SORT_POINTERS));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_COUNT)).lw(c);
        c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
        c.g[S4][0] = 0;
        let has_items = c.g[A0][0] as i64 > 0;
        c.g[S3][0] = Addr(r32(c, V1)).lw(c);
        if has_items {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, S4) << 2);
            return lobby_sort_permute(c, frame, false);
        }
    }
    lobby_sort_free(c, frame)
}

fn lobby_sort_permute(c: &mut Cpu, frame: Frame, mut resume_rotation: bool) -> Flow {
    loop {
        let mut has_entry = true;
        let mut rotate = resume_rotation;
        if !resume_rotation {
            c.cycles += 4;
            c.g[S0][0] = addu(c.g[V0][0], c.g[S3][0]);
            c.g[A1][0] = Addr(r32(c, S0)).lw(c);
            has_entry = c.g[A1][0] != 0;
            c.g[S5][0] = addu(c.g[S4][0], 1);
            if has_entry {
                c.cycles += 5;
                c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).lw(c);
                c.g[A2][0] = c.g[FP][0];
                call_ptr(c, MEMCPY.addr, 0x003e_f4ec)?;

                c.cycles += 7;
                c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_ITEMS)).lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T1)));
                c.div(0, V1, FP);
                c.g[S2][0] = c.lo[0];
                rotate = c.g[S2][0] != c.g[S4][0];
                c.g[S0][0] = c.g[S4][0];
                if rotate {
                    c.cycles += 2;
                    c.g[S0][0] = sx(r32(c, S0) << 2);
                }
            }
        }

        if has_entry {
            while rotate {
                c.cycles += 10;
                c.g[S1][0] = sx(r32(c, S2) << 2);
                c.g[S0][0] = addu(c.g[S0][0], c.g[S3][0]);
                c.g[S1][0] = addu(c.g[S1][0], c.g[S3][0]);
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                c.g[A2][0] = c.g[FP][0];
                c.g[A1][0] = Addr(r32(c, S1)).lw(c);
                call_ptr(c, MEMCPY.addr, 0x003e_f534)?;
                Addr(r32(c, S0)).sw(c, 0);

                c.cycles += 8;
                c.g[S0][0] = c.g[S2][0];
                c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_ITEMS)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
                c.div(0, V0, FP);
                c.g[S2][0] = c.lo[0];
                rotate = c.g[S2][0] != c.g[S4][0];
                if rotate {
                    c.g[S0][0] = sx(r32(c, S0) << 2);
                    if back_edge_unwinds(c, LOBBY_SORT_ROTATE_ENTRY) {
                        return Err(Unwound);
                    }
                }
            }

            c.cycles += 7;
            c.g[S0][0] = sx(r32(c, S0) << 2);
            c.g[A1][0] = Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).lw(c);
            c.g[S0][0] = addu(c.g[S0][0], c.g[S3][0]);
            c.g[A2][0] = c.g[FP][0];
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            call_ptr(c, MEMCPY.addr, 0x003e_f578)?;
            Addr(r32(c, S0)).sw(c, 0);
        }

        c.cycles += 5;
        c.g[V1][0] = Addr(frame.at(c, LOBBY_SORT_COUNT)).lw(c);
        c.g[S4][0] = c.g[S5][0];
        c.g[V0][0] = ((c.g[S4][0] as i64) < c.g[V1][0] as i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, S4) << 2);
        if !more {
            break;
        }
        if back_edge_unwinds(c, LOBBY_SORT_MOVE_ENTRY) {
            return Err(Unwound);
        }
        resume_rotation = false;
    }
    lobby_sort_free(c, frame)
}

fn lobby_sort_free(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_RUN_LENGTHS)).lw(c);
    let heap_runs = c.g[A0][0] != c.g[SP][0];
    c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).lw(c);
    if heap_runs {
        c.cycles += 7;
        call_ptr(c, callee::LOBBY_MEM_FREE, 0x003e_f5a4)?;
        c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_POINTERS)).lw(c);
        call_ptr(c, callee::LOBBY_MEM_FREE, 0x003e_f5ac)?;
        c.g[A0][0] = Addr(frame.at(c, LOBBY_SORT_POINTERS_ALT)).lw(c);
        call_ptr(c, callee::LOBBY_MEM_FREE, 0x003e_f5b4)?;
        c.g[T1][0] = Addr(frame.at(c, LOBBY_SORT_ITEM_SCRATCH)).lw(c);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(frame.at(c, LOBBY_SORT_STACK_SCRATCH)).lw(c);
    let heap_item = c.g[T1][0] != c.g[V0][0];
    frame.restore(c, S0, LOBBY_SORT_SAVE_S0);
    if heap_item {
        c.cycles += 2;
        c.g[A0][0] = c.g[T1][0];
        call_ptr(c, callee::LOBBY_MEM_FREE, 0x003e_f5cc)?;
        c.cycles += 1;
        frame.restore(c, S0, LOBBY_SORT_SAVE_S0);
    }
    lobby_sort_return(c, frame);
    Ok(())
}

fn lobby_sort_return(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, LOBBY_SORT_SAVE_S1);
    frame.restore(c, S2, LOBBY_SORT_SAVE_S2);
    frame.restore(c, S3, LOBBY_SORT_SAVE_S3);
    frame.restore(c, S4, LOBBY_SORT_SAVE_S4);
    frame.restore(c, S5, LOBBY_SORT_SAVE_S5);
    frame.restore(c, S6, LOBBY_SORT_SAVE_S6);
    frame.restore(c, S7, LOBBY_SORT_SAVE_S7);
    frame.restore(c, FP, LOBBY_SORT_SAVE_FP);
    frame.restore_ra(c, LOBBY_SORT_SAVE_RA);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::{check_paths, check_paths_lohi, fresh, scramble, Rng, DATA, GAME_GP};

    const TEXT_OUTPUT: u32 = DATA + 0x2000;
    const TEXT_INPUT: u32 = DATA + 0x3000;
    const TEXT_TABLE: u32 = DATA + 0x4000;

    fn fill_bytes(c: &mut Cpu, address: u32, bytes: &[u8]) {
        for (offset, &byte) in bytes.iter().enumerate() {
            c.bus.write8(address + offset as u32, byte);
        }
    }

    fn prepare_text(c: &mut Cpu, pc: u32, seed: u32) -> Rng {
        let mut rng = Rng(0x4221_4ab8_c95d_cb03 ^ seed as u64);
        scramble(c, &mut rng);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = pc;
        fill_bytes(c, TEXT_OUTPUT, &[0xcd; 96]);
        rng
    }

    fn prepare_cipher(c: &mut Cpu, pc: u32, seed: u32) -> Rng {
        let rng = prepare_text(c, pc, seed);
        for index in 0..LOBBY_CIPHER_BYTES {
            // This is the standard reflected CRC-32 lookup used by the cipher.
            let crc = (0..8).fold(index, |crc, _| {
                (crc >> 1) ^ if crc & 1 == 0 { 0 } else { 0xedb8_8320 }
            });
            c.bus.write32(LOBBY_CIPHER_CRC_TABLE + index * 4, crc);
            c.bus.write8(DATA + index, 255 - index as u8);
        }
        c.bus.write8(DATA + LOBBY_CIPHER_CURSOR, 0xff);
        c.bus.write32(DATA + LOBBY_CIPHER_ACCUMULATOR, 0x846f_ba31);
        c.g[A0][0] = sx(DATA);
        rng
    }

    fn lobby_cipher_apply_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_cipher(c, 0x003f_3530, seed);
        c.g[A1][0] = sx(if seed / 4 % 2 == 0 {
            TEXT_OUTPUT
        } else {
            DATA + 4
        });
        c.g[A2][0] = sx([u32::MAX, 0, 1, 4][(seed % 4) as usize]);
        rng
    }

    #[test]
    fn lobby_cipher_apply_matches() {
        check_paths(
            ee::s14::f_003f3530__generated,
            lobby_cipher_apply,
            8,
            &[9, 40, 127],
            &|c, seed| {
                lobby_cipher_apply_fixture(c, seed);
            },
        );
    }

    driven!(lobby_cipher_apply_generated_driven, ee::s14::f_003f3530__generated);
    driven!(lobby_cipher_apply_hand_driven, lobby_cipher_apply);

    #[test]
    fn lobby_cipher_apply_events() {
        check_events(
            lobby_cipher_apply_generated_driven,
            lobby_cipher_apply_hand_driven,
            9,
            &|c, seed| {
                let mut rng = lobby_cipher_apply_fixture(c, 3);
                let mode = schedule(c, &mut rng, seed, 0x003f_3530);
                if mode != Events::Off {
                    c.next_event = c.cycles + 33;
                }
                mode
            },
        );
    }

    fn lobby_cipher_init_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_cipher(c, 0x003f_33b0, seed);
        let (key, length, rounds) = match seed % 7 {
            0 | 6 => (b"".as_slice(), 0, 0),
            1 => (b"abc".as_slice(), 3, 0),
            2 => (b"".as_slice(), -1, 0),
            3 => (b"abc".as_slice(), -1, 0),
            4 => (b"abc".as_slice(), 3, -1),
            _ => (b"abc".as_slice(), 3, 1),
        };
        c.bus.write32(
            GAME_GP + LOBBY_CIPHER_INITIALIZED_GP,
            (seed % 7 != 6) as u32,
        );
        fill_bytes(c, TEXT_INPUT, key);
        // Synthetic non-empty key for the cold-init recursion.
        fill_bytes(c, LOBBY_CIPHER_STATIC_KEY, b"x\0");
        c.g[A1][0] = sx(TEXT_INPUT);
        c.g[A2][0] = sx(length as u32);
        c.g[A3][0] = sx(rounds as u32);
        rng
    }

    #[test]
    fn lobby_cipher_init_matches() {
        check_paths_lohi(
            ee::s14::f_003f33b0__generated,
            lobby_cipher_init,
            7,
            &[1819, 1824, 1822, 1850, 7970, 9763, 83027],
            &|c, seed| {
                lobby_cipher_init_fixture(c, seed);
            },
        );
    }

    driven!(lobby_cipher_init_generated_driven, ee::s14::f_003f33b0__generated);
    driven!(lobby_cipher_init_hand_driven, lobby_cipher_init);

    #[test]
    fn lobby_cipher_init_events() {
        check_events(
            lobby_cipher_init_generated_driven,
            lobby_cipher_init_hand_driven,
            9,
            &|c, seed| {
                let (fixture, delay) = [(0, 18), (3, 1818), (4, 54)][(seed / 3) as usize];
                let mut rng = lobby_cipher_init_fixture(c, fixture);
                let mode = schedule(c, &mut rng, seed, 0x003f_33b0);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            },
        );
    }

    fn prepare_md5(c: &mut Cpu, pc: u32, seed: u32, count: u32) -> Rng {
        let mut rng = prepare_text(c, pc, seed);
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + MD5_COUNT, count);
        for offset in (MD5_STATE..MD5_BLOCK + MD5_BLOCK_BYTES as u32).step_by(4) {
            c.bus.write32(DATA + offset, rng.next_u32() | 0x101);
        }
        rng
    }

    fn crypt_md5_final_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let count = [0, 55, 56, 63][(seed % 4) as usize];
        let rng = prepare_md5(c, 0x003f_31f8, seed, count);
        c.g[A1][0] = sx(TEXT_OUTPUT);
        c.g[A2][0] = sx([0, 16, 33][(seed / 4 % 3) as usize]);
        c.bus.write32(GAME_GP + MD5_HEX_TABLE_GP, TEXT_TABLE);
        fill_bytes(c, TEXT_TABLE, b"0123456789abcdef");
        rng
    }

    #[test]
    fn crypt_md5_final_matches() {
        check_paths(
            ee::s14::f_003f31f8__generated,
            crypt_md5_final,
            12,
            &[1785, 1816, 2009, 1486, 3084, 3042],
            &|c, seed| {
                crypt_md5_final_fixture(c, seed);
            },
        );
    }

    driven!(crypt_md5_final_generated_driven, ee::s14::f_003f31f8__generated);
    driven!(crypt_md5_final_hand_driven, crypt_md5_final);

    #[test]
    fn crypt_md5_final_events() {
        check_events(
            crypt_md5_final_generated_driven,
            crypt_md5_final_hand_driven,
            6,
            &|c, seed| {
                let mut rng = crypt_md5_final_fixture(c, 4);
                let mode = schedule(c, &mut rng, seed, 0x003f_31f8);
                if mode != Events::Off {
                    c.next_event = c.cycles + [25, 1600][(seed / 3) as usize];
                }
                mode
            },
        );
    }

    fn crypt_md5_update_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let count = [0, 61][(seed / 4 % 2) as usize];
        let rng = prepare_md5(c, 0x003f_3128, seed, count);
        c.g[A1][0] = sx(TEXT_INPUT);
        c.g[A2][0] = sx([0, u32::MAX, 3, u32::MAX][(seed % 4) as usize]);
        if seed % 4 != 1 {
            fill_bytes(c, TEXT_INPUT, b"abc\0");
        }
        rng
    }

    #[test]
    fn crypt_md5_update_matches() {
        check_paths(
            ee::s14::f_003f3128__generated,
            crypt_md5_update,
            8,
            &[22, 25, 59, 85, 1279, 1305],
            &|c, seed| {
                crypt_md5_update_fixture(c, seed);
            },
        );
    }

    driven!(crypt_md5_update_generated_driven, ee::s14::f_003f3128__generated);
    driven!(crypt_md5_update_hand_driven, crypt_md5_update);

    #[test]
    fn crypt_md5_update_events() {
        check_events(
            crypt_md5_update_generated_driven,
            crypt_md5_update_hand_driven,
            6,
            &|c, seed| {
                let (fixture, delay) = [(3, 19), (2, 22)][(seed / 3) as usize];
                let mut rng = crypt_md5_update_fixture(c, fixture);
                let mode = schedule(c, &mut rng, seed, 0x003f_3128);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            },
        );
    }

    #[test]
    fn crypt_md5_init_matches() {
        check_paths(ee::s14::f_003f30f0__generated, crypt_md5_init, 8, &[14], &|c, seed| {
            prepare_md5(c, 0x003f_30f0, seed, u32::MAX);
        });
    }

    #[test]
    fn crypt_md5_stream_known_answers() {
        let vectors = [
            (
                b"".as_slice(),
                [0xd9_8c_1d_d4, 0x04_b2_00_8f, 0x98_09_80_e9, 0x7e_42_f8_ec],
            ),
            (
                b"abc".as_slice(),
                [0x98_50_01_90, 0xb0_4f_d2_3c, 0x7d_3f_96_d6, 0x72_7f_e1_28],
            ),
            (
                b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                    .as_slice(),
                [0xa2_f4_ed_57, 0x55_c9_e3_2b, 0x2e_da_49_ac, 0x7a_b6_07_21],
            ),
        ];
        for (message, expected) in vectors {
            let mut c = fresh();
            c.g[A0][0] = sx(DATA);
            crypt_md5_init(&mut c);
            fill_bytes(&mut c, TEXT_INPUT, message);
            for (part, slice) in message.chunks(17).enumerate() {
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(TEXT_INPUT + (part * 17) as u32);
                c.g[A2][0] = slice.len() as u64;
                crypt_md5_update(&mut c);
            }
            fill_bytes(&mut c, TEXT_OUTPUT, &[0xcd; 17]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(TEXT_OUTPUT);
            c.g[A2][0] = MD5_DIGEST_BYTES;
            crypt_md5_final(&mut c);
            let digest = std::array::from_fn(|i| c.bus.read32(TEXT_OUTPUT + (i as u32) * 4));
            assert_eq!(digest, expected);
            assert_eq!(c.bus.read8(TEXT_OUTPUT + MD5_DIGEST_BYTES as u32), 0xcd);
        }
    }

    fn lobby_hash_memory_ranges_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_text(c, 0x003e_fd30, seed);
        let input: &[u8] = match seed % 9 {
            0 => b"",
            1 => b"0:800103\0",
            2 => b"800101:0\0",
            3 => b"8000ff:800102\0",
            4 => b"800103:800110\0",
            5 => b"800103:800102\0",
            6 => b"800101:800104\0",
            7 => b"800101:800104,800105:800107\0",
            _ => b"800102\0",
        };
        fill_bytes(c, TEXT_INPUT, input);
        fill_bytes(c, DATA + 0x100, b"memory span");
        if seed % 9 == 8 {
            // A missing colon leaves the caller's end slot unchanged.
            c.bus
                .write32(crate::difftest::SP - 0xd0 + LOBBY_HASH_END, DATA + 0x10b);
        }
        c.bus.write32(GAME_GP + LOBBY_HASH_MINIMUM_GP, DATA + 0x100);
        c.bus.write32(GAME_GP + LOBBY_HASH_LIMIT_GP, DATA + 0x110);
        c.g[A0][0] = sx(if input.is_empty() { 0 } else { TEXT_INPUT });
        c.g[A1][0] = (!input.is_empty()) as u64;
        c.g[A2][0] = sx(TEXT_OUTPUT);
        c.g[A3][0] = MD5_DIGEST_BYTES;
        rng
    }

    #[test]
    fn lobby_hash_memory_ranges_matches() {
        check_paths(
            ee::s14::f_003efd30__generated,
            lobby_hash_memory_ranges,
            9,
            &[1909, 2075, 2078, 2194, 2183, 2185, 2228, 2484, 2125],
            &|c, seed| {
                lobby_hash_memory_ranges_fixture(c, seed);
            },
        );
    }

    driven!(
        lobby_hash_memory_ranges_generated_driven,
        ee::s14::f_003efd30__generated
    );
    driven!(
        lobby_hash_memory_ranges_hand_driven,
        lobby_hash_memory_ranges
    );

    #[test]
    fn lobby_hash_memory_ranges_events() {
        check_events(
            lobby_hash_memory_ranges_generated_driven,
            lobby_hash_memory_ranges_hand_driven,
            6,
            &|c, seed| {
                let mut rng = lobby_hash_memory_ranges_fixture(c, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_fd30);
                if mode != Events::Off {
                    c.next_event = c.cycles + 350;
                }
                mode
            },
        );
    }

    #[test]
    fn lobby_parse_hex_range_matches() {
        check_paths(
            ee::s14::f_003efcd8__generated,
            lobby_parse_hex_range,
            6,
            &[43, 98],
            &|c, seed| {
                prepare_text(c, 0x003e_fcd8, seed);
                let input: &[u8] = match seed % 3 {
                    0 => b"1\0",
                    1 => b"a:B\0",
                    _ => b"a:B,9\0",
                };
                fill_bytes(c, TEXT_INPUT, input);
                c.g[A0][0] = sx(TEXT_INPUT);
                c.g[A1][0] = sx(TEXT_OUTPUT);
                c.g[A2][0] = sx(TEXT_OUTPUT + 4);
            },
        );
    }

    fn lobby_parse_hex_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_text(c, 0x003e_fc68, seed);
        let input: &[u8] = match seed % 6 {
            0 => b"\0",
            1 => b"9\0",
            2 => b"f\0",
            3 => b"F\0",
            4 => b"1aB\xff\0",
            _ => b"ffffffff9:\0",
        };
        fill_bytes(c, TEXT_INPUT, input);
        c.g[A0][0] = sx(TEXT_INPUT);
        c.g[A1][0] = sx(TEXT_OUTPUT);
        rng
    }

    #[test]
    fn lobby_parse_hex_matches() {
        check_paths(
            ee::s14::f_003efc68__generated,
            lobby_parse_hex,
            6,
            &[17, 29, 36, 41, 72, 181],
            &|c, seed| {
                lobby_parse_hex_fixture(c, seed);
            },
        );
    }

    driven!(lobby_parse_hex_generated_driven, ee::s14::f_003efc68__generated);
    driven!(lobby_parse_hex_hand_driven, lobby_parse_hex);

    fn require_lobby_resume(
        generated: GuestFn,
        hand: GuestFn,
        head: u32,
        setup: &dyn Fn(&mut Cpu) -> Events,
    ) {
        for test in [generated, hand] {
            let mut c = fresh();
            assert_eq!(setup(&mut c), Events::Unwind);
            test(&mut c);
            assert_eq!(c.pc, head | crate::difftest::events::UNWIND);
        }
    }

    #[test]
    fn lobby_parse_hex_events() {
        let setup = |c: &mut Cpu, seed| {
            let mut rng = lobby_parse_hex_fixture(c, 4);
            let mode = schedule(c, &mut rng, seed, 0x003e_fc68);
            if mode != Events::Off {
                c.next_event = c.cycles + 10;
            }
            mode
        };
        require_lobby_resume(
            ee::s14::f_003efc68__generated,
            lobby_parse_hex,
            LOBBY_HEX_NEXT_BYTE,
            &|c| setup(c, 1),
        );
        check_events(
            lobby_parse_hex_generated_driven,
            lobby_parse_hex_hand_driven,
            6,
            &setup,
        );
    }

    const UTF8_TEST_MAP_BYTES: u32 = TEXT_TABLE + 0x100;

    fn utf8_translation_table(c: &mut Cpu) {
        c.bus.write32(TEXT_TABLE, b'A' as u32);
        c.bus
            .write32(TEXT_TABLE + UTF8_TRANSLATION_END, b'B' as u32);
        c.bus
            .write32(TEXT_TABLE + UTF8_TRANSLATION_BYTES, UTF8_TEST_MAP_BYTES);
        let row = TEXT_TABLE + UTF8_TRANSLATION_ROW as u32;
        c.bus.write32(row, 0xa2);
        c.bus.write32(row + UTF8_TRANSLATION_END, 0xa2);
        c.bus
            .write32(row + UTF8_TRANSLATION_BYTES, UTF8_TEST_MAP_BYTES + 2);
        c.bus
            .write32(row + UTF8_TRANSLATION_ROW as u32 + UTF8_TRANSLATION_END, 0);
        fill_bytes(c, UTF8_TEST_MAP_BYTES, &[0x31, 0xff, 0xf3]);
    }

    fn utf8_byte_string_fixture(c: &mut Cpu, entry: u32, seed: u32) -> Rng {
        let rng = prepare_text(c, entry, seed);
        let input: &[u8] = match seed % 8 {
            0 => b"\0",
            1 => b"A\0",
            2 => b"\xc2\xa2\0",
            3 => b"\xe2\x82\xac\0",
            4 => b"\xf0\x9f\x99\x82A\0",
            5 => b"A\xc2\xa2z\0",
            6 => b"Z\0",
            _ => b"AB\0",
        };
        fill_bytes(c, TEXT_INPUT, input);
        c.g[A0][0] = sx(TEXT_OUTPUT);
        c.g[A1][0] = sx([96, 1, 2, 3][(seed / 8 % 4) as usize]);
        c.g[A2][0] = sx(TEXT_INPUT);
        c.g[A3][0] = [0, 0xff][(seed / 32 % 2) as usize];
        c.g[T0][0] = sx(TEXT_TABLE);
        rng
    }

    fn utf8_translate_to_8_bit_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = utf8_byte_string_fixture(c, 0x003e_fba0, seed);
        utf8_translation_table(c);
        rng
    }

    #[test]
    fn utf8_translate_to_8_bit_matches() {
        check_paths(
            ee::s14::f_003efba0__generated,
            utf8_translate_to_8_bit,
            64,
            &[32, 33, 92, 111, 112, 114, 94],
            &|c, seed| {
                utf8_translate_to_8_bit_fixture(c, seed);
            },
        );
    }

    driven!(
        utf8_translate_to_8_bit_generated_driven,
        ee::s14::f_003efba0__generated
    );
    driven!(utf8_translate_to_8_bit_hand_driven, utf8_translate_to_8_bit);

    #[test]
    fn utf8_translate_to_8_bit_events() {
        check_events(
            utf8_translate_to_8_bit_generated_driven,
            utf8_translate_to_8_bit_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_translate_to_8_bit_fixture(c, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_fba0);
                if mode != Events::Off {
                    c.next_event = c.cycles + 80;
                }
                mode
            },
        );
    }

    fn utf8_encode_from_8_bit_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = utf8_byte_string_fixture(c, 0x003e_faf8, seed);
        let input: &[u8] = match seed % 5 {
            0 => b"\0",
            1 => b"A\0",
            2 => b"\x90\0",
            3 => b"\xf0\0",
            _ => b"A\x90\xf0\0",
        };
        fill_bytes(c, TEXT_INPUT, input);
        c.g[A3][0] = sx(TEXT_TABLE);
        c.g[A1][0] = [96, 1, 3][(seed / 5 % 3) as usize];
        for (byte, code_point) in [(b'A', 0x7f), (0x90, 0x80), (0xf0, 0xffff)] {
            c.bus
                .write16(TEXT_TABLE + byte as u32 * UCS2_BYTES as u32, code_point);
        }
        rng
    }

    #[test]
    fn utf8_encode_from_8_bit_matches() {
        check_paths(
            ee::s14::f_003efaf8__generated,
            utf8_encode_from_8_bit,
            15,
            &[25, 26, 47, 54, 58, 50],
            &|c, seed| {
                utf8_encode_from_8_bit_fixture(c, seed);
            },
        );
    }

    driven!(utf8_encode_from_8_bit_generated_driven, ee::s14::f_003efaf8__generated);
    driven!(utf8_encode_from_8_bit_hand_driven, utf8_encode_from_8_bit);

    #[test]
    fn utf8_encode_from_8_bit_events() {
        check_events(
            utf8_encode_from_8_bit_generated_driven,
            utf8_encode_from_8_bit_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_encode_from_8_bit_fixture(c, 4);
                let mode = schedule(c, &mut rng, seed, 0x003e_faf8);
                if mode != Events::Off {
                    c.next_event = c.cycles + 37;
                }
                mode
            },
        );
    }

    #[test]
    fn utf8_decode_to_ucs2_matches() {
        const BYTE_STRING_CASES: u32 = 32;
        check_paths(
            ee::s14::f_003efa48__generated,
            utf8_decode_to_ucs2,
            BYTE_STRING_CASES + 1,
            &[28, 30, 56, 58, 65, 71, 83, 94, 98, 119, 125],
            &|c, seed| {
                utf8_byte_string_fixture(c, 0x003e_fa48, seed);
                if seed == BYTE_STRING_CASES {
                    // The guest skips five bytes for this lead byte, then decodes B.
                    fill_bytes(c, TEXT_INPUT, b"\xf0\x9f\x99\x82AB\0");
                }
            },
        );
    }

    driven!(utf8_decode_to_ucs2_generated_driven, ee::s14::f_003efa48__generated);
    driven!(utf8_decode_to_ucs2_hand_driven, utf8_decode_to_ucs2);

    #[test]
    fn utf8_decode_to_ucs2_events() {
        check_events(
            utf8_decode_to_ucs2_generated_driven,
            utf8_decode_to_ucs2_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_byte_string_fixture(c, 0x003e_fa48, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_fa48);
                if mode != Events::Off {
                    c.next_event = c.cycles + 44;
                }
                mode
            },
        );
    }

    fn utf8_encode_from_ucs2_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_text(c, 0x003e_f9b8, seed);
        let input: &[u16] = match seed % 5 {
            0 => &[0],
            1 => &[0x7f, 0],
            2 => &[0x80, 0],
            3 => &[0xffff, 0],
            _ => &[0x7f, 0x80, 0xffff, 0],
        };
        for (index, &code_point) in input.iter().enumerate() {
            c.bus
                .write16(TEXT_INPUT + index as u32 * UCS2_BYTES as u32, code_point);
        }
        c.g[A0][0] = sx(TEXT_OUTPUT);
        c.g[A1][0] = [96, 1, 3][(seed / 5 % 3) as usize];
        c.g[A2][0] = sx(TEXT_INPUT);
        rng
    }

    #[test]
    fn utf8_encode_from_ucs2_matches() {
        check_paths(
            ee::s14::f_003ef9b8__generated,
            utf8_encode_from_ucs2,
            15,
            &[22, 23, 41, 48, 52, 44],
            &|c, seed| {
                utf8_encode_from_ucs2_fixture(c, seed);
            },
        );
    }

    driven!(utf8_encode_from_ucs2_generated_driven, ee::s14::f_003ef9b8__generated);
    driven!(utf8_encode_from_ucs2_hand_driven, utf8_encode_from_ucs2);

    #[test]
    fn utf8_encode_from_ucs2_events() {
        check_events(
            utf8_encode_from_ucs2_generated_driven,
            utf8_encode_from_ucs2_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_encode_from_ucs2_fixture(c, 4);
                let mode = schedule(c, &mut rng, seed, 0x003e_f9b8);
                if mode != Events::Off {
                    c.next_event = c.cycles + 33;
                }
                mode
            },
        );
    }

    fn utf8_str_len_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = utf8_byte_string_fixture(c, 0x003e_f940, seed);
        c.g[A0][0] = sx(TEXT_INPUT);
        rng
    }

    #[test]
    fn utf8_str_len_matches() {
        check_paths(
            ee::s14::f_003ef940__generated,
            utf8_str_len,
            8,
            &[17, 26, 51, 58, 65],
            &|c, seed| {
                utf8_str_len_fixture(c, seed);
            },
        );
    }

    driven!(utf8_str_len_generated_driven, ee::s14::f_003ef940__generated);
    driven!(utf8_str_len_hand_driven, utf8_str_len);

    #[test]
    fn utf8_str_len_events() {
        check_events(
            utf8_str_len_generated_driven,
            utf8_str_len_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_str_len_fixture(c, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_f940);
                if mode != Events::Off {
                    c.next_event = c.cycles + 17;
                }
                mode
            },
        );
    }

    fn utf8_replace_fixture(c: &mut Cpu, entry: u32, seed: u32) -> Rng {
        let rng = utf8_byte_string_fixture(c, entry, seed);
        c.g[A1][0] = [96, 0, 1][(seed / 8 % 3) as usize];
        c.g[A3][0] = [0, 0xff][(seed / 24 % 2) as usize];
        rng
    }

    #[test]
    fn utf8_replace_matches() {
        check_paths(
            ee::s14::f_003ef828__generated,
            utf8_replace,
            48,
            &[30, 42, 72, 76, 37, 45],
            &|c, seed| {
                utf8_replace_fixture(c, 0x003e_f828, seed);
            },
        );
    }

    driven!(utf8_replace_generated_driven, ee::s14::f_003ef828__generated);
    driven!(utf8_replace_hand_driven, utf8_replace);

    #[test]
    fn utf8_replace_events() {
        check_events(
            utf8_replace_generated_driven,
            utf8_replace_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_replace_fixture(c, 0x003e_f828, 0);
                let (input, delay): (&[u8], u64) = if seed / 3 == 0 {
                    (b"aa\xc2\xa2aa\0", 25)
                } else {
                    (b"\xc2\xa2az\0", 59)
                };
                fill_bytes(c, TEXT_INPUT, input);
                let mode = schedule(c, &mut rng, seed, 0x003e_f828);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            },
        );
    }

    #[test]
    fn utf8_strip_matches() {
        check_paths(
            ee::s14::f_003ef808__generated,
            utf8_strip,
            8,
            &[37, 79, 49],
            &|c, seed| {
                utf8_replace_fixture(c, 0x003e_f808, seed);
                c.g[A3][0] = 0xff;
            },
        );
    }

    fn utf8_translate_code_pt_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let rng = prepare_text(c, 0x003e_f770, seed);
        utf8_translation_table(c);
        c.g[A0][0] = sx(TEXT_OUTPUT);
        c.g[A1][0] = sx(TEXT_TABLE);
        c.g[A2][0] = [0, 0x43, 0xa3, 0x41, 0x42, 0xa2][(seed % 6) as usize];
        c.g[A3][0] = [0, 0xff][(seed / 6 % 2) as usize];
        rng
    }

    #[test]
    fn utf8_translate_code_pt_matches() {
        check_paths(
            ee::s14::f_003ef770__generated,
            utf8_translate_code_pt,
            12,
            &[29, 31, 32, 34, 35, 37, 30, 33, 40],
            &|c, seed| {
                utf8_translate_code_pt_fixture(c, seed);
            },
        );
    }

    driven!(utf8_translate_code_pt_generated_driven, ee::s14::f_003ef770__generated);
    driven!(utf8_translate_code_pt_hand_driven, utf8_translate_code_pt);

    #[test]
    fn utf8_translate_code_pt_events() {
        check_events(
            utf8_translate_code_pt_generated_driven,
            utf8_translate_code_pt_hand_driven,
            6,
            &|c, seed| {
                let mut rng = utf8_translate_code_pt_fixture(c, 7);
                let mode = schedule(c, &mut rng, seed, 0x003e_f770);
                if mode != Events::Off {
                    c.next_event = c.cycles + 13;
                }
                mode
            },
        );
    }

    #[test]
    fn utf8_encode_from_ucs2_code_pt_matches() {
        check_paths(
            ee::s14::f_003ef6f8__generated,
            utf8_encode_from_ucs2_code_pt,
            8,
            &[9, 16, 20],
            &|c, seed| {
                prepare_text(c, 0x003e_f6f8, seed);
                c.g[A0][0] = sx(TEXT_OUTPUT);
                c.g[A1][0] = 0x1234_5678_ffff_0000
                    | [0, 0x7f, 0x80, 0x7ff, 0x800, 0xd800, 0xffff, 0x20ac][seed as usize];
            },
        );
    }

    #[test]
    fn utf8_decode_code_pt_matches() {
        check_paths(
            ee::s14::f_003ef640__generated,
            utf8_decode_code_pt,
            8,
            &[14, 23, 29, 35, 56, 67],
            &|c, seed| {
                prepare_text(c, 0x003e_f640, seed);
                let bytes: &[u8] = match seed % 8 {
                    0 => b"A",
                    1 => b"\xc2\xa2",
                    2 => b"\xe2\x82\xac",
                    3 => b"\x80\xb0",
                    4 => b"\xf0\x9f\x99\x82",
                    5 => b"\xfe\x80\x80\x80\x80\x80\x80",
                    6 => b"\xc0\x80",
                    _ => b"\xe0\x80\x80",
                };
                fill_bytes(c, TEXT_INPUT, bytes);
                c.g[A0][0] = sx(TEXT_OUTPUT);
                c.g[A1][0] = sx(TEXT_INPUT);
            },
        );
    }

    #[test]
    fn utf8_get_seq_len_matches() {
        check_paths(
            ee::s14::f_003ef600__generated,
            utf8_get_seq_len,
            8,
            &[6, 15, 22, 29, 36, 43, 47],
            &|c, seed| {
                prepare_text(c, 0x003e_f600, seed);
                c.g[A0][0] = 0x1234_5678_ffff_0000
                    | [0x7f, 0x80, 0xc0, 0xe0, 0xf0, 0xf8, 0xfc, 0xff][seed as usize];
            },
        );
    }

    driven!(utf8_get_seq_len_generated_driven, ee::s14::f_003ef600__generated);
    driven!(utf8_get_seq_len_hand_driven, utf8_get_seq_len);

    #[test]
    fn utf8_get_seq_len_events() {
        check_events(
            utf8_get_seq_len_generated_driven,
            utf8_get_seq_len_hand_driven,
            6,
            &|c, seed| {
                let mut rng = prepare_text(c, 0x003e_f600, seed);
                c.g[A0][0] = 0xf8;
                let mode = schedule(c, &mut rng, seed, 0x003e_f600);
                if mode != Events::Off {
                    c.next_event = c.cycles + 10;
                }
                mode
            },
        );
    }

    const LOBBY_SORT_TEST_GATE: u32 = DATA + 0x33000;
    const LOBBY_SORT_TEST_ALLOCATIONS: u32 = LOBBY_SORT_TEST_GATE + 4;
    const LOBBY_SORT_TEST_FREES: u32 = LOBBY_SORT_TEST_GATE + 8;
    const LOBBY_SORT_TEST_MAGIC: u32 = 0xb86d_2e91;
    const LOBBY_SORT_TEST_ITEMS: u32 = DATA + 0x34000;
    const LOBBY_SORT_TEST_HEAP: u32 = DATA + 0x38000;
    const LOBBY_SORT_TEST_ALLOCATION_BYTES: u32 = 0x1000;

    fn lobby_sort_compare(c: &mut Cpu) {
        use crate::difftest::fakes;
        if c.bus.read32(LOBBY_SORT_TEST_GATE) != LOBBY_SORT_TEST_MAGIC {
            fakes::fall_back(
                c,
                fakes::prev_cell(lobby_sort_compare),
                ee::s15::f_004165a8__generated,
            );
            return;
        }
        c.cycles += 1;
        let left = c.bus.read8(r32(c, A1)) as i32;
        let right = c.bus.read8(r32(c, A2)) as i32;
        c.g[V0][0] = sx(left.wrapping_sub(right) as u32);
        ret(c);
    }

    fn lobby_sort_alloc(c: &mut Cpu) {
        use crate::difftest::fakes;
        if c.bus.read32(LOBBY_SORT_TEST_GATE) != LOBBY_SORT_TEST_MAGIC {
            fakes::fall_back(
                c,
                fakes::prev_cell(lobby_sort_alloc),
                ee::s10::f_00317e30__generated,
            );
            return;
        }
        let index = c.bus.read32(LOBBY_SORT_TEST_ALLOCATIONS);
        assert!(index < 4);
        assert!(r32(c, A0) <= LOBBY_SORT_TEST_ALLOCATION_BYTES);
        c.bus.write32(LOBBY_SORT_TEST_ALLOCATIONS, index + 1);
        c.g[V0][0] = sx(LOBBY_SORT_TEST_HEAP + index * LOBBY_SORT_TEST_ALLOCATION_BYTES);
        c.cycles += 1;
        ret(c);
    }

    fn lobby_sort_free_test(c: &mut Cpu) {
        use crate::difftest::fakes;
        if c.bus.read32(LOBBY_SORT_TEST_GATE) != LOBBY_SORT_TEST_MAGIC {
            fakes::fall_back(
                c,
                fakes::prev_cell(lobby_sort_free_test),
                ee::s10::f_00317e98__generated,
            );
            return;
        }
        let index = c.bus.read32(LOBBY_SORT_TEST_FREES);
        assert_eq!(
            r32(c, A0),
            LOBBY_SORT_TEST_HEAP + index * LOBBY_SORT_TEST_ALLOCATION_BYTES
        );
        c.bus.write32(LOBBY_SORT_TEST_FREES, index + 1);
        c.cycles += 1;
        ret(c);
    }

    fn lobby_sort_test_hooks() {
        use crate::difftest::fakes::{hook, prev_cell};
        use std::ptr::addr_of_mut;
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            lobby_sort_compare,
            prev_cell(lobby_sort_compare),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lobby_sort_alloc,
            prev_cell(lobby_sort_alloc),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lobby_sort_free_test,
            prev_cell(lobby_sort_free_test),
        );
    }

    fn lobby_merge_sort_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let mut rng = prepare_text(c, 0x003e_f210, seed);
        let (count, width, keys): (u32, u32, &[u8]) = match seed % 12 {
            0 => (0, 4, &[]),
            1 => (1, 4, &[5]),
            2 => (2, 0, &[3, 1]),
            3 => (2, 4, &[1, 2]),
            4 => (4, 4, &[4, 3, 2, 1]),
            5 => (5, 4, &[1, 4, 2, 5, 6]),
            6 => (3, 4, &[3, 1, 2]),
            7 => (6, 4, &[2, 4, 1, 2, 3, 4]),
            8 => (512, 4, &[2]),
            9 => (2, 1028, &[2, 1]),
            10 => (3, 4, &[3, 2, 1]),
            _ => (6, 4, &[1, 6, 5, 4, 3, 2]),
        };
        for index in 0..count {
            let item = LOBBY_SORT_TEST_ITEMS + index * width;
            let key = keys[index as usize % keys.len()];
            c.bus
                .write32(item, (rng.next_u32() & !0xff) | u32::from(key));
            if width > 4 {
                c.bus.write32(item + width - 4, 0xbad0_0000 | index);
            }
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(LOBBY_SORT_TEST_ITEMS);
        c.g[A2][0] = sx(count);
        c.g[A3][0] = sx(width);
        c.g[T0][0] = sx(crate::game::lib::libc::callee::STRCMP.addr);
        c.bus.write32(LOBBY_SORT_TEST_GATE, LOBBY_SORT_TEST_MAGIC);
        rng
    }

    #[test]
    fn lobby_merge_sort_matches() {
        lobby_sort_test_hooks();
        check_paths_lohi(
            ee::s14::f_003ef210__generated,
            lobby_merge_sort,
            12,
            &[30, 70, 88, 443],
            &|c, seed| {
                lobby_merge_sort_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_merge_sort_preserves_equal_item_order() {
        lobby_sort_test_hooks();
        let mut c = fresh();
        lobby_merge_sort_fixture(&mut c, 7);
        let original: [u32; 6] =
            std::array::from_fn(|index| c.bus.read32(LOBBY_SORT_TEST_ITEMS + (index as u32) * 4));
        lobby_merge_sort(&mut c);
        let reordered: [u32; 6] =
            std::array::from_fn(|index| c.bus.read32(LOBBY_SORT_TEST_ITEMS + (index as u32) * 4));
        assert_eq!(
            reordered,
            [
                original[2],
                original[0],
                original[3],
                original[4],
                original[1],
                original[5]
            ]
        );
    }

    driven!(lobby_merge_sort_generated_driven, ee::s14::f_003ef210__generated);
    driven!(lobby_merge_sort_hand_driven, lobby_merge_sort);

    #[test]
    fn lobby_merge_sort_events() {
        lobby_sort_test_hooks();
        let cases = [
            (4, 58, LOBBY_SORT_SCAN_RUN),
            (4, 138, LOBBY_SORT_PICK_ENTRY),
            (5, 202, LOBBY_SORT_ADVANCE_RIGHT),
            (4, 162, LOBBY_SORT_MERGE_PAIR),
            (4, 215, LOBBY_SORT_MERGE_PASS),
            (6, 301, LOBBY_SORT_ROTATE_ENTRY),
            (6, 405, LOBBY_SORT_MOVE_ENTRY),
        ];
        let setup = |c: &mut Cpu, seed| {
            let (fixture, delay, _) = cases[(seed / 3) as usize];
            let mut rng = lobby_merge_sort_fixture(c, fixture);
            let mode = schedule(c, &mut rng, seed, 0x003e_f210);
            if mode != Events::Off {
                c.next_event = c.cycles + delay;
            }
            mode
        };
        for (index, &(_, _, head)) in cases.iter().enumerate() {
            require_lobby_resume(ee::s14::f_003ef210__generated, lobby_merge_sort, head, &|c| {
                setup(c, index as u32 * 3 + 1)
            });
        }
        check_events(
            lobby_merge_sort_generated_driven,
            lobby_merge_sort_hand_driven,
            3 * cases.len() as u32,
            &setup,
        );
    }
}
