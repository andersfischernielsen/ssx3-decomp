//! Staging module oc422 (wave wave219): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc422`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

const BGB_GET_INSTANCE: Callee = Callee {
    addr: 0x0014_dc80,
    f: ee::s01::BELibrary_GetInstance,
};
const BGB_GET_INTERFACE: Callee = Callee {
    addr: 0x0014_dd58,
    f: ee::s01::BELibrary_GetInterface,
};
const BGB_GET_ALT_CHARACTER: Callee = Callee {
    addr: 0x0014_74c8,
    f: ee::s01::BENewPlayerInt_GetAltCharacterBySlot,
};
const BGB_GET_CHARACTER: Callee = Callee {
    addr: 0x0014_7488,
    f: ee::s01::BENewPlayerInt_GetCharacterBySlot,
};
const BGB_GET_CHAR_BOLTS: Callee = Callee {
    addr: 0x0014_d9d0,
    f: ee::s01::Bolts_GetCharBolts_2,
};
const BGB_GET_CHAR_BOLT_COUNT: Callee = Callee {
    addr: 0x0014_d988,
    f: ee::s01::Bolts_GetCharBoltCount,
};
const BGB_FREE_DB: Callee = Callee {
    addr: 0x0014_d240,
    f: ee::s01::Bolts_FreeDB,
};
const BGB_STRLEN: u32 = 0x0041_6810;
const BGB_STRCPY: u32 = 0x0041_66f4;
const BGB_OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;
const BGB_MEM_COPY: u32 = 0x003e_6574;

/// Bolt entry layout (0x38 bytes): short key at +4, a name string at +0x14
/// (kept only for flags 0x100/0x200/0x400/0x800), four strings at
/// +0x18..+0x24, strings at +0x28 and +0x2c, scratch word at +0x30, flags at
/// +0x34 (bit 7 = marked as kept).
pub(crate) const BGB_BOLT_KEY: u32 = 0x4;
pub(crate) const BGB_BOLT_FLAGS: u32 = 0x34;
/// The per-slot character records (`0x9b50` per rider slot, `0xf88` per
/// character) pass 1 consults, and the slot-independent ones of pass 2 and 3.
#[cfg(test)]
pub(crate) const BGB_SLOT_RECORDS: u32 = 0x004a_6ca8;
#[cfg(test)]
pub(crate) const BGB_PLAIN_RECORDS: u32 = 0x004b_a348;

fn bgb_lw(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    c.lw(a)
}
/// `sw src, off(base)`.
fn bgb_sw_r(c: &mut Cpu, base: usize, off: u32, src: usize) {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    let v = c.g[src][0];
    c.sw(a, v);
}
/// `sw zero, off(base)`.
fn bgb_sw_zero(c: &mut Cpu, base: usize, off: u32) {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    c.sw(a, 0);
}
fn bgb_addi(c: &mut Cpu, dst: usize, src: usize, imm: u32) {
    c.g[dst][0] = sx((c.g[src][0] as u32).wrapping_add(imm));
}
/// `dst = src + 1` style bump of the string-length accumulator: `s0 = s4 + 1`,
/// `strlen(a0)`, `s4 = s0 + v0`.
fn bgb_add_len(c: &mut Cpu, ret: u32) -> Flow {
    bgb_addi(c, S0, S4, 1);
    call_ptr(c, BGB_STRLEN, ret)?;
    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Ok(())
}

