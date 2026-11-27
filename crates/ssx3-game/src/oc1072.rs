//! Staging module oc1072 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1072`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0038_d660,
        ps2_bezier_man_free_caches_stub,
        "PS2BezierMan_FreeCachesStub",
    ),
    (
        0x0038_f820,
        tex_man_swizzle_t4_block,
        "TexMan_SwizzleT4Block",
    ),
    (0x0039_1508, font_style_ctor, "FontStyle_ctor"),
    (0x0039_2de8, fx_trail_set_flag_a4, "FXTrail_SetFlagA4"),
    (
        0x0039_6160,
        vfx_component_base_dtor,
        "VfxComponent_BaseDtor",
    ),
    (
        0x0039_84e8,
        ui_font_set_parse_font_pack,
        "UIFontSet_ParseFontPack",
    ),
    (0x0039_8a60, ui_icon_swap_words, "UIIcon_SwapWords"),
];

/// `void PS2BezierMan_FreeCachesStub(void)` (0x38d660): the caches are not
/// kept any more, so there is nothing to free; it only returns.
pub fn ps2_bezier_man_free_caches_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `void FXTrail_SetFlagA4(FXTrail* self, u32 flag)` (0x392de8): store the
/// flag word at `+0xa4` of the trail.
pub fn fx_trail_set_flag_a4(c: &mut Cpu) {
    c.cycles += 2;
    let flag = c.g[A1][0];
    Addr(abi::arg(c, 0).wrapping_add(0xa4)).sw(c, flag);
    abi::ret(c);
}

/// `void UIIcon_SwapWords(u32* a, u32* b)` (0x398a60): exchange the two
/// words (the values go through the FPU registers f0/f1, as compiled).
pub fn ui_icon_swap_words(c: &mut Cpu) {
    c.cycles += 5;
    let a = Addr(abi::arg(c, 0));
    let b = Addr(abi::arg(c, 1));
    c.f[0] = b.read_u32(c);
    c.f[1] = a.read_u32(c);
    let (v0, v1) = (c.f[0], c.f[1]);
    a.write_u32(c, v0);
    b.write_u32(c, v1);
    abi::ret(c);
}

/// `VfxComponent` base destructor (0x396160): set the base vtable (0x488680)
/// at the object, then free the object when bit 0 of the flags (`a1`) is set
/// (`operator_delete`, 0x317e50).
pub fn vfx_component_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x7980));
        c.g[A1][0] = c.g[A1][0] & 0x1;
        let obj = Addr(abi::arg(c, 0));
        let vtbl = c.g[V0][0];
        obj.sw(c, vtbl);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0039_6184)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void UIFontSet_ParseFontPack(UIFontSet* self, u32* pack)` (0x3984e8):
/// keep the pack pointer in `*self`, then add each font entry of the pack
/// (`pack[0]` entries, each `8 + step` bytes apart) through
/// `UIFontSet_AddFont` (0x3984b0).
pub fn ui_font_set_parse_font_pack(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
        frame.save_ra(c, 0);
        let set = c.g[S2][0] as u32;
        let pack = c.g[A1][0];
        Addr(set).sw(c, pack);
        c.g[V0][0] = Addr(pack as u32).lw(c);
        c.g[S1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            loop {
                c.cycles += 13;
                let entry = c.g[S0][0] as u32;
                c.g[A3][0] = Addr(entry).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = sx(entry.wrapping_add(0x8));
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039_84b0, 0x0039_852c)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = Addr(set).lw(c);
                c.g[V1][0] = Addr(entry.wrapping_add(0x4)).lw(c);
                let first = c.g[V0][0] as u32;
                c.g[A0][0] = Addr(first).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = (c.g[S1][0] < c.g[A0][0]) as u64;
                let more = c.g[A0][0] != 0;
                c.g[S0][0] = sx(entry.wrapping_add(c.g[V1][0] as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_8518);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FontStyle_ctor(FontStyle* self)` (0x391508): reset the style block
