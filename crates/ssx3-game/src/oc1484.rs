//! Staging module oc1484 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1484`.

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
    (0x0037_d348, ps2_graphics_man_set_render_target_texture, "SetRenderTargetTexture"),
    (0x0037_d800, ps2_graphics_man_fill_render_target, "FillRenderTarget"),
    (0x0039_6b40, ps2_graphics_man_emit_lighting_packet, "PS2GraphicsMan_EmitLightingPacket"),
    (0x002b_db38, twpin_fog_is_equal, "tWPInGameData_Fog_isEqual"),
    (0x002b_dc78, twpin_screen_tint_is_equal, "tWPInGameData_ScreenTint_isEqual"),
    (0x002b_dd38, twpin_sun_is_equal, "tWPInGameData_Sun_isEqual"),
    (0x002b_dee0, twpin_weather_is_equal, "tWPInGameData_Weather_isEqual"),
];

/// Effective address `base + off` for a 64-bit register base (32-bit wrap).
#[inline]
fn render_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// One compared field of a record, in the order the generated `isEqual`
/// bodies make their compares. The cycles are charged before the loads.
#[derive(Clone, Copy)]
enum WpinField {
    /// Float field: `lwc1` from `a + oa` (f1) and `b + ob` (f0), `c.eq.s`.
    Float(u64, u32, u32),
    /// Word field: `lw` from `a + oa` (v1) and `b + ob` (v0), then `!=`.
    Word(u64, u32, u32),
}

