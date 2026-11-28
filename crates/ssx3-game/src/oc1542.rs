//! Staging module oc1542 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1542`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001f_f748, ov_rewards_on_widget_menu, "cOVState_REWARDS_onWidgetMenu"),
    (0x001f_f780, lbl_001ff780_show_page_tail, "lbl_001FF780"),
    (0x001f_f790, lbl_001ff790_tail, "lbl_001FF790"),
    (0x001f_f798, lbl_001ff798_tail, "lbl_001FF798"),
    (0x0020_df10, lbl_0020df10_count_check, "lbl_0020DF10"),
    (0x0021_2788, ov_list_clear, "OVList_Clear"),
    (0x0021_c640, ov_state_overlay_ctor, "cOVStateOverlay_ctor"),
    (0x0021_cb08, ov_state_view_popup_ctor_2, "cOVState_ViewPopup_ctor_2"),
];

/// `cOVState_REWARDS::onWidgetMenu` (0x001ff748): a state dispatcher. The
/// state word (`A0`) minus 2 indexes a jump table of 8 words at 0x004703e0.
/// States past the table tail-call 0x001ff7a8 (the shared return-0x101 tail).
/// A table entry of 0x001ff748 loops back here after an event check; any other
/// entry is a jump out of the function. Kept as the generated loop, so the
/// frame grows with each pass exactly as the original does.
pub fn ov_rewards_on_widget_menu(c: &mut Cpu) {
    loop {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x2));
        let s = sp(c).wrapping_sub(0x10);
        c.g[SP][0] = sx(s);
        let ra = c.g[RA][0];
        c.sd(s, ra);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A1][0] = c.g[A3][0];
        if t {
            c.pc = 0x001f_f7a8;
            ee::s05::f_001ff7a8(c);
            return;
        }
        c.cycles += 7;
        c.g[V0][0] = 0x470000;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e0));
        let entry_addr = (c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32);
        c.g[V1][0] = sx(entry_addr);
        c.g[A0][0] = c.lw(entry_addr);
        let target = c.g[A0][0] as u32;
        if target == 0x001f_f748 {
            if event_due(c) {
                deliver_event_here(c, 0x001f_f748);
            }
            continue;
        }
        c.jump_out(target, 0x001f_f748, 0x001f_f780);
        return;
    }
}

/// `0x001ff780` (no name): calls `OVScreenRewardList_ShowPage(A2)` (0x001ffd08),
/// then tail-calls the epilogue at 0x001ff7ac with `V0 = 1`.
pub fn lbl_001ff780_show_page_tail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[A0][0] = c.g[A2][0];
        Callee {
            addr: 0x001f_fd08,
            f: ee::s05::f_001ffd08,
        }
        .call(c, 0x001f_f788)?;
        c.g[V0][0] = 0x1;
        c.pc = 0x001f_f7ac;
        call_indirect(c, 0x001f_f7ac);
        Ok(())
    });
}

/// `0x001ff790` (no name): returns 0 through the shared 0x001ff7ac epilogue
/// (`V0 = 0`, tail jump).
pub fn lbl_001ff790_tail(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    c.pc = 0x001f_f7ac;
    call_indirect(c, 0x001f_f7ac);
}

/// `0x001ff798` (no name): `V0 = 0`, and a tail jump to the 0x001ff7ac epilogue
/// when the word at `A2 + 0x9c` (signed) is below 10, else to 0x001ff7a8.
pub fn lbl_001ff798_tail(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x9c));
    c.g[V1][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
    let t = c.g[V1][0] != 0u64;
    c.g[V0][0] = 0u64;
    if t {
        c.pc = 0x001f_f7ac;
        call_indirect(c, 0x001f_f7ac);
    } else {
        c.pc = 0x001f_f7a8;
        ee::s05::f_001ff7a8(c);
    }
}

/// `0x0020df10` (no name, a leaf): `V0` is 0 when the word at `A0 + 0x9c` is
/// zero, else 0x101 unless `3 < A2 - 2` (unsigned), which gives 0.
pub fn lbl_0020df10_count_check(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x9c));
    let t = c.g[V1][0] == 0u64;
    c.g[V0][0] = 0u64;
    if !t {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x2));
        c.g[V1][0] = 0x3;
        c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
        c.g[V0][0] = 0x101;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0u64;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `OVList_Clear(this)` (0x00212788): unlinks and deletes every node of the
