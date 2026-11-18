//! Staging module oc435 (wave wave221): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc435`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0027_c958,
        script_rider_get_params,
        "ScriptRider_GetParams",
    ),
    (
        0x0012_b280,
        ai_load_anim_data_sources,
        "AI_LoadAnimDataSources",
    ),
    (0x0022_de98, section_man_apply_sky, "SectionMan_ApplySky"),
    (
        0x003a_dc80,
        t_w_script_cache_get_global_count,
        "tWScriptCache_GetGlobalCount",
    ),
    (
        0x003a_6c38,
        world_get_section_processes,
        "World_GetSectionProcesses",
    ),
    (
        0x003a_6c08,
        world_get_section_process_count,
        "World_GetSectionProcessCount",
    ),
    (
        0x003a_6ba8,
        world_get_section_shutdown_func_count,
        "World_GetSectionShutdownFuncCount",
    ),
    (
        0x0027_c078,
        script_data_get_bank_list,
        "ScriptData_GetBankList",
    ),
    (
        0x0027_a668,
        script_engine_get_instance_script,
        "ScriptEngine_GetInstanceScript",
    ),
    (
        0x0027_9ed8,
        script_engine_find_anim_bank,
        "ScriptEngine_FindAnimBank",
    ),
    (0x0015_ad58, rewards_file_load, "RewardsFile_Load"),
    (0x0035_3cf0, sky_box_set_section, "SkyBox_SetSection"),
    (
        0x0022_e180,
        section_man_select_peak_lighting,
        "SectionMan_SelectPeakLighting",
    ),
    (
        0x0035_7b10,
        multi_particle_man_remove_instance,
        "MultiParticleMan_RemoveInstance",
    ),
    (
        0x0017_6da8,
        default_3_chase_camera_algorithm_dtor,
        "Default_3ChaseCameraAlgorithm_dtor",
    ),
    (0x0023_0050, game_module_enter, "GameModule_Enter"),
    (
        0x0014_af10,
        be_bolt_int_restore_all_equip_flags,
        "BEBoltInt_RestoreAllEquipFlags",
    ),
    (0x003a_6ae0, world_load, "World_Load"),
    (0x001d_d530, fe_ui_input_dtor, "FEUIInput_dtor"),
    (0x0032_3098, input_parser_parse_all, "InputParser_ParseAll"),
];

/// `Callee` for a named generated function (direct call through its alias).
macro_rules! oc435_callee {
    ($addr:expr, $f:path) => {
        Callee { addr: $addr, f: $f }
    };
}

/// `int ScriptRider_GetParams(ScriptRider *self)` (0x27c958): the word at
/// `+0x20`.
pub fn script_rider_get_params(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x20)).lw(c);
    ret(c);
}

/// `void AI_LoadAnimDataSources(void)` (0x12b280): mounts the AI anim big
/// file (0x3ded50), then creates one anim data source (`AdfDataSource_ctor`
/// 0x311250 on a fresh 0x18-byte block, 0x317d70) for the one name in the
/// table at `gp - 0x1ff0`, stores it in the `gp + 0xd08` table, registers it
/// (0x314f30), runs the post hook (0x3150b0) and unmounts (0x3dedc0).
/// The loop runs once (its counter starts at 0 and is tested `<= 0` after
/// the increment). Frame 0x50 (s0..s2).
pub fn ai_load_anim_data_sources(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = 0x45_0000;
        frame.save(c, S0, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x7f58));
        frame.save(c, S2, 0x20);
        c.g[A2][0] = 0x64;
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.g[SP][0];
        c.g[S0][0] = 0;
        call_ptr(c, 0x003d_ed50, 0x0012_b2b0)?;
        c.g[S2][0] = 0x45_0000;
        c.g[S1][0] = sx(r32(c, GP).wrapping_sub(0x1ff0));
        c.g[A2][0] = 0;
        loop {
            c.cycles += 20;
            c.g[A3][0] = 0;
            c.g[A0][0] = 0x18;
            c.g[A1][0] = sx(r32(c, S2).wrapping_add(0x7f70));
            call_ptr(c, 0x0031_7d70, 0x0012_b2d0)?;
            c.g[A1][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            call_ptr(c, 0x0031_1250, 0x0012_b2e0)?;
            c.g[V1][0] = Addr(r32(c, GP).wrapping_add(0xd08)).lw(c);
            c.g[A0][0] = c.g[S0][0] & 0xff;
            c.g[A0][0] = sx(r32(c, A0) << 2);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0xd8c)).lw(c);
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            Addr(r32(c, V1)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0031_4f30, 0x0012_b308)?;
            let again = (c.g[S0][0] as i64) <= 0;
            c.g[A2][0] = 0;
            if again {
                if event_due(c) {
                    deliver_event_here(c, 0x0012_b2c0);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0xd8c)).lw(c);
        call_ptr(c, 0x0031_50b0, 0x0012_b318)?;
        c.g[A0][0] = Addr(r32(c, SP)).lw(c);
        c.g[A1][0] = 0x64;
        call_ptr(c, 0x003d_edc0, 0x0012_b324)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SectionMan` fields of `SectionMan_ApplySky`.
pub(crate) const SECTION_MAN_SKY_PENDING: u32 = 0x1a4;
pub(crate) const SECTION_MAN_SKY_ACTIVE: u32 = 0x1a8;
pub(crate) const SECTION_MAN_SKY_CURRENT: u32 = 0x1ac;
pub(crate) const SECTION_MAN_SKY_BOX: u32 = 0x1b0;

/// `void SectionMan_ApplySky(SectionMan *self)` (0x22de98): when a sky change
/// is active (`+0x1a8`) and pending (`+0x1a4`), unloads every section
/// 0x2c..0x30 other than the current sky (`+0x1ac`), then points the sky box
/// (`+0x1b0 -> +0x14`) at the section the sky table (0x442168, 16-byte
/// entries, word 1) names for the current sky, clearing it first when that
/// differs from the box's present section (`+0x10`); finally clears pending
/// and active and sets the current sky to -1. Frame 0x30 (s0, s1).
pub fn section_man_apply_sky(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_ACTIVE)).lw(c);
        let inactive = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !inactive {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_PENDING)).lw(c);
            let none_pending = c.g[V0][0] == 0;
            c.g[S0][0] = 0x2c;
            if !none_pending {
                c.cycles += 1;
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_CURRENT)).lw(c);
                loop {
                    c.cycles += 2;
                    let is_current = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    if !is_current {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S0][0];
                        oc435_callee!(0x0022_d598, ee::s05::SectionMan_RequestUnload)
                            .call(c, 0x0022_ded8)?;
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 49) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_CURRENT)).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_dec8);
                        }
                        continue;
                    }
                    break;
                }
                c.cycles += 11;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_CURRENT)).lw(c);
                c.g[V0][0] = 0x44_0000;
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_BOX)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x2168));
                c.g[V1][0] = sx(r32(c, V1) << 4);
                c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0x14)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[S0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
                let same = c.g[V1][0] == c.g[S0][0];
                c.g[V0][0] = u64::MAX;
                if !same {
                    c.cycles += 7;
                    call_ptr(c, 0x0035_3d98, 0x0022_df1c)?;
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_BOX)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
                    call_ptr(c, 0x0035_3cf0, 0x0022_df2c)?;
                    c.g[V0][0] = u64::MAX;
                }
                c.cycles += 3;
                Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_PENDING)).sw(c, 0);
                Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_CURRENT)).sw(c, c.g[V0][0]);
                Addr(r32(c, S1).wrapping_add(SECTION_MAN_SKY_ACTIVE)).sw(c, 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int tWScriptCache_GetGlobalCount(tWScriptCache *self)` (0x3adc80): when
