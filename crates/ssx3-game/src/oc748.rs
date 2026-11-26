//! Staging module oc748 (wave wave243): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc748`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::back_edge_unwinds;
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_1620,
        cfe_state_char_select_on_widget_created,
        "cFEStateCharSelect_OnWidgetCreated",
    ),
    (0x0019_d000, bolt_file_unpack_slot, "BoltFile_UnpackSlot"),
    (
        0x0019_d140,
        bolt_file_start_next_queued,
        "BoltFile_StartNextQueued",
    ),
    (0x0019_d8b8, bolt_file_load_slot, "BoltFile_LoadSlot"),
];

/// `void cFEStateCharSelect_OnWidgetCreated(cFEStateCharSelect* self, Widget* w)`
/// (0x181620): sets a widget of the character-select screen up by its name
/// hash (`w+0x38`). The menu-names widget only gets its flag word (`+0x14`)
/// fixed; the title widgets call the widget's own refresh (vtable `+0x48`),
/// or with game path 2 the per-player title text; the ten character keys
/// `"0".."9"` get the character's short name, their index in `w+0x18`, and the
/// same refresh. Written as the generated `match pc` body, the normal entry
/// only; a resume at another `pc` runs the generated body.
pub fn cfe_state_char_select_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x00181620 {
        return ee::s02::f_00181620__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00181620;
        loop {
            match pc {
                0x00181620 => {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = 0x460000;
                    c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x0018164c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x29f0));
                    call_ptr(c, 0x00317670, 0x0018164c)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[V1][0] = 0xfffffffffffffffe;
                    if t {
                        pc = 0x00181668;
                        continue;
                    }
                    pc = 0x00181654;
                    continue;
                }
                0x00181654 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[V0][0] = c.g[V0][0] | 0x80;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    if t {
                        pc = 0x001817cc;
                        continue;
                    }
                    pc = 0x00181668;
                    continue;
                }
                0x00181668 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00181670);
                    call_ptr(c, 0x0014dc80, 0x00181670)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0018167c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0018167c)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x00181688);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x29e0));
                    call_ptr(c, 0x00317670, 0x00181688)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001816b0;
                        continue;
                    }
                    pc = 0x00181690;
                    continue;
                }
                0x00181690 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x530000;
                    c.g[A0][0] = 0x2;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001816b0;
                        continue;
                    }
                    pc = 0x001816a4;
                    continue;
                }
                0x001816a4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00181770;
                        continue;
                    }
                    pc = 0x001816b0;
                    continue;
                }
                0x001816b0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001816b8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x29c8));
                    call_ptr(c, 0x00317670, 0x001816b8)?;
                    let t = c.g[S2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00181788;
                        continue;
                    }
                    pc = 0x001816c0;
                    continue;
                }
                0x001816c0 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x530000;
                    c.g[A0][0] = 0x2;
                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));

                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        pc = 0x00181720;
                        continue;
                    }
                    pc = 0x001816d4;
                    continue;
                }
                0x001816d4 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                    let t = c.g[A0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x001816f0;
                        continue;
                    }
                    pc = 0x001816e0;
                    continue;
                }
                0x001816e0 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001816fc;
                        continue;
                    }
                    pc = 0x001816e8;
                    continue;
                }
                0x001816e8 => {
                    c.cycles += 2;
                    let t = true;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x001817d4;
                        continue;
                    }
                    pc = 0x001816f0;
                    continue;
                }
                0x001816f0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x29a8));
                    if t {
                        pc = 0x00181704;
                        continue;
                    }
                    pc = 0x001816fc;
                    continue;
                }
                0x001816fc => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2988));
                    pc = 0x00181704;
                    continue;
                }
                0x00181704 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0018170c);
                    call_ptr(c, 0x00317670, 0x0018170c)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00181718);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00181718)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001817d0;
                        continue;
                    }
                    pc = 0x00181720;
                    continue;
                }
                0x00181720 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00181770;
                        continue;
                    }
                    pc = 0x00181728;
                    continue;
                }
                0x00181728 => {
                    c.cycles += 18;
                    c.g[RA][0] = sx(0x00181730);
                    call_ptr(c, 0x0014dc80, 0x00181730)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0018173c);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0014dd58, 0x0018173c)?;
                    c.g[V1][0] = 0x440000;
                    c.g[A2][0] = 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf68));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x00181758);
                    c.g[A1][0] = c.lb(c.g[V1][0] as u32);
                    call_ptr(c, 0x0014eec8, 0x00181758)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00181764);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003a0c50, 0x00181764)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S0][0]);
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x00181770;
                    continue;
                }
                0x00181770 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00181780);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00181780)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x001817d0;
                        continue;
                    }
                    pc = 0x00181788;
                    continue;
                }
                0x00181788 => {
                    c.cycles += 8;
                    c.g[S0][0] = 0u64;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13a0));
                    c.g[V1][0] = c.lb(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.sb(c.g[SP][0] as u32, c.g[V1][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                    pc = 0x001817a8;
                    continue;
                }
                0x001817a8 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001817b4);
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001817b4)?;
                    let t = c.g[S2][0] == c.g[V0][0];

                    if t {
                        if back_edge_unwinds(c, 0x00181728) {
                            return Ok(());
                        }
                        pc = 0x00181728;
                        continue;
                    }
                    pc = 0x001817bc;
                    continue;
                }
                0x001817bc => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                    if t {
                        if back_edge_unwinds(c, 0x001817a8) {
                            return Ok(());
                        }
                        pc = 0x001817a8;
                        continue;
                    }
                    pc = 0x001817cc;
                    continue;
                }
                0x001817cc => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = 0x001817d0;
                    continue;
                }
                0x001817d0 => {
                    c.cycles += 1;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x001817d4;
                    continue;
                }
                0x001817d4 => {
                    c.cycles += 4;
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00181620, 0x001817e4);
                    return Ok(());
                }
            }
        }
    });
}

