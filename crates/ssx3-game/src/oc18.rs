//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_4c08, world_trigger_instance_matches, "WorldTriggerInstance_Matches"),
    (0x002b_6900, world_trigger_manager_reset_played_list, "WorldTriggerManager_ResetPlayedList"),
    (0x002b_9100, audio_debug_printf, "Audio_DebugPrintf"),
    (0x002b_9ce0, bank_monitor_set_param_callback, "BankMonitor_SetParamCallback"),
    (0x002b_b130, cworld_painter_man_ctor, "cWorldPainterMan_ctor"),
    (0x002b_b370, wpfile_ctor, "WPFile_ctor"),
    (0x002b_c6f8, twpigd_music_trigger_ctor, "tWPIGD_MusicTrigger_ctor"),
    (0x002b_c728, twpigd_mix_ctor, "tWPIGD_Mix_ctor"),
    (0x002b_c750, twpigd_ambience_ctor, "tWPIGD_Ambience_ctor"),
    (0x002b_c778, twpigd_speech_ctor, "tWPIGD_Speech_ctor"),
    (0x002b_c7a0, twpigd_camera_ctor, "tWPIGD_Camera_ctor"),
    (0x002b_c7c8, twpigd_fog_ctor, "tWPIGD_Fog_ctor"),
    (0x002b_c830, twpigd_light_glow_ctor, "tWPIGD_LightGlow_ctor"),
    (0x002b_c890, twpigd_screen_tint_ctor, "tWPIGD_ScreenTint_ctor"),
    (0x002b_c8e8, twpigd_sky_box_ctor, "tWPIGD_SkyBox_ctor"),
    (0x002b_c910, twpigd_sun_ctor, "tWPIGD_Sun_ctor"),
    (0x002b_c980, twpigd_surface_ctor, "tWPIGD_Surface_ctor"),
    (0x002b_caf8, twpigd_weather_ctor, "tWPIGD_Weather_ctor"),
    (0x002b_cbd0, twpigd_danger_ctor, "tWPIGD_Danger_ctor"),
    (0x002c_16a8, quad_tree_node_set_leaf, "QuadTreeNode_SetLeaf"),
    (0x002c_16b8, quad_tree_node_set_children, "QuadTreeNode_SetChildren"),
    (0x002c_17c0, quad_tree_init_free_list, "QuadTree_InitFreeList"),
    (0x002c_18e8, quad_tree_free_children, "QuadTree_FreeChildren"),
    (0x002c_2338, memcard_ascii_char_to_sjis, "MemCard_AsciiCharToSjis"),
    (0x002c_2430, memcard_sjis_char_to_ascii, "MemCard_SjisCharToAscii"),
    (0x002c_51d0, cps2memcard_get_port_error, "cPS2MemCard_GetPortError"),
    (0x002c_5250, cps2memcard_clear_port_info, "cPS2MemCard_ClearPortInfo"),
    (0x002c_5320, cps2memcard_get_free_space_for_port, "cPS2MemCard_GetFreeSpaceForPort"),
    (0x002c_5570, cps2memcard_mark_save_invalid, "cPS2MemCard_MarkSaveInvalid"),
    (0x002c_63e8, cps2memcard_is_mc_error, "cPS2MemCard_IsMcError"),
    (0x002c_71c0, date_is_leap_year, "Date_IsLeapYear"),
    (0x002c_7328, time_add_seconds, "Time_AddSeconds"),
    (0x002c_a258, menu_sys_item_ctor, "MenuSysItem_ctor"),
    (0x002c_a378, menu_sys_item_is_visible, "MenuSysItem_IsVisible"),
    (0x002c_a388, menu_sys_item_set_visible, "MenuSysItem_SetVisible"),
    (0x002c_a3b0, menu_sys_item_is_disabled, "MenuSysItem_IsDisabled"),
    (0x002c_aa58, menu_sys_page_ctor, "MenuSysPage_ctor"),
    (0x002c_be10, menu_sys_page_remove_all_items, "MenuSysPage_RemoveAllItems"),
    (0x002c_c070, menu_sys_reset, "MenuSys_Reset"),
    (0x002d_1af0, world_get_rider_count, "World_GetRiderCount"),
    (0x002d_1b08, world_get_rider, "World_GetRider"),
    (0x002d_1ce0, world_get_instances_man, "World_GetInstancesMan"),
    (0x002d_21c0, ssx_stats_man_ctor, "SSXStatsMan_ctor"),
    (0x002d_21d0, serialize_write_u32, "Serialize_WriteU32"),
    (0x002d_2210, serialize_write_u32_2, "Serialize_WriteU32_2"),
    (0x002d_23c8, serialize_write_u8, "Serialize_WriteU8"),
    (0x002d_2510, serialize_read_u32, "Serialize_ReadU32"),
    (0x002d_2558, serialize_read_u32_2, "Serialize_ReadU32_2"),
    (0x002d_2790, serialize_read_u8, "Serialize_ReadU8"),
    (0x002d_5598, avalanche_pack_bytes3, "Avalanche_PackBytes3"),
];

// ---------------------------------------------------------------------------
// World triggers.