/// The four-name loop (`s1` walks the strings at bolt+0x18, `s3` counts 3
/// down to 0): add every name's length plus one to `s4`.
fn bgb_names(c: &mut Cpu, head: u32, ret: u32) -> Flow {
    loop {
        c.cycles += 3;
        c.g[A0][0] = bgb_lw(c, S1, 0);
        if c.g[A0][0] == 0 {
            bgb_addi(c, S3, S3, 0xffff_ffff);
        } else {
            c.cycles += 4;
            bgb_add_len(c, ret)?;
            bgb_addi(c, S3, S3, 0xffff_ffff);
        }
        c.cycles += 2;
        let more = (c.g[S3][0] as i64) >= 0;
        bgb_addi(c, S1, S1, 4);
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The tail of a bolt (`s2`) tally: the +0x2c and +0x28 names, then mark the
/// bolt kept (`flags |= 0x80`) and take the new count `s5` into `fp`.
fn bgb_tail(c: &mut Cpu, pre: u64, ret_a: u32, ret_b: u32) -> Flow {
    c.cycles += pre;
    c.g[A0][0] = bgb_lw(c, S2, 0x2c);
    if c.g[A0][0] == 0 {
        c.g[A0][0] = bgb_lw(c, S2, 0x28);
    } else {
        c.cycles += 4;
        bgb_add_len(c, ret_a)?;
        c.g[A0][0] = bgb_lw(c, S2, 0x28);
    }
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.g[V0][0] = bgb_lw(c, S2, 0x34);
    } else {
        c.cycles += 4;
        bgb_add_len(c, ret_b)?;
        c.g[V0][0] = bgb_lw(c, S2, 0x34);
    }
    c.cycles += 3;
    c.g[FP][0] = c.g[S5][0];
    c.g[V0][0] |= 0x80;
    bgb_sw_r(c, S2, 0x34, V0);
    Ok(())
}

/// `s5 = fp + 1; s1 = bolt + 0x18; s3 = 3`: the setup of the name loop.
fn bgb_names_setup(c: &mut Cpu) {
    bgb_addi(c, S5, FP, 1);
    bgb_addi(c, S1, S2, 0x18);
    c.g[S3][0] = 3;
}

/// The key to flags lookup: `*(base + 0x288)` is a table of shorts indexed by
/// the bolt's key (negative = none); the entry is `base + 0x290 + 4 * index`
/// and `dst` receives it (0 when none).
fn bgb_entry(c: &mut Cpu, base: u32, dst: usize) {
    let key = bgb_lh(c, S2, BGB_BOLT_KEY);
    c.g[V0][0] = key;
    c.g[V1][0] = c.lw(base.wrapping_add(0x288));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lh(c.g[V0][0] as u32);
    let none = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if !none {
        c.cycles += 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
        c.g[dst][0] = sx(base.wrapping_add(c.g[V0][0] as u32));
    } else {
        c.cycles += 1;
        c.g[dst][0] = 0;
    }
}
fn bgb_lh(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = (c.g[base][0] as u32).wrapping_add(off);
    c.lh(a)
}
fn bgb_lhu_2(c: &mut Cpu, base: usize) -> u64 {
    let a = (c.g[base][0] as u32).wrapping_add(2);
    c.lhu(a)
}

/// Pass 1, character 0..9 (`s0`) of rider slot `s3`: every unmarked bolt of
/// the character whose record flags (selected by the bolt's own flags) say so
/// is tallied (names, then marked kept).
fn bgb_pass1_low(c: &mut Cpu) -> Flow {
    c.cycles += 21;
    c.g[A1][0] = c.g[S0][0];
    c.g[S1][0] = 0;
    BGB_GET_CHAR_BOLTS.call(c, 0x0014_c89c)?;
    bgb_addi(c, A1, S3, 1);
    c.g[A0][0] = bgb_lw(c, SP, 0);
    bgb_sw_r(c, SP, 0x40, A1);
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    BGB_GET_CHAR_BOLT_COUNT.call(c, 0x0014_c8b4)?;
    c.g[V1][0] = 0x9b50;
    c.g[A0][0] = 0x4a0000;
    bgb_sw_r(c, SP, 0xc, V0);
    c.mult(0, V1, S3, V1);
    bgb_addi(c, A0, A0, 0x6ca8);
    c.g[V0][0] = 0xf88;
    c.lo[0] = c.g[A0][0];
    c.g[A1][0] = bgb_lw(c, SP, 0xc);
    c.madd(0, V0, S0, V0);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let none = (c.g[A1][0] as i64) <= 0;
    bgb_sw_r(c, SP, 0x10, V1);
    if none {
        c.cycles += 1;
        c.g[S3][0] = bgb_lw(c, SP, 0x40);
        return Ok(());
    }
    c.cycles += 1;
    loop {
        'entry: {
            c.cycles += 6;
            c.g[V0][0] = bgb_lw(c, S2, BGB_BOLT_FLAGS);
            bgb_addi(c, S7, S1, 1);
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] &= 0x80;
            let kept = c.g[V0][0] != 0;
            bgb_addi(c, S6, S2, 0x38);
            if kept {
                break 'entry;
            }
            c.cycles += 8;
            c.g[A2][0] = bgb_lw(c, SP, 0x10);
            let base = c.g[A2][0] as u32;
            bgb_entry(c, base, V1);
            // The bolt's flags (a0) choose which tests apply.
            let mut plain = false;
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & 0x100;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = bgb_lhu_2(c, V1);
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[A0][0] & 0x200;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = bgb_lhu_2(c, V1);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] & 0x400;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = bgb_lhu_2(c, V1);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[A0][0] & 0x800;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = bgb_lhu_2(c, V1);
                            plain = true;
                        } else {
                            c.cycles += 1;
                            c.g[V1][0] = bgb_lhu_2(c, V1);
                        }
                    }
                }
            }
            if !plain {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x10;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = bgb_lw(c, S2, 0x14);
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V1][0] & 0x2;
                    bgb_addi(c, S7, S1, 1);
                    let skip = c.g[V0][0] != 0;
                    bgb_addi(c, S6, S2, 0x38);
                    if skip {
                        break 'entry;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = bgb_lw(c, S2, 0x14);
                }
                c.cycles += 2;
                let no_name = c.g[A0][0] == 0;
                bgb_addi(c, S7, S1, 1);
                if !no_name {
                    c.cycles += 4;
                    bgb_add_len(c, 0x0014_c994)?;
                    bgb_addi(c, S7, S1, 1);
                }
                c.cycles += 5;
                bgb_addi(c, S5, FP, 1);
                bgb_addi(c, S6, S2, 0x38);
                bgb_addi(c, S1, S2, 0x18);
                c.g[S3][0] = 3;
                bgb_names(c, 0x0014_c9b0, 0x0014_c9c4)?;
                bgb_tail(c, 4, 0x0014_ca30, 0x0014_ca48)?;
            } else {
                c.cycles += 4;
                bgb_addi(c, S7, S1, 1);
                c.g[V0][0] &= 0x10;
                let skip = c.g[V0][0] == 0;
                bgb_addi(c, S6, S2, 0x38);
                if skip {
                    break 'entry;
                }
                c.cycles += 3;
                bgb_names_setup(c);
                bgb_names(c, 0x0014_c9f8, 0x0014_ca0c)?;
                bgb_tail(c, 3, 0x0014_ca30, 0x0014_ca48)?;
            }
        }
        c.cycles += 5;
        c.g[A3][0] = bgb_lw(c, SP, 0xc);
        c.g[S1][0] = c.g[S7][0];
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[S6][0];
        if !more {
            c.cycles += 2;
            c.g[S3][0] = bgb_lw(c, SP, 0x40);
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_c8e8);
        }
    }
}

