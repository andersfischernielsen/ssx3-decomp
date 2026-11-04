//! `src/worldpainter/wpigd.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::visualfx::boardwakefx::lh_2 as lh;
pub(crate) use crate::game::visualfx::boardwakefx::lw_2 as lw;
pub(crate) use crate::game::visualfx::boardwakefx::p;
pub(crate) use crate::game::visualfx::boardwakefx::r32_2 as r32;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_c6f8,
        twpigd_music_trigger_ctor,
        "tWPIGD_MusicTrigger_ctor",
    ),
    (0x002b_c728, twpigd_mix_ctor, "tWPIGD_Mix_ctor"),
    (0x002b_c750, twpigd_ambience_ctor, "tWPIGD_Ambience_ctor"),
    (0x002b_c778, twpigd_speech_ctor, "tWPIGD_Speech_ctor"),
    (0x002b_c7a0, twpigd_camera_ctor, "tWPIGD_Camera_ctor"),
    (0x002b_c7c8, twpigd_fog_ctor, "tWPIGD_Fog_ctor"),
    (0x002b_c830, twpigd_light_glow_ctor, "tWPIGD_LightGlow_ctor"),
    (
        0x002b_c890,
        twpigd_screen_tint_ctor,
        "tWPIGD_ScreenTint_ctor",
    ),
    (0x002b_c8e8, twpigd_sky_box_ctor, "tWPIGD_SkyBox_ctor"),
    (0x002b_c910, twpigd_sun_ctor, "tWPIGD_Sun_ctor"),
    (0x002b_c980, twpigd_surface_ctor, "tWPIGD_Surface_ctor"),
    (0x002b_caf8, twpigd_weather_ctor, "tWPIGD_Weather_ctor"),
    (0x002b_cbd0, twpigd_danger_ctor, "tWPIGD_Danger_ctor"),
    (0x002b_feb0, weather_save_state, "tWPIGD_Weather_SaveState"),
    (0x002b_d698, twpigd_weather_blend, "tWPIGD_Weather_Blend"),
];

// ---------------------------------------------------------------------------
// World-painter instance-data constructors.

/// `tWPIGD_MusicTrigger* tWPIGD_MusicTrigger_ctor(tWPIGD_MusicTrigger* self)`
/// (0x2bc6f8): store the class descriptor at +4, load a float literal into f0,
/// store it at +0 and four -1 sentinels at +8/+0xc/+0x10/+0x14.
pub fn twpigd_music_trigger_ctor(c: &mut Cpu) {
    c.cycles += 12;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x480000;
    let init = Addr((c.g[GP][0] as u32).wrapping_sub(0x42fc)).read_fpu(c);
    c.f[0] = init.0;
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x5af8));
    c.g[V1][0] = u64::MAX;
    obj.off(0x4).sw(c, c.g[V0][0]);
    obj.write_fpu(c, init);
    c.g[V0][0] = c.g[A0][0];
    obj.off(0x10).sw(c, c.g[V1][0]);
    obj.off(0xc).sw(c, c.g[V1][0]);
    obj.off(0x8).sw(c, c.g[V1][0]);
    obj.off(0x14).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// Shared body of the simple `tWPIGD_*_ctor`s: descriptor at +4, one float
