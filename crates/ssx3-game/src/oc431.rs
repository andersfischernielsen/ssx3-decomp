//! Staging module oc431 (wave wave220): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc431`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_be70, bolts_read_file, "Bolts_ReadFile"),
    (0x0022_d088, section_man_load_event, "SectionMan_LoadEvent"),
    (
        0x0028_1400,
        script_view_obj_deactivate,
        "ScriptViewObj_Deactivate",
    ),
    (0x003a_7fc0, world_cache_load, "cWorldCache_Load"),
    (
        0x0016_cbe0,
        camera_trigger_man_remove_all_triggers,
        "CameraTriggerMan_RemoveAllTriggers",
    ),
    (
        0x0014_dc40,
        bolts_get_char_table4_count,
        "Bolts_GetCharTable4Count",
    ),
    (
        0x0014_dc00,
        bolts_get_char_table3_count,
        "Bolts_GetCharTable3Count",
    ),
    (
        0x0011_bb98,
        rider_create_rider_metrix,
        "Rider_CreateRiderMetrix",
    ),
    (0x0014_d068, bolts_prepare_for_game, "Bolts_PrepareForGame"),
    (0x0028_1018, script_view_obj_ctor, "ScriptViewObj_ctor"),
    (0x0026_b880, path_data_read_markers, "PathData_ReadMarkers"),
    (
        0x0032_6078,
        input_preproc_process_file,
        "InputPreproc_ProcessFile",
    ),
    (0x0032_1428, input_parser_ctor, "InputParser_ctor"),
    (
        0x0023_0430,
        game_module_create_section_script_table,
        "GameModule_CreateSectionScriptTable",
    ),
    (
        0x0031_4a40,
        mdf_file_set_borrowed_data,
        "MdfFile_SetBorrowedData",
    ),
    (0x001d_d580, ui_screen_flags_ctor, "UIScreenFlags_ctor"),
    (0x0027_7f20, script_engine_ctor, "ScriptEngine_ctor"),
    (0x0022_e3b8, game_view_man_ctor, "GameViewMan_ctor"),
    (
        0x0017_6460,
        camera_bound_obj_get_scale,
        "CameraBoundObj_GetScale",
    ),
    (
        0x0017_6458,
        camera_bound_obj_get_field14,
        "CameraBoundObj_GetField14",
    ),
];

/// `int Bolts_GetCharTable4Count(BoltsLib* self, int character)`
/// (0x0014dc40): the number of rows of the character's table 4, the word at
/// `self + character * 4 + 0x3ec`.
pub fn bolts_get_char_table4_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(BOLTS_CHAR_TABLE4_COUNTS));
    c.pc = ra;
}

/// Offset of the per-character table 4 row counts in the bolts library.
pub(crate) const BOLTS_CHAR_TABLE4_COUNTS: u32 = 0x3ec;

/// `int Bolts_GetCharTable3Count(BoltsLib* self, int character)`
/// (0x0014dc00): the number of rows of the character's table 3, the word at
/// `self + character * 4 + 0x2fc`.
pub fn bolts_get_char_table3_count(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(BOLTS_CHAR_TABLE3_COUNTS));
    c.pc = ra;
}

/// Offset of the per-character table 3 row counts in the bolts library.
pub(crate) const BOLTS_CHAR_TABLE3_COUNTS: u32 = 0x2fc;

/// `void* CameraBoundObj_GetScale(CameraBoundObj* self)` (0x00176460):
/// the address of the scale field (`self + 0x18`).
pub fn camera_bound_obj_get_scale(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    c.pc = ra;
}

/// `void* CameraBoundObj_GetField14(CameraBoundObj* self)` (0x00176458):
/// the address of the field at `self + 0x14`.
pub fn camera_bound_obj_get_field14(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
    c.pc = ra;
}

/// `UIScreenFlags* UIScreenFlags_ctor(UIScreenFlags* self)` (0x001dd580):
/// sets bit 0 and clears bit 1 of the flag halfword at +0 and returns `self`.
pub fn ui_screen_flags_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] |= 0x1;
    c.g[V1][0] &= 0xfffd;
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[A0][0] as u32, c.g[V1][0]);
    c.pc = ra;
}

/// `void Bolts_ReadFile(BoltsLib* self)`: loads the bolts data file (`BXFILE_sizez`, `new`, `FILE_loadatz` on the path at `gp-0x1f24`) into a new buffer and points the library's tables into it: 0x38-byte entries (n0 at +0x18/+0x1c), then three tables (12-, 8- and 8-byte rows) and the string pool at +0x14. The global flag word at `gp-0x1f08` gets the file's first word (+1 when odd). Every entry's string offsets (name, +0x28/+0x2c/+0x30 and the 4 sub-names) are relocated to pointers into the pool.
/// (`Bolts_ReadFile`, 0x0014be70)
pub fn bolts_read_file(c: &mut Cpu) {
    guest(c, |c| {
        'b_14bf64: {
            c.cycles += 59;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f24));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            call_ptr(c, 0x00317f60, 0x0014be90)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0x460000;
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x5a00));
            c.g[A2][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0014beac)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f24));
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.sw((c.g[S1][0] as u32), c.g[V0][0]);
            call_ptr(c, 0x003e1bb8, 0x0014bec0)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32));
            c.g[V0][0] = 0x38;
            c.g[A3][0] = 0xc;
            c.g[T2][0] = c.lw((c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f08), c.g[T2][0]);
            c.g[T0][0] = c.g[T2][0] & 0x1;
            c.g[T1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.mult(0, 2, 9, 2);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[T1][0]);
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.mult(0, 3, 6, 7);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[A2][0]);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A1][0] = sx((c.g[A2][0] as u32) << 3);
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x24), c.g[A2][0]);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A1][0] = sx((c.g[A2][0] as u32) << 3);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            let t = c.g[T0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
            if t {
                break 'b_14bf64;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f08), c.g[V0][0]);
                break 'b_14bf64;
            }
        }
        'b_14c050: {
            c.cycles += 2;
            let t = (c.g[T1][0] as i64) <= 0;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_14c050;
            } else {
                c.cycles += 3;
                c.g[T1][0] = 0x38;
                c.g[T0][0] = 0x38;
                c.mult(0, 5, 6, 9);
                'l_14bf78: loop {
                    'b_14bfa0: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_14bfa0;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            break 'b_14bfa0;
                        }
                    }
                    'b_14bfb8: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_14bfb8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                            break 'b_14bfb8;
                        }
                    }
                    'b_14bfe0: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_14bfe0;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            break 'b_14bfe0;
                        }
                    }
                    'b_14bff8: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                        let t = c.g[V1][0] == 0u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_14bff8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                            break 'b_14bff8;
                        }
                    }
                    c.cycles += 4;
                    c.mult(0, 7, 6, 8);
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    'l_14c008: loop {
                        'b_14c030: {
                            c.cycles += 7;
                            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18));
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            if t {
                                break 'b_14c030;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                break 'b_14c030;
                            }
                        }
                        c.cycles += 4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            if event_due(c) {
                                deliver_event_here(c, 0x0014c008);
                            }
                            continue 'l_14c008;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.mult(0, 5, 6, 9);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014bf78);
                                }
                                continue 'l_14bf78;
                            } else {
                                break 'b_14c050;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void SectionMan_LoadEvent(SectionMan* self, int idx, int mode)`: unloads every section (0..49) that the location `idx` does not keep (sections 43..48 are never unloaded), marks the location activated, then activates its sections, the sky/lighting, the extra section and the location's mode (mode 7 means "the table's own mode").
/// (`SectionMan_LoadEvent`, 0x0022d088)
pub fn section_man_load_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_22d0e4: {
            'b_22d0e0: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.g[V0][0] = 0x7;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S3][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = c.g[A2][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] != c.g[V0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_22d0e0;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x440000;
                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2820));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    let t = true;
                    c.g[FP][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    if t {
                        break 'b_22d0e4;
                    } else {
                        break 'b_22d0e0;
                    }
                }
            }
            c.cycles += 1;
            c.g[FP][0] = c.g[A1][0];
            break 'b_22d0e4;
        }
        c.cycles += 15;
        c.g[V1][0] = 0x28;
        c.g[V0][0] = 0x440000;
        c.mult(0, 3, 18, 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2488));
        c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[S6][0] = 0x31;
        c.g[A1][0] = 0u64;
        c.g[S1][0] = 0x28;
        c.g[S7][0] = 0x4;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = ((c.g[A0][0] as i64) < 0i64) as u64;
        if c.g[V1][0] == 0 {
            c.g[S6][0] = c.g[A0][0];
        }
        c.g[A3][0] = 0x1;
        'l_22d120: loop {
            c.cycles += 4;
            c.g[A2][0] = 0u64;
            c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2c));
            c.g[T1][0] = c.g[A1][0] ^ 0x2b;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            'l_22d130: loop {
                'b_22d160: {
                    c.cycles += 6;
                    c.mult(0, 4, 18, 17);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                    if t {
                        break 'b_22d160;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        if c.g[A1][0] != c.g[V1][0] {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x0022d130);
                            }
                            continue 'l_22d130;
                        } else {
                            c.cycles += 1;
                            c.g[A3][0] = 0u64;
                            break 'b_22d160;
                        }
                    }
                }
                'b_22d180: {
                    c.cycles += 5;
                    c.g[V0][0] = (c.g[S7][0] < c.g[T0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A3][0] = 0u64;
                    }
                    if c.g[T1][0] == 0 {
                        c.g[A3][0] = 0u64;
                    }
                    if c.g[A3][0] == 0u64 {
                        c.g[A1][0] = c.g[S0][0];
                        break 'b_22d180;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x0022d598,
                            f: ee::s05::SectionMan_RequestUnload,
                        }
                        .call(c, 0x0022d17c)?;
                        c.g[A1][0] = c.g[S0][0];
                        break 'b_22d180;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 50i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A3][0] = 0x1;
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0022d120);
                    }
                    continue 'l_22d120;
                } else {
                    'b_22d1f0: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x28;
                        c.g[V1][0] = 0x1;
                        c.mult(0, 5, 18, 2);
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V1][0]);
                        c.g[V0][0] = 0x440000;
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x1bc), c.g[S2][0]);
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2488));
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[S1][0] = 0u64;
                        if t {
                            break 'b_22d1f0;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                            c.g[S4][0] = c.g[V1][0];
                            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            'l_22d1d0: loop {
                                c.cycles += 8;
                                c.g[A0][0] = c.g[S3][0];
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                Callee {
                                    addr: 0x0022d478,
                                    f: ee::s05::SectionMan_RequestActivate,
                                }
                                .call(c, 0x0022d1e0)?;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0022d1d0);
                                    }
                                    continue 'l_22d1d0;
                                } else {
                                    break 'b_22d1f0;
                                }
                            }
                        }
                    }
                    'b_22d210: {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0022e180,
                            f: ee::s05::SectionMan_SelectPeakLighting,
                        }
                        .call(c, 0x0022d1fc)?;
                        c.g[V0][0] = c.g[S6][0] ^ 0x31;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[S6][0];
                        if t {
                            break 'b_22d210;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S3][0];
                            Callee {
                                addr: 0x0022d478,
                                f: ee::s05::SectionMan_RequestActivate,
                            }
                            .call(c, 0x0022d210)?;
                            break 'b_22d210;
                        }
                    }
                    'b_22d23c: {
                        c.cycles += 8;
                        c.g[V0][0] = 0x28;
                        c.g[A0][0] = 0x440000;
                        c.mult(0, 2, 18, 2);
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2488));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        let t = (c.g[A1][0] as i64) < 0;
                        c.g[A0][0] = c.g[S3][0];
                        if t {
                            break 'b_22d23c;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[S3][0];
                            Callee {
                                addr: 0x0022d478,
                                f: ee::s05::SectionMan_RequestActivate,
                            }
                            .call(c, 0x0022d238)?;
                            c.g[A0][0] = c.g[S3][0];
                            break 'b_22d23c;
                        }
                    }
                    c.cycles += 14;
                    c.g[A1][0] = c.g[FP][0];
                    Callee {
                        addr: 0x0022d668,
                        f: ee::s05::SectionMan_AdvanceEventSwitchSync,
                    }
                    .call(c, 0x0022d244)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
            }
        }
    })
}

