//! Staging module oc857 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc857`.

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
    (
        0x0019_a638,
        cfe_state_char_equip_update_preview,
        "cFEStateCharEquip_UpdatePreview",
    ),
    (
        0x0019_a758,
        cfe_state_char_equip_release_preview_bolt_textures,
        "cFEStateCharEquip_ReleasePreviewBoltTextures",
    ),
    (
        0x0019_a798,
        cfe_state_char_equip_update_help_text,
        "cFEStateCharEquip_UpdateHelpText",
    ),
    (
        0x0019_aa08,
        cfe_state_char_equip_refresh_equip_info,
        "cFEStateCharEquip_RefreshEquipInfo",
    ),
    (
        0x0019_ab78,
        cfe_state_char_equip_refresh_buy_info,
        "cFEStateCharEquip_RefreshBuyInfo",
    ),
    (
        0x0019_b180,
        cfe_state_char_equip_build_item_list,
        "cFEStateCharEquip_BuildItemList",
    ),
];

/// `void cFEStateCharEquip_UpdatePreview(cFEStateCharEquip* self)` (0x19a638): per-frame
/// load of the large item preview texture. In buy mode (`+0xc8 == 1`) it counts
/// down the delay at `+0x95c`; at zero it acquires the preview bolt file
/// (`+0xa6c` = file index). Once the file is loaded, its first entry becomes the
/// preview texture (`+0xa70`, through the graphics manager's AddTexture vtable
/// slot), the file is released and the item info is refreshed.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_update_preview(c: &mut Cpu) {
    guest(c, |c| {
        'b_19a744: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_19a744;
            } else {
                'b_19a740: {
                    'b_19a6a4: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x95c));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        if t {
                            break 'b_19a6a4;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x95c), c.g[V0][0]);
                            if t {
                                break 'b_19a740;
                            } else {
                                c.cycles += 13;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x960));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                                c.g[A3][0] = 0x9;
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                                c.g[T0][0] = 0x1;
                                c.g[T1][0] = 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
                                Callee {
                                    addr: 0x0019da10,
                                    f: ee::s03::f_0019da10,
                                }
                                .call(c, 0x0019a694)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa6c), c.g[V0][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x95c), c.g[V1][0]);
                                break 'b_19a740;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa6c));
                    let t = (c.g[A2][0] as i64) < 0;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_19a740;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a70));
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x0019e238,
                            f: ee::s03::f_0019e238,
                        }
                        .call(c, 0x0019a6c4)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_19a740;
                        } else {
                            'b_19a720: {
                                'b_19a718: {
                                    c.cycles += 5;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa6c));
                                    Callee {
                                        addr: 0x0019e2b0,
                                        f: ee::s03::f_0019e2b0,
                                    }
                                    .call(c, 0x0019a6d8)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    if t {
                                        break 'b_19a718;
                                    } else {
                                        c.cycles += 14;
                                        c.g[A2][0] = 0x4a0000;
                                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1960));
                                        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                                        c.g[A3][0] = 0u64;
                                        c.g[A1][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        c.g[T0][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                                        c.g[T1][0] = 0xffffffffffffffff;
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[T2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x0019a710)?;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa70), c.g[V0][0]);
                                        break 'b_19a720;
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = 0xffffffffffffffff;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xa70), c.g[V0][0]);
                                break 'b_19a720;
                            }
                            c.cycles += 8;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa6c));
                            Callee {
                                addr: 0x0019dc20,
                                f: ee::s03::f_0019dc20,
                            }
                            .call(c, 0x0019a730)?;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.g[A0][0] = c.g[S0][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xa6c), c.g[V0][0]);
                            Callee {
                                addr: 0x0019a9b8,
                                f: ee::s03::f_0019a9b8,
                            }
                            .call(c, 0x0019a740)?;
                            break 'b_19a740;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_19a744;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_ReleasePreviewBoltTextures(self)` (0x19a758): if the
