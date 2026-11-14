//! Staging module oc364 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc364`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_b498,
    cps2_chunk_resolve_release,
    "cPS2ChunkResolve_Release",
)];

// ---------------------------------------------------------------------------
// cPS2ChunkResolve_Release (0x3ab498)

/// The world view's message handler (`func_003AB498` in
/// `world/worldview.cpp`): `self` (a0, s4), message (a1), parameter (a2, s0),
/// data (a3, s3). Layout constants.
pub(crate) mod ccr_release {
    pub const ENTRY: u32 = 0x003a_b498;
    pub const END: u32 = 0x003a_be40;
    /// The message jump table (.rodata, 23 entries).
    pub const TABLE: u32 = 0x0049_5010;
    pub const MESSAGES: u64 = 0x17;
    /// `self` fields: vtable, the spatial hash (world cells), a base index.
    pub const SELF_VT: u32 = 0x8;
    pub const SELF_HASH: u32 = 0xc;
    pub const SELF_BASE: u32 = 0x10;
    /// Spatial hash: update counter; eight roots of 0x14 bytes, picked by
    /// the key's signs (x: +0x50, y: +0x28, z: +0x14), node pointer at +0x10.
    pub const HASH_UPDATES: u32 = 0xa0;
    pub const ROOT_NODE: u32 = 0x10;
    /// Item fields: kind (+8); a list's count (+0x20) and first item (+0x24);
    /// an item's next (+0x64).
    pub const ITEM_KIND: u32 = 0x8;
    pub const LIST_COUNT: u32 = 0x20;
    pub const LIST_FIRST: u32 = 0x24;
    pub const ITEM_NEXT: u32 = 0x64;
    /// gp-relative globals.
    pub const GP_MISSION_MAN: u32 = 0x2a90;
    pub const GP_WP_FILES: u32 = 0x16c8;
    pub const GP_AMBIENT: u32 = 0x84c; // gp - 0x84c
    pub const GP_WORLD: u32 = 0x848; // gp - 0x848
    /// Callees.
    pub const KEY_FROM_BOX: u32 = 0x0032_8f28;
    pub const HASH_REMOVE: u32 = 0x0032_84b8;
    pub const DELETE: u32 = 0x0031_7e50;
}

/// Exit of the message switch: through 0x3abe10 (`v0 = data`) or straight
/// to the epilogue (v0 already set).
#[derive(Clone, Copy, PartialEq)]
enum CcrExit {
    ResultData,
    Epilogue,
}

