//! Staging module oc707 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc707`.

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
    (0x0039_fb30, ui_object_ctor, "cUIThing_cUIThing"),
    (0x002a_7ef0, ini_parser_expect, "IniParser_Expect"),
];

/// `IniParser_SkipWhitespace` (0x2a78f8).
const INI_SKIP_WHITESPACE: Callee = Callee {
    addr: 0x002a_78f8,
    f: ee::s08::f_002a78f8,
};

/// `IniParser_GetChar` (0x2a78e0).
const INI_GET_CHAR: Callee = Callee {
    addr: 0x002a_78e0,
    f: ee::s08::f_002a78e0,
};

/// `cUIThing_cUIThing(void* self, void* pScreen, void* pParent)` (0x39fb30):
/// the UI object constructor. Sets the vtable (at +8), the self-linked list
/// node (+0 next, +4 prev), the attribute flags (+0x14: bit 0 and bit 1 set,
/// bits 2, 4, 5, 7, 8 cleared, bits 3 and 6 set, the align field set to 9),
/// zeroes the anim pointer, playback byte, misc data and the zero vectors
/// (copied from a static zero table), sets colour and scale to 1.0 and the
/// two sound ids to 4 and 6. Field offsets follow the reference struct; the
/// generated body copies the zero vectors through `ldl`/`ldr` into
/// t1/t4/a2/a1, so those registers are left as the copies leave them.
pub fn ui_object_ctor(c: &mut Cpu) {
    let this = c.g[A0][0] as u32;
    c.cycles += 69;
    c.g[V0][0] = c.lw(this.wrapping_add(0x14));
    c.g[V1][0] = 0xffff_ffff_ffff_ff7f;
    c.g[A3][0] = 0xffff_ffff_ffff_e0ff;
    c.g[T1][0] = 0xffff_ffff_ffff_ffdf;
    c.g[V0][0] = c.g[V0][0] | 0x1;
    c.g[T2][0] = 0xffff_ffff_ffff_ffef;
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[T3][0] = 0xffff_ffff_ffff_fffb;
    c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
    c.g[T0][0] = 0xffff_ffff_fff8_0000;
    c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
    c.g[T0][0] = c.g[T0][0] | 0x1fff;
    c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
    c.g[A3][0] = 0x10000;
    c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
    c.g[V1][0] = 0x490000;
    c.g[V0][0] = c.g[V0][0] | 0x4a;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[A3][0] = c.g[A3][0] | 0x2000;
    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4bd8));
    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
    c.sw(this.wrapping_add(0x8), c.g[V1][0]);
    c.g[T0][0] = 0x4;
    c.sw(this.wrapping_add(0x14), c.g[V0][0]);
    c.g[V1][0] = 0x500000;
    c.sw(this.wrapping_add(0x4), c.g[A0][0]);
    c.g[A3][0] = 0x6;
    c.sw(this, c.g[A0][0]);
    c.g[V0][0] = c.g[A0][0];
    c.sw(this.wrapping_add(0xc), 0);
    c.sb(this.wrapping_add(0x10), 0);
    c.sw(this.wrapping_add(0x18), 0);
    c.swc1(0, this.wrapping_add(0x1c));
    c.swc1(0, this.wrapping_add(0x20));
    c.swc1(0, this.wrapping_add(0x24));
    c.swc1(0, this.wrapping_add(0x28));
    // Zero vector at +0x2c (12 bytes, copied through the 8-byte loads).
    c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf28));
    c.ldl(9, (c.g[T4][0] as u32).wrapping_add(0x7));
    c.ldr(9, c.g[T4][0] as u32);
    c.g[T2][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x8));
    c.sdl(9, this.wrapping_add(0x33));
    c.sdr(9, this.wrapping_add(0x2c));
    c.sw(this.wrapping_add(0x34), c.g[T2][0]);
    c.sw(this.wrapping_add(0x40), c.g[A2][0]);
    c.sw(this.wrapping_add(0x38), 0);
    c.sw(this.wrapping_add(0x3c), 0);
    // Zero vector at +0x44.
    c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf28));
    c.ldl(6, (c.g[T3][0] as u32).wrapping_add(0x7));
    c.ldr(6, c.g[T3][0] as u32);
    c.g[T1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
    c.sdl(6, this.wrapping_add(0x4b));
    c.sdr(6, this.wrapping_add(0x44));
    c.sw(this.wrapping_add(0x4c), c.g[T1][0]);
    c.swc1(0, this.wrapping_add(0x58));
    c.sw(this.wrapping_add(0x5c), c.g[A1][0]);
    c.swc1(0, this.wrapping_add(0x50));
    c.swc1(0, this.wrapping_add(0x54));
    // Zero vector at +0x60.
    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xf28));
    c.ldl(12, (c.g[T1][0] as u32).wrapping_add(0x7));
    c.ldr(12, c.g[T1][0] as u32);
    c.g[A1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
    c.sdl(12, this.wrapping_add(0x67));
    c.sdr(12, this.wrapping_add(0x60));
    c.sw(this.wrapping_add(0x68), c.g[A1][0]);
    c.sw(this.wrapping_add(0x6c), c.g[T0][0]);
    c.sw(this.wrapping_add(0x70), c.g[A3][0]);
    ret(c);
}