/// Pass 1, character 10..29 (the alternate characters), or any other: the
/// slot counter is saved; for 10..29 every unmarked bolt is tallied.
fn bgb_pass1_high(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
    bgb_addi(c, S3, S3, 1);
    c.g[V0][0] = (c.g[V0][0] < 0x14) as u64;
    let other = c.g[V0][0] == 0;
    bgb_sw_r(c, SP, 0x40, S3);
    if !other {
        c.cycles += 10;
        c.g[A0][0] = bgb_lw(c, SP, 0);
        c.g[A1][0] = c.g[S0][0];
        BGB_GET_CHAR_BOLTS.call(c, 0x0014_ca98)?;
        c.g[A0][0] = bgb_lw(c, SP, 0);
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        BGB_GET_CHAR_BOLT_COUNT.call(c, 0x0014_caa8)?;
        bgb_sw_r(c, SP, 0x14, V0);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lbu(bgb_a(c, S2, BGB_BOLT_FLAGS));
                bgb_addi(c, S6, V1, 1);
                c.g[V0][0] >>= 7;
                let kept = c.g[V0][0] != 0;
                bgb_addi(c, S7, S2, 0x38);
                if !kept {
                    c.cycles += 3;
                    bgb_names_setup(c);
                    bgb_names(c, 0x0014_cad8, 0x0014_caec)?;
                    bgb_tail(c, 3, 0x0014_cb10, 0x0014_cb28)?;
                }
                c.cycles += 5;
                c.g[A0][0] = bgb_lw(c, SP, 0x14);
                c.g[V1][0] = c.g[S6][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[S7][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_cab8);
                }
            }
        }
    }
    c.cycles += 1;
    c.g[S3][0] = bgb_lw(c, SP, 0x40);
    Ok(())
}

fn bgb_a(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}

/// Pass 2: characters 0..9 (`s0`) of the slot-independent records: the
/// unmarked bolts whose record flags have bit 4 are tallied.
fn bgb_pass2(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[S0][0] = 0;
    c.g[V0][0] = 0xf88;
    loop {
        c.cycles += 17;
        c.g[A1][0] = 0x4c0000;
        c.mult(0, V0, S0, V0);
        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5cb8));
        c.g[A0][0] = bgb_lw(c, SP, 0);
        bgb_addi(c, A2, S0, 1);
        c.g[A1][0] = c.g[S0][0];
        bgb_sw_r(c, SP, 0x38, A2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        bgb_sw_r(c, SP, 0x18, V0);
        BGB_GET_CHAR_BOLTS.call(c, 0x0014_cb90)?;
        c.g[A0][0] = bgb_lw(c, SP, 0);
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        BGB_GET_CHAR_BOLT_COUNT.call(c, 0x0014_cba0)?;
        bgb_sw_r(c, SP, 0x1c, V0);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !none {
            c.cycles += 1;
            loop {
                'entry: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lbu(bgb_a(c, S2, BGB_BOLT_FLAGS));
                    bgb_addi(c, S7, A0, 1);
                    c.g[V0][0] >>= 7;
                    let kept = c.g[V0][0] != 0;
                    bgb_addi(c, S6, S2, 0x38);
                    if kept {
                        break 'entry;
                    }
                    c.cycles += 8;
                    c.g[A3][0] = bgb_lw(c, SP, 0x18);
                    let base = c.g[A3][0] as u32;
                    bgb_entry(c, base, V0);
                    c.cycles += 5;
                    c.g[V0][0] = bgb_lhu_2(c, V0);
                    bgb_addi(c, S7, A0, 1);
                    c.g[V0][0] &= 0x10;
                    let skip = c.g[V0][0] == 0;
                    bgb_addi(c, S6, S2, 0x38);
                    if skip {
                        break 'entry;
                    }
                    c.cycles += 4;
                    bgb_names_setup(c);
                    bgb_names(c, 0x0014_cc18, 0x0014_cc2c)?;
                    bgb_tail(c, 3, 0x0014_cc50, 0x0014_cc68)?;
                }
                c.cycles += 5;
                c.g[V1][0] = bgb_lw(c, SP, 0x1c);
                c.g[A0][0] = c.g[S7][0];
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[S6][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_cbb0);
                }
            }
        }
        c.cycles += 4;
        c.g[S0][0] = bgb_lw(c, SP, 0x38);
        c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0xf88;
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0014_cb68);
        }
    }
}