/// `void* func_003AB498(char* self, unsigned msg, unsigned param, char* data)`
/// (0x3ab498, named `cPS2ChunkResolve_Release` here): the world view's message
/// handler. Messages 1/3/5/6/7/8 re-file items in the spatial hash (box from
/// the item, key, `SpatialHash_Remove`, delete the root node when it is
/// empty); 9/10 forward `param >> 8` to the mission manager's vtable slot
/// 50; the others call one subsystem each (sound banks, waypoints, occlusion,
/// avalanche reset ...). Returns `data` (0 for message 14). Frame 0xc0
/// (box at 0, key at 0x20, ra 0x30, s0-s6, f20 at 0xb0).
pub fn cps2_chunk_resolve_release(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xc0 };
        c.cycles += 15;
        Frame::open(c, 0xc0);
        c.g[V0][0] = (c.g[A1][0] < ccr_release::MESSAGES) as u64;
        frame.save(c, S0, 0xa0);
        frame.save(c, S3, 0x70);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S1, 0x90);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0xb0);

        let exit = if c.g[V0][0] == 0 {
            CcrExit::ResultData
        } else {
            ccr_dispatch(c)?
        };
        if exit == CcrExit::ResultData {
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// The jump table and the message bodies.
fn ccr_dispatch(c: &mut Cpu) -> Result<CcrExit, crate::call::Unwound> {
    use ccr_release as k;
    c.cycles += 7;
    c.g[V0][0] = sx(k::TABLE);
    c.g[V1][0] = sx(r32(c, A1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(k::TABLE));
    c.g[A0][0] = c.lw(r32(c, V1));
    match r32(c, A0) {
        // 0, 2, 4, 12, 16: nothing.
        0x003a_be10 => Ok(CcrExit::ResultData),
        // 1: re-file `data` (box at +0x158) as kind 1.
        0x003a_b4f0 => {
            c.cycles += 46;
            ccr_std_update(c, 0x158, 1, CCR_SCAN_1)
        }
        // 3: box at +0x60, kind 0.
        0x003a_b664 => {
            c.cycles += 46;
            ccr_std_update(c, 0x60, 0, CCR_SCAN_3)
        }
        // 5: kind word 7, box at +0x68, kind 2.
        0x003a_bbb0 => {
            c.cycles += 48;
            c.g[V0][0] = 7;
            c.sw(r32(c, S3).wrapping_add(k::ITEM_KIND), 7);
            ccr_std_update(c, 0x68, 2, CCR_SCAN_5)
        }
        // 6: if self->vt[22](data): kind word 6, box at +0x44, kind 2.
        0x003a_b918 => {
            c.cycles += 8;
            c.g[V1][0] = c.lw(r32(c, S4).wrapping_add(k::SELF_VT));
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0xb0));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xb4));
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x003a_b930)?;
            let refile = c.g[V0][0] != 0;
            c.g[V0][0] = 6;
            if !refile {
                return Ok(CcrExit::ResultData);
            }
            c.cycles += 47;
            c.sw(r32(c, S3).wrapping_add(k::ITEM_KIND), 6);
            ccr_std_update(c, 0x44, 2, CCR_SCAN_6)
        }
        // 7: kind word 8, box at +0x28, kind 2.
        0x003a_ba70 => {
            c.cycles += 48;
            c.g[V0][0] = 8;
            c.sw(r32(c, S3).wrapping_add(k::ITEM_KIND), 8);
            ccr_std_update(c, 0x28, 2, CCR_SCAN_7)
        }
        // 8: every item of the list.
        0x003a_b798 => ccr_list_update(c),
        // 9: mission manager vt[50](param >> 8).
        0x003a_b628 => {
            c.cycles += 4;
            c.g[A2][0] = c.lw(r32(c, GP).wrapping_add(k::GP_MISSION_MAN));
            c.g[A1][0] = sx(r32(c, S0) >> 8);
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10d8));
            ccr_mission_call(c)
        }
        // 10: mission manager vt[50]((param >> 8) + self->base).
        0x003a_b638 => {
            c.cycles += 5;
            c.g[A2][0] = c.lw(r32(c, GP).wrapping_add(k::GP_MISSION_MAN));
            c.g[V1][0] = sx(r32(c, S0) >> 8);
            c.g[A1][0] = c.lw(r32(c, S4).wrapping_add(k::SELF_BASE));
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x10d8));
            c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
            ccr_mission_call(c)
        }
        // 13: func_002B6B30(func_0028B1C8()->+0x4c, data, param).
        0x003a_bd04 => {
            c.cycles += 8;
            call_ptr(c, 0x0028_b1c8, 0x003a_bd0c)?;
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x4c));
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x002b_6b30, 0x003a_bd1c)?;
            Ok(ccr_done(c))
        }
        // 17: func_0016CF18(param).
        0x003a_bd24 => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0016_cf18, 0x003a_bd2c)?;
            Ok(ccr_done(c))
        }
        // 15: WPFile_Unload(&files->table[param & 0xff]) if loaded.
        0x003a_bd34 => {
            c.cycles += 3;
            c.g[A3][0] = c.lw(r32(c, GP).wrapping_add(k::GP_WP_FILES));
            c.g[A1][0] = c.g[S0][0] & 0xff;
            if c.g[A3][0] == 0 {
                return Ok(CcrExit::ResultData);
            }
            c.cycles += 8;
            c.g[V0][0] = 0xc;
            c.mult(0, A1, A1, V0);
            c.g[V1][0] = c.lw(r32(c, A3).wrapping_add(0x8));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x4));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            call_ptr(c, 0x002b_b5a8, 0x003a_bd58)?;
            Ok(ccr_done(c))
        }
        // 18: func_00278A58(global, param & 0xff).
        0x003a_bd60 => {
            c.cycles += 5;
            c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(k::GP_AMBIENT));
            c.g[A1][0] = c.g[S0][0] & 0xff;
            call_ptr(c, 0x0027_8a58, 0x003a_bd6c)?;
            Ok(ccr_done(c))
        }
        // 20: SSXAudioSystem_UnloadMemoryBank(func_0028B180(), param, data).
        0x003a_bd74 => {
            c.cycles += 8;
            call_ptr(c, 0x0028_b180, 0x003a_bd7c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S3][0];
            call_ptr(c, 0x0028_6d18, 0x003a_bd8c)?;
            Ok(ccr_done(c))
        }
        // 19: the BE's interface 10, stub(param & 0xff).
        0x003a_bd94 => {
            c.cycles += 10;
            call_ptr(c, 0x0014_dc80, 0x003a_bd9c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014_dd58, 0x003a_bda8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0] & 0xff;
            call_ptr(c, 0x0015_47e0, 0x003a_bdb4)?;
            Ok(ccr_done(c))
        }
        // 21: func_00210608(data).
        0x003a_bdbc => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0021_0608, 0x003a_bdc4)?;
            Ok(ccr_done(c))
        }
        // 11: func_00242570(GameModule_GetOcclusionMan(), data).
        0x003a_bdcc => {
            c.cycles += 7;
            Callee {
                addr: 0x003a_d198,
                f: ee::s13::GameModule_GetOcclusionMan,
            }
            .call(c, 0x003a_bdd4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x0024_2570, 0x003a_bde0)?;
            Ok(ccr_done(c))
        }
        // 14: func_0012A490(world->+0x84->+0xc, param & 0xff); returns 0.
        0x003a_bde8 => {
            c.cycles += 8;
            c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(k::GP_WORLD));
            c.g[A1][0] = c.g[S0][0] & 0xff;
            c.g[S3][0] = 0;
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x84));
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0xc));
            call_ptr(c, 0x0012_a490, 0x003a_be00)?;
            Ok(ccr_done(c))
        }
        // 22: cAvalanche_reset().
        0x003a_be08 => {
            c.cycles += 2;
            call_ptr(c, 0x002d_9b40, 0x003a_be10)?;
            Ok(CcrExit::ResultData)
        }
        pc => {
            c.jump_out(pc, k::ENTRY, k::END);
            Err(crate::call::Unwound)
        }
    }
}