/// front end exists, ask its rider preview for the user's rider (`self+0x44`)
/// and release that rider's bolt textures.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_release_preview_bolt_textures(c: &mut Cpu) {
    guest(c, |c| {
        'b_19a790: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.g[A0][0];
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            let t = c.g[A0][0] == 0u64;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            if t {
                break 'b_19a790;
            } else {
                c.cycles += 7;
                c.g[A1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                Callee {
                    addr: 0x001a0548,
                    f: ee::s03::f_001a0548,
                }
                .call(c, 0x0019a780)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x0019fa78,
                    f: ee::s03::f_0019fa78,
                }
                .call(c, 0x0019a78c)?;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                break 'b_19a790;
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_UpdateHelpText(self)` (0x19a798): hide the help text
/// widget (`+0x4c`), choose the help string for the selected item (no items,
/// category row, affordable or not in buy mode, outfit polygon budget in equip
/// mode), set it, and show the widget again.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_update_help_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_19a9a4: {
            'b_19a9a0: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    break 'b_19a9a0;
                } else {
                    'b_19a85c: {
                        'b_19a7f4: {
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0019a7d0)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[S2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                            if (c.g[S2][0] as i64) < 0 {
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                break 'b_19a7f4;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                                if t {
                                    break 'b_19a85c;
                                } else {
                                    c.cycles += 1;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                    break 'b_19a7f4;
                                }
                            }
                        }
                        'b_19a840: {
                            'b_19a83c: {
                                c.cycles += 10;
                                c.g[A1][0] = 0x1;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0019a80c)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                if c.g[V0][0] == 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_19a83c;
                                } else {
                                    'b_19a834: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xae4));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A0][0] = 0x460000;
                                            break 'b_19a834;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x8d8));
                                            break 'b_19a840;
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8f0));
                                    break 'b_19a840;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x908));
                            break 'b_19a840;
                        }
                        c.cycles += 7;
                        call_ptr(c, 0x00317670, 0x0019a848)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x0019a854)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_19a9a4;
                    }
                    'b_19a988: {
                        'b_19a970: {
                            'b_19a954: {
                                'b_19a87c: {
                                    c.cycles += 6;
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                    c.g[V1][0] = c.g[V1][0] & 0x20;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_19a87c;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x920));
                                        break 'b_19a954;
                                    }
                                }
                                'b_19a8c0: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_19a8c0;
                                    } else {
                                        'b_19a89c: {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                                            let t = (c.g[V1][0] as i64) <= 0;
                                            c.g[V0][0] = 0xa;
                                            if t {
                                                break 'b_19a89c;
                                            } else {
                                                c.cycles += 1;
                                                c.mult(0, 3, 3, 2);
                                                break 'b_19a89c;
                                            }
                                        }
                                        'b_19a8b8: {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A0][0] = 0x460000;
                                                break 'b_19a8b8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x460000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x940));
                                                break 'b_19a954;
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x958));
                                        break 'b_19a954;
                                    }
                                }
                                'b_19a914: {
                                    'b_19a910: {
                                        c.cycles += 17;
                                        call_ptr(c, 0x0014dc80, 0x0019a8c8)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x9;
                                        call_ptr(c, 0x0014dd58, 0x0019a8d4)?;
                                        c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                                        call_ptr(c, 0x0014ad28, 0x0019a8e4)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x288));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        if t {
                                            break 'b_19a910;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            c.g[A2][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            break 'b_19a914;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A2][0] = 0u64;
                                    break 'b_19a914;
                                }
                                c.cycles += 12;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
                                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x98));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1818));
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa78));
                                c.g[A1][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                if c.g[A0][0] != 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_19a970;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x2));
                                    c.g[V0][0] = c.g[V0][0] & 0x4;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x970));
                                    break 'b_19a954;
                                }
                            }
                            c.cycles += 7;
                            call_ptr(c, 0x00317670, 0x0019a95c)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x0019a968)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                            break 'b_19a988;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x988));
                        call_ptr(c, 0x00317670, 0x0019a978)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x0019a984)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                        break 'b_19a988;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0019a9a0)?;
                    break 'b_19a9a0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_19a9a4;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshEquipInfo(self)` (0x19aa08): equip mode panel
/// refresh: hide the polygon group, the small bolt icon and the cost widget,
/// show them for a selected non-category item, set the icon texture, and
/// update the polygon preview bar.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_refresh_equip_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_19ab5c: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_19ab5c;
            } else {
                'b_19aa50: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_19aa50;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019aa50)?;
                        break 'b_19aa50;
                    }
                }
                'b_19aa70: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_19aa70;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019aa70)?;
                        break 'b_19aa70;
                    }
                }
                'b_19aa90: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_19aa90;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019aa90)?;
                        break 'b_19aa90;
                    }
                }
                'b_19ab50: {
                    'b_19ab4c: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                        c.g[S1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x95));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S3][0] = 0u64;
                        if t {
                            break 'b_19ab4c;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                            c.g[V0][0] = c.g[V1][0] & 0x4;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[V1][0] & 0x20;
                            if t {
                                break 'b_19ab4c;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = c.g[S0][0];
                                    break 'b_19ab50;
                                } else {
                                    c.cycles += 15;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0019aae8)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                                    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x98));
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa78));
                                    if t {
                                        break 'b_19ab4c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A0][0] = c.g[S0][0];
                                            break 'b_19ab50;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xa60));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_19ab4c;
                                            } else {
                                                c.cycles += 11;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x0019ab34)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x0019a098,
                                                    f: ee::s03::f_0019a098,
                                                }
                                                .call(c, 0x0019ab40)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x78),
                                                    c.g[V0][0],
                                                );
                                                c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), 0u64);
                                                break 'b_19ab4c;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    break 'b_19ab50;
                }
                c.cycles += 3;
                c.g[A1][0] = c.g[S3][0];
                Callee {
                    addr: 0x0019bd48,
                    f: ee::s03::f_0019bd48,
                }
                .call(c, 0x0019ab58)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_19ab5c;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshBuyInfo(self)` (0x19ab78): buy mode panel refresh:
/// show the large preview texture for the selected item, and its price
/// (x10 when positive) in the cost text.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_refresh_buy_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_19ac90: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_19ac90;
            } else {
                'b_19abbc: {
                    c.cycles += 3;
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                    let t = (c.g[V1][0] as i64) < 0;
                    c.g[S1][0] = 0u64;
                    if t {
                        break 'b_19abbc;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x958));
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                        if t {
                            break 'b_19abbc;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
                            break 'b_19abbc;
                        }
                    }
                }
                'b_19ac18: {
                    'b_19ac14: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_19ac14;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0019abdc)?;
                            if c.g[S1][0] == 0u64 {
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                break 'b_19ac18;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa70));
                                let t = (c.g[V1][0] as i64) < 0;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_19ac14;
                                } else {
                                    c.cycles += 9;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0u64);
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0019ac14)?;
                                    break 'b_19ac14;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                    break 'b_19ac18;
                }
                'b_19ac8c: {
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_19ac8c;
                    } else {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019ac34)?;
                        if c.g[S1][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_19ac90;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                            c.g[V0][0] = c.g[V0][0] & 0x20;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_19ac90;
                            } else {
                                'b_19ac5c: {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xe));
                                    let t = (c.g[A0][0] as i64) <= 0;
                                    c.g[V0][0] = 0xa;
                                    if t {
                                        break 'b_19ac5c;
                                    } else {
                                        c.cycles += 1;
                                        c.mult(0, 4, 4, 2);
                                        break 'b_19ac5c;
                                    }
                                }
                                c.cycles += 12;
                                Callee {
                                    addr: 0x00198af0,
                                    f: ee::s03::f_00198af0,
                                }
                                .call(c, 0x0019ac64)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x003a0c50, 0x0019ac70)?;
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                c.g[A1][0] = 0x1;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0019ac8c)?;
                                break 'b_19ac8c;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_19ac90;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int cFEStateCharEquip_BuildItemList(self, category)` (0x19b180): fill the item