/// list whose sentinel is at `this + 4` (`__builtin_delete` on each node), then
/// makes the sentinel point to itself. Returns nothing.
pub fn ov_list_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x10));
        c.sq(16, s.wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        let ra = c.g[RA][0];
        let s = sp(c);
        c.sd(s, ra);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[S0][0] = c.lw(c.g[V0][0] as u32);
        if c.g[S0][0] == c.g[V0][0] {
            let s = sp(c);
            c.lq(16, s.wrapping_add(0x20));
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[S0][0] = c.lw(c.g[S0][0] as u32);
                call_ptr(c, 0x0031_7e50, 0x0021_27b8)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0021_27b0);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                let s = sp(c);
                c.lq(16, s.wrapping_add(0x20));
                break;
            }
        }
        c.cycles += 7;
        let v0 = c.g[V0][0];
        c.sw(v0 as u32, v0);
        let s = sp(c);
        c.g[RA][0] = c.ld(s);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        let s = sp(c);
        c.lq(17, s.wrapping_add(0x10));
        let v1 = c.g[V1][0];
        c.sw((v1 as u32).wrapping_add(0x4), v1);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// The shared body of the two `cOVState` overlay constructors: calls
/// `UIState_ctor(this, 0)` (0x0039e2a0) returning to `ret`, then stores the
/// vtable `0x470000 + vtable_lo` at `this + 8` and returns `this` in `V0`.
fn ov_state_base_ctor(c: &mut Cpu, ret: u32, vtable_lo: u32) -> Flow {
    c.cycles += 14;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    c.g[A1][0] = 0u64;
    let s = sp(c);
    c.sq(16, s.wrapping_add(0x10));
    let ra = c.g[RA][0];
    c.sd(s, ra);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, 0x0039_e2a0, ret)?;
    c.g[V1][0] = 0x470000;
    c.g[V0][0] = c.g[S0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(vtable_lo));
    let s = sp(c);
    c.g[RA][0] = c.ld(s);
    let this = c.g[S0][0] as u32;
    let vt = c.g[V1][0];
    c.sw(this.wrapping_add(0x8), vt);
    let s = sp(c);
    c.lq(16, s.wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
    c.pc = t;
    Ok(())
}

/// `cOVStateOverlay_ctor(this)` (0x0021c640): base constructor through
/// `UIState_ctor`, vtable 0x474e08. Returns `this`.
pub fn ov_state_overlay_ctor(c: &mut Cpu) {
    guest(c, |c| ov_state_base_ctor(c, 0x0021_c658, 0x4e08));
}