/// at `self` (0x68 bytes): the float 1.0 (through f0) and the zero fields,
/// copied through the stack with the unaligned `ldl/ldr/sdl/sdr` pairs.
/// The `0xffffffff` word at `+0x60` is the "no font" marker.
pub fn font_style_ctor(c: &mut Cpu) {
    c.cycles += 55;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    let stk = sp(c);
    let obj = abi::arg(c, 0);
    c.sw(stk, 0);
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.sw(stk.wrapping_add(0x4), 0);
    c.g[V0][0] = c.g[A0][0];
    c.ldl(A1, stk.wrapping_add(0x7));
    c.ldr(A1, stk);
    c.sdl(A1, obj.wrapping_add(0x2f));
    c.sdr(A1, obj.wrapping_add(0x28));
    c.swc1(0, stk);
    c.swc1(0, stk.wrapping_add(0x4));
    c.ldl(A1, stk.wrapping_add(0x7));
    c.ldr(A1, stk);
    c.sdl(A1, obj.wrapping_add(0x37));
    c.sdr(A1, obj.wrapping_add(0x30));
    c.swc1(0, stk.wrapping_add(0x10));
    c.swc1(0, stk.wrapping_add(0x14));
    c.swc1(0, stk.wrapping_add(0x18));
    c.swc1(0, stk.wrapping_add(0x1c));
    c.ldl(A1, obj.wrapping_add(0x37));
    c.ldr(A1, obj.wrapping_add(0x30));
    c.sdl(A1, obj.wrapping_add(0x3f));
    c.sdr(A1, obj.wrapping_add(0x38));
    c.ldl(A1, stk.wrapping_add(0x17));
    c.ldr(A1, stk.wrapping_add(0x10));
    c.ldl(A2, stk.wrapping_add(0x1f));
    c.ldr(A2, stk.wrapping_add(0x18));
    c.sdl(A1, obj.wrapping_add(0x47));
    c.sdr(A1, obj.wrapping_add(0x40));
    c.sdl(A2, obj.wrapping_add(0x4f));
    c.sdr(A2, obj.wrapping_add(0x48));
    c.swc1(0, stk);
    c.sw(stk.wrapping_add(0x4), 0);
    c.sw(stk.wrapping_add(0x8), 0);
    c.sw(stk.wrapping_add(0xc), 0);
    c.ldl(A1, stk.wrapping_add(0x7));
    c.ldr(A1, stk);
    c.ldl(A2, stk.wrapping_add(0xf));
    c.ldr(A2, stk.wrapping_add(0x8));
    c.sdl(A1, obj.wrapping_add(0x57));
    c.sdr(A1, obj.wrapping_add(0x50));
    c.sdl(A2, obj.wrapping_add(0x5f));
    c.sdr(A2, obj.wrapping_add(0x58));
    let marker = c.g[V1][0];
    c.sw(obj.wrapping_add(0x60), marker);
    c.sw(obj, 0);
    c.sw(obj.wrapping_add(0x4), 0);
    c.sw(obj.wrapping_add(0x8), 0);
    c.sw(obj.wrapping_add(0x14), 0);
    c.sw(obj.wrapping_add(0xc), 0);
    c.sw(obj.wrapping_add(0x10), 0);
    c.sb(obj.wrapping_add(0x64), 0);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    c.pc = t;
}

