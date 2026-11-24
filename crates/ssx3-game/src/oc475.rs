//! Staging module oc475 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc475`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0024_7e60,
    render_toggles_menu_ctor,
    "RenderTogglesMenu_ctor",
)];

/// Offsets of the title (null) item and the spacer item in the menu.
const RTM_TITLE: u32 = 0x130;
const RTM_SPACE: u32 = 0x14c;

/// The argument of a toggle constructor call.
#[derive(Clone, Copy)]
enum RtmCall {
    /// `cIntMenuItem(item, index, &var, 0, max, name)` (0x2cd8f8).
    Int { max: u32, name: u32 },
    /// `cFloatMenuItem_ctor(item, index, &var, min, max, name)` (0x2cdce8):
    /// the bounds in f12/f13 (`Some` bit patterns of two floats), or the
    /// saved f21/f20 (`None`, -10.0 and 10.0), and the name in a3.
    Float {
        bounds: Option<(u32, u32)>,
        name: u32,
    },
}

/// One toggle item, in the order the constructor builds them. `stash` lists
/// the item addresses written to frame slots just before the call, as
/// `(slot, item offset)` with the slot at `sp + 4 * slot`.
#[derive(Clone, Copy)]
struct RtmToggle {
    item: u32,
    index: u32,
    var: u32,
    ret_addr: u32,
    stash: &'static [(u32, u32)],
    call: RtmCall,
}