/// Compares two records field by field, stopping at the first difference,
/// and returns from the caller with v0 = 1 when all fields match, else 0.
/// Charges the two cycles of the exit block after the last compare.
fn wpin_record_equal(c: &mut Cpu, a: u32, b: u32, fields: &[WpinField]) {
    let mut equal = true;
    for &field in fields {
        match field {
            WpinField::Float(charge, oa, ob) => {
                c.cycles += charge;
                let x = Addr(a.wrapping_add(oa)).read_fpu(c);
                c.f[1] = x.0;
                let y = Addr(b.wrapping_add(ob)).read_fpu(c);
                c.f[0] = y.0;
                equal = x.eq_s(y);
                crate::abi::set_fcc(c, equal);
            }
            WpinField::Word(charge, oa, ob) => {
                c.cycles += charge;
                c.g[V1][0] = Addr(a.wrapping_add(oa)).lw(c);
                c.g[V0][0] = Addr(b.wrapping_add(ob)).lw(c);
                equal = c.g[V1][0] == c.g[V0][0];
            }
        }
        if !equal {
            break;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = equal as u64;
    ret(c);
}

/// Fields of `tWPInGameData_Fog` (0x2bdb38): density, near, far, the three
/// colour components.
const FOG_FIELDS: [WpinField; 6] = [
    WpinField::Float(6, 0x08, 0x04),
    WpinField::Float(4, 0x10, 0x08),
    WpinField::Float(5, 0x18, 0x0c),
    WpinField::Float(5, 0x20, 0x10),
    WpinField::Float(6, 0x28, 0x14),
    WpinField::Float(5, 0x30, 0x18),
];

/// `bool tWPInGameData_Fog_isEqual(const Fog* a, const Fog* b)` (0x2bdb38):
/// true when the six float fields of the two records are equal.
pub fn twpin_fog_is_equal(c: &mut Cpu) {
    let (a, b) = (crate::abi::arg(c, 0), crate::abi::arg(c, 1));
    wpin_record_equal(c, a, b, &FOG_FIELDS);
}

/// `bool tWPInGameData_ScreenTint_isEqual(const ScreenTint* a, ...)`
/// (0x2bdc78): the same six-float compare as the fog record, at the screen
/// tint scale/offset fields (identical generated body).
pub fn twpin_screen_tint_is_equal(c: &mut Cpu) {
    let (a, b) = (crate::abi::arg(c, 0), crate::abi::arg(c, 1));
    wpin_record_equal(c, a, b, &FOG_FIELDS);
}

/// Fields of `tWPInGameData_Sun` (0x2bdd38): pitch, yaw, three colour
/// floats, intensity, then the integer sun type, lens intensity and radius.
const SUN_FIELDS: [WpinField; 9] = [
    WpinField::Float(6, 0x08, 0x04),
    WpinField::Float(4, 0x10, 0x08),
    WpinField::Float(5, 0x18, 0x0c),
    WpinField::Float(5, 0x20, 0x10),
    WpinField::Float(5, 0x28, 0x14),
    WpinField::Float(5, 0x30, 0x18),
    WpinField::Word(4, 0x38, 0x1c),
    WpinField::Float(6, 0x40, 0x20),
    WpinField::Float(5, 0x48, 0x24),
];

/// `bool tWPInGameData_Sun_isEqual(const Sun* a, const Sun* b)` (0x2bdd38):
/// the float fields (the sun type is an integer compare) of two sun records.
pub fn twpin_sun_is_equal(c: &mut Cpu) {
    let (a, b) = (crate::abi::arg(c, 0), crate::abi::arg(c, 1));
    wpin_record_equal(c, a, b, &SUN_FIELDS);
}

/// Fields of `tWPInGameData_Weather` (0x2bdee0): snow, flake size, wind,
/// temperature, flurries, snow fluff and flake/fluff colours.
const WEATHER_FIELDS: [WpinField; 19] = [
    WpinField::Float(6, 0x08, 0x04),
    WpinField::Float(4, 0x10, 0x08),
    WpinField::Float(5, 0x18, 0x0c),
    WpinField::Float(5, 0x20, 0x10),
    WpinField::Float(5, 0x28, 0x14),
    WpinField::Float(5, 0x30, 0x18),
    WpinField::Float(5, 0x38, 0x1c),
    WpinField::Float(5, 0x40, 0x20),
    WpinField::Float(5, 0x48, 0x24),
    WpinField::Float(5, 0x50, 0x28),
    WpinField::Float(5, 0x58, 0x2c),
    WpinField::Float(5, 0x60, 0x30),
    WpinField::Float(5, 0x68, 0x34),
    WpinField::Float(5, 0x70, 0x38),
    WpinField::Float(5, 0x78, 0x3c),
    WpinField::Float(5, 0x80, 0x40),
    WpinField::Float(5, 0x88, 0x44),
    WpinField::Float(6, 0x90, 0x48),
    WpinField::Float(5, 0x98, 0x4c),
];

/// `bool tWPInGameData_Weather_isEqual(const Weather* a, const Weather* b)`
/// (0x2bdee0): the nineteen float fields of two weather records.
pub fn twpin_weather_is_equal(c: &mut Cpu) {
    let (a, b) = (crate::abi::arg(c, 0), crate::abi::arg(c, 1));
    wpin_record_equal(c, a, b, &WEATHER_FIELDS);
}

/// `EmitLightingPacket(PS2GraphicsMan* gm)` (0x396b40, the type-6 lighting
/// packet): leaf. Writes header words at the render-state record (`gm+0xe84`),
/// copies ten 16-byte blocks from `gm+0x6bb0` to the list at `gm+0x5a00 |
/// 0x3000_0000`, writes the list tail, and stores the new list pointer back
/// at `gm+0x5a00`. Mechanical port of the generated body.
pub fn ps2_graphics_man_emit_lighting_packet(c: &mut Cpu) {
    c.cycles += 38;
    c.g[T5][0] = c.g[A0][0];
    c.g[A1][0] = !0x3c0; // 0xffff_ffff_ffff_fc3f
    c.g[V1][0] = Addr(render_ea(c.g[T5][0], 0xe84)).lw(c);
    c.g[A0][0] = 0x3fff_0000;
    c.g[A0][0] |= 0xffff;
    c.g[A2][0] = !0x3fff_ffff; // 0xffff_ffff_c000_0000
    let rs = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(rs).lw(c);
    c.g[T0][0] = 0x3000_0000;
    c.g[T1][0] = 0x6000_0000;
    c.g[T1][0] |= 0xc;
    c.g[T2][0] = 0x8000;
    c.g[T2][0] <<= 45;
    c.g[V0][0] &= c.g[A1][0];
    // A3 = 0x6c0a_0007_0100_0101 built from halves.
    c.g[A3][0] = 0x6c0a_0000;
    c.g[A3][0] |= 0x7;
    c.g[A3][0] <<= 16;
    c.g[A3][0] |= 0x100;
    c.g[A3][0] <<= 16;
    c.g[A3][0] |= 0x101;
    c.g[V0][0] |= 0x180;
    c.g[T3][0] = sx(render_ea(c.g[T5][0], 0x6bb0));
    let v0 = c.g[V0][0];
    Addr(rs).sw(c, v0);
    c.g[T4][0] = 0x9;
    c.g[A1][0] = Addr(render_ea(c.g[T5][0], 0xe84)).lw(c);
    c.g[V1][0] = Addr(render_ea(c.g[T5][0], 0x5a00)).lw(c);
    c.g[V0][0] = Addr(render_ea(c.g[A1][0], 0xc)).lw(c);
    c.g[V1][0] &= c.g[A0][0];
    c.g[V0][0] &= c.g[A2][0];
    c.g[V0][0] |= c.g[V1][0];
    let v0 = c.g[V0][0];
    Addr(render_ea(c.g[A1][0], 0xc)).sw(c, v0);
    c.g[V1][0] = Addr(render_ea(c.g[T5][0], 0x5a00)).lw(c);
    c.g[A1][0] = c.g[V1][0] | c.g[T0][0];
    let a1 = render_ea(c.g[A1][0], 0);
    let t1 = c.g[T1][0];
    Addr(a1).sd(c, t1);
    let t2 = c.g[T2][0];
    Addr(a1.wrapping_add(0x10)).sd(c, t2);
    let a3 = c.g[A3][0];
    Addr(a1.wrapping_add(0x18)).sd(c, a3);
    Addr(a1.wrapping_add(0x8)).sd(c, 0);
    c.g[A1][0] = sx(a1.wrapping_add(0x20));
    c.g[V1][0] = c.g[A1][0];
    // Ten 16-byte blocks: T3 walks the source, V1 the destination.
    loop {
        c.cycles += 7;
        let src = render_ea(c.g[T3][0], 0);
        c.lq(2, src);
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x10));
        let dst = render_ea(c.g[V1][0], 0);
        c.sq(2, dst);
        let more = (c.g[T4][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0039_6bd8);
            }
            continue;
        }
        // Tail: the 16-byte tag at a1+0xa0 and the list end at a1+0xa8.
        c.cycles += 14;
        c.g[V0][0] = 0x0;
        c.g[A0][0] = 0x1400_0000;
        c.g[V0][0] = sx(render_ea(c.g[V0][0], 0x1150));
        Addr(render_ea(c.g[A1][0], 0xa0)).sd(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V1][0] = 0x0fff_0000;
        c.g[V0][0] |= c.g[A0][0];
        c.g[V1][0] |= 0xffff;
        c.g[V0][0] <<= 32;
        let end = c.g[V0][0];
        Addr(render_ea(c.g[A1][0], 0xa8)).sd(c, end);
        c.g[A1][0] = sx(render_ea(c.g[A1][0], 0xb0));
        c.g[V1][0] = c.g[A1][0] & c.g[V1][0];
        let t = c.g[RA][0] as u32;
        let list = c.g[V1][0];
        Addr(render_ea(c.g[T5][0], 0x5a00)).sw(c, list);
        c.pc = t;
        return;
    }
}