/// literal at +0, and -1 sentinels at +8/+0xc.
pub(crate) fn wpigd_simple_ctor(c: &mut Cpu, lit_off: u32, kind: u32) {
    c.cycles += 10;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x480000;
    let init = Addr((c.g[GP][0] as u32).wrapping_sub(lit_off)).read_fpu(c);
    c.f[0] = init.0;
    c.g[V0][0] = sx(0x480000u32.wrapping_add(kind));
    c.g[V1][0] = u64::MAX;
    obj.off(0x4).sw(c, c.g[V0][0]);
    obj.write_fpu(c, init);
    c.g[V0][0] = c.g[A0][0];
    obj.off(0x8).sw(c, c.g[V1][0]);
    obj.off(0xc).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `tWPIGD_Mix* tWPIGD_Mix_ctor(tWPIGD_Mix* self)` (0x2bc728).
pub fn twpigd_mix_ctor(c: &mut Cpu) {
    wpigd_simple_ctor(c, 0x42f8, 0x58c0);
}

/// `tWPIGD_Ambience* tWPIGD_Ambience_ctor(tWPIGD_Ambience* self)` (0x2bc750).
pub fn twpigd_ambience_ctor(c: &mut Cpu) {
    wpigd_simple_ctor(c, 0x42f4, 0x5688);
}

/// `tWPIGD_Speech* tWPIGD_Speech_ctor(tWPIGD_Speech* self)` (0x2bc778).
pub fn twpigd_speech_ctor(c: &mut Cpu) {
    wpigd_simple_ctor(c, 0x42f0, 0x5450);
}

/// `tWPIGD_Camera* tWPIGD_Camera_ctor(tWPIGD_Camera* self)` (0x2bc7a0):
/// descriptor at +4, float literal f0 at +8/+0xc and f1 at +0.
pub fn twpigd_camera_ctor(c: &mut Cpu) {
    c.cycles += 10;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x480000;
    let f0 = Addr((c.g[GP][0] as u32).wrapping_sub(0x42ec)).read_fpu(c);
    c.f[0] = f0.0;
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x5218));
    let f1 = Addr((c.g[GP][0] as u32).wrapping_sub(0x42e8)).read_fpu(c);
    c.f[1] = f1.0;
    obj.off(0x4).sw(c, c.g[V0][0]);
    obj.off(0x8).write_fpu(c, f0);
    c.g[V0][0] = c.g[A0][0];
    obj.write_fpu(c, f1);
    obj.off(0xc).write_fpu(c, f0);
    abi::ret(c);
}

/// Store a list of (offset, float-register) pairs on `base`.
pub(crate) fn store_fregs(c: &mut Cpu, base: Addr, items: &[(u32, usize)]) {
    for &(off, f) in items {
        base.off(off).write_fpu(c, FpuF32(c.f[f]));
    }
}

/// `tWPIGD_Fog* tWPIGD_Fog_ctor(tWPIGD_Fog* self)` (0x2bc7c8): descriptor at
/// +4, floats at +0/+8..+0x34 with f6 = 0 duplicated.
pub fn twpigd_fog_ctor(c: &mut Cpu) {
    c.cycles += 25;
    c.f[6] = c.g[0][0] as u32;
    let obj = Addr(abi::arg(c, 0));
    c.g[V1][0] = 0x480000;
    let gp = c.g[GP][0] as u32;
    c.f[5] = Addr(gp.wrapping_sub(0x42e4)).read_u32(c);
    c.g[V1][0] = sx(0x480000u32.wrapping_add(0x4fe0));
    c.f[4] = Addr(gp.wrapping_sub(0x42e0)).read_u32(c);
    c.g[V0][0] = c.g[A0][0];
    c.f[3] = Addr(gp.wrapping_sub(0x42dc)).read_u32(c);
    c.f[1] = Addr(gp.wrapping_sub(0x42d8)).read_u32(c);
    c.f[2] = Addr(gp.wrapping_sub(0x42d4)).read_u32(c);
    c.f[0] = Addr(gp.wrapping_sub(0x42d0)).read_u32(c);
    obj.off(0x4).sw(c, c.g[V1][0]);
    store_fregs(
        c,
        obj,
        &[
            (0x0, 0),
            (0x8, 6),
            (0x10, 5),
            (0x18, 4),
            (0x20, 3),
            (0x28, 1),
            (0x30, 2),
            (0xc, 6),
            (0x14, 5),
            (0x1c, 4),
            (0x24, 3),
            (0x2c, 1),
            (0x34, 2),
        ],
    );
    abi::ret(c);
}

/// `tWPIGD_LightGlow* tWPIGD_LightGlow_ctor(tWPIGD_LightGlow* self)`
/// (0x2bc830): descriptor at +4, f0 = 1.0, f1 = 0 and f2 = literal.
pub fn twpigd_light_glow_ctor(c: &mut Cpu) {
    c.cycles += 24;
    c.g[V0][0] = 0x480000;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[1][0] as u32;
    c.f[1] = c.g[0][0] as u32;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x4da8));
    c.f[2] = Addr((c.g[GP][0] as u32).wrapping_sub(0x42cc)).read_u32(c);
    obj.off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    store_fregs(
        c,
        obj,
        &[
            (0x0, 2),
            (0x28, 0),
            (0x38, 1),
            (0xc, 0),
            (0x8, 0),
            (0x14, 0),
            (0x10, 0),
            (0x1c, 0),
            (0x18, 0),
            (0x24, 0),
            (0x20, 0),
            (0x2c, 0),
            (0x34, 1),
            (0x30, 1),
            (0x3c, 1),
        ],
    );
    abi::ret(c);
}