/// `void TexMan_SwizzleT4Block(void* self, u8* src, u8* dst)` (0x38f820): unpack
/// a 4-bit (T4) texture block from `src` into `dst`, two texels per byte, with
/// the 32-bit words of the swizzle table at `0x44b9e0` (the game image's
/// `D_0044B9E0`) selecting each texel's position.
pub fn tex_man_swizzle_t4_block(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    c.g[T3][0] = c.g[A1][0];
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A2][0];
    frame.save(c, S2, 0x0);
    c.g[T5][0] = 0;
    c.g[S2][0] = sx(0x0045_0000u32.wrapping_sub(0x4620));
    c.g[V1][0] = 0;
    c.g[T7][0] = 0xf;
    c.g[S1][0] = 0xffff_ffff_ffff_fff0;
    loop {
        c.cycles += 6;
        c.g[V0][0] = c.g[V1][0] & 0x1;
        c.g[T9][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[T2][0] = sx((c.g[V0][0] as u32) << 7);
        c.g[V1][0] = 0;
        c.g[T8][0] = sx((c.g[T3][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = sx((c.g[T2][0] as u32) << 2);
        loop {
            c.cycles += 4;
            c.g[T6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[T4][0] = 0;
            loop {
                c.cycles += 31;
                c.g[A0][0] = Addr(c.g[T1][0] as u32).lw(c);
                c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[T5][0] as u32));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A0][0] as u32) >> 1);
                c.g[A3][0] = c.g[A0][0] & 0x1;
                c.g[A0][0] = Addr(c.g[T1][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[A0][0] as u32) >> 1);
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.g[A0][0] & 0x1;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A2][0] = sx((c.g[T7][0] as u32) << ((c.g[A0][0] as u32) & 31));
                c.g[V1][0] = sx((c.g[T7][0] as u32) << ((c.g[A3][0] as u32) & 31));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[A0][0] as u32) & 31)) as u32);
                c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> ((c.g[A3][0] as u32) & 31)) as u32);
                c.g[V0][0] = c.g[V0][0] & 0xff;
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
                c.g[V1][0] = (c.g[T4][0] < 0x4u64) as u64;
                c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
                let more = c.g[V1][0] != 0;
                let texel = c.g[A1][0];
                Addr(c.g[T0][0] as u32).sb(c, texel);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0038_f878);
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[T6][0];
            c.g[V0][0] = (c.g[V1][0] < 0x10u64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[T2][0] as u32) << 2);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0038_f868);
            }
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[T9][0];
        c.g[V0][0] = (c.g[V1][0] < 0x4u64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[T3][0] = c.g[T8][0];
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0038_f850);
        }
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x20);
    c.g[V0][0] = 0;
    frame.restore(c, S1, 0x10);
    frame.restore(c, S2, 0x0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and log cell of the operator_delete fake (unique to this module).
    const OC1072_GATE: u32 = DATA + 0x4000;
    const OC1072_LOG: u32 = DATA + 0x4004;
    const OC1072_MAGIC: u32 = 0x1072_0396;
    static OC1072_DELETE_PREV: AtomicUsize = AtomicUsize::new(0);

    /// Gated stand-in for `operator_delete` (0x317e50): when the gate is
    /// set it logs its argument and returns; otherwise the real body runs.
    fn oc1072_delete_fake(c: &mut Cpu) {
        if c.bus.read32(OC1072_GATE) != OC1072_MAGIC {
            return fakes::fall_back(c, &OC1072_DELETE_PREV, ee::s10::f_00317e50__generated);
        }
        let p = c.g[A0][0] as u32;
        c.bus.write32(OC1072_LOG, p);
        abi::ret(c);
    }

    #[test]
    fn ps2_bezier_man_free_caches_stub_matches() {
        check(
            ssx3_ee::s12::f_0038d660__generated,
            ps2_bezier_man_free_caches_stub,
            200,
            &|c, seed| {
                let mut r = Rng(0x38d6_6000 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn fx_trail_set_flag_a4_matches() {
        check(
            ssx3_ee::s13::f_00392de8__generated,
            fx_trail_set_flag_a4,
            200,
            &|c, seed| {
                let mut r = Rng(0x392d_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(r.next_u32());
                c.bus.write32(DATA + 0xa4, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_icon_swap_words_matches() {
        check(
            ssx3_ee::s13::f_00398a60__generated,
            ui_icon_swap_words,
            200,
            &|c, seed| {
                let mut r = Rng(0x398a_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // Both distinct and aliased pointers.
                let b = if seed % 5 == 0 { DATA } else { DATA + 0x40 };
                c.g[A1][0] = sx(b);
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 0x40, r.next_u32());
            },
        );
    }

    #[test]
    fn vfx_component_base_dtor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1072_delete_fake,
            &OC1072_DELETE_PREV,
        );
        check(
            ssx3_ee::s13::f_00396160__generated,
            vfx_component_base_dtor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3961_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                // Bit 0 of the flags picks the delete path, the rest is junk.
                let flags = (r.next_u32() & !1) | (seed & 1);
                c.g[A1][0] = sx(flags);
                c.bus.write32(OC1072_GATE, OC1072_MAGIC);
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn ui_font_set_parse_font_pack_matches() {
        check(
            ssx3_ee::s13::f_003984e8__generated,
            ui_font_set_parse_font_pack,
            200,
            &|c, seed| {
                let mut r = Rng(0x3984_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                // The font set's style byte: below 4 adds an entry, else not.
                c.bus.write8(DATA + 0x34, (seed % 6) as u8);
                // Pack: entry count, then entries of (word, step) with the
                // next entry `step + 8` bytes on.
                let n = seed % 4;
                c.bus.write32(DATA + 0x200, n);
                let mut e = DATA + 0x204;
                for _ in 0..4 {
                    let step = r.next_u32() % 0x10;
                    c.bus.write32(e, r.next_u32());
                    c.bus.write32(e + 4, step);
                    e += step + 8;
                }
            },
        );
    }

    #[test]
    fn font_style_ctor_matches() {
        check(
            ssx3_ee::s12::f_00391508__generated,
            font_style_ctor,
            200,
            &|c, seed| {
                let mut r = Rng(0x3915_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA + 0x100 * (seed % 3));
                for i in 0..0x30 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn tex_man_swizzle_t4_block_matches() {
        check(
            ssx3_ee::s12::f_0038f820__generated,
            tex_man_swizzle_t4_block,
            120,
            &|c, seed| {
                let mut r = Rng(0x38f8_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x800);
                // Swizzle table in the game image (small words: the indices
                // stay inside the source block).
                for i in 0..0x400 {
                    c.bus.write32(0x0044_b9e0 + i * 4, r.next_u32() % 0x4000);
                }
                for i in 0..0x400 {
                    c.bus.write8(DATA + 0x400 + i, r.next_u32() as u8);
                }
            },
        );
    }
}