/// `v0 = data` in the delay slot of the branch to the epilogue.
fn ccr_done(c: &mut Cpu) -> CcrExit {
    c.g[V0][0] = c.g[S3][0];
    CcrExit::Epilogue
}

/// Messages 9/10 (0x3ab64c, a2 = the manager, a1 = the argument, v0 = its
/// vtable): `vt[50].fn(mgr + vt[50].delta, a1)`.
fn ccr_mission_call(c: &mut Cpu) -> Result<CcrExit, crate::call::Unwound> {
    c.cycles += 6;
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x190));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x194));
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x003a_b65c)?;
    Ok(ccr_done(c))
}

/// How one copy of the inlined "is the node empty" scan differs: the cycles
/// after the eighth child, the item loop's resume points, whether it is the
/// message-5 copy (different tail), and the call return addresses.
#[derive(Clone, Copy)]
struct CcrScan {
    key_ret: u32,
    remove_ret: u32,
    after_children: u64,
    items_head: u32,
    msg5: bool,
}

const CCR_SCAN_1: CcrScan = CcrScan {
    key_ret: 0x003a_b54c,
    remove_ret: 0x003a_b598,
    after_children: 3,
    items_head: 0x003a_b608,
    msg5: false,
};
const CCR_SCAN_3: CcrScan = CcrScan {
    key_ret: 0x003a_b6c0,
    remove_ret: 0x003a_b70c,
    after_children: 2,
    items_head: 0x003a_b778,
    msg5: false,
};
const CCR_SCAN_6: CcrScan = CcrScan {
    key_ret: 0x003a_b998,
    remove_ret: 0x003a_b9e4,
    after_children: 2,
    items_head: 0x003a_ba50,
    msg5: false,
};
const CCR_SCAN_7: CcrScan = CcrScan {
    key_ret: 0x003a_bad4,
    remove_ret: 0x003a_bb20,
    after_children: 3,
    items_head: 0x003a_bb90,
    msg5: false,
};
const CCR_SCAN_5: CcrScan = CcrScan {
    key_ret: 0x003a_bc14,
    remove_ret: 0x003a_bc60,
    after_children: 1,
    items_head: 0x003a_bcd0,
    msg5: true,
};
/// Message 5's "not empty" block (event resume point).
const CCR_NOT_EMPTY: u32 = 0x003a_bcc4;