/// the cache kind (`+0`, halfword) is 3 and its data (`+8`) exists, the data's
/// word at `+0x10`; else 0.
pub fn t_w_script_cache_get_global_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A0)).lh(c);
    c.g[V1][0] = 3;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(8)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
            ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// Shared body of the three `World_GetSection*` wrappers (frame 0x10 and the
/// `mult` that scales the section index by the 12-byte entry size):
/// `callee(world->cache(+4)->sections(+4) + index * 12)`.
fn world_section_call(c: &mut Cpu, callee: Callee, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, 5, 5, 2);
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(4)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(4)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        callee.call(c, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `World_GetSectionProcesses(world, section)` (0x3a6c38):
/// `tWScriptCache_GetProcesses(section entry)`.
pub fn world_get_section_processes(c: &mut Cpu) {
    world_section_call(
        c,
        oc435_callee!(0x003a_dd10, ee::s13::tWScriptCache_GetProcesses),
        0x003a_6c58,
    )
}

/// `World_GetSectionProcessCount(world, section)` (0x3a6c08):
/// `tWScriptCache_GetProcessCount(section entry)`.
pub fn world_get_section_process_count(c: &mut Cpu) {
    world_section_call(
        c,
        oc435_callee!(0x003a_dce0, ee::s13::tWScriptCache_GetProcessCount),
        0x003a_6c28,
    )
}

/// `World_GetSectionShutdownFuncCount(world, section)` (0x3a6ba8):
/// `tWScriptCache_GetGlobalCount(section entry)`.
pub fn world_get_section_shutdown_func_count(c: &mut Cpu) {
    world_section_call(
        c,
        oc435_callee!(0x003a_dc80, ee::s13::tWScriptCache_GetGlobalCount),
        0x003a_6bc8,
    )
}

/// `void *ScriptData_GetBankList(ScriptData *d)` (0x27c078):
/// `InSeq_GetHeaderExtra(d)`.
pub fn script_data_get_bank_list(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(oc435_callee!(0x0027_4c38, ee::s07::InSeq_GetHeaderExtra)),
        0x0027_c088,
    )
}

/// `ScriptEngine_GetInstanceScript(engine, inst)` (0x27a668):
/// `ScriptEngine_GetSeqProcessScript(engine, inst)`.
pub fn script_engine_get_instance_script(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(oc435_callee!(
            0x0028_2bb0,
            ee::s07::ScriptEngine_GetSeqProcessScript
        )),
        0x0027_a678,
    )
}