/// `void ScriptViewObj_Deactivate(ScriptViewObj* self, int player)`: clears the engine's binding filter, releases the (up to two) camera bindings, stops the fade transition, removes the focus box, resets the view object and deactivates it as a script object.
/// (`ScriptViewObj_Deactivate`, 0x00281400)
pub fn script_view_obj_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S5][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xdc));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = 0x1;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S4][0] = 0xffffffffffffffff;
        Callee {
            addr: 0x0027d2e8,
            f: ee::s07::ScriptEngine_GetInstance,
        }
        .call(c, 0x0028143c)?;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x0027d170,
            f: ee::s07::ScriptEngine_ClearObjBindingFilter,
        }
        .call(c, 0x00281448)?;
        'l_281448: loop {
            'b_281468: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                let t = (c.g[V0][0] as i64) < 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t {
                    break 'b_281468;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    Callee {
                        addr: 0x0027d688,
                        f: ee::s07::ScriptObjCam_SetBinding,
                    }
                    .call(c, 0x00281464)?;
                    c.sw((c.g[S0][0] as u32), c.g[S4][0]);
                    break 'b_281468;
                }
            }
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            let t = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00281448);
                }
                continue 'l_281448;
            } else {
                'b_281498: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xd8));
                    let t = c.g[A0][0] == 0u64;
                    c.g[A1][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_281498;
                    } else {
                        c.cycles += 6;
                        c.mtc1(0, 12);
                        c.g[A2][0] = 0x9;
                        c.mov_s(13, 12);
                        c.mov_s(14, 12);
                        c.mov_s(15, 12);
                        Callee {
                            addr: 0x00282db0,
                            f: ee::s07::cTransitionMan_StopTransition,
                        }
                        .call(c, 0x00281498)?;
                        break 'b_281498;
                    }
                }
                'b_2814bc: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x110));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_2814bc;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa4));
                        call_ptr(c, 0x001033f8, 0x002814b8)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x110), 0u64);
                        break 'b_2814bc;
                    }
                }
                c.cycles += 14;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x00281a50,
                    f: ee::s07::ScriptViewObj_Reset,
                }
                .call(c, 0x002814c4)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S5][0];
                Callee {
                    addr: 0x00283440,
                    f: ee::s07::ScriptObj_Deactivate,
                }
                .call(c, 0x002814d0)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `int cWorldCache_Load(cWorldCache* self, char* name)`: allocates the world memory manager and the shared/section cache tables, loads `<dir>/<name>.sdb`, builds the chunk resolver, then initialises the stream manager and the world view (returns 0 when either fails) and the page map.
/// (`cWorldCache_Load`, 0x003a7fc0)
pub fn world_cache_load(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a811c: {
            c.cycles += 76;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.g[A3][0] = 0u64;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S4][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = c.g[A0][0];
            c.g[S5][0] = c.g[A2][0];
            c.g[A1][0] = 0x490000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A2][0] = 0u64;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4e30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[A0][0] = 0x13c;
            call_ptr(c, 0x00317d70, 0x003a8004)?;
            c.g[A0][0] = c.g[V0][0];
            Callee {
                addr: 0x003a7a20,
                f: ee::s13::WorldMemman_ctor,
            }
            .call(c, 0x003a800c)?;
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = 0x4a0000;
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x47c0));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S4][0];
            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            call_ptr(c, 0x003182d8, 0x003a802c)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003186d0, 0x003a803c)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x47c8));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x00318630, 0x003a8050)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32));
            c.g[A1][0] = 0x2000000;
            call_ptr(c, 0x003e18c8, 0x003a805c)?;
            c.g[A0][0] = c.g[SP][0];
            c.sw((c.g[S3][0] as u32), c.g[V0][0]);
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x003a806c)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x00318260, 0x003a8078)?;
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00318260, 0x003a8084)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32));
            c.g[A1][0] = 0x490000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4e40));
            c.g[A2][0] = 0u64;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
            c.g[A3][0] = 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = 0xe8;
            call_ptr(c, 0x00317d70, 0x003a80a8)?;
            c.g[A1][0] = c.lw((c.g[S3][0] as u32));
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x18));
            Callee {
                addr: 0x003a7c88,
                f: ee::s13::SectionTable_ctor,
            }
            .call(c, 0x003a80b8)?;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A1][0] = 0x490000;
            c.g[V0][0] = c.lw((c.g[S3][0] as u32));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4e50));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x003a80dc)?;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S3][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_3a811c;
            } else {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                'l_3a80f8: loop {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x003a80f8);
                        }
                        continue 'l_3a80f8;
                    } else {
                        break 'b_3a811c;
                    }
                }
            }
        }
        'b_3a8184: {
            'b_3a8180: {
                c.cycles += 15;
                Callee {
                    addr: 0x003aad98,
                    f: ee::s13::cPS2ChunkResolve_Create,
                }
                .call(c, 0x003a8124)?;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x3e8), c.g[V0][0]);
                c.g[A1][0] = c.g[S5][0];
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                let t = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x003a8140)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x300));
                c.g[A2][0] = c.g[S3][0];
                Callee {
                    addr: 0x003a6e98,
                    f: ee::s13::cStreamMan_Init,
                }
                .call(c, 0x003a8150)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                if t {
                    break 'b_3a8180;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32));
                    c.g[A2][0] = c.g[S3][0];
                    Callee {
                        addr: 0x003a9000,
                        f: ee::s13::cWorldCache_StreamerAttachSdb,
                    }
                    .call(c, 0x003a8164)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_3a8184;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x003a82c8,
                            f: ee::s13::cWorldCache_InitHullSlots,
                        }
                        .call(c, 0x003a8178)?;
                        let t = true;
                        c.g[V0][0] = c.g[S0][0];
                        if t {
                            break 'b_3a8184;
                        } else {
                            break 'b_3a8180;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_3a8184;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

/// `void Bolts_PrepareForGame(BoltsLib* self)`: for each of the 3 profiles and 10 characters marks the profile's bolt entries (flag 0x10) that are unlocked (flag 4) or required by a bolt rule, then rebuilds the DB and the character indexes.
/// (`Bolts_PrepareForGame`, 0x0014d068)
pub fn bolts_prepare_for_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V1][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = 0x4a0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S6][0] = 0x9b50;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.mult(0, 19, 3, 22);
        'l_14d0a0: loop {
            c.cycles += 2;
            c.g[S0][0] = 0u64;
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            'l_14d0a8: loop {
                'b_14d110: {
                    c.cycles += 17;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0014dc10,
                        f: ee::s01::Bolts_GetCharTable3,
                    }
                    .call(c, 0x0014d0b4)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[S1][0] = c.g[V0][0];
                    Callee {
                        addr: 0x0014dc00,
                        f: ee::s01::Bolts_GetCharTable3Count,
                    }
                    .call(c, 0x0014d0c4)?;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x6ca8));
                    c.g[V0][0] = 0xf88;
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.mult(0, 2, 16, 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28c));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x290));
                    if t {
                        break 'b_14d110;
                    } else {
                        c.cycles += 1;
                        c.g[A2][0] = c.g[V0][0];
                        'l_14d0f0: loop {
                            'b_14d104: {
                                c.cycles += 4;
                                c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = c.g[V1][0] & 0x4;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.g[V1][0] | 0x10;
                                if t {
                                    break 'b_14d104;
                                } else {
                                    c.cycles += 1;
                                    c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                    break 'b_14d104;
                                }
                            }
                            c.cycles += 3;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                            let t = c.g[A2][0] != 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014d0f0);
                                }
                                continue 'l_14d0f0;
                            } else {
                                break 'b_14d110;
                            }
                        }
                    }
                }
                'b_14d1a8: {
                    c.cycles += 2;
                    if (c.g[A3][0] as i64) <= 0 {
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x288));
                        break 'b_14d1a8;
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = c.g[A3][0];
                        'l_14d120: loop {
                            'b_14d14c: {
                                'b_14d148: {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x288));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    if t {
                                        break 'b_14d148;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                        let t = true;
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        if t {
                                            break 'b_14d14c;
                                        } else {
                                            break 'b_14d148;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = 0u64;
                                break 'b_14d14c;
                            }
                            'b_14d19c: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = c.g[V0][0] & 0x4;
                                if c.g[V0][0] == 0u64 {
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    break 'b_14d19c;
                                } else {
                                    'b_14d198: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x6));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        if t {
                                            break 'b_14d198;
                                        } else {
                                            'b_14d18c: {
                                                'b_14d188: {
                                                    c.cycles += 5;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[A1][0] as u32).wrapping_add(0x288)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                                    let t = (c.g[V0][0] as i64) < 0;
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    if t {
                                                        break 'b_14d188;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x290));
                                                        let t = true;
                                                        c.g[V1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        if t {
                                                            break 'b_14d18c;
                                                        } else {
                                                            break 'b_14d188;
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V1][0] = 0u64;
                                                break 'b_14d18c;
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                            c.g[V0][0] = c.g[V0][0] | 0x10;
                                            c.sh((c.g[V1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                            break 'b_14d198;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                    break 'b_14d19c;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[A2][0] != 0u64;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014d120);
                                }
                                continue 'l_14d120;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x288));
                                break 'b_14d1a8;
                            }
                        }
                    }
                }
                'b_14d1c4: {
                    'b_14d1c0: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        if t {
                            break 'b_14d1c0;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                            let t = true;
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            if t {
                                break 'b_14d1c4;
                            } else {
                                break 'b_14d1c0;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    break 'b_14d1c4;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[S0][0] = c.g[T0][0];
                c.g[V1][0] = ((c.g[S0][0] as i64) < 10i64) as u64;
                c.g[V0][0] = c.g[V0][0] | 0x10;
                let t = c.g[V1][0] != 0u64;
                c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0014d0a8);
                    }
                    continue 'l_14d0a8;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = c.g[S4][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 19, 3, 22);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014d0a0);
                        }
                        continue 'l_14d0a0;
                    } else {
                        c.cycles += 20;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c7a0,
                            f: ee::s01::Bolts_BuildGameBolts,
                        }
                        .call(c, 0x0014d1f4)?;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c2b0,
                            f: ee::s01::Bolts_IndexBoltsByChar,
                        }
                        .call(c, 0x0014d1fc)?;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c320,
                            f: ee::s01::Bolts_IndexTable2ByChar,
                        }
                        .call(c, 0x0014d204)?;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c3c8,
                            f: ee::s01::Bolts_IndexTable3ByChar,
                        }
                        .call(c, 0x0014d20c)?;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x0014c488,
                            f: ee::s01::Bolts_IndexTable4ByChar,
                        }
                        .call(c, 0x0014d214)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `ScriptViewObj* ScriptViewObj_ctor(ScriptViewObj* self)`: the script object base, the binding at +0x10, nine camera bindings (0x14 bytes each from +0x1c), the vtables (+0xc, +0x10), zeroes the two camera words at +0xd0/+0xd4, points every binding's owner at `self`, clears the fade (+0xd8), sets the focus id (+0xf8) to -1 and resets the object. Returns `self`.