/// `bool WorldTriggerInstance_Matches(WorldTriggerInstance* t, uint a, uint b,
/// uint c)` (0x2b4c08): whether the trigger's fields at +4/+8/+0x3c equal the
/// three arguments (0 if the first mismatch is at +4, 1 only if all match).
pub fn world_trigger_instance_matches(c: &mut Cpu) {
    let t = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = t.off(0x4).lw(c);
    if c.g[A1][0] != c.g[V0][0] {
        c.g[V0][0] = 0;
    } else {
        c.cycles += 3;
        c.g[V0][0] = t.off(0x8).lw(c);
        if c.g[A2][0] != c.g[V0][0] {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V1][0] = t.off(0x3c).lw(c);
            if c.g[A3][0] == c.g[V1][0] {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void WorldTriggerManager_ResetPlayedList(WorldTriggerManager* m)`
/// (0x2b6900): clear four words of the played list (the last two set to -1).
pub fn world_trigger_manager_reset_played_list(c: &mut Cpu) {
    c.cycles += 6;
    let m = Addr(abi::arg(c, 0));
    c.g[V0][0] = u64::MAX;
    m.off(0x330).sw(c, 0);
    m.off(0x32c).sw(c, c.g[V0][0]);
    m.off(0x324).sw(c, 0);
    m.off(0x328).sw(c, c.g[V0][0]);
    abi::ret(c);
}

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

/// `void BankMonitor_SetParamCallback(BankMonitor* m, void* cb, ...)`
/// (0x2b9ce0): store `cb` (a1) unaligned into the bank's parameter slot at
/// `m + (*(m+0x1c) << 3) + 0x40`.
pub fn bank_monitor_set_param_callback(c: &mut Cpu) {
    c.cycles += 11;
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x10);
    c.g[SP][0] = sx(sp);
    let cb = c.g[A1][0];
    Addr(sp).sd(c, cb);
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x1c).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    // ldl/ldr reload the callback value out of the frame.
    c.g[V0][0] = cb;
    Addr(c.g[A0][0] as u32).off(0x40).write_u64_unaligned(c, cb);
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World painter files.

/// `cWorldPainterMan* cWorldPainterMan_ctor(cWorldPainterMan* self)`
/// (0x2bb130): clear +4, return self.
pub fn cworld_painter_man_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `WPFile* WPFile_ctor(WPFile* f)` (0x2bb370): clear the two half-word fields
/// at +0/+2, set +4 to -1 and +8 to 0.
pub fn wpfile_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let f = Addr(abi::arg(c, 0));
    c.g[V0][0] = u64::MAX;
    f.off(0x8).sw(c, 0);
    f.off(0x4).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    f.sh(c, 0);
    f.off(0x2).sh(c, 0);
    abi::ret(c);
}

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
fn wpigd_simple_ctor(c: &mut Cpu, lit_off: u32, kind: u32) {
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
fn store_fregs(c: &mut Cpu, base: Addr, items: &[(u32, usize)]) {
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
fn obj_weather_stores(c: &mut Cpu, obj: Addr, desc: u64) {
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
// Quad tree.

/// `void QuadTreeNode_SetLeaf(QuadTreeNode* n, uint a, uint b)` (0x2c16a8):
/// store `b` at +0 and `a` at +4.
pub fn quad_tree_node_set_leaf(c: &mut Cpu) {
    c.cycles += 3;
    let n = Addr(abi::arg(c, 0));
    n.sw(c, c.g[A2][0]);
    n.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}

/// `void QuadTreeNode_SetChildren(QuadTreeNode* n, uint c0, uint c1, uint c2,
/// uint c3)` (0x2c16b8): update the node's low half-words from the child
/// indices (each doubled, low bit from the node).
pub fn quad_tree_node_set_children(c: &mut Cpu) {
    c.cycles += 23;
    let n = Addr(abi::arg(c, 0));
    c.g[T1][0] = n.off(0x2).lhu(c);
    c.g[A1][0] &= 0xffff;
    c.g[V1][0] = n.off(0x4).lhu(c);
    c.g[A2][0] &= 0xffff;
    c.g[V0][0] = n.off(0x6).lhu(c);
    c.g[A3][0] &= 0xffff;
    c.g[T0][0] &= 0xffff;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 1);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
    c.g[T1][0] &= 0x1;
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 1);
    c.g[V1][0] &= 0x1;
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 1);
    c.g[V0][0] &= 0x1;
    c.g[A2][0] |= c.g[T1][0];
    c.g[A3][0] |= c.g[V1][0];
    c.g[T0][0] |= c.g[V0][0];
    c.g[A1][0] |= 0x1;
    n.off(0x6).sh(c, c.g[T0][0]);
    n.sh(c, c.g[A1][0]);
    n.off(0x2).sh(c, c.g[A2][0]);
    n.off(0x4).sh(c, c.g[A3][0]);
    abi::ret(c);
}

/// `void QuadTree_InitFreeList(QuadTree* q)` (0x2c17c0): point the free-list
/// head and tail at the embedded list node at +0x28.
pub fn quad_tree_init_free_list(c: &mut Cpu) {
    c.cycles += 4;
    let q = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(q.0.wrapping_add(0x28));
    q.off(0x24).sw(c, c.g[V0][0]);
    q.off(0x20).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void QuadTree_FreeChildren(QuadTree* q, uint i0, uint i1, uint i2, uint
/// i3)` (0x2c18e8): splice the free-list node at +0x20, unlink the four child
/// entries and decrement the count at +0xc.
pub fn quad_tree_free_children(c: &mut Cpu) {
    c.cycles += 25;
    let q = Addr(abi::arg(c, 0));
    c.g[T1][0] = q.off(0x20).lw(c);
    c.g[A1][0] &= 0xffff;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    c.g[A2][0] &= 0xffff;
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] &= 0xffff;
    Addr(c.g[V0][0] as u32).sh(c, c.g[A2][0]);
    c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
    Addr(c.g[V1][0] as u32).sh(c, c.g[A3][0]);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T0][0] &= 0xffff;
    Addr(c.g[V0][0] as u32).sh(c, c.g[T0][0]);
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
    c.g[V0][0] = q.off(0x24).lw(c);
    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    Addr(c.g[T1][0] as u32).sh(c, c.g[V1][0]);
    c.g[A2][0] = q.off(0x24).lw(c);
    Addr(c.g[A2][0] as u32).sh(c, c.g[A1][0]);
    c.g[V0][0] = q.off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
    q.off(0xc).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory card / character conversion.

/// `short MemCard_AsciiCharToSjis(uchar ch)` (0x2c2338): map an ASCII byte to
/// its SJIS code through two static tables, 0 for unmapped bytes.
pub fn memcard_ascii_char_to_sjis(c: &mut Cpu) {
    c.cycles += 6;
    let ch = abi::arg(c, 0) & 0xff;
    c.g[A2][0] = ch as u64;
    c.g[V1][0] = 0;
    c.g[A0][0] = 0;
    c.g[V0][0] = sx(ch.wrapping_sub(0x20));
    c.g[V0][0] = (c.g[V0][0] < 0x10) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V1][0] = 0x1;
    } else {
        c.cycles += 4;
        c.g[V0][0] = sx(ch.wrapping_sub(0x30));
        let digits = (c.g[V0][0] < 0xa) as u64;
        c.g[V0][0] = sx(ch.wrapping_sub(0x3a));
        if digits == 0 {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0x7) as u64;
            let not_punct = c.g[V0][0] == 0;
            c.g[V0][0] = sx(ch.wrapping_sub(0x41));
            if !not_punct {
                c.cycles += 2;
                c.g[V1][0] = 0xb;
            } else {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let not_upper = c.g[V0][0] == 0;
                c.g[V0][0] = sx(ch.wrapping_sub(0x5b));
                if !not_upper {
                    c.cycles += 2;
                    c.g[A0][0] = 0x1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x6) as u64;
                    let not_lbracket = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(ch.wrapping_sub(0x61));
                    if !not_lbracket {
                        c.cycles += 2;
                        c.g[V1][0] = 0x25;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                        let not_lower = c.g[V0][0] == 0;
                        c.g[V0][0] = sx(ch.wrapping_sub(0x7b));
                        if !not_lower {
                            c.cycles += 2;
                            c.g[A0][0] = 0x2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V0][0] < 0x4) as u64;
                            c.g[V1][0] = 0x3f;
                            if c.g[V0][0] == 0 {
                                c.cycles += 2;
                                c.g[V0][0] = 0;
                                abi::ret(c);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    // End of the mapping chain: byte-swap the mapped half-word.
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        c.g[V1][0] = 0x440000;
        c.cycles += 9;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5940));
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lhu(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2)).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(ch));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.g[V0][0] &= 0xffff;
    } else {
        c.cycles += 8;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1f));
        c.g[V1][0] = 0x440000;
        c.g[V0][0] = sx(ch.wrapping_sub(c.g[V0][0] as u32));
        c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5950));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lhu(c);
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[V1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
    abi::ret(c);
}