/// `cOVState_ViewPopup_ctor_2(this)` (0x0021cb08): the same constructor with
/// vtable 0x474248.
pub fn ov_state_view_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| ov_state_base_ctor(c, 0x0021_cb20, 0x4248));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Each fake's gate word in the test machine's RAM, and the magic that opens it.
    const GATE_DELETE: u32 = DATA + 0x20_1542;
    const GATE_SHOW: u32 = DATA + 0x20_1546;
    const GATE_CTOR: u32 = DATA + 0x20_154a;
    const GATE_MAGIC: u32 = 0x1542_c0de;
    /// The jump table of `ov_rewards_on_widget_menu` (game image data).
    const REWARDS_TABLE: u32 = 0x0047_03e0;

    /// `__builtin_delete` (0x317e50), gated: logs the call and returns.
    fn oc1542_fake_delete(c: &mut Cpu) {
        if c.bus.read32(GATE_DELETE) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1542_fake_delete),
                ee::s10::f_00317e50__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7e50);
    }

    /// `OVScreenRewardList_ShowPage` (0x1ffd08), gated.
    fn oc1542_fake_show_page(c: &mut Cpu) {
        if c.bus.read32(GATE_SHOW) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1542_fake_show_page),
                ee::s05::f_001ffd08__generated,
            );
        }
        fakes::fake_call(c, 0x001f_fd08);
    }

    /// `UIState_ctor` (0x39e2a0), gated.
    fn oc1542_fake_ui_state_ctor(c: &mut Cpu) {
        if c.bus.read32(GATE_CTOR) != GATE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1542_fake_ui_state_ctor),
                ee::s13::f_0039e2a0__generated,
            );
        }
        fakes::fake_call(c, 0x0039_e2a0);
    }

    /// The event hook of the dispatcher test: makes every table entry a jump
    /// out and stops further events, so the loop leaves after one pass.
    fn oc1542_rewrite_table(c: &mut Cpu) -> bool {
        for k in 0..8u32 {
            c.bus.write32(REWARDS_TABLE + 4 * k, 0x0040_0000);
        }
        c.next_event = u64::MAX;
        false
    }

    #[test]
    fn ov_rewards_on_widget_menu_matches() {
        check(
            ee::s05::f_001ff748__generated,
            ov_rewards_on_widget_menu,
            300,
            &|c, seed| {
                machine(c, seed, 0x1542_1748, 0);
                // kind 0: every entry jumps out; 1: every entry loops (one
                // event, which rewrites the table); 2: odd entries loop.
                let kind = (seed / 5) % 3;
                for k in 0..8u32 {
                    let v = match kind {
                        0 => 0x0040_0000,
                        1 => 0x001f_f748,
                        _ if k % 2 == 1 => 0x001f_f748,
                        _ => 0x0040_0010,
                    };
                    c.bus.write32(REWARDS_TABLE + 4 * k, v);
                }
                // States 0..7 index the table; seeds with `seed % 5 == 0` take the tail.
                let state = if seed % 5 == 0 {
                    10 + seed
                } else {
                    2 + seed % 8
                };
                c.g[A0][0] = sx(state);
                if kind != 0 {
                    c.next_event = 0;
                    c.hooks.event = oc1542_rewrite_table;
                }
            },
        );
    }

    #[test]
    fn lbl_001ff780_matches() {
        check(
            ee::s05::f_001ff780__generated,
            lbl_001ff780_show_page_tail,
            200,
            &|c, seed| {
                machine(c, seed, 0x1542_1780, 2);
                fakes::hook(
                    addr_of_mut!(ee::s05::OVERRIDE_001FFD08),
                    oc1542_fake_show_page,
                    fakes::prev_cell(oc1542_fake_show_page),
                );
                c.bus.write32(GATE_SHOW, GATE_MAGIC);
                c.g[A2][0] = sx(DATA + 0x20_3000 + 0x10 * (seed % 4));
            },
        );
    }

    #[test]
    fn lbl_001ff790_matches() {
        check(
            ee::s05::f_001ff790__generated,
            lbl_001ff790_tail,
            50,
            &|c, seed| {
                machine(c, seed, 0x1542_1790, 0);
            },
        );
    }

    #[test]
    fn lbl_001ff798_matches() {
        check(
            ee::s05::f_001ff798__generated,
            lbl_001ff798_tail,
            200,
            &|c, seed| {
                machine(c, seed, 0x1542_1798, 0);
                let obj = DATA + 0x20_3400;
                // Below 10 (signed) takes the 0x1ff7ac tail, else 0x1ff7a8.
                let v: u32 = match seed % 3 {
                    0 => seed % 9,
                    1 => 10 + seed % 7,
                    _ => 0xffff_fff0,
                };
                c.bus.write32(obj + 0x9c, v);
                c.g[A2][0] = sx(obj);
            },
        );
    }

    #[test]
    fn lbl_0020df10_matches() {
        check(
            ee::s05::f_0020df10__generated,
            lbl_0020df10_count_check,
            200,
            &|c, seed| {
                machine(c, seed, 0x1542_0df1, 0);
                let obj = DATA + 0x20_3800;
                let count: u32 = if seed % 3 == 0 { 0 } else { 1 + seed % 5 };
                c.bus.write32(obj + 0x9c, count);
                c.g[A0][0] = sx(obj);
                c.g[A2][0] = sx(seed % 12);
            },
        );
    }

    #[test]
    fn ov_list_clear_matches() {
        check(
            ee::s05::f_00212788__generated,
            ov_list_clear,
            300,
            &|c, seed| {
                machine(c, seed, 0x1542_2788, 5);
                fakes::hook(
                    addr_of_mut!(ee::s10::OVERRIDE_00317E50),
                    oc1542_fake_delete,
                    fakes::prev_cell(oc1542_fake_delete),
                );
                c.bus.write32(GATE_DELETE, GATE_MAGIC);
                let obj = DATA + 0x20_2000;
                let head = DATA + 0x20_2100;
                let count = seed % 5;
                let mut at = head;
                for i in 0..count {
                    let node = DATA + 0x20_2200 + 0x20 * i;
                    c.bus.write32(at, node);
                    at = node;
                }
                c.bus.write32(at, head);
                c.bus.write32(obj + 0x4, head);
                c.g[A0][0] = sx(obj);
            },
        );
    }

    #[test]
    fn ov_state_overlay_ctor_matches() {
        check(
            ee::s05::f_0021c640__generated,
            ov_state_overlay_ctor,
            200,
            &|c, seed| {
                machine(c, seed, 0x1542_c640, 1);
                fakes::hook(
                    addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
                    oc1542_fake_ui_state_ctor,
                    fakes::prev_cell(oc1542_fake_ui_state_ctor),
                );
                c.bus.write32(GATE_CTOR, GATE_MAGIC);
                c.g[A0][0] = sx(DATA + 0x20_3c00 + 0x10 * (seed % 4));
            },
        );
    }

    #[test]
    fn ov_state_view_popup_ctor_2_matches() {
        check(
            ee::s05::f_0021cb08__generated,
            ov_state_view_popup_ctor_2,
            200,
            &|c, seed| {
                machine(c, seed, 0x1542_cb08, 1);
                fakes::hook(
                    addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
                    oc1542_fake_ui_state_ctor,
                    fakes::prev_cell(oc1542_fake_ui_state_ctor),
                );
                c.bus.write32(GATE_CTOR, GATE_MAGIC);
                c.g[A0][0] = sx(DATA + 0x20_3c40 + 0x10 * (seed % 4));
            },
        );
    }
}