/// Pass 3: characters 10..29: every unmarked bolt is tallied.
fn bgb_pass3(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[S0][0] = 0xa;
    c.g[A0][0] = bgb_lw(c, SP, 0);
    loop {
        c.cycles += 11;
        c.g[A1][0] = c.g[S0][0];
        BGB_GET_CHAR_BOLTS.call(c, 0x0014_ccb0)?;
        bgb_addi(c, A0, S0, 1);
        c.g[S2][0] = c.g[V0][0];
        bgb_sw_r(c, SP, 0x2c, A0);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = bgb_lw(c, SP, 0);
        BGB_GET_CHAR_BOLT_COUNT.call(c, 0x0014_ccc8)?;
        bgb_sw_r(c, SP, 0x20, V0);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 1;
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.lbu(bgb_a(c, S2, BGB_BOLT_FLAGS));
                bgb_addi(c, S6, V1, 1);
                c.g[V0][0] >>= 7;
                let kept = c.g[V0][0] != 0;
                bgb_addi(c, S7, S2, 0x38);
                if !kept {
                    c.cycles += 3;
                    bgb_names_setup(c);
                    bgb_names(c, 0x0014_ccf8, 0x0014_cd0c)?;
                    bgb_tail(c, 3, 0x0014_cd30, 0x0014_cd48)?;
                }
                c.cycles += 5;
                c.g[A1][0] = bgb_lw(c, SP, 0x20);
                c.g[V1][0] = c.g[S6][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[S7][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_ccd8);
                }
            }
        }
        c.cycles += 4;
        c.g[S0][0] = bgb_lw(c, SP, 0x2c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < 30) as u64;
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.g[A0][0] = bgb_lw(c, SP, 0);
        if event_due(c) {
            deliver_event_here(c, 0x0014_cca8);
        }
    }
}

/// `strcpy` the string `a1` into the block at `s2` and advance `s2` past it
/// (`s2 += strlen(src) + 1`); `a0 = s2` going in, as the original passes it.
fn bgb_copy_string_call(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, BGB_STRCPY, ret)
}

