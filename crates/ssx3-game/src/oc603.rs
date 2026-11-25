//! Staging module oc603 (wave 233, s2): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc603`.

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
    (0x0034_7d90, flex_rail_node_ctor, "FlexRail_ctor"),
    (0x0034_7ea8, flex_rail_node_ctor_2, "FlexRail_ctor_2"),
    (0x0034_80c8, flex_rail_render, "FlexRail_Render"),
    (
        0x0034_8b90,
        flex_rail_space_node_register,
        "FlexRail_RegisterSpaceNode",
    ),
];

/// The base constructors the rail node constructors call first.
const FR_WORLD_OBJ_CTOR: Callee = Callee {
    addr: 0x0034_fb00,
    f: ee::s11::f_0034fb00,
};
const FR_WORLD_OBJ_CTOR_2: Callee = Callee {
    addr: 0x0034_fb90,
    f: ee::s11::f_0034fb90,
};

/// `base + off` as a 32-bit guest address.
#[inline(always)]
fn frail_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `gp - off` as a 32-bit guest address (the game's globals sit below gp).
#[inline(always)]
fn frail_gp_at(gp: u64, off: u32) -> u32 {
    (gp as u32).wrapping_sub(off)
}

/// `cFlexRailNode::cFlexRailNode(self, type, nodeType, desc)` (0x347d90):
/// base-object constructor, vtable, then the node count (the descriptor's
/// eight instance ids that are not -1), the flags, the segment array
/// (`NumNodes - 1` entries of 0x40 bytes, stored at +0x50) and the tube.
pub fn flex_rail_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S1, 0x20);
        c.g[A3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x1;
        FR_WORLD_OBJ_CTOR.call(c, 0x0034_7dbc)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = sx(frail_at(c.g[V1][0], 0xd0));
        c.g[A0][0] |= 0xffff;
        c.sw(frail_at(c.g[S1][0], 0xc), c.g[V1][0]);
        c.g[A1][0] = 0x7;
        c.g[V1][0] = sx(frail_at(c.g[S0][0], 0x4));
        c.g[V0][0] = c.lw(frail_at(c.g[S0][0], 0x44));
        c.sw(frail_at(c.g[S1][0], 0x20), 0);
        c.sw(frail_at(c.g[S1][0], 0x24), c.g[V0][0]);
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[V0][0] == c.g[A0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.lw(frail_at(c.g[S1][0], 0x20));
                c.g[V0][0] = sx(frail_at(c.g[V0][0], 0x1));
                c.sw(frail_at(c.g[S1][0], 0x20), c.g[V0][0]);
            }
            c.cycles += 2;
            let more = (c.g[A1][0] as i64) >= 0;
            c.g[V1][0] = sx(frail_at(c.g[V1][0], 0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_7de8);
            }
        }
        c.cycles += 14;
        c.g[S0][0] = c.lw(frail_at(c.g[S1][0], 0x20));
        c.g[A1][0] = 0x49_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1838));
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
        c.g[S2][0] = sx(frail_at(c.g[S1][0], 0x50));
        call_ptr(c, 0x0031_7e30, 0x0034_7e2c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = u64::MAX;
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[A1][0] = 0x4a_0000;
        if !t {
            loop {
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[A1][0] = 0x4a_0000;
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_7e40);
                }
            }
        }
        c.cycles += 19;
        c.sw(c.g[S2][0] as u32, c.g[V1][0]);
        c.g[A1][0] = sx(frail_at(c.g[A1][0], 0x4008));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0xc;
        call_ptr(c, 0x0031_7d70, 0x0034_7e74)?;
        c.g[A1][0] = c.lw(frail_at(c.g[S1][0], 0x20));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x9));
        call_ptr(c, 0x0037_2520, 0x0034_7e88)?;
        c.sw(frail_at(c.g[S1][0], 0x58), c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFlexRailNode::cFlexRailNode` second form (0x347ea8): the same node
/// setup, but the base constructor is `WorldObj_ctor_2`, and the node count
/// comes from a virtual call on the descriptor (`desc->vt[+0x14]`, this at
/// `desc + delta`), which writes the count at `self+0x20`.
pub fn flex_rail_node_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(frail_at(c.g[S1][0], 0x50));
        FR_WORLD_OBJ_CTOR_2.call(c, 0x0034_7ecc)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = sx(frail_at(c.g[S1][0], 0x20));
        c.g[V1][0] = sx(frail_at(c.g[V1][0], 0xd0));
        c.g[A2][0] = 0x30;
        c.sw(frail_at(c.g[S1][0], 0xc), c.g[V1][0]);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lh(frail_at(c.g[V0][0], 0x10));
        c.g[V1][0] = c.lw(frail_at(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(frail_at(c.g[S0][0], c.g[A0][0] as u32));
        call_ptr(c, target, 0x0034_7ef4)?;
        c.g[S0][0] = c.lw(frail_at(c.g[S1][0], 0x20));
        c.g[A1][0] = 0x49_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1838));
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
        call_ptr(c, 0x0031_7e30, 0x0034_7f14)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
        c.g[V0][0] = u64::MAX;
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[A1][0] = 0x4a_0000;
        if !t {
            loop {
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                let t = c.g[S0][0] != c.g[V0][0];
                c.g[A1][0] = 0x4a_0000;
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_7f28);
                }
            }
        }
        c.cycles += 19;
        c.sw(c.g[S2][0] as u32, c.g[V1][0]);
        c.g[A1][0] = sx(frail_at(c.g[A1][0], 0x4008));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0xc;
        call_ptr(c, 0x0031_7d70, 0x0034_7f5c)?;
        c.g[A1][0] = c.lw(frail_at(c.g[S1][0], 0x20));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x9));
        call_ptr(c, 0x0037_2520, 0x0034_7f70)?;
        c.sw(frail_at(c.g[S1][0], 0x58), c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFlexRailNode_render(self)` (0x3480c8): cull the node's bounds
/// against the current view (`vt[93]`, with the matrix from `vt[43]`); if
/// visible, make sure the shared colour table is set up once, switch the
/// current draw state to texture 0 (-1) with the group bits cleared to 4, and
/// walk the `NumNodes - 1` segments (an empty inner loop of 8 steps each).
pub fn flex_rail_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S4, 0x60);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        frame.save(c, S1, 0x90);
        c.g[S5][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        c.g[A0][0] = c.g[SP][0];
        c.g[S4][0] = c.lw(frail_at(c.g[GP][0], 0x2a90));
        c.g[S3][0] = sx(frail_at(c.g[S5][0], 0x30));
        frame.save(c, S0, 0xa0);
        c.g[S2][0] = sx(frail_at(c.g[S5][0], 0x40));
        c.g[S0][0] = c.lw(frail_at(c.g[S4][0], 0x10d8));
        c.g[S1][0] = c.lh(frail_at(c.g[S0][0], 0x2e8));
        c.g[A1][0] = c.lh(frail_at(c.g[S0][0], 0x158));
        c.g[V0][0] = c.lw(frail_at(c.g[S0][0], 0x15c));
        c.g[S1][0] = sx(frail_at(c.g[S4][0], c.g[S1][0] as u32));
        let get_mat = c.g[V0][0] as u32;
        c.g[A1][0] = sx(frail_at(c.g[S4][0], c.g[A1][0] as u32));
        call_ptr(c, get_mat, 0x0034_8118)?;
        c.g[V0][0] = c.lw(frail_at(c.g[S0][0], 0x2ec));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S2][0];
        let cull = c.g[V0][0] as u32;
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, cull, 0x0034_8130)?;
        c.g[A0][0] = 0x1;
        let culled = c.g[V0][0] == c.g[A0][0];
        c.g[V0][0] = c.lw(frail_at(c.g[GP][0], 0x2890));
        'done: {
            if culled {
                break 'done;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 47;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0);
                c.g[V1][0] = 0x50_0000;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x940));
                c.lwc1(3, frail_gp_at(c.g[GP][0], 0x2b34));
                c.lwc1(1, frail_gp_at(c.g[GP][0], 0x2b30));
                c.lwc1(2, frail_gp_at(c.g[GP][0], 0x2b2c));
                c.swc1(0, frail_at(c.g[V0][0], 0x7c));
                c.lwc1(4, frail_gp_at(c.g[GP][0], 0x2b28));
                c.lwc1(5, frail_gp_at(c.g[GP][0], 0x2b24));
                c.lwc1(6, frail_gp_at(c.g[GP][0], 0x2b20));
                c.lwc1(7, frail_gp_at(c.g[GP][0], 0x2b1c));
                c.lwc1(8, frail_gp_at(c.g[GP][0], 0x2b18));
                c.lwc1(9, frail_gp_at(c.g[GP][0], 0x2b14));
                c.sw((c.g[V1][0] as u32).wrapping_sub(0x940), 0);
                c.swc1(4, frail_at(c.g[V0][0], 0x10));
                c.swc1(5, frail_at(c.g[V0][0], 0x14));
                c.swc1(6, frail_at(c.g[V0][0], 0x18));
                c.swc1(7, frail_at(c.g[V0][0], 0x20));
                c.swc1(8, frail_at(c.g[V0][0], 0x24));
                c.swc1(9, frail_at(c.g[V0][0], 0x28));
                c.swc1(3, frail_at(c.g[V0][0], 0x70));
                c.swc1(1, frail_at(c.g[V0][0], 0x74));
                c.swc1(2, frail_at(c.g[V0][0], 0x78));
                c.sw(frail_at(c.g[GP][0], 0x2890), c.g[A0][0]);
                c.sw(frail_at(c.g[V0][0], 0x4), 0);
                c.sw(frail_at(c.g[V0][0], 0x8), 0);
                c.swc1(0, frail_at(c.g[V0][0], 0xc));
                c.swc1(0, frail_at(c.g[V0][0], 0x1c));
                c.swc1(0, frail_at(c.g[V0][0], 0x2c));
                c.swc1(3, frail_at(c.g[V0][0], 0x30));
                c.swc1(1, frail_at(c.g[V0][0], 0x34));
                c.swc1(2, frail_at(c.g[V0][0], 0x38));
                c.swc1(0, frail_at(c.g[V0][0], 0x3c));
                c.swc1(3, frail_at(c.g[V0][0], 0x40));
                c.swc1(1, frail_at(c.g[V0][0], 0x44));
                c.swc1(2, frail_at(c.g[V0][0], 0x48));
                c.swc1(0, frail_at(c.g[V0][0], 0x4c));
                c.swc1(3, frail_at(c.g[V0][0], 0x50));
                c.swc1(1, frail_at(c.g[V0][0], 0x54));
                c.swc1(2, frail_at(c.g[V0][0], 0x58));
                c.swc1(0, frail_at(c.g[V0][0], 0x5c));
                c.swc1(3, frail_at(c.g[V0][0], 0x60));
                c.swc1(1, frail_at(c.g[V0][0], 0x64));
                c.swc1(2, frail_at(c.g[V0][0], 0x68));
                c.swc1(0, frail_at(c.g[V0][0], 0x6c));
            }
            c.g[V1][0] = c.lw(frail_at(c.g[S4][0], 0xe84));
            c.cycles += 12;
            c.g[V0][0] = u64::MAX;
            c.g[A1][0] = 0xffff_ffff_ffff_ff83;
            c.sh(frail_at(c.g[V1][0], 0x10), c.g[V0][0]);
            c.g[A0][0] = c.lw(frail_at(c.g[S4][0], 0xe84));
            c.g[V0][0] = c.lw(frail_at(c.g[A0][0], 0x4));
            c.g[V0][0] &= c.g[A1][0];
            c.g[V0][0] |= 0x4;
            c.sw(frail_at(c.g[A0][0], 0x4), c.g[V0][0]);
            c.g[V1][0] = c.lw(frail_at(c.g[S5][0], 0x20));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A2][0] = 0;
            if empty {
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            'outer: loop {
                c.cycles += 2;
                c.g[A1][0] = 0x7;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                'inner: loop {
                    c.cycles += 7;
                    if (c.g[A1][0] as i64) >= 0 {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0034_8240);
                        }
                        continue 'inner;
                    }
                    c.cycles += 4;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let again = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    if !again {
                        break 'done;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0034_8238);
                    }
                    continue 'outer;
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void FlexRail_RegisterSpaceNode(self)` (0x348b90): allocate the space
/// node (`0x10` bytes, `+0xc` = self, `+8` = type 3), store it at `self+0x54`
/// and add it to the nash octant the bounding box falls in (x/y/z sign picks
/// one of eight octants of the cell at offset 0x14 each).
pub fn flex_rail_space_node_register(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 45;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x17f8));
        c.g[A0][0] = 0x10;
        call_ptr(c, 0x0031_7d70, 0x0034_8bc0)?;
        c.g[V1][0] = 0x3;
        c.sw(frail_at(c.g[V0][0], 0xc), c.g[S0][0]);
        c.sw(frail_at(c.g[V0][0], 0x8), c.g[V1][0]);
        c.sw(frail_at(c.g[S0][0], 0x54), c.g[V0][0]);
        call_ptr(c, 0x002d_1be0, 0x0034_8bd4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = sx(frail_at(c.g[S0][0], 0x30));
        c.g[S2][0] = c.lw(frail_at(c.g[S0][0], 0x54));
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8f28, 0x0034_8be8)?;
        c.g[V0][0] = c.lw(frail_at(c.g[SP][0], 0x4));
        c.g[A3][0] = u64::MAX;
        c.g[V1][0] = sx(frail_at(c.g[S1][0], 0x50));
        c.g[A1][0] = c.lw(frail_at(c.g[SP][0], 0x8));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A2][0] = c.lw(frail_at(c.g[SP][0], 0xc));
        if c.g[V0][0] != 0 {
            c.g[V1][0] = c.g[S1][0];
        }
        c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.g[A0][0] = sx(frail_at(c.g[V1][0], 0x28));
        c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
        if c.g[A1][0] != 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        c.g[A2][0] = c.g[S2][0];
        c.g[V0][0] = sx(frail_at(c.g[A0][0], 0x14));
        c.g[A1][0] = 0x2;
        if c.g[A3][0] == 0 {
            c.g[A0][0] = c.g[V0][0];
        }
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x0032_8c20, 0x0034_8c2c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word: the fakes below answer only while it holds `OC603_MAGIC`
    /// (other tests run through them with the gate off).
    const OC603_CTRL: u32 = DATA + 0x20_0000;
    const OC603_MAGIC: u32 = 0x0603_0603;
    /// The key `findkey` writes (x, y, z) for the space-node test.
    const OC603_KEY: u32 = DATA + 0x20_0010;
    /// The node (`self`), its descriptor and the descriptor's vtable.
    const OC603_OBJ: u32 = DATA + 0x20_1000;
    const OC603_DESC: u32 = DATA + 0x20_2000;
    const OC603_DESC_VT: u32 = DATA + 0x20_3000;
    /// The render context (`gp+0x2a90`), its vtable and the draw state.
    const OC603_CTX: u32 = DATA + 0x20_8000;
    const OC603_CTX_VT: u32 = DATA + 0x20_5000;
    const OC603_DRAW: u32 = DATA + 0x20_6000;

    /// A gated fake: records the call to `$target` (the callee's entry) while
    /// the gate is up, else runs the body it replaced.
    macro_rules! oc603_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC603_CTRL) != OC603_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                let ret = c.g[RA][0] as u32;
                fake(c, $target, ret);
            }
        };
    }

    oc603_fake!(
        oc603_fake_world_obj,
        OC603_PREV_WORLD_OBJ,
        0x0034_fb00,
        ee::s11::f_0034fb00__generated
    );
    oc603_fake!(
        oc603_fake_world_obj_2,
        OC603_PREV_WORLD_OBJ_2,
        0x0034_fb90,
        ee::s11::f_0034fb90__generated
    );
    oc603_fake!(
        oc603_fake_alloc,
        OC603_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    oc603_fake!(
        oc603_fake_new_array,
        OC603_PREV_NEW_ARRAY,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );
    oc603_fake!(
        oc603_fake_tube,
        OC603_PREV_TUBE,
        0x0037_2520,
        ee::s12::f_00372520__generated
    );
    oc603_fake!(
        oc603_fake_get_nash,
        OC603_PREV_GET_NASH,
        0x002d_1be0,
        ee::s09::f_002d1be0__generated
    );
    oc603_fake!(
        oc603_fake_add_node,
        OC603_PREV_ADD_NODE,
        0x0032_8c20,
        ee::s10::f_00328c20__generated
    );

    static OC603_PREV_FIND_KEY: AtomicUsize = AtomicUsize::new(0);

    /// `nash_Nash_findkey(key, box)`: writes the key's x, y, z from the test's
    /// words, then records the call.
    fn oc603_fake_find_key(c: &mut Cpu) {
        if c.bus.read32(OC603_CTRL) != OC603_MAGIC {
            return fall_back(c, &OC603_PREV_FIND_KEY, ee::s10::f_00328f28__generated);
        }
        let key = c.g[A0][0] as u32;
        for i in 0..3u32 {
            let v = c.bus.read32(OC603_KEY + 4 * i);
            c.bus.write32(key + 4 + 4 * i, v);
        }
        let ret = c.g[RA][0] as u32;
        fake(c, 0x0032_8f28, ret);
    }

    fn oc603_install() {
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FB00),
            oc603_fake_world_obj,
            &OC603_PREV_WORLD_OBJ,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034FB90),
            oc603_fake_world_obj_2,
            &OC603_PREV_WORLD_OBJ_2,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc603_fake_alloc,
            &OC603_PREV_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc603_fake_new_array,
            &OC603_PREV_NEW_ARRAY,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00372520),
            oc603_fake_tube,
            &OC603_PREV_TUBE,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D1BE0),
            oc603_fake_get_nash,
            &OC603_PREV_GET_NASH,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc603_fake_find_key,
            &OC603_PREV_FIND_KEY,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328C20),
            oc603_fake_add_node,
            &OC603_PREV_ADD_NODE,
        );
    }

    #[test]
    fn flex_rail_node_ctor_matches() {
        oc603_install();
        covered(ee::s11::f_00347d90__generated, flex_rail_node_ctor, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_7d90, 4);
            c.pc = 0x0034_7d90;
            c.bus.write32(OC603_CTRL, OC603_MAGIC);
            // Instance ids 1..8 are -1 or random; id 0 never is, so the
            // node count is at least one.
            c.bus.write32(OC603_DESC + 4, r.next_u32() & 0x7fff_ffff);
            for i in 1..8u32 {
                let v = if (seed >> i) & 1 == 1 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(OC603_DESC + 4 + 4 * i, v);
            }
            c.bus.write32(OC603_DESC + 0x44, r.next_u32());
            c.g[A0][0] = sx(OC603_OBJ);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(r.next_u32());
            c.g[A3][0] = sx(OC603_DESC);
        });
    }

    #[test]
    fn flex_rail_node_ctor_2_matches() {
        oc603_install();
        covered(
            ee::s11::f_00347ea8__generated,
            flex_rail_node_ctor_2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_7ea8, 4);
                c.pc = 0x0034_7ea8;
                c.bus.write32(OC603_CTRL, OC603_MAGIC);
                // The descriptor's vtable: this + 0x30 is the method's object,
                // its target is a fake (a miss hook) that writes nothing, so
                // the node count is preset.
                c.bus.write32(OC603_DESC, OC603_DESC_VT);
                c.bus.write16(OC603_DESC_VT + 0x10, 0x30);
                c.bus.write32(OC603_DESC_VT + 0x14, FAKE + 0x40);
                c.bus.write32(OC603_OBJ + 0x20, 1 + seed % 6);
                c.g[A0][0] = sx(OC603_OBJ);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(OC603_DESC);
                c.g[A3][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn flex_rail_render_matches() {
        oc603_install();
        covered(ee::s11::f_003480c8__generated, flex_rail_render, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0034_80c8, 2);
            c.pc = 0x0034_80c8;
            c.bus.write32(OC603_CTRL, OC603_MAGIC);
            // The render context and its vtable: GetMat is slot 43, Cull
            // slot 93 (8-byte entries: a 16-bit this-delta, then the
            // target). Both targets are fakes.
            c.bus.write32(GAME_GP + 0x2a90, OC603_CTX);
            c.bus.write32(OC603_CTX + 0x10d8, OC603_CTX_VT);
            c.bus.write32(OC603_CTX + 0xe84, OC603_DRAW);
            c.bus.write16(OC603_CTX_VT + 0x158, 0x40);
            c.bus.write32(OC603_CTX_VT + 0x15c, FAKE + 0x10);
            c.bus.write16(OC603_CTX_VT + 0x2e8, 0x80);
            c.bus.write32(OC603_CTX_VT + 0x2ec, FAKE + 0x20);
            c.bus.write32(OC603_DRAW + 4, r.next_u32());
            c.bus.write32(OC603_DRAW + 0x10, r.next_u32());
            for off in [
                0x2b34u32, 0x2b30, 0x2b2c, 0x2b28, 0x2b24, 0x2b20, 0x2b1c, 0x2b18, 0x2b14,
            ] {
                c.bus.write32(GAME_GP - off, r.next_u32());
            }
            // The colour-table flag: clear on odd seeds (the table is
            // built), set on even ones.
            c.bus.write32(GAME_GP + 0x2890, 1 - seed % 2);
            // Cull is call 1: force "culled" on every fourth seed.
            if seed % 4 == 3 {
                c.bus.write32(LOG_RESULTS + 4, 1);
            }
            fill(c, &mut r, OC603_OBJ + 0x30, 6);
            c.bus.write32(OC603_OBJ + 0x20, 1 + seed % 4);
            c.g[A0][0] = sx(OC603_OBJ);
        });
    }

    #[test]
    fn flex_rail_space_node_register_matches() {
        oc603_install();
        covered(
            ee::s11::f_00348b90__generated,
            flex_rail_space_node_register,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0034_8b90, 4);
                c.pc = 0x0034_8b90;
                c.bus.write32(OC603_CTRL, OC603_MAGIC);
                // The octant is picked by the key's signs: each of x, y, z is
                // negative on some seeds.
                for k in 0..3u32 {
                    let v = r.next_u32() & 0xff;
                    let neg = (seed >> k) & 1 == 1;
                    c.bus
                        .write32(OC603_KEY + 4 * k, if neg { 0xffff_ff00 | v } else { v });
                }
                fill(c, &mut r, OC603_OBJ + 0x30, 6);
                c.g[A0][0] = sx(OC603_OBJ);
            },
        );
    }
}