/// `uchar MemCard_SjisCharToAscii(ushort sjis)` (0x2c2430): map a two-byte SJIS
/// code back to ASCII.
pub fn memcard_sjis_char_to_ascii(c: &mut Cpu) {
    c.cycles += 6;
    let code = abi::arg(c, 0);
    let lead = (code >> 8) & 0xff;
    let trail = code & 0xff;
    c.g[V0][0] = sx(code >> 8);
    c.g[A2][0] = lead as u64;
    c.g[A0][0] = trail as u64;
    c.g[A1][0] = 0;
    if trail != 0x81 {
        c.cycles += 3;
        if trail != 0x82 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        let mut a1: u32 = 0;
        if lead.wrapping_sub(0x4f) >= 0xa {
            c.cycles += 4;
            if lead.wrapping_sub(0x60) < 0x1a {
                a1 = 1;
            } else {
                c.cycles += 6;
                let v = lead.wrapping_add(0x7f) & 0xff;
                a1 = if 0x19 < v { 0 } else { 2 };
            }
            c.cycles += 1;
            a1 <<= 1;
        }
        c.g[A1][0] = a1 as u64;
        c.cycles += 10;
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = sx(gp.wrapping_add(0x7a0).wrapping_add(a1));
        c.g[A0][0] = sx(gp.wrapping_add(0x7a1));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
        let a1b = a1.wrapping_add(gp.wrapping_add(0x7a1));
        c.g[A1][0] = sx(a1b);
        c.g[V0][0] = Addr(a1b).lbu(c);
        c.g[V1][0] = sx(lead.wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32)) & 0xff;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5998).wrapping_add(lead));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x40)).lbu(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory-card port accessors.

/// `int cPS2MemCard_GetPortError(cPS2MemCard* mc, int port)` (0x2c51d0):
/// classify the port's status word into an error code (-10 when unknown).
pub fn cps2memcard_get_port_error(c: &mut Cpu) {
    c.cycles += 8;
    let base = c.g[A0][0] as u32;
    let port = c.g[A1][0] as u32;
    c.g[V0][0] = 0x14;
    c.g[A2][0] = c.g[A0][0];
    let prod = abi::mult(c, port, 0x14);
    c.g[V1][0] = sx(prod);
    c.g[V0][0] = sx(prod.wrapping_add(base));
    let e = Addr(c.g[V0][0] as u32).off(0x184).lw(c);
    c.g[A0][0] = e;
    c.g[V1][0] = ((e as i64) < -7) as u64;
    c.g[V0][0] = ((e as i64) < -10) as u64;
    let retry = if c.g[V1][0] == 0 {
        // status >= -7
        c.cycles += 2;
        c.g[V1][0] = 0x14;
        if (e as i64) > 0 {
            false
        } else {
            c.cycles += 1;
            let p = abi::mult(c, c.g[A1][0] as u32, 0x14);
            c.g[V0][0] = sx(p);
            true
        }
    } else {
        c.cycles += 2;
        let v0_zero = (e as i64) >= -10;
        let below10k = (e as i64) < -10000;
        if v0_zero {
            false
        } else {
            c.cycles += 2;
            if !below10k {
                false
            } else {
                c.cycles += 2;
                c.g[V1][0] = 0x14;
                if (e as i64) < -10002 {
                    false
                } else {
                    c.cycles += 2;
                    let p = abi::mult(c, c.g[A1][0] as u32, 0x14);
                    c.g[V0][0] = sx(p);
                    true
                }
            }
        }
    };
    if !retry {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_fff6;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x184).lw(c);
    abi::ret(c);
}

/// `void cPS2MemCard_ClearPortInfo(cPS2MemCard* mc)` (0x2c5250): clear the two
/// words at the current port's +0x17c/+0x180.
pub fn cps2memcard_clear_port_info(c: &mut Cpu) {
    c.cycles += 10;
    let base = c.g[A0][0] as u32;
    let h = Addr(base).off(0xc).lw(c);
    c.g[V0][0] = h;
    c.g[A1][0] = 0x14;
    let p1 = abi::mult(c, h as u32, 0x14);
    c.g[V1][0] = sx(p1);
    c.g[V0][0] = sx(p1.wrapping_add(base));
    Addr(c.g[V0][0] as u32).off(0x17c).sw(c, 0);
    let h2 = Addr(base).off(0xc).lw(c);
    c.g[V1][0] = h2;
    let p2 = abi::mult(c, h2 as u32, 0x14);
    c.g[V0][0] = sx(p2);
    c.g[V1][0] = sx(p2.wrapping_add(base));
    Addr(c.g[V1][0] as u32).off(0x180).sw(c, 0);
    abi::ret(c);
}

/// `int cPS2MemCard_GetFreeSpaceForPort(cPS2MemCard* mc, int port)`
/// (0x2c5320): read the port's free-space word at +0x180.
pub fn cps2memcard_get_free_space_for_port(c: &mut Cpu) {
    c.cycles += 5;
    let base = c.g[A0][0] as u32;
    c.g[V0][0] = 0x14;
    let prod = abi::mult(c, c.g[A1][0] as u32, 0x14);
    c.g[V1][0] = sx(prod);
    c.g[A0][0] = sx(prod.wrapping_add(base));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x180).lw(c);
    abi::ret(c);
}

/// `void cPS2MemCard_MarkSaveInvalid(cPS2MemCard* mc, int port)`
/// (0x2c5570): set the port's invalid flag at +0x1a8.
pub fn cps2memcard_mark_save_invalid(c: &mut Cpu) {
    c.cycles += 6;
    let base = c.g[A0][0] as u32;
    c.g[V0][0] = 0x78;
    c.g[V1][0] = 0x1;
    let prod = abi::mult(c, c.g[A1][0] as u32, 0x78);
    c.g[A2][0] = sx(prod);
    c.g[A0][0] = sx(prod.wrapping_add(base));
    Addr(c.g[A0][0] as u32).off(0x1a8).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `bool cPS2MemCard_IsMcError(int err)` (0x2c63e8): whether `err` is in the
/// MC error range `(-0x270f, -0x2709)`.
pub fn cps2memcard_is_mc_error(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x270f));
    c.g[V0][0] = (c.g[V0][0] < 0x2706) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Date / time.