/// list (`+0x124`, count at `+0x958`) with the category's items that are listed in
/// this mode (buy: not yet bought and for the current slot; equip: owned),
/// then sort it. Returns the count.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_parens
)]
pub fn cfe_state_char_equip_build_item_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_19b414: {
            'b_19b410: {
                'b_19b334: {
                    c.cycles += 43;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8e0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x8c0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                    c.g[S1][0] = c.g[A1][0];
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8d0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x8a0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x890));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x860));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x880));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x870));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x850));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x840), c.g[RA][0]);
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x124));
                    call_ptr(c, 0x0014dc80, 0x0019b1bc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x9;
                    call_ptr(c, 0x0014dd58, 0x0019b1c8)?;
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014bdb8, 0x0019b1d0)?;
                    c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                    c.g[S7][0] = c.g[V0][0];
                    call_ptr(c, 0x0014ad28, 0x0019b1e4)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                    c.g[T0][0] = 0xffffffffffffffff;
                    c.g[T1][0] = 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[SP][0];
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x0014d7e8, 0x0019b204)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x958), 0u64);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x834;
                    call_ptr(c, 0x003e6448, 0x0019b21c)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc8));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S6][0] = c.g[S4][0];
                    if t {
                        break 'b_19b334;
                    } else {
                        c.cycles += 2;
                        let t = (c.g[S0][0] as i64) <= 0;
                        c.g[FP][0] = 0x1a0000;
                        if t {
                            break 'b_19b410;
                        } else {
                            c.cycles += 3;
                            c.g[S5][0] = c.g[S0][0];
                            c.g[S4][0] = c.g[SP][0];
                            'l_19b240: loop {
                                'b_19b324: {
                                    c.cycles += 4;
                                    c.g[S0][0] = c.lw((c.g[S4][0] as u32));
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                        break 'b_19b324;
                                    } else {
                                        'b_19b274: {
                                            c.cycles += 5;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[V0][0] = c.g[V0][0] & 0x4;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                                break 'b_19b274;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[A0][0] & 0x20;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[S5][0] =
                                                        sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                                    break 'b_19b324;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] =
                                                        c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    break 'b_19b274;
                                                }
                                            }
                                        }
                                        'b_19b2a0: {
                                            'b_19b29c: {
                                                c.cycles += 6;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x288));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[A3][0] =
                                                    c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    break 'b_19b29c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                                    c.g[S1][0] = sx((c.g[S3][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    break 'b_19b2a0;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[S1][0] = 0u64;
                                            break 'b_19b2a0;
                                        }
                                        'b_19b308: {
                                            'b_19b2d8: {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[A0][0] & 0x20;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
                                                if t {
                                                    break 'b_19b2d8;
                                                } else {
                                                    c.cycles += 7;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32
                                                        >> 16)
                                                        as u32);
                                                    c.g[A1][0] = c.g[S7][0];
                                                    c.g[A2][0] = c.g[S3][0];
                                                    Callee {
                                                        addr: 0x0019b098,
                                                        f: ee::s03::f_0019b098,
                                                    }
                                                    .call(c, 0x0019b2c0)?;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x958));
                                                        break 'b_19b308;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x34));
                                                        c.g[V0][0] = c.g[V0][0] & 0x20;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[S5][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            break 'b_19b324;
                                                        } else {
                                                            break 'b_19b2d8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
                                            c.g[V0][0] = c.g[V0][0] & 0x2;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S5][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                                break 'b_19b324;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0xcc));
                                                c.g[V0][0] = 0xffffffffffffffff;
                                                if c.g[V1][0] == c.g[V0][0] {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x958)
                                                        );
                                                    break 'b_19b308;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lb((c.g[S0][0] as u32).wrapping_add(0x2));
                                                    if c.g[V1][0] != c.g[V0][0] {
                                                        c.g[S5][0] =
                                                            sx((c.g[S5][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        break 'b_19b324;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x958));
                                                        break 'b_19b308;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x958), c.g[V1][0]);
                                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                        break 'b_19b324;
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S5][0] != 0u64;
                                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019b240);
                                    }
                                    continue 'l_19b240;
                                } else {
                                    c.cycles += 2;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                    break 'b_19b414;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = (c.g[S0][0] as i64) <= 0;
                c.g[FP][0] = 0x1a0000;
                if t {
                    break 'b_19b410;
                } else {
                    c.cycles += 3;
                    c.g[S4][0] = c.g[S0][0];
                    c.g[S1][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32));
                    'l_19b348: loop {
                        'b_19b404: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            if c.g[V0][0] == 0u64 {
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_19b404;
                            } else {
                                'b_19b378: {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[V0][0] = c.g[V0][0] & 0x4;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                        break 'b_19b378;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[A0][0] & 0x20;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                            break 'b_19b404;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lh((c.g[S0][0] as u32).wrapping_add(0x4));
                                            break 'b_19b378;
                                        }
                                    }
                                }
                                'b_19b3a4: {
                                    'b_19b3a0: {
                                        c.cycles += 6;
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x288));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[V0][0] = c.lh((c.g[V0][0] as u32));
                                        let t = (c.g[V0][0] as i64) < 0;
                                        c.g[A3][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_19b3a0;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x290));
                                            c.g[V1][0] = sx((c.g[S3][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            break 'b_19b3a4;
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = 0u64;
                                    break 'b_19b3a4;
                                }
                                'b_19b3e8: {
                                    'b_19b3c8: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.g[A0][0] & 0x20;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
                                            break 'b_19b3c8;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x2));
                                            c.g[V0][0] = c.g[V0][0] & 0x2;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                                break 'b_19b3e8;
                                            } else {
                                                c.cycles += 2;
                                                c.g[S1][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                                break 'b_19b404;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 16) as u32);
                                    c.g[A1][0] = c.g[S7][0];
                                    c.g[A2][0] = c.g[S3][0];
                                    Callee {
                                        addr: 0x0019afd0,
                                        f: ee::s03::f_0019afd0,
                                    }
                                    .call(c, 0x0019b3dc)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                        break 'b_19b404;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                        break 'b_19b3e8;
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.sw((c.g[V0][0] as u32), c.g[S0][0]);
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x958), c.g[V1][0]);
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_19b404;
                            }
                        }
                        c.cycles += 3;
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                        if c.g[S4][0] != 0u64 {
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32));
                            if event_due(c) {
                                deliver_event_here(c, 0x0019b348);
                            }
                            continue 'l_19b348;
                        } else {
                            break 'b_19b410;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
            break 'b_19b414;
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x7260));
        c.g[A2][0] = 0x4;
        call_ptr(c, 0x00418ef8, 0x0019b424)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x958));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8d0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x8c0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x850));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x840));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8e0));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all at DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC857_FX_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC857_FX_MAGIC: u32 = 0x4f43_3835;
    /// The equip state object (0x1000 bytes).
    pub(crate) const OC857_SELF: u32 = DATA + 0x21_0000;
    pub(crate) const OC857_WORLD: u32 = DATA + 0x22_0000;
    /// The front end object `world+0x7c` points at.
    pub(crate) const OC857_FE: u32 = DATA + 0x22_1000;
    /// Widgets: `OC857_WIDGET + 0x100 * n`, all sharing one vtable.
    pub(crate) const OC857_WIDGET: u32 = DATA + 0x23_0000;
    pub(crate) const OC857_VT: u32 = DATA + 0x24_0000;
    /// Item structs, 0x40 bytes each.
    pub(crate) const OC857_ITEMS: u32 = DATA + 0x25_0000;
    pub(crate) const OC857_DB: u32 = DATA + 0x26_0000;
    pub(crate) const OC857_DB_TABLE: u32 = DATA + 0x26_4000;
    pub(crate) const OC857_MENU: u32 = DATA + 0x27_0000;
    pub(crate) const OC857_GR: u32 = DATA + 0x28_0000;
    pub(crate) const OC857_GR_SCREEN: u32 = DATA + 0x28_4000;
    pub(crate) const OC857_FILE: u32 = DATA + 0x29_0000;
    /// Forced results of the fakes: one word per callee.
    pub(crate) const OC857_MAIL: u32 = DATA + 0x30_0000;

    fn oc857_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    fn oc857_mail(addr: u32) -> u32 {
        OC857_MAIL + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    fn oc857_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc857_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    fn oc857_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc857_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (none: the fake's pointer) unless it unwound.
    fn oc857_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc857_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc857_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC857_FX_CTRL) != OC857_FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc857_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc857_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc857_fakes!(
        (
            oc857_fake_19da10,
            OC857_PREV_19DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10
        ),
        (
            oc857_fake_19e238,
            OC857_PREV_19E238,
            ee::s03::OVERRIDE_0019E238,
            ee::s03::f_0019e238__generated,
            0x0019_e238
        ),
        (
            oc857_fake_19e2b0,
            OC857_PREV_19E2B0,
            ee::s03::OVERRIDE_0019E2B0,
            ee::s03::f_0019e2b0__generated,
            0x0019_e2b0
        ),
        (
            oc857_fake_19dc20,
            OC857_PREV_19DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20
        ),
        (
            oc857_fake_19a9b8,
            OC857_PREV_19A9B8,
            ee::s03::OVERRIDE_0019A9B8,
            ee::s03::f_0019a9b8__generated,
            0x0019_a9b8
        ),
        (
            oc857_fake_1a0548,
            OC857_PREV_1A0548,
            ee::s03::OVERRIDE_001A0548,
            ee::s03::f_001a0548__generated,
            0x001a_0548
        ),
        (
            oc857_fake_19fa78,
            OC857_PREV_19FA78,
            ee::s03::OVERRIDE_0019FA78,
            ee::s03::f_0019fa78__generated,
            0x0019_fa78
        ),
        (
            oc857_fake_19a098,
            OC857_PREV_19A098,
            ee::s03::OVERRIDE_0019A098,
            ee::s03::f_0019a098__generated,
            0x0019_a098
        ),
        (
            oc857_fake_19b098,
            OC857_PREV_19B098,
            ee::s03::OVERRIDE_0019B098,
            ee::s03::f_0019b098__generated,
            0x0019_b098
        ),
        (
            oc857_fake_19afd0,
            OC857_PREV_19AFD0,
            ee::s03::OVERRIDE_0019AFD0,
            ee::s03::f_0019afd0__generated,
            0x0019_afd0
        ),
        (
            oc857_fake_317670,
            OC857_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc857_fake_3a1000,
            OC857_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc857_fake_3a0c50,
            OC857_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc857_fake_14dc80,
            OC857_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc857_fake_14dd58,
            OC857_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc857_fake_14ad28,
            OC857_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc857_fake_14bdb8,
            OC857_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc857_fake_14d7e8,
            OC857_PREV_14D7E8,
            ee::s01::OVERRIDE_0014D7E8,
            ee::s01::f_0014d7e8__generated,
            0x0014_d7e8
        ),
        (
            oc857_fake_3e6448,
            OC857_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc857_fake_418ef8,
            OC857_PREV_418EF8,
            ee::s15::OVERRIDE_00418EF8,
            ee::s15::f_00418ef8__generated,
            0x0041_8ef8
        ),
    );

    /// Machine with the fakes on, the front end (`world+0x7c`), the widgets'
    /// shared vtable (slot 0x48 `setVisible`, 0x58 a second fake), the equip
    /// state at `OC857_SELF` in `a0`, and a screen manager at `gp-0x854` whose
    /// vtable slot 0x170 is a fake.
    fn oc857_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC857_FX_CTRL, OC857_FX_MAGIC);
        oc857_install_fakes();
        c.bus.write16(OC857_VT + 0x48, 0);
        oc857_w(c, OC857_VT + 0x4c, FAKE + 0x10);
        c.bus.write16(OC857_VT + 0x58, 0);
        oc857_w(c, OC857_VT + 0x5c, FAKE + 0x20);
        for n in 0..8 {
            oc857_w(c, OC857_WIDGET + 0x100 * n + 8, OC857_VT);
        }
        oc857_w(c, GAME_GP - 0x848, OC857_WORLD);
        oc857_w(c, OC857_WORLD + 0x7c, OC857_FE);
        oc857_w(c, GAME_GP - 0x854, OC857_GR);
        oc857_w(c, OC857_GR + 0x10d8, OC857_GR_SCREEN);
        c.bus.write16(OC857_GR_SCREEN + 0x170, 0);
        oc857_w(c, OC857_GR_SCREEN + 0x174, FAKE + 0x30);
        // The polygon limit (`gp-0x1818`).
        oc857_w(c, GAME_GP - 0x1818, 100);
        c.g[A0][0] = sx(OC857_SELF);
        r
    }

    fn oc857_widget(n: u32) -> u32 {
        OC857_WIDGET + 0x100 * n
    }

    /// A widget pointer field: the widget `n`, or null for some seeds.
    fn oc857_put_widget(c: &mut Cpu, off: u32, n: u32, null: bool) {
        oc857_w(c, OC857_SELF + off, if null { 0 } else { oc857_widget(n) });
    }

    /// The menu (`self+0x5c`, selected index `+0x95`, scroll offset `+0x98`)
    /// and `count` items (`self+0x958`, pointers at `+0x124`); each item
    /// gets `flags`, `+0x30` an icon name pointer (or none), cost `+0xe`.
    fn oc857_put_menu(c: &mut Cpu, r: &mut Rng, sel: u32, count: u32) {
        oc857_w(c, OC857_SELF + 0x5c, OC857_MENU);
        c.bus.write8(OC857_MENU + 0x95, sel as u8);
        let off = r.next_u32() % (sel + 1);
        c.bus.write8(OC857_MENU + 0x98, off as u8);
        oc857_w(c, OC857_SELF + 0x958, count);
        for i in 0..6 {
            let item = OC857_ITEMS + 0x40 * i;
            oc857_w(c, OC857_SELF + 0x124 + 4 * i, item);
            oc857_w(
                c,
                item + 0x34,
                [4, 0x24, 0x20, 0, 0x44, 0x2c][(r.next_u32() % 6) as usize],
            );
            oc857_w(
                c,
                item + 0x30,
                if r.next_u32() % 3 == 0 { 0 } else { OC857_FILE },
            );
            c.bus.write16(
                item + 0xe,
                [-5i16, 0, 3, 100][(r.next_u32() % 4) as usize] as u16,
            );
            c.bus.write16(item + 4, (r.next_u32() % 8) as u16);
        }
        for i in 0..8 {
            oc857_w(c, OC857_SELF + 0xa78 + 4 * i, r.next_u32() % 60);
        }
    }

    /// The profile DB (`14ad28` result): the item id table and its entries.
    fn oc857_put_db(c: &mut Cpu, r: &mut Rng) {
        oc857_force(c, 0x0014_ad28, OC857_DB);
        oc857_w(c, OC857_DB + 0x288, OC857_DB_TABLE);
        for id in 0..8u32 {
            let idx = [0xffffu16, 0, 1, 2][(r.next_u32() % 4) as usize];
            c.bus.write16(OC857_DB_TABLE + 2 * id, idx);
        }
        for e in 0..3u32 {
            c.bus
                .write16(OC857_DB + 0x290 + 4 * e + 2, r.next_u32() as u16);
        }
    }

    // ------------------------------------------------------- UpdatePreview

    /// `UpdatePreview`: delay counting, acquire, loaded check, data entry or
    /// none, and the non-buy-mode early exit.
    #[test]
    fn cfe_state_char_equip_update_preview_matches() {
        check(
            ee::s03::f_0019a638__generated,
            cfe_state_char_equip_update_preview,
            144,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_a638, 6);
                c.pc = 0x0019_a638;
                let s = OC857_SELF;
                let mode = if seed % 9 == 8 {
                    [0u32, 2, 5][(seed / 9 % 3) as usize]
                } else {
                    1
                };
                oc857_w(c, s + 0xc8, mode);
                oc857_w(c, s + 0x95c, [0, u32::MAX, 1, 2][(seed % 4) as usize]);
                let idx = if (seed / 4) % 2 == 0 {
                    u32::MAX
                } else {
                    (seed / 8) % 5
                };
                oc857_w(c, s + 0xa6c, idx);
                oc857_w(c, s + 0xa70, r.next_u32());
                oc857_w(c, s + 0xbc, r.next_u32() % 10);
                if (seed / 8) % 2 == 0 {
                    oc857_force_zero(c, 0x0019_e238);
                } else {
                    oc857_force(c, 0x0019_e238, 1);
                }
                if (seed / 16) % 2 == 0 {
                    oc857_force_zero(c, 0x0019_e2b0);
                } else {
                    oc857_force(c, 0x0019_e2b0, OC857_FILE);
                    oc857_w(c, OC857_FILE + 0x14, r.next_u32() % 0x100);
                }
                oc857_force(c, 0x0019_da10, r.next_u32() % 20);
            },
        );
    }

    // -------------------------------------------- ReleasePreviewBoltTextures

    #[test]
    fn cfe_state_char_equip_release_preview_bolt_textures_matches() {
        check(
            ee::s03::f_0019a758__generated,
            cfe_state_char_equip_release_preview_bolt_textures,
            24,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_a758, 3);
                c.pc = 0x0019_a758;
                if seed % 3 == 0 {
                    oc857_w(c, OC857_WORLD + 0x7c, 0);
                }
                c.bus.write8(OC857_SELF + 0x44, r.next_u32() as u8);
            },
        );
    }

    // ------------------------------------------------------- UpdateHelpText

    /// `UpdateHelpText`: no widget, no item (both modes), category row,
    /// buy mode money check, equip mode polygon budget check.
    #[test]
    fn cfe_state_char_equip_update_help_text_matches() {
        check(
            ee::s03::f_0019a798__generated,
            cfe_state_char_equip_update_help_text,
            240,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_a798, 7);
                c.pc = 0x0019_a798;
                let s = OC857_SELF;
                oc857_put_widget(c, 0x4c, 0, seed % 13 == 0);
                let sel = [0u32, 1, 2, 3, 4, 5][(r.next_u32() % 6) as usize];
                oc857_put_menu(c, &mut r, sel, 4);
                oc857_w(c, s + 0xc8, [1u32, 0, 2, 1][(seed / 3 % 4) as usize]);
                oc857_w(c, s + 0xae4, r.next_u32() & 1);
                oc857_w(c, s + 0xc0, r.next_u32() % 2000);
                oc857_w(c, s + 0xc4, r.next_u32() % 90);
                oc857_w(c, s + 0xbc, r.next_u32() % 10);
                c.bus.write8(s + 0x44, r.next_u32() as u8);
                oc857_put_db(c, &mut r);
            },
        );
    }

    // ----------------------------------------------------- RefreshEquipInfo

    /// `RefreshEquipInfo`: widgets present or not, an item that is a
    /// category row or not, icon textures loaded or not.
    #[test]
    fn cfe_state_char_equip_refresh_equip_info_matches() {
        check(
            ee::s03::f_0019aa08__generated,
            cfe_state_char_equip_refresh_equip_info,
            240,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_aa08, 9);
                c.pc = 0x0019_aa08;
                let s = OC857_SELF;
                let nulls = r.next_u32();
                oc857_put_widget(c, 0x6c, 0, nulls % 7 == 0);
                oc857_put_widget(c, 0x84, 1, nulls / 7 % 7 == 0);
                oc857_put_widget(c, 0x7c, 2, nulls / 49 % 7 == 0);
                oc857_w(c, s + 0x74, 0);
                let sel = [0u32, 1, 2, 3, 4, 5][(r.next_u32() % 6) as usize];
                oc857_put_menu(c, &mut r, sel, 4);
                if seed % 17 == 0 {
                    oc857_w(c, s + 0x5c, 0);
                }
                oc857_w(c, s + 0xa60, r.next_u32() & 1);
            },
        );
    }

    // ------------------------------------------------------- RefreshBuyInfo

    /// `RefreshBuyInfo`: large preview widget and texture, the cost text
    /// (real `getDollarString`, with the game image for libc).
    #[test]
    fn cfe_state_char_equip_refresh_buy_info_matches() {
        check(
            ee::s03::f_0019ab78__generated,
            cfe_state_char_equip_refresh_buy_info,
            120,
            &|c, seed| {
                crate::difftest::elf_image(c);
                let mut r = oc857_fixture(c, seed, 0x19_ab78, 8);
                c.pc = 0x0019_ab78;
                let s = OC857_SELF;
                let nulls = r.next_u32();
                oc857_put_widget(c, 0x88, 3, nulls % 5 == 0);
                oc857_put_widget(c, 0x68, 4, nulls / 5 % 5 == 0);
                let sel = [0u32, 1, 2, 3, 4, 5][(r.next_u32() % 6) as usize];
                oc857_put_menu(c, &mut r, sel, 4);
                if seed % 19 == 0 {
                    oc857_w(c, s + 0x5c, 0);
                }
                oc857_w(c, s + 0xa70, [u32::MAX, 3, 7][(r.next_u32() % 3) as usize]);
            },
        );
    }

    // ------------------------------------------------------- BuildItemList

    /// `BuildItemList`: both modes' filters (name, flags, slot filter,
    /// bought flag, category check) over a stack list the (faked)
    /// `GetItems` leaves, then the sort call.
    #[test]
    fn cfe_state_char_equip_build_item_list_matches() {
        check(
            ee::s03::f_0019b180__generated,
            cfe_state_char_equip_build_item_list,
            240,
            &|c, seed| {
                let mut r = oc857_fixture(c, seed, 0x19_b180, 14);
                c.pc = 0x0019_b180;
                let s = OC857_SELF;
                c.g[A1][0] = r.next_u32() as u64 % 20;
                oc857_w(c, s + 0xc8, [1u32, 0, 2][(seed % 3) as usize]);
                oc857_w(c, s + 0xcc, [u32::MAX, 0, 1][(seed / 3 % 3) as usize]);
                oc857_w(c, s + 0xbc, r.next_u32() % 10);
                c.bus.write8(s + 0x44, r.next_u32() as u8);
                oc857_put_db(c, &mut r);
                let n = (seed / 9 % 8).min(6);
                oc857_force(c, 0x0014_d7e8, if seed % 29 == 0 { 0 } else { n });
                // The item list the callee leaves at the new sp.
                let list = crate::difftest::SP - 0x8e0;
                for i in 0..6u32 {
                    let item = OC857_ITEMS + 0x40 * i;
                    oc857_w(c, list + 4 * i, item);
                    oc857_w(c, item + 0x14, if r.next_u32() % 5 == 0 { 0 } else { 1 });
                    oc857_w(
                        c,
                        item + 0x34,
                        [0u32, 4, 0x20, 0x24, 0x2c][(r.next_u32() % 5) as usize],
                    );
                    c.bus.write16(item + 4, (r.next_u32() % 8) as u16);
                    c.bus.write8(item + 2, (r.next_u32() % 3) as u8);
                }
                if (seed / 72) % 2 == 0 {
                    oc857_force_zero(c, 0x0019_b098);
                    oc857_force_zero(c, 0x0019_afd0);
                } else {
                    oc857_force(c, 0x0019_b098, 1);
                    oc857_force(c, 0x0019_afd0, 1);
                }
            },
        );
    }
}