/// (`ScriptViewObj_ctor`, 0x00281018)
pub fn script_view_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A1][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S2][0] = 0x8;
        Callee {
            addr: 0x00283200,
            f: ee::s07::ScriptObj_ctor,
        }
        .call(c, 0x00281044)?;
        c.g[S3][0] = 0xffffffffffffffff;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        Callee {
            addr: 0x00282f60,
            f: ee::s07::ScriptBinding_ctor,
        }
        .call(c, 0x00281050)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2360));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2398));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        'l_281068: loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            Callee {
                addr: 0x00283088,
                f: ee::s07::ScriptCamBinding_ctor,
            }
            .call(c, 0x00281074)?;
            let t = c.g[S2][0] != c.g[S3][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00281068);
                }
                continue 'l_281068;
            } else {
                c.cycles += 3;
                c.g[V1][0] = 0x1;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd4));
                'l_281088: loop {
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00281088);
                        }
                        continue 'l_281088;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                        c.g[V1][0] = 0x8;
                        'l_2810b0: loop {
                            c.cycles += 7;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002810b0);
                                }
                                continue 'l_2810b0;
                            } else {
                                c.cycles += 13;
                                c.g[V0][0] = 0xffffffffffffffff;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xd8), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00281a50,
                                    f: ee::s07::ScriptViewObj_Reset,
                                }
                                .call(c, 0x002810e0)?;
                                c.g[V0][0] = c.g[S0][0];
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    })
}

/// `int PathData_ReadMarkers(PathData* self, char* p)`: reads the marker table of a path file at `p` (count, then per node id, type key, position, direction and two indices) into `self` and returns the number of bytes consumed. The node type is the index of its key in the 3-entry table at 0x445428.
/// (`PathData_ReadMarkers`, 0x0026b880)
pub fn path_data_read_markers(c: &mut Cpu) {
    guest(c, |c| {
        'b_26ba18: {
            c.cycles += 21;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S0][0] = c.g[A1][0];
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = c.g[SP][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S7][0] = c.g[S0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[S3][0] = 0u64;
            Callee {
                addr: 0x0026ba68,
                f: ee::s07::PathData_ReadInt,
            }
            .call(c, 0x0026b8c4)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S0][0]);
            let t = (c.g[V0][0] as i64) <= 0;
            c.sw((c.g[S1][0] as u32), c.g[V0][0]);
            if t {
                break 'b_26ba18;
            } else {
                c.cycles += 5;
                c.g[S6][0] = 0x440000;
                c.g[S4][0] = 0x28;
                c.g[S5][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5428));
                c.mult(0, 18, 19, 20);
                'l_26b8e8: loop {
                    'b_26b9c4: {
                        'b_26b990: {
                            c.cycles += 37;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                            Callee {
                                addr: 0x0026ba68,
                                f: ee::s07::PathData_ReadInt,
                            }
                            .call(c, 0x0026b8fc)?;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0026ba68,
                                f: ee::s07::PathData_ReadInt,
                            }
                            .call(c, 0x0026b908)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                            Callee {
                                addr: 0x0026ba48,
                                f: ee::s07::PathData_ReadVec3,
                            }
                            .call(c, 0x0026b924)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                            Callee {
                                addr: 0x0026ba48,
                                f: ee::s07::PathData_ReadVec3,
                            }
                            .call(c, 0x0026b93c)?;
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x0026ba68,
                                f: ee::s07::PathData_ReadInt,
                            }
                            .call(c, 0x0026b948)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
                            Callee {
                                addr: 0x0026ba68,
                                f: ee::s07::PathData_ReadInt,
                            }
                            .call(c, 0x0026b958)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x5428));
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_26b990;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                let t = true;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                                if t {
                                    break 'b_26b9c4;
                                } else {
                                    c.cycles += 1;
                                    break 'b_26b990;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        'l_26b994: loop {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 3i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                            if t {
                                break 'b_26b9c4;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                if c.g[V1][0] != c.g[A0][0] {
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0026b994);
                                    }
                                    continue 'l_26b994;
                                } else {
                                    c.cycles += 4;
                                    c.mult(0, 2, 19, 20);
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                                    break 'b_26b9c4;
                                }
                            }
                        }
                    }
                    c.cycles += 21;
                    c.mult(0, 6, 19, 20);
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A3][0] = 0x3c;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    c.g[S3][0] = c.g[T0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.mult(0, 4, 4, 7);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.mult(0, 18, 19, 20);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026b8e8);
                        }
                        continue 'l_26b8e8;
                    } else {
                        break 'b_26ba18;
                    }
                }
            }
        }
        c.cycles += 12;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[S7][0] as u32)));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

/// `void InputPreproc_ProcessFile(InputPreProcessor* self, char* path, char* name)`: formats the file name, loads the file, emits the first line marker, preprocesses it text chunk by chunk and frees the buffer.
/// (`InputPreproc_ProcessFile`, 0x00326078)
pub fn input_preproc_process_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S3][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = 0x4a0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[A0][0] = c.g[SP][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[A3][0] = c.g[S2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3fa0));
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x00417828, 0x003260b8)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa4));
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x003e1a10, 0x003260cc)?;
        c.g[S4][0] = c.g[S0][0];
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xac), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
        Callee {
            addr: 0x00326478,
            f: ee::s10::InputPreproc_NextChar,
        }
        .call(c, 0x003260ec)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xac));
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        Callee {
            addr: 0x00326430,
            f: ee::s10::InputPreproc_EmitLineMarker,
        }
        .call(c, 0x003260fc)?;
        c.g[A0][0] = c.g[S1][0];
        'l_326100: loop {
            c.cycles += 6;
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[S2][0];
            Callee {
                addr: 0x00326150,
                f: ee::s10::InputPreproc_ProcessText,
            }
            .call(c, 0x00326110)?;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S1][0];
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00326100);
                }
                continue 'l_326100;
            } else {
                'b_326130: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa0));
                    let t = c.g[A0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                    if t {
                        break 'b_326130;
                    } else {
                        c.cycles += 3;
                        Callee {
                            addr: 0x00317e98,
                            f: ee::s10::EA_MemFree,
                        }
                        .call(c, 0x0032612c)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                        break 'b_326130;
                    }
                }
                c.cycles += 7;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `InputParser* InputParser_ctor(InputParser* self, char* file, int length)`: sets the source and length, builds the free-id stack (63..1) and the 0x800-node free list (the loop at 0x321480 is `ids[n++] = i`), and parses the whole file.
/// (`InputParser_ctor`, 0x00321428)
pub fn input_parser_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[V0][0] = 0x8000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = 0x10000;
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A0][0] = 0x1;
        c.sw((c.g[S0][0] as u32), c.g[A1][0]);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), 0u64);
        c.g[A1][0] = c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x90), c.g[A0][0]);
        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8ec));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8c), c.g[A0][0]);
        c.sb((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.g[A0][0] = 0x3f;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x94), 0u64);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x9e8), 0u64);
        'l_321480: loop {
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x9e8));
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let t = (c.g[A0][0] as i64) > 0;
            c.sw((c.g[A1][0] as u32).wrapping_add(0x9e8), c.g[V1][0]);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00321480);
                }
                continue 'l_321480;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe4));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                c.g[A0][0] = 0x7fe;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf4));
                'l_3214b8: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    let t = (c.g[A0][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003214b8);
                        }
                        continue 'l_3214b8;
                    } else {
                        c.cycles += 10;
                        c.g[V0][0] = 0x8000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), 0u64);
                        Callee {
                            addr: 0x00323098,
                            f: ee::s10::InputParser_ParseAll,
                        }
                        .call(c, 0x003214e8)?;
                        c.g[V0][0] = c.g[S0][0];
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `void GameModule_CreateSectionScriptTable(GameModule* self, int sectionId)` (the section id is `a1`; `a0` is overwritten): when the section's entry (12 bytes each, kind >= 2 and count > 0) says it has scripts, creates the section's local script table (`WScriptMan_CreateLocalTable`).
/// (`GameModule_CreateSectionScriptTable`, 0x00230430)
pub fn game_module_create_section_script_table(c: &mut Cpu) {
    guest(c, |c| {
        'b_230468: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A0][0] = 0xc;
            c.mult(0, 4, 5, 4);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.g[V0][0] = c.lh((c.g[A0][0] as u32));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = 0u64;
            if t {
                break 'b_230468;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
                break 'b_230468;
            }
        }
        'b_23047c: {
            c.cycles += 2;
            let t = c.g[A2][0] == 0u64;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            if t {
                break 'b_23047c;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                call_ptr(c, 0x00308b58, 0x00230478)?;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                break 'b_23047c;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `ScriptEngine* ScriptEngine_ctor(ScriptEngine* self)`: constructs the two view objects, the script/movie/data tables, the dummy, audio, anim bank and sound bank objects, clears the engine state, installs the instance pointer at `gp+0x3c0` and numbers the two view slots.
/// (`ScriptEngine_ctor`, 0x00277f20)
pub fn script_engine_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = 0x1;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
        Callee {
            addr: 0x00281e80,
            f: ee::s07::ScriptEngine_ScriptDbCtor,
        }
        .call(c, 0x00277f48)?;
        c.g[S3][0] = 0xffffffffffffffff;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x500));
        Callee {
            addr: 0x00283798,
            f: ee::s07::ScriptMovie_ctor,
        }
        .call(c, 0x00277f54)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x51c));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x100;
        Callee {
            addr: 0x00275828,
            f: ee::s07::DataInstanceMan_ctor,
        }
        .call(c, 0x00277f64)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = 0x480000;
        c.g[A0][0] = 0x480000;
        c.g[A1][0] = 0x480000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2100));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x21a0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2168));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x21b8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2ac), c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x518), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x528), c.g[A0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x2a8), c.g[A1][0]);
        'l_277f98: loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            Callee {
                addr: 0x00281018,
                f: ee::s07::ScriptViewObj_ctor,
            }
            .call(c, 0x00277fa4)?;
            let t = c.g[S2][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x120));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00277f98);
                }
                continue 'l_277f98;
            } else {
                c.cycles += 33;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x910));
                c.g[S2][0] = 0x31;
                Callee {
                    addr: 0x0027bfd0,
                    f: ee::s07::ScriptDummyObj_ctor,
                }
                .call(c, 0x00277fb8)?;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x92c));
                Callee {
                    addr: 0x002805b8,
                    f: ee::s07::ScriptAudioObj_ctor,
                }
                .call(c, 0x00277fc0)?;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa28));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0xa;
                c.g[A3][0] = 0xf0;
                Callee {
                    addr: 0x002753b8,
                    f: ee::s07::ScriptAnimBankMan_ctor,
                }
                .call(c, 0x00277fd4)?;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa38));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0xa;
                c.g[A3][0] = 0x11;
                Callee {
                    addr: 0x00283450,
                    f: ee::s07::ScriptSoundBankMan_ctor,
                }
                .call(c, 0x00277fe8)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x52c), 0u64);
                c.g[V1][0] = 0xa;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x530), 0u64);
                c.g[V0][0] = 0x2;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x534), 0u64);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x61c));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x538), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x53c), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x540), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x544), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x548), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x3c0), c.g[S1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0xa48), c.g[V1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x550), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x54c), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x554), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x620), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x624), 0u64);
                'l_278030: loop {
                    c.cycles += 7;
                    c.sw((c.g[A0][0] as u32), 0u64);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00278030);
                        }
                        continue 'l_278030;
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x628));
                        c.g[S2][0] = 0x9;
                        'l_278058: loop {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            Callee {
                                addr: 0x00277f08,
                                f: ee::s07::ScriptBankSlot_Clear,
                            }
                            .call(c, 0x00278064)?;
                            let t = (c.g[S2][0] as i64) >= 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00278058);
                                }
                                continue 'l_278058;
                            } else {
                                c.cycles += 3;
                                c.g[S2][0] = 0u64;
                                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                'l_278078: loop {
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    Callee {
                                        addr: 0x00281160,
                                        f: ee::s07::ScriptViewObj_SetViewIndex,
                                    }
                                    .call(c, 0x00278084)?;
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x120));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00278078);
                                        }
                                        continue 'l_278078;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V0][0] = c.g[S1][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.pc = t;
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

