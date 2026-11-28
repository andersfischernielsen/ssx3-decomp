//! `src/bx/seg_1ba100.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::render::lightman::eq_s_latched;
pub(crate) use crate::game::render::lightman::load_fpr;
pub(crate) use crate::game::worldpainter::worldpaintermanquery::wp_field;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_9100, audio_debug_printf, "Audio_DebugPrintf"),
    (
        0x002b_acf0,
        speech_host_base_get_speech_category_002bacf0,
        "SpeechHostBase_GetSpeechCategory",
    ),
    (
        0x002b_acf8,
        speech_host_base_get_speech_category_volume_002bacf8,
        "SpeechHostBase_GetSpeechCategoryVolume",
    ),
    (0x002b_af08, wp_data_relocate, "WPData_Relocate"),
    (
        0x002b_b5f0,
        wp_file_relocate,
        "cWorldPainterSection_resolve",
    ),
    (
        0x002b_da98,
        wp_igd_music_trigger_is_equal,
        "tWPInGameData_MusicTrigger_isEqual",
    ),
    (0x002b_db10, camera_is_equal, "tWPInGameData_Camera_isEqual"),
    (0x002b_db38, twpin_fog_is_equal, "tWPInGameData_Fog_isEqual"),
    (
        0x002b_dbd0,
        light_glow_is_equal,
        "tWPInGameData_LightGlow_isEqual",
    ),
    (
        0x002b_dc78,
        twpin_screen_tint_is_equal,
        "tWPInGameData_ScreenTint_isEqual",
    ),
    (
        0x002b_dd10,
        sky_box_is_equal,
        "tWPInGameData_SkyBox_isEqual",
    ),
    (0x002b_dd38, twpin_sun_is_equal, "tWPInGameData_Sun_isEqual"),
    (
        0x002b_de08,
        surface_is_equal,
        "tWPInGameData_Surface_isEqual",
    ),
    (
        0x002b_dee0,
        twpin_weather_is_equal,
        "tWPInGameData_Weather_isEqual",
    ),
    (
        0x002b_e078,
        wp_igd_danger_is_equal,
        "tWPInGameData_Danger_isEqual",
    ),
    (
        0x002b_e1f8,
        tw_ingame_lighting_set_defaults,
        "tWPInGameData_Lighting_setDefaults",
    ),
    (
        0x002b_e258,
        wp_in_game_weather_set_defaults,
        "tWPInGameData_Weather_setDefaults",
    ),
];

// ---------------------------------------------------------------------------
// Audio.

/// `void Audio_DebugPrintf(int unused, ...)` (0x2b9100): spill the register
/// arguments a1..a3/t0..t3 and float arguments f12/f14/f16/f18 into the caller's
/// frame (a debug printf stub with an empty body).
pub fn audio_debug_printf(c: &mut Cpu) {
    c.cycles += 14;
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x80);
    c.g[SP][0] = sx(sp);
    let s = Addr(sp);
    s.off(0x48).sd(c, c.g[A1][0]);
    s.off(0x50).sd(c, c.g[A2][0]);
    s.off(0x58).sd(c, c.g[A3][0]);
    s.off(0x60).sd(c, c.g[T0][0]);
    s.off(0x68).sd(c, c.g[T1][0]);
    s.off(0x70).sd(c, c.g[T2][0]);
    s.off(0x78).sd(c, c.g[T3][0]);
    s.off(0x38).write_fpu(c, FpuF32(c.f[12]));
    s.off(0x3c).write_fpu(c, FpuF32(c.f[14]));
    s.off(0x40).write_fpu(c, FpuF32(c.f[16]));
    s.off(0x44).write_fpu(c, FpuF32(c.f[18]));
    c.g[SP][0] = sx(sp.wrapping_add(0x80));
    abi::ret(c);
}