/// The item's box (`base + off`: min xyz, max xyz) on the stack with w =
/// 1.0 from FPU register `one`; a0 = the key's slot (sp + 0x20), a1 = the box.
fn ccr_fill_box(c: &mut Cpu, base: u32, off: u32, one: usize) {
    let s = sp(c);
    let src = base.wrapping_add(off);
    c.lwc1(0, src);
    c.g[A0][0] = sx(s.wrapping_add(0x20));
    c.g[A1][0] = c.g[SP][0];
    c.swc1(0, s);
    c.lwc1(0, src.wrapping_add(0x4));
    c.swc1(0, s + 0x4);
    c.lwc1(1, src.wrapping_add(0x8));
    c.swc1(one, s + 0xc);
    c.swc1(1, s + 0x8);
    c.lwc1(0, src.wrapping_add(0xc));
    c.swc1(0, s + 0x10);
    c.lwc1(1, src.wrapping_add(0x10));
    c.swc1(1, s + 0x14);
    c.lwc1(0, src.wrapping_add(0x14));
    c.swc1(one, s + 0x1c);
    c.swc1(0, s + 0x18);
}

/// The inlined `update_B498` up to `SpatialHash_Remove` (box already on the
/// stack): count the update, key from the box, the root by the key's signs
/// (s0), remove `item` (kind `kind`) from its node. Leaves v1 = the node and
/// v0 = its first child.
fn ccr_remove(
    c: &mut Cpu,
    item: u64,
    kind: u64,
    key_ret: u32,
    remove_ret: u32,
) -> crate::call::Flow {
    use ccr_release as k;
    c.g[S0][0] = c.lw(r32(c, S4).wrapping_add(k::SELF_HASH));
    let updates = sx((c.lw(r32(c, S0).wrapping_add(k::HASH_UPDATES)) as u32).wrapping_add(1));
    c.g[V0][0] = updates;
    c.sw(r32(c, S0).wrapping_add(k::HASH_UPDATES), updates);
    call_ptr(c, k::KEY_FROM_BOX, key_ret)?;
    let s = sp(c);
    let kx = c.lw(s + 0x24) as i64;
    let ky = c.lw(s + 0x28) as i64;
    let kz = c.lw(s + 0x2c) as i64;
    let hash = r32(c, S0);
    let rx = if -1 < kx {
        hash
    } else {
        hash.wrapping_add(0x50)
    };
    let ry = if -1 < ky { rx } else { rx.wrapping_add(0x28) };
    let root = if -1 < kz { ry } else { ry.wrapping_add(0x14) };
    c.g[V1][0] = sx(rx);
    c.g[V0][0] = sx(ry);
    c.g[S0][0] = sx(root);
    c.g[A0][0] = c.lw(root.wrapping_add(k::ROOT_NODE));
    c.g[A1][0] = kind;
    c.g[A2][0] = item;
    c.g[A3][0] = sx(s.wrapping_add(0x20));
    c.g[T0][0] = c.g[S0][0];
    call_ptr(c, k::HASH_REMOVE, remove_ret)?;
    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(k::ROOT_NODE));
    c.g[V0][0] = c.lw(r32(c, V1));
    Ok(())
}