/// `tWPIGD_ScreenTint* tWPIGD_ScreenTint_ctor(tWPIGD_ScreenTint* self)`
/// (0x2bc890): descriptor at +4, f0 = 1.0, f1 = 0 and f2 = literal.
pub fn twpigd_screen_tint_ctor(c: &mut Cpu) {
    c.cycles += 22;
    c.g[V0][0] = 0x480000;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[1][0] as u32;
    c.f[1] = c.g[0][0] as u32;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x4b70));
    c.f[2] = Addr((c.g[GP][0] as u32).wrapping_sub(0x42c8)).read_u32(c);
    obj.off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    store_fregs(
        c,
        obj,
        &[
            (0x0, 2),
            (0x18, 0),
            (0x30, 1),
            (0xc, 0),
            (0x8, 0),
            (0x14, 0),
            (0x10, 0),
            (0x1c, 0),
            (0x24, 1),
            (0x20, 1),
            (0x2c, 1),
            (0x28, 1),
            (0x34, 1),
        ],
    );
    abi::ret(c);
}

/// `tWPIGD_SkyBox* tWPIGD_SkyBox_ctor(tWPIGD_SkyBox* self)` (0x2bc8e8):
/// descriptor at +4, f0 = 0 at +8/+0xc, f1 = literal at +0.
pub fn twpigd_sky_box_ctor(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x480000;
    c.f[0] = c.g[0][0] as u32;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x4938));
    c.f[1] = Addr((c.g[GP][0] as u32).wrapping_sub(0x42c4)).read_u32(c);
    obj.off(0x4).sw(c, c.g[V0][0]);
    obj.off(0x8).write_fpu(c, FpuF32(c.f[0]));
    c.g[V0][0] = c.g[A0][0];
    obj.write_fpu(c, FpuF32(c.f[1]));
    obj.off(0xc).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

/// `tWPIGD_Sun* tWPIGD_Sun_ctor(tWPIGD_Sun* self)` (0x2bc910): descriptor at
/// +4, f0 = 1.0, f1 = 0, f2 = literal, plus two zero words at +0x38/+0x3c.
pub fn twpigd_sun_ctor(c: &mut Cpu) {
    c.cycles += 28;
    c.g[V0][0] = 0x480000;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[1][0] as u32;
    c.f[1] = c.g[0][0] as u32;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x4700));
    c.f[2] = Addr((c.g[GP][0] as u32).wrapping_sub(0x42c0)).read_u32(c);
    obj.off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    store_fregs(
        c,
        obj,
        &[
            (0x0, 2),
            (0x10, 1),
            (0x48, 0),
            (0xc, 1),
            (0x8, 1),
            (0x14, 1),
            (0x1c, 0),
            (0x18, 0),
            (0x24, 0),
            (0x20, 0),
            (0x2c, 0),
            (0x28, 0),
            (0x34, 0),
            (0x30, 0),
        ],
    );
    obj.off(0x3c).sw(c, 0);
    obj.off(0x38).sw(c, 0);
    store_fregs(c, obj, &[(0x44, 0), (0x40, 0), (0x4c, 0)]);
    abi::ret(c);
}

/// `tWPIGD_Surface* tWPIGD_Surface_ctor(tWPIGD_Surface* self)` (0x2bc980):
/// descriptor at +4, f0 = 1.0 at +8/+0xc, f1 = literal at +0.
pub fn twpigd_surface_ctor(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0x480000;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[1][0] as u32;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x44c8));
    c.f[1] = Addr((c.g[GP][0] as u32).wrapping_sub(0x42bc)).read_u32(c);
    obj.off(0x4).sw(c, c.g[V0][0]);
    obj.off(0x8).write_fpu(c, FpuF32(c.f[0]));
    c.g[V0][0] = c.g[A0][0];
    obj.write_fpu(c, FpuF32(c.f[1]));
    obj.off(0xc).write_fpu(c, FpuF32(c.f[0]));
    abi::ret(c);
}