/// One compared field of a record, in the order the generated `isEqual`
/// bodies make their compares. The cycles are charged before the loads.
#[derive(Clone, Copy)]
pub(crate) enum WpinField {
    /// Float field: `lwc1` from `a + oa` (f1) and `b + ob` (f0), `c.eq.s`.
    Float(u64, u32, u32),
    /// Word field: `lw` from `a + oa` (v1) and `b + ob` (v0), then `!=`.
    Word(u64, u32, u32),
}

/// Compares two records field by field, stopping at the first difference,
/// and returns from the caller with v0 = 1 when all fields match, else 0.
/// Charges the two cycles of the exit block after the last compare.
pub(crate) fn wpin_record_equal(c: &mut Cpu, a: u32, b: u32, fields: &[WpinField]) {
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
pub(crate) const FOG_FIELDS: [WpinField; 6] = [
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
pub(crate) const SUN_FIELDS: [WpinField; 9] = [
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
pub(crate) const WEATHER_FIELDS: [WpinField; 19] = [
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

/// `bool tWPInGameData_Camera_isEqual(a0, a1)` (0x002bdb10): the float at +0x8
/// of `a0` against the float at +0x4 of `a1` (`c.eq.s`); v0 = 1 when equal.
pub fn camera_is_equal(c: &mut Cpu) {
    c.cycles += 6;
    let ax = r32(c, A0).wrapping_add(0x8);
    let x = load_fpr(c, 1, ax);
    let ay = r32(c, A1).wrapping_add(0x4);
    let y = load_fpr(c, 0, ay);
    let eq = eq_s_latched(c, x, y);
    c.g[V0][0] = 0;
    if eq {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool tWPInGameData_Surface_isEqual(a0, a1)` (0x002bde08): the same compare as
/// [`camera_is_equal`] (+0x8 of `a0` against +0x4 of `a1`), a separate body in
/// the generated code.
pub fn surface_is_equal(c: &mut Cpu) {
    c.cycles += 6;
    let ax = r32(c, A0).wrapping_add(0x8);
    let x = load_fpr(c, 1, ax);
    let ay = r32(c, A1).wrapping_add(0x4);
    let y = load_fpr(c, 0, ay);
    let eq = eq_s_latched(c, x, y);
    c.g[V0][0] = 0;
    if eq {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool tWPInGameData_LightGlow_isEqual(a0, a1)` (0x002bdbd0): seven float pairs
/// must all compare equal (`c.eq.s`): `a0` at +0x8, +0x10, +0x18, +0x20, +0x28,
/// +0x30, +0x38 against `a1` at +0x4, +0x8, +0xc, +0x10, +0x14, +0x18, +0x1c.
/// Stops at the first pair that differs (v0 = 0); v0 = 1 when all match.
pub fn light_glow_is_equal(c: &mut Cpu) {
    const PAIRS: [(u32, u32); 7] = [
        (0x08, 0x04),
        (0x10, 0x08),
        (0x18, 0x0c),
        (0x20, 0x10),
        (0x28, 0x14),
        (0x30, 0x18),
        (0x38, 0x1c),
    ];
    // The `cycles += N` the generated body charges before each pair's loads.
    const CYCLES_BEFORE: [u64; 7] = [6, 4, 5, 5, 5, 6, 5];
    let a0 = r32(c, A0);
    let a1 = r32(c, A1);
    let mut all_equal = true;
    for (k, &(ao, bo)) in PAIRS.iter().enumerate() {
        c.cycles += CYCLES_BEFORE[k];
        let x = load_fpr(c, 1, a0.wrapping_add(ao));
        let y = load_fpr(c, 0, a1.wrapping_add(bo));
        if !eq_s_latched(c, x, y) {
            all_equal = false;
            break;
        }
    }
    c.g[V0][0] = all_equal as u64;
    c.cycles += 2;
    abi::ret(c);
}

/// `bool tWPInGameData_SkyBox_isEqual(a, b)` (0x002bdd10): true when the
/// sky-box rotation floats of the two records are equal (`a` at +0x8, `b`
/// at +0x4 in the generated body's order).
pub fn sky_box_is_equal(c: &mut Cpu) {
    c.cycles += 6;
    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.c_eq_s(1, 0);
    c.g[V0][0] = 0u64;
    if c.fcc() {
        c.cycles += 1;
        c.g[V0][0] = 0x1;
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// World data relocation.

/// `void WPData_Relocate(WPData* d)` (0x2baf08): turns the offsets in the
/// painter data into pointers. `d->+0` becomes `d + d->+0` and is passed to
/// `QuadTree_InitFreeList`; `d->+8` becomes `d + d->+8`, the base of an array
/// of `d->+4` records (8 bytes each, the word at `+4`): a record word of -1
/// becomes 0, any other becomes `d + word`. Frame 0x20 (s0).
pub fn wp_data_relocate(c: &mut Cpu) {
    const QUAD_TREE_INIT_FREE_LIST: Callee = Callee {
        addr: 0x002c_17c0,
        f: ee::s08::f_002c17c0,
    };
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        let relocated = c.g[V0][0];
        Addr(this).sw(c, relocated);
        QUAD_TREE_INIT_FREE_LIST.call(c, 0x002b_af2c)?;
        c.g[V0][0] = Addr(this.wrapping_add(0x8)).lw(c);
        c.g[A1][0] = 0;
        c.g[V1][0] = Addr(this.wrapping_add(0x4)).lw(c);
        c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        let empty = (c.g[V1][0] as i64) <= 0;
        let base = c.g[V0][0];
        Addr(this.wrapping_add(0x8)).sw(c, base);
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = u64::MAX;
            loop {
                let record = r32(c, A0);
                c.cycles += 3;
                c.g[V0][0] = Addr(record.wrapping_add(0x4)).lw(c);
                let unset = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
                if unset {
                    c.cycles += 1;
                    Addr(record.wrapping_add(0x4)).sw(c, 0);
                } else {
                    c.cycles += 2;
                    let value = c.g[V0][0];
                    Addr(record.wrapping_add(0x4)).sw(c, value);
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(this.wrapping_add(0x4)).lw(c);
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x8));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_af50);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `tWPInGameData_Weather_setDefaults(this)` (0x2be258): sets the weather
/// defaults in the record in `a0`: the floats 1.0 / 6.0 / 10.0 and the value of
/// the global at `gp-0x4290` at their offsets, and clears the integer fields.
pub fn wp_in_game_weather_set_defaults(c: &mut Cpu) {
    c.cycles += 28;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[AT][0] = 0x40c0_0000;
    c.mtc1(1, 1);
    c.g[AT][0] = 0x4120_0000;
    c.mtc1(1, 2);
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4290));
    let this = c.g[A0][0] as u32;
    c.swc1(1, this.wrapping_add(0x10));
    c.swc1(2, this.wrapping_add(0x30));
    c.swc1(3, this.wrapping_add(0x80));
    c.swc1(0, this.wrapping_add(0x98));
    c.sw(this, 0u64);
    c.sw(this.wrapping_add(0x8), 0u64);
    c.sw(this.wrapping_add(0x18), 0u64);
    c.sw(this.wrapping_add(0x20), 0u64);
    c.sw(this.wrapping_add(0x28), 0u64);
    c.swc1(0, this.wrapping_add(0x38));
    c.swc1(0, this.wrapping_add(0x40));
    c.sw(this.wrapping_add(0x48), 0u64);
    c.sw(this.wrapping_add(0x50), 0u64);
    c.sw(this.wrapping_add(0x58), 0u64);
    c.swc1(0, this.wrapping_add(0x60));
    c.swc1(0, this.wrapping_add(0x68));
    c.swc1(0, this.wrapping_add(0x70));
    c.swc1(0, this.wrapping_add(0x78));
    c.swc1(0, this.wrapping_add(0x88));
    c.swc1(0, this.wrapping_add(0x90));
    ret(c);
}

/// `void tWPInGameData::Lighting::setDefaults(Lighting* l)` (0x2be1f8):
/// writes the default lighting block at `l`: four unaligned 8-byte copies of
/// a constant pair from the GP area (the last one after the float constants),
/// the floats 1.0 at `+0x48` and 0.75 at `+0x50`, and a zero word at `+0`.
pub fn tw_ingame_lighting_set_defaults(c: &mut Cpu) {
    c.cycles += 24;
    let pair_hi = (c.g[GP][0] as u32).wrapping_add(0x22e7);
    let pair_lo = (c.g[GP][0] as u32).wrapping_add(0x22e0);
    let l = c.g[A0][0] as u32;
    for (hi, lo) in [(0x0fu32, 0x08u32), (0x1f, 0x18), (0x2f, 0x28)] {
        c.ldl(2, pair_hi);
        c.ldr(2, pair_lo);
        c.sdl(2, l.wrapping_add(hi));
        c.sdr(2, l.wrapping_add(lo));
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.g[AT][0] = 0x3f40_0000;
    c.mtc1(1, 0);
    c.ldl(2, pair_hi);
    c.ldr(2, pair_lo);
    c.sdl(2, l.wrapping_add(0x3f));
    c.sdr(2, l.wrapping_add(0x38));
    c.swc1(1, l.wrapping_add(0x48));
    c.swc1(0, l.wrapping_add(0x50));
    Addr(l).sw(c, 0);
    ret(c);
}

/// `void cWorldPainterSection::resolve(self)` (0x2bb5f0), the `WPFile_Relocate`
/// name: when `self->+8` is non-null and its first word is 0x10 (or the
/// `+2` halfword of `self` is non-zero, else it calls 0x317e98 first), loops
/// `count` (`*(p+4)`) entries of `p+8`: each -1 entry is cleared, others are
/// rebased to `p + entry` and passed to `WPData_Relocate`. Ends by storing 3
/// as a halfword at `self`.
pub fn wp_file_relocate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
        c.g[V0][0] = 0x10;
        'end: {
            if c.g[A0][0] == 0 {
                break 'end;
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(2)).lh(c);
                if c.g[V0][0] != 0 {
                    Addr(r32(c, S1).wrapping_add(8)).sw(c, 0);
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x0031_7e98, 0x002b_b638)?;
                    Addr(r32(c, S1).wrapping_add(8)).sw(c, 0);
                }
                c.cycles += 1;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            }
            c.cycles += 2;
            let null_entry = c.g[V1][0] == 0;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(8));
            if null_entry {
                break 'end;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = 0;
            if none {
                break 'end;
            }
            c.cycles += 1;
            c.g[S3][0] = 0xffff_ffff_ffff_ffff;
            loop {
                'after_relocate: {
                    'entry_done: {
                        c.cycles += 3;
                        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                        if c.g[V1][0] != c.g[S3][0] {
                            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
                            break 'entry_done;
                        }
                        c.cycles += 2;
                        Addr(r32(c, S0)).sw(c, 0);
                        break 'after_relocate;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.g[V0][0];
                    let rebased = c.g[V0][0];
                    Addr(r32(c, S0)).sw(c, rebased);
                    call_ptr(c, 0x002b_af08, 0x002b_b67c)?;
                }
                c.cycles += 6;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(4)).lw(c);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break 'end;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_b658);
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = 3;
        let three = c.g[V0][0];
        Addr(r32(c, S1)).sh(c, three);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SpeechHostBase_GetSpeechCategory` (0x002b_acf0): straight-line leaf. Mechanical port of the generated body.
pub fn speech_host_base_get_speech_category_002bacf0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0xffffffffffffffff;
    ret(c);
}

/// `SpeechHostBase_GetSpeechCategoryVolume` (0x002b_acf8): straight-line leaf. Mechanical port of the generated body.
pub fn speech_host_base_get_speech_category_volume_002bacf8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `bool tWPInGameData_MusicTrigger_isEqual(a, b)` (0x2bda98): false unless
/// `a+8 == b+4`; then true when `a+0x10 == b+8` (32-bit words, compared as
/// 64-bit register values).
pub fn wp_igd_music_trigger_is_equal(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(wp_field(c.g[A0][0], 8)).lw(c);
    c.g[V0][0] = Addr(wp_field(c.g[A1][0], 4)).lw(c);
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 5;
        c.g[V0][0] = Addr(wp_field(c.g[A0][0], 0x10)).lw(c);
        c.g[V1][0] = Addr(wp_field(c.g[A1][0], 8)).lw(c);
        let diff = c.g[V0][0] ^ c.g[V1][0];
        c.g[V0][0] = (diff < 1) as u64;
    }
    ret(c);
}

/// `bool tWPInGameData_Danger_isEqual(a, b)` (0x2be078): as the music trigger
/// one, but the last two words are floats compared with `c.eq.s`.
pub fn wp_igd_danger_is_equal(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(wp_field(c.g[A0][0], 8)).lw(c);
    c.g[V0][0] = Addr(wp_field(c.g[A1][0], 4)).lw(c);
    let differ = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = 0;
    if !differ {
        c.cycles += 5;
        let a_rate = wp_field(c.g[A0][0], 0x10);
        c.lwc1(1, a_rate);
        let b_rate = wp_field(c.g[A1][0], 8);
        c.lwc1(0, b_rate);
        c.c_eq_s(1, 0);
        if c.fcc() {
            c.cycles += 2;
            c.g[V0][0] = 1;
            ret(c);
            return;
        }
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;

    #[test]
    fn audio_debug_printf_matches() {
        check(
            ssx3_ee::s08::f_002b9100,
            audio_debug_printf,
            100,
            &|c, seed| {
                let mut r = Rng(0x9100 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    use crate::difftest::check_paths;

    use crate::difftest::fakes;

    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::machine;

    use crate::difftest::fill;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::fe::ovstatereplay::tests::oc1493_install_fakes;

    pub(crate) use crate::game::fe::ovstatereplay::tests::OC1493_QUAD_GATE;
    pub(crate) use crate::game::fe::ovstatereplay::tests::OC1493_QUAD_MAGIC;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Two records (`a` at DATA, `b` at DATA+0x100) with random words. Field
    /// `k` of `b` differs from `a` for `k` in the field list, and `k ==
    /// fields.len()` is the full match, so each early exit and the success
    /// return is reached.
    pub(crate) fn wpin_record_setup(c: &mut Cpu, seed: u32, fields: &[WpinField], salt: u64) {
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

    /// Two floats: `x` at `a0 + 8` and `y` at `a1 + 4`; equal for a third of the
    /// seeds, one ulp apart for another third, unrelated for the rest.
    pub(crate) fn oc1485_pair_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1485_0b10 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x2000);
        let x = r.f32(4.0);
        let y = match seed % 3 {
            0 => x,
            1 => x ^ 1,
            _ => r.f32(4.0),
        };
        c.bus.write32(DATA + 0x1008, x);
        c.bus.write32(DATA + 0x2004, y);
    }

    /// Seven float pairs; the first `seed % 8` pairs match, the next one differs
    /// (`seed % 8 == 7`: all match).
    pub(crate) fn oc1485_glow_setup(c: &mut Cpu, seed: u32) {
        const AO: [u32; 7] = [0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38];
        const BO: [u32; 7] = [0x04, 0x08, 0x0c, 0x10, 0x14, 0x18, 0x1c];
        let mut r = Rng(0x1485_2bd0 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x2000);
        let fail = seed % 8;
        for j in 0..7u32 {
            let x = r.f32(4.0);
            let y = if j < fail {
                x
            } else if j == fail {
                x ^ 1
            } else {
                r.f32(4.0)
            };
            c.bus.write32(DATA + 0x1000 + AO[j as usize], x);
            c.bus.write32(DATA + 0x2000 + BO[j as usize], y);
        }
    }

    #[test]
    fn camera_is_equal_matches() {
        check(
            ee::s08::f_002bdb10__generated,
            camera_is_equal,
            200,
            &|c, seed| oc1485_pair_setup(c, seed),
        );
    }

    #[test]
    fn surface_is_equal_matches() {
        check(
            ee::s08::f_002bde08__generated,
            surface_is_equal,
            200,
            &|c, seed| oc1485_pair_setup(c, seed),
        );
    }

    #[test]
    fn light_glow_is_equal_matches() {
        check_paths(
            ee::s08::f_002bdbd0__generated,
            light_glow_is_equal,
            200,
            &[8, 12, 17, 22, 27, 33, 38],
            &|c, seed| oc1485_glow_setup(c, seed),
        );
    }

    #[test]
    fn sky_box_is_equal_matches() {
        check(
            ee::s08::f_002bdd10__generated,
            sky_box_is_equal,
            200,
            &|c, seed| {
                let mut r = Rng(0x2bdd_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                let v = r.edge_f32();
                c.bus.write32(DATA + 0x8, v);
                let w = if seed % 2 == 0 { v } else { r.edge_f32() };
                c.bus.write32(DATA + 0x100 + 0x4, w);
            },
        );
    }

    /// `WPData_Relocate`: the head offset, the record count (negative, zero
    /// and up to three records) and records that are unset (-1) or not.
    #[test]
    fn wp_data_relocate_matches() {
        oc1493_install_fakes();
        fakes::covered(
            ee::s08::f_002baf08__generated,
            wp_data_relocate,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2baf08, 1);
                c.bus.write32(OC1493_QUAD_GATE, OC1493_QUAD_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, r.next_u32() % 0x800);
                let count = [0u32, 1, 2, 3, 0xffff_ffff][seed as usize % 5];
                c.bus.write32(this + 0x4, count);
                c.bus.write32(this + 0x8, 0x400);
                for i in 0..3u32 {
                    let word = if r.next_u32() % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 0x800
                    };
                    c.bus.write32(this + 0x400 + 8 * i + 0x4, word);
                }
            },
        );
    }

    /// `tWPInGameData_Weather_setDefaults`: straight-line, one fixture.
    #[test]
    fn wp_in_game_weather_set_defaults_matches() {
        check(
            ee::s08::f_002be258__generated,
            wp_in_game_weather_set_defaults,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_e258, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x4290), r.next_u32());
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn tw_ingame_lighting_set_defaults_matches() {
        check(
            ee::s08::f_002be1f8__generated,
            tw_ingame_lighting_set_defaults,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x002b_e1f8, 0);
                c.g[A0][0] = sx(DATA + 0x100 + 0x40 * (seed % 3));
            },
        );
    }

    /// Gate word in the machine's RAM: the test fakes act only when it is set.
    pub(crate) const OC1496_GATE: u32 = DATA + 0x2_0000;

    pub(crate) const OC1496_MAGIC: u32 = 0x1496_1496;

    pub(crate) static OC1496_PREV_WP_DATA_RELOCATE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1496_PREV_317E98: AtomicUsize = AtomicUsize::new(0);

    /// `WPData_Relocate` replaced by a return (the relocation itself is not
    /// under test here).
    pub(crate) fn oc1496_fake_wp_data_relocate(c: &mut Cpu) {
        if c.bus.read32(OC1496_GATE) != OC1496_MAGIC {
            return fall_back(
                c,
                &OC1496_PREV_WP_DATA_RELOCATE,
                ee::s08::f_002baf08__generated,
            );
        }
        ret(c);
    }

    /// The `0x317e98` callee replaced by a return.
    pub(crate) fn oc1496_fake_317e98(c: &mut Cpu) {
        if c.bus.read32(OC1496_GATE) != OC1496_MAGIC {
            return fall_back(c, &OC1496_PREV_317E98, ee::s10::f_00317e98__generated);
        }
        ret(c);
    }

    #[test]
    fn wp_file_relocate_matches() {
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BAF08),
            oc1496_fake_wp_data_relocate,
            &OC1496_PREV_WP_DATA_RELOCATE,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1496_fake_317e98,
            &OC1496_PREV_317E98,
        );
        check(
            ee::s08::f_002bb5f0__generated,
            wp_file_relocate,
            240,
            &|c, seed| {
                let _ = machine(c, seed, 0x002b_b5f0, 0);
                c.bus.write32(OC1496_GATE, OC1496_MAGIC);
                let s = DATA + 0x1000;
                let obj = DATA + 0x1100;
                c.g[A0][0] = sx(s);
                // seed % 6: 0 = null pointer; 1, 4, 5 = header 0x10; 2 = header
                // other, halfword set; 3 = header other, halfword zero (calls 0x317e98).
                let case = seed % 6;
                let kind: u32 = if case == 2 || case == 3 { 0x11 } else { 0x10 };
                c.bus.write32(s, if case == 2 { 0x0001_0000 } else { 0 });
                c.bus.write32(s + 8, if case == 0 { 0 } else { obj });
                c.bus.write32(obj, kind);
                let count: u32 = (seed / 6) % 4;
                c.bus.write32(obj + 4, count);
                for i in 0u32..4 {
                    let e = if (seed / 6 + i) % 2 == 0 {
                        0xffff_ffff
                    } else {
                        0x40 * (i + 1)
                    };
                    c.bus.write32(obj + 8 + 4 * i, e);
                }
            },
        );
    }

    /// `SpeechHostBase_GetSpeechCategory` (0x002b_acf0) against the generated body.
    #[test]
    fn speech_host_base_get_speech_category_002bacf0_matches() {
        check(
            ee::s08::f_002bacf0__generated,
            speech_host_base_get_speech_category_002bacf0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bacf0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SpeechHostBase_GetSpeechCategoryVolume` (0x002b_acf8) against the generated body.
    #[test]
    fn speech_host_base_get_speech_category_volume_002bacf8_matches() {
        check(
            ee::s08::f_002bacf8__generated,
            speech_host_base_get_speech_category_volume_002bacf8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bacf8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    #[test]
    fn wp_igd_music_trigger_is_equal_matches() {
        check(
            ee::s08::f_002bda98__generated,
            wp_igd_music_trigger_is_equal,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1550_07, 0);
                let a = DATA + 0x100;
                let b = DATA + 0x200;
                let w = r.next_u32();
                let x = r.next_u32();
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
                c.bus.write32(a + 8, w);
                c.bus
                    .write32(b + 4, if seed % 3 == 0 { w } else { r.next_u32() });
                c.bus.write32(a + 0x10, x);
                c.bus
                    .write32(b + 8, if seed % 2 == 0 { x } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn wp_igd_danger_is_equal_matches() {
        check(
            ee::s08::f_002be078__generated,
            wp_igd_danger_is_equal,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1550_08, 0);
                let a = DATA + 0x100;
                let b = DATA + 0x200;
                let w = r.next_u32();
                let f = if seed % 4 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(3.0)
                };
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
                c.bus.write32(a + 8, w);
                c.bus
                    .write32(b + 4, if seed % 3 == 0 { w } else { r.next_u32() });
                c.bus.write32(a + 0x10, f);
                c.bus
                    .write32(b + 8, if seed % 2 == 0 { f } else { r.f32(3.0) });
            },
        );
    }
}