/// `int IniParser_Expect(IniParser* self, const char* text)` (0x2a7ef0):
/// skip whitespace, then compare the input with `text` one character at a
/// time (the input is read through `GetChar`, which advances the cursor
/// stored at `self+8`). A mismatch returns 1. Otherwise the result is the
/// character of `text` where the comparison stopped: 0 when the whole text
/// matched or the input ended, as the reference body `return name[i]` does.
/// The loop checks events at its back edge, as the generated code does.
pub fn ini_parser_expect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 19, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        INI_SKIP_WHITESPACE.call(c, 0x002a_7f18)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(8).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
        let head_end = c.g[V0][0] == 0;
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
        let name_end = if head_end {
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S3][0] as u32).lb(c);
            c.g[S0][0] = c.g[S3][0];
            c.g[V0][0] == 0
        };
        let mut mismatch = false;
        if !name_end {
            c.cycles += 1;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                INI_GET_CHAR.call(c, 0x002a_7f40)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lb(c);
                mismatch = c.g[V0][0] != c.g[V1][0];
                c.g[V0][0] = 1;
                if mismatch {
                    break;
                }
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(8).lw(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
                let input_end = c.g[V0][0] == 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                if !input_end {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                    let more = c.g[V1][0] != 0;
                    c.g[S0][0] = c.g[V0][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002a_7f38);
                        }
                        continue;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                break;
            }
        }
        if !mismatch {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    #[test]
    fn ui_object_ctor_matches() {
        check(
            ee::s13::f_0039fb30__generated,
            ui_object_ctor,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = Rng(0x0039_fb30 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                c.bus.write32(DATA + 0x14, r.next_u32());
            },
        );
    }

    /// `IniParser_Expect`: the parser's cursor (self+8) points at input text,
    /// the name is at DATA+0x200. The name is 0-3 characters from a small
    /// alphabet; on most seeds the input starts with the name (the match
    /// path), on some it differs (the mismatch path), and on some the input
    /// is empty (the head-exit path). Real `IniParser_GetChar` and
    /// `IniParser_SkipWhitespace` run as they are.
    #[test]
    fn ini_parser_expect_matches() {
        check(
            ee::s08::f_002a7ef0__generated,
            ini_parser_expect,
            200,
            &|c: &mut Cpu, seed: u32| {
                let mut r = Rng(0x002a_7ef0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                const ALPHA: [u8; 4] = [b'a', b'b', b' ', b'c'];
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 8, DATA + 0x100);
                let name_len = r.next_u32() % 4;
                for i in 0..name_len {
                    let ch = ALPHA[(r.next_u32() % 4) as usize];
                    c.bus.write8(DATA + 0x200 + i, ch);
                }
                c.bus.write8(DATA + 0x200 + name_len, 0);
                let text_len = if seed % 5 == 0 {
                    0
                } else {
                    name_len + r.next_u32() % 3
                };
                for i in 0..text_len {
                    let ch = if seed % 3 != 0 && i < name_len {
                        c.bus.read8(DATA + 0x200 + i)
                    } else {
                        ALPHA[(r.next_u32() % 4) as usize]
                    };
                    c.bus.write8(DATA + 0x100 + i, ch);
                }
                c.bus.write8(DATA + 0x100 + text_len, 0);
            },
        );
    }
}