/// `tWPIGD_Weather* tWPIGD_Weather_ctor(tWPIGD_Weather* self)` (0x2bcaf8):
/// descriptor at +4; f0 = 1.0, f1 = 0, f2 = literal, f3 = 6.0, f4 = 10.0,
/// f5 = literal stored across the 0xa0-byte block.
pub fn twpigd_weather_ctor(c: &mut Cpu) {
    c.cycles += 53;
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[1][0] as u32;
    c.g[V1][0] = 0x480000;
    c.f[1] = c.g[0][0] as u32;
    c.g[V1][0] = sx(0x480000u32.wrapping_add(0x4058));
    c.g[AT][0] = 0x40c0_0000;
    c.f[3] = c.g[1][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.g[AT][0] = 0x4120_0000;
    c.f[4] = c.g[1][0] as u32;
    let gp = c.g[GP][0] as u32;
    c.f[5] = Addr(gp.wrapping_sub(0x42b4)).read_u32(c);
    c.f[2] = Addr(gp.wrapping_sub(0x42b0)).read_u32(c);
    obj_weather_stores(c, Addr(abi::arg(c, 0)), c.g[V1][0]);
    abi::ret(c);
}

/// The store list of [`twpigd_weather_ctor`] (kept separate to stay readable).
pub(crate) fn obj_weather_stores(c: &mut Cpu, obj: Addr, desc: u64) {
    obj.off(0x4).sw(c, desc);
    store_fregs(
        c,
        obj,
        &[
            (0x0, 2),
            (0x10, 3),
            (0x30, 4),
            (0x58, 1),
            (0xc, 1),
            (0x8, 1),
            (0x14, 3),
            (0x1c, 1),
            (0x18, 1),
            (0x24, 1),
            (0x20, 1),
            (0x2c, 1),
            (0x28, 1),
            (0x34, 4),
            (0x3c, 0),
            (0x38, 0),
            (0x44, 0),
            (0x40, 0),
            (0x4c, 1),
            (0x48, 1),
            (0x54, 1),
            (0x50, 1),
            (0x5c, 1),
            (0x64, 0),
            (0x60, 0),
            (0x6c, 0),
            (0x68, 0),
            (0x74, 0),
            (0x70, 0),
            (0x7c, 0),
            (0x78, 0),
            (0x84, 5),
            (0x80, 5),
            (0x98, 0),
            (0x8c, 0),
            (0x88, 0),
            (0x94, 0),
            (0x90, 0),
            (0x9c, 0),
        ],
    );
}

/// `tWPIGD_Danger* tWPIGD_Danger_ctor(tWPIGD_Danger* self)` (0x2bcbd0):
/// descriptor at +4, f0 = literal at +0, f1 = 0 at +0x10/+0x14, two zero words
/// at +8/+0xc.
pub fn twpigd_danger_ctor(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x480000;
    c.f[1] = c.g[0][0] as u32;
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x3e20));
    c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x42ac)).read_u32(c);
    let obj = Addr(abi::arg(c, 0));
    obj.off(0x4).sw(c, c.g[V0][0]);
    obj.off(0x10).write_fpu(c, FpuF32(c.f[1]));
    c.g[V0][0] = c.g[A0][0];
    obj.write_fpu(c, FpuF32(c.f[0]));
    obj.off(0xc).sw(c, 0);
    obj.off(0x8).sw(c, 0);
    obj.off(0x14).write_fpu(c, FpuF32(c.f[1]));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Weather.