/// Pass 4: allocate the new block, copy every kept bolt and its strings, copy
/// the second table behind it, free the old block and install the new one.
fn bgb_pass4(c: &mut Cpu) -> Flow {
    c.cycles += 23;
    c.g[S0][0] = 0x38;
    c.g[V0][0] = bgb_lw(c, SP, 0x3c);
    c.mult(0, S0, FP, S0);
    c.g[A1][0] = 0x460000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x59d8));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[S6][0] = 0;
    c.g[S1][0] = 0;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    call_ptr(c, BGB_OPERATOR_NEW_NAMED, 0x0014_cdb0)?;
    c.g[V1][0] = bgb_lw(c, SP, 0);
    c.g[S5][0] = c.g[V0][0];
    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
    c.g[A0][0] = bgb_lw(c, SP, 0x3c);
    c.g[V0][0] = bgb_lw(c, V1, 0x1c);
    bgb_sw_r(c, SP, 0x24, S0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[S2][0] = c.g[A0][0];
    c.g[S3][0] = bgb_lw(c, V1, 0x4);
    let none = (c.g[V0][0] as i64) <= 0;
    bgb_sw_r(c, SP, 0x28, A0);
    if !none {
        c.cycles += 1;
        loop {
            'entry: {
                c.cycles += 6;
                c.g[V0][0] = c.lbu(bgb_a(c, S3, BGB_BOLT_FLAGS));
                bgb_addi(c, A1, S1, 1);
                bgb_sw_r(c, SP, 0x34, A1);
                c.g[V0][0] >>= 7;
                let skip = c.g[V0][0] == 0;
                bgb_addi(c, S7, S3, 0x38);
                if skip {
                    break 'entry;
                }
                c.cycles += 39;
                // a0 = new bolt (new block + 0x38 * s6); copy the 0x38 bytes.
                c.g[V0][0] = 0x38;
                c.mult(0, V0, S6, V0);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                let (src, dst) = (c.g[S3][0] as u32, c.g[A0][0] as u32);
                c.ldl(3, src.wrapping_add(0x7));
                c.ldr(3, src);
                c.ldl(5, src.wrapping_add(0xf));
                c.ldr(5, src.wrapping_add(0x8));
                c.ldl(6, src.wrapping_add(0x17));
                c.ldr(6, src.wrapping_add(0x10));
                c.ldl(7, src.wrapping_add(0x1f));
                c.ldr(7, src.wrapping_add(0x18));
                c.sdl(3, dst.wrapping_add(0x7));
                c.sdr(3, dst);
                c.sdl(5, dst.wrapping_add(0xf));
                c.sdr(5, dst.wrapping_add(0x8));
                c.sdl(6, dst.wrapping_add(0x17));
                c.sdr(6, dst.wrapping_add(0x10));
                c.sdl(7, dst.wrapping_add(0x1f));
                c.sdr(7, dst.wrapping_add(0x18));
                c.ldl(3, src.wrapping_add(0x27));
                c.ldr(3, src.wrapping_add(0x20));
                c.ldl(5, src.wrapping_add(0x2f));
                c.ldr(5, src.wrapping_add(0x28));
                c.ldl(6, src.wrapping_add(0x37));
                c.ldr(6, src.wrapping_add(0x30));
                c.sdl(3, dst.wrapping_add(0x27));
                c.sdr(3, dst.wrapping_add(0x20));
                c.sdl(5, dst.wrapping_add(0x2f));
                c.sdr(5, dst.wrapping_add(0x28));
                c.sdl(6, dst.wrapping_add(0x37));
                c.g[V0][0] = 0xffff_ffff_ffff_ff7f;
                c.sdr(6, dst.wrapping_add(0x30));
                // The copy is not marked kept.
                c.g[V1][0] = bgb_lw(c, A0, BGB_BOLT_FLAGS);
                c.g[V1][0] &= c.g[V0][0];
                bgb_sw_r(c, A0, BGB_BOLT_FLAGS, V1);
                c.g[V1][0] = bgb_lw(c, S3, BGB_BOLT_FLAGS);
                // The +0x14 name is copied only for the flags 0x100..0x800.
                let mut have_name = true;
                c.g[V0][0] = c.g[V1][0] & 0x100;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = bgb_lw(c, S3, 0x14);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 0x200;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = bgb_lw(c, S3, 0x14);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V1][0] & 0x400;
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = bgb_lw(c, S3, 0x14);
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V1][0] & 0x800;
                            if c.g[V0][0] == 0 {
                                bgb_sw_zero(c, A0, 0x14);
                                have_name = false;
                                c.cycles += 1;
                                c.g[V0][0] = 0x38;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = bgb_lw(c, S3, 0x14);
                            }
                        }
                    }
                }
                if have_name {
                    c.cycles += 2;
                    let empty = c.g[A1][0] == 0;
                    c.g[V0][0] = 0x38;
                    if !empty {
                        c.cycles += 10;
                        bgb_copy_string_call(c, 0x0014_cecc)?;
                        c.g[V0][0] = 0x38;
                        c.mult(0, V0, S6, V0);
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                        bgb_sw_r(c, V1, 0x14, S2);
                        c.g[A0][0] = bgb_lw(c, S3, 0x14);
                        call_ptr(c, BGB_STRLEN, 0x0014_cee4)?;
                        bgb_addi(c, V0, V0, 1);
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.cycles += 1;
                        c.g[V0][0] = 0x38;
                    }
                }
                c.cycles += 10;
                bgb_addi(c, S1, S1, 1);
                c.mult(0, V0, S6, V0);
                bgb_addi(c, A0, S6, 1);
                bgb_addi(c, V1, S5, 0x18);
                bgb_sw_r(c, SP, 0x34, S1);
                bgb_sw_r(c, SP, 0x30, A0);
                bgb_addi(c, S7, S3, 0x38);
                bgb_addi(c, S0, S3, 0x18);
                c.g[S4][0] = 3;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                loop {
                    c.cycles += 3;
                    c.g[A1][0] = bgb_lw(c, S0, 0);
                    if c.g[A1][0] == 0 {
                        bgb_addi(c, S1, S1, 4);
                    } else {
                        c.cycles += 8;
                        bgb_copy_string_call(c, 0x0014_cf2c)?;
                        bgb_sw_r(c, S1, 0, S2);
                        c.g[A0][0] = bgb_lw(c, S0, 0);
                        call_ptr(c, BGB_STRLEN, 0x0014_cf38)?;
                        bgb_addi(c, V0, V0, 1);
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        bgb_addi(c, S1, S1, 4);
                    }
                    c.cycles += 3;
                    bgb_addi(c, S4, S4, 0xffff_ffff);
                    let more = (c.g[S4][0] as i64) >= 0;
                    bgb_addi(c, S0, S0, 4);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0014_cf18);
                    }
                }
                // The +0x2c and +0x28 names.
                c.cycles += 3;
                c.g[A1][0] = bgb_lw(c, S3, 0x2c);
                if c.g[A1][0] == 0 {
                    c.g[A1][0] = bgb_lw(c, S3, 0x28);
                } else {
                    c.cycles += 11;
                    bgb_copy_string_call(c, 0x0014_cf64)?;
                    c.g[V0][0] = 0x38;
                    c.mult(0, V0, S6, V0);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    bgb_sw_r(c, V1, 0x2c, S2);
                    c.g[A0][0] = bgb_lw(c, S3, 0x2c);
                    call_ptr(c, BGB_STRLEN, 0x0014_cf7c)?;
                    bgb_addi(c, V0, V0, 1);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = bgb_lw(c, S3, 0x28);
                }
                c.cycles += 2;
                let none28 = c.g[A1][0] == 0;
                c.g[V1][0] = 0x38;
                if !none28 {
                    c.cycles += 11;
                    bgb_copy_string_call(c, 0x0014_cf98)?;
                    c.g[V0][0] = 0x38;
                    c.mult(0, V0, S6, V0);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    bgb_sw_r(c, V1, 0x28, S2);
                    c.g[A0][0] = bgb_lw(c, S3, 0x28);
                    call_ptr(c, BGB_STRLEN, 0x0014_cfb0)?;
                    bgb_addi(c, V0, V0, 1);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = 0x38;
                }
                c.cycles += 4;
                c.mult(0, V1, S6, V1);
                c.g[S6][0] = bgb_lw(c, SP, 0x30);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                bgb_sw_zero(c, V0, 0x30);
            }
            c.cycles += 6;
            c.g[A0][0] = bgb_lw(c, SP, 0);
            c.g[S1][0] = bgb_lw(c, SP, 0x34);
            c.g[V0][0] = bgb_lw(c, A0, 0x1c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = c.g[S7][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_cde0);
            }
        }
    }
    // Copy the second table behind the bolts.
    c.cycles += 3;
    c.g[A1][0] = bgb_lw(c, SP, 0x8);
    let no_table = (c.g[A1][0] as i64) <= 0;
    c.g[A3][0] = bgb_lw(c, SP, 0);
    if !no_table {
        c.cycles += 5;
        c.g[A0][0] = bgb_lw(c, SP, 0x24);
        c.g[A2][0] = bgb_lw(c, A3, 0x24);
        c.g[A1][0] = bgb_lw(c, A3, 0xc);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
        call_ptr(c, BGB_MEM_COPY, 0x0014_d004)?;
    }
    c.cycles += 24;
    c.g[A0][0] = bgb_lw(c, SP, 0);
    BGB_FREE_DB.call(c, 0x0014_d00c)?;
    Ok(())
}