/// Messages 1/3/5/6/7 after the block's charge: box from `data + off`,
/// remove as `kind`, then delete the root's node if it is empty.
fn ccr_std_update(
    c: &mut Cpu,
    off: u32,
    kind: u64,
    scan: CcrScan,
) -> Result<CcrExit, crate::call::Unwound> {
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    let data = r32(c, S3);
    ccr_fill_box(c, data, off, 2);
    let item = c.g[S3][0];
    ccr_remove(c, item, kind, scan.key_ret, scan.remove_ret)?;

    // Empty: no children (+0..+0x1c) and no items (+0x20..+0x28).
    let empty = 'scan: {
        let node = r32(c, V1);
        for i in 0..8u32 {
            if i > 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw(node.wrapping_add(4 * i));
            }
            let child = c.g[V0][0] != 0;
            if scan.msg5 && i == 7 {
                if child {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'scan false;
                }
                c.g[V1][0] = sx(node.wrapping_add(0x20));
                break;
            }
            c.g[V0][0] = 0;
            if child {
                break 'scan false;
            }
        }
        c.cycles += scan.after_children;
        if !scan.msg5 {
            c.g[V1][0] = sx(node.wrapping_add(0x20));
        }
        c.g[A0][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            if c.g[V0][0] != 0 {
                if scan.msg5 && event_due(c) {
                    deliver_event_here(c, CCR_NOT_EMPTY);
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'scan false;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, scan.items_head);
            }
        }
        c.cycles += if scan.msg5 { 1 } else { 2 };
        c.g[V0][0] = 1;
        true
    };

    // 0x3abcec: delete the empty node.
    c.cycles += 2;
    c.g[V0][0] = c.g[S3][0];
    if !empty {
        return Ok(CcrExit::Epilogue);
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(ccr_release::ROOT_NODE));
    call_ptr(c, ccr_release::DELETE, 0x003a_bcfc)?;
    c.sw(r32(c, S0).wrapping_add(ccr_release::ROOT_NODE), 0);
    Ok(CcrExit::ResultData)
}