/// `void tWPIGD_Weather_SaveState(void* self /*a0*/, void* owner /*a1*/)`
/// (0x002bfeb0): for each of nineteen 4-byte fields, calls a virtual method of
/// `owner` (vtable slot `+0xc`, `this = owner + lh(vt+8)`) passing the field
/// address from `self` and a 4-byte size. The first source field is `self+8`;
/// the rest come in pairs: `self+8+8k` then `self+0`.
pub fn weather_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 277;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 4;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        for i in 0..38u32 {
            // The vtable is re-read from the owner before every call.
            c.g[V1][0] = lw(c, r32(c, S0));
            let src = if i % 2 == 0 {
                sx(p(r32(c, S1), 8 + 8 * (i / 2)))
            } else {
                c.g[S1][0]
            };
            c.g[A2][0] = 4;
            let delta = lh(c, p(r32(c, V1), 8));
            c.g[V0][0] = lw(c, p(r32(c, V1), 0xc));
            let target = r32(c, V0);
            c.g[A1][0] = src;
            // The virtual method runs on the *owner* (a1), adjusted by the
            // vtable's this-delta; the fields come from `self` (a0).
            c.g[A0][0] = sx(p(r32(c, S0), delta as u32));
            call_ptr(c, target, 0x002b_fee4 + 0x1c * i)?;
        }
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod weather_blend {

    /// `self`: the vtable pointer; the vtable's hook (called before and
    /// after the blend) is `{i16 delta at +0x218, fn at +0x21c}`.
    pub const VPTR: u32 = 0x04;

    pub const VT_HOOK_DELTA: u32 = 0x218;

    pub const VT_HOOK: u32 = 0x21c;

    /// `self`: 19 pairs `{float current, float target}` from +0x08.
    pub const PAIRS: u32 = 0x08;

    /// `target`: the 19 keyframe values from +0x04.
    pub const VALUES: u32 = 0x04;

    /// Return addresses of the two hook calls.
    pub const RET_PRE: u32 = 0x002b_d6d0;

    pub const RET_POST: u32 = 0x002b_d950;
}

/// Call `self`'s vtable hook (`this + delta` in a0) returning to `ret`.
#[inline(always)]
pub(crate) fn wb_hook(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    use crate::guest::Addr;
    use weather_blend::*;
    let vt = r32(c, V1);
    c.g[A0][0] = Addr(vt.wrapping_add(VT_HOOK_DELTA)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(VT_HOOK)).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// One scheduling group of the blend, in the compiled code's memory order:
/// pairs `first ..= first + olds.len()`. The first pair's `t * target` goes
/// through `acc`, its old value through f0; the other pairs' old values are
/// loaded up front into `olds` and pre-scaled by `1 - t` (f3). Then each
/// pair stores `t * target + (1 - t) * current` and copies the target.
/// With `tail`, the last pair's old value (in `olds`' last register) is
/// scaled into f3 and blended through f20 (the compiled code's last pair).
pub(crate) fn wb_group(c: &mut Cpu, first: u32, acc: usize, olds: &[usize], tail: bool) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use weather_blend::*;
    let this = Addr(r32(c, S0));
    let key = Addr(r32(c, S1));
    let cur = |n: u32| this.off(PAIRS + 8 * n);
    let val = |n: u32| key.off(VALUES + 4 * n);
    let t = FpuF32(c.f[20]);
    let k = FpuF32(c.f[3]);
    c.f[acc] = val(first).read_fpu(c).0;
    c.f[0] = cur(first).read_fpu(c).0;
    for (i, &r) in olds.iter().enumerate() {
        c.f[r] = cur(first + 1 + i as u32).read_fpu(c).0;
    }
    c.f[acc] = (t * FpuF32(c.f[acc])).0;
    c.f[0] = (k * FpuF32(c.f[0])).0;
    for (i, &r) in olds.iter().enumerate() {
        let dst = if tail && i + 1 == olds.len() { 3 } else { r };
        c.f[dst] = (k * FpuF32(c.f[r])).0;
    }
    c.f[acc] = (FpuF32(c.f[acc]) + FpuF32(c.f[0])).0;
    cur(first).write_fpu(c, FpuF32(c.f[acc]));
    c.f[1] = val(first).read_fpu(c).0;
    cur(first).off(4).write_fpu(c, FpuF32(c.f[1]));
    for (i, &r) in olds.iter().enumerate() {
        let n = first + 1 + i as u32;
        if tail && i + 1 == olds.len() {
            c.f[0] = val(n).read_fpu(c).0;
            c.f[20] = (t * FpuF32(c.f[0]) + FpuF32(c.f[3])).0;
            cur(n).write_fpu(c, FpuF32(c.f[20]));
            c.f[0] = val(n).read_fpu(c).0;
            cur(n).off(4).write_fpu(c, FpuF32(c.f[0]));
        } else {
            c.f[0] = val(n).read_fpu(c).0;
            c.f[0] = (t * FpuF32(c.f[0]) + FpuF32(c.f[r])).0;
            cur(n).write_fpu(c, FpuF32(c.f[0]));
            c.f[1] = val(n).read_fpu(c).0;
            cur(n).off(4).write_fpu(c, FpuF32(c.f[1]));
        }
    }
}