/// The 47 toggle constructor calls (calls 3..=49 of the generated body).
/// `var` is the gp offset of the toggle's variable.
static RTM_TOGGLES: [RtmToggle; 47] = [
    RtmToggle { item: 0x164, index: 0, var: 0x13d0, ret_addr: 0x0024_7f3c, stash: &[(3, 0x194), (4, 0x1c4)], call: RtmCall::Int { max: 0x3c, name: 0x0047_dc10 } },
    RtmToggle { item: 0x194, index: 1, var: 0x13d4, ret_addr: 0x0024_7f6c, stash: &[(5, 0x1f4), (6, 0x224)], call: RtmCall::Int { max: 1, name: 0x0047_dc30 } },
    RtmToggle { item: 0x1c4, index: 2, var: 0x13d8, ret_addr: 0x0024_7f9c, stash: &[(7, 0x254), (8, 0x284)], call: RtmCall::Int { max: 1, name: 0x0047_dc48 } },
    RtmToggle { item: 0x1f4, index: 3, var: 0x13dc, ret_addr: 0x0024_7fcc, stash: &[(9, 0x2b4), (10, 0x2e4)], call: RtmCall::Int { max: 1, name: 0x0047_dc58 } },
    RtmToggle { item: 0x224, index: 4, var: 0x13e0, ret_addr: 0x0024_7ffc, stash: &[(11, 0x314), (12, 0x344)], call: RtmCall::Int { max: 1, name: 0x0047_dc70 } },
    RtmToggle { item: 0x254, index: 5, var: 0x13e4, ret_addr: 0x0024_802c, stash: &[(13, 0x374), (14, 0x3a4)], call: RtmCall::Int { max: 1, name: 0x0047_dc88 } },
    RtmToggle { item: 0x284, index: 6, var: 0x13e8, ret_addr: 0x0024_805c, stash: &[(15, 0x3d4), (16, 0x404)], call: RtmCall::Int { max: 1, name: 0x0047_dc98 } },
    RtmToggle { item: 0x2b4, index: 7, var: 0x13ec, ret_addr: 0x0024_808c, stash: &[(17, 0x434), (18, 0x464)], call: RtmCall::Int { max: 1, name: 0x0047_dcb8 } },
    RtmToggle { item: 0x2e4, index: 8, var: 0x13f0, ret_addr: 0x0024_80bc, stash: &[(19, 0x494), (20, 0x4c4)], call: RtmCall::Int { max: 1, name: 0x0047_dcd0 } },
    RtmToggle { item: 0x314, index: 9, var: 0x13f4, ret_addr: 0x0024_80ec, stash: &[(21, 0x4f4), (22, 0x524)], call: RtmCall::Int { max: 1, name: 0x0047_dce8 } },
    RtmToggle { item: 0x344, index: 10, var: 0x13f8, ret_addr: 0x0024_811c, stash: &[(23, 0x554), (24, 0x584)], call: RtmCall::Int { max: 3, name: 0x0047_dd00 } },
    RtmToggle { item: 0x374, index: 11, var: 0x13fc, ret_addr: 0x0024_814c, stash: &[(25, 0x5b4), (26, 0x5e4)], call: RtmCall::Int { max: 1, name: 0x0047_dd18 } },
    RtmToggle { item: 0x3a4, index: 12, var: 0x1400, ret_addr: 0x0024_817c, stash: &[(27, 0x614), (28, 0x644)], call: RtmCall::Int { max: 1, name: 0x0047_dd38 } },
    RtmToggle { item: 0x3d4, index: 13, var: 0x1404, ret_addr: 0x0024_81ac, stash: &[(29, 0x674), (30, 0x6a4)], call: RtmCall::Int { max: 1, name: 0x0047_dd48 } },
    RtmToggle { item: 0x404, index: 14, var: 0x1408, ret_addr: 0x0024_81dc, stash: &[(31, 0x6d4), (32, 0x704)], call: RtmCall::Int { max: 1, name: 0x0047_dd60 } },
    RtmToggle { item: 0x434, index: 15, var: 0x140c, ret_addr: 0x0024_820c, stash: &[(33, 0x734), (34, 0x764)], call: RtmCall::Int { max: 1, name: 0x0047_dd78 } },
    RtmToggle { item: 0x464, index: 16, var: 0x1410, ret_addr: 0x0024_823c, stash: &[(35, 0x794), (36, 0x7c4)], call: RtmCall::Int { max: 1, name: 0x0047_dd98 } },
    RtmToggle { item: 0x494, index: 17, var: 0x1414, ret_addr: 0x0024_826c, stash: &[(37, 0x7f8), (38, 0x828)], call: RtmCall::Int { max: 1, name: 0x0047_ddb8 } },
    RtmToggle { item: 0x4c4, index: 18, var: 0x1418, ret_addr: 0x0024_8294, stash: &[(39, 0x858)], call: RtmCall::Int { max: 1, name: 0x0047_ddc8 } },
    RtmToggle { item: 0x4f4, index: 19, var: 0x141c, ret_addr: 0x0024_82b4, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_dde0 } },
    RtmToggle { item: 0x524, index: 20, var: 0x1420, ret_addr: 0x0024_82dc, stash: &[(40, 0x8f0)], call: RtmCall::Int { max: 1, name: 0x0047_ddf8 } },
    RtmToggle { item: 0x554, index: 21, var: 0x1424, ret_addr: 0x0024_82fc, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_de10 } },
    RtmToggle { item: 0x584, index: 22, var: 0x1428, ret_addr: 0x0024_831c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_de20 } },
    RtmToggle { item: 0x5b4, index: 23, var: 0x142c, ret_addr: 0x0024_833c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_de38 } },
    RtmToggle { item: 0x5e4, index: 24, var: 0x1430, ret_addr: 0x0024_835c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_de50 } },
    RtmToggle { item: 0x614, index: 25, var: 0x1434, ret_addr: 0x0024_837c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_de70 } },
    RtmToggle { item: 0x644, index: 26, var: 0x1438, ret_addr: 0x0024_839c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_de88 } },
    RtmToggle { item: 0x674, index: 27, var: 0x143c, ret_addr: 0x0024_83bc, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_dea0 } },
    RtmToggle { item: 0x6a4, index: 28, var: 0x1440, ret_addr: 0x0024_83dc, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_deb0 } },
    RtmToggle { item: 0x6d4, index: 29, var: 0x1444, ret_addr: 0x0024_83fc, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_ded0 } },
    RtmToggle { item: 0x704, index: 30, var: 0x1448, ret_addr: 0x0024_841c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_dee8 } },
    RtmToggle { item: 0x734, index: 31, var: 0x144c, ret_addr: 0x0024_843c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_df00 } },
    RtmToggle { item: 0x764, index: 32, var: 0x1450, ret_addr: 0x0024_845c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_df28 } },
    RtmToggle { item: 0x794, index: 33, var: 0x1454, ret_addr: 0x0024_847c, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_df40 } },
    RtmToggle { item: 0x7c4, index: 34, var: 0x1458, ret_addr: 0x0024_84a4, stash: &[], call: RtmCall::Float { bounds: Some((0x3f80_0000, 0x44fa_0000)), name: 0x0047_df50 } },
    RtmToggle { item: 0x7f8, index: 35, var: 0x145c, ret_addr: 0x0024_84c4, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_df68 } },
    RtmToggle { item: 0x828, index: 36, var: 0x1460, ret_addr: 0x0024_84e4, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_df80 } },
    RtmToggle { item: 0x858, index: 37, var: 0x1464, ret_addr: 0x0024_8504, stash: &[], call: RtmCall::Int { max: 0x14, name: 0x0047_dfa0 } },
    RtmToggle { item: 0x888, index: 38, var: 0x1468, ret_addr: 0x0024_8524, stash: &[], call: RtmCall::Float { bounds: None, name: 0x004a_2cd0 } },
    RtmToggle { item: 0x8bc, index: 39, var: 0x146c, ret_addr: 0x0024_8544, stash: &[], call: RtmCall::Float { bounds: None, name: 0x004a_2cd8 } },
    RtmToggle { item: 0x8f0, index: 40, var: 0x1470, ret_addr: 0x0024_8564, stash: &[], call: RtmCall::Float { bounds: None, name: 0x004a_2ce0 } },
    RtmToggle { item: 0x924, index: 41, var: 0x1474, ret_addr: 0x0024_8584, stash: &[], call: RtmCall::Float { bounds: None, name: 0x004a_2ce8 } },
    RtmToggle { item: 0x958, index: 42, var: 0x1478, ret_addr: 0x0024_85a4, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_dfb0 } },
    RtmToggle { item: 0x988, index: 43, var: 0x147c, ret_addr: 0x0024_85c4, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_dfc8 } },
    RtmToggle { item: 0x9b8, index: 44, var: 0x1480, ret_addr: 0x0024_85e4, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_dfe8 } },
    RtmToggle { item: 0x9e8, index: 45, var: 0x1484, ret_addr: 0x0024_8604, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_e000 } },
    RtmToggle { item: 0xa18, index: 46, var: 0x1488, ret_addr: 0x0024_8624, stash: &[], call: RtmCall::Int { max: 1, name: 0x0047_e020 } },
];