/// `bool Date_IsLeapYear(int year)` (0x2c71c0): the usual 4/100/400 rule.
pub fn date_is_leap_year(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A1][0] & 0x3;
    c.g[A0][0] = 0;
    let year = c.g[A1][0] as u32;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[V0][0] = 0x64;
        c.g[A0][0] = 0x1;
        let _ = abi::div(c, year, 0x64);
        c.cycles += 3;
        c.g[V1][0] = c.hi[0];
        c.g[V0][0] = 0x190;
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            let _ = abi::div(c, year, 0x190);
            c.cycles += 2;
            c.g[V1][0] = c.hi[0];
            c.g[A0][0] = (c.g[V1][0] < 0x1) as u64;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void Time_AddSeconds(int* hours, int* minutes, int* seconds, int extra,
/// int carry)` (0x2c7328): fold a second count back into wall-clock fields.
pub fn time_add_seconds(c: &mut Cpu) {
    c.cycles += 12;
    let a1 = c.g[A1][0] as u32;
    let a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0] as u32;
    c.g[V0][0] = Addr(a3).lw(c);
    c.g[T3][0] = 0x3c;
    c.g[T1][0] = Addr(a2).lw(c);
    c.g[T2][0] = 0xe10;
    let p = abi::mult(c, c.g[V0][0] as u32, 0xe10);
    c.g[V0][0] = sx(p);
    c.g[T5][0] = Addr(a1).lw(c);
    let p = abi::mult(c, c.g[T1][0] as u32, 0x3c);
    c.g[T1][0] = sx(p);
    c.g[A0][0] = 0x10000;
    c.g[A0][0] |= 0x5180;
    c.g[V1][0] = 0x10000;
    c.cycles += 25;
    c.g[V1][0] |= 0x5180;
    c.g[T4][0] = 0x18;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
    c.g[T7][0] = 0x3c;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T5][0] as u32));
    c.g[T1][0] = 0xe10;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    let (q, r) = abi::div(c, c.g[V0][0] as u32, 0x15180);
    c.g[T1][0] = 0x3c;
    c.g[T5][0] = 0x3c;
    c.g[V1][0] = sx(r);
    c.g[V0][0] = sx(q);
    let (q, _r) = abi::div(c, c.g[V1][0] as u32, 0xe10);
    c.g[T2][0] = sx(q);
    let (q, r) = abi::div(c, c.g[V1][0] as u32, 0x3c);
    c.g[V1][0] = sx(q);
    c.g[T6][0] = sx(r);
    let (_q, r) = abi::div(c, c.g[T2][0] as u32, 0x18);
    c.g[A0][0] = sx(r);
    let (_q2, r2) = abi::div(c, c.g[V1][0] as u32, 0x3c);
    Addr(a3).sw(c, c.g[A0][0]);
    c.g[T0][0] = sx(r2);
    Addr(a2).sw(c, c.g[T0][0]);
    Addr(a1).sw(c, c.g[T6][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Menu system.

/// `MenuSysItem* MenuSysItem_ctor(MenuSysItem* item, void* data)` (0x2ca258):
/// set the vtable at +0x10, flags at +4, name at +8 and clear +0xc.
pub fn menu_sys_item_ctor(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x480000;
    c.g[V1][0] = 0x1;
    c.g[V0][0] = sx(0x480000u32.wrapping_add(0x6f28));
    let item = Addr(abi::arg(c, 0));
    item.off(0x4).sw(c, c.g[V1][0]);
    item.off(0x10).sw(c, c.g[V0][0]);
    c.g[V0][0] = c.g[A0][0];
    item.off(0x8).sw(c, c.g[A1][0]);
    item.off(0xc).sw(c, 0);
    abi::ret(c);
}

/// `bool MenuSysItem_IsVisible(MenuSysItem* item)` (0x2ca378): the visible bit.
pub fn menu_sys_item_is_visible(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    c.g[V0][0] &= 0x1;
    abi::ret(c);
}

/// `void MenuSysItem_SetVisible(MenuSysItem* item, bool visible)` (0x2ca388):
/// set or clear the visible bit.
pub fn menu_sys_item_set_visible(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    if c.g[A1][0] == 0 {
        c.cycles += 4;
        c.g[V1][0] = 0xffff_ffff_ffff_fffe;
        c.g[V0][0] &= c.g[V1][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] |= 0x1;
    }
    Addr(abi::arg(c, 0)).off(0x4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `bool MenuSysItem_IsDisabled(MenuSysItem* item)` (0x2ca3b0): the disabled bit.
pub fn menu_sys_item_is_disabled(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x4).lw(c);
    c.g[V0][0] &= 0x8;
    abi::ret(c);
}

/// `MenuSysPage* MenuSysPage_ctor(MenuSysPage* page)` (0x2caa58): set the
/// vtable at +0x12c and clear the four state words.
pub fn menu_sys_page_ctor(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V1][0] = 0x480000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x480000u32.wrapping_add(0x6ed0));
    let page = Addr(abi::arg(c, 0));
    page.sw(c, 0);
    page.off(0x12c).sw(c, c.g[V1][0]);
    page.off(0x4).sw(c, 0);
    page.off(0x8).sw(c, 0);
    page.off(0x124).sw(c, 0);
    abi::ret(c);
}

/// `void MenuSysPage_RemoveAllItems(MenuSysPage* page)` (0x2cbe10): clear the
/// item list head/tail.
pub fn menu_sys_page_remove_all_items(c: &mut Cpu) {
    c.cycles += 3;
    let page = Addr(abi::arg(c, 0));
    page.sw(c, 0);
    page.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `void MenuSys_Reset(MenuSys* menu)` (0x2cc070): reset the menu state.
pub fn menu_sys_reset(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 0xb;
    let menu = Addr(abi::arg(c, 0));
    menu.off(0x50).sw(c, c.g[V1][0]);
    menu.off(0x48).sw(c, c.g[V0][0]);
    menu.off(0x58).sw(c, 0);
    menu.off(0x54).sw(c, 0);
    menu.sw(c, 0);
    menu.off(0x64).sw(c, 0);
    menu.off(0x4c).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// World.

/// `int World_GetRiderCount(void)` (0x2d1af0): follow the world singleton to
/// the rider manager and return its count at +0x78.
pub fn world_get_rider_count(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let man = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = man;
    let sub = Addr(man as u32).off(0xc).lw(c);
    c.g[A0][0] = sub;
    c.g[V0][0] = Addr(sub as u32).off(0x78).lw(c);
    abi::ret(c);
}

/// `Rider* World_GetRider(World* w, int index)` (0x2d1b08): look up the rider
/// pointer table entry, returning null when the entry is absent.
pub fn world_get_rider(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x84).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c0));
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `void* World_GetInstancesMan(void)` (0x2d1ce0): follow the world singleton
/// to the instance manager at +0x88.
pub fn world_get_instances_man(c: &mut Cpu) {
    c.cycles += 4;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = world;
    let man = Addr(world as u32).off(0x84).lw(c);
    c.g[V1][0] = man;
    c.g[V0][0] = Addr(man as u32).off(0x88).lw(c);
    abi::ret(c);
}

/// `SSXStatsMan* SSXStatsMan_ctor(SSXStatsMan* self)` (0x2d21c0): publish the
/// singleton pointer at `gp + 0x910`.
pub fn ssx_stats_man_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr((c.g[GP][0] as u32).wrapping_add(0x910)).sw(c, c.g[A0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Serialization streams.

/// `void Serialize_WriteU32(void** stream, ...)` (0x2d21d0): write the u32 in
/// `f12` big-endian at `*stream` and advance the stream word by 4.
pub fn serialize_write_u32(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = abi::mfc1(FpuF32(c.f[12]));
    let stream = Addr(abi::arg(c, 0));
    c.g[A1][0] = stream.lw(c);
    c.g[A2][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 24);
    c.g[V1][0] = sx((c.g[A2][0] as u32) >> 8);
    c.g[A3][0] = sx((c.g[A2][0] as u32) >> 16);
    let buf = c.g[A1][0] as u32;
    Addr(buf).off(0x3).sb(c, c.g[V0][0]);
    Addr(buf).off(0x1).sb(c, c.g[V1][0]);
    c.g[V0][0] = 0x4;
    Addr(buf).off(0x2).sb(c, c.g[A3][0]);
    Addr(buf).sb(c, c.g[A2][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x4));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void Serialize_WriteU32_2(void** stream, uint value)` (0x2d2210): write
/// `value` big-endian and advance the stream word by 4.
pub fn serialize_write_u32_2(c: &mut Cpu) {
    c.cycles += 13;
    let stream = Addr(abi::arg(c, 0));
    c.g[A2][0] = stream.lw(c);
    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 24);
    c.g[V1][0] = sx((c.g[A1][0] as u32) >> 8);
    c.g[A3][0] = sx((c.g[A1][0] as u32) >> 16);
    let buf = c.g[A2][0] as u32;
    Addr(buf).off(0x3).sb(c, c.g[V0][0]);
    Addr(buf).off(0x1).sb(c, c.g[V1][0]);
    c.g[V0][0] = 0x4;
    Addr(buf).off(0x2).sb(c, c.g[A3][0]);
    Addr(buf).sb(c, c.g[A1][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x4));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void Serialize_WriteU8(void** stream, uchar value)` (0x2d23c8): write the
/// byte and advance the stream word by 1.
pub fn serialize_write_u8(c: &mut Cpu) {
    c.cycles += 7;
    let stream = Addr(abi::arg(c, 0));
    c.g[A2][0] = stream.lw(c);
    c.g[V0][0] = 0x1;
    Addr(c.g[A2][0] as u32).sb(c, c.g[A1][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x1));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void Serialize_ReadU32(void** stream, uint* out)` (0x2d2510): read a
/// big-endian u32 into `*out` and advance the stream word by 4.
pub fn serialize_read_u32(c: &mut Cpu) {
    c.cycles += 17;
    let stream = Addr(abi::arg(c, 0));
    c.g[A2][0] = stream.lw(c);
    c.g[V0][0] = 0x4;
    let buf = c.g[A2][0] as u32;
    c.g[V1][0] = Addr(buf).off(0x3).lbu(c);
    c.g[A3][0] = Addr(buf).off(0x2).lbu(c);
    c.g[T0][0] = Addr(buf).off(0x1).lbu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
    c.g[T1][0] = Addr(buf).lbu(c);
    c.g[V1][0] |= c.g[A3][0];
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 8);
    c.g[V1][0] |= c.g[T0][0];
    c.g[A3][0] = c.g[V1][0] | c.g[T1][0];
    Addr(abi::arg(c, 1)).sw(c, c.g[A3][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x4));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void Serialize_ReadU32_2(void** stream, uint* out)` (0x2d2558): as
/// `Serialize_ReadU32` with a different scratch-register allocation.
pub fn serialize_read_u32_2(c: &mut Cpu) {
    c.cycles += 17;
    let stream = Addr(abi::arg(c, 0));
    c.g[V1][0] = stream.lw(c);
    c.g[V0][0] = 0x4;
    let buf = c.g[V1][0] as u32;
    c.g[A2][0] = Addr(buf).off(0x3).lbu(c);
    c.g[A3][0] = Addr(buf).off(0x2).lbu(c);
    c.g[T0][0] = Addr(buf).off(0x1).lbu(c);
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
    c.g[T1][0] = Addr(buf).lbu(c);
    c.g[A2][0] |= c.g[A3][0];
    c.g[T0][0] = sx((c.g[T0][0] as u32) << 8);
    c.g[A2][0] |= c.g[T0][0];
    c.g[A2][0] |= c.g[T1][0];
    Addr(abi::arg(c, 1)).sw(c, c.g[A2][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x4));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void Serialize_ReadU8(void** stream, uchar* out)` (0x2d2790): read a byte
/// into `*out` and advance the stream word by 1.
pub fn serialize_read_u8(c: &mut Cpu) {
    c.cycles += 8;
    let stream = Addr(abi::arg(c, 0));
    c.g[V1][0] = stream.lw(c);
    c.g[V0][0] = 0x1;
    c.g[A2][0] = Addr(c.g[V1][0] as u32).lbu(c);
    Addr(abi::arg(c, 1)).sb(c, c.g[A2][0]);
    c.g[V1][0] = sx((stream.lw(c) as u32).wrapping_add(0x1));
    stream.sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Avalanche compression.

/// `void Avalanche_PackBytes3(uchar* dst, float a, float b, float c)`
/// (0x2d5598): truncate three floats to int and pack the low bytes.
pub fn avalanche_pack_bytes3(c: &mut Cpu) {
    c.cycles += 10;
    let x = FpuF32(c.f[12]).to_i32();
    c.f[0] = x as u32;
    c.g[A1][0] = x as i64 as u64;
    let y = FpuF32(c.f[13]).to_i32();
    c.f[0] = y as u32;
    c.g[V0][0] = y as i64 as u64;
    let z = FpuF32(c.f[14]).to_i32();
    c.f[0] = z as u32;
    c.g[V1][0] = z as i64 as u64;
    let dst = Addr(abi::arg(c, 0));
    dst.sb(c, c.g[A1][0]);
    dst.off(0x1).sb(c, c.g[V0][0]);
    dst.off(0x2).sb(c, c.g[V1][0]);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn world_trigger_instance_matches_matches() {
        check(ssx3_ee::s08::f_002b4c08, world_trigger_instance_matches, 300, &|c, seed| {
            let mut r = Rng(0x4c08 ^ seed as u64);
            scramble(c, &mut r);
            let a = r.next_u32();
            let b = r.next_u32();
            let d = r.next_u32();
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x4, a);
            c.bus.write32(DATA + 0x8, b);
            c.bus.write32(DATA + 0x3c, d);
            match seed % 4 {
                0 => {
                    c.g[A1][0] = sx(a).wrapping_add(1);
                    c.g[A2][0] = sx(b);
                    c.g[A3][0] = sx(d);
                }
                1 => {
                    c.g[A1][0] = sx(a);
                    c.g[A2][0] = sx(b).wrapping_add(1);
                    c.g[A3][0] = sx(d);
                }
                2 => {
                    c.g[A1][0] = sx(a);
                    c.g[A2][0] = sx(b);
                    c.g[A3][0] = sx(d);
                }
                _ => {
                    c.g[A1][0] = sx(a);
                    c.g[A2][0] = sx(b);
                    c.g[A3][0] = sx(d).wrapping_add(1);
                }
            }
        });
    }

    #[test]
    fn world_trigger_manager_reset_played_list_matches() {
        check(ssx3_ee::s08::f_002b6900, world_trigger_manager_reset_played_list, 50, &|c, seed| {
            let mut r = Rng(0x6900 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for k in 0..8u32 {
                c.bus.write32(DATA + 0x320 + 4 * k, r.next_u32());
            }
        });
    }

    #[test]
    fn audio_debug_printf_matches() {
        check(ssx3_ee::s08::f_002b9100, audio_debug_printf, 100, &|c, seed| {
            let mut r = Rng(0x9100 ^ seed as u64);
            scramble(c, &mut r);
        });
    }

    #[test]
    fn bank_monitor_set_param_callback_matches() {
        check(ssx3_ee::s08::f_002b9ce0, bank_monitor_set_param_callback, 200, &|c, seed| {
            let mut r = Rng(0x9ce0 ^ seed as u64);
            scramble(c, &mut r);
            let base = DATA + (seed % 8);
            c.g[A0][0] = base as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(base + 0x1c, r.next_u32() % 8);
        });
    }

    #[test]
    fn cworld_painter_man_ctor_matches() {
        check(ssx3_ee::s08::f_002bb130, cworld_painter_man_ctor, 50, &|c, seed| {
            let mut r = Rng(0xb130 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    #[test]
    fn wpfile_ctor_matches() {
        check(ssx3_ee::s08::f_002bb370, wpfile_ctor, 50, &|c, seed| {
            let mut r = Rng(0xb370 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    fn ctor_simple_setup(c: &mut Cpu, seed: u32, lit_off: u32, salt: u64) {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP.wrapping_sub(lit_off), r.next_u32());
        c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
    }

    #[test]
    fn twpigd_music_trigger_ctor_matches() {
        check(ssx3_ee::s08::f_002bc6f8, twpigd_music_trigger_ctor, 50, &|c, seed| {
            let mut r = Rng(0xc6f8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x42fc), r.next_u32());
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    #[test]
    fn twpigd_mix_ctor_matches() {
        check(ssx3_ee::s08::f_002bc728, twpigd_mix_ctor, 50, &|c, seed| {
            ctor_simple_setup(c, seed, 0x42f8, 0xc728);
        });
    }

    #[test]
    fn twpigd_ambience_ctor_matches() {
        check(ssx3_ee::s08::f_002bc750, twpigd_ambience_ctor, 50, &|c, seed| {
            ctor_simple_setup(c, seed, 0x42f4, 0xc750);
        });
    }

    #[test]
    fn twpigd_speech_ctor_matches() {
        check(ssx3_ee::s08::f_002bc778, twpigd_speech_ctor, 50, &|c, seed| {
            ctor_simple_setup(c, seed, 0x42f0, 0xc778);
        });
    }

    /// Setup for the multi-literal world-painter constructors: `lits` are the
    /// `gp`-relative literal offsets the ctor reads.
    fn ctor_multi_setup(c: &mut Cpu, seed: u32, salt: u64, lits: &[u32]) {
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
        check(ssx3_ee::s08::f_002bc7a0, twpigd_camera_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xc7a0, &[0x42ec, 0x42e8]);
        });
    }

    #[test]
    fn twpigd_fog_ctor_matches() {
        check(ssx3_ee::s08::f_002bc7c8, twpigd_fog_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xc7c8, &[0x42e4, 0x42e0, 0x42dc, 0x42d8, 0x42d4, 0x42d0]);
        });
    }

    #[test]
    fn twpigd_light_glow_ctor_matches() {
        check(ssx3_ee::s08::f_002bc830, twpigd_light_glow_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xc830, &[0x42cc]);
        });
    }

    #[test]
    fn twpigd_screen_tint_ctor_matches() {
        check(ssx3_ee::s08::f_002bc890, twpigd_screen_tint_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xc890, &[0x42c8]);
        });
    }

    #[test]
    fn twpigd_sky_box_ctor_matches() {
        check(ssx3_ee::s08::f_002bc8e8, twpigd_sky_box_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xc8e8, &[0x42c4]);
        });
    }

    #[test]
    fn twpigd_sun_ctor_matches() {
        check(ssx3_ee::s08::f_002bc910, twpigd_sun_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xc910, &[0x42c0]);
        });
    }

    #[test]
    fn twpigd_surface_ctor_matches() {
        check(ssx3_ee::s08::f_002bc980, twpigd_surface_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xc980, &[0x42bc]);
        });
    }

    #[test]
    fn twpigd_weather_ctor_matches() {
        check(ssx3_ee::s08::f_002bcaf8, twpigd_weather_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xcaf8, &[0x42b4, 0x42b0]);
        });
    }

    #[test]
    fn twpigd_danger_ctor_matches() {
        check(ssx3_ee::s08::f_002bcbd0, twpigd_danger_ctor, 50, &|c, seed| {
            ctor_multi_setup(c, seed, 0xcbd0, &[0x42ac]);
        });
    }

    #[test]
    fn quad_tree_node_set_leaf_matches() {
        check(ssx3_ee::s08::f_002c16a8, quad_tree_node_set_leaf, 50, &|c, seed| {
            let mut r = Rng(0x16a8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }

    #[test]
    fn quad_tree_node_set_children_matches() {
        check(ssx3_ee::s08::f_002c16b8, quad_tree_node_set_children, 200, &|c, seed| {
            let mut r = Rng(0x16b8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for (reg, v) in [(A1, r.next_u32()), (A2, r.next_u32()), (A3, r.next_u32()), (T0, r.next_u32())] {
                c.g[reg][0] = ((v as u64) << 32) | v as u64;
            }
            for k in 0..4u32 {
                c.bus.write16(DATA + 2 * k, r.next_u32() as u16);
            }
        });
    }

    #[test]
    fn quad_tree_init_free_list_matches() {
        check(ssx3_ee::s08::f_002c17c0, quad_tree_init_free_list, 50, &|c, seed| {
            let mut r = Rng(0x17c0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + (seed % 8)) as u64;
        });
    }

    #[test]
    fn quad_tree_free_children_matches() {
        check(ssx3_ee::s08::f_002c18e8, quad_tree_free_children, 200, &|c, seed| {
            let mut r = Rng(0x18e8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for reg in [A1, A2, A3, T0] {
                c.g[reg][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            }
            c.bus.write32(DATA + 0x20, DATA + 0x400);
            c.bus.write32(DATA + 0x24, DATA + 0x600);
            c.bus.write32(DATA + 0xc, r.next_u32());
            c.bus.write32(DATA + 0x400, r.next_u32());
            c.bus.write32(DATA + 0x600, r.next_u32() % 8);
        });
    }

    /// Fill the two static SJIS conversion tables around 0x445940.
    fn sjis_tables(c: &mut Cpu, r: &mut Rng) {
        for k in 0..0x60u32 {
            c.bus.write16(0x0044_5940 + 2 * k, r.next_u32() as u16);
        }
    }

    #[test]
    fn memcard_ascii_char_to_sjis_matches() {
        check(ssx3_ee::s08::f_002c2338, memcard_ascii_char_to_sjis, 300, &|c, seed| {
            let mut r = Rng(0x2338 ^ seed as u64);
            scramble(c, &mut r);
            sjis_tables(c, &mut r);
            let ch = match seed % 9 {
                0 => 0x20 + (r.next_u32() % 0x10),
                1 => 0x30 + (r.next_u32() % 0xa),
                2 => 0x3a + (r.next_u32() % 7),
                3 => 0x41 + (r.next_u32() % 0x1a),
                4 => 0x5b + (r.next_u32() % 6),
                5 => 0x61 + (r.next_u32() % 0x1a),
                6 => 0x7b + (r.next_u32() % 4),
                7 => r.next_u32() % 0x20,
                _ => r.next_u32() & 0xff,
            };
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | ch as u64;
        });
    }

    #[test]
    fn memcard_sjis_char_to_ascii_matches() {
        check(ssx3_ee::s08::f_002c2430, memcard_sjis_char_to_ascii, 300, &|c, seed| {
            let mut r = Rng(0x2430 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            sjis_tables(c, &mut r);
            for k in 0..0x20u32 {
                c.bus.write8(GAME_GP + 0x7a0 + k, r.next_u32() as u8);
            }
            let (lead, trail) = match seed % 5 {
                0 => (r.next_u32() as u8, 0x81u8),
                1 => (r.next_u32() as u8, 0x80u8),
                _ => {
                    let lead = match seed % 6 {
                        0 => (0x4f + (r.next_u32() % 10)) as u8,
                        1 => (0x60 + (r.next_u32() % 0x1a)) as u8,
                        2 => (r.next_u32() % 0x4f) as u8,
                        3 => (0x59 + (r.next_u32() % 7)) as u8,
                        4 => (0x7a + (r.next_u32() % 6)) as u8,
                        _ => r.next_u32() as u8,
                    };
                    (lead, 0x82u8)
                }
            };
            let code = ((lead as u32) << 8) | trail as u32;
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | code as u64;
        });
    }

    #[test]
    fn cps2memcard_get_port_error_matches() {
        check_lohi(ssx3_ee::s08::f_002c51d0, cps2memcard_get_port_error, 300, &|c, seed| {
            let mut r = Rng(0x51d0 ^ seed as u64);
            scramble(c, &mut r);
            let port = seed % 4;
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            c.g[A1][0] = port as u64;
            let e = match seed % 12 {
                0 => 0,
                1 => 1,
                2 => (-7i32) as u32,
                3 => (-8i32) as u32,
                4 => (-10i32) as u32,
                5 => (-11i32) as u32,
                6 => (-10000i32) as u32,
                7 => (-10001i32) as u32,
                8 => (-10002i32) as u32,
                9 => (-10003i32) as u32,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + port * 0x14 + 0x184, e);
        });
    }

    #[test]
    fn cps2memcard_clear_port_info_matches() {
        check_lohi(ssx3_ee::s08::f_002c5250, cps2memcard_clear_port_info, 200, &|c, seed| {
            let mut r = Rng(0x5250 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0xc, r.next_u32() % 4);
        });
    }

    #[test]
    fn cps2memcard_get_free_space_for_port_matches() {
        check_lohi(ssx3_ee::s08::f_002c5320, cps2memcard_get_free_space_for_port, 200, &|c, seed| {
            let mut r = Rng(0x5320 ^ seed as u64);
            scramble(c, &mut r);
            let port = seed % 4;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = port as u64;
            c.bus.write32(DATA + port * 0x14 + 0x180, r.next_u32());
        });
    }

    #[test]
    fn cps2memcard_mark_save_invalid_matches() {
        check_lohi(ssx3_ee::s08::f_002c5570, cps2memcard_mark_save_invalid, 200, &|c, seed| {
            let mut r = Rng(0x5570 ^ seed as u64);
            scramble(c, &mut r);
            let port = seed % 4;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = port as u64;
            c.bus.write32(DATA + port * 0x78 + 0x1a8, r.next_u32());
        });
    }

    #[test]
    fn cps2memcard_is_mc_error_matches() {
        check(ssx3_ee::s08::f_002c63e8, cps2memcard_is_mc_error, 300, &|c, seed| {
            let mut r = Rng(0x63e8 ^ seed as u64);
            scramble(c, &mut r);
            let v = match seed % 4 {
                0 => (r.next_u32() % 0x3000).wrapping_sub(0x2800),
                1 => 0xffff_d8f1, // -9999
                2 => 0x7fff_ffff,
                _ => r.next_u32(),
            };
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | v as u64;
        });
    }

    #[test]
    fn date_is_leap_year_matches() {
        check_lohi(ssx3_ee::s08::f_002c71c0, date_is_leap_year, 300, &|c, seed| {
            let mut r = Rng(0x71c0 ^ seed as u64);
            scramble(c, &mut r);
            let year = match seed % 5 {
                0 => (r.next_u32() % 1000).wrapping_mul(4).wrapping_add(1),
                1 => (r.next_u32() % 1000).wrapping_mul(4),
                2 => (r.next_u32() % 100).wrapping_mul(100),
                3 => (r.next_u32() % 100).wrapping_mul(400),
                _ => r.next_u32(),
            };
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | year as u64;
        });
    }

    #[test]
    fn time_add_seconds_matches() {
        check_lohi(ssx3_ee::s08::f_002c7328, time_add_seconds, 200, &|c, seed| {
            let mut r = Rng(0x7328 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            c.g[A3][0] = (DATA + 0x300) as u64;
            c.g[T0][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x100, r.next_u32());
            c.bus.write32(DATA + 0x200, r.next_u32());
            c.bus.write32(DATA + 0x300, r.next_u32());
        });
    }

    #[test]
    fn menu_sys_item_ctor_matches() {
        check(ssx3_ee::s09::f_002ca258, menu_sys_item_ctor, 50, &|c, seed| {
            let mut r = Rng(0xa258 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }

    #[test]
    fn menu_sys_item_is_visible_matches() {
        check(ssx3_ee::s09::f_002ca378, menu_sys_item_is_visible, 50, &|c, seed| {
            let mut r = Rng(0xa378 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x4, r.next_u32());
        });
    }

    #[test]
    fn menu_sys_item_set_visible_matches() {
        check(ssx3_ee::s09::f_002ca388, menu_sys_item_set_visible, 100, &|c, seed| {
            let mut r = Rng(0xa388 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            if seed % 2 == 0 {
                c.g[A1][0] = 0;
            } else {
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 1);
            }
            c.bus.write32(DATA + 0x4, r.next_u32());
        });
    }

    #[test]
    fn menu_sys_item_is_disabled_matches() {
        check(ssx3_ee::s09::f_002ca3b0, menu_sys_item_is_disabled, 50, &|c, seed| {
            let mut r = Rng(0xa3b0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x4, r.next_u32());
        });
    }

    #[test]
    fn menu_sys_page_ctor_matches() {
        check(ssx3_ee::s09::f_002caa58, menu_sys_page_ctor, 50, &|c, seed| {
            let mut r = Rng(0xaa58 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    #[test]
    fn menu_sys_page_remove_all_items_matches() {
        check(ssx3_ee::s09::f_002cbe10, menu_sys_page_remove_all_items, 50, &|c, seed| {
            let mut r = Rng(0xbe10 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    #[test]
    fn menu_sys_reset_matches() {
        check(ssx3_ee::s09::f_002cc070, menu_sys_reset, 50, &|c, seed| {
            let mut r = Rng(0xc070 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    #[test]
    fn world_get_rider_count_matches() {
        check(ssx3_ee::s09::f_002d1af0, world_get_rider_count, 50, &|c, seed| {
            let mut r = Rng(0x1af0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            let sub = DATA + 0x300;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, man);
            c.bus.write32(man + 0xc, sub);
            c.bus.write32(sub + 0x78, r.next_u32());
        });
    }

    #[test]
    fn world_get_rider_matches() {
        check(ssx3_ee::s09::f_002d1b08, world_get_rider, 200, &|c, seed| {
            let mut r = Rng(0x1b08 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            let table = DATA + 0x400;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, man);
            c.bus.write32(man + 0xc, table);
            let idx = seed % 4;
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | idx as u64;
            for i in 0..4u32 {
                let e = DATA + 0x800 + i * 0x100;
                c.bus.write32(table + i * 4, e);
                let v = if i == idx && seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(e + 0x28, v);
            }
        });
    }

    #[test]
    fn world_get_instances_man_matches() {
        check(ssx3_ee::s09::f_002d1ce0, world_get_instances_man, 50, &|c, seed| {
            let mut r = Rng(0x1ce0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, man);
            c.bus.write32(man + 0x88, r.next_u32());
        });
    }

    #[test]
    fn ssx_stats_man_ctor_matches() {
        check(ssx3_ee::s09::f_002d21c0, ssx_stats_man_ctor, 50, &|c, seed| {
            let mut r = Rng(0x21c0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
        });
    }

    #[test]
    fn serialize_write_u32_matches() {
        check(ssx3_ee::s09::f_002d21d0, serialize_write_u32, 200, &|c, seed| {
            let mut r = Rng(0x21d0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.f[12] = r.next_u32();
        });
    }

    #[test]
    fn serialize_write_u32_2_matches() {
        check(ssx3_ee::s09::f_002d2210, serialize_write_u32_2, 200, &|c, seed| {
            let mut r = Rng(0x2210 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA, DATA + 0x100);
        });
    }

    #[test]
    fn serialize_write_u8_matches() {
        check(ssx3_ee::s09::f_002d23c8, serialize_write_u8, 200, &|c, seed| {
            let mut r = Rng(0x23c8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA, DATA + 0x100);
        });
    }

    #[test]
    fn serialize_read_u32_matches() {
        check(ssx3_ee::s09::f_002d2510, serialize_read_u32, 200, &|c, seed| {
            let mut r = Rng(0x2510 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA, DATA + 0x100);
            for k in 0..4u32 {
                c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn serialize_read_u32_2_matches() {
        check(ssx3_ee::s09::f_002d2558, serialize_read_u32_2, 200, &|c, seed| {
            let mut r = Rng(0x2558 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA, DATA + 0x100);
            for k in 0..4u32 {
                c.bus.write8(DATA + 0x100 + k, r.next_u32() as u8);
            }
        });
    }

    #[test]
    fn serialize_read_u8_matches() {
        check(ssx3_ee::s09::f_002d2790, serialize_read_u8, 200, &|c, seed| {
            let mut r = Rng(0x2790 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA, DATA + 0x100);
            c.bus.write8(DATA + 0x100, r.next_u32() as u8);
        });
    }

    #[test]
    fn avalanche_pack_bytes3_matches() {
        check(ssx3_ee::s09::f_002d5598, avalanche_pack_bytes3, 300, &|c, seed| {
            let mut r = Rng(0x5598 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (DATA + (seed % 8)) as u64;
            c.f[12] = r.edge_f32();
            c.f[13] = r.edge_f32();
            c.f[14] = r.edge_f32();
        });
    }
}