/// `void tWPIGD_Weather::Blend(const tWPIGD_Weather* target, float t)`
/// (0x2bd698): blends the weather's 19 settings toward a keyframe with the
/// eased weight `t^2`: each pair `{current, target}` (+0x08 + 8i) gets
/// `current = t^2 * key[i] + (1 - t^2) * current` and `target = key[i]`
/// (`key` from target+0x04). Calls its vtable hook (+0x218/+0x21c) before
/// and after. Frame 0x40 (s0, s1, f20); 180 cycles charged at entry.
pub fn twpigd_weather_blend(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    use weather_blend::*;
    guest(c, |c| {
        c.cycles += 180;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        let t = FpuF32(c.f[12]);
        c.f[20] = t.0;
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(VPTR)).lw(c);
        c.f[20] = (t * t).0;
        wb_hook(c, RET_PRE)?;
        c.g[AT][0] = 0x3f80_0000;
        c.f[3] = (FpuF32::ONE - FpuF32(c.f[20])).0;
        // Pairs 0..=6 (+0x08..+0x38), 7..=13 (+0x40..+0x70), 14..=18
        // (+0x78..+0x98), as the compiler grouped them.
        wb_group(c, 0, 1, &[2, 4, 5, 6, 7, 8], false);
        wb_group(c, 7, 2, &[4, 5, 6, 7, 8, 9], false);
        wb_group(c, 14, 2, &[4, 5, 6, 1], true);
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(VPTR)).lw(c);
        wb_hook(c, RET_POST)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) fn ctor_simple_setup(c: &mut Cpu, seed: u32, lit_off: u32, salt: u64) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP.wrapping_sub(lit_off), r.next_u32());
        c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
    }

    #[test]
    fn twpigd_music_trigger_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bc6f8,
            twpigd_music_trigger_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0xc6f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x42fc), r.next_u32());
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            },
        );
    }

    #[test]
    fn twpigd_mix_ctor_matches() {
        check(ssx3_ee::s08::f_002bc728, twpigd_mix_ctor, 50, &|c, seed| {
            ctor_simple_setup(c, seed, 0x42f8, 0xc728);
        });
    }

    #[test]
    fn twpigd_ambience_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bc750,
            twpigd_ambience_ctor,
            50,
            &|c, seed| {
                ctor_simple_setup(c, seed, 0x42f4, 0xc750);
            },
        );
    }

    #[test]
    fn twpigd_speech_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bc778,
            twpigd_speech_ctor,
            50,
            &|c, seed| {
                ctor_simple_setup(c, seed, 0x42f0, 0xc778);
            },
        );
    }

    /// Setup for the multi-literal world-painter constructors: `lits` are the
    /// `gp`-relative literal offsets the ctor reads.
    pub(crate) fn ctor_multi_setup(c: &mut Cpu, seed: u32, salt: u64, lits: &[u32]) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        for &o in lits {
            c.bus.write32(GAME_GP.wrapping_sub(o), r.next_u32());
        }
        c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
    }

    #[test]
    fn twpigd_camera_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bc7a0,
            twpigd_camera_ctor,
            50,
            &|c, seed| {
                ctor_multi_setup(c, seed, 0xc7a0, &[0x42ec, 0x42e8]);
            },
        );
    }

    #[test]
    fn twpigd_fog_ctor_matches() {
        check(ssx3_ee::s08::f_002bc7c8, twpigd_fog_ctor, 50, &|c, seed| {
            ctor_multi_setup(
                c,
                seed,
                0xc7c8,
                &[0x42e4, 0x42e0, 0x42dc, 0x42d8, 0x42d4, 0x42d0],
            );
        });
    }

    #[test]
    fn twpigd_light_glow_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bc830,
            twpigd_light_glow_ctor,
            50,
            &|c, seed| {
                ctor_multi_setup(c, seed, 0xc830, &[0x42cc]);
            },
        );
    }

    #[test]
    fn twpigd_screen_tint_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bc890,
            twpigd_screen_tint_ctor,
            50,
            &|c, seed| {
                ctor_multi_setup(c, seed, 0xc890, &[0x42c8]);
            },
        );
    }

    #[test]
    fn twpigd_sky_box_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bc8e8,
            twpigd_sky_box_ctor,
            50,
            &|c, seed| {
                ctor_multi_setup(c, seed, 0xc8e8, &[0x42c4]);
            },
        );
    }

    #[test]
    fn twpigd_sun_ctor_matches() {
        check(ssx3_ee::s08::f_002bc910, twpigd_sun_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xc910, &[0x42c0]);
        });
    }

    #[test]
    fn twpigd_surface_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bc980,
            twpigd_surface_ctor,
            50,
            &|c, seed| {
                ctor_multi_setup(c, seed, 0xc980, &[0x42bc]);
            },
        );
    }

    #[test]
    fn twpigd_weather_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bcaf8,
            twpigd_weather_ctor,
            50,
            &|c, seed| {
                ctor_multi_setup(c, seed, 0xcaf8, &[0x42b4, 0x42b0]);
            },
        );
    }

    #[test]
    fn twpigd_danger_ctor_matches() {
        check(
            ssx3_ee::s08::f_002bcbd0,
            twpigd_danger_ctor,
            50,
            &|c, seed| {
                ctor_multi_setup(c, seed, 0xcbd0, &[0x42ac]);
            },
        );
    }

    #[test]
    fn weather_save_state_matches() {
        covered(
            ssx3_ee::s08::f_002bfeb0__generated,
            weather_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x5a17, 38);
                c.pc = 0x002b_feb0;
                let owner = DATA + 0x1_0000;
                let vt = DATA + 0x2_0000;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = owner as u64;
                c.bus.write32(owner, vt);
                c.bus
                    .write16(vt + 8, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0xc, FAKE + 0x10 * (r.next_u32() % 4));
            },
        );
    }
    use super::*;

    /// `tWPIGD_Weather_Blend`: the weather at a random data address with
    /// 19 random `{current, target}` pairs (an edge-case float on one pair
    /// in four), a keyframe with random values (on one seed in seven it
    /// overlaps the weather's pairs, which checks the load/store order),
    /// t (f12) in [-1.2, 1.2) or an edge case on one seed in five, and a
    /// vtable whose hook is a fake (random this-delta) that unwinds on
    /// either of the two calls on some seeds.
    #[test]
    fn twpigd_weather_blend_matches() {
        use crate::difftest::fakes::{covered, machine, ptr, FAKE};
        use weather_blend::*;
        covered(
            ee::s08::f_002bd698__generated,
            twpigd_weather_blend,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bd698, 2);
                c.pc = 0x002b_d698;
                let this = ptr(&mut r) & !0xfff;
                for i in 0..38 {
                    let v = if r.next_u32() % 4 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(500.0)
                    };
                    c.bus.write32(this + PAIRS + 4 * i, v);
                }
                let key = if seed % 7 == 0 {
                    this + 4 * (r.next_u32() % 16)
                } else {
                    let k = this + 0x400;
                    for i in 0..19 {
                        let v = if r.next_u32() % 8 == 0 {
                            r.edge_f32()
                        } else {
                            r.f32(500.0)
                        };
                        c.bus.write32(k + VALUES + 4 * i, v);
                    }
                    k
                };
                // The vtable clear of the blended data (the blend cannot
                // overwrite its hook slot before the second call).
                let vt = this + 0x800;
                c.bus
                    .write16(vt + VT_HOOK_DELTA, (r.next_u32() as i16 & !3) as u16);
                c.bus.write32(vt + VT_HOOK, FAKE);
                c.bus.write32(this + VPTR, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = key as u64;
                c.f[12] = if seed % 5 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(1.2)
                };
            },
        );
    }
}
