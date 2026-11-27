//! Staging module oc943 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc943`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// A widget's virtual call `widget->vtbl[off]` with this-adjust (`lh off`, `lw off + 4`),
/// as the compiler emits it: v0 = vtable, a0 = widget + delta, v1 = function.
/// The caller sets a1.. beforehand; `x` is the register holding the widget.
fn oc943_vcall(c: &mut Cpu, x: usize, off: u32, ret: u32) -> crate::call::Flow {
    let widget = c.g[x][0] as u32;
    c.g[V0][0] = Addr(widget).off(8).lw(c);
    let vt = c.g[V0][0] as u32;
    c.g[A0][0] = Addr(vt).off(off).lh(c);
    c.g[V1][0] = Addr(vt).off(off + 4).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx(widget.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `void cFEStatePeakRoom_RefreshGoalList(sRoom* self, int peak)` (0x001d3f80): hides the six widget rows of the five goal slots (icon, box, name, bar, medal, lock), then for each challenge of the peak shows its name, bar and box and either the unlocked icon (texture looked up through the reward entry's hash) with the medal, or the lock.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn c_fe_state_peak_room_refresh_goal_list(c: &mut Cpu) {
    if c.pc != 0x001d3f80 {
        return ee::s04::f_001d3f80__generated(c);
    }
    guest(c, |c| {
        c.cycles += 26;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x80));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S2][0] = 0u64;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xbc));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa8));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x94));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[RA][0] = sx(0x001d3fe0);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
        Callee {
            addr: 0x001ce6f0,
            f: ee::s04::cFEStateTrophyRoom_CountEventEntries,
        }
        .call(c, 0x001d3fe0)?;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
        'l_1d3fe8: loop {
            c.cycles += 52;
            c.g[A1][0] = 0u64;
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
            oc943_vcall(c, A2, 0x48, 0x001d400c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c));
            oc943_vcall(c, A2, 0x48, 0x001d402c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
            oc943_vcall(c, A2, 0x48, 0x001d404c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
            oc943_vcall(c, A2, 0x48, 0x001d406c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa8));
            oc943_vcall(c, A2, 0x48, 0x001d408c)?;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
            oc943_vcall(c, A2, 0x48, 0x001d40ac)?;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001d3fe8);
                }
                continue 'l_1d3fe8;
            } else {
                'b_1d4238: {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001d40c0);
                    c.g[S2][0] = 0u64;
                    call_ptr(c, 0x0014dc80, 0x001d40c0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d40cc);
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x001d40cc)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S7][0] = c.g[S4][0];
                    if t {
                        break 'b_1d4238;
                    } else {
                        c.cycles += 5;
                        c.g[FP][0] = c.g[S3][0];
                        c.g[S4][0] = c.g[S6][0];
                        c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        'l_1d40f0: loop {
                            'b_1d4214: {
                                'b_1d41f4: {
                                    c.cycles += 36;
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32));
                                    c.g[T0][0] = c.g[S2][0];
                                    c.g[A0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdc));
                                    c.g[RA][0] = sx(0x001d4108);
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                                    Callee {
                                        addr: 0x001ced90,
                                        f: ee::s04::cFEStateTrophyRoom_QueryEventReward,
                                    }
                                    .call(c, 0x001d4108)?;
                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001d4120);
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                                    Callee {
                                        addr: 0x001ce758,
                                        f: ee::s04::cFEStateTrophyRoom_SetEventName,
                                    }
                                    .call(c, 0x001d4120)?;
                                    c.g[A2][0] = c.lw((c.g[S3][0] as u32));
                                    c.g[A1][0] = 0x1;
                                    oc943_vcall(c, A2, 0x48, 0x001d413c)?;
                                    c.g[A2][0] = c.lw((c.g[S5][0] as u32));
                                    c.g[A1][0] = 0x1;
                                    oc943_vcall(c, A2, 0x48, 0x001d4158)?;
                                    c.g[A2][0] = c.lw((c.g[S6][0] as u32));
                                    c.g[A1][0] = 0x1;
                                    oc943_vcall(c, A2, 0x48, 0x001d4174)?;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[S0][0] == c.g[V1][0];
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                                    if t {
                                        break 'b_1d41f4;
                                    } else {
                                        c.cycles += 29;
                                        c.g[A2][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001d418c);
                                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        call_ptr(c, 0x00156ae0, 0x001d418c)?;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[RA][0] = sx(0x001d4198);
                                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        call_ptr(c, 0x00317670, 0x001d4198)?;
                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                        c.g[RA][0] = sx(0x001d41a4);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x00398380, 0x001d41a4)?;
                                        c.g[V1][0] = c.lw((c.g[S4][0] as u32));
                                        c.g[A1][0] = 0x1;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                        c.g[A2][0] = c.lw((c.g[S4][0] as u32));
                                        oc943_vcall(c, A2, 0x48, 0x001d41d0)?;
                                        c.g[A2][0] = c.lw((c.g[S7][0] as u32));
                                        c.g[A1][0] = 0x1;
                                        oc943_vcall(c, A2, 0x48, 0x001d41ec)?;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_1d4214;
                                        } else {
                                            break 'b_1d41f4;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A2][0] = c.lw((c.g[FP][0] as u32));
                                c.g[A1][0] = 0x1;
                                oc943_vcall(c, A2, 0x48, 0x001d4210)?;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                break 'b_1d4214;
                            }
                            c.cycles += 9;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4));
                            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001d40f0);
                                }
                                continue 'l_1d40f0;
                            } else {
                                break 'b_1d4238;
                            }
                        }
                    }
                }
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateTrophyRoom_OnWidgetCreated(self, cWidget* w)` (0x001d43f0): binds the trophy room's widgets by name hash (trophy image, a hidden button, the title text from the peak's string id, the six challenge names and their boxes) and shows or hides each by the number of event entries of the peak.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn c_fe_state_trophy_room_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d43f0 {
        return ee::s04::f_001d43f0__generated(c);
    }
    guest(c, |c| {
        'b_1d463c: {
            'b_1d4448: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S0][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = 0x4a0000;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1fe8));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = sx(0x001d4428);
                c.g[S3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                call_ptr(c, 0x00317670, 0x001d4428)?;
                let t = c.g[S3][0] != c.g[V0][0];
                c.g[A0][0] = 0x4a0000;
                if t {
                    break 'b_1d4448;
                } else {
                    c.cycles += 6;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[S0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001d4440);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x9c));
                    Callee {
                        addr: 0x001d47a0,
                        f: ee::s04::cFEStateTrophyRoom_ShowTrophyImage,
                    }
                    .call(c, 0x001d4440)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_1d463c;
                    } else {
                        break 'b_1d4448;
                    }
                }
            }
            'b_1d447c: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001d4450);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ff0));
                call_ptr(c, 0x00317670, 0x001d4450)?;
                if c.g[S3][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x460000;
                    break 'b_1d447c;
                } else {
                    c.cycles += 9;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                    c.g[A1][0] = 0u64;
                    oc943_vcall(c, S0, 0x48, 0x001d4474)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_1d463c;
                    } else {
                        break 'b_1d447c;
                    }
                }
            }
            'b_1d4508: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001d4484);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                call_ptr(c, 0x00317670, 0x001d4484)?;
                let t = c.g[S3][0] != c.g[V0][0];
                c.g[V1][0] = 0x440000;
                if t {
                    break 'b_1d4508;
                } else {
                    'b_1d44f8: {
                        'b_1d44ec: {
                            'b_1d44e4: {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                c.g[V0][0] = 0x1;
                                let t = c.g[A0][0] == c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1d44e4;
                                } else {
                                    'b_1d44dc: {
                                        'b_1d44b8: {
                                            c.cycles += 3;
                                            c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_1d44b8;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] == 0u64;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1d44dc;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1d44f8;
                                                    } else {
                                                        break 'b_1d44b8;
                                                    }
                                                }
                                            }
                                        }
                                        'b_1d44d0: {
                                            c.cycles += 2;
                                            let t = c.g[A0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1d44d0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] != c.g[V0][0];
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1d44f8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x76d8));
                                                    if t {
                                                        break 'b_1d44ec;
                                                    } else {
                                                        break 'b_1d44d0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76f0));
                                        if t {
                                            break 'b_1d44ec;
                                        } else {
                                            break 'b_1d44dc;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7708));
                                    if t {
                                        break 'b_1d44ec;
                                    } else {
                                        break 'b_1d44e4;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7720));
                            break 'b_1d44ec;
                        }
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x001d44f4);
                        call_ptr(c, 0x00317670, 0x001d44f4)?;
                        c.g[A1][0] = c.g[V0][0];
                        break 'b_1d44f8;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d4500);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x001d4500)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_1d463c;
                    } else {
                        break 'b_1d4508;
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = 0x440000;
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1618));
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1600));
            c.g[S2][0] = 0u64;
            'l_1d4518: loop {
                'b_1d462c: {
                    'b_1d4628: {
                        'b_1d45d0: {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001d4520);
                            c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                            call_ptr(c, 0x00317670, 0x001d4520)?;
                            let t = c.g[S3][0] != c.g[V0][0];

                            if t {
                                break 'b_1d45d0;
                            } else {
                                'b_1d4578: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                    c.g[RA][0] = sx(0x001d4534);
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                    Callee {
                                        addr: 0x001ce6f0,
                                        f: ee::s04::cFEStateTrophyRoom_CountEventEntries,
                                    }
                                    .call(c, 0x001d4534)?;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
                                    if t {
                                        break 'b_1d4578;
                                    } else {
                                        c.cycles += 14;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d4558);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001d4558)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d4570);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001d4570)?;
                                        let t = true;
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_1d4628;
                                        } else {
                                            break 'b_1d4578;
                                        }
                                    }
                                }
                                'b_1d45b8: {
                                    c.cycles += 2;
                                    if c.g[S2][0] != 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                        break 'b_1d45b8;
                                    } else {
                                        c.cycles += 14;
                                        c.g[RA][0] = sx(0x001d4588);
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        call_ptr(c, 0x0014dc80, 0x001d4588)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001d4594);
                                        c.g[A1][0] = 0xd;
                                        call_ptr(c, 0x0014dd58, 0x001d4594)?;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001d45a4);
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                        call_ptr(c, 0x00156a90, 0x001d45a4)?;
                                        c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                        c.g[RA][0] = sx(0x001d45b0);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x003a0c50, 0x001d45b0)?;
                                        let t = true;
                                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_1d462c;
                                        } else {
                                            break 'b_1d45b8;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                c.g[RA][0] = sx(0x001d45c8);
                                c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                Callee {
                                    addr: 0x001ce758,
                                    f: ee::s04::cFEStateTrophyRoom_SetEventName,
                                }
                                .call(c, 0x001d45c8)?;
                                let t = true;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1d4628;
                                } else {
                                    break 'b_1d45d0;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001d45d8);
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32));
                        call_ptr(c, 0x00317670, 0x001d45d8)?;
                        if c.g[S3][0] != c.g[V0][0] {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            break 'b_1d4628;
                        } else {
                            'b_1d4624: {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                                c.g[RA][0] = sx(0x001d45ec);
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                                Callee {
                                    addr: 0x001ce6f0,
                                    f: ee::s04::cFEStateTrophyRoom_CountEventEntries,
                                }
                                .call(c, 0x001d45ec)?;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1d4624;
                                } else {
                                    c.cycles += 11;
                                    oc943_vcall(c, S0, 0x48, 0x001d460c)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001d4624);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001d4624)?;
                                    break 'b_1d4624;
                                }
                            }
                            c.cycles += 1;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            break 'b_1d4628;
                        }
                    }
                    c.cycles += 1;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    break 'b_1d462c;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 6i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001d4518);
                    }
                    continue 'l_1d4518;
                } else {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'b_1d463c;
                }
            }
        }
        c.cycles += 8;
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
    });
}

/// `void MemCardUI_WriteSaveData(void)` (0x001d59a0): starts the save of the current memory-card operation: settings write, replay write (replay data size, file name and product code formatted into a title) or profile write, then, if no status popup is up and autosave is off, raises the "saving" popup with the overlay manager's message.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn mem_card_ui_write_save_data(c: &mut Cpu) {
    if c.pc != 0x001d59a0 {
        return ee::s04::f_001d59a0__generated(c);
    }
    guest(c, |c| {
        'b_1d5b68: {
            'b_1d59f8: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x7a0));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x730));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x790));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x780));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x770));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x760));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x750));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x740));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x720), c.g[RA][0]);
                c.g[RA][0] = sx(0x001d59d0);
                call_ptr(c, 0x00227f80, 0x001d59d0)?;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                c.g[S6][0] = c.g[V0][0];
                c.g[V0][0] = 0x2;
                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1bc));
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_1d59f8;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d59f0);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002412a0, 0x001d59f0)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    if t {
                        break 'b_1d5b68;
                    } else {
                        break 'b_1d59f8;
                    }
                }
            }
            'b_1d5ac0: {
                c.cycles += 2;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_1d5ac0;
                } else {
                    c.cycles += 48;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x001d5a10);
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x1c8), c.g[A1][0]);
                    call_ptr(c, 0x00270e68, 0x001d5a10)?;
                    c.g[RA][0] = sx(0x001d5a18);
                    call_ptr(c, 0x0014dc80, 0x001d5a18)?;
                    c.g[RA][0] = sx(0x001d5a20);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014e048, 0x001d5a20)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[RA][0] = sx(0x001d5a30);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x001d9948,
                        f: ee::s04::MemCardStateBase_AllocReadBuffer,
                    }
                    .call(c, 0x001d5a30)?;
                    c.g[RA][0] = sx(0x001d5a38);
                    call_ptr(c, 0x0014dc80, 0x001d5a38)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d5a48);
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d4));
                    call_ptr(c, 0x0014e050, 0x001d5a48)?;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x41;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001d5a5c);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x00241d40, 0x001d5a5c)?;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x320));
                    c.g[A1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x77a8));
                    c.g[RA][0] = sx(0x001d5a70);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c2540, 0x001d5a70)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[A1][0] = 0x100;
                    c.g[RA][0] = sx(0x001d5a80);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00241aa0, 0x001d5a80)?;
                    c.g[T0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xf8));
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001d5aa0);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x002c26d0, 0x001d5aa0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x001d5ab8);
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d4));
                    call_ptr(c, 0x00241000, 0x001d5ab8)?;
                    let t = true;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                    if t {
                        break 'b_1d5b68;
                    } else {
                        break 'b_1d5ac0;
                    }
                }
            }
            c.cycles += 42;
            c.g[RA][0] = sx(0x001d5ac8);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            call_ptr(c, 0x00227f80, 0x001d5ac8)?;
            c.g[RA][0] = sx(0x001d5ad0);
            c.g[S3][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001d5ad0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001d5adc);
            c.g[A1][0] = 0x5;
            call_ptr(c, 0x0014dd58, 0x001d5adc)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001d5aec);
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1cc));
            call_ptr(c, 0x00152950, 0x001d5aec)?;
            c.g[RA][0] = sx(0x001d5af4);
            c.g[S5][0] = c.g[V0][0];
            call_ptr(c, 0x00152ba8, 0x001d5af4)?;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0x41;
            c.g[S4][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            c.g[RA][0] = sx(0x001d5b0c);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x00241d40, 0x001d5b0c)?;
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x520));
            c.g[A1][0] = 0x460000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x77a8));
            c.g[RA][0] = sx(0x001d5b20);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c2540, 0x001d5b20)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[A1][0] = 0x100;
            c.g[RA][0] = sx(0x001d5b30);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00241aa0, 0x001d5b30)?;
            c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xf8));
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001d5b50);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
            call_ptr(c, 0x002c26d0, 0x001d5b50)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[RA][0] = sx(0x001d5b64);
            c.g[A3][0] = c.g[S4][0];
            call_ptr(c, 0x00241000, 0x001d5b64)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
            break 'b_1d5b68;
        }
        'b_1d5c04: {
            'b_1d5c00: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x210));
                let t = c.g[V0][0] != 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x790));
                if t {
                    break 'b_1d5c00;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x208));
                    let t = c.g[V0][0] != 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x780));
                    if t {
                        break 'b_1d5c04;
                    } else {
                        'b_1d5bfc: {
                            c.cycles += 29;
                            c.g[V0][0] = 0x16;
                            c.g[A1][0] = 0x460000;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x19c), c.g[V0][0]);
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d60));
                            c.g[A0][0] = 0x640;
                            c.g[RA][0] = sx(0x001d5ba0);
                            c.g[A2][0] = 0x100;
                            call_ptr(c, 0x00317e30, 0x001d5ba0)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S6][0];
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001d5bb4);
                            c.g[A2][0] = 0x320;
                            call_ptr(c, 0x00241e18, 0x001d5bb4)?;
                            c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = 0x1;
                            c.g[A3][0] = 0x1;
                            c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x8));
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0u64;
                            c.g[T2][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x120));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001d5be4);
                            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001d5be4)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                            c.g[V0][0] = 0x1;
                            let t = c.g[S0][0] == 0u64;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x210), c.g[V0][0]);
                            if t {
                                break 'b_1d5bfc;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x001d5bfc);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x001d5bfc)?;
                                break 'b_1d5bfc;
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x790));
                        break 'b_1d5c00;
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x780));
            break 'b_1d5c04;
        }
        c.cycles += 8;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x770));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x760));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x750));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x740));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x730));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x720));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x7a0));
        c.pc = t;
        return Ok(());
    });
}

/// `int cFEStateNetHelpPopup_TransitionIn(self, bool first)` (0x001da238): on the first call, fills the help popup's title, description and bottom texts (or the empty string), hides the nine icons, then shows the rows below the line count, positions each row's icon 22 units per row down and sets the row texts. Returns 1.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn c_fe_state_net_help_popup_transition_in(c: &mut Cpu) {
    if c.pc != 0x001da238 {
        return ee::s04::f_001da238__generated(c);
    }
    guest(c, |c| {
        'b_1da448: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
            let t = c.g[A1][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            if t {
                break 'b_1da448;
            } else {
                'b_1da290: {
                    'b_1da280: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                        if c.g[A1][0] == 0u64 {
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1da280;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001da278);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa8));
                            call_ptr(c, 0x003a1000, 0x001da278)?;
                            let t = true;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                            if t {
                                break 'b_1da290;
                            } else {
                                break 'b_1da280;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa8));
                    c.g[RA][0] = sx(0x001da28c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x001da28c)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
                    break 'b_1da290;
                }
                'b_1da2b8: {
                    'b_1da2a8: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x4a0000;
                        if t {
                            break 'b_1da2a8;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001da2a0);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001da110,
                                f: ee::s04::cFEStateNetHelpPopup_BuildBodyText,
                            }
                            .call(c, 0x001da2a0)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                            if t {
                                break 'b_1da2b8;
                            } else {
                                break 'b_1da2a8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xac));
                    c.g[RA][0] = sx(0x001da2b4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x001da2b4)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    break 'b_1da2b8;
                }
                'b_1da2e0: {
                    'b_1da2d0: {
                        c.cycles += 2;
                        if c.g[A1][0] == 0u64 {
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1da2d0;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001da2c8);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                            call_ptr(c, 0x003a1000, 0x001da2c8)?;
                            let t = true;
                            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                            if t {
                                break 'b_1da2e0;
                            } else {
                                break 'b_1da2d0;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb0));
                    c.g[RA][0] = sx(0x001da2dc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x001da2dc)?;
                    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    break 'b_1da2e0;
                }
                c.cycles += 4;
                c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0xb4));
                c.g[S0][0] = c.g[S3][0];
                c.g[S2][0] = 0x8;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                'l_1da2f0: loop {
                    c.cycles += 10;
                    c.g[A1][0] = 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    oc943_vcall(c, A2, 0x48, 0x001da310)?;
                    if (c.g[S2][0] as i64) >= 0 {
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                        if event_due(c) {
                            deliver_event_here(c, 0x001da2f0);
                        }
                        continue 'l_1da2f0;
                    } else {
                        c.cycles += 6;
                        c.g[S5][0] = c.g[S3][0];
                        c.g[S2][0] = c.g[S4][0];
                        c.g[S3][0] = 0u64;
                        c.g[S4][0] = 0u64;
                        c.g[S6][0] = 0x4a0000;
                        'l_1da330: loop {
                            'b_1da438: {
                                'b_1da418: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_1da418;
                                    } else {
                                        'b_1da408: {
                                            c.cycles += 46;
                                            c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                                            oc943_vcall(c, A2, 0x48, 0x001da358)?;
                                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
                                            c.g[A1][0] = 0x1;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[S4][0] as u32)));
                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[S5][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                            c.g[A2][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[A2][0] as u32;
                                            c.g[RA][0] = sx(0x001da388);
                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001da388)?;
                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                            c.mtc1(19, 1);
                                            c.cvt_s_w(1, 1);
                                            c.g[AT][0] = 0x41b00000;
                                            c.mtc1(1, 0);
                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[S4][0] as u32)));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[S5][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.mul_s(1, 1, 0);
                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x4b));
                                            c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x44));
                                            c.g[A1][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                            c.sdr(2, (c.g[SP][0] as u32));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                            c.add_s(0, 0, 1);
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                            c.ldr(2, (c.g[SP][0] as u32));
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x4b));
                                            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x44));
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x4c),
                                                c.g[A1][0],
                                            );
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x68));
                                            if c.g[A1][0] == 0u64 {
                                                c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                                                break 'b_1da408;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001da400);
                                                c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                                                call_ptr(c, 0x003a1000, 0x001da400)?;
                                                let t = true;
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_1da438;
                                                } else {
                                                    break 'b_1da408;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001da410);
                                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1408));
                                        call_ptr(c, 0x003a0c50, 0x001da410)?;
                                        let t = true;
                                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                        if t {
                                            break 'b_1da438;
                                        } else {
                                            break 'b_1da418;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                                c.g[A1][0] = 0u64;
                                oc943_vcall(c, A2, 0x48, 0x001da434)?;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                break 'b_1da438;
                            }
                            c.cycles += 4;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = ((c.g[S3][0] as i64) < 7i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001da330);
                                }
                                continue 'l_1da330;
                            } else {
                                break 'b_1da448;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = 0x1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEPopupScreenPos_onCreateScreen(self)` (0x001dee10): adds the popup screen (hash of the screen name) to the UI engine (`self+0x10`), stores it at `+0x40` and, when it exists, plays its first frame and stores the name hash at `+0xc`; then copies the 0x280-byte position table at 0x00535610 to the stack (inlined struct copy, 8-byte aligned path) and stores two of its bytes (sign-extended) at `+0x48` and `+0x4c`.