/// `ScriptEngine_FindAnimBank(engine, id)` (0x279ed8):
/// `ScriptAnimBankMan_FindBank(engine + 0xa28, id)`.
pub fn script_engine_find_anim_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xa28));
        oc435_callee!(0x0027_5718, ee::s07::ScriptAnimBankMan_FindBank).call(c, 0x0027_9ee8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void RewardsFile_Load(RewardsFile **out, int a1)` (0x15ad58): with `a1 == 0`
/// reads the rewards file (size 0x317f60, buffer 0x317e30, load 0x3e1bb8) and
/// parses it (`RewardsFile_Parse(out, data)`), result -1; otherwise starts the
/// async bolt file acquire (0x19da10, world + 0x7c + 0x1a70); stores the
/// result in `*out`. Frame 0x40 (s0..s2).
pub fn rewards_file_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        let async_load = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if !async_load {
            c.cycles += 19;
            c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x1e80)).lw(c);
            call_ptr(c, 0x0031_7f60, 0x0015_ad7c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0x46_0000;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x5488));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0015_ad98)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x1e80)).lw(c);
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x003e_1bb8, 0x0015_adac)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S2][0];
            oc435_callee!(0x0015_a818, ee::s01::RewardsFile_Parse).call(c, 0x0015_adb8)?;
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 9;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
            c.g[T0][0] = c.g[A1][0];
            c.g[A2][0] = Addr(r32(c, GP).wrapping_sub(0x1e80)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x7c)).lw(c);
            c.g[A3][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1a70));
            call_ptr(c, 0x0019_da10, 0x0015_ade4)?;
        }
        c.cycles += 7;
        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SkyBox_SetSection(SkyBox *self, int section)` (0x353cf0): stores the
/// section at `+0x10`; a negative one clears the box (`SkyBox_Clear`), else
/// looks the section's world mesh up (via 0x2d1bd8 -> manager, entry `section *
/// 0x58`) and, when it has meshes (`+0x26 > 0`) and the table entry
/// `table[section & 0xff]` exists, stores `(entry->+0x1c->word >> 8) * 4` (or
/// 0) at `+0x14`. Frame 0x30 (s0, s1).
pub fn sky_box_set_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let non_negative = (c.g[S0][0] as i64) >= 0;
        Addr(r32(c, S1).wrapping_add(0x10)).sw(c, c.g[S0][0]);
        if !non_negative {
            c.cycles += 4;
            oc435_callee!(0x0035_3d98, ee::s11::SkyBox_Clear).call(c, 0x0035_3d18)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 11;
            call_ptr(c, 0x002d_1bd8, 0x0035_3d28)?;
            c.g[A2][0] = Addr(r32(c, V0)).lw(c);
            c.g[V0][0] = 0x58;
            c.g[V1][0] = Addr(r32(c, A2)).lw(c);
            c.mult(0, 2, 16, 2);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x14)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x26)).lh(c);
            let no_meshes = (c.g[V1][0] as i64) <= 0;
            c.g[A1][0] = c.g[S0][0] & 0xff;
            if !no_meshes {
                c.cycles += 6;
                c.g[V1][0] = Addr(r32(c, A2).wrapping_add(8)).lw(c);
                c.g[V0][0] = sx(r32(c, A1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[V0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x1c)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) >> 8);
                    let nonzero = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if !nonzero {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    }
                }
                c.cycles += 1;
                Addr(r32(c, S1).wrapping_add(0x14)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_SelectPeakLighting(SectionMan *self, int sky)` (0x22e180):
/// picks the irradiance name for the sky (0x4a2a20..0x4a2a40; skies 1/11/18,
/// 2/9/12/19, 3/6/15/20, 4/7/10/13/16/21 get their own, the rest the
/// default) through a `jr` table at 0x47b4e0 and stores
/// `cIrradianceDataBase_GetIDByName(0x5047f8, name)` (0x38ac50) at
/// `gp + 0x12d4`. Has an `__entries` twin: other entries go to it.
pub fn section_man_select_peak_lighting(c: &mut Cpu) {
    if c.pc != 0x0022_e180 {
        return ee::s05::f_0022e180__generated(c);
    }
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[A1][0] < 0x16) as u64;
        let out_of_range = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        // The name offset (from 0x4a0000) of the arm taken; `cycles` is the
        // block charge (5 for a table arm, 4 for the default).
        let (cycles, name_off) = if out_of_range {
            (4, 0x2a20)
        } else {
            c.cycles += 7;
            c.g[V0][0] = 0x48_0000;
            c.g[V1][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4b20));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let target = r32(c, A0);
            match target {
                0x0022_e1ac => (5, 0x2a28),
                0x0022_e1c0 => (5, 0x2a30),
                0x0022_e1d4 => (5, 0x2a38),
                0x0022_e1e8 => (5, 0x2a40),
                0x0022_e1fc => (4, 0x2a20),
                _ => {
                    c.jump_out(target, 0x0022_e180, 0x0022_e224);
                    return Ok(());
                }
            }
        };
        c.cycles += cycles;
        c.g[A0][0] = 0x50_0000;
        c.g[A1][0] = 0x4a_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x47f8));
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(name_off));
        c.cycles += 6;
        call_ptr(c, 0x0038_ac50, 0x0022_e214)?;
        Addr(r32(c, GP).wrapping_add(0x12d4)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MultiParticleMan_RemoveInstance(MultiParticleMan *m, int index, int
/// key)` (0x357b10): `MultiParticle_RemoveInstance(m->particles[index], key)`.
pub fn multi_particle_man_remove_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(r32(c, A0)).lw(c);
        oc435_callee!(0x0035_81f0, ee::s11::MultiParticle_RemoveInstance).call(c, 0x0035_7b2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Default_3ChaseCameraAlgorithm_dtor(self)` (0x176da8): sets the
/// vtable (0x45ca38) at `+0x10`, then runs the base destructor
/// (`ChaseCameraAlgorithm_dtor`, 0x162458).
pub fn default_3_chase_camera_algorithm_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x46_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x35c8));
        Addr(r32(c, V1).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0016_2458, 0x0017_6dc4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int GameModule_Enter(GameModule *self)` (0x230050): the per-event
/// enter hook: starts the world's bolt/anim managers (0x195d50), clears
/// `gp - 0x9e4`, runs 0x1f30d8 on `self->+0x94`, 0x278710, two virtual calls
/// on the object at `gp + 0x1158` (slots at `+0x88`/`+0x8c` and
/// `+0x20`/`+0x24` of its vtable), then 0x369ff8, 0x3a67e0 (`self->+0x10`),
/// 0x2b68d0 and 0x2867e8, and sets the three "entered" flags (`gp + 0x12d8`,
/// `0x1234`, `0x14e8`) to 1. Frame 0x20 (s0).
pub fn game_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x8c)).lw(c);
        call_ptr(c, 0x0019_5d50, 0x0023_006c)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x94)).lw(c);
        Addr(r32(c, GP).wrapping_sub(0x9e4)).sw(c, 0);
        call_ptr(c, 0x001f_30d8, 0x0023_0078)?;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x84c)).lw(c);
        call_ptr(c, 0x0027_8710, 0x0023_0080)?;
        // Two virtual calls on the object at gp + 0x1158 (vtable at +4: a
        // halfword `this` delta and the function).
        for (delta_off, slot_off, ret_addr) in
            [(0x88, 0x8c, 0x0023_0098), (0x20, 0x24, 0x0023_00b0)]
        {
            c.g[A1][0] = Addr(r32(c, GP).wrapping_add(0x1158)).lw(c);
            c.g[V0][0] = Addr(r32(c, A1).wrapping_add(4)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(delta_off)).lh(c);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(slot_off)).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, ret_addr)?;
        }
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x854)).lw(c);
        call_ptr(c, 0x0036_9ff8, 0x0023_00b8)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x10)).lw(c);
        call_ptr(c, 0x003a_67e0, 0x0023_00c0)?;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x21e4)).lw(c);
        call_ptr(c, 0x002b_68d0, 0x0023_00c8)?;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x410)).lw(c);
        call_ptr(c, 0x0028_67e8, 0x0023_00d0)?;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, GP).wrapping_add(0x12d8)).sw(c, c.g[V0][0]);
        Addr(r32(c, GP).wrapping_add(0x1234)).sw(c, c.g[V0][0]);
        Addr(r32(c, GP).wrapping_add(0x14e8)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void BEBoltInt_RestoreAllEquipFlags(void)` (0x14af10): for each of the 3
/// profiles (0x9b50 bytes apart from 0x4a6ca8) and 10 characters (0xf88
/// each), every flag entry (count at `+0x28c`, 4-byte entries from `+0x290`,
/// flags halfword at `+2`) gets bit 0x10 set when bit 4 is set, else
/// cleared. The loop counters live in a0 (profile), v1 (character), a2 (entry).
pub fn be_bolt_int_restore_all_equip_flags(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x4a_0000;
    c.g[A0][0] = 0;
    c.g[T4][0] = sx(r32(c, V0).wrapping_add(0x6ca8));
    c.g[T5][0] = 0x9b50;
    c.g[T3][0] = 0xf88;
    c.mult(0, 9, 4, 13);
    'profiles: loop {
        c.cycles += 4;
        c.g[V1][0] = 0;
        c.g[T2][0] = sx(r32(c, A0).wrapping_add(1));
        c.mult(0, 4, 3, 11);
        'characters: loop {
            c.cycles += 6;
            c.g[A3][0] = sx(r32(c, V1).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, T4)));
            c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x28c)).lw(c);
            let no_flags = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x290));
            if !no_flags {
                c.cycles += 2;
                c.g[A2][0] = c.g[V1][0];
                c.g[T0][0] = 0xffff_ffff_ffff_ffef;
                'entries: loop {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(2)).lhu(c);
                    c.g[A1][0] = c.g[V0][0] | 0x10;
                    c.g[V1][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V0][0] &= 4;
                    let clear = c.g[V0][0] == 0;
                    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
                    if clear {
                        c.cycles += 1;
                        Addr(r32(c, A0).wrapping_add(2)).sh(c, c.g[V1][0]);
                    } else {
                        c.cycles += 2;
                        Addr(r32(c, A0).wrapping_add(2)).sh(c, c.g[A1][0]);
                    }
                    c.cycles += 2;
                    let more = c.g[A2][0] != 0;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014_af58);
                        }
                        continue 'entries;
                    }
                    break 'entries;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[A3][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 10) as u64;
            let more_characters = c.g[V0][0] != 0;
            c.mult(0, 4, 3, 11);
            if more_characters {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af38);
                }
                continue 'characters;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[T2][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
            let more_profiles = c.g[V0][0] != 0;
            c.mult(0, 9, 4, 13);
            if more_profiles {
                if event_due(c) {
                    deliver_event_here(c, 0x0014_af28);
                }
                continue 'profiles;
            }
            c.cycles += 2;
            ret(c);
            return;
        }
    }
}

/// `void World_Load(World *w, a1, a2)` (0x3a6ae0): resets and loads the world
/// cache (`w->+0`), sizes the script caches by the streamer's section count
/// (`cWScriptCache_Alloc(w->+4, n)`, 0x2bb1d0 on `w->+8`), bracketed by a
/// 0x3182d8 / 0x318260 pair on a stack record. Frame 0x50 (s0..s2).
pub fn world_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, 0x0031_82d8, 0x003a_6b08)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        oc435_callee!(0x003a_7f90, ee::s13::cWorldCache_Reset).call(c, 0x003a_6b10)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        oc435_callee!(0x003a_7fc0, ee::s13::cWorldCache_Load).call(c, 0x003a_6b20)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        oc435_callee!(0x003a_9800, ee::s13::cWorldCache_StreamerGetSectionCount)
            .call(c, 0x003a_6b2c)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        oc435_callee!(0x003a_c800, ee::s13::cWScriptCache_Alloc).call(c, 0x003a_6b38)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        oc435_callee!(0x003a_9800, ee::s13::cWorldCache_StreamerGetSectionCount)
            .call(c, 0x003a_6b44)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x002b_b1d0, 0x003a_6b50)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x003a_6b5c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void FEUIInput_dtor(FEUIInput *self, int flags)` (0x1dd530): resets the
/// vtable (0x46dba8) at `+8`; with `flags & 1` deletes `self`
/// (`__builtin_delete`, 0x317e50).
pub fn fe_ui_input_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2458));
        c.g[A1][0] &= 1;
        let keep = c.g[A1][0] == 0;
        Addr(r32(c, A0).wrapping_add(8)).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x001d_d554)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputParser_ParseAll(InputParser *p)` (0x323098): reads the first
/// token, then parses statements until the parser's token kind (`+0x98`)
/// is 0 (end of input). Frame 0x20 (s0).
pub fn input_parser_parse_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        oc435_callee!(0x0032_1af0, ee::s10::InputParser_NextToken).call(c, 0x0032_30ac)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x98)).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                oc435_callee!(0x0032_30e0, ee::s10::InputParser_ParseStatement)
                    .call(c, 0x0032_30c0)?;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x98)).lw(c);
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0032_30b8);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
                break;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA};
    use ssx3_ee_rt::funcs::register_functions;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 200;

    /// Fixture objects live above the area the fakes' default results point
    /// into (`DATA..DATA + 0x40000`).
    const BASE: u32 = DATA + 0x4_0000;

    /// Stub controls of the parser test (`DATA + 0x200000` and up).
    const CTRL: u32 = DATA + 0x20_0000;

    fn w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// Set in the child process of the tests whose callees are real guest
    /// code (`call_ptr` targets): there the function table is empty, so every
    /// such callee lands in the logging fake, and the direct callees' override
    /// slots hold logging fakes too. A table swap in the shared test process
    /// would race with the other tests' `fresh()`.
    const OC435_CHILD_ENV: &str = "SSX3_OC435_CHILD";

    /// Run test `name` of this module in a child process (the same test
    /// binary, `--exact`), where `body` runs against the stub table.
    fn in_child(name: &str, body: fn()) {
        if std::env::var_os(OC435_CHILD_ENV).is_some() {
            install_stubs();
            return body();
        }
        let path = format!(
            "{}::{name}",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([path.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC435_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
    }

    /// Per-seed machine of the child tests: scrambled registers, the fake
    /// hooks and an empty function table.
    fn child_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        register_functions(&[]);
        r
    }

    /// A logging stub for a direct callee (reached through its override slot).
    macro_rules! oc435_stub {
        ($fake:ident, $prev:ident, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra)
            }
        };
    }

    oc435_stub!(stub_request_unload, PREV_REQUEST_UNLOAD, 0x0022_d598);
    oc435_stub!(stub_sky_box_clear, PREV_SKY_BOX_CLEAR, 0x0035_3d98);
    oc435_stub!(stub_rewards_parse, PREV_REWARDS_PARSE, 0x0015_a818);
    oc435_stub!(stub_world_cache_reset, PREV_WORLD_CACHE_RESET, 0x003a_7f90);
    oc435_stub!(stub_world_cache_load, PREV_WORLD_CACHE_LOAD, 0x003a_7fc0);
    oc435_stub!(stub_section_count, PREV_SECTION_COUNT, 0x003a_9800);
    oc435_stub!(
        stub_script_cache_alloc,
        PREV_SCRIPT_CACHE_ALLOC,
        0x003a_c800
    );

    static PREV_NEXT_TOKEN: AtomicUsize = AtomicUsize::new(0);
    static PREV_PARSE_STATEMENT: AtomicUsize = AtomicUsize::new(0);

    /// `InputParser_NextToken`: the token kind (`+0x98`) becomes the word at
    /// `CTRL`.
    fn stub_next_token(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        let p = c.g[A0][0] as u32;
        fakes::fake(c, 0x0032_1af0, ra);
        if c.pc == ra {
            let kind = c.bus.read32(CTRL);
            w(c, p.wrapping_add(0x98), kind);
        }
    }

    /// `InputParser_ParseStatement`: consumes one of the `CTRL + 4` rounds
    /// left; the last one ends the input (kind 0).
    fn stub_parse_statement(c: &mut Cpu) {
        let ra = c.g[RA][0] as u32;
        let p = c.g[A0][0] as u32;
        fakes::fake(c, 0x0032_30e0, ra);
        if c.pc == ra {
            let left = c.bus.read32(CTRL + 4);
            if left == 0 {
                w(c, p.wrapping_add(0x98), 0);
            } else {
                w(c, CTRL + 4, left - 1);
            }
        }
    }

    /// Child process only: put the logging stubs in the direct callees' slots.
    fn install_stubs() {
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D598),
            stub_request_unload,
            &PREV_REQUEST_UNLOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00353D98),
            stub_sky_box_clear,
            &PREV_SKY_BOX_CLEAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015A818),
            stub_rewards_parse,
            &PREV_REWARDS_PARSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A7F90),
            stub_world_cache_reset,
            &PREV_WORLD_CACHE_RESET,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A7FC0),
            stub_world_cache_load,
            &PREV_WORLD_CACHE_LOAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9800),
            stub_section_count,
            &PREV_SECTION_COUNT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003AC800),
            stub_script_cache_alloc,
            &PREV_SCRIPT_CACHE_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321AF0),
            stub_next_token,
            &PREV_NEXT_TOKEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003230E0),
            stub_parse_statement,
            &PREV_PARSE_STATEMENT,
        );
    }

    /// `ScriptRider_GetParams`: the word at `+0x20`.
    #[test]
    fn script_rider_get_params_matches() {
        check(
            ee::s07::f_0027c958__generated,
            script_rider_get_params,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27c958, 0);
                let this = BASE + 0x100;
                c.g[A0][0] = this as u64;
                w(c, this + 0x20, r.next_u32());
            },
        );
    }

    /// `tWScriptCache_GetGlobalCount`: kind 3 with data returns `data+0x10`;
    /// other kinds (also a sign-extended halfword) and missing data return 0.
    #[test]
    fn t_w_script_cache_get_global_count_matches() {
        check(
            ee::s13::f_003adc80__generated,
            t_w_script_cache_get_global_count,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3adc80, 0);
                let cache = BASE + 0x100;
                let data = BASE + 0x200;
                c.g[A0][0] = cache as u64;
                c.bus
                    .write16(cache, [3, 3, 3, 1, 0x8003][seed as usize % 5]);
                w(c, cache + 8, if seed % 3 == 0 { 0 } else { data });
                w(c, data + 0x10, r.next_u32());
            },
        );
    }

    /// The `World_GetSection*` fixture: world -> cache -> entry table of 12
    /// byte entries (kind halfword, data pointer at `+8`), the data blocks
    /// random. The callees (the `tWScriptCache_*` leaves) run for real.
    fn world_section_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let (world, cache, table) = (BASE + 0x100, BASE + 0x200, BASE + 0x300);
        w(c, world + 4, cache);
        w(c, cache + 4, table);
        for i in 0..4u32 {
            let entry = table + 12 * i;
            let data = BASE + 0x1000 + 0x100 * i;
            c.bus
                .write16(entry, if (seed + i) % 5 == 4 { 1 } else { 3 });
            w(c, entry + 8, if (seed + i) % 7 == 3 { 0 } else { data });
            for k in 0..0x20u32 {
                w(c, data + 4 * k, r.next_u32());
            }
        }
        c.g[A0][0] = world as u64;
        c.g[A1][0] = (seed % 4) as u64;
    }

    /// `World_GetSectionProcesses`, `World_GetSectionProcessCount` and
    /// `World_GetSectionShutdownFuncCount`: the section's cache entry passed
    /// to the matching `tWScriptCache_*` getter.
    #[test]
    fn world_get_section_wrappers_match() {
        for (n, (gen, hand)) in [
            (
                ee::s13::f_003a6c38__generated as fn(&mut Cpu),
                world_get_section_processes as fn(&mut Cpu),
            ),
            (ee::s13::f_003a6c08__generated, world_get_section_process_count),
            (ee::s13::f_003a6ba8__generated, world_get_section_shutdown_func_count),
        ]
        .into_iter()
        .enumerate()
        {
            check(gen, hand, SEEDS, &|c, seed| {
                world_section_setup(c, seed, 0x3a6000 + n as u64)
            });
        }
    }

    /// `ScriptData_GetBankList`: `InSeq_GetHeaderExtra` on the data (the
    /// header's version halfword at `+2` decides).
    #[test]
    fn script_data_get_bank_list_matches() {
        check(
            ee::s07::f_0027c078__generated,
            script_data_get_bank_list,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x27c078, 0);
                c.g[A0][0] = (BASE + 0x100) as u64;
                c.bus.write16(BASE + 0x102, (seed % 20) as u16);
            },
        );
    }

    /// `ScriptEngine_GetInstanceScript`: `ScriptEngine_GetSeqProcessScript`
    /// reads `inst->+0xc->+0xc`.
    #[test]
    fn script_engine_get_instance_script_matches() {
        check(
            ee::s07::f_0027a668__generated,
            script_engine_get_instance_script,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27a668, 0);
                let (inst, mid) = (BASE + 0x100, BASE + 0x200);
                c.g[A1][0] = inst as u64;
                w(c, inst + 0xc, mid);
                w(c, mid + 0xc, r.next_u32());
            },
        );
    }

    /// `ScriptEngine_FindAnimBank`: `ScriptAnimBankMan_FindBank` on the bank
    /// manager at `engine + 0xa28` (0x14-byte bank entries, live flag at +8).
    #[test]
    fn script_engine_find_anim_bank_matches() {
        check(
            ee::s07::f_00279ed8__generated,
            script_engine_find_anim_bank,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x279ed8, 0);
                let engine = BASE + 0x1000;
                let mgr = engine + 0xa28;
                let banks = BASE + 0x4000;
                let count: u32 = [0, 1, 2, 6, 0x8000_0000][seed as usize % 5];
                w(c, mgr + 4, count);
                w(c, mgr + 8, banks);
                for i in 0..6u32 {
                    let e = banks + 0x14 * i;
                    w(c, e, i);
                    w(c, e + 8, if i == 2 || i == 4 { 0 } else { 1 });
                }
                c.g[A0][0] = engine as u64;
                c.g[A1][0] = [0u64, 2, 4, 5, 9][seed as usize % 5];
            },
        );
    }

    /// `MultiParticleMan_RemoveInstance`: `MultiParticle_RemoveInstance` on
    /// the manager's particle `index` (instance array at `+0xc`, count `+4`).
    #[test]
    fn multi_particle_man_remove_instance_matches() {
        check(
            ee::s11::f_00357b10__generated,
            multi_particle_man_remove_instance,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x357b10, 0);
                let man = BASE + 0x100;
                let index = seed % 3;
                let particle = BASE + 0x1000;
                let entries = BASE + 0x2000;
                let count = seed % 5;
                w(c, man + 4 * index, particle);
                w(c, particle + 4, count);
                w(c, particle + 0xc, entries);
                for i in 0..5u32 {
                    w(c, entries + 4 * i, if i < count { r.next_u32() } else { 0 });
                }
                c.g[A0][0] = man as u64;
                c.g[A1][0] = index as u64;
                c.g[A2][0] = if seed % 2 == 0 && count > 0 {
                    c.bus.read32(entries + 4 * (seed % count)) as u64
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    /// `BEBoltInt_RestoreAllEquipFlags`: 3 profiles x 10 characters, each with
    /// a count (0, negative or a few) of flag entries.
    #[test]
    fn be_bolt_int_restore_all_equip_flags_matches() {
        check(
            ee::s01::f_0014af10__generated,
            be_bolt_int_restore_all_equip_flags,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x14af10, 0);
                for p in 0..3u32 {
                    for ch in 0..10u32 {
                        let rec = 0x4a_6ca8 + p * 0x9b50 + ch * 0xf88;
                        let count = match r.next_u32() % 8 {
                            0 => 0,
                            1 => 0xffff_fffe,
                            n => n % 5,
                        };
                        w(c, rec + 0x28c, count);
                        for i in 0..5u32 {
                            let flags = r.next_u32();
                            w(c, rec + 0x290 + 4 * i, flags);
                        }
                    }
                }
            },
        );
    }

    // ---- tests whose callees are real guest code: run in a child process

    fn ai_load_anim_data_sources_checked() {
        covered(
            ee::s00::f_0012b280__generated,
            ai_load_anim_data_sources,
            100,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x12b280, 6);
                w(c, GAME_GP_TEST.wrapping_add(0xd08), BASE + 0x1000);
                w(c, GAME_GP_TEST.wrapping_add(0xd8c), r.next_u32());
            },
        );
    }

    /// `AI_LoadAnimDataSources`: the six guest calls (mount, data source
    /// allocation and construction, registration, post hook, unmount) with
    /// their arguments, the table store, unwinds at any call.
    #[test]
    fn ai_load_anim_data_sources_matches() {
        in_child(
            "ai_load_anim_data_sources_matches",
            ai_load_anim_data_sources_checked,
        );
    }

    const GAME_GP_TEST: u32 = crate::difftest::GAME_GP;

    fn section_man_apply_sky_checked() {
        covered(
            ee::s05::f_0022de98__generated,
            section_man_apply_sky,
            200,
            &|c, seed| {
                let mut r = child_machine(c, seed, 0x22de98, 8);
                let (this, sky_box, obj) = (BASE + 0x100, BASE + 0x200, BASE + 0x300);
                c.g[A0][0] = this as u64;
                w(c, this + SECTION_MAN_SKY_ACTIVE, (seed % 4 != 0) as u32);
                w(c, this + SECTION_MAN_SKY_PENDING, (seed % 5 != 0) as u32);
                let current = [0x2c_u32, 0x2e, 0x30, 5, 0xffff_ffff, 0x2d][(seed / 3) as usize % 6];
                w(c, this + SECTION_MAN_SKY_CURRENT, current);
                w(c, this + SECTION_MAN_SKY_BOX, sky_box);
                w(c, sky_box + 0x14, obj);
                let section = r.next_u32() & 0xff;
                w(c, obj + 0x10, section);
                // The sky table: 16-byte entries from 0x442168, word 1 the section.
                let entry = 0x44_2168u32.wrapping_add(current << 4);
                w(
                    c,
                    entry + 4,
                    if seed % 2 == 0 { section } else { section + 1 },
                );
            },
        );
    }

    /// `SectionMan_ApplySky`: inactive and nothing pending (both skipped),
    /// the unload loop with the current sky in or out of 0x2c..0x30, and the
    /// sky box cleared (section differs) or kept (same).
    #[test]
    fn section_man_apply_sky_matches() {
        in_child(
            "section_man_apply_sky_matches",
            section_man_apply_sky_checked,
        );
    }

    fn rewards_file_load_checked() {
        covered(ee::s01::f_0015ad58__generated, rewards_file_load, 100, &|c, seed| {
            let mut r = child_machine(c, seed, 0x15ad58, 5);
            let gp = GAME_GP_TEST;
            let (out, world) = (BASE + 0x100, BASE + 0x200);
            c.g[A0][0] = out as u64;
            c.g[A1][0] = [0u64, 0, 1, 0x77][seed as usize % 4];
            w(c, gp.wrapping_sub(0x848), world);
            w(c, world + 0x7c, r.next_u32());
            w(c, gp.wrapping_sub(0x1e80), r.next_u32());
        });
    }

    /// `RewardsFile_Load`: the synchronous read-and-parse path (`a1 == 0`) and
    /// the asynchronous acquire (`a1 != 0`), the result stored in `*out`.
    #[test]
    fn rewards_file_load_matches() {
        in_child("rewards_file_load_matches", rewards_file_load_checked);
    }

    fn sky_box_set_section_checked() {
        covered(ee::s11::f_00353cf0__generated, sky_box_set_section, 200, &|c, seed| {
            let mut r = child_machine(c, seed, 0x353cf0, 2);
            let this = BASE + 0x100;
            let (p, a, b, c_arr, table) = (
                BASE + 0x400,
                BASE + 0x500,
                BASE + 0x600,
                BASE + 0x1000,
                BASE + 0x700,
            );
            c.g[A0][0] = this as u64;
            let section = [0xffff_ffffu32, 0xffff_fffb, 0, 1, 2, 3, 0x102][seed as usize % 7];
            c.g[A1][0] = section as u64;
            // The first fake call (0x2d1bd8) returns `p`.
            w(c, LOG_RESULTS, p);
            w(c, p, a);
            w(c, a, b);
            w(c, b + 0x14, c_arr);
            w(c, a + 8, table);
            for i in 0..4u32 {
                // Mesh counts: some zero or negative.
                c.bus.write16(
                    c_arr + 0x58 * i + 0x26,
                    [3u16, 0, 0xfffe, 1][((seed + i) % 4) as usize],
                );
                let entry = BASE + 0x800 + 0x20 * i;
                w(
                    c,
                    table + 4 * i,
                    if (seed + i) % 5 == 1 { 0 } else { entry },
                );
                let words = BASE + 0xa00 + 0x10 * i;
                w(c, entry + 0x1c, words);
                w(
                    c,
                    words,
                    if (seed + i) % 3 == 0 {
                        0xff
                    } else {
                        r.next_u32()
                    },
                );
            }
        });
    }

    /// `SkyBox_SetSection`: a negative section clears the box; otherwise the
    /// mesh table lookup (no meshes, no table entry, a zero or non-zero
    /// size word) decides what is stored at `+0x14`.
    #[test]
    fn sky_box_set_section_matches() {
        in_child("sky_box_set_section_matches", sky_box_set_section_checked);
    }

    fn section_man_select_peak_lighting_checked() {
        covered(
            ee::s05::f_0022e180__generated,
            section_man_select_peak_lighting,
            200,
            &|c, seed| {
                child_machine(c, seed, 0x22e180, 1);
                c.pc = 0x0022_e180;
                // The jr table (0x47b4e0): case -> arm (the real mapping), with
                // one bogus target at index 21 for half of the seeds.
                let arms: [u32; 22] = [
                    0x1fc, 0x1ac, 0x1c0, 0x1d4, 0x1e8, 0x1fc, 0x1d4, 0x1e8, 0x1fc, 0x1c0, 0x1e8,
                    0x1ac, 0x1c0, 0x1e8, 0x1fc, 0x1d4, 0x1e8, 0x1fc, 0x1ac, 0x1c0, 0x1d4, 0x1e8,
                ];
                for (i, a) in arms.iter().enumerate() {
                    w(c, 0x47_b4e0 + 4 * i as u32, 0x0022_e000 + a);
                }
                if (seed / 14) % 2 == 0 {
                    w(c, 0x47_b4e0 + 4 * 21, 0x0022_e100);
                }
                c.g[A1][0] = [
                    0u64,
                    1,
                    2,
                    3,
                    4,
                    5,
                    9,
                    11,
                    14,
                    18,
                    21,
                    0x16,
                    0x40,
                    0xffff_ffff,
                ][seed as usize % 14];
            },
        );
    }

    /// `SectionMan_SelectPeakLighting`: every table arm, the default, the
    /// out-of-range sky and a table target the original sends to `jump_out`.
    #[test]
    fn section_man_select_peak_lighting_matches() {
        in_child(
            "section_man_select_peak_lighting_matches",
            section_man_select_peak_lighting_checked,
        );
    }

    fn default_3_chase_camera_algorithm_dtor_checked() {
        covered(
            ee::s02::f_00176da8__generated,
            default_3_chase_camera_algorithm_dtor,
            100,
            &|c, seed| {
                child_machine(c, seed, 0x176da8, 1);
                c.g[A0][0] = (BASE + 0x100) as u64;
            },
        );
    }

    /// `Default_3ChaseCameraAlgorithm_dtor`: vtable store, base destructor.
    #[test]
    fn default_3_chase_camera_algorithm_dtor_matches() {
        in_child(
            "default_3_chase_camera_algorithm_dtor_matches",
            default_3_chase_camera_algorithm_dtor_checked,
        );
    }

    fn game_module_enter_checked() {
        covered(ee::s06::f_00230050__generated, game_module_enter, 100, &|c, seed| {
            let mut r = child_machine(c, seed, 0x230050, 9);
            let gp = GAME_GP_TEST;
            let (this, world, obj, vt) = (BASE + 0x100, BASE + 0x200, BASE + 0x300, BASE + 0x400);
            c.g[A0][0] = this as u64;
            for off in [0x10u32, 0x94] {
                w(c, this + off, r.next_u32());
            }
            w(c, gp.wrapping_sub(0x848), world);
            w(c, world + 0x8c, r.next_u32());
            for off in [0x84cu32, 0x854] {
                w(c, gp.wrapping_sub(off), r.next_u32());
            }
            for off in [0x21e4u32, 0x410] {
                w(c, gp.wrapping_add(off), r.next_u32());
            }
            w(c, gp.wrapping_add(0x1158), obj);
            w(c, obj + 4, vt);
            c.bus.write16(vt + 0x88, (r.next_u32() & 0xfffc) as u16);
            w(c, vt + 0x8c, FAKE);
            c.bus.write16(vt + 0x20, (r.next_u32() & 0xfffc) as u16);
            w(c, vt + 0x24, FAKE + 0x10);
        });
    }

    /// `GameModule_Enter`: the nine guest calls (two of them virtual) in order
    /// with their arguments, the three flags set, unwinds at any call.
    #[test]
    fn game_module_enter_matches() {
        in_child("game_module_enter_matches", game_module_enter_checked);
    }

    fn world_load_checked() {
        covered(ee::s13::f_003a6ae0__generated, world_load, 100, &|c, seed| {
            let mut r = child_machine(c, seed, 0x3a6ae0, 8);
            let world = BASE + 0x100;
            c.g[A0][0] = world as u64;
            for off in [0u32, 4, 8] {
                w(c, world + off, r.next_u32());
            }
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }

    /// `World_Load`: the eight guest calls (reset, load, two section counts,
    /// the two allocations, the stack-record pair) with their arguments.
    #[test]
    fn world_load_matches() {
        in_child("world_load_matches", world_load_checked);
    }

    fn fe_ui_input_dtor_checked() {
        covered(ee::s04::f_001dd530__generated, fe_ui_input_dtor, 100, &|c, seed| {
            let mut r = child_machine(c, seed, 0x1dd530, 1);
            c.g[A0][0] = (BASE + 0x100) as u64;
            c.g[A1][0] = (r.next_u32() & !1 | (seed & 1)) as u64;
        });
    }

    /// `FEUIInput_dtor`: vtable store; delete only with the flag bit set.
    #[test]
    fn fe_ui_input_dtor_matches() {
        in_child("fe_ui_input_dtor_matches", fe_ui_input_dtor_checked);
    }

    fn input_parser_parse_all_checked() {
        covered(
            ee::s10::f_00323098__generated,
            input_parser_parse_all,
            200,
            &|c, seed| {
                child_machine(c, seed, 0x323098, 6);
                c.g[A0][0] = (BASE + 0x100) as u64;
                // First token: end of input (0) or a statement start; then 0..3
                // further statements before the input ends.
                w(c, CTRL, if seed % 4 == 0 { 0 } else { 1 + seed % 5 });
                w(c, CTRL + 4, (seed / 4) % 4);
            },
        );
    }

    /// `InputParser_ParseAll`: empty input, and 1..4 statements parsed.
    #[test]
    fn input_parser_parse_all_matches() {
        in_child(
            "input_parser_parse_all_matches",
            input_parser_parse_all_checked,
        );
    }
}