/// Message 8 (0x3ab798): for each of the list's items (count `data+0x20`,
/// first `data+0x24`, next `+0x64`): kind word 1, box at +0x6c, remove as
/// kind 2, delete the root's node if it is empty.
fn ccr_list_update(c: &mut Cpu) -> Result<CcrExit, crate::call::Unwound> {
    use ccr_release as k;
    c.cycles += 4;
    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(k::LIST_COUNT));
    c.g[S2][0] = 0;
    c.g[S1][0] = c.lw(r32(c, S3).wrapping_add(k::LIST_FIRST));
    if (c.g[V0][0] as i64) <= 0 {
        return Ok(CcrExit::ResultData);
    }
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 20);
    c.g[S6][0] = 1;
    c.g[S5][0] = u64::MAX;
    loop {
        // 0x3ab7b8
        c.cycles += 44;
        let item = r32(c, S1);
        c.sw(item.wrapping_add(k::ITEM_KIND), c.g[S6][0]);
        ccr_fill_box(c, item, 0x6c, 20);
        let item = c.g[S1][0];
        ccr_remove(c, item, 2, 0x003a_b810, 0x003a_b858)?;

        let empty = 'scan: {
            let node = r32(c, V1);
            for i in 0..8u32 {
                if i > 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(node.wrapping_add(4 * i));
                }
                let child = c.g[V0][0] != 0;
                if i == 6 {
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                } else if child && i < 6 {
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                }
                if child {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'scan false;
                }
            }
            c.cycles += 3;
            c.g[V1][0] = sx(node.wrapping_add(0x20));
            c.g[A0][0] = 0;
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw(r32(c, V1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_b8bc);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'scan false;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003a_b8d0);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
            true
        };

        // 0x3ab8ec
        c.cycles += 2;
        if empty {
            c.cycles += 4;
            c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(k::ROOT_NODE));
            call_ptr(c, k::DELETE, 0x003a_b8fc)?;
            c.sw(r32(c, S0).wrapping_add(k::ROOT_NODE), 0);
        }
        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(k::LIST_COUNT));
        // 0x3ab904
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[S1][0] = c.lw(r32(c, S1).wrapping_add(k::ITEM_NEXT));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003a_b7b8);
        }
    }
    c.cycles += 2;
    Ok(ccr_done(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{fill, put_words, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's callee fakes (in the machine's RAM).
    const CCR_CTRL: u32 = DATA + 0xf_fff0;
    const CCR_MAGIC: u32 = 0x5eed_b498;

    /// The message jump table (.rodata 0x495010) from the game image.
    const CCR_TABLE_WORDS: [u32; 23] = [
        0x003a_be10,
        0x003a_b4f0,
        0x003a_be10,
        0x003a_b664,
        0x003a_be10,
        0x003a_bbb0,
        0x003a_b918,
        0x003a_ba70,
        0x003a_b798,
        0x003a_b628,
        0x003a_b638,
        0x003a_bdcc,
        0x003a_be10,
        0x003a_bd04,
        0x003a_bde8,
        0x003a_bd34,
        0x003a_be10,
        0x003a_bd24,
        0x003a_bd60,
        0x003a_bd94,
        0x003a_bd74,
        0x003a_bdbc,
        0x003a_be08,
    ];

    // Fixture layout.
    const CCR_SELF: u32 = DATA + 0x4_0000;
    const CCR_VT6: u32 = DATA + 0x4_1000;
    const CCR_HASH: u32 = DATA + 0x4_2000;
    const CCR_NODES: u32 = DATA + 0x4_3000;
    const CCR_ITEM: u32 = DATA + 0x5_0000;
    const CCR_LIST: u32 = DATA + 0x5_1000;
    const CCR_MISSION: u32 = DATA + 0x6_0000;
    const CCR_MISSION_VT: u32 = DATA + 0x6_1000;
    const CCR_WPF: u32 = DATA + 0x6_2000;
    const CCR_WPF_TABLE: u32 = DATA + 0x6_3000;
    const CCR_WORLD: u32 = DATA + 0x6_5000;
    const CCR_AUDIO: u32 = DATA + 0x8_0000;

    macro_rules! ccr_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(CCR_CTRL) == CCR_MAGIC {
                    return fakes::fake_call(c, $addr);
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    ccr_gated!(
        ccr_fake_key,
        CCR_PREV_KEY,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );
    ccr_gated!(
        ccr_fake_remove,
        CCR_PREV_REMOVE,
        0x0032_84b8,
        ee::s10::f_003284b8__generated
    );
    ccr_gated!(
        ccr_fake_delete,
        CCR_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );
    ccr_gated!(
        ccr_fake_28b1c8,
        CCR_PREV_28B1C8,
        0x0028_b1c8,
        ee::s07::f_0028b1c8__generated
    );
    ccr_gated!(
        ccr_fake_278a58,
        CCR_PREV_278A58,
        0x0027_8a58,
        ee::s07::f_00278a58__generated
    );
    ccr_gated!(
        ccr_fake_28b180,
        CCR_PREV_28B180,
        0x0028_b180,
        ee::s07::f_0028b180__generated
    );
    ccr_gated!(
        ccr_fake_2b6b30,
        CCR_PREV_2B6B30,
        0x002b_6b30,
        ee::s08::f_002b6b30__generated
    );
    ccr_gated!(
        ccr_fake_16cf18,
        CCR_PREV_16CF18,
        0x0016_cf18,
        ee::s02::f_0016cf18__generated
    );
    ccr_gated!(
        ccr_fake_14dc80,
        CCR_PREV_14DC80,
        0x0014_dc80,
        ee::s01::f_0014dc80__generated
    );
    ccr_gated!(
        ccr_fake_14dd58,
        CCR_PREV_14DD58,
        0x0014_dd58,
        ee::s01::f_0014dd58__generated
    );
    ccr_gated!(
        ccr_fake_210608,
        CCR_PREV_210608,
        0x0021_0608,
        ee::s05::f_00210608__generated
    );
    ccr_gated!(
        ccr_fake_3ad198,
        CCR_PREV_3AD198,
        0x003a_d198,
        ee::s13::f_003ad198__generated
    );
    ccr_gated!(
        ccr_fake_242570,
        CCR_PREV_242570,
        0x0024_2570,
        ee::s06::f_00242570__generated
    );
    ccr_gated!(
        ccr_fake_12a490,
        CCR_PREV_12A490,
        0x0012_a490,
        ee::s00::f_0012a490__generated
    );
    ccr_gated!(
        ccr_fake_2d9b40,
        CCR_PREV_2D9B40,
        0x002d_9b40,
        ee::s09::f_002d9b40__generated
    );

    /// Gated fakes in the callees' override slots (once per process). The
    /// callees without a slot run for real on short paths: WPFile_Unload on
    /// an unloaded entry, the bank unload on a slot that does not match, the
    /// empty BE stub.
    #[allow(unused_unsafe)]
    fn ccr_install_fakes() {
        use std::ptr::addr_of_mut;
        unsafe {
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00328F28),
                ccr_fake_key,
                &CCR_PREV_KEY,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_003284B8),
                ccr_fake_remove,
                &CCR_PREV_REMOVE,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00317E50),
                ccr_fake_delete,
                &CCR_PREV_DELETE,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_0028B1C8),
                ccr_fake_28b1c8,
                &CCR_PREV_28B1C8,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_00278A58),
                ccr_fake_278a58,
                &CCR_PREV_278A58,
            );
            fakes::hook(
                addr_of_mut!(ee::s07::OVERRIDE_0028B180),
                ccr_fake_28b180,
                &CCR_PREV_28B180,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002B6B30),
                ccr_fake_2b6b30,
                &CCR_PREV_2B6B30,
            );
            fakes::hook(
                addr_of_mut!(ee::s02::OVERRIDE_0016CF18),
                ccr_fake_16cf18,
                &CCR_PREV_16CF18,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
                ccr_fake_14dc80,
                &CCR_PREV_14DC80,
            );
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
                ccr_fake_14dd58,
                &CCR_PREV_14DD58,
            );
            fakes::hook(
                addr_of_mut!(ee::s05::OVERRIDE_00210608),
                ccr_fake_210608,
                &CCR_PREV_210608,
            );
            fakes::hook(
                addr_of_mut!(ee::s13::OVERRIDE_003AD198),
                ccr_fake_3ad198,
                &CCR_PREV_3AD198,
            );
            fakes::hook(
                addr_of_mut!(ee::s06::OVERRIDE_00242570),
                ccr_fake_242570,
                &CCR_PREV_242570,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0012A490),
                ccr_fake_12a490,
                &CCR_PREV_12A490,
            );
            fakes::hook(
                addr_of_mut!(ee::s09::OVERRIDE_002D9B40),
                ccr_fake_2d9b40,
                &CCR_PREV_2D9B40,
            );
        }
    }

    /// One message per seed (half of them the spatial-hash ones, a few out
    /// of range, one with the upper half of a1 set), the hash's eight root
    /// nodes with zero or one non-empty word (so every scan exit is taken),
    /// random key signs, a 0..3-item list for message 8.
    fn ccr_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0xb498, 6);
        c.bus.write32(CCR_CTRL, CCR_MAGIC);
        put_words(c, ccr_release::TABLE, &CCR_TABLE_WORDS);

        let msg: u64 = if seed % 2 == 0 {
            [1, 3, 5, 6, 7, 8][(seed / 2 % 6) as usize]
        } else {
            match seed / 2 % 25 {
                23 => 0x17 + (r.next_u32() % 0x100) as u64,
                24 => 1u64 << 32 | 1,
                m => m as u64,
            }
        };
        let param: u64 = if msg == 20 {
            // A bank slot that does not match: the unload returns at once.
            ((r.next_u32() % 16) << 8 | (1 + r.next_u32() % 255)) as u64
        } else {
            (r.next_u32() as u64) << 32 | r.next_u32() as u64
        };
        if msg == 20 {
            c.bus.write32(LOG_RESULTS, CCR_AUDIO);
        }

        // self: vtable (slot 22 at +0xb0), the hash, a base index.
        c.bus.write32(CCR_SELF + ccr_release::SELF_VT, CCR_VT6);
        c.bus.write32(CCR_SELF + ccr_release::SELF_HASH, CCR_HASH);
        c.bus
            .write32(CCR_SELF + ccr_release::SELF_BASE, r.next_u32());
        c.bus.write16(CCR_VT6 + 0xb0, (r.next_u32() as u16) & !3);
        let slot22 = if r.next_u32() % 3 == 0 {
            0x0014_06c0
        } else {
            FAKE
        };
        c.bus.write32(CCR_VT6 + 0xb4, slot22);

        // The hash: update counter, eight roots and their nodes.
        c.bus
            .write32(CCR_HASH + ccr_release::HASH_UPDATES, r.next_u32());
        for k in 0..8u32 {
            let node = CCR_NODES + 0x40 * k;
            c.bus
                .write32(CCR_HASH + 0x14 * k + ccr_release::ROOT_NODE, node);
            let busy = r.next_u32() % 14;
            for w in 0..11u32 {
                let v = if w == busy { r.next_u32() | 1 } else { 0 };
                c.bus.write32(node + 4 * w, v);
            }
        }
        // The key (x, y, z words) the fake KEY_FROM_BOX leaves on the stack.
        let key = crate::difftest::SP - 0xc0 + 0x24;
        for i in 0..3 {
            let v = r.next_u32();
            c.bus.write32(
                key + 4 * i,
                if r.next_u32() % 2 == 0 {
                    v >> 1
                } else {
                    v | 1 << 31
                },
            );
        }

        // data: boxes everywhere, a list of 0..3 items (count may be <= 0).
        for i in 0..0x60u32 {
            let f = r.f32(100.0);
            c.bus.write32(CCR_ITEM + 4 * i, f);
        }
        let count = (r.next_u32() % 5) as i32 - 1;
        c.bus
            .write32(CCR_ITEM + ccr_release::LIST_COUNT, count as u32);
        c.bus.write32(CCR_ITEM + ccr_release::LIST_FIRST, CCR_LIST);
        for i in 0..4u32 {
            let e = CCR_LIST + 0x100 * i;
            for w in 0..0x22u32 {
                let f = r.f32(100.0);
                c.bus.write32(e + 4 * w, f);
            }
            c.bus.write32(e + ccr_release::ITEM_NEXT, e + 0x100);
        }

        // gp globals: mission manager (vt slot 50 at +0x190), waypoint files
        // (unloaded entries), an ambient pointer, the world.
        let gp = GAME_GP;
        c.bus.write32(gp + ccr_release::GP_MISSION_MAN, CCR_MISSION);
        c.bus.write32(CCR_MISSION + 0x10d8, CCR_MISSION_VT);
        c.bus
            .write16(CCR_MISSION_VT + 0x190, (r.next_u32() as u16) & !3);
        c.bus.write32(CCR_MISSION_VT + 0x194, FAKE + 0x40);
        let wpf = if r.next_u32() % 3 == 0 { 0 } else { CCR_WPF };
        c.bus.write32(gp + ccr_release::GP_WP_FILES, wpf);
        c.bus.write32(CCR_WPF + 0x8, CCR_WPF + 0x100);
        c.bus.write32(CCR_WPF + 0x104, CCR_WPF_TABLE);
        c.bus.write32(gp - ccr_release::GP_AMBIENT, r.next_u32());
        c.bus.write32(gp - ccr_release::GP_WORLD, CCR_WORLD);
        c.bus.write32(CCR_WORLD + 0x84, CCR_WORLD + 0x100);
        fill(c, &mut r, CCR_WORLD + 0x100, 8);

        c.g[A0][0] = CCR_SELF as u64;
        c.g[A1][0] = msg;
        c.g[A2][0] = param;
        c.g[A3][0] = CCR_ITEM as u64;
        c.pc = ccr_release::ENTRY;
    }

    /// The world view's message handler against the generated body.
    #[test]
    fn cps2_chunk_resolve_release_matches() {
        ccr_install_fakes();
        covered(
            ee::s13::f_003ab498__generated,
            cps2_chunk_resolve_release,
            500,
            &|c, seed| ccr_setup(c, seed),
        );
    }
}