/// `SetRenderTargetTexture(PS2GraphicsMan* gm, int id)` (0x37d348; the
/// reference names it `cPS2GraphicsMan_LockFrameToTexture`): builds the
/// upload packet through `TexMan_BuildUploadPacket`, then, if the word the
/// pointer at `gm+0x18f0` leads to is below 2600, rewrites the packet
/// header at `gm+0xe84` and writes a packet record with halfwords at +0x1c
/// and +0x1e. Always ends by adding 0x10 to the list pointer at `gm+0x5a00`.
pub fn ps2_graphics_man_set_render_target_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = 0x3000_0000;
        frame.save(c, S1, 0x30);
        c.g[S1][0] = Addr(render_ea(c.g[S0][0], 0x5a00)).lw(c);
        c.g[A0][0] = Addr(render_ea(c.g[S0][0], 0x18f4)).lw(c);
        c.g[V0][0] = c.g[S1][0] | c.g[S2][0];
        let header = c.g[V0][0];
        Addr(sp(c)).sw(c, header);
        call_ptr(c, 0x0036_7db8, 0x0037_d37c)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 0x0fff_0000;
        c.g[V0][0] |= 0xffff;
        c.g[A3][0] = Addr(render_ea(c.g[S0][0], 0x18f0)).lw(c);
        c.g[T1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[T0][0] = Addr(render_ea(c.g[S0][0], 0xe84)).lw(c);
        let t1 = c.g[T1][0];
        Addr(render_ea(c.g[S0][0], 0x5a00)).sw(c, t1);
        c.g[V0][0] = Addr(render_ea(c.g[A3][0], 0)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
        let skip = c.g[V0][0] == 0;
        c.g[A1][0] = 0x6_0000;
        if !skip {
            c.cycles += 31;
            c.g[A0][0] = Addr(render_ea(c.g[T0][0], 0x8)).lw(c);
            c.g[A1][0] |= 0x8000;
            c.g[V1][0] = !0x1f;
            c.g[A1][0] = sx(render_ea(c.g[A3][0], c.g[A1][0] as u32));
            c.g[A0][0] &= c.g[V1][0];
            c.g[V0][0] = Addr(render_ea(c.g[A1][0], 0x1cc4)).lw(c);
            c.g[A2][0] = sx(render_ea(c.g[A3][0], 0x80));
            c.g[A2][0] |= c.g[S2][0];
            c.g[V0][0] &= 0x1f;
            c.g[A0][0] |= c.g[V0][0];
            let a0 = c.g[A0][0];
            Addr(render_ea(c.g[T0][0], 0x8)).sw(c, a0);
            c.g[V1][0] = Addr(render_ea(c.g[A3][0], 0)).lw(c);
            c.g[A0][0] = Addr(render_ea(c.g[A1][0], 0x1cc0)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            let v1 = c.g[V1][0];
            Addr(render_ea(c.g[A3][0], 0)).sw(c, v1);
            c.ldl(3, render_ea(c.g[T0][0], 0x7));
            c.ldr(3, render_ea(c.g[T0][0], 0));
            c.ldl(5, render_ea(c.g[T0][0], 0xf));
            c.ldr(5, render_ea(c.g[T0][0], 0x8));
            c.g[A2][0] = Addr(render_ea(c.g[T0][0], 0x10)).lw(c);
            c.sdl(3, render_ea(c.g[V0][0], 0x7));
            c.sdr(3, render_ea(c.g[V0][0], 0));
            c.sdl(5, render_ea(c.g[V0][0], 0xf));
            c.sdr(5, render_ea(c.g[V0][0], 0x8));
            let a2 = c.g[A2][0];
            Addr(render_ea(c.g[V0][0], 0x10)).sw(c, a2);
            let s1 = c.g[S1][0];
            Addr(render_ea(c.g[V0][0], 0x14)).sw(c, s1);
            let t1 = c.g[T1][0];
            Addr(render_ea(c.g[V0][0], 0x18)).sw(c, t1);
            let a0 = c.g[A0][0];
            Addr(render_ea(c.g[V0][0], 0x1c)).sh(c, a0);
            Addr(render_ea(c.g[V0][0], 0x1e)).sh(c, 0);
        }
        // Epilogue (also the skip path).
        c.cycles += 10;
        c.g[V1][0] = Addr(render_ea(c.g[S0][0], 0x5a00)).lw(c);
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
        frame.restore(c, S2, 0x20);
        let v1 = c.g[V1][0];
        Addr(render_ea(c.g[S0][0], 0x5a00)).sw(c, v1);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `FillRenderTarget(PS2GraphicsMan* gm, int id, u32 argb)` (0x37d800; the
/// reference names it `cPS2GraphicsMan_ShadowClearLockTexture`): builds the
/// sprite packet through `TexMan_BuildSpritePacket`, then, under the same
/// 2600 test as `SetRenderTargetTexture`, writes the packet record and its
/// halfwords at +0x1c/+0x1e. Always adds 0x10 to the list pointer at
/// `gm+0x5a00`.
pub fn ps2_graphics_man_fill_render_target(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = 0x3000_0000;
        frame.save(c, S1, 0x30);
        c.g[S1][0] = Addr(render_ea(c.g[S0][0], 0x5a00)).lw(c);
        c.g[A0][0] = Addr(render_ea(c.g[S0][0], 0x18f4)).lw(c);
        c.g[V0][0] = c.g[S1][0] | c.g[S2][0];
        let header = c.g[V0][0];
        Addr(sp(c)).sw(c, header);
        call_ptr(c, 0x0036_7f18, 0x0037_d834)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 0x0fff_0000;
        c.g[V0][0] |= 0xffff;
        c.g[A3][0] = Addr(render_ea(c.g[S0][0], 0x18f0)).lw(c);
        c.g[T1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[T0][0] = Addr(render_ea(c.g[S0][0], 0xe84)).lw(c);
        let t1 = c.g[T1][0];
        Addr(render_ea(c.g[S0][0], 0x5a00)).sw(c, t1);
        c.g[V0][0] = Addr(render_ea(c.g[A3][0], 0)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
        let skip = c.g[V0][0] == 0;
        c.g[A1][0] = 0x6_0000;
        if !skip {
            c.cycles += 32;
            c.g[A0][0] = Addr(render_ea(c.g[T0][0], 0x8)).lw(c);
            c.g[A1][0] |= 0x8000;
            c.g[V1][0] = !0x1f;
            c.g[A1][0] = sx(render_ea(c.g[A3][0], c.g[A1][0] as u32));
            c.g[A0][0] &= c.g[V1][0];
            c.g[V0][0] = Addr(render_ea(c.g[A1][0], 0x1cc4)).lw(c);
            c.g[A2][0] = sx(render_ea(c.g[A3][0], 0x80));
            c.g[A2][0] |= c.g[S2][0];
            c.g[V0][0] &= 0x1f;
            c.g[A0][0] |= c.g[V0][0];
            let a0 = c.g[A0][0];
            Addr(render_ea(c.g[T0][0], 0x8)).sw(c, a0);
            c.g[V1][0] = Addr(render_ea(c.g[A3][0], 0)).lw(c);
            c.g[A0][0] = Addr(render_ea(c.g[A1][0], 0x1cc0)).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            let v1 = c.g[V1][0];
            Addr(render_ea(c.g[A3][0], 0)).sw(c, v1);
            c.ldl(3, render_ea(c.g[T0][0], 0x7));
            c.ldr(3, render_ea(c.g[T0][0], 0));
            c.ldl(5, render_ea(c.g[T0][0], 0xf));
            c.ldr(5, render_ea(c.g[T0][0], 0x8));
            c.g[A2][0] = Addr(render_ea(c.g[T0][0], 0x10)).lw(c);
            c.sdl(3, render_ea(c.g[V0][0], 0x7));
            c.sdr(3, render_ea(c.g[V0][0], 0));
            c.sdl(5, render_ea(c.g[V0][0], 0xf));
            c.sdr(5, render_ea(c.g[V0][0], 0x8));
            let a2 = c.g[A2][0];
            Addr(render_ea(c.g[V0][0], 0x10)).sw(c, a2);
            c.g[V1][0] = 0x1;
            let s1 = c.g[S1][0];
            Addr(render_ea(c.g[V0][0], 0x14)).sw(c, s1);
            let v1 = c.g[V1][0];
            Addr(render_ea(c.g[V0][0], 0x1e)).sh(c, v1);
            let t1 = c.g[T1][0];
            Addr(render_ea(c.g[V0][0], 0x18)).sw(c, t1);
            let a0 = c.g[A0][0];
            Addr(render_ea(c.g[V0][0], 0x1c)).sh(c, a0);
        }
        // Epilogue (also the skip path).
        c.cycles += 9;
        c.g[V0][0] = Addr(render_ea(c.g[S0][0], 0x5a00)).lw(c);
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        frame.restore(c, S2, 0x20);
        let v0 = c.g[V0][0];
        Addr(render_ea(c.g[S0][0], 0x5a00)).sw(c, v0);
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// Two records (`a` at DATA, `b` at DATA+0x100) with random words. Field
    /// `k` of `b` differs from `a` for `k` in the field list, and `k ==
    /// fields.len()` is the full match, so each early exit and the success
    /// return is reached.
    fn wpin_record_setup(c: &mut Cpu, seed: u32, fields: &[WpinField], salt: u64) {
        let mut r = Rng(salt ^ (seed as u64) << 12);
        scramble(c, &mut r);
        c.pc = 0x002b_db38;
        let a = DATA;
        let b = DATA + 0x100;
        fill(c, &mut r, a, 0x30);
        for i in 0..0x30 {
            let v = c.bus.read32(a + 4 * i);
            c.bus.write32(b + 4 * i, v);
        }
        let n = fields.len() as u32;
        let k = seed % (n + 1);
        if k < n {
            let (oa, ob) = match fields[k as usize] {
                WpinField::Float(_, oa, ob) | WpinField::Word(_, oa, ob) => (oa, ob),
            };
            let v = c.bus.read32(a + oa) ^ 1;
            c.bus.write32(b + ob, v);
        }
        c.g[A0][0] = sx(a);
        c.g[A1][0] = sx(b);
    }

    #[test]
    fn twpin_fog_is_equal_matches() {
        check(
            ee::s08::f_002bdb38__generated,
            twpin_fog_is_equal,
            300,
            &|c, seed| wpin_record_setup(c, seed, &FOG_FIELDS, 0x2bdb38),
        );
    }

    #[test]
    fn twpin_screen_tint_is_equal_matches() {
        check(
            ee::s08::f_002bdc78__generated,
            twpin_screen_tint_is_equal,
            300,
            &|c, seed| wpin_record_setup(c, seed, &FOG_FIELDS, 0x2bdc78),
        );
    }

    #[test]
    fn twpin_sun_is_equal_matches() {
        check(
            ee::s08::f_002bdd38__generated,
            twpin_sun_is_equal,
            300,
            &|c, seed| wpin_record_setup(c, seed, &SUN_FIELDS, 0x2bdd38),
        );
    }

    #[test]
    fn twpin_weather_is_equal_matches() {
        check(
            ee::s08::f_002bdee0__generated,
            twpin_weather_is_equal,
            300,
            &|c, seed| wpin_record_setup(c, seed, &WEATHER_FIELDS, 0x2bdee0),
        );
    }

    /// Lighting list fixture: the gm record at DATA, its render-state record
    /// at DATA+0x1000, the source table at gm+0x6bb0 and the list pointer at
    /// gm+0x5a00 (destination in the 0x3000_0000 window).
    fn emit_lighting_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x396b_40 ^ (seed as u64) << 9);
        scramble(c, &mut r);
        c.pc = 0x0039_6b40;
        let gm = DATA;
        c.bus.write32(gm + 0xe84, DATA + 0x1000);
        c.bus.write32(gm + 0x5a00, DATA + 0x9000 + (seed % 4) * 0x100);
        fill(c, &mut r, DATA + 0x1000, 0x10);
        fill(c, &mut r, gm + 0x6bb0, 0x40);
        fill(c, &mut r, DATA + 0x9000, 0x80);
        c.g[A0][0] = sx(gm);
    }

    #[test]
    fn ps2_graphics_man_emit_lighting_packet_matches() {
        check(
            ee::s13::f_00396b40__generated,
            ps2_graphics_man_emit_lighting_packet,
            200,
            &|c, seed| emit_lighting_setup(c, seed),
        );
    }

    /// Render-target fixture: the gm record at DATA with its texture id, the
    /// texture record at DATA+0x2000 (its first word picks the 2600 branch)
    /// and the render-target record at DATA+0x3000. The texture packet
    /// builders are the real callees, so the machine gets the runtime.
    fn render_target_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = Rng(salt ^ (seed as u64) << 10);
        crate::difftest::fakes::machine_rt(c, seed, salt, 0);
        let gm = DATA;
        let tex = DATA + 0x2000;
        let target = DATA + 0x3000;
        c.bus.write32(gm + 0x5a00, (r.next_u32() & 0x00ff_ffff) | DATA);
        c.bus.write32(gm + 0x18f4, r.next_u32() & 0xff);
        c.bus.write32(gm + 0x18f0, tex);
        c.bus.write32(gm + 0xe84, target);
        // At or above 2600 on one seed in four (the skip path), else below.
        let first = if seed % 4 == 3 {
            2600 + r.next_u32() % 1000
        } else {
            r.next_u32() % 2600
        };
        c.bus.write32(tex, first);
        fill(c, &mut r, tex + 4, 0x20);
        fill(c, &mut r, target, 0x10);
        c.g[A0][0] = sx(gm);
    }

    #[test]
    fn ps2_graphics_man_set_render_target_texture_matches() {
        check(
            ee::s12::f_0037d348__generated,
            ps2_graphics_man_set_render_target_texture,
            200,
            &|c, seed| render_target_setup(c, seed, 0x37d348),
        );
    }

    #[test]
    fn ps2_graphics_man_fill_render_target_matches() {
        check(
            ee::s12::f_0037d800__generated,
            ps2_graphics_man_fill_render_target,
            200,
            &|c, seed| render_target_setup(c, seed, 0x37d800),
        );
    }
}