/// `GameViewMan* GameViewMan_ctor(GameViewMan* self, uint count)`: creates `count` (at most 2) game cameras (far clip 30000, or 20000 with two views), sets their controller, and stores the screen width and height.
/// (`GameViewMan_ctor`, 0x0022e3b8)
pub fn game_view_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_22e3f4: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V0][0] = 0x1;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = 0xffffffffffffffff;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
            c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x51d4));
            let t = c.g[V0][0] != 0u64;
            c.sw((c.g[S2][0] as u32), c.g[A1][0]);
            if t {
                break 'b_22e3f4;
            } else {
                c.cycles += 1;
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x51d0));
                break 'b_22e3f4;
            }
        }
        c.cycles += 3;
        c.g[S1][0] = 0u64;
        c.g[S3][0] = 0x480000;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
        'l_22e400: loop {
            'b_22e474: {
                'b_22e46c: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0x20000000;
                    if t {
                        break 'b_22e46c;
                    } else {
                        c.cycles += 23;
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = 0x4b0;
                        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x4ac8));
                        call_ptr(c, 0x00317d70, 0x0022e420)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0015dc70, 0x0022e42c)?;
                        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                        c.g[A1][0] = c.g[S1][0];
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x51cc));
                        c.mov_s(14, 20);
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
                        c.g[AT][0] = 0x41f00000;
                        c.mtc1(1, 13);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                        let t = c.g[A2][0] as u32;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0022e458)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0015dfd8, 0x0022e464)?;
                        let t = true;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_22e474;
                        } else {
                            break 'b_22e46c;
                        }
                    }
                }
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32), 0u64);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                break 'b_22e474;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[S1][0] < 0x2u64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0022e400);
                }
                continue 'l_22e400;
            } else {
                'b_22e4dc: {
                    'b_22e4c0: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x1;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), 0u64);
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0022e4a8)?;
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        if t {
                            break 'b_22e4c0;
                        } else {
                            c.cycles += 4;
                            c.mtc1(2, 20);
                            c.cvt_s_w(20, 20);
                            let t = true;
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                            if t {
                                break 'b_22e4dc;
                            } else {
                                break 'b_22e4c0;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.mtc1(2, 20);
                    c.cvt_s_w(20, 20);
                    c.add_s(20, 20, 20);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                    break 'b_22e4dc;
                }
                'b_22e51c: {
                    'b_22e504: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0022e4ec)?;
                        let t = (c.g[V0][0] as i64) < 0;
                        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                        if t {
                            break 'b_22e504;
                        } else {
                            c.cycles += 4;
                            c.mtc1(2, 0);
                            c.cvt_s_w(0, 0);
                            let t = true;
                            c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_22e51c;
                            } else {
                                break 'b_22e504;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.mtc1(2, 0);
                    c.cvt_s_w(0, 0);
                    c.add_s(0, 0, 0);
                    c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x20));
                    break 'b_22e51c;
                }
                c.cycles += 12;
                c.g[V0][0] = c.g[S2][0];
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x24));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x18), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), 0u64);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void Rider_CreateRiderMetrix(Rider* self)`: allocates a 0x1cc-byte RiderMetrix, constructs it, stores it at +0x790 and initialises its HUD effects.
/// (`Rider_CreateRiderMetrix`, 0x0011bb98)
pub fn rider_create_rider_metrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[A1][0] = 0x450000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7970));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x1cc;
        call_ptr(c, 0x00317d70, 0x0011bbc0)?;
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x00117248,
            f: ee::s00::RiderMetrix_ctor,
        }
        .call(c, 0x0011bbc8)?;
        c.g[A1][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x790), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        Callee {
            addr: 0x001172e0,
            f: ee::s00::RiderMetrix_InitHUDFX,
        }
        .call(c, 0x0011bbd8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `void MdfFile_SetBorrowedData(MdfFile* self, ...)`: clears the first word (the owned-buffer pointer) and parses the header from the borrowed data.
/// (`MdfFile_SetBorrowedData`, 0x00314a40)
pub fn mdf_file_set_borrowed_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.sw((c.g[V0][0] as u32), 0u64);
        Callee {
            addr: 0x00314aa0,
            f: ee::s10::MdfFile_ParseHeader,
        }
        .call(c, 0x00314a54)?;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void CameraTriggerMan_RemoveAllTriggers(CameraTriggerMan* self)` (0x0016cbe0): removes every registered trigger from the spatial hash (bounds through the shape vtable, `SpatialKey_FromAABB`, `SpatialHash_Remove`; frees the cell when all its 11 words are empty), then resets the two trigger stacks, the trigger manager, both active lists and the chase controller switches. Dispatcher-shaped like the original (resumable at its block heads after an event unwind).
