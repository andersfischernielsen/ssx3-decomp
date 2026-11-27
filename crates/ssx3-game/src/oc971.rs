//! Staging module oc971 (wave wave258): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc971`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001c_a6a0,
    oc971_check_and_fix_text_entry,
    "cFEPopupConfirm_CheckAndFixTextEntry",
)];

/// `void cFEPopupConfirm_CheckAndFixTextEntry(self, text, str)` (0x1ca6a0):
/// make a text entry fit its widget. Measures `str` and an ellipsis string with
/// the text object's font scale (font `+0x30/+0x34` times the text's scale at
/// `+0x50`, stored back at font `+0x38/+0x3c`); while the width left by the
/// ellipsis (`size.x - dots - 1.0`, `size` from the text vtable's `0xa0`
/// method) is smaller than the string, it cuts one character off a private
/// copy (`self+0x328` bytes + 4). If anything was cut, it appends the ellipsis
/// and sets the copy as the text, else it sets the original; then frees the copy.
pub fn oc971_check_and_fix_text_entry(c: &mut Cpu) {
    guest(c, |c| {
        'b_1ca838: {
            c.cycles += 80;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A2][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.g[S5][0] = 0u64;
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x57));
            c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[SP][0] as u32);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001ca704);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001ca704)?;
            c.g[RA][0] = sx(0x001ca70c);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a04f0, 0x001ca70c)?;
            c.lwc1(2, c.g[SP][0] as u32);
            c.g[A1][0] = 0x460000;
            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A2][0] = 0x100;
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.g[A3][0] = 0u64;
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x61a0));
            c.mul_s(1, 1, 2);
            c.mul_s(0, 0, 3);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x38));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3c));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x328));
            c.g[RA][0] = sx(0x001ca748);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x00317e30, 0x001ca748)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001ca754);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x00416810, 0x001ca754)?;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001ca764);
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            call_ptr(c, 0x004166f4, 0x001ca764)?;
            c.g[V1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001ca774);
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1bd0));
            call_ptr(c, 0x003a04f0, 0x001ca774)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.g[A1][0] = c.g[S4][0];
            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
            c.g[RA][0] = sx(0x001ca790);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00391fb0, 0x001ca790)?;
            c.g[RA][0] = sx(0x001ca798);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a04f0, 0x001ca798)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
            c.g[RA][0] = sx(0x001ca7b4);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00391fb0, 0x001ca7b4)?;
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 3);
            c.sub_s(0, 0, 1);
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.sub_s(0, 0, 3);
            c.c_lt_s(0, 2);
            let t = !c.fcc();
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            if t {
                break 'b_1ca838;
            } else {
                c.cycles += 2;
                c.mov_s(20, 3);
                c.g[S5][0] = 0x1;
                'l_1ca7e8: loop {
                    c.cycles += 20;
                    c.g[A0][0] = c.g[S1][0];
                    c.sb(c.g[V0][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x001ca7f8);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x003a04f0, 0x001ca7f8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                    c.g[RA][0] = sx(0x001ca814);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00391fb0, 0x001ca814)?;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                    c.sub_s(1, 1, 0);
                    c.sub_s(1, 1, 20);
                    c.c_lt_s(1, 2);
                    let t = c.fcc();
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001ca7e8);
                        }
                        continue 'l_1ca7e8;
                    } else {
                        break 'b_1ca838;
                    }
                }
            }
        }
        'b_1ca868: {
            'b_1ca85c: {
                c.cycles += 2;
                let t = c.g[S5][0] == 0u64;
                c.g[A1][0] = c.g[S4][0];
                if t {
                    break 'b_1ca85c;
                } else {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001ca848);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x004162d0, 0x001ca848)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001ca854);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0c50, 0x001ca854)?;
                    let t = true;

                    if t {
                        break 'b_1ca868;
                    } else {
                        break 'b_1ca85c;
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001ca868);
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x003a0c50, 0x001ca868)?;
            break 'b_1ca868;
        }
        'b_1ca87c: {
            c.cycles += 2;
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_1ca87c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001ca878);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e98, 0x001ca878)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_1ca87c;
            }
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered};
    use crate::difftest::{elf_image, fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and magic of this module's fakes (own words, so other tests
    /// that reach the same slots fall through to the real bodies).
    const OC971_CTRL: u32 = DATA + 0xe_7100;
    const OC971_MAGIC: u32 = 0x0971_f00d;
    const OC971_SELF: u32 = DATA + 0x9_0000;
    const OC971_TEXT: u32 = DATA + 0x9_1000;
    const OC971_P1: u32 = DATA + 0x9_2000;
    const OC971_Q: u32 = DATA + 0x9_2100;
    const OC971_A1: u32 = DATA + 0x9_2200;
    const OC971_TBL: u32 = DATA + 0x9_2300;
    const OC971_FONT: u32 = DATA + 0x9_2400;
    const OC971_VT: u32 = DATA + 0x9_3000;
    const OC971_STR: u32 = DATA + 0x9_4000;
    /// The ellipsis string `D_004A1BD0` in the game image.
    const OC971_DOTS: u32 = 0x004a_1bd0;
    /// Width of one character in the fake `Font_MeasureText`.
    const OC971_CHAR_W: f32 = 4.0;

    fn oc971_strlen(c: &mut Cpu, mut p: u32) -> u32 {
        let mut n = 0;
        while c.bus.read8(p) != 0 && n < 4096 {
            p += 1;
            n += 1;
        }
        n
    }

    /// Logged `Font_MeasureText(font, str, out, 0)`: writes the measured width
    /// (characters times a fixed width) to `out[2]` like the real one does.
    fn oc971_fake_measure(c: &mut Cpu) {
        if c.bus.read32(OC971_CTRL) != OC971_MAGIC {
            return fakes::fall_back(c, &OC971_PREV_MEASURE, ee::s12::f_00391fb0__generated);
        }
        let ra = c.g[RA][0] as u32;
        let s = c.g[A1][0] as u32;
        let out = c.g[A2][0] as u32;
        fakes::fake(c, 0x0039_1fb0, ra);
        if c.pc != ra {
            return;
        }
        let w = oc971_strlen(c, s) as f32 * OC971_CHAR_W;
        c.bus.write32(out + 8, w.to_bits());
    }
    static OC971_PREV_MEASURE: AtomicUsize = AtomicUsize::new(0);

    /// Logged `UIText_SetString(text, str)`.
    fn oc971_fake_set_string(c: &mut Cpu) {
        if c.bus.read32(OC971_CTRL) != OC971_MAGIC {
            return fakes::fall_back(c, &OC971_PREV_SET_STRING, ee::s13::f_003a0c50__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003a_0c50, ra);
    }
    static OC971_PREV_SET_STRING: AtomicUsize = AtomicUsize::new(0);

    fn oc971_world(c: &mut Cpu, seed: u32) {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391FB0),
            oc971_fake_measure,
            &OC971_PREV_MEASURE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc971_fake_set_string,
            &OC971_PREV_SET_STRING,
        );
        let mut r: Rng = fakes::machine_rt(c, seed, 0x1ca6a0, 4);
        elf_image(c);
        c.bus.write32(OC971_CTRL, OC971_MAGIC);
        fill(c, &mut r, OC971_SELF, 0x340);
        fill(c, &mut r, OC971_TEXT, 0x80);
        c.bus.write32(OC971_SELF + 0x328, 64);
        // text -> +0x5c -> +0xd0 -> +0x10 -> +8 -> table[(text+0x74 & 3) * 12 + 0xc] = font
        c.bus.write32(OC971_TEXT + 0x8, OC971_VT);
        c.bus.write32(OC971_TEXT + 0x50, r.f32(3.0));
        c.bus.write32(OC971_TEXT + 0x54, r.f32(3.0));
        c.bus.write32(OC971_TEXT + 0x58, r.f32(3.0));
        c.bus.write32(OC971_TEXT + 0x5c, OC971_P1);
        c.bus.write32(OC971_P1 + 0xd0, OC971_Q);
        c.bus.write32(OC971_Q + 0x10, OC971_A1);
        c.bus.write32(OC971_A1 + 8, OC971_TBL);
        for k in 0..4 {
            c.bus.write32(OC971_TBL + 12 * k + 0xc, OC971_FONT);
        }
        fill(c, &mut r, OC971_FONT, 0x40);
        // The vtable method at 0xa0 is a logged fake (the size is on the stack).
        c.bus.write16(OC971_VT + 0xa0, (r.next_u32() & 0x3c) as u16);
        c.bus.write32(OC971_VT + 0xa4, fakes::FAKE);
        // The string: seed picks length 0, short (fits), or long (gets cut).
        let len = match seed % 6 {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 12,
            4 => 30,
            _ => 3 + r.next_u32() % 50,
        };
        for i in 0..len {
            c.bus.write8(OC971_STR + i, b'a' + (r.next_u32() % 26) as u8);
        }
        c.bus.write8(OC971_STR + len, 0);
        // `size.x` (the callee's out slot at sp-0xc0+0x10): the ellipsis width
        // plus at least 1.0, plus room for 0..15 characters.
        let dots = oc971_strlen(c, OC971_DOTS) as f32 * OC971_CHAR_W;
        let room = (dots + 1.0) + OC971_CHAR_W * (r.next_u32() % 16) as f32 + (r.next_u32() % 4) as f32;
        let sp0 = c.g[SP][0] as u32;
        c.bus.write32(sp0.wrapping_sub(0xc0) + 0x10, room.to_bits());
        c.g[A0][0] = OC971_SELF as u64;
        c.g[A1][0] = OC971_TEXT as u64;
        c.g[A2][0] = OC971_STR as u64;
    }

    /// `CheckAndFixTextEntry`: the string fits (set as is), is cut and gets the
    /// ellipsis, or is empty; also unwinds out of any logged call. Callees:
    /// real allocator, `strlen`/`strcpy`/`strcat`, `UIText_GetFont`; faked
    /// (gated): the text vtable's size method, `Font_MeasureText`,
    /// `UIText_SetString`.
    #[test]
    fn oc971_check_and_fix_text_entry_matches() {
        covered(
            ee::s03::f_001ca6a0__generated,
            oc971_check_and_fix_text_entry,
            48,
            &oc971_world,
        );
    }

    /// Both arms really run: the loop's measure call (return `0x1ca814`) and
    /// both `UIText_SetString` calls (`0x1ca854` cut copy, `0x1ca868` original).
    #[test]
    fn oc971_check_and_fix_text_entry_paths() {
        fakes::check_reaching(
            ee::s03::f_001ca6a0__generated,
            oc971_check_and_fix_text_entry,
            48,
            &[0x001c_a814, 0x001c_a854, 0x001c_a868],
            &oc971_world,
        );
    }
}
