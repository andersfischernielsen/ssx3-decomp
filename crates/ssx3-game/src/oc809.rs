//! Staging module oc809 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc809`.

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
    (
        0x002c0408,
        world_painter_query_ctor,
        "WorldPainterQuery_ctor",
    ),
    (0x002da390, board_spark_fx_ctor, "cBoardSparkFX_ctor"),
    (0x002db6d8, crowd_render_2d_ctor, "cCrowdRender2D_ctor"),
    (0x002de4a8, dynamic_spray_fx_ctor, "cDynamicSprayFX_ctor"),
];

/// `void* WorldPainterQuery_ctor(void* self, u32 kind)` (0x2c0408): stores the
/// vtable (0x483e00), the float at gp-0x4288 (twice) and the query kind (low
/// byte at +0x10, the rest of `kind` in the word), then, for kinds 0..14 (the
/// byte-sized selector `kind >> 8`), allocates and constructs the matching
/// `tWPIGD_*` data object (jump table at 0x483db0) into `*self`; finally links
/// itself into the list at gp+0x790 (`self->next = head; head = self`). Returns
/// `self`.
///
/// Converted from the generated body (every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn world_painter_query_ctor(c: &mut Cpu) {
    if c.pc != 0x002c0408 {
        return ee::s08::f_002c0408__generated(c);
    }
    guest(c, |c| {
        'b_2c069c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[V0][0] = 0x480000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e00));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sb((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4288));
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[V0][0] = (c.g[A0][0] < 0xeu64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
            if t {
                break 'b_2c069c;
            } else {
                'b_2c0678: {
                    'b_2c0650: {
                        'b_2c0628: {
                            'b_2c0600: {
                                'b_2c05d8: {
                                    'b_2c05b0: {
                                        'b_2c0588: {
                                            'b_2c0560: {
                                                'b_2c0538: {
                                                    'b_2c0510: {
                                                        'b_2c04e8: {
                                                            'b_2c04c0: {
                                                                'b_2c0498: {
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = 0x480000;
                                                                    c.g[V1][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x3db0));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    c.g[A0][0] =
                                                                        c.lw(c.g[V1][0] as u32);
                                                                    let t = c.g[A0][0] as u32;
                                                                    match t {
                                                                        0x002c0470 => {
                                                                            c.cycles += 10;
                                                                            c.g[A1][0] = 0x480000;
                                                                            c.g[A0][0] = 0x18;
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x3cb8,
                                                                                    ));
                                                                            c.g[A2][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x002c0488);
                                                                            c.g[A3][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00317d70,
                                                                                0x002c0488,
                                                                            )?;
                                                                            c.g[RA][0] =
                                                                                sx(0x002c0490);
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            Callee { addr: 0x002bc6f8, f: ee::s08::tWPIGD_MusicTrigger_ctor }.call(c, 0x002c0490)?;
                                                                            let t = true;
                                                                            c.sw(
                                                                                c.g[S0][0] as u32,
                                                                                c.g[V0][0],
                                                                            );
                                                                            if t {
                                                                                break 'b_2c069c;
                                                                            } else {
                                                                                break 'b_2c0498;
                                                                            }
                                                                        }
                                                                        0x002c0498 => {
                                                                            break 'b_2c0498;
                                                                        }
                                                                        0x002c04c0 => {
                                                                            break 'b_2c04c0;
                                                                        }
                                                                        0x002c04e8 => {
                                                                            break 'b_2c04e8;
                                                                        }
                                                                        0x002c0510 => {
                                                                            break 'b_2c0510;
                                                                        }
                                                                        0x002c0538 => {
                                                                            break 'b_2c0538;
                                                                        }
                                                                        0x002c0560 => {
                                                                            break 'b_2c0560;
                                                                        }
                                                                        0x002c0588 => {
                                                                            break 'b_2c0588;
                                                                        }
                                                                        0x002c05b0 => {
                                                                            break 'b_2c05b0;
                                                                        }
                                                                        0x002c05d8 => {
                                                                            break 'b_2c05d8;
                                                                        }
                                                                        0x002c0600 => {
                                                                            break 'b_2c0600;
                                                                        }
                                                                        0x002c0628 => {
                                                                            break 'b_2c0628;
                                                                        }
                                                                        0x002c0650 => {
                                                                            break 'b_2c0650;
                                                                        }
                                                                        0x002c0678 => {
                                                                            break 'b_2c0678;
                                                                        }
                                                                        _ => {
                                                                            c.jump_out(
                                                                                t, 0x002c0408,
                                                                                0x002c06bc,
                                                                            );
                                                                            return Ok(());
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 10;
                                                                c.g[A1][0] = 0x480000;
                                                                c.g[A0][0] = 0x10;
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x3cd0));
                                                                c.g[A2][0] = 0u64;
                                                                c.g[RA][0] = sx(0x002c04b0);
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00317d70, 0x002c04b0,
                                                                )?;
                                                                c.g[RA][0] = sx(0x002c04b8);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                Callee {
                                                                    addr: 0x002bc728,
                                                                    f: ee::s08::tWPIGD_Mix_ctor,
                                                                }
                                                                .call(c, 0x002c04b8)?;
                                                                let t = true;
                                                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                                if t {
                                                                    break 'b_2c069c;
                                                                } else {
                                                                    break 'b_2c04c0;
                                                                }
                                                            }
                                                            c.cycles += 10;
                                                            c.g[A1][0] = 0x480000;
                                                            c.g[A0][0] = 0x10;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x3ce0));
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x002c04d8);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x002c04d8)?;
                                                            c.g[RA][0] = sx(0x002c04e0);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            Callee {
                                                                addr: 0x002bc750,
                                                                f: ee::s08::tWPIGD_Ambience_ctor,
                                                            }
                                                            .call(c, 0x002c04e0)?;
                                                            let t = true;
                                                            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                            if t {
                                                                break 'b_2c069c;
                                                            } else {
                                                                break 'b_2c04e8;
                                                            }
                                                        }
                                                        c.cycles += 10;
                                                        c.g[A1][0] = 0x480000;
                                                        c.g[A0][0] = 0x10;
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x3cf0));
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x002c0500);
                                                        c.g[A3][0] = 0u64;
                                                        call_ptr(c, 0x00317d70, 0x002c0500)?;
                                                        c.g[RA][0] = sx(0x002c0508);
                                                        c.g[A0][0] = c.g[V0][0];
                                                        Callee {
                                                            addr: 0x002bc778,
                                                            f: ee::s08::tWPIGD_Speech_ctor,
                                                        }
                                                        .call(c, 0x002c0508)?;
                                                        let t = true;
                                                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                        if t {
                                                            break 'b_2c069c;
                                                        } else {
                                                            break 'b_2c0510;
                                                        }
                                                    }
                                                    c.cycles += 10;
                                                    c.g[A1][0] = 0x480000;
                                                    c.g[A0][0] = 0x10;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x3d00));
                                                    c.g[A2][0] = 0u64;
                                                    c.g[RA][0] = sx(0x002c0528);
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x00317d70, 0x002c0528)?;
                                                    c.g[RA][0] = sx(0x002c0530);
                                                    c.g[A0][0] = c.g[V0][0];
                                                    Callee {
                                                        addr: 0x002bc7a0,
                                                        f: ee::s08::tWPIGD_Camera_ctor,
                                                    }
                                                    .call(c, 0x002c0530)?;
                                                    let t = true;
                                                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                    if t {
                                                        break 'b_2c069c;
                                                    } else {
                                                        break 'b_2c0538;
                                                    }
                                                }
                                                c.cycles += 10;
                                                c.g[A1][0] = 0x480000;
                                                c.g[A0][0] = 0x38;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x3d10));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x002c0550);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x002c0550)?;
                                                c.g[RA][0] = sx(0x002c0558);
                                                c.g[A0][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x002bc7c8,
                                                    f: ee::s08::tWPIGD_Fog_ctor,
                                                }
                                                .call(c, 0x002c0558)?;
                                                let t = true;
                                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                                if t {
                                                    break 'b_2c069c;
                                                } else {
                                                    break 'b_2c0560;
                                                }
                                            }
                                            c.cycles += 10;
                                            c.g[A1][0] = 0x480000;
                                            c.g[A0][0] = 0x40;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x3d20));
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x002c0578);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x002c0578)?;
                                            c.g[RA][0] = sx(0x002c0580);
                                            c.g[A0][0] = c.g[V0][0];
                                            Callee {
                                                addr: 0x002bc830,
                                                f: ee::s08::tWPIGD_LightGlow_ctor,
                                            }
                                            .call(c, 0x002c0580)?;
                                            let t = true;
                                            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                            if t {
                                                break 'b_2c069c;
                                            } else {
                                                break 'b_2c0588;
                                            }
                                        }
                                        c.cycles += 10;
                                        c.g[A1][0] = 0x480000;
                                        c.g[A0][0] = 0x38;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d38));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x002c05a0);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x002c05a0)?;
                                        c.g[RA][0] = sx(0x002c05a8);
                                        c.g[A0][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x002bc890,
                                            f: ee::s08::tWPIGD_ScreenTint_ctor,
                                        }
                                        .call(c, 0x002c05a8)?;
                                        let t = true;
                                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                        if t {
                                            break 'b_2c069c;
                                        } else {
                                            break 'b_2c05b0;
                                        }
                                    }
                                    c.cycles += 10;
                                    c.g[A1][0] = 0x480000;
                                    c.g[A0][0] = 0x10;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d50));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x002c05c8);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x002c05c8)?;
                                    c.g[RA][0] = sx(0x002c05d0);
                                    c.g[A0][0] = c.g[V0][0];
                                    Callee {
                                        addr: 0x002bc8e8,
                                        f: ee::s08::tWPIGD_SkyBox_ctor,
                                    }
                                    .call(c, 0x002c05d0)?;
                                    let t = true;
                                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                    if t {
                                        break 'b_2c069c;
                                    } else {
                                        break 'b_2c05d8;
                                    }
                                }
                                c.cycles += 10;
                                c.g[A1][0] = 0x480000;
                                c.g[A0][0] = 0x50;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d60));
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x002c05f0);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x002c05f0)?;
                                c.g[RA][0] = sx(0x002c05f8);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x002bc910,
                                    f: ee::s08::tWPIGD_Sun_ctor,
                                }
                                .call(c, 0x002c05f8)?;
                                let t = true;
                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                if t {
                                    break 'b_2c069c;
                                } else {
                                    break 'b_2c0600;
                                }
                            }
                            c.cycles += 10;
                            c.g[A1][0] = 0x480000;
                            c.g[A0][0] = 0x10;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d70));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x002c0618);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x002c0618)?;
                            c.g[RA][0] = sx(0x002c0620);
                            c.g[A0][0] = c.g[V0][0];
                            Callee {
                                addr: 0x002bc980,
                                f: ee::s08::tWPIGD_Surface_ctor,
                            }
                            .call(c, 0x002c0620)?;
                            let t = true;
                            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_2c069c;
                            } else {
                                break 'b_2c0628;
                            }
                        }
                        c.cycles += 10;
                        c.g[A1][0] = 0x480000;
                        c.g[A0][0] = 0x58;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d80));
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x002c0640);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x002c0640)?;
                        c.g[RA][0] = sx(0x002c0648);
                        c.g[A0][0] = c.g[V0][0];
                        Callee {
                            addr: 0x002bc9b0,
                            f: ee::s08::tWPIGD_Lighting_ctor,
                        }
                        .call(c, 0x002c0648)?;
                        let t = true;
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        if t {
                            break 'b_2c069c;
                        } else {
                            break 'b_2c0650;
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0xa0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3d90));
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x002c0668);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00317d70, 0x002c0668)?;
                    c.g[RA][0] = sx(0x002c0670);
                    c.g[A0][0] = c.g[V0][0];
                    Callee {
                        addr: 0x002bcaf8,
                        f: ee::s08::tWPIGD_Weather_ctor,
                    }
                    .call(c, 0x002c0670)?;
                    let t = true;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    if t {
                        break 'b_2c069c;
                    } else {
                        break 'b_2c0678;
                    }
                }
                c.cycles += 9;
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = 0x18;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3da0));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x002c0690);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x002c0690)?;
                c.g[RA][0] = sx(0x002c0698);
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x002bcbd0,
                    f: ee::s08::tWPIGD_Danger_ctor,
                }
                .call(c, 0x002c0698)?;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                break 'b_2c069c;
            }
        }
        c.cycles += 8;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x790));
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.sw((c.g[GP][0] as u32).wrapping_add(0x790), c.g[S0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cBoardSparkFX* cBoardSparkFX_ctor(cBoardSparkFX* self, Rider* rider)`
/// (0x2da390): constructs the embedded rider sub-object when `rider` is set,
/// copies two spark tables, allocates the emitter, particle and parameter
/// blocks, fills the 0xd8-byte emitter parameter table with constants, then
/// calls the emitter's virtual setup and `cBoardSparkFX_Reset`. Returns `self`.
///
/// Converted from the generated body (every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn board_spark_fx_ctor(c: &mut Cpu) {
    if c.pc != 0x002da390 {
        return ee::s09::f_002da390__generated(c);
    }
    guest(c, |c| {
        'b_2da3d8: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1b0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
            let t = c.g[S1][0] == 0u64;
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            if t {
                break 'b_2da3d8;
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002da3d8);
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x0011b698, 0x002da3d8)?;
                break 'b_2da3d8;
            }
        }
        'b_2da4f8: {
            c.cycles += 23;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[V0][0] = 0x490000;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7350));
            c.g[A2][0] = c.g[V0][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A1][0]);
            c.g[V0][0] = 0x460000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
            c.g[V1][0] = 0x490000;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[T3][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7308));
            c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7c));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
            c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
            c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            c.g[S5][0] = 0x480000;
            c.g[S6][0] = 0x480000;
            c.g[S7][0] = 0x480000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
            let t = c.g[S1][0] != 0u64;
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
            if t {
                break 'b_2da4f8;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                'l_2da440: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T5][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T5][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002da440);
                        }
                        continue 'l_2da440;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7350));
                        c.g[A2][0] = c.ld(c.g[A1][0] as u32);
                        c.sd(c.g[V1][0] as u32, c.g[A2][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                        c.g[A2][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7308));
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A1][0] = c.g[T0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb0));
                        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        'l_2da4a4: loop {
                            c.cycles += 11;
                            c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                            c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                            c.g[T5][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T5][0]);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                            let t = c.g[A2][0] != c.g[T1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002da4a4);
                                }
                                continue 'l_2da4a4;
                            } else {
                                c.cycles += 10;
                                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7308));
                                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                                c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T0][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                                break 'b_2da4f8;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0x2;
        c.g[V1][0] = 0xffffffffffffffff;
        'l_2da500: loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0u64;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002da500);
                }
                continue 'l_2da500;
            } else {
                c.cycles += 5;
                c.g[V0][0] = 0x1;
                c.g[V1][0] = c.g[T2][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                c.g[A1][0] = 0xffffffffffffffff;
                'l_2da530: loop {
                    c.cycles += 7;
                    c.sh(c.g[V1][0] as u32, c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002da530);
                        }
                        continue 'l_2da530;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[T4][0];
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x002da55c);
                        c.g[A2][0] = 0x10;
                        call_ptr(c, 0x00416210, 0x002da55c)?;
                        c.g[A1][0] = 0xffffffffffffffff;
                        c.g[V1][0] = c.g[S3][0];
                        c.g[A0][0] = 0u64;
                        'l_2da568: loop {
                            c.cycles += 7;
                            c.sh(c.g[V1][0] as u32, c.g[A1][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = (c.g[A0][0] < 0x2u64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002da568);
                                }
                                continue 'l_2da568;
                            } else {
                                c.cycles += 217;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = 0u64;
                                c.mtc1(0, 20);
                                c.g[RA][0] = sx(0x002da598);
                                c.g[A2][0] = 0x10;
                                call_ptr(c, 0x00416210, 0x002da598)?;
                                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x7588));
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.g[RA][0] = sx(0x002da5ac);
                                c.g[A0][0] = 0x210;
                                call_ptr(c, 0x00317d70, 0x002da5ac)?;
                                c.g[RA][0] = sx(0x002da5b4);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x003714b8, 0x002da5b4)?;
                                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x7598));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                c.g[A0][0] = 0xd8;
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x002da5cc);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x002da5cc)?;
                                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x75a8));
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                                c.g[RA][0] = sx(0x002da5e4);
                                c.g[A0][0] = 0x150;
                                call_ptr(c, 0x00317d70, 0x002da5e4)?;
                                c.g[RA][0] = sx(0x002da5ec);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x002f7a68,
                                    f: ee::s09::BoardSparkEmitter_ctor,
                                }
                                .call(c, 0x002da5ec)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x002da5f8);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                                call_ptr(c, 0x0036ccb8, 0x002da5f8)?;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3cb4));
                                c.g[A3][0] = c.g[S2][0];
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                c.g[A2][0] = 0x8;
                                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3cb0));
                                c.g[A1][0] = 0x1e;
                                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x3cac));
                                c.g[AT][0] = 0x3fc00000;
                                c.mtc1(1, 14);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x108));
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x104));
                                c.g[RA][0] = sx(0x002da630);
                                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x10c));
                                call_ptr(c, 0x0036cbf8, 0x002da630)?;
                                c.g[AT][0] = 0x40f00000;
                                c.mtc1(1, 12);
                                c.mov_s(13, 20);
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                c.g[RA][0] = sx(0x002da648);
                                c.mov_s(14, 12);
                                call_ptr(c, 0x0036ce00, 0x002da648)?;
                                c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x3ca8));
                                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3ca4));
                                c.g[RA][0] = sx(0x002da658);
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                call_ptr(c, 0x0036ce28, 0x002da658)?;
                                c.g[RA][0] = sx(0x002da660);
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                call_ptr(c, 0x0036d008, 0x002da660)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                                c.g[RA][0] = sx(0x002da66c);
                                c.mov_s(12, 20);
                                call_ptr(c, 0x0036d3d8, 0x002da66c)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = 0x1;
                                c.g[AT][0] = 0xffffffffbf800000;
                                c.mtc1(1, 0);
                                c.g[A1][0] = 0x5;
                                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                c.g[A2][0] = 0x8;
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 1);
                                c.g[A3][0] = 0xe;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.g[AT][0] = 0x40c00000;
                                c.mtc1(1, 3);
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                                c.g[AT][0] = 0x40800000;
                                c.mtc1(1, 2);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.g[AT][0] = 0x44160000;
                                c.mtc1(1, 5);
                                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[AT][0] = 0x43480000;
                                c.mtc1(1, 4);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.g[AT][0] = 0x42c80000;
                                c.mtc1(1, 0);
                                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                                c.g[AT][0] = 0xffffffffc4480000;
                                c.mtc1(1, 6);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3ca0));
                                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x10));
                                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3c9c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x3c98));
                                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x14));
                                c.g[AT][0] = 0x41d80000;
                                c.mtc1(1, 10);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x20));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x24));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x28));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x2c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x30));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x34));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x38));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x3c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x44));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x4c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x50));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x54));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x5c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x60));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x64));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x68));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x6c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x70));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x74));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x78));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x7c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x80));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x88));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x8c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x90));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0x94));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x98));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x9c));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xa0));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0xa4));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xa8));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0xac));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xb0));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0xb4));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xb8));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V1][0] as u32).wrapping_add(0xbc));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xc0));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.sw((c.g[V1][0] as u32).wrapping_add(0xc8), c.g[A0][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xcc));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.sw((c.g[V1][0] as u32).wrapping_add(0xc4), c.g[A1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[A2][0]);
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.swc1(10, (c.g[V1][0] as u32).wrapping_add(0xd4));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A3][0]);
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f8));
                                c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                let t = c.g[A2][0] as u32;
                                c.g[RA][0] = sx(0x002da8a8);
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                call_ptr(c, t, 0x002da8a8)?;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), 0u64);
                                c.g[RA][0] = sx(0x002da8b4);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x002daa78,
                                    f: ee::s09::cBoardSparkFX_Reset,
                                }
                                .call(c, 0x002da8b4)?;
                                c.g[V0][0] = c.g[S0][0];
                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x180));
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x190));
                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x170));
                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x160));
                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x140));
                                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x130));
                                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x120));
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
                                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                                let t = c.g[RA][0] as u32;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1b0));
                                c.pc = t;
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
    });
}

/// `cCrowdRender2D* cCrowdRender2D_ctor(cCrowdRender2D* this)` (0x2db6d8):
/// runs `cCrowdRender2D_LoadTextures(this)` and returns `this`.
pub fn crowd_render_2d_ctor(c: &mut Cpu) {
    if c.pc != 0x002d_b6d8 {
        return ee::s09::f_002db6d8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002d_b748,
            f: ee::s09::f_002db748,
        }
        .call(c, 0x002d_b6ec)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cDynamicSprayFX* cDynamicSprayFX_ctor(cDynamicSprayFX* self, Rider* rider)`
/// (0x2de4a8): constructs the embedded rider sub-object when `rider` is set,
/// copies the spray tables, allocates and constructs the emitters, fills the
/// emitter parameter blocks with constants, then calls
/// `cDynamicSprayFX_ConfigureForRider`, `_SetupEmitters` and `_Reset`.
/// Returns `self`.
///
/// Converted from the generated body (every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn dynamic_spray_fx_ctor(c: &mut Cpu) {
    if c.pc != 0x002de4a8 {
        return ee::s09::f_002de4a8__generated(c);
    }
    guest(c, |c| {
        'b_2de4ec: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.g[S0][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.g[S3][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
            let t = c.g[S0][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
            if t {
                break 'b_2de4ec;
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x130));
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002de4ec);
                c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                call_ptr(c, 0x0011b698, 0x002de4ec)?;
                break 'b_2de4ec;
            }
        }
        'b_2de5fc: {
            c.cycles += 19;
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
            c.g[V0][0] = 0x490000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ca0));
            c.g[A3][0] = c.g[V0][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
            c.g[V0][0] = 0x460000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
            c.g[V1][0] = 0x490000;
            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
            c.g[T3][0] = c.g[V1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7c58));
            c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x28));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            c.g[S5][0] = 0x480000;
            c.g[S7][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
            let t = c.g[S0][0] != 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
            if t {
                break 'b_2de5fc;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                'l_2de544: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                    c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002de544);
                        }
                        continue 'l_2de544;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ca0));
                        c.g[A3][0] = c.ld(c.g[A2][0] as u32);
                        c.sd(c.g[V1][0] as u32, c.g[A3][0]);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                        c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7c58));
                        c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[T2][0] = sx((c.g[A3][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                        c.g[A2][0] = c.g[T1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x130));
                        c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        'l_2de5a8: loop {
                            c.cycles += 11;
                            c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                            c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                            c.g[T4][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                            c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                            c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                            let t = c.g[A3][0] != c.g[T2][0];
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002de5a8);
                                }
                                continue 'l_2de5a8;
                            } else {
                                c.cycles += 10;
                                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7c58));
                                c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V0][0] = c.ld(c.g[A3][0] as u32);
                                c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.sd(c.g[A2][0] as u32, c.g[V0][0]);
                                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
                                c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                                break 'b_2de5fc;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 25;
        c.g[AT][0] = 0x42480000;
        c.mtc1(1, 0);
        c.g[V0][0] = 0x1;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7998));
        c.sw((c.g[S3][0] as u32).wrapping_add(0x14), 0u64);
        c.g[A0][0] = 0x14b0;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x18), 0u64);
        c.g[A2][0] = 0u64;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x20), 0u64);
        c.g[A3][0] = 0u64;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x24), 0u64);
        c.g[S1][0] = 0x9;
        c.sw((c.g[S3][0] as u32).wrapping_add(0xe0), 0u64);
        c.g[S4][0] = 0xffffffffffffffff;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x110), 0u64);
        c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
        c.sw((c.g[S3][0] as u32).wrapping_add(0x60), c.g[V0][0]);
        c.g[RA][0] = sx(0x002de64c);
        c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x00317e30, 0x002de64c)?;
        c.g[V1][0] = 0xa;
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[S0][0] = c.g[S2][0];
        'l_2de660: loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002de66c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            call_ptr(c, 0x003714b8, 0x002de66c)?;
            let t = c.g[S1][0] != c.g[S4][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x210));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002de660);
                }
                continue 'l_2de660;
            } else {
                c.cycles += 725;
                c.sw(c.g[S6][0] as u32, c.g[S2][0]);
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x79b0));
                c.g[A0][0] = 0x910;
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x002de68c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317e30, 0x002de68c)?;
                c.g[AT][0] = 0x42480000;
                c.mtc1(1, 0);
                c.g[T4][0] = 0x2;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 9);
                c.g[V1][0] = 0x1;
                c.g[AT][0] = 0x41200000;
                c.mtc1(1, 13);
                c.g[T3][0] = 0x5;
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 12);
                c.g[T1][0] = 0xe;
                c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x3bd8));
                c.g[T0][0] = 0x8;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x74));
                c.g[T2][0] = 0x6;
                c.g[AT][0] = 0x40200000;
                c.mtc1(1, 4);
                c.g[T5][0] = 0x3;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                c.g[T6][0] = 0x19;
                c.g[AT][0] = 0xffffffffc3160000;
                c.mtc1(1, 5);
                c.g[T7][0] = 0x1;
                c.sw(c.g[V0][0] as u32, c.g[T4][0]);
                c.g[S0][0] = 0x5;
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x79c8));
                c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = 0u64;
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A3][0] = 0u64;
                c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[A0][0] = 0x78;
                c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x54));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x24), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x28), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x44), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x50), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x60), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x68), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x70), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0u64);
                c.g[AT][0] = 0x41000000;
                c.mtc1(1, 2);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 1);
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x80));
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 10);
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x3bd4));
                c.g[AT][0] = 0x420c0000;
                c.mtc1(1, 0);
                c.g[AT][0] = 0x40c00000;
                c.mtc1(1, 3);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xf8));
                c.g[AT][0] = 0x43480000;
                c.mtc1(1, 7);
                c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x84));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x88));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x90));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x98));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x9c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa0));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[T3][0]);
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xd4));
                c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0xd8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), c.g[T3][0]);
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xf0));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xf4));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xfc));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xcc));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x100));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x94), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xa8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xac), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xb0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xb4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xb8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xec), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x104), 0u64);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3bd0));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x108), 0u64);
                c.g[AT][0] = 0x41f00000;
                c.mtc1(1, 14);
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x13c));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x14c));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x15c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x16c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x170));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x174));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x178));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x17c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x180));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x184));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x188));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x168));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x110), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x114), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x118), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x11c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x120), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x124), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x128), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x12c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x130), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x134), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x138), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x140), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x144), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x148), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x150), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x154), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x158), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x160), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x164), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18c), 0u64);
                c.g[AT][0] = 0x447a0000;
                c.mtc1(1, 3);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3bcc));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x3bc8));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3bc4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x190), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1ac), c.g[T1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1b8), c.g[T0][0]);
                c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x1bc));
                c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x1c0));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1d0), c.g[T2][0]);
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x1d8));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1dc));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1e4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1cc));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1b0), c.g[V1][0]);
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x1c4));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x1c8));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1e8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x194), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x198), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x19c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1a0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1a4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1a8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1b4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1ec), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1f0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1f4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1f8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x200), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x204), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x208), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x20c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x210), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x214), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x218), 0u64);
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x224));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x234));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x244));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x254));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x258));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x25c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x260));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x264));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x268));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x26c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x270));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x294), c.g[T1][0]);
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x250));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x29c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x298), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x21c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x220), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x228), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x22c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x230), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x238), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x23c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x240), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x248), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x24c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x274), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x278), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x280), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x284), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x288), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x28c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x290), 0u64);
                c.g[AT][0] = 0x40600000;
                c.mtc1(1, 0);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2a0), c.g[T0][0]);
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3bc0));
                c.g[AT][0] = 0x42340000;
                c.mtc1(1, 8);
                c.g[AT][0] = 0x42c80000;
                c.mtc1(1, 6);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2c8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2b8), c.g[T5][0]);
                c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x2a4));
                c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x2a8));
                c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x2b4));
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x2c0));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2c4));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2cc));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x2ac));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x2b0));
                c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x2d0));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x30c));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x31c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2bc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2d4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2d8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2dc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2e0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2e4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2e8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2ec), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2f0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2f4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2f8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x2fc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x300), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x304), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x308), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x310), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x314), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x318), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x320), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x324), 0u64);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3bbc));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x328), 0u64);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3bb8));
                c.g[AT][0] = 0x3e800000;
                c.mtc1(1, 11);
                c.g[AT][0] = 0x44610000;
                c.mtc1(1, 3);
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3bb4));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3bb0));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x338));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x37c), c.g[T1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x388), c.g[T0][0]);
                c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x38c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x33c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x34c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x390));
                c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x39c));
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x3a8));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x32c));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x358));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x394));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x398));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x3ac));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x340));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x344));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x348));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x350));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x354));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x380), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3a0), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x330), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x334), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x35c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x360), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x364), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x368), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x36c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x370), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x374), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x378), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x384), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3a4), 0u64);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3bac));
                c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x3b0));
                c.g[AT][0] = 0x42200000;
                c.mtc1(1, 4);
                c.g[AT][0] = 0x43960000;
                c.mtc1(1, 2);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3ba8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3b4));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x3f4));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x400));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x414));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x428));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x42c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x430));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x420));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x424));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3b8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3bc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3c0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3c4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3c8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3cc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3d0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3d8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3dc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3e0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3e4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3e8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3ec), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3f0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3f8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x3fc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x404), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x408), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x40c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x410), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x418), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x41c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x434), 0u64);
                c.g[AT][0] = 0x42000000;
                c.mtc1(1, 0);
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x438));
                c.g[AT][0] = 0x42700000;
                c.mtc1(1, 5);
                c.g[AT][0] = 0x40800000;
                c.mtc1(1, 6);
                c.g[AT][0] = 0x425c0000;
                c.mtc1(1, 2);
                c.g[AT][0] = 0x41c80000;
                c.mtc1(1, 3);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x46c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x464), c.g[T6][0]);
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x43c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x440));
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x474));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x478));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x488), c.g[T4][0]);
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x490));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x49c));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x498));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4a0));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x468), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x470), c.g[V1][0]);
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x494));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x4b8));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x4bc));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x444), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x448), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x44c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x450), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x454), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x458), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x45c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x460), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x47c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x480), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x484), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x48c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4a4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4a8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4ac), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4b0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4b4), 0u64);
                c.g[AT][0] = 0x43c80000;
                c.mtc1(1, 0);
                c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x4c0));
                c.g[AT][0] = 0x43fa0000;
                c.mtc1(1, 3);
                c.g[AT][0] = 0xffffffffc4e10000;
                c.mtc1(1, 2);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4d8));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x4fc));
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x50c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x510));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x514));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x518));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x520));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x524));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x528));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4ec));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x508));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4dc));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4cc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4d0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4d4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4e0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4e4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4e8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4f0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4f4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4f8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x500), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x504), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x51c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x52c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x530), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x534), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x538), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x53c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x540), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x544), 0u64);
                c.g[AT][0] = 0x432a0000;
                c.mtc1(1, 0);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x548), 0u64);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x554));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x54c), c.g[T2][0]);
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x55c));
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x578));
                c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x580));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x584));
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x588));
                c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x5a0));
                c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x5a4));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x5c0));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x57c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x550), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x558), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x570), c.g[V1][0]);
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x5c4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x560), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x564), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x568), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x56c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x574), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x590), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x594), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x598), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x59c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5a8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5ac), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5b0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5b4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5b8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5bc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5c8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5cc), 0u64);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ba4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5d0), 0u64);
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 2);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5f0));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x634), c.g[T2][0]);
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x5f8));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x5fc));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x600));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x608));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x60c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x610));
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x644));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x5f4));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x63c));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x5d4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x638), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x640), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5d8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5dc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5e0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5e4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5e8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x5ec), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x604), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x614), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x618), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x61c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x620), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x624), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x628), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x62c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x630), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x648), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x64c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x650), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x654), 0u64);
                c.g[AT][0] = 0x41700000;
                c.mtc1(1, 0);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x658), c.g[T4][0]);
                c.g[AT][0] = 0x3fc00000;
                c.mtc1(1, 5);
                c.g[AT][0] = 0xffffffffc1a00000;
                c.mtc1(1, 2);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x670));
                c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x668));
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x66c));
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x660));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x664));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x6ac));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x6bc));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x6cc));
                c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x6dc));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x6d8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x65c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x674), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x678), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x67c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x680), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x684), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x688), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x68c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x690), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x694), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x698), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x69c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6a0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6a4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6a8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6b0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6b4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6b8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6c4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6c8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6d4), 0u64);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ba0));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6e0));
                c.g[AT][0] = 0x42820000;
                c.mtc1(1, 3);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3b9c));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3b98));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x730));
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x74c));
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x750));
                c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x758));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6e4));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6e8));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6f0));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6f4));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6f8));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x724));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x71c), c.g[T3][0]);
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x72c));
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x734));
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x748));
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x754));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x720), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x728), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x740), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6ec), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6fc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x700), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x704), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x708), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x70c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x710), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x714), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x718), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x738), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x73c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x744), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x75c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x760), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x764), 0u64);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b94));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x768), 0u64);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x7c4));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x7c8));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x7cc));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x7d0));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x7d8));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x7dc));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x7e0));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x76c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x770), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x774), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x778), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x77c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x780), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x784), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x788), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x78c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x790), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x794), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x798), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x79c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7a0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7a4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7a8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7ac), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7b0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7b4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7b8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7bc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7c0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7d4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7e4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7e8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7ec), 0u64);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b90));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7f0), 0u64);
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x80c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x804), c.g[T3][0]);
                c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x818));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x828), c.g[T4][0]);
                c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x830));
                c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x834));
                c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x83c));
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x814));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x810), c.g[V1][0]);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x838));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x808), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7f4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7f8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7fc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x800), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x81c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x820), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x824), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x82c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x840), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x844), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x848), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x84c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x850), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x854), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x858), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x85c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x860), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x864), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x868), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x86c), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x870), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x874), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x878), 0u64);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x89c));
                c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x8ac));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8c8));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x87c));
                c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x88c));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8b0));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8b4));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8b8));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8c0));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8c4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8d4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x880), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x884), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x888), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x890), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x894), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x898), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8a0), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8a4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8a8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8bc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8cc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8d0), 0u64);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8d8), 0u64);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3b8c));
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8dc), 0u64);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8e0), 0u64);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8e4), 0u64);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8e8), 0u64);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8f0), c.g[T7][0]);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8f4));
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8ec), c.g[S0][0]);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8f8), c.g[T7][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x8fc));
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x900));
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x904), 0u64);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x908), 0u64);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                c.g[RA][0] = sx(0x002df17c);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x90c), 0u64);
                call_ptr(c, 0x00317e30, 0x002df17c)?;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                c.g[RA][0] = sx(0x002df188);
                c.g[A0][0] = c.g[S3][0];
                Callee {
                    addr: 0x002e26b8,
                    f: ee::s09::cDynamicSprayFX_ConfigureForRider,
                }
                .call(c, 0x002df188)?;
                c.g[RA][0] = sx(0x002df190);
                c.g[A0][0] = c.g[S3][0];
                Callee {
                    addr: 0x002df448,
                    f: ee::s09::cDynamicSprayFX_SetupEmitters,
                }
                .call(c, 0x002df190)?;
                c.g[RA][0] = sx(0x002df198);
                c.g[A0][0] = c.g[S3][0];
                Callee {
                    addr: 0x002df3b0,
                    f: ee::s09::cDynamicSprayFX_Reset,
                }
                .call(c, 0x002df198)?;
                c.g[V0][0] = c.g[S3][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: the magic word at `OC809_CTRL`.
    pub(crate) const OC809_CTRL: u32 = DATA + 0xf_d000;
    pub(crate) const OC809_MAGIC: u32 = 0x8095_a7c1;
    /// `(address, v0)` pairs: the result of every fake call to `address`.
    pub(crate) const OC809_BY_ADDR: u32 = OC809_CTRL + 0x100;

    /// A gated fake call: logs and clobbers like `fakes::fake`, then forces
    /// `v0` from the result table (unless the call unwound).
    pub(crate) fn oc809_fake_body(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc == ra {
            for k in 0..8 {
                let slot = OC809_BY_ADDR + 8 * k;
                if c.bus.read32(slot) == addr {
                    c.g[V0][0] = sx(c.bus.read32(slot + 4));
                }
            }
        }
    }

    /// Make every fake call to `addr` return `v` in `v0`.
    pub(crate) fn oc809_force(c: &mut Cpu, addr: u32, v: u32) {
        for k in 0..8 {
            let slot = OC809_BY_ADDR + 8 * k;
            if c.bus.read32(slot) == 0 {
                c.bus.write32(slot, addr);
                c.bus.write32(slot + 4, v);
                return;
            }
        }
        panic!("oc809_force: table full");
    }

    /// Gated fakes of the callees (each `hook`ed once per process; with the
    /// gate off they run the body they replaced).
    macro_rules! oc809_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC809_CTRL) != OC809_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc809_fake_body(c, $addr)
                }
            )*
            pub(crate) fn oc809_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc809_fakes! {
        oc809_fake_0011b698, OC809_PREV_0011B698, 0x0011_b698, ee::s00::OVERRIDE_0011B698, ee::s00::f_0011b698__generated;
        oc809_fake_002daa78, OC809_PREV_002DAA78, 0x002d_aa78, ee::s09::OVERRIDE_002DAA78, ee::s09::f_002daa78__generated;
        oc809_fake_002db748, OC809_PREV_002DB748, 0x002d_b748, ee::s09::OVERRIDE_002DB748, ee::s09::f_002db748__generated;
        oc809_fake_002df3b0, OC809_PREV_002DF3B0, 0x002d_f3b0, ee::s09::OVERRIDE_002DF3B0, ee::s09::f_002df3b0__generated;
        oc809_fake_002df448, OC809_PREV_002DF448, 0x002d_f448, ee::s09::OVERRIDE_002DF448, ee::s09::f_002df448__generated;
        oc809_fake_002e26b8, OC809_PREV_002E26B8, 0x002e_26b8, ee::s09::OVERRIDE_002E26B8, ee::s09::f_002e26b8__generated;
        oc809_fake_002f7a68, OC809_PREV_002F7A68, 0x002f_7a68, ee::s09::OVERRIDE_002F7A68, ee::s09::f_002f7a68__generated;
        oc809_fake_0036cbf8, OC809_PREV_0036CBF8, 0x0036_cbf8, ee::s12::OVERRIDE_0036CBF8, ee::s12::f_0036cbf8__generated;
        oc809_fake_0036ccb8, OC809_PREV_0036CCB8, 0x0036_ccb8, ee::s12::OVERRIDE_0036CCB8, ee::s12::f_0036ccb8__generated;
        oc809_fake_0036ce00, OC809_PREV_0036CE00, 0x0036_ce00, ee::s12::OVERRIDE_0036CE00, ee::s12::f_0036ce00__generated;
        oc809_fake_0036ce28, OC809_PREV_0036CE28, 0x0036_ce28, ee::s12::OVERRIDE_0036CE28, ee::s12::f_0036ce28__generated;
        oc809_fake_0036d008, OC809_PREV_0036D008, 0x0036_d008, ee::s12::OVERRIDE_0036D008, ee::s12::f_0036d008__generated;
        oc809_fake_0036d3d8, OC809_PREV_0036D3D8, 0x0036_d3d8, ee::s12::OVERRIDE_0036D3D8, ee::s12::f_0036d3d8__generated;
        oc809_fake_003714b8, OC809_PREV_003714B8, 0x0037_14b8, ee::s12::OVERRIDE_003714B8, ee::s12::f_003714b8__generated;
        oc809_fake_00317d70, OC809_PREV_00317D70, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc809_fake_00317e30, OC809_PREV_00317E30, 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated;
        oc809_fake_00416210, OC809_PREV_00416210, 0x0041_6210, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated;
    }

    /// A scrambled machine with the fakes on, the object registers set and
    /// `pc` at the function's entry; unwinds at one of `calls` fake calls.
    fn oc809_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc809_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC809_CTRL, OC809_MAGIC);
        r
    }

    /// The set of total cycle costs the generated body takes over the seeds.
    fn oc809_paths(
        f: ssx3_ee_rt::funcs::GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> std::collections::BTreeSet<u64> {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            f(&mut c);
            seen.insert(c.cycles - before);
        }
        seen
    }

    /// The object under construction, away from the fakes' allocations
    /// (`DATA..DATA+0x40000`) and the call log (`DATA+0x100000`).
    const OC809_THIS: u32 = DATA + 0x8_0000;

    /// `WorldPainterQuery_ctor` setup: `kind` selects the data object (0..15),
    /// the jump table of the generated body is written, the allocator is faked
    /// and the data-object constructors run for real.
    fn oc809_wpq_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc809_machine(c, seed, 0x2c0408, 1, 0x002c_0408);
        const TARGETS: [u32; 14] = [
            0x002c_0470,
            0x002c_0498,
            0x002c_04c0,
            0x002c_04e8,
            0x002c_0510,
            0x002c_0538,
            0x002c_0560,
            0x002c_0588,
            0x002c_05b0,
            0x002c_05d8,
            0x002c_0600,
            0x002c_0628,
            0x002c_0650,
            0x002c_0678,
        ];
        for (i, t) in TARGETS.iter().enumerate() {
            c.bus.write32(0x0048_3db0 + 4 * i as u32, *t);
        }
        // Kind 0..15: the selector `kind >> 8` is 14 or 15 for the no-object path.
        let sel = seed % 16;
        c.g[A0][0] = OC809_THIS as u64;
        c.g[A1][0] = ((sel << 8) | (r.next_u32() & 0xff)) as u64;
        c.bus
            .write32(GAME_GP + 0x790, DATA + 0x8_4000 + (r.next_u32() & 0xf0));
        c.bus.write32(GAME_GP.wrapping_sub(0x4288), r.next_u32());
    }

    /// `WorldPainterQuery_ctor` against the generated body: every selector
    /// (14 data-object kinds and the two without), the allocator faked (and
    /// unwinding), the data-object constructors real, the list link.
    #[test]
    fn world_painter_query_ctor_matches() {
        fakes::covered(
            ee::s08::f_002c0408__generated,
            world_painter_query_ctor,
            128,
            &oc809_wpq_setup,
        );
        let paths = oc809_paths(ee::s08::f_002c0408__generated, 128, &|c, seed| {
            oc809_wpq_setup(c, seed);
            c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        });
        assert!(paths.len() >= 8, "paths {paths:?}");
    }

    /// `cCrowdRender2D_ctor` against the generated body: `LoadTextures` is a
    /// gated fake (it unwinds in some seeds).
    #[test]
    fn crowd_render_2d_ctor_matches() {
        fakes::covered(
            ee::s09::f_002db6d8__generated,
            crowd_render_2d_ctor,
            60,
            &|c, seed| {
                oc809_machine(c, seed, 0x2db6d8, 1, 0x002d_b6d8);
                c.g[A0][0] = (DATA + 0x1000 + 16 * seed) as u64;
            },
        );
    }

    /// Fixture shared by the two FX constructors: `this` with a rider pointer
    /// (or none), the emitter object's virtual-call table, and unwinds.
    fn oc809_fx_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32, calls: u32) {
        let mut r = oc809_machine(c, seed, salt, calls, entry);
        c.g[A0][0] = OC809_THIS as u64;
        // `rider` set in half of the seeds; else `this->0` is the owner object.
        c.g[A1][0] = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u64 | 1
        };
        c.bus.write32(OC809_THIS, DATA + 0x9_0000);
        // The emitter object (result of `mEmitter_ctor`): +0x1f8 is its vtable,
        // whose entry (+0x18 delta, +0x1c method) points at a fake method.
        let emitter = DATA + 0xa_0000;
        let vtable = DATA + 0xa_1000;
        oc809_force(c, 0x0037_14b8, emitter);
        c.bus.write32(emitter + 0x1f8, vtable);
        c.bus
            .write16(vtable + 0x18, ((r.next_u32() as i16) & !3) as u16);
        c.bus.write32(vtable + 0x1c, fakes::FAKE);
        c.bus.write32(emitter + 0x58, r.next_u32());
    }

    /// `cBoardSparkFX_ctor` against the generated body: with and without a
    /// rider, the table copies, the three allocations, the emitter set-up
    /// calls and the virtual call (all gated fakes, unwinding at any of them).
    #[test]
    fn board_spark_fx_ctor_matches() {
        fakes::covered(
            ee::s09::f_002da390__generated,
            board_spark_fx_ctor,
            120,
            &|c, seed| oc809_fx_setup(c, seed, 0x2da390, 0x002d_a390, 16),
        );
        let paths = oc809_paths(ee::s09::f_002da390__generated, 8, &|c, seed| {
            oc809_fx_setup(c, seed, 0x2da390, 0x002d_a390, 0);
        });
        assert!(paths.len() >= 2, "paths {paths:?}");
    }

    /// `cDynamicSprayFX_ctor` against the generated body: with and without a
    /// rider, the table copies, the allocations and the ten emitter
    /// constructions, then configure/setup/reset (gated fakes, unwinding).
    #[test]
    fn dynamic_spray_fx_ctor_matches() {
        fakes::covered(
            ee::s09::f_002de4a8__generated,
            dynamic_spray_fx_ctor,
            120,
            &|c, seed| oc809_fx_setup(c, seed, 0x2de4a8, 0x002d_e4a8, 18),
        );
        let paths = oc809_paths(ee::s09::f_002de4a8__generated, 8, &|c, seed| {
            oc809_fx_setup(c, seed, 0x2de4a8, 0x002d_e4a8, 0);
        });
        assert!(paths.len() >= 2, "paths {paths:?}");
    }
}