/// `this + off` as the 64-bit register value the generated code computes.
fn rtm_item(this: u64, off: u32) -> u64 {
    sx((this as u32).wrapping_add(off))
}

/// `V0 = this + item; sw V0, sp + 4 * slot`: an item address stashed in the frame.
fn rtm_stash(c: &mut Cpu, frame: Frame, slot: u32, item: u32) {
    c.g[V0][0] = rtm_item(c.g[S0][0], item);
    let addr = frame.at(c, slot * 4);
    let v = c.g[V0][0];
    c.sw(addr, v);
}

/// `cIntMenuItem(item, index, &var, 0, max, name)`.
fn rtm_int(c: &mut Cpu, t: &RtmToggle, max: u32, name: u32) -> Flow {
    c.g[A0][0] = rtm_item(c.g[S0][0], t.item);
    c.g[A1][0] = t.index as u64;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(t.var));
    c.g[A3][0] = 0;
    c.g[T0][0] = max as u64;
    c.g[T1][0] = sx(name);
    call_ptr(c, 0x002c_d8f8, t.ret_addr)
}

/// `cFloatMenuItem_ctor(item, index, &var, min, max, name)`: the bounds are
/// already in f12/f13; T0 and T1 keep their values, as in the generated body.
fn rtm_float(c: &mut Cpu, t: &RtmToggle, name: u32) -> Flow {
    c.g[A0][0] = rtm_item(c.g[S0][0], t.item);
    c.g[A1][0] = t.index as u64;
    c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(t.var));
    c.g[A3][0] = sx(name);
    call_ptr(c, 0x002c_dce8, t.ret_addr)
}

/// `MenuSysPage_InsertItem(this, item, -1)`.
fn rtm_insert(c: &mut Cpu, item: u32, ret_addr: u32) -> Flow {
    c.g[A1][0] = rtm_item(c.g[S0][0], item);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = u64::MAX;
    call_ptr(c, 0x002c_bcc8, ret_addr)
}