/// Reproduces every register, cycle and call of the generated body; `pc != entry`
/// (a resume after an unwound callee) runs the generated body.
pub fn c_fe_popup_screen_pos_init(c: &mut Cpu) {
    if c.pc != 0x001dee10 {
        return ee::s04::f_001dee10__generated(c);
    }
    guest(c, |c| {
        'b_1dee74: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2d0));
            c.g[V0][0] = 0x460000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x290), c.g[RA][0]);
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2228));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001dee3c);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001dee3c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001dee50);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001dee50)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1dee74;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001dee68);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001dee68)?;
                c.g[RA][0] = sx(0x001dee70);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00317670, 0x001dee70)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                break 'b_1dee74;
            }
        }
        'b_1def34: {
            'b_1def00: {
                c.cycles += 12;
                c.g[RA][0] = sx(0x001dee7c);
                call_ptr(c, 0x0014dc80, 0x001dee7c)?;
                c.g[A1][0] = 0x4;
                c.g[RA][0] = sx(0x001dee88);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x001dee88)?;
                c.g[V0][0] = 0x530000;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.g[V0][0];
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5610));
                c.g[V0][0] = c.g[A3][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = c.g[SP][0];
                if t {
                    break 'b_1def00;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
                    'l_1deeac: loop {
                        c.cycles += 19;
                        c.ldl(8, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(8, (c.g[A3][0] as u32));
                        c.ldl(9, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.ldr(9, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.ldr(10, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.ldl(11, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.ldr(11, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(8, (c.g[A2][0] as u32));
                        c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(11, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(11, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001deeac);
                            }
                            continue 'l_1deeac;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_1def34;
                            } else {
                                break 'b_1def00;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
            'l_1def08: loop {
                c.cycles += 11;
                c.g[T0][0] = c.ld((c.g[A3][0] as u32));
                c.g[T1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A2][0] as u32), c.g[T0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                let t = c.g[A3][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001def08);
                    }
                    continue 'l_1def08;
                } else {
                    break 'b_1def34;
                }
            }
        }
        'b_1defec: {
            'b_1defbc: {
                c.cycles += 12;
                c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[A3][0] as u32));
                c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[A2][0] as u32));
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x6));
                c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
                c.g[A2][0] = c.g[SP][0];
                c.g[V1][0] = c.g[A3][0] & 0x7;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                let t = c.g[V1][0] == 0u64;
                c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                if t {
                    break 'b_1defbc;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
                    'l_1def68: loop {
                        c.cycles += 19;
                        c.ldl(11, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(11, (c.g[A3][0] as u32));
                        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.ldr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0x17));
                        c.ldr(5, (c.g[A3][0] as u32).wrapping_add(0x10));
                        c.ldl(8, (c.g[A3][0] as u32).wrapping_add(0x1f));
                        c.ldr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                        c.sdl(11, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.sdr(11, (c.g[A2][0] as u32));
                        c.sdl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.sdr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.sdr(5, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        let t = c.g[A3][0] != c.g[V0][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001def68);
                            }
                            continue 'l_1def68;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_1defec;
                            } else {
                                break 'b_1defbc;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x280));
            'l_1defc0: loop {
                c.cycles += 11;
                c.g[T1][0] = c.ld((c.g[A3][0] as u32));
                c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A2][0] as u32), c.g[T1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                let t = c.g[A3][0] != c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001defc0);
                    }
                    continue 'l_1defc0;
                } else {
                    break 'b_1defec;
                }
            }
        }
        c.cycles += 14;
        c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(5, (c.g[A3][0] as u32));
        c.sdl(5, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.sdr(5, (c.g[A2][0] as u32));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x5));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2a0));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x290));
        c.sw((c.g[A0][0] as u32), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.pc = t;
        return Ok(());
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d3f80,
        c_fe_state_peak_room_refresh_goal_list,
        "cFEStatePeakRoom_RefreshGoalList",
    ),
    (
        0x001d43f0,
        c_fe_state_trophy_room_on_widget_created,
        "cFEStateTrophyRoom_OnWidgetCreated",
    ),
    (
        0x001d59a0,
        mem_card_ui_write_save_data,
        "MemCardUI_WriteSaveData",
    ),
    (
        0x001da238,
        c_fe_state_net_help_popup_transition_in,
        "cFEStateNetHelpPopup_TransitionIn",
    ),
    (
        0x001dee10,
        c_fe_popup_screen_pos_init,
        "cFEPopupScreenPos_Init",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{elf_image, fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and its magic (each machine of a diff gets its own).
    const OC943_CTRL: u32 = DATA + 0x2e_0000;
    const OC943_MAGIC: u32 = 0x4f43_3934;
    /// Forced results: entry `i` is `{flag, value}`; the fake sign-extends the value into v0.
    const OC943_MBOX: u32 = DATA + 0x2e_0100;
    /// Result table of the `GetHashValue32` fake, indexed by `a0 & 0xffffc`.
    const OC943_HASH: u32 = DATA + 0x60_0000;
    /// The widgets' shared vtable.
    const OC943_VT: u32 = DATA + 0x1_0000;

    macro_rules! oc943_fakes {
        ($install:ident; $(($idx:expr, $fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC943_CTRL) != OC943_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc943_fake_call(c, $idx, $addr);
                }
            )*
            fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    /// A logged fake call, then the forced result of mailbox `idx` (if set) or, for
    /// `GetHashValue32`, the hash table entry of the string address.
    fn oc943_fake_call(c: &mut Cpu, idx: u32, addr: u32) {
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let mb = OC943_MBOX + 8 * idx;
        if c.bus.read32(mb) != 0 {
            c.g[V0][0] = sx(c.bus.read32(mb + 4));
        } else if addr == 0x0031_7670 {
            let h = c.bus.read32(OC943_HASH + (a0 & 0xf_fffc));
            if h != 0 {
                c.g[V0][0] = h as u64;
            }
        }
    }

    /// Force the result of fake `idx` (index in the `oc943_fakes!` table).
    fn oc943_force(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC943_MBOX + 8 * idx, 1);
        c.bus.write32(OC943_MBOX + 8 * idx + 4, v);
    }

    const OC943_GET_APP: u32 = 11;
    const OC943_NEW: u32 = 21;
    const OC943_ADD_SCREEN: u32 = 23;

    oc943_fakes!(
        oc943_install_fakes;
        (0, oc943_fake_0014dc80, OC943_PREV_0014DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80),
        (1, oc943_fake_0014dd58, OC943_PREV_0014DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58),
        (2, oc943_fake_0014e048, OC943_PREV_0014E048, ee::s01::OVERRIDE_0014E048, ee::s01::f_0014e048__generated, 0x0014_e048),
        (3, oc943_fake_0014e050, OC943_PREV_0014E050, ee::s01::OVERRIDE_0014E050, ee::s01::f_0014e050__generated, 0x0014_e050),
        (4, oc943_fake_00152950, OC943_PREV_00152950, ee::s01::OVERRIDE_00152950, ee::s01::f_00152950__generated, 0x0015_2950),
        (5, oc943_fake_00152ba8, OC943_PREV_00152BA8, ee::s01::OVERRIDE_00152BA8, ee::s01::f_00152ba8__generated, 0x0015_2ba8),
        (6, oc943_fake_00156a90, OC943_PREV_00156A90, ee::s01::OVERRIDE_00156A90, ee::s01::f_00156a90__generated, 0x0015_6a90),
        (7, oc943_fake_00156ae0, OC943_PREV_00156AE0, ee::s01::OVERRIDE_00156AE0, ee::s01::f_00156ae0__generated, 0x0015_6ae0),
        (8, oc943_fake_001d47a0, OC943_PREV_001D47A0, ee::s04::OVERRIDE_001D47A0, ee::s04::f_001d47a0__generated, 0x001d_47a0),
        (9, oc943_fake_001d9948, OC943_PREV_001D9948, ee::s04::OVERRIDE_001D9948, ee::s04::f_001d9948__generated, 0x001d_9948),
        (10, oc943_fake_001da110, OC943_PREV_001DA110, ee::s04::OVERRIDE_001DA110, ee::s04::f_001da110__generated, 0x001d_a110),
        (11, oc943_fake_00227f80, OC943_PREV_00227F80, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated, 0x0022_7f80),
        (12, oc943_fake_00241000, OC943_PREV_00241000, ee::s06::OVERRIDE_00241000, ee::s06::f_00241000__generated, 0x0024_1000),
        (13, oc943_fake_002412a0, OC943_PREV_002412A0, ee::s06::OVERRIDE_002412A0, ee::s06::f_002412a0__generated, 0x0024_12a0),
        (14, oc943_fake_00241aa0, OC943_PREV_00241AA0, ee::s06::OVERRIDE_00241AA0, ee::s06::f_00241aa0__generated, 0x0024_1aa0),
        (15, oc943_fake_00241d40, OC943_PREV_00241D40, ee::s06::OVERRIDE_00241D40, ee::s06::f_00241d40__generated, 0x0024_1d40),
        (16, oc943_fake_00241e18, OC943_PREV_00241E18, ee::s06::OVERRIDE_00241E18, ee::s06::f_00241e18__generated, 0x0024_1e18),
        (17, oc943_fake_00270e68, OC943_PREV_00270E68, ee::s07::OVERRIDE_00270E68, ee::s07::f_00270e68__generated, 0x0027_0e68),
        (18, oc943_fake_002c2540, OC943_PREV_002C2540, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated, 0x002c_2540),
        (19, oc943_fake_002c26d0, OC943_PREV_002C26D0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated, 0x002c_26d0),
        (20, oc943_fake_00317670, OC943_PREV_00317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (21, oc943_fake_00317e30, OC943_PREV_00317E30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated, 0x0031_7e30),
        (22, oc943_fake_00317e98, OC943_PREV_00317E98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, 0x0031_7e98),
        (23, oc943_fake_00397cc0, OC943_PREV_00397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x0039_7cc0),
        (24, oc943_fake_00398380, OC943_PREV_00398380, ee::s13::OVERRIDE_00398380, ee::s13::f_00398380__generated, 0x0039_8380),
        (25, oc943_fake_0039c870, OC943_PREV_0039C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (26, oc943_fake_003a0c50, OC943_PREV_003A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (27, oc943_fake_003a1000, OC943_PREV_003A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000),
        (28, oc943_fake_00144c60, OC943_PREV_00144C60, ee::s01::OVERRIDE_00144C60, ee::s01::f_00144c60__generated, 0x0014_4c60),
        (29, oc943_fake_001509c0, OC943_PREV_001509C0, ee::s01::OVERRIDE_001509C0, ee::s01::f_001509c0__generated, 0x0015_09c0),
        (30, oc943_fake_00153390, OC943_PREV_00153390, ee::s01::OVERRIDE_00153390, ee::s01::f_00153390__generated, 0x0015_3390),
        (31, oc943_fake_00153688, OC943_PREV_00153688, ee::s01::OVERRIDE_00153688, ee::s01::f_00153688__generated, 0x0015_3688),
        (32, oc943_fake_00154368, OC943_PREV_00154368, ee::s01::OVERRIDE_00154368, ee::s01::f_00154368__generated, 0x0015_4368),
        (33, oc943_fake_001544d0, OC943_PREV_001544D0, ee::s01::OVERRIDE_001544D0, ee::s01::f_001544d0__generated, 0x0015_44d0),
        (34, oc943_fake_00158960, OC943_PREV_00158960, ee::s01::OVERRIDE_00158960, ee::s01::f_00158960__generated, 0x0015_8960),
        (35, oc943_fake_00198af0, OC943_PREV_00198AF0, ee::s03::OVERRIDE_00198AF0, ee::s03::f_00198af0__generated, 0x0019_8af0),
        (36, oc943_fake_002c27c0, OC943_PREV_002C27C0, ee::s08::OVERRIDE_002C27C0, ee::s08::f_002c27c0__generated, 0x002c_27c0),
        (37, oc943_fake_003a0e90, OC943_PREV_003A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90),
        (38, oc943_fake_00413af8, OC943_PREV_00413AF8, ee::s15::OVERRIDE_00413AF8, ee::s15::f_00413af8__generated, 0x0041_3af8),
        (39, oc943_fake_00417828, OC943_PREV_00417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828),
        (40, oc943_fake_001589b0, OC943_PREV_001589B0, ee::s01::OVERRIDE_001589B0, ee::s01::f_001589b0__generated, 0x0015_89b0),
        (41, oc943_fake_00158a50, OC943_PREV_00158A50, ee::s01::OVERRIDE_00158A50, ee::s01::f_00158a50__generated, 0x0015_8a50),
        (42, oc943_fake_00158910, OC943_PREV_00158910, ee::s01::OVERRIDE_00158910, ee::s01::f_00158910__generated, 0x0015_8910),
        (43, oc943_fake_00158af0, OC943_PREV_00158AF0, ee::s01::OVERRIDE_00158AF0, ee::s01::f_00158af0__generated, 0x0015_8af0),
    );

    /// Gate open, the widgets' vtable (`+0x40`, `+0x48` and `+0x120` slots are fake calls with
    /// a this-adjust of 0, 0x10 or 0x20).
    fn oc943_open(c: &mut Cpu, seed: u32) {
        c.bus.write32(OC943_CTRL, OC943_MAGIC);
        for (k, off) in [0x40u32, 0x48, 0x120].into_iter().enumerate() {
            c.bus.write16(OC943_VT + off, 0x10 * (seed % 3) as u16);
            c.bus.write32(OC943_VT + off + 4, FAKE + 0x10 * k as u32);
        }
    }

    /// A widget at `at` with the shared vtable.
    fn oc943_widget(c: &mut Cpu, at: u32) {
        c.bus.write32(at + 8, OC943_VT);
    }

    /// Hash table value of the string at `addr`.
    fn oc943_set_hash(c: &mut Cpu, addr: u32, h: u32) {
        c.bus.write32(OC943_HASH + (addr & 0xf_fffc), h);
    }

    /// `RefreshGoalList(self, peak)`: the six rows of five widgets, the event count of
    /// (mountain, peak) from the real table, and per goal either a locked (reward -1) or
    /// an unlocked icon (reward id, texture handle store). The real `QueryEventReward` and
    /// `SetEventName` run on faked back-end callees.
    #[test]
    fn c_fe_state_peak_room_refresh_goal_list_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001d3f80__generated,
            c_fe_state_peak_room_refresh_goal_list,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d3f80, 30);
                elf_image(c);
                oc943_open(c, seed);
                let this = DATA + 0x1000;
                for (k, base) in [0x58u32, 0x6c, 0x80, 0x94, 0xa8, 0xbc]
                    .into_iter()
                    .enumerate()
                {
                    for j in 0..5u32 {
                        let widget = DATA + 0x2000 + 0x100 * (k as u32 * 5 + j);
                        c.bus.write32(this + base + 4 * j, widget);
                        oc943_widget(c, widget);
                    }
                }
                c.bus.write8(this + 0x44, (seed % 3) as u8);
                c.bus.write32(this + 0x10, DATA + 0x8000);
                c.bus.write32(this + 0xdc, 7 + seed % 2);
                c.bus.write32(this + 0xe0, (seed / 4) % 3);
                for idx in 40..=43 {
                    oc943_force(c, idx, if seed % 3 == 0 { u32::MAX } else { seed % 7 });
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x001d_3f80;
            },
        );
    }

    /// `OnWidgetCreated(self, w)`: the widget's name hash picks the trophy image, the hidden
    /// button, the title text by peak, or one of the six challenge names / boxes (shown or
    /// hidden by the peak's entry count; entry 0 reads the peak name from the reward table).
    #[test]
    fn c_fe_state_trophy_room_on_widget_created_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001d43f0__generated,
            c_fe_state_trophy_room_on_widget_created,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d43f0, 14);
                elf_image(c);
                oc943_open(c, seed);
                let this = DATA + 0x1000;
                let w = DATA + 0x3000;
                oc943_widget(c, w);
                // Strings of the loop tables (the tables are in the image: rewrite
                // them to known pointers) and their hashes.
                for i in 0..6u32 {
                    c.bus.write32(0x0044_1600 + 4 * i, 0x004a_3000 + 0x10 * i);
                    c.bus.write32(0x0044_1618 + 4 * i, 0x004a_3100 + 0x10 * i);
                    oc943_set_hash(c, 0x004a_3000 + 0x10 * i, 0x4000 + i);
                    oc943_set_hash(c, 0x004a_3100 + 0x10 * i, 0x5000 + i);
                }
                oc943_set_hash(c, 0x004a_1fe8, 0x1111);
                oc943_set_hash(c, 0x004a_1ff0, 0x2222);
                oc943_set_hash(c, 0x0046_6e60, 0x3333);
                let kind = seed % 20;
                let h = match kind {
                    0 => 0x1111,
                    1 => 0x2222,
                    2..=4 => 0x3333,
                    5..=10 => 0x4000 + (kind - 5),
                    11..=16 => 0x5000 + (kind - 11),
                    _ => 0x7777,
                };
                c.bus.write32(w + 0x38, h);
                c.bus.write32(this + 0x54, (seed / 7) % 3);
                c.bus.write32(this + 0x58, (seed / 20) % 5);
                c.bus.write32(this + 0x9c, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(w);
                c.pc = 0x001d_43f0;
            },
        );
    }

    /// `WriteSaveData()`: settings write (mode 2), replay write (mode 1: data size, file name,
    /// product code, formatted title) or profile write (other modes); then the "saving"
    /// popup unless a status popup is up or autosave is on, freeing the message buffer when
    /// it was allocated.
    #[test]
    fn mem_card_ui_write_save_data_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001d59a0__generated,
            mem_card_ui_write_save_data,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d59a0, 24);
                oc943_open(c, seed);
                let inst = DATA + 0x1000;
                let app = DATA + 0x4000;
                let world = DATA + 0x5000;
                c.bus.write32(GAME_GP.wrapping_sub(0x10c8), inst);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                fill(c, &mut r, world + 0x80, 8);
                c.bus.write32(world + 0x84, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000 + 0x20, 4);
                c.bus
                    .write32(inst + 0x1bc, [2, 1, 0, 3][(seed % 4) as usize]);
                c.bus.write32(inst + 0x1d4, r.next_u32());
                c.bus.write32(inst + 0x1cc, r.next_u32());
                c.bus.write32(inst + 0x208, (seed / 4) % 2);
                c.bus.write32(inst + 0x210, (seed / 8) % 2);
                oc943_widget(c, inst);
                c.bus.write32(app + 0xf8, r.next_u32());
                oc943_force(c, OC943_GET_APP, app);
                oc943_force(
                    c,
                    OC943_NEW,
                    if (seed / 16) % 2 == 0 {
                        0
                    } else {
                        DATA + 0x7000
                    },
                );
                c.pc = 0x001d_59a0;
            },
        );
    }

    /// `TransitionIn(self, first)`: nothing on a repeat call; the first call sets the three
    /// texts (string id or empty), hides the nine icons and fills the rows below the state
    /// (row shown, icon shown and moved down 22 per row, row text).
    #[test]
    fn c_fe_state_net_help_popup_transition_in_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001da238__generated,
            c_fe_state_net_help_popup_transition_in,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1da238, 40);
                oc943_open(c, seed);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x54, (seed % 2) * r.next_u32());
                c.bus.write32(this + 0x58, (seed / 2) % 2);
                c.bus.write32(this + 0x64, (seed / 4) % 2 * r.next_u32());
                for (k, off) in [0xa8u32, 0xac, 0xb0].into_iter().enumerate() {
                    let widget = DATA + 0x4000 + 0x100 * k as u32;
                    c.bus.write32(this + off, widget);
                    oc943_widget(c, widget);
                }
                c.bus.write32(this + 0x48, (seed / 8) % 9);
                for i in 0..7u32 {
                    let row = DATA + 0x6000 + 0x100 * i;
                    c.bus.write32(this + 0xb4 + 4 * i, row);
                    oc943_widget(c, row);
                    c.bus.write32(
                        this + 0x68 + 4 * i,
                        if (seed + i) % 3 == 0 { 0 } else { 0x77 + i },
                    );
                    c.bus.write32(this + 0x84 + 4 * i, (seed + i * 3) % 9);
                }
                for j in 0..9u32 {
                    let icon = DATA + 0x5000 + 0x100 * j;
                    c.bus.write32(this + 0xd0 + 4 * j, icon);
                    oc943_widget(c, icon);
                    for k in 0..3 {
                        c.bus.write32(icon + 0x44 + 4 * k, r.f32(100.0));
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { 1 };
                c.pc = 0x001d_a238;
            },
        );
    }

    /// `ScreenPos::Init(self)`: adds the screen by name hash (found or missing, playing its
    /// first frame and storing the hash when found), then the inlined copy of the
    /// 0x280-byte position table with two byte fields stored. The table address is a constant
    /// 8-byte aligned, so the unaligned copy variants of the generated body are not reachable.
    #[test]
    fn c_fe_popup_screen_pos_init_matches() {
        oc943_install_fakes();
        fakes::covered(
            ee::s04::f_001dee10__generated,
            c_fe_popup_screen_pos_init,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1dee10, 8);
                elf_image(c);
                oc943_open(c, seed);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x10, DATA + 0x8000);
                fill(c, &mut r, 0x0053_5610, 0xa0);
                oc943_set_hash(c, 0x0045_ddd8, 0x1234_5678);
                oc943_force(
                    c,
                    OC943_ADD_SCREEN,
                    if seed % 3 == 0 { 0 } else { DATA + 0x3000 },
                );
                c.g[A0][0] = sx(this);
                c.pc = 0x001d_ee10;
            },
        );
    }
}