/// `void BoltFile_UnpackSlot(BoltTable* self, int bank, BoltSlot* slot)`
/// (0x19d000): replaces the compressed data of a loaded slot (`+0x104`) by
/// its unpacked copy: asks for the unpacked size, makes room
/// (`BoltFile_EnsureBudget`), allocates the buffer (evicting and retrying
/// while the allocator fails), unpacks, frees the old data and books the size
/// difference in the table's memory counter (`+0xb4000`).
pub fn bolt_file_unpack_slot(c: &mut Cpu) {
    guest(c, |c| {
        'b_19d118: {
            'b_19d114: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A2][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S4][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S5][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S2][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_19d114;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                    let t = c.g[A0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    if t {
                        break 'b_19d118;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x0019d048);
                        call_ptr(c, 0x003b4818, 0x0019d048)?;
                        c.g[S1][0] = c.g[V0][0];
                        let t = (c.g[S1][0] as i64) <= 0;
                        c.g[A0][0] = c.g[S5][0];
                        if t {
                            break 'b_19d114;
                        } else {
                            'b_19d0d4: {
                                c.cycles += 13;
                                c.g[A1][0] = c.g[S4][0];
                                c.g[A2][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x0019d064);
                                c.g[S0][0] = 0x460000;
                                Callee {
                                    addr: 0x0019d578,
                                    f: ee::s03::BoltFile_EnsureBudget,
                                }
                                .call(c, 0x0019d064)?;
                                c.g[S6][0] = c.g[S0][0];
                                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9d8));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = 0x60000000;
                                c.g[RA][0] = sx(0x0019d07c);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317e30, 0x0019d07c)?;
                                c.g[S0][0] = c.g[V0][0];
                                if c.g[S0][0] != 0u64 {
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                                    break 'b_19d0d4;
                                } else {
                                    c.cycles += 2;
                                    c.g[V0][0] = 0x540000;
                                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7500));
                                    'l_19d090: loop {
                                        c.cycles += 16;
                                        c.g[RA][0] = sx(0x0019d098);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x003e5700, 0x0019d098)?;
                                        c.g[RA][0] = sx(0x0019d0a0);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x003e5760, 0x0019d0a0)?;
                                        c.g[A0][0] = c.g[S5][0];
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[RA][0] = sx(0x0019d0b0);
                                        c.g[A2][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x0019d738,
                                            f: ee::s03::BoltFile_EvictBytes,
                                        }
                                        .call(c, 0x0019d0b0)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x9d8));
                                        c.g[A2][0] = 0x60000000;
                                        c.g[RA][0] = sx(0x0019d0c4);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317e30, 0x0019d0c4)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        let t = c.g[S0][0] == 0u64;

                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0019d090);
                                            }
                                            continue 'l_19d090;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                                            break 'b_19d0d4;
                                        }
                                    }
                                }
                            }
                            'b_19d0f4: {
                                c.cycles += 5;
                                c.g[RA][0] = sx(0x0019d0dc);
                                c.g[A1][0] = c.g[S0][0];
                                call_ptr(c, 0x003b47f8, 0x0019d0dc)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                                if c.g[A0][0] == 0u64 {
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x104), c.g[S0][0]);
                                    break 'b_19d0f4;
                                } else {
                                    c.cycles += 3;
                                    c.g[RA][0] = sx(0x0019d0f0);
                                    call_ptr(c, 0x00317e98, 0x0019d0f0)?;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x104), c.g[S0][0]);
                                    break 'b_19d0f4;
                                }
                            }
                            c.cycles += 8;
                            c.g[V0][0] = 0xb0000;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4000));
                            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4000), c.g[A0][0]);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x118), c.g[S1][0]);
                            break 'b_19d114;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_19d118;
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `void BoltFile_StartNextQueued(BoltTable* self, int bank)` (0x19d140):
/// picks the queued slot (flags 3) with the oldest stamp (`+0x11c`) in
/// `bank`, else in the first other bank that has one, and loads it
/// (`BoltFile_LoadSlot`).
pub fn bolt_file_start_next_queued(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = 0x10000;
        c.g[T6][0] = c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0x2000;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.mult(0, 3, 14, 2);
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[T2][0] = 0u64;
        c.g[A2][0] = 0xffffffffffffffff;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0x3;
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        'l_19d170: loop {
            'b_19d19c: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                if c.g[V0][0] != c.g[T1][0] {
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    break 'b_19d19c;
                } else {
                    'b_19d190: {
                        c.cycles += 2;
                        let t = c.g[T2][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x11c));
                        if t {
                            break 'b_19d190;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V1][0] < c.g[T2][0]) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                break 'b_19d19c;
                            } else {
                                break 'b_19d190;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[T2][0] = c.g[V1][0];
                    c.g[A2][0] = c.g[T0][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    break 'b_19d19c;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T0][0] as i64) < 256i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x120));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0019d170);
                }
                continue 'l_19d170;
            } else {
                'b_19d244: {
                    'b_19d238: {
                        'b_19d230: {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffffffffffffffff;
                            let t = c.g[A2][0] != c.g[V0][0];

                            if t {
                                break 'b_19d230;
                            } else {
                                c.cycles += 5;
                                c.g[T3][0] = 0x10000;
                                c.g[T1][0] = 0u64;
                                c.g[T5][0] = 0u64;
                                c.g[T3][0] = c.g[T3][0] | 0x2000;
                                'l_19d1c8: loop {
                                    'b_19d224: {
                                        c.cycles += 2;
                                        if c.g[T1][0] == c.g[T6][0] {
                                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                            break 'b_19d224;
                                        } else {
                                            c.cycles += 4;
                                            c.g[A3][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[T5][0] as u32));
                                            c.g[T0][0] = 0u64;
                                            c.g[T4][0] = 0x3;
                                            'l_19d1e0: loop {
                                                'b_19d210: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                    if c.g[V0][0] != c.g[T4][0] {
                                                        c.g[T0][0] =
                                                            sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        break 'b_19d210;
                                                    } else {
                                                        'b_19d200: {
                                                            c.cycles += 2;
                                                            let t = c.g[T2][0] == 0u64;
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x11c));
                                                            if t {
                                                                break 'b_19d200;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = (c.g[V1][0]
                                                                    < c.g[T2][0])
                                                                    as u64;
                                                                if c.g[V0][0] == 0u64 {
                                                                    c.g[T0][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    break 'b_19d210;
                                                                } else {
                                                                    break 'b_19d200;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[T2][0] = c.g[V1][0];
                                                        c.g[A1][0] = c.g[T1][0];
                                                        c.g[A2][0] = c.g[T0][0];
                                                        c.g[T0][0] =
                                                            sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        break 'b_19d210;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = ((c.g[T0][0] as i64) < 256i64) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x120));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0019d1e0);
                                                    }
                                                    continue 'l_19d1e0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = (c.g[A2][0] as i64) >= 0;
                                                    c.g[T1][0] =
                                                        sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                                    if t {
                                                        break 'b_19d238;
                                                    } else {
                                                        break 'b_19d224;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[T1][0] as i64) < 10i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[T5][0] =
                                        sx((c.g[T5][0] as u32).wrapping_add(c.g[T3][0] as u32));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0019d1c8);
                                        }
                                        continue 'l_19d1c8;
                                    } else {
                                        break 'b_19d230;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = (c.g[A2][0] as i64) < 0;
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        if t {
                            break 'b_19d244;
                        } else {
                            break 'b_19d238;
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0019d240);
                    Callee {
                        addr: 0x0019d8b8,
                        f: ee::s03::BoltFile_LoadSlot,
                    }
                    .call(c, 0x0019d240)?;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    break 'b_19d244;
                }
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void BoltFile_LoadSlot(BoltTable* self, int bank, int idx)` (0x19d8b8):
/// asks the file's size, makes room, allocates the data buffer (evicting and
/// retrying while the allocator fails), books the size, and loads the file:
/// synchronously (flags 1) or as an asynchronous read (flags 2, counted in
/// `+0xb4004`).
pub fn bolt_file_load_slot(c: &mut Cpu) {
    guest(c, |c| {
        'b_19d984: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = 0x10000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.g[V0][0] | 0x2000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = c.g[A0][0];
            c.g[S4][0] = c.g[A1][0];
            c.g[V1][0] = 0x120;
            c.mult(0, 4, 20, 2);
            c.mult(0, 6, 6, 3);
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = 0x460000;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x0019d90c);
            c.g[S5][0] = c.g[S0][0];
            call_ptr(c, 0x00317f60, 0x0019d90c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0019d920);
            c.g[A2][0] = c.g[S1][0];
            Callee {
                addr: 0x0019d578,
                f: ee::s03::BoltFile_EnsureBudget,
            }
            .call(c, 0x0019d920)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9f0));
            c.g[A2][0] = 0x60000000;
            c.g[RA][0] = sx(0x0019d934);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x0019d934)?;
            let t = c.g[V0][0] != 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x104), c.g[V0][0]);
            if t {
                break 'b_19d984;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x540000;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7500));
                'l_19d948: loop {
                    c.cycles += 15;
                    c.g[RA][0] = sx(0x0019d950);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003e5700, 0x0019d950)?;
                    c.g[RA][0] = sx(0x0019d958);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003e5760, 0x0019d958)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0019d968);
                    c.g[A2][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0019d738,
                        f: ee::s03::BoltFile_EvictBytes,
                    }
                    .call(c, 0x0019d968)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x9f0));
                    c.g[A2][0] = 0x60000000;
                    c.g[RA][0] = sx(0x0019d97c);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x0019d97c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019d948);
                        }
                        continue 'l_19d948;
                    } else {
                        break 'b_19d984;
                    }
                }
            }
        }
        'b_19d9ec: {
            'b_19d9d0: {
                c.cycles += 8;
                c.g[V0][0] = 0xb0000;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4000));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4000), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10c));
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x2;
                if t {
                    break 'b_19d9d0;
                } else {
                    c.cycles += 11;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x118), c.g[S1][0]);
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
                    c.g[A2][0] = c.g[S1][0];
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[RA][0] = sx(0x0019d9bc);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x003df748, 0x0019d9bc)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4004));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4004), c.g[V0][0]);
                    if t {
                        break 'b_19d9ec;
                    } else {
                        break 'b_19d9d0;
                    }
                }
            }
            c.cycles += 7;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x118), c.g[S1][0]);
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x104));
            c.g[RA][0] = sx(0x0019d9e4);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x003e1bb8, 0x0019d9e4)?;
            c.g[V1][0] = 0x1;
            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
            break 'b_19d9ec;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    const FX_CTRL: u32 = DATA + 0x20_0000;
    const FX_MAGIC: u32 = 0x4f43_3734;
    /// Allocation fake: calls made (`+0x10`) and calls that fail first (`+0x14`).
    const FX_ALLOC_COUNT: u32 = FX_CTRL + 0x10;
    const FX_ALLOC_FAILS: u32 = FX_CTRL + 0x14;
    const FX_SELF: u32 = DATA + 0x21_0000;
    const FX_WIDGET: u32 = DATA + 0x22_0000;
    const FX_VTABLE: u32 = DATA + 0x23_0000;
    const FX_PACKED: u32 = DATA + 0x24_0000;
    /// The Bolt file table (`cFEBoltFileManager`): 10 banks of 256 slots,
    /// then the memory counters (`+0xb4000` bytes used, `+0xb4004` async loads).
    const FX_TABLE: u32 = DATA + 0x30_0000;

    /// The fake callee: logs the call like `fakes::fake_call`, then gives the
    /// results the ported code's branches need: `GetHashValue32` (an
    /// injective hash: static strings hash to their address, the key buffer
    /// on the stack to `0x70000000 + first byte`), a file size from
    /// `BXFILE_sizez`, and an allocator that fails its first calls.
    fn oc748_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0031_7670 => {
                c.g[V0][0] = if a0 >= 0x0100_0000 {
                    let b = c.bus.read8(a0);
                    0x7000_0000 + b as u64
                } else {
                    a0 as u64
                };
            }
            0x0031_7f60 => c.g[V0][0] = 0x1000 + (a0 & 0xff0) as u64,
            0x0031_7e30 => {
                let n = c.bus.read32(FX_ALLOC_COUNT);
                c.bus.write32(FX_ALLOC_COUNT, n + 1);
                if n < c.bus.read32(FX_ALLOC_FAILS) {
                    c.g[V0][0] = 0;
                }
            }
            _ => {}
        }
    }

    macro_rules! oc748_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL) != FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc748_fake_call(c, $addr);
                }
            )*
            fn oc748_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc748_fakes!(
        (
            oc748_fake_317670,
            OC748_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc748_fake_14dc80,
            OC748_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc748_fake_14dd58,
            OC748_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc748_fake_14eec8,
            OC748_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc748_fake_3a1000,
            OC748_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc748_fake_3a0c70,
            OC748_PREV_3A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc748_fake_3b47f8,
            OC748_PREV_3B47F8,
            ee::s13::OVERRIDE_003B47F8,
            ee::s13::f_003b47f8__generated,
            0x003b_47f8
        ),
        (
            oc748_fake_317e30,
            OC748_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc748_fake_317e98,
            OC748_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc748_fake_317f60,
            OC748_PREV_317F60,
            ee::s10::OVERRIDE_00317F60,
            ee::s10::f_00317f60__generated,
            0x0031_7f60
        ),
        (
            oc748_fake_3e5700,
            OC748_PREV_3E5700,
            ee::s14::OVERRIDE_003E5700,
            ee::s14::f_003e5700__generated,
            0x003e_5700
        ),
        (
            oc748_fake_3e5760,
            OC748_PREV_3E5760,
            ee::s14::OVERRIDE_003E5760,
            ee::s14::f_003e5760__generated,
            0x003e_5760
        ),
        (
            oc748_fake_3df748,
            OC748_PREV_3DF748,
            ee::s14::OVERRIDE_003DF748,
            ee::s14::f_003df748__generated,
            0x003d_f748
        ),
        (
            oc748_fake_3e1bb8,
            OC748_PREV_3E1BB8,
            ee::s14::OVERRIDE_003E1BB8,
            ee::s14::f_003e1bb8__generated,
            0x003e_1bb8
        ),
    );

    /// Machine state of the four tests: scrambled registers, fake hooks and
    /// their gate, the allocator's failure count, and the junk below `sp`.
    fn oc748_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(FX_CTRL, FX_MAGIC);
        c.bus.write32(FX_ALLOC_COUNT, 0);
        c.bus.write32(FX_ALLOC_FAILS, (seed / 7) % 3);
        for i in 1..=0x300u32 {
            let w = if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32()
            };
            c.bus.write32(crate::difftest::SP - 4 * i, w);
        }
        r
    }

    /// Address of slot `idx` of `bank` in the fixture table.
    fn oc748_slot(bank: u32, idx: u32) -> u32 {
        FX_TABLE + bank * 0x12000 + idx * 0x120
    }

    /// A queued slot: `flags`, a load-time stamp (`+0x11c`), a data pointer
    /// (never with flags 0, which would send the real `EvictBytes` into
    /// `FreeSlot`), the async flag, and sizes.
    fn oc748_put_slot(c: &mut Cpu, r: &mut Rng, bank: u32, idx: u32, flags: u32, stamp: u32) {
        let s = oc748_slot(bank, idx);
        c.bus.write32(s, flags);
        c.bus
            .write32(s + 0x104, 0x0090_0000 + (r.next_u32() & 0xfff0));
        c.bus.write32(s + 0x10c, r.next_u32() & 1);
        c.bus.write32(s + 0x114, r.next_u32() & 1);
        c.bus.write32(s + 0x118, r.next_u32() & 0xffff);
        c.bus.write32(s + 0x11c, stamp);
        c.bus.write32(s + 4, r.next_u32());
    }

    /// `cFEStateCharSelect_OnWidgetCreated`: each widget hash (menu title,
    /// the two titles with `cGamePath` 2 or not, the per-user text for
    /// users 0, 1 and neither, the ten character keys, no match); the
    /// widget's vtable call is a fake.
    #[test]
    fn cfe_state_char_select_on_widget_created_matches() {
        oc748_install_fakes();
        fakes::covered(
            ee::s02::f_00181620__generated,
            cfe_state_char_select_on_widget_created,
            450,
            &|c, seed| {
                let mut r = oc748_fixture(c, seed, 0x0018_1620, 10);
                let sel = seed % 15;
                let hash = match sel {
                    0 => 0x0045_d610,
                    1 => 0x0045_d620,
                    2 => 0x0045_d638,
                    3..=12 => 0x7000_0030 + (sel - 3),
                    _ => r.next_u32() | 1,
                };
                c.bus.write32(FX_WIDGET + 0x38, hash);
                c.bus.write32(FX_WIDGET + 0x14, r.next_u32());
                c.bus.write32(FX_WIDGET + 0x18, r.next_u32());
                c.bus.write32(FX_WIDGET + 8, FX_VTABLE);
                c.bus.write16(FX_VTABLE + 0x48, 0x10 * (seed % 3) as u16);
                c.bus.write32(FX_VTABLE + 0x4c, fakes::FAKE + 0x10);
                // cGamePath (byte at 0x535c11): 2 on every other group.
                let path = if (seed / 15) % 2 == 0 {
                    2
                } else {
                    seed as u8 % 2
                };
                c.bus.write8(0x0053_5c11, path);
                // The state object: mUserNum 0, 1 or 2 (neither).
                c.bus.write8(FX_SELF + 0x44, ((seed / 30) % 3) as u8);
                // The character table and the key template ("0" + junk).
                for i in 0..10u32 {
                    c.bus.write8(0x0044_0f68 + i, r.next_u32() as u8);
                }
                c.bus.write8(0x004a_13a0, r.next_u32() as u8);
                c.bus.write8(0x004a_13a1, r.next_u32() as u8);
                c.g[A0][0] = sx(FX_SELF);
                c.g[A1][0] = sx(FX_WIDGET);
                c.pc = 0x0018_1620;
            },
        );
    }

    /// The compressed data of an unpack test: a header byte (known size
    /// formats and unknown ones), the `0xfb` marker (or not) and a size that
    /// is positive, zero or (32-bit formats) negative.
    fn oc748_packed(c: &mut Cpu, r: &mut Rng, seed: u32) {
        const HDRS: [u8; 8] = [0x10, 0x11, 0x90, 0x91, 0x30, 0xb0, 0x34, 0x77];
        let hdr = HDRS[(seed % 8) as usize];
        c.bus.write8(FX_PACKED, hdr);
        c.bus
            .write8(FX_PACKED + 1, if (seed / 8) % 6 == 0 { 0x12 } else { 0xfb });
        let size = (r.next_u32() % 100) + 1;
        let (b2, b3, b4, b5) = match (seed / 5) % 5 {
            0 => (0, 0, 0, 0),
            1 => (0x80, 0, 0, 0),
            _ => (0, 0, size as u8, (r.next_u32() % 3) as u8),
        };
        c.bus.write8(FX_PACKED + 2, b2);
        c.bus.write8(FX_PACKED + 3, b3);
        c.bus.write8(FX_PACKED + 4, b4);
        c.bus.write8(FX_PACKED + 5, b5);
    }

    /// `BoltFile_UnpackSlot`: no slot, no data, unknown or empty or negative
    /// sizes, and a real unpack with the allocator failing 0 to 2 times (the
    /// real `EnsureBudget` returns at once, the real `EvictBytes` finds no
    /// slot to free).
    #[test]
    fn bolt_file_unpack_slot_matches() {
        oc748_install_fakes();
        fakes::covered(
            ee::s03::f_0019d000__generated,
            bolt_file_unpack_slot,
            400,
            &|c, seed| {
                let mut r = oc748_fixture(c, seed, 0x0019_d000, 8);
                let bank = seed % 10;
                let slot = oc748_slot(bank, (seed / 10) % 256);
                c.bus
                    .write32(slot + 0x104, if seed % 11 == 1 { 0 } else { FX_PACKED });
                c.bus.write32(slot + 0x118, r.next_u32() & 0xffff);
                // Flags non-zero: the real `EvictBytes` must not free this slot.
                c.bus.write32(slot, 1);
                c.bus.write32(FX_TABLE + 0xb4000, r.next_u32() & 0xffff);
                oc748_packed(c, &mut r, seed);
                c.g[A0][0] = sx(FX_TABLE);
                c.g[A1][0] = bank as u64;
                c.g[A2][0] = if seed % 13 == 0 { 0 } else { sx(slot) };
                c.pc = 0x0019_d000;
            },
        );
    }

    /// `BoltFile_StartNextQueued`: no queued slot, the oldest queued slot in
    /// the own bank (ties and the stamp-0 rule), and a queued slot found only
    /// in another bank; the real `LoadSlot` runs on the fakes.
    #[test]
    fn bolt_file_start_next_queued_matches() {
        oc748_install_fakes();
        fakes::covered(
            ee::s03::f_0019d140__generated,
            bolt_file_start_next_queued,
            300,
            &|c, seed| {
                let mut r = oc748_fixture(c, seed, 0x0019_d140, 12);
                let bank = seed % 10;
                // Decoys that are not queued (flags 1 and 2).
                for k in 0..3u32 {
                    let b = r.next_u32() % 10;
                    let i = r.next_u32() % 256;
                    let stamp = r.next_u32() % 4;
                    oc748_put_slot(c, &mut r, b, i, 1 + (k & 1), stamp);
                }
                match (seed / 10) % 3 {
                    0 => {}
                    1 => {
                        for _ in 0..1 + r.next_u32() % 3 {
                            let i = r.next_u32() % 256;
                            let stamp = r.next_u32() % 3;
                            oc748_put_slot(c, &mut r, bank, i, 3, stamp);
                        }
                    }
                    _ => {
                        let b = (bank + 1 + r.next_u32() % 9) % 10;
                        for _ in 0..1 + r.next_u32() % 3 {
                            let i = r.next_u32() % 256;
                            let stamp = r.next_u32() % 3;
                            oc748_put_slot(c, &mut r, b, i, 3, stamp);
                        }
                        if seed % 2 == 0 {
                            let b2 = (b + 1 + r.next_u32() % 9) % 10;
                            if b2 != bank {
                                let i = r.next_u32() % 256;
                                oc748_put_slot(c, &mut r, b2, i, 3, 1);
                            }
                        }
                    }
                }
                c.bus.write32(FX_TABLE + 0xb4000, 0);
                c.g[A0][0] = sx(FX_TABLE);
                c.g[A1][0] = bank as u64;
                c.pc = 0x0019_d140;
            },
        );
    }

    /// `BoltFile_LoadSlot`: synchronous and asynchronous slots, the allocator
    /// failing 0 to 2 times first (the real `EvictBytes` finds nothing to
    /// free).
    #[test]
    fn bolt_file_load_slot_matches() {
        oc748_install_fakes();
        fakes::covered(
            ee::s03::f_0019d8b8__generated,
            bolt_file_load_slot,
            300,
            &|c, seed| {
                let mut r = oc748_fixture(c, seed, 0x0019_d8b8, 8);
                let bank = seed % 10;
                let idx = r.next_u32() % 256;
                let slot = oc748_slot(bank, idx);
                c.bus.write32(slot + 0x10c, (seed / 10) % 2);
                c.bus.write32(slot + 4, r.next_u32());
                c.bus.write32(FX_TABLE + 0xb4000, r.next_u32() & 0xffff);
                c.bus.write32(FX_TABLE + 0xb4004, r.next_u32() & 0xff);
                c.g[A0][0] = sx(FX_TABLE);
                c.g[A1][0] = bank as u64;
                c.g[A2][0] = idx as u64;
                c.pc = 0x0019_d8b8;
            },
        );
    }
}