/// `RenderTogglesMenu_ctor(RenderTogglesMenu *this)` (0x247e60): build the
/// debug render-toggles menu in `this`. It constructs the page
/// (`MenuSysPage_ctor`), the title and spacer items, then one `MenuSysItem`
/// per toggle (int, or float with bounds), each toggle's variable at a gp
/// offset, and finally inserts the 49 items into the page in order. Returns
/// `this` in v0.
///
/// The generated body writes the item addresses into frame slots (`sp + 4n`)
/// ahead of the constructors that read them back, and keeps the rest in
/// s1..s7/fp; the port writes the same slots before the same calls, so an
/// unwinding callee leaves the same stack.
pub fn render_toggles_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 708;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S0, 0x140);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        frame.save(c, S3, 0x110);
        frame.save(c, S4, 0x100);
        c.g[S2][0] = rtm_item(c.g[S0][0], 0x888);
        frame.save(c, S5, 0xf0);
        c.g[S3][0] = rtm_item(c.g[S0][0], 0x8bc);
        frame.save(c, S6, 0xe0);
        c.g[S5][0] = rtm_item(c.g[S0][0], 0x924);
        frame.save(c, S7, 0xd0);
        c.g[S4][0] = rtm_item(c.g[S0][0], 0x958);
        frame.save(c, FP, 0xc0);
        c.g[S7][0] = rtm_item(c.g[S0][0], 0x988);
        frame.save_ra(c, 0xb0);
        c.g[FP][0] = rtm_item(c.g[S0][0], 0x9b8);
        frame.save_fpr(c, 21, 0x158);
        c.g[S1][0] = rtm_item(c.g[S0][0], 0x9e8);
        frame.save_fpr(c, 20, 0x150);
        c.g[S6][0] = rtm_item(c.g[S0][0], 0xa18);

        // The page: MenuSysPage_ctor(this).
        rtm_stash(c, frame, 0, RTM_TITLE);
        call_ptr(c, 0x002c_aa58, 0x0024_7ec4)?;

        // Title item (null item, name "Render Toggles Menu") and vtable.
        c.g[AT][0] = 0xffff_ffff_c120_0000;
        c.mtc1(1, 21); // f21 = -10.0
        rtm_stash(c, frame, 1, RTM_SPACE);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x2408));
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 20); // f20 = 10.0
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_sub(0x320)); // cRenderTogglesMenu_vtbl
        let vtbl_addr = (c.g[S0][0] as u32).wrapping_add(0x12c);
        let vtbl = c.g[V0][0];
        c.sw(vtbl_addr, vtbl);
        c.g[A0][0] = rtm_item(c.g[S0][0], RTM_TITLE);
        call_ptr(c, 0x002c_d2a0, 0x0024_7ef8)?;

        // Spacer item.
        c.g[A0][0] = rtm_item(c.g[S0][0], RTM_SPACE);
        rtm_stash(c, frame, 2, 0x164);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002c_d350, 0x0024_7f0c)?;

        // The toggles, each with the frame slots the generated body fills
        // before its call.
        for t in RTM_TOGGLES.iter() {
            for &(slot, item) in t.stash {
                rtm_stash(c, frame, slot, item);
            }
            match t.call {
                RtmCall::Int { max, name } => {
                    rtm_int(c, t, max, name)?;
                }
                RtmCall::Float { bounds, name } => {
                    match bounds {
                        Some((lo, hi)) => {
                            c.g[AT][0] = lo as u64;
                            c.mtc1(1, 12);
                            c.g[AT][0] = hi as u64;
                            c.mtc1(1, 13);
                        }
                        None => {
                            c.mov_s(12, 21);
                            c.mov_s(13, 20);
                        }
                    }
                    rtm_float(c, t, name)?;
                }
            }
        }

        // Insert the 49 items into the page, in order.
        let order = core::iter::once(RTM_TITLE)
            .chain(core::iter::once(RTM_SPACE))
            .chain(RTM_TOGGLES.iter().map(|t| t.item));
        for (i, item) in order.enumerate() {
            rtm_insert(c, item, 0x0024_8634 + 0x10 * i as u32)?;
        }

        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore(c, S6, 0xe0);
        frame.restore(c, S7, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.restore_fpr(c, 21, 0x158);
        frame.restore_fpr(c, 20, 0x150);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use ssx3_ee as ee;

    #[test]
    fn render_toggles_menu_ctor_matches() {
        check(
            ee::s06::f_00247e60__generated,
            render_toggles_menu_ctor,
            100,
            &|c, seed| {
                machine(c, seed, 0x7e60, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }
}