/// `void Bolts_BuildGameBolts(Bolts* self)` (0x14c7a0): rebuild the bolt
/// database for the game. Pass 1 clears every bolt's 0x80 mark, then for the
/// two rider slots counts the bolts of the character (and its alternate) that
/// qualify; pass 2 and 3 count the bolts of characters 0..9 and 10..29 that
/// qualify; every kept bolt adds its name strings. One block (`0x38 * kept +
/// strings + 8 * table2`) is allocated, the marked bolts and their strings are
/// copied into it, the second table follows, the old block is freed and the
/// database points at the new one.
pub fn bolts_build_game_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xf0);
        let s = sp(c);
        c.sw(s, c.g[A0][0]);
        frame.save(c, 20, 0xa0);
        frame.save(c, 30, 0x60);
        c.g[S4][0] = 0;
        frame.save(c, 16, 0xe0);
        frame.save(c, 17, 0xd0);
        frame.save(c, 18, 0xc0);
        frame.save(c, 19, 0xb0);
        frame.save(c, 21, 0x90);
        frame.save(c, 22, 0x80);
        frame.save(c, 23, 0x70);
        c.sd(s.wrapping_add(0x50), c.g[RA][0]);
        c.g[FP][0] = 0;
        BGB_GET_INSTANCE.call(c, 0x0014_c7dc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        BGB_GET_INTERFACE.call(c, 0x0014_c7e8)?;
        c.sw(s.wrapping_add(0x4), c.g[V0][0]);
        c.g[V0][0] = bgb_lw(c, SP, 0);
        c.g[V1][0] = bgb_lw(c, SP, 0);
        c.g[V0][0] = bgb_lw(c, V0, 0x24);
        bgb_sw_r(c, SP, 0x8, V0);
        c.g[V0][0] = bgb_lw(c, V1, 0x1c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0;
        if !empty {
            // Clear the 0x80 mark of every bolt.
            c.cycles += 2;
            c.g[A3][0] = 0xffff_ffff_ffff_ff7f;
            c.g[A1][0] = 0;
            loop {
                c.cycles += 12;
                c.g[V0][0] = bgb_lw(c, SP, 0);
                bgb_addi(c, A2, A2, 1);
                c.g[A0][0] = bgb_lw(c, V0, 0x4);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = bgb_lw(c, A0, BGB_BOLT_FLAGS);
                c.g[V1][0] &= c.g[A3][0];
                bgb_sw_r(c, A0, BGB_BOLT_FLAGS, V1);
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = bgb_lw(c, V1, 0x1c);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                bgb_addi(c, A1, A1, 0x38);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0014_c810);
                }
            }
        }

        c.cycles += 6;
        c.g[A0][0] = bgb_lw(c, SP, 0x8);
        c.g[S3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        bgb_sw_r(c, SP, 0x3c, A0);
        c.g[A0][0] = bgb_lw(c, SP, 0x4);

        // Pass 1: the two rider slots (s3 = slot).
        loop {
            c.cycles += 7;
            c.g[A1][0] = c.g[S3][0];
            BGB_GET_ALT_CHARACTER.call(c, 0x0014_c860)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
            c.g[V0][0] = (c.g[V0][0] < 0x14) as u64;
            let alt_ok = c.g[V0][0] != 0;
            c.g[V0][0] = (c.g[S0][0] < 0xa) as u64;
            if !alt_ok {
                c.cycles += 5;
                c.g[A0][0] = bgb_lw(c, SP, 0x4);
                c.g[A1][0] = c.g[S3][0];
                BGB_GET_CHARACTER.call(c, 0x0014_c880)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[S0][0] < 0xa) as u64;
            }
            c.cycles += 2;
            let not_low = c.g[V0][0] == 0;
            c.g[A0][0] = bgb_lw(c, SP, 0);
            if !not_low {
                bgb_pass1_low(c)?;
            } else {
                bgb_pass1_high(c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S3][0] as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = bgb_lw(c, SP, 0x4);
            if event_due(c) {
                deliver_event_here(c, 0x0014_c858);
            }
        }
        bgb_pass2(c)?;
        bgb_pass3(c)?;
        bgb_pass4(c)?;

        c.g[V1][0] = bgb_lw(c, SP, 0x8);
        c.g[V0][0] = bgb_lw(c, SP, 0);
        frame.restore(c, 16, 0xe0);
        bgb_sw_r(c, V0, 0x4, S5);
        bgb_sw_r(c, V0, 0x1c, FP);
        bgb_sw_r(c, V0, 0x24, V1);
        frame.restore(c, 17, 0xd0);
        c.g[A0][0] = bgb_lw(c, SP, 0x28);
        frame.restore(c, 18, 0xc0);
        bgb_sw_r(c, V0, 0x14, A0);
        frame.restore(c, 19, 0xb0);
        c.g[A1][0] = bgb_lw(c, SP, 0x24);
        bgb_sw_r(c, V0, 0, S5);
        frame.restore(c, 20, 0xa0);
        frame.restore(c, 21, 0x90);
        frame.restore(c, 22, 0x80);
        frame.restore(c, 23, 0x70);
        frame.restore(c, 30, 0x60);
        frame.restore_ra(c, 0x50);
        bgb_sw_r(c, V0, 0xc, A1);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0014_c7a0, bolts_build_game_bolts, "Bolts_BuildGameBolts")];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, scramble, Rng, DATA, GAME_GP};

    const DB: u32 = DATA + 0x1000;
    const BOLTS: u32 = DATA + 0x2000;
    const LIB: u32 = DATA + 0x3000;
    const IFACE: u32 = DATA + 0x3100;
    const IFACE_VT: u32 = DATA + 0x3200;
    const STRINGS: u32 = DATA + 0x4000;
    const TABLE2: u32 = DATA + 0x6000;
    const KEY_TABLE: u32 = DATA + 0x7000;
    /// `g_beLibrary[]` (the interface list) and `BENewPlayerInt`'s slot table.
    const BE_LIBRARY: u32 = 0x0053_08a8;
    const PLAYER_SLOTS: u32 = 0x0053_4fe0;
    const RETURN_ZERO: u32 = 0x003a_5ab0;

    /// A record table (`+0x288` = the key table, four entries at `+0x290`
    /// whose halfword at +2 holds the flags) with bit 0x10 / 0x2 at random.
    fn record(c: &mut Cpu, r: &mut Rng, base: u32) {
        c.bus.write32(base + 0x288, KEY_TABLE);
        for i in 0..4 {
            let mut flags = r.next_u32() as u16 & 0xff00;
            if r.next_u32() % 2 == 0 {
                flags |= 0x10;
            }
            if r.next_u32() % 2 == 0 {
                flags |= 0x2;
            }
            c.bus.write16(base + 0x290 + 4 * i + 2, flags);
        }
    }

    /// `Bolts_BuildGameBolts`: per-character bolt lists with random keys,
    /// flags (the 0x80 mark, the 0x100..0x800 name classes) and name
    /// strings; the two rider slots pick a low character, an alternate, none
    /// or an out-of-range one; a second table of 0..3 entries. Real
    /// callees, allocator included.
    #[test]
    fn bolts_build_game_bolts_matches() {
        check_lohi(
            ssx3_ee::s01::f_0014c7a0__generated,
            bolts_build_game_bolts,
            80,
            &|c, seed| {
                let mut r = Rng(0x0014_c7a0 ^ (seed as u64) << 20 | 1);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                crate::game::ai::computer::tests::rbc_test_allocator(c);

                // The library singleton, interface 0 and its method.
                c.bus.write32(GAME_GP - 0x1f04, LIB);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_LIBRARY, IFACE);
                c.bus.write32(IFACE + 4, IFACE_VT);
                c.bus.write16(IFACE_VT + 8, 0);
                c.bus.write32(IFACE_VT + 0xc, RETURN_ZERO);

                // Names: eight strings, 0..12 characters.
                for k in 0..8u32 {
                    let len = r.next_u32() % 13;
                    for i in 0..len {
                        c.bus
                            .write8(STRINGS + 0x20 * k + i, b'a' + (r.next_u32() % 26) as u8);
                    }
                    c.bus.write8(STRINGS + 0x20 * k + len, 0);
                }
                // Key table: key -> entry index (negative = none).
                for k in 0..8u32 {
                    let v = [-1i16, 0, 1, 2, 3, 0, -1, 2][k as usize];
                    c.bus.write16(KEY_TABLE + 2 * k, v as u16);
                }
                // Record tables of both rider slots and of passes 2 and 3.
                for slot in 0..2u32 {
                    for ch in 0..10u32 {
                        record(c, &mut r, BGB_SLOT_RECORDS + slot * 0x9b50 + ch * 0xf88);
                    }
                }
                for ch in 0..10u32 {
                    record(c, &mut r, BGB_PLAIN_RECORDS + ch * 0xf88);
                }
                // The rider slots' characters: mode picks low / alternate /
                // none / out of range.
                for slot in 0..2u32 {
                    let mode = (seed >> (2 * slot)) % 4;
                    let (ch, alt): (i8, i8) = match mode {
                        0 => (r.next_u32() as i8 % 10, 10 + (r.next_u32() % 20) as i8),
                        1 => ((r.next_u32() % 10) as i8, -1),
                        2 => (10 + (r.next_u32() % 20) as i8, -1),
                        _ => (if r.next_u32() % 2 == 0 { 40 } else { -1 }, 35),
                    };
                    c.bus.write8(PLAYER_SLOTS + 0x1c * slot + 0x11, ch as u8);
                    c.bus.write8(PLAYER_SLOTS + 0x1c * slot + 0x12, alt as u8);
                }
                // The bolts: per character 0..2 contiguous, then spares.
                let mut n = 0u32;
                for ch in 0..30u32 {
                    let cnt = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() % 3
                    };
                    c.bus.write32(DB + 0xa4 + 4 * ch, n);
                    c.bus.write32(DB + 0x2c + 4 * ch, cnt);
                    n += cnt;
                }
                n += r.next_u32() % 3;
                let name = |r: &mut Rng| -> u32 {
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        STRINGS + 0x20 * (r.next_u32() % 8)
                    }
                };
                for i in 0..n {
                    let b = BOLTS + 0x38 * i;
                    c.bus.write16(b + BGB_BOLT_KEY, (r.next_u32() % 8) as u16);
                    let mut flags = r.next_u32() & 0x7f;
                    if r.next_u32() % 3 == 0 {
                        flags |= 0x80;
                    }
                    flags |= [0, 0, 0x100, 0x200, 0x400, 0x800, 0x300][(r.next_u32() % 7) as usize];
                    c.bus.write32(b + BGB_BOLT_FLAGS, flags);
                    for off in [0x14u32, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
                        let p = name(&mut r);
                        c.bus.write32(b + off, p);
                    }
                    c.bus.write32(b + 0x30, r.next_u32());
                    c.bus.write32(b, r.next_u32());
                    c.bus.write32(b + 8, r.next_u32());
                    c.bus.write32(b + 0xc, r.next_u32());
                }
                let n2 = [0u32, 1, 2, 3][(seed % 4) as usize];
                for i in 0..n2 * 2 {
                    c.bus.write32(TABLE2 + 4 * i, r.next_u32());
                }
                // The header: no allocation to free, the arrays, the counts.
                c.bus.write32(DB, 0);
                c.bus.write32(DB + 0x4, BOLTS);
                c.bus.write32(DB + 0xc, TABLE2);
                c.bus.write32(DB + 0x14, 0);
                c.bus.write32(DB + 0x18, 0);
                c.bus.write32(DB + 0x1c, n);
                c.bus.write32(DB + 0x24, n2);

                c.g[A0][0] = DB as u64;
                c.pc = 0x0014_c7a0;
            },
        );
    }
}