pub fn camera_trigger_man_remove_all_triggers(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0016cbe0 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x0016cd74;
                        continue;
                    }
                    pc = 0x0016cc10;
                    continue;
                }
                0x0016cc10 => {
                    c.cycles += 2;
                    c.g[S5][0] = 0xffffffffffffffff;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                    pc = 0x0016cc18;
                    continue;
                }
                0x0016cc18 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0016cd60;
                        continue;
                    }
                    pc = 0x0016cc30;
                    continue;
                }
                0x0016cc30 => {
                    c.cycles += 39;
                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x24));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[S0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    call_ptr(c, t, 0x0016cc5c)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x0016cc74)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S5][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = 0x2;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x0016ccbc)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cccc;
                    continue;
                }
                0x0016cccc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016ccd8;
                    continue;
                }
                0x0016ccd8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cce4;
                    continue;
                }
                0x0016cce4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016ccf0;
                    continue;
                }
                0x0016ccf0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016ccfc;
                    continue;
                }
                0x0016ccfc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cd08;
                    continue;
                }
                0x0016cd08 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));

                    if c.g[V0][0] != 0u64 {
                        c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cd14;
                    continue;
                }
                0x0016cd14 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        pc = 0x0016cd2c;
                        continue;
                    }
                    pc = 0x0016cd20;
                    continue;
                }
                0x0016cd20 => {
                    c.cycles += 1;
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x0016cd24;
                    continue;
                }
                0x0016cd24 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0016cd4c;
                        continue;
                    }
                    pc = 0x0016cd2c;
                    continue;
                }
                0x0016cd2c => {
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    pc = 0x0016cd30;
                    continue;
                }
                0x0016cd30 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0016cd24) {
                            return Ok(());
                        }
                        pc = 0x0016cd24;
                        continue;
                    }
                    pc = 0x0016cd3c;
                    continue;
                }
                0x0016cd3c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0016cd30) {
                            return Ok(());
                        }
                        pc = 0x0016cd30;
                        continue;
                    }
                    pc = 0x0016cd48;
                    continue;
                }
                0x0016cd48 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0016cd4c;
                    continue;
                }
                0x0016cd4c => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                        pc = 0x0016cd64;
                        continue;
                    }
                    pc = 0x0016cd54;
                    continue;
                }
                0x0016cd54 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0016cd5c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    pc = 0x0016cd60;
                    continue;
                }
                0x0016cd60 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    pc = 0x0016cd64;
                    continue;
                }
                0x0016cd64 => {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S4][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32));
                        if back_edge_unwinds(c, 0x0016cc18) {
                            return Ok(());
                        }
                        pc = 0x0016cc18;
                        continue;
                    }
                    pc = 0x0016cd74;
                    continue;
                }
                0x0016cd74 => {
                    c.cycles += 21;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                    Callee {
                        addr: 0x0016d200,
                        f: ee::s02::CameraTriggerStack_Clear,
                    }
                    .call(c, 0x0016cd7c)?;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3c));
                    Callee {
                        addr: 0x0016d200,
                        f: ee::s02::CameraTriggerStack_Clear,
                    }
                    .call(c, 0x0016cd84)?;
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0016c098,
                        f: ee::s02::CameraTriggerMan_Destroy,
                    }
                    .call(c, 0x0016cd8c)?;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
                    Callee {
                        addr: 0x0016c628,
                        f: ee::s02::ActiveCameraTriggerList_Clear,
                    }
                    .call(c, 0x0016cd94)?;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x18));
                    Callee {
                        addr: 0x0016c628,
                        f: ee::s02::ActiveCameraTriggerList_Clear,
                    }
                    .call(c, 0x0016cd9c)?;
                    call_ptr(c, 0x0015dc18, 0x0016cda4)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0016cbe0, 0x0016cdc8);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_COUNT, LOG_RESULTS};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP, SP as TEST_SP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the fakes below: they act only in a machine whose RAM holds
    /// `OC431_MAGIC` here (other units' tests run the real callees).
    pub(crate) const OC431_CTRL: u32 = DATA + 0xf_f000;
    pub(crate) const OC431_MAGIC: u32 = 0x4f43_3431;
    /// Fixture objects live above the area the fakes' default results point into
    /// (`DATA..DATA + 0x40000`).
    pub(crate) const OC431_BASE: u32 = DATA + 0x4_0000;

    /// Make fake call `n` (counted from 0 over all fakes of the run) return `v`
    /// in v0 (also 0 and negative values, which `LOG_RESULTS` cannot do).
    pub(crate) fn oc431_force(c: &mut Cpu, n: u32, v: u32) {
        c.bus.write32(OC431_CTRL + 0x20 + 8 * n, 1);
        c.bus.write32(OC431_CTRL + 0x24 + 8 * n, v);
    }

    /// Effects some fakes have on the caller's data (what the real callee
    /// would change that the caller then reads).
    pub(crate) fn oc431_post(c: &mut Cpu, addr: u32, a0: u32) {
        match addr {
            // InputParser_NextToken: the parser's "more input" word (+0x98).
            0x0032_1af0 => {
                let v = c.bus.read32(OC431_CTRL + 4);
                c.bus.write32(a0.wrapping_add(0x98), v);
            }
            // InputParser_ParseStatement consumes the input.
            0x0032_30e0 => c.bus.write32(a0.wrapping_add(0x98), 0),
            // EAFile_QueueOpen: the file does not open.
            0x003d_d4e8 => c.g[V0][0] = 0,
            _ => {}
        }
    }

    /// A fake callee: log the call like `fakes::fake`, then apply forced
    /// results and the fake's data effects (unless the call unwound).
    pub(crate) fn oc431_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0] as u32;
        let n = c.bus.read32(LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if c.bus.read32(OC431_CTRL + 0x20 + 8 * n) != 0 {
            c.g[V0][0] = sx(c.bus.read32(OC431_CTRL + 0x24 + 8 * n));
        }
        oc431_post(c, addr, a0);
    }

    /// Calls to addresses outside the function table (vtable slots pointing
    /// at `FAKE`) are fake callees too.
    pub(crate) fn oc431_miss(c: &mut Cpu, target: u32) {
        oc431_fake_call(c, target)
    }

    macro_rules! oc431_fake {
        ($fake:ident, $prev:ident, $addr:expr, $generated:path) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC431_CTRL) != OC431_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                oc431_fake_call(c, $addr)
            }
        };
    }

    oc431_fake!(
        oc431_fake_00317d70,
        OC431_PREV_00317D70,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    oc431_fake!(
        oc431_fake_00317e30,
        OC431_PREV_00317E30,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    oc431_fake!(
        oc431_fake_00317e98,
        OC431_PREV_00317E98,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );
    oc431_fake!(
        oc431_fake_001172e0,
        OC431_PREV_001172E0,
        0x0011_72e0,
        ee::s00::f_001172e0__generated
    );
    oc431_fake!(
        oc431_fake_00117540,
        OC431_PREV_00117540,
        0x0011_7540,
        ee::s00::f_00117540__generated
    );
    oc431_fake!(
        oc431_fake_001033f8,
        OC431_PREV_001033F8,
        0x0010_33f8,
        ee::s00::f_001033f8__generated
    );
    oc431_fake!(
        oc431_fake_00314aa0,
        OC431_PREV_00314AA0,
        0x0031_4aa0,
        ee::s10::f_00314aa0__generated
    );
    oc431_fake!(
        oc431_fake_00321af0,
        OC431_PREV_00321AF0,
        0x0032_1af0,
        ee::s10::f_00321af0__generated
    );
    oc431_fake!(
        oc431_fake_003230e0,
        OC431_PREV_003230E0,
        0x0032_30e0,
        ee::s10::f_003230e0__generated
    );
    oc431_fake!(
        oc431_fake_00224da0,
        OC431_PREV_00224DA0,
        0x0022_4da0,
        ee::s05::f_00224da0__generated
    );
    oc431_fake!(
        oc431_fake_00224df0,
        OC431_PREV_00224DF0,
        0x0022_4df0,
        ee::s05::f_00224df0__generated
    );
    oc431_fake!(
        oc431_fake_0022d478,
        OC431_PREV_0022D478,
        0x0022_d478,
        ee::s05::f_0022d478__generated
    );
    oc431_fake!(
        oc431_fake_0022d598,
        OC431_PREV_0022D598,
        0x0022_d598,
        ee::s05::f_0022d598__generated
    );
    oc431_fake!(
        oc431_fake_0038ac50,
        OC431_PREV_0038AC50,
        0x0038_ac50,
        ee::s12::f_0038ac50__generated
    );
    oc431_fake!(
        oc431_fake_0015cd80,
        OC431_PREV_0015CD80,
        0x0015_cd80,
        ee::s01::f_0015cd80__generated
    );
    oc431_fake!(
        oc431_fake_0015d698,
        OC431_PREV_0015D698,
        0x0015_d698,
        ee::s01::f_0015d698__generated
    );
    oc431_fake!(
        oc431_fake_001619a8,
        OC431_PREV_001619A8,
        0x0016_19a8,
        ee::s01::f_001619a8__generated
    );
    oc431_fake!(
        oc431_fake_0015dfd8,
        OC431_PREV_0015DFD8,
        0x0015_dfd8,
        ee::s01::f_0015dfd8__generated
    );
    oc431_fake!(
        oc431_fake_00168fe8,
        OC431_PREV_00168FE8,
        0x0016_8fe8,
        ee::s02::f_00168fe8__generated
    );
    oc431_fake!(
        oc431_fake_00176d68,
        OC431_PREV_00176D68,
        0x0017_6d68,
        ee::s02::f_00176d68__generated
    );
    oc431_fake!(
        oc431_fake_0027d2e8,
        OC431_PREV_0027D2E8,
        0x0027_d2e8,
        ee::s07::f_0027d2e8__generated
    );
    oc431_fake!(
        oc431_fake_0027d170,
        OC431_PREV_0027D170,
        0x0027_d170,
        ee::s07::f_0027d170__generated
    );
    oc431_fake!(
        oc431_fake_0027d688,
        OC431_PREV_0027D688,
        0x0027_d688,
        ee::s07::f_0027d688__generated
    );
    oc431_fake!(
        oc431_fake_00281a50,
        OC431_PREV_00281A50,
        0x0028_1a50,
        ee::s07::f_00281a50__generated
    );
    oc431_fake!(
        oc431_fake_00283440,
        OC431_PREV_00283440,
        0x0028_3440,
        ee::s07::f_00283440__generated
    );
    oc431_fake!(
        oc431_fake_00283200,
        OC431_PREV_00283200,
        0x0028_3200,
        ee::s07::f_00283200__generated
    );
    oc431_fake!(
        oc431_fake_00283088,
        OC431_PREV_00283088,
        0x0028_3088,
        ee::s07::f_00283088__generated
    );
    oc431_fake!(
        oc431_fake_00281e80,
        OC431_PREV_00281E80,
        0x0028_1e80,
        ee::s07::f_00281e80__generated
    );
    oc431_fake!(
        oc431_fake_00283798,
        OC431_PREV_00283798,
        0x0028_3798,
        ee::s07::f_00283798__generated
    );
    oc431_fake!(
        oc431_fake_00275828,
        OC431_PREV_00275828,
        0x0027_5828,
        ee::s07::f_00275828__generated
    );
    oc431_fake!(
        oc431_fake_0027bfd0,
        OC431_PREV_0027BFD0,
        0x0027_bfd0,
        ee::s07::f_0027bfd0__generated
    );
    oc431_fake!(
        oc431_fake_002805b8,
        OC431_PREV_002805B8,
        0x0028_05b8,
        ee::s07::f_002805b8__generated
    );
    oc431_fake!(
        oc431_fake_00283450,
        OC431_PREV_00283450,
        0x0028_3450,
        ee::s07::f_00283450__generated
    );
    oc431_fake!(
        oc431_fake_00277f08,
        OC431_PREV_00277F08,
        0x0027_7f08,
        ee::s07::f_00277f08__generated
    );
    oc431_fake!(
        oc431_fake_0016c098,
        OC431_PREV_0016C098,
        0x0016_c098,
        ee::s02::f_0016c098__generated
    );
    oc431_fake!(
        oc431_fake_0016c628,
        OC431_PREV_0016C628,
        0x0016_c628,
        ee::s02::f_0016c628__generated
    );
    oc431_fake!(
        oc431_fake_0015dc18,
        OC431_PREV_0015DC18,
        0x0015_dc18,
        ee::s01::f_0015dc18__generated
    );
    oc431_fake!(
        oc431_fake_00328f28,
        OC431_PREV_00328F28,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );
    oc431_fake!(
        oc431_fake_003284b8,
        OC431_PREV_003284B8,
        0x0032_84b8,
        ee::s10::f_003284b8__generated
    );
    oc431_fake!(
        oc431_fake_00317e50,
        OC431_PREV_00317E50,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );
    oc431_fake!(
        oc431_fake_0014c2b0,
        OC431_PREV_0014C2B0,
        0x0014_c2b0,
        ee::s01::f_0014c2b0__generated
    );
    oc431_fake!(
        oc431_fake_0014c320,
        OC431_PREV_0014C320,
        0x0014_c320,
        ee::s01::f_0014c320__generated
    );
    oc431_fake!(
        oc431_fake_0014c3c8,
        OC431_PREV_0014C3C8,
        0x0014_c3c8,
        ee::s01::f_0014c3c8__generated
    );
    oc431_fake!(
        oc431_fake_0014c488,
        OC431_PREV_0014C488,
        0x0014_c488,
        ee::s01::f_0014c488__generated
    );
    oc431_fake!(
        oc431_fake_0014c7a0,
        OC431_PREV_0014C7A0,
        0x0014_c7a0,
        ee::s01::f_0014c7a0__generated
    );
    oc431_fake!(
        oc431_fake_003e3208,
        OC431_PREV_003E3208,
        0x003e_3208,
        ee::s14::f_003e3208__generated
    );
    oc431_fake!(
        oc431_fake_003e32f8,
        OC431_PREV_003E32F8,
        0x003e_32f8,
        ee::s14::f_003e32f8__generated
    );
    oc431_fake!(
        oc431_fake_003e34e8,
        OC431_PREV_003E34E8,
        0x003e_34e8,
        ee::s14::f_003e34e8__generated
    );
    oc431_fake!(
        oc431_fake_003ddfa8,
        OC431_PREV_003DDFA8,
        0x003d_dfa8,
        ee::s14::f_003ddfa8__generated
    );
    oc431_fake!(
        oc431_fake_003dd4e8,
        OC431_PREV_003DD4E8,
        0x003d_d4e8,
        ee::s14::f_003dd4e8__generated
    );
    oc431_fake!(
        oc431_fake_003a6e98,
        OC431_PREV_003A6E98,
        0x003a_6e98,
        ee::s13::f_003a6e98__generated
    );
    oc431_fake!(
        oc431_fake_003a7a20,
        OC431_PREV_003A7A20,
        0x003a_7a20,
        ee::s13::f_003a7a20__generated
    );
    oc431_fake!(
        oc431_fake_003a7c88,
        OC431_PREV_003A7C88,
        0x003a_7c88,
        ee::s13::f_003a7c88__generated
    );
    oc431_fake!(
        oc431_fake_003a82c8,
        OC431_PREV_003A82C8,
        0x003a_82c8,
        ee::s13::f_003a82c8__generated
    );
    oc431_fake!(
        oc431_fake_003aad98,
        OC431_PREV_003AAD98,
        0x003a_ad98,
        ee::s13::f_003aad98__generated
    );
    oc431_fake!(
        oc431_fake_00318260,
        OC431_PREV_00318260,
        0x0031_8260,
        ee::s10::f_00318260__generated
    );
    oc431_fake!(
        oc431_fake_003182d8,
        OC431_PREV_003182D8,
        0x0031_82d8,
        ee::s10::f_003182d8__generated
    );
    oc431_fake!(
        oc431_fake_00318630,
        OC431_PREV_00318630,
        0x0031_8630,
        ee::s10::f_00318630__generated
    );
    oc431_fake!(
        oc431_fake_003186d0,
        OC431_PREV_003186D0,
        0x0031_86d0,
        ee::s10::f_003186d0__generated
    );
    oc431_fake!(
        oc431_fake_00326150,
        OC431_PREV_00326150,
        0x0032_6150,
        ee::s10::f_00326150__generated
    );
    oc431_fake!(
        oc431_fake_00326430,
        OC431_PREV_00326430,
        0x0032_6430,
        ee::s10::f_00326430__generated
    );
    oc431_fake!(
        oc431_fake_00326478,
        OC431_PREV_00326478,
        0x0032_6478,
        ee::s10::f_00326478__generated
    );
    oc431_fake!(
        oc431_fake_00417828,
        OC431_PREV_00417828,
        0x0041_7828,
        ee::s15::f_00417828__generated
    );

    pub(crate) fn oc431_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc431_fake_00317d70,
            &OC431_PREV_00317D70,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc431_fake_00317e30,
            &OC431_PREV_00317E30,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc431_fake_00317e98,
            &OC431_PREV_00317E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_001172E0),
            oc431_fake_001172e0,
            &OC431_PREV_001172E0,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_00117540),
            oc431_fake_00117540,
            &OC431_PREV_00117540,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_001033F8),
            oc431_fake_001033f8,
            &OC431_PREV_001033F8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00314AA0),
            oc431_fake_00314aa0,
            &OC431_PREV_00314AA0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00321AF0),
            oc431_fake_00321af0,
            &OC431_PREV_00321AF0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003230E0),
            oc431_fake_003230e0,
            &OC431_PREV_003230E0,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00224DA0),
            oc431_fake_00224da0,
            &OC431_PREV_00224DA0,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00224DF0),
            oc431_fake_00224df0,
            &OC431_PREV_00224DF0,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D478),
            oc431_fake_0022d478,
            &OC431_PREV_0022D478,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022D598),
            oc431_fake_0022d598,
            &OC431_PREV_0022D598,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038AC50),
            oc431_fake_0038ac50,
            &OC431_PREV_0038AC50,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015CD80),
            oc431_fake_0015cd80,
            &OC431_PREV_0015CD80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015D698),
            oc431_fake_0015d698,
            &OC431_PREV_0015D698,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001619A8),
            oc431_fake_001619a8,
            &OC431_PREV_001619A8,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015DFD8),
            oc431_fake_0015dfd8,
            &OC431_PREV_0015DFD8,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00168FE8),
            oc431_fake_00168fe8,
            &OC431_PREV_00168FE8,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_00176D68),
            oc431_fake_00176d68,
            &OC431_PREV_00176D68,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027D2E8),
            oc431_fake_0027d2e8,
            &OC431_PREV_0027D2E8,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027D170),
            oc431_fake_0027d170,
            &OC431_PREV_0027D170,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027D688),
            oc431_fake_0027d688,
            &OC431_PREV_0027D688,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00281A50),
            oc431_fake_00281a50,
            &OC431_PREV_00281A50,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283440),
            oc431_fake_00283440,
            &OC431_PREV_00283440,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283200),
            oc431_fake_00283200,
            &OC431_PREV_00283200,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283088),
            oc431_fake_00283088,
            &OC431_PREV_00283088,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00281E80),
            oc431_fake_00281e80,
            &OC431_PREV_00281E80,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283798),
            oc431_fake_00283798,
            &OC431_PREV_00283798,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00275828),
            oc431_fake_00275828,
            &OC431_PREV_00275828,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027BFD0),
            oc431_fake_0027bfd0,
            &OC431_PREV_0027BFD0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002805B8),
            oc431_fake_002805b8,
            &OC431_PREV_002805B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283450),
            oc431_fake_00283450,
            &OC431_PREV_00283450,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277F08),
            oc431_fake_00277f08,
            &OC431_PREV_00277F08,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C098),
            oc431_fake_0016c098,
            &OC431_PREV_0016C098,
        );
        fakes::hook(
            addr_of_mut!(ee::s02::OVERRIDE_0016C628),
            oc431_fake_0016c628,
            &OC431_PREV_0016C628,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0015DC18),
            oc431_fake_0015dc18,
            &OC431_PREV_0015DC18,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc431_fake_00328f28,
            &OC431_PREV_00328F28,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003284B8),
            oc431_fake_003284b8,
            &OC431_PREV_003284B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc431_fake_00317e50,
            &OC431_PREV_00317E50,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C2B0),
            oc431_fake_0014c2b0,
            &OC431_PREV_0014C2B0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C320),
            oc431_fake_0014c320,
            &OC431_PREV_0014C320,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C3C8),
            oc431_fake_0014c3c8,
            &OC431_PREV_0014C3C8,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C488),
            oc431_fake_0014c488,
            &OC431_PREV_0014C488,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014C7A0),
            oc431_fake_0014c7a0,
            &OC431_PREV_0014C7A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E3208),
            oc431_fake_003e3208,
            &OC431_PREV_003E3208,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E32F8),
            oc431_fake_003e32f8,
            &OC431_PREV_003E32F8,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E34E8),
            oc431_fake_003e34e8,
            &OC431_PREV_003E34E8,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
            oc431_fake_003ddfa8,
            &OC431_PREV_003DDFA8,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            oc431_fake_003dd4e8,
            &OC431_PREV_003DD4E8,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A6E98),
            oc431_fake_003a6e98,
            &OC431_PREV_003A6E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A7A20),
            oc431_fake_003a7a20,
            &OC431_PREV_003A7A20,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A7C88),
            oc431_fake_003a7c88,
            &OC431_PREV_003A7C88,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A82C8),
            oc431_fake_003a82c8,
            &OC431_PREV_003A82C8,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003AAD98),
            oc431_fake_003aad98,
            &OC431_PREV_003AAD98,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc431_fake_00318260,
            &OC431_PREV_00318260,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            oc431_fake_003182d8,
            &OC431_PREV_003182D8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            oc431_fake_00318630,
            &OC431_PREV_00318630,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            oc431_fake_003186d0,
            &OC431_PREV_003186D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326150),
            oc431_fake_00326150,
            &OC431_PREV_00326150,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326430),
            oc431_fake_00326430,
            &OC431_PREV_00326430,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00326478),
            oc431_fake_00326478,
            &OC431_PREV_00326478,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc431_fake_00417828,
            &OC431_PREV_00417828,
        );
    }

    /// A test machine: scrambled registers, game-like gp, fake callees
    /// armed, `pc` at the function's entry, unwinding at one of `calls` fake
    /// calls in a third of the seeds.
    pub(crate) fn oc431_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc431_install();
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = oc431_miss;
        c.pc = entry;
        c.bus.write32(OC431_CTRL, OC431_MAGIC);
        r
    }

    /// `n` random words below `bound` at `addr` (small counts and indices).
    pub(crate) fn oc431_small_words(c: &mut Cpu, r: &mut Rng, addr: u32, n: u32, bound: u32) {
        for i in 0..n {
            c.bus.write32(addr + 4 * i, r.next_u32() % bound);
        }
    }

    // -----------------------------------------------------------------------
    // Leaves.

    /// Table counts: the word at `self + character * 4 + offset`, for
    /// characters from -4 to 11 over a random library.
    fn oc431_count_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = machine(c, seed, entry as u64, 0);
        fill(c, &mut r, OC431_BASE, 0x400);
        c.g[A0][0] = OC431_BASE as u64 + 0x40;
        c.g[A1][0] = ((seed % 16) as i64 - 4) as u64;
        c.pc = entry;
    }

    #[test]
    fn bolts_get_char_table4_count_matches() {
        check(
            ee::s01::f_0014dc40__generated,
            bolts_get_char_table4_count,
            200,
            &|c, seed| oc431_count_setup(c, seed, 0x0014_dc40),
        );
    }

    #[test]
    fn bolts_get_char_table3_count_matches() {
        check(
            ee::s01::f_0014dc00__generated,
            bolts_get_char_table3_count,
            200,
            &|c, seed| oc431_count_setup(c, seed, 0x0014_dc00),
        );
    }

    #[test]
    fn camera_bound_obj_get_scale_matches() {
        check(
            ee::s02::f_00176460__generated,
            camera_bound_obj_get_scale,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x176460, 0);
                c.pc = 0x0017_6460;
            },
        );
    }

    #[test]
    fn camera_bound_obj_get_field14_matches() {
        check(
            ee::s02::f_00176458__generated,
            camera_bound_obj_get_field14,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x176458, 0);
                c.pc = 0x0017_6458;
            },
        );
    }

    /// Flag halfword with every combination of bits 0 and 1 and random
    /// upper bits.
    #[test]
    fn ui_screen_flags_ctor_matches() {
        check(
            ee::s04::f_001dd580__generated,
            ui_screen_flags_ctor,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dd580, 0);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                c.bus.write32(OC431_BASE + 0x100, r.next_u32());
                c.pc = 0x001d_d580;
            },
        );
    }

    // -----------------------------------------------------------------------
    // Constructors and small drivers over fake callees.

    /// `MdfFile_ParseHeader` is a fake; the owned-data word is cleared first.
    #[test]
    fn mdf_file_set_borrowed_data_matches() {
        oc431_install();
        covered(
            ee::s10::f_00314a40__generated,
            mdf_file_set_borrowed_data,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x314a40, 1, 0x0031_4a40);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                c.bus.write32(OC431_BASE + 0x100, r.next_u32() | 1);
            },
        );
    }

    /// `Rider_CreateRiderMetrix`: the allocation, `RiderMetrix_Reset` (inside
    /// the real constructor) and `InitHUDFX` are fakes.
    #[test]
    fn rider_create_rider_metrix_matches() {
        oc431_install();
        covered(
            ee::s00::f_0011bb98__generated,
            rider_create_rider_metrix,
            150,
            &|c, seed| {
                let _ = oc431_machine(c, seed, 0x11bb98, 3, 0x0011_bb98);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                c.bus.write32(LOG_RESULTS, OC431_BASE + 0x1000);
            },
        );
    }

    /// `GameModule_CreateSectionScriptTable`: sections whose entry has a kind
    /// below 2 or no scripts are skipped; otherwise the local table is created
    /// (a present one is destroyed first). Allocation and the Luno table
    /// constructor/destructor are fakes.
    #[test]
    fn game_module_create_section_script_table_matches() {
        oc431_install();
        covered(
            ee::s06::f_00230430__generated,
            game_module_create_section_script_table,
            200,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x230430, 3, 0x0023_0430);
                let (root, mid, table, wman) = (
                    OC431_BASE + 0x100,
                    OC431_BASE + 0x200,
                    OC431_BASE + 0x300,
                    OC431_BASE + 0x1000,
                );
                let id = seed % 6;
                c.g[A1][0] = id as u64;
                c.bus.write32(GAME_GP + 0x16c8, root);
                c.bus.write32(root + 4, mid);
                c.bus.write32(mid + 4, table);
                let kind = [0u32, 1, 2, 5][(r.next_u32() % 4) as usize];
                c.bus.write16(table + 12 * id, kind as u16);
                let scripts = [0u32, 3, 1][(r.next_u32() % 3) as usize];
                c.bus.write32(table + 12 * id + 8, scripts);
                c.bus.write32(GAME_GP + 0xce8, wman);
                let present = if r.next_u32() & 1 == 0 {
                    0
                } else {
                    OC431_BASE + 0x2000
                };
                c.bus.write32(wman + 0x3cc + 4 * id, present);
            },
        );
    }

    /// `InputParser_ctor`: the free-id stack and the node free list are
    /// built; `ParseAll` (real) runs `NextToken` and, while the parser has
    /// input, `ParseStatement` (fakes).
    #[test]
    fn input_parser_ctor_matches() {
        oc431_install();
        covered(ee::s10::f_00321428__generated, input_parser_ctor, 100, &|c, seed| {
            let mut r = oc431_machine(c, seed, 0x321428, 2, 0x0032_1428);
            c.bus.write32(OC431_CTRL + 4, seed % 2);
            c.g[A0][0] = (OC431_BASE + 0x1_0000) as u64;
            c.g[A1][0] = (OC431_BASE + 0x100) as u64;
            c.g[A2][0] = (r.next_u32() % 0x1000) as u64;
        });
    }

    /// `ScriptViewObj_ctor`: the base object, the +0x10 binding (real
    /// `ScriptBinding_ctor`), nine camera bindings and the reset (fakes).
    #[test]
    fn script_view_obj_ctor_matches() {
        oc431_install();
        covered(
            ee::s07::f_00281018__generated,
            script_view_obj_ctor,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x281018, 11, 0x0028_1018);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                fill(c, &mut r, OC431_BASE + 0x100, 0x48);
            },
        );
    }

    /// `ScriptEngine_ctor`: both view objects (real `ScriptViewObj_ctor`),
    /// the tables, the anim bank manager (real, with a fake allocation), the
    /// bank slots and the view indices (real).
    #[test]
    fn script_engine_ctor_matches() {
        oc431_install();
        covered(ee::s07::f_00277f20__generated, script_engine_ctor, 100, &|c, seed| {
            let mut r = oc431_machine(c, seed, 0x277f20, 40, 0x0027_7f20);
            c.g[A0][0] = (OC431_BASE + 0x100) as u64;
            fill(c, &mut r, OC431_BASE + 0x100, 0x2c0);
        });
    }

    /// Address of the vtable the fixture's camera (`GameCamera_ctor` puts
    /// `0x45b888` at +0x90) is called through.
    pub(crate) const OC431_CAMERA_VT: u32 = 0x0045_b888;

    /// `GameViewMan_ctor`: 0 to 3 views (far clip 20000 from two views on),
    /// real `GameCamera_ctor` over fake controller constructors, the camera's
    /// vtable method and the two display-size calls (positive and negative
    /// results, the latter through the halving path).
    #[test]
    fn game_view_man_ctor_matches() {
        oc431_install();
        covered(ee::s05::f_0022e3b8__generated, game_view_man_ctor, 200, &|c, seed| {
            let mut r = oc431_machine(c, seed, 0x22e3b8, 26, 0x0022_e3b8);
            let (this, grman, disp) =
                (OC431_BASE + 0x100, OC431_BASE + 0x1000, OC431_BASE + 0x1100);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(GAME_GP - 0x51d4, 30000.0f32.to_bits());
            c.bus.write32(GAME_GP - 0x51d0, 20000.0f32.to_bits());
            c.bus.write32(GAME_GP - 0x51cc, 0.785_398_2f32.to_bits());
            c.bus
                .write16(OC431_CAMERA_VT + 0x10, (r.next_u32() & 0xfc) as u16);
            c.bus.write32(OC431_CAMERA_VT + 0x14, FAKE);
            c.bus.write32(GAME_GP - 0x854, grman);
            c.bus.write32(grman + 0x10d8, disp);
            c.bus.write16(disp + 0x50, (r.next_u32() & 0xfc) as u16);
            c.bus.write32(disp + 0x54, FAKE);
            c.bus.write16(disp + 0x58, (r.next_u32() & 0xfc) as u16);
            c.bus.write32(disp + 0x5c, FAKE);
            // The two display-size calls are the last two fake calls (the
            // first return value of each camera is irrelevant).
            let cams = (seed % 4).min(2);
            let last = cams * 11;
            for k in 0..2 {
                let v = match r.next_u32() % 3 {
                    0 => 640,
                    1 => 0xffff_fff1,
                    _ => 0x8000_0001,
                };
                oc431_force(c, last + k, v);
            }
        });
    }

    /// `ScriptViewObj_Deactivate`: camera slots (-1 and bound), the fade
    /// object (absent, already the "none" state, or stopped through its
    /// vtable by the real `cTransitionMan_StopTransition`), the pending focus
    /// box (removed through a fake) and the reset/deactivate calls.
    #[test]
    fn script_view_obj_deactivate_matches() {
        oc431_install();
        covered(
            ee::s07::f_00281400__generated,
            script_view_obj_deactivate,
            200,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x281400, 10, 0x0028_1400);
                let (this, fade, vt, world, focus1, focus2) = (
                    OC431_BASE + 0x100,
                    OC431_BASE + 0x1000,
                    OC431_BASE + 0x1100,
                    OC431_BASE + 0x2000,
                    OC431_BASE + 0x2100,
                    OC431_BASE + 0x2200,
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                fill(c, &mut r, this, 0x50);
                c.bus.write32(this + 4, r.next_u32());
                for k in 0..2 {
                    let slot = [u32::MAX, 0, 1, 2][(r.next_u32() % 4) as usize];
                    c.bus.write32(this + 0xdc + 4 * k, slot);
                }
                // The fade object and its vtable (stop: +0x28/+0x2c, set: +0x20/+0x24).
                let has_fade = r.next_u32() % 3 != 0;
                c.bus.write32(this + 0xd8, if has_fade { fade } else { 0 });
                c.bus.write32(
                    fade + 0x18,
                    if r.next_u32() & 1 == 0 {
                        9
                    } else {
                        r.next_u32() % 9
                    },
                );
                c.bus.write32(fade + 0xc, vt);
                for off in [0x20u32, 0x28] {
                    c.bus.write16(vt + off, (r.next_u32() & 0xfc) as u16);
                    c.bus.write32(vt + off + 4, FAKE);
                }
                c.bus.write32(GAME_GP - 0x4778, 1.0f32.to_bits());
                // The pending focus box: world -> +0x84 -> +0xc -> +0xa4.
                let pending = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(this + 0x110, pending);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, focus1);
                c.bus.write32(focus1 + 0xc, focus2);
                c.bus.write32(focus2 + 0xa4, r.next_u32());
            },
        );
    }

    /// Table of the three marker kinds the node type is looked up in.
    pub(crate) const OC431_PATH_TYPES: u32 = 0x0044_5428;

    /// `PathData_ReadMarkers`: 0 to 3 nodes read in place from a file image
    /// (count, then id, kind key, position, direction and two indices per
    /// node), the kind key matching table entry 0, 1, 2 or none. The real
    /// `PathData_ReadInt`/`ReadVec3` run.
    #[test]
    fn path_data_read_markers_matches() {
        check(
            ee::s07::f_0026b880__generated,
            path_data_read_markers,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26b880, 0);
                c.pc = 0x0026_b880;
                let (this, file) = (OC431_BASE + 0x100, OC431_BASE + 0x1000);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = file as u64;
                fill(c, &mut r, this, 8);
                fill(c, &mut r, file, 0x200);
                let n = (seed % 5) as i32 - 1;
                c.bus.write32(file, n as u32);
                let keys = [11u32, 22, 33];
                for (j, k) in keys.iter().enumerate() {
                    c.bus.write32(OC431_PATH_TYPES + 4 * j as u32, *k);
                }
                for i in 0..4u32 {
                    let key = if r.next_u32() % 4 == 0 {
                        77
                    } else {
                        keys[(r.next_u32() % 3) as usize]
                    };
                    // node i starts at 4 + 40 * i: id, key, pos[3], dir[3], a, b
                    c.bus.write32(file + 4 + 40 * i + 4, key);
                    c.bus.write32(file + 4 + 40 * i + 32, r.next_u32() % 4);
                    c.bus.write32(file + 4 + 40 * i + 36, r.next_u32() % 4);
                }
            },
        );
    }

    /// First profile record of the bolts library (stride 0x9b50 per profile,
    /// 0xf88 per character).
    pub(crate) const OC431_PROFILES: u32 = 0x004a_6ca8;

    /// `Bolts_PrepareForGame`: three profiles x ten characters; per character
    /// the flag word (+2 of each 4-byte entry) of unlocked entries (bit 2)
    /// gains bit 4, as do the entries a bolt rule requires (rule rows give an
    /// entry index and an optional second one, -1 for none). The index
    /// rebuilds at the end are fakes.
    #[test]
    fn bolts_prepare_for_game_matches() {
        oc431_install();
        covered(
            ee::s01::f_0014d068__generated,
            bolts_prepare_for_game,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x14d068, 5, 0x0014_d068);
                let (lib, rules) = (OC431_BASE + 0x100, OC431_BASE + 0x800);
                c.g[A0][0] = lib as u64;
                c.bus.write32(lib + 0xc, rules);
                for ch in 0..10u32 {
                    c.bus.write32(lib + 0x2fc + 4 * ch, r.next_u32() % 4);
                    c.bus.write32(lib + 0x374 + 4 * ch, r.next_u32() % 3);
                }
                for k in 0..3u32 {
                    let a = (r.next_u32() % 6) as i32 - 1;
                    let b = (r.next_u32() % 6) as i32 - 1;
                    c.bus.write16(rules + 8 * k + 4, a as u16);
                    c.bus.write16(rules + 8 * k + 6, b as u16);
                }
                for p in 0..3u32 {
                    for ch in 0..10u32 {
                        let rec = OC431_PROFILES + p * 0x9b50 + ch * 0xf88;
                        let ids = OC431_BASE + 0x1000 + (p * 10 + ch) * 0x40;
                        c.bus.write32(rec + 0x28c, r.next_u32() % 5);
                        c.bus.write32(rec + 0x288, ids);
                        for e in 0..8u32 {
                            c.bus.write32(rec + 0x290 + 4 * e, r.next_u32());
                        }
                        for k in 0..16u32 {
                            let v = (r.next_u32() % 7) as i32 - 1;
                            c.bus.write16(ids + 2 * k, v as u16);
                        }
                    }
                }
            },
        );
    }

    /// `CameraTriggerMan_RemoveAllTriggers`: for each registered trigger
    /// (bounds through the shape's vtable, spatial-hash removal in the chosen
    /// cell), a cell that ends up empty (11 zero words) is freed. Then the
    /// stacks, lists and chase controllers are reset (fakes).
    #[test]
    fn camera_trigger_man_remove_all_triggers_matches() {
        oc431_install();
        covered(
            ee::s02::f_0016cbe0__generated,
            camera_trigger_man_remove_all_triggers,
            200,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x16cbe0, 20, 0x0016_cbe0);
                let (this, list, world, w2, w3, mgr, vt, cells) = (
                    OC431_BASE + 0x100,
                    OC431_BASE + 0x200,
                    OC431_BASE + 0x300,
                    OC431_BASE + 0x400,
                    OC431_BASE + 0x500,
                    OC431_BASE + 0x600,
                    OC431_BASE + 0x1000,
                    OC431_BASE + 0x2000,
                );
                c.g[A0][0] = this as u64;
                let n = r.next_u32() % 4;
                c.bus.write32(this, list);
                c.bus.write32(this + 4, n);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, w2);
                c.bus.write32(w2 + 0x20, mgr);
                let _ = w3;
                c.bus.write32(mgr + 0xa0, r.next_u32());
                c.bus.write16(vt + 0x30, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(vt + 0x34, FAKE);
                for i in 0..4u32 {
                    let trig = OC431_BASE + 0x3000 + 0x80 * i;
                    let shape = OC431_BASE + 0x3800 + 0x40 * i;
                    c.bus.write32(list + 4 * i, trig);
                    c.bus.write32(trig + 8, shape);
                    c.bus.write32(trig + 0x20, (r.next_u32() % 3 != 0) as u32);
                    c.bus.write32(shape + 0x24, vt);
                    fill(c, &mut r, trig + 0x10, 4);
                }
                // The box the fake bounds call leaves on the stack decides the cell:
                // negative coordinates pick the lower cells.
                for k in 0..3 {
                    let v = if r.next_u32() & 1 == 0 {
                        u32::MAX - 2
                    } else {
                        1 + r.next_u32() % 5
                    };
                    c.bus.write32(TEST_SP - 0xa0 + 0x24 + 4 * k, v);
                }
                // Each candidate cell holds a pointer (+0x10) to 11 words that
                // are all zero, or zero up to a random position.
                for (i, off) in [0u32, 0x14, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c]
                    .into_iter()
                    .enumerate()
                {
                    let cell = cells + 0x40 * i as u32;
                    c.bus.write32(mgr + off + 0x10, cell);
                    for k in 0..11u32 {
                        c.bus.write32(cell + 4 * k, 0);
                    }
                    if r.next_u32() % 3 != 0 {
                        c.bus
                            .write32(cell + 4 * (r.next_u32() % 11), 1 + r.next_u32() % 9);
                    }
                }
            },
        );
    }

    /// Per-location section lists (0x28 bytes per location: +4 count, +8 extra
    /// section, +0xc second extra section, +0x10 ids) and per-location modes.
    pub(crate) const OC431_LOCATIONS: u32 = 0x0044_2488;
    pub(crate) const OC431_LOCATION_MODES: u32 = 0x0044_2820;

    /// `SectionMan_LoadEvent`: a location keeps 0 to 4 sections (some of the
    /// protected 43..48), has or lacks the extra sections, and the mode is
    /// the caller's or (7) the location's own. Requests are fakes; the real
    /// `SectionMan_AdvanceEventSwitchSync` and `SectionMan_SelectPeakLighting`
    /// run (their jump tables are empty in the test RAM).
    #[test]
    fn section_man_load_event_matches() {
        oc431_install();
        covered(
            ee::s05::f_0022d088__generated,
            section_man_load_event,
            100,
            &|c, seed| {
                let mut r = oc431_machine(c, seed, 0x22d088, 60, 0x0022_d088);
                let this = OC431_BASE + 0x100;
                let idx = seed % 3;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = [7u32, 0, 3, 7][(r.next_u32() % 4) as usize] as u64;
                c.bus.write32(this + 0x1b4, r.next_u32() % 8);
                for i in 0..3u32 {
                    let loc = OC431_LOCATIONS + 0x28 * i;
                    c.bus.write32(loc + 4, r.next_u32() % 5);
                    let extra = [u32::MAX, 5, 49, 44][(r.next_u32() % 4) as usize];
                    c.bus.write32(loc + 8, extra);
                    let second = [u32::MAX, 9, 20][(r.next_u32() % 3) as usize];
                    c.bus.write32(loc + 0xc, second);
                    for k in 0..4u32 {
                        let id = [3u32, 43, 46, 12, 49, 0][(r.next_u32() % 6) as usize];
                        c.bus.write32(loc + 0x10 + 4 * k, id);
                    }
                    c.bus
                        .write32(OC431_LOCATION_MODES + 8 * i + 4, r.next_u32() % 6);
                }
            },
        );
    }

    /// Fixture of the file loaders: the file queue never opens the file
    /// (`EAFile_QueueOpen` fake returns 0), so every load returns 0 and its
    /// buffer is the fixture's.
    pub(crate) const OC431_PATH: u32 = OC431_BASE + 0x100;

    /// `Bolts_ReadFile`: the size query (real path parsing over the platform
    /// file fakes), the buffer (fake allocation forced to a fixture file image
    /// holding 0 to 2 entries and the three tables), and the pointer
    /// relocation of every entry's strings. The load itself does nothing.
    #[test]
    fn bolts_read_file_matches() {
        oc431_install();
        covered(ee::s01::f_0014be70__generated, bolts_read_file, 100, &|c, seed| {
            let mut r = oc431_machine(c, seed, 0x14be70, 8, 0x0014_be70);
            let this = OC431_BASE + 0x200;
            let image = OC431_BASE + 0x1000;
            c.g[A0][0] = this as u64;
            c.bus.write32(GAME_GP - 0x1f24, OC431_PATH);
            for (i, b) in b"data.bin\0".iter().enumerate() {
                c.bus.write8(OC431_PATH + i as u32, *b);
            }
            // The size query makes 3 fake calls (open, size, close), then
            // the allocation is call 3.
            oc431_force(c, 3, image);
            // File image: flags, n0, n0 x 0x38-byte entries, then three
            // (count, rows) tables of 12-, 8- and 8-byte rows, a 4-byte word.
            let n0 = r.next_u32() % 3;
            let mut at = image;
            let flags = r.next_u32();
            c.bus.write32(at, flags);
            c.bus.write32(at + 4, n0);
            at += 8;
            for _ in 0..n0 {
                for k in 0..14u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 0x40
                    };
                    c.bus.write32(at + 4 * k, v);
                }
                at += 0x38;
            }
            for row in [12u32, 8, 8] {
                let n = r.next_u32() % 3;
                c.bus.write32(at, n);
                at += 4;
                for k in 0..n * row / 4 {
                    c.bus.write32(at + 4 * k, r.next_u32());
                }
                at += n * row;
            }
            c.bus.write32(at, r.next_u32());
        });
    }

    /// `cWorldCache_Load`: the allocations and constructors are fakes, the
    /// file load returns 0 so the header is address 0 (its counts are written
    /// to the low words), the chunk resolver comes from a forced pointer with
    /// a vtable, the stream manager init and the sdb attach decide the result
    /// (0 or the attach's).
    #[test]
    fn world_cache_load_matches() {
        oc431_install();
        covered(ee::s13::f_003a7fc0__generated, world_cache_load, 100, &|c, seed| {
            let mut r = oc431_machine(c, seed, 0x3a7fc0, 20, 0x003a_7fc0);
            let (this, nash, table, resolver, rvt) = (
                OC431_BASE + 0x100,
                OC431_BASE + 0x400,
                OC431_BASE + 0x1000,
                OC431_BASE + 0x1400,
                OC431_BASE + 0x1500,
            );
            c.g[A0][0] = this as u64;
            c.g[A1][0] = OC431_PATH as u64;
            c.g[A2][0] = nash as u64;
            // Header words at address 0: the section count (8) and the next count (0xc).
            c.bus.write32(8, r.next_u32() % 3);
            c.bus.write32(0xc, 0);
            // Fake calls: 0 alloc, 1 memman ctor, 2-4 strings, 5 queue open,
            // 6 service queue, 7-9 string dtors, 10 alloc, 11 table ctor,
            // 12 section table allocation, 13 chunk resolver, 14 its vtable
            // call, 15 stream init, 16.. hull slots.
            oc431_force(c, 12, table);
            oc431_force(c, 13, resolver);
            c.bus.write32(resolver + 8, rvt);
            c.bus.write16(rvt + 0x10, (r.next_u32() & 0xfc) as u16);
            c.bus.write32(rvt + 0x14, FAKE);
            let init_ok = r.next_u32() % 4 != 0;
            oc431_force(c, 15, init_ok as u32);
            oc431_force(c, 16, r.next_u32() % 2);
        });
    }

    /// `InputPreproc_ProcessFile`: the name is formatted (fake `sprintf`), the
    /// file size query finds no file (queue fakes), then `ProcessText` (fake)
    /// returns nonzero 0 to 2 times and then 0.
    #[test]
    fn input_preproc_process_file_matches() {
        oc431_install();
        covered(
            ee::s10::f_00326078__generated,
            input_preproc_process_file,
            100,
            &|c, seed| {
                let _ = oc431_machine(c, seed, 0x326078, 8, 0x0032_6078);
                c.g[A0][0] = (OC431_BASE + 0x100) as u64;
                c.g[A1][0] = (OC431_BASE + 0x200) as u64;
                c.g[A2][0] = (OC431_BASE + 0x300) as u64;
                // Fake calls: 0 sprintf, 1 queue open, 2 service queue, 3 next
                // char, 4 line marker, 5.. process text.
                let more = seed % 3;
                for k in 0..more {
                    oc431_force(c, 5 + k, 1);
                }
                oc431_force(c, 5 + more, 0);
            },
        );
    }
}
