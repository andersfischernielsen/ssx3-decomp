//! Staging module oc463 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc463`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0033_a3d8,
    cell_gather_obj_inst_recursive,
    "Cell_GatherObjInstRecursive",
)];

/// `Cell_GatherObjects(ctx, node)`: the node's own objects into the callback.
const GATHER_OBJECTS: Callee = Callee {
    addr: 0x0033_09d8,
    f: ee::s11::f_003309d8,
};
/// `Cell_GatherObjInstSubtree(node, ctx)`: a whole subtree into the callback.
const GATHER_SUBTREE: Callee = Callee {
    addr: 0x0034_0c20,
    f: ee::s11::f_00340c20,
};
/// This function again (through its override slot).
const RECURSE: Callee = Callee {
    addr: 0x0033_a3d8,
    f: ee::s11::f_0033a3d8,
};

/// How a collected child's subtree chain ends. Seven octants reload `V0` from
/// the stack and leave the child; the last one (child 4) restores `s0` and
/// leaves the function.
#[derive(Clone, Copy)]
enum Trail {
    Reload(u32),
    Restore,
}

/// The frame of the function: `0xe0` bytes, `ra` at +0x40, callee-saved
/// registers at +0x50..+0xd0.
#[inline]
fn frame() -> Frame {
    Frame { size: 0xe0 }
}

/// `lw` from an absolute address.
#[inline]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

/// `lw` from `reg + off`.
#[inline]
fn lw_reg(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    Addr(a).lw(c)
}

/// `lw` from `sp + off`.
#[inline]
fn ld_sp(c: &mut Cpu, off: u32) -> u64 {
    let a = sp(c).wrapping_add(off);
    Addr(a).lw(c)
}

/// `sw` to `sp + off`.
#[inline]
fn st_sp(c: &mut Cpu, off: u32, v: u64) {
    let a = sp(c).wrapping_add(off);
    Addr(a).sw(c, v);
}

/// `lq s0, 0xd0(sp)`: every exit of the function restores `s0` this way.
#[inline]
fn restore_s0(c: &mut Cpu) {
    frame().restore(c, S0, 0xd0);
}

/// The box pointer (`a3` on entry, kept in `s2`).
#[inline]
fn boxp(c: &Cpu) -> u32 {
    c.g[S2][0] as u32
}

#[inline]
fn fr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `sub.s fd, fs, ft`.
#[inline]
fn fsub(c: &mut Cpu, d: usize, a: usize, b: usize) {
    let v = fr(c, a) - fr(c, b);
    c.f[d] = v.0;
}

/// `add.s fd, fs, ft`.
#[inline]
fn fadd(c: &mut Cpu, d: usize, a: usize, b: usize) {
    let v = fr(c, a) + fr(c, b);
    c.f[d] = v.0;
}

/// `mul.s fd, fs, ft`.
#[inline]
fn fmul(c: &mut Cpu, d: usize, a: usize, b: usize) {
    let v = fr(c, a) * fr(c, b);
    c.f[d] = v.0;
}

/// `cvt.s.w fd, fs`.
#[inline]
fn cvt_w(c: &mut Cpu, d: usize, s: usize) {
    let v = FpuF32::from_i32(c.f[s] as i32);
    c.f[d] = v.0;
}

/// `c.lt.s fa, fb`, leaving the condition bit set to the result.
#[inline]
fn fcc_lt(c: &mut Cpu, a: usize, b: usize) -> bool {
    let t = fr(c, a).lt(fr(c, b));
    crate::abi::set_fcc(c, t);
    t
}

/// `c.le.s fa, fb`, leaving the condition bit set to the result.
#[inline]
fn fcc_le(c: &mut Cpu, a: usize, b: usize) -> bool {
    let t = fr(c, a).le(fr(c, b));
    crate::abi::set_fcc(c, t);
    t
}

/// A test on a register: `cy` cycles, branch on `reg == 0`, then `V0 =
/// sp[reload]`. The branch reads `reg` before the reload.
#[inline]
fn zchk(c: &mut Cpu, cy: u64, reg: usize, reload: u32) -> bool {
    let t = c.g[reg][0] == 0;
    c.cycles += cy;
    c.g[V0][0] = ld_sp(c, reload);
    t
}

/// A test on `sp[src]` (loaded into `V0` first), then `V0 = sp[reload]`.
#[inline]
fn zload(c: &mut Cpu, cy: u64, src: u32, reload: u32) -> bool {
    c.cycles += cy;
    c.g[V0][0] = ld_sp(c, src);
    let t = c.g[V0][0] == 0;
    c.g[V0][0] = ld_sp(c, reload);
    t
}

/// One slab test of the object loop: loads `f1` from `f1_addr` and `f0` from
/// `f0_addr`, then `f0 <= f1` (or `f1 <= f0` when `swap`), setting the
/// condition bit.
fn slab_test(c: &mut Cpu, cy: u64, f1_addr: u32, f0_addr: u32, swap: bool) -> bool {
    c.cycles += cy;
    c.lwc1(1, f1_addr);
    c.lwc1(0, f0_addr);
    if swap {
        fcc_le(c, 1, 0)
    } else {
        fcc_le(c, 0, 1)
    }
}

/// Does the object at `obj` overlap the box at `bx`? Six slab tests against
/// the object's bounds (min x/y/z at +0x158/+0x15c/+0x160, max at
/// +0x164/+0x168/+0x16c); each stops at the first miss, as the loop does.
fn overlaps_box(c: &mut Cpu, obj: u32, bx: u32) -> bool {
    slab_test(c, 6, obj + 0x164, bx, false)
        && slab_test(c, 5, bx + 0x10, obj + 0x158, false)
        && slab_test(c, 5, bx + 0x4, obj + 0x168, true)
        && slab_test(c, 5, bx + 0x14, obj + 0x15c, false)
        && slab_test(c, 5, bx + 0x8, obj + 0x16c, true)
        && slab_test(c, 6, bx + 0x18, obj + 0x160, false)
}

/// The object loop. `A2` walks the node's list (each object's first word is
/// the next link), `V0` is the box, `A3` is the callback (`+0` array, `+4`
/// count pointer). An overlapping object is appended to the callback's array.
/// Returns when the list ends; a due event delivers at `head`.
fn push_objects(c: &mut Cpu, head: u32) -> Flow {
    loop {
        let obj = c.g[A2][0] as u32;
        let bx = c.g[V0][0] as u32;
        let hit = overlaps_box(c, obj, bx);
        c.g[V1][0] = hit as u64;
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.g[A2][0] = lw(c, obj);
        } else {
            c.cycles += 9;
            let a3 = c.g[A3][0] as u32;
            c.g[A1][0] = lw(c, a3.wrapping_add(4));
            c.g[A0][0] = lw(c, a3);
            let a1 = c.g[A1][0] as u32;
            c.g[V1][0] = lw(c, a1);
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            let slot = c.g[V0][0] as u32;
            let link = c.g[A2][0];
            Addr(slot).sw(c, link);
            let count = c.g[V1][0];
            Addr(a1).sw(c, count);
            c.g[A2][0] = lw(c, obj);
        }
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            let a3 = c.g[A3][0] as u32;
            c.g[V0][0] = lw(c, a3.wrapping_add(0xc));
            if event_due(c) {
                deliver_event_here(c, head);
            }
        } else {
            return Ok(());
        }
    }
}

/// `Cell_GatherObjects(ctx, node)`, then the node's object list pushed into
/// the callback (`ctx_reg`, `node_reg` name the registers holding them).
/// `after_go` is the cycle charge before the list loop starts.
fn gather_objects(
    c: &mut Cpu,
    ctx_reg: usize,
    node_reg: usize,
    go_ret: u32,
    head: u32,
    after_go: u64,
) -> Flow {
    c.g[A0][0] = c.g[ctx_reg][0];
    c.g[A1][0] = c.g[node_reg][0];
    GATHER_OBJECTS.call(c, go_ret)?;
    let node = c.g[node_reg][0] as u32;
    c.g[A2][0] = lw(c, node.wrapping_add(0x24));
    let ctx = c.g[ctx_reg][0] as u32;
    c.g[A3][0] = sx(ctx.wrapping_add(0x10));
    if c.g[A2][0] == 0 {
        return Ok(());
    }
    c.cycles += after_go;
    let a3 = c.g[A3][0] as u32;
    c.g[V0][0] = lw(c, a3.wrapping_add(0xc));
    push_objects(c, head)
}

/// The eight child subtrees of the collected cell (`s0`), each gathered into
/// the callback `s1`: child `j` is block `j` of the chain, whose return
/// addresses step by 0x14 from `base`. Block 7 is the trailing block.
fn gather_subtrees(c: &mut Cpu, base: u32, trail: Trail) -> Flow {
    c.cycles += 3;
    let s0 = c.g[S0][0] as u32;
    c.g[A0][0] = lw(c, s0);
    for j in 0..7u32 {
        if j > 0 {
            c.cycles += 2;
        }
        if c.g[A0][0] == 0 {
            let s0 = c.g[S0][0] as u32;
            c.g[A0][0] = lw(c, s0.wrapping_add(4 * (j + 1)));
        } else {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            GATHER_SUBTREE.call(c, base + 0x14 * j)?;
            let s0 = c.g[S0][0] as u32;
            c.g[A0][0] = lw(c, s0.wrapping_add(4 * (j + 1)));
        }
    }
    // Trailing block: child 7.
    c.cycles += 2;
    let t = c.g[A0][0] == 0;
    match trail {
        Trail::Reload(off) => {
            c.g[V0][0] = ld_sp(c, off);
            if t {
                return Ok(());
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            GATHER_SUBTREE.call(c, base + 0x14 * 7)?;
            c.g[V0][0] = ld_sp(c, off);
            Ok(())
        }
        Trail::Restore => {
            restore_s0(c);
            if t {
                return Ok(());
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            GATHER_SUBTREE.call(c, base + 0x14 * 7)?;
            restore_s0(c);
            Ok(())
        }
    }
}

/// `collect_33A3D8(subNode, ctx2)`: the sub-cell's own objects, then its
/// eight children's whole subtrees. `go_ret`/`head`/`after_go` are the
/// generated body's addresses and charge for this octant's copy.
fn collect(
    c: &mut Cpu,
    go_ret: u32,
    head: u32,
    after_go: u64,
    base: u32,
    trail: Trail,
) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    gather_objects(c, S1, S0, go_ret, head, after_go)?;
    gather_subtrees(c, base, trail)
}

/// The recursive visit of an octant (`nash_privy_QueryHelper_helpX3` on the
/// child, with the sub-key at `sp`).
fn recurse(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 7;
    c.g[T0][0] = c.g[S0][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[T1][0] = c.g[SP][0];
    RECURSE.call(c, ret)
}

/// The three containment tests of an octant (`cx` tests): `first` (a flag
/// register or `V0`), then the two stack flags at `reloads`. Returns true when
/// one fails (the octant recurses); otherwise sets `A0 = ctx2` for the
/// collect.
fn cx_chain(c: &mut Cpu, first: usize, reloads: [u32; 2]) -> bool {
    if zchk(c, 2, first, reloads[0]) {
        return true;
    }
    if zchk(c, 2, V0, reloads[1]) {
        return true;
    }
    c.cycles += 2;
    let t = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S1][0];
    t
}

/// Octant 0 (child `+0`, no sub-key change). `Ok(())` when the visit leaves
/// by its outer exit.
fn octant0(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[S0][0] = lw_reg(c, S3, 0);
    let null = c.g[S0][0] == 0;
    c.g[V0][0] = ld_sp(c, 0xc);
    if null {
        return Ok(());
    }
    if octant0_in(c)? {
        c.cycles += 1;
        c.g[V0][0] = ld_sp(c, 0xc);
    }
    Ok(())
}

/// The inner part of octant 0. `Ok(true)` leaves by the inner exit (the
/// generated body's `+1` and reload), `Ok(false)` by the outer one.
fn octant0_in(c: &mut Cpu) -> Result<bool, Unwound> {
    if zchk(c, 2, S7, 0x14) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0x18) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0xc) {
        return Ok(false);
    }
    if cx_chain(c, S6, [0x24, 0x2c]) {
        recurse(c, 0x0033_aa30)?;
        return Ok(true);
    }
    collect(c, 0x0033_a8a8, 0x0033_a8b8, 1, 0x0033_a980, Trail::Reload(0xc))?;
    Ok(false)
}

/// Octant 1: child `+4`, sub-key z+1.
fn octant1(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[S0][0] = lw_reg(c, S3, 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let null = c.g[S0][0] == 0;
    let v = c.g[V0][0];
    st_sp(c, 0xc, v);
    let inside = null || octant1_in(c)?;
    if inside {
        c.cycles += 1;
        c.g[V0][0] = ld_sp(c, 8);
    }
    Ok(())
}

fn octant1_in(c: &mut Cpu) -> Result<bool, Unwound> {
    if zchk(c, 2, S7, 0x14) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0x1c) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0x8) {
        return Ok(false);
    }
    if cx_chain(c, S6, [0x24, 0x30]) {
        recurse(c, 0x0033_ac08)?;
        return Ok(true);
    }
    collect(c, 0x0033_aa7c, 0x0033_aa90, 2, 0x0033_ab58, Trail::Reload(0x8))?;
    Ok(false)
}

/// Octant 3: child `+0xc`, sub-key y+1.
fn octant3(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[S0][0] = lw_reg(c, S3, 0xc);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let null = c.g[S0][0] == 0;
    let v = c.g[V0][0];
    st_sp(c, 8, v);
    let inside = null || octant3_in(c)?;
    if inside {
        c.cycles += 1;
        c.g[V0][0] = ld_sp(c, 0xc);
    }
    Ok(())
}

fn octant3_in(c: &mut Cpu) -> Result<bool, Unwound> {
    if zchk(c, 2, S7, 0xc) {
        return Ok(false);
    }
    if zchk(c, 2, FP, 0x1c) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0xc) {
        return Ok(false);
    }
    if cx_chain(c, S6, [0x28, 0x30]) {
        recurse(c, 0x0033_ade0)?;
        return Ok(true);
    }
    collect(c, 0x0033_ac54, 0x0033_ac68, 2, 0x0033_ad30, Trail::Reload(0xc))?;
    Ok(false)
}

/// Octant 2: child `+8`, sub-key z-1.
fn octant2(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[S0][0] = lw_reg(c, S3, 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let null = c.g[S0][0] == 0;
    let v = c.g[V0][0];
    st_sp(c, 0xc, v);
    let inside = null || octant2_in(c)?;
    if inside {
        c.cycles += 1;
        c.g[V0][0] = ld_sp(c, 4);
    }
    Ok(())
}

fn octant2_in(c: &mut Cpu) -> Result<bool, Unwound> {
    if zchk(c, 2, S7, 0x4) {
        return Ok(false);
    }
    if zchk(c, 2, FP, 0x18) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0x4) {
        return Ok(false);
    }
    if cx_chain(c, S6, [0x28, 0x2c]) {
        recurse(c, 0x0033_afb8)?;
        return Ok(true);
    }
    collect(c, 0x0033_ae2c, 0x0033_ae40, 2, 0x0033_af08, Trail::Reload(0x4))?;
    Ok(false)
}

/// Octant 6: child `+0x18`, sub-key x+1.
fn octant6(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[S0][0] = lw_reg(c, S3, 0x18);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let null = c.g[S0][0] == 0;
    let v = c.g[V0][0];
    st_sp(c, 4, v);
    let inside = null || octant6_in(c)?;
    if inside {
        c.cycles += 1;
        c.g[V0][0] = ld_sp(c, 0xc);
    }
    Ok(())
}

fn octant6_in(c: &mut Cpu) -> Result<bool, Unwound> {
    if zload(c, 3, 0x10, 0xc) {
        return Ok(false);
    }
    if zchk(c, 2, FP, 0x18) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0x20) {
        return Ok(true);
    }
    if cx_chain(c, V0, [0x28, 0x2c]) {
        recurse(c, 0x0033_b190)?;
        return Ok(true);
    }
    collect(c, 0x0033_b008, 0x0033_b018, 1, 0x0033_b0e0, Trail::Reload(0xc))?;
    Ok(false)
}

/// Octant 7: child `+0x1c`, sub-key z+1.
fn octant7(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[S0][0] = lw_reg(c, S3, 0x1c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let null = c.g[S0][0] == 0;
    let v = c.g[V0][0];
    st_sp(c, 0xc, v);
    let inside = null || octant7_in(c)?;
    if inside {
        c.cycles += 1;
        c.g[V0][0] = ld_sp(c, 8);
    }
    Ok(())
}

fn octant7_in(c: &mut Cpu) -> Result<bool, Unwound> {
    if zload(c, 3, 0x10, 0x8) {
        return Ok(false);
    }
    if zchk(c, 2, FP, 0x1c) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0x20) {
        return Ok(true);
    }
    if cx_chain(c, V0, [0x28, 0x30]) {
        recurse(c, 0x0033_b368)?;
        return Ok(true);
    }
    collect(c, 0x0033_b1e0, 0x0033_b1f0, 1, 0x0033_b2b8, Trail::Reload(0x8))?;
    Ok(false)
}

/// Octant 5: child `+0x14`, sub-key y-1.
fn octant5(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[S0][0] = lw_reg(c, S3, 0x14);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let null = c.g[S0][0] == 0;
    let v = c.g[V0][0];
    st_sp(c, 8, v);
    let inside = null || octant5_in(c)?;
    if inside {
        c.cycles += 1;
        c.g[V0][0] = ld_sp(c, 0xc);
    }
    Ok(())
}

fn octant5_in(c: &mut Cpu) -> Result<bool, Unwound> {
    if zload(c, 3, 0x10, 0x14) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0x1c) {
        return Ok(true);
    }
    if zchk(c, 2, V0, 0x20) {
        return Ok(true);
    }
    if cx_chain(c, V0, [0x24, 0x30]) {
        recurse(c, 0x0033_b540)?;
        return Ok(true);
    }
    collect(c, 0x0033_b3b8, 0x0033_b3c8, 1, 0x0033_b490, Trail::Reload(0xc))?;
    Ok(false)
}

/// The function's own exit after an inner visit: `+1`, restore `s0`.
fn finish_inner(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    restore_s0(c);
    Ok(())
}

/// Octant 4: child `+0x10`, sub-key z-1. Its outer exits leave the function.
fn octant4(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[S0][0] = lw_reg(c, S3, 0x10);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let null = c.g[S0][0] == 0;
    let v = c.g[V0][0];
    st_sp(c, 0xc, v);
    if null {
        return finish_inner(c);
    }
    if zload(c, 3, 0x10, 0x14) {
        return finish_inner(c);
    }
    if zchk(c, 2, V0, 0x18) {
        return finish_inner(c);
    }
    if zchk(c, 2, V0, 0x20) {
        return finish_inner(c);
    }
    if cx_chain(c, V0, [0x24, 0x2c]) {
        recurse(c, 0x0033_b718)?;
        return finish_inner(c);
    }
    collect(c, 0x0033_b590, 0x0033_b5a0, 1, 0x0033_b668, Trail::Restore)
}

/// `'b_33a79c`..`'b_33a864`: one containment flag on a pair of edges. `slot`
/// gets `box.min < edge_lo && edge_hi < box.max` (`bound` is the box max).
fn range_flag(c: &mut Cpu, lo: usize, edge_lo: usize, bound: u32, edge_hi: usize, slot: u32) {
    c.cycles += 4;
    let ok = fcc_lt(c, lo, edge_lo);
    st_sp(c, slot, 0);
    if ok {
        c.cycles += 5;
        let b = boxp(c).wrapping_add(bound);
        c.lwc1(0, b);
        c.g[V0][0] = 1;
        if fcc_lt(c, edge_hi, 0) {
            c.cycles += 1;
            let v = c.g[V0][0];
            st_sp(c, slot, v);
        }
    }
}

/// The octants of a cell above the leaves: the eight child visits with the
/// per-axis overlap and containment flags in `sp+0x10..0x30` (ox1, oy0,
/// oz0, oz1, cx1, cy0, cy1, cz0, cz1) and `s7`/`fp`/`s6` (ox0, oy1, cx0).
fn octants(c: &mut Cpu) -> Flow {
    // Edges of the sub-cells (`sub = 2 * cell`, size 2^(level-1)): f4 is the
    // cell size, f2 the 0.2 margin, f0/f1 the low/high edge of x0.
    c.cycles += 28;
    c.g[A0][0] = lw_reg(c, S0, 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
    let margin = (c.g[GP][0] as u32).wrapping_sub(0x2d28);
    c.lwc1(2, margin);
    c.mtc1(2, 4);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.mtc1(4, 0);
    cvt_w(c, 0, 0);
    c.mtc1(2, 1);
    cvt_w(c, 1, 1);
    let bx = boxp(c);
    c.lwc1(3, bx);
    c.g[V1][0] = lw_reg(c, S0, 0xc);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
    c.g[V0][0] = lw_reg(c, S0, 8);
    fsub(c, 0, 0, 2);
    fadd(c, 1, 1, 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    let v = c.g[A1][0];
    st_sp(c, 0, v);
    c.g[S7][0] = 0;
    fmul(c, 15, 0, 4);
    fmul(c, 16, 1, 4);
    let v = c.g[V0][0];
    st_sp(c, 8, v);
    let v = c.g[V1][0];
    st_sp(c, 0xc, v);
    let ok = fcc_lt(c, 3, 16);
    let v = c.g[A0][0];
    st_sp(c, 4, v);
    if ok {
        c.cycles += 5;
        c.lwc1(0, bx.wrapping_add(0x10));
        if fcc_lt(c, 15, 0) {
            c.g[S7][0] = 1;
        }
    }
    // ox1 (x1 pair, f2 = box min x)
    c.cycles += 18;
    c.g[V0][0] = ld_sp(c, 4);
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d24));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(2));
    st_sp(c, 0x10, 0);
    c.mtc1(3, 1);
    cvt_w(c, 1, 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.mtc1(2, 0);
    cvt_w(c, 0, 0);
    c.lwc1(2, bx);
    fadd(c, 1, 1, 3);
    c.f[13] = c.f[2];
    fsub(c, 0, 0, 3);
    fmul(c, 14, 1, 4);
    let ok = fcc_lt(c, 2, 14);
    fmul(c, 12, 0, 4);
    if ok {
        c.cycles += 5;
        c.lwc1(0, bx.wrapping_add(0x10));
        c.g[V0][0] = 1;
        if fcc_lt(c, 12, 0) {
            c.cycles += 1;
            let v = c.g[V0][0];
            st_sp(c, 0x10, v);
        }
    }
    // ox early exit: no ox0 and no ox1 is the whole cell's miss.
    c.cycles += 2;
    let t = c.g[S7][0] != 0;
    c.g[V1][0] = ld_sp(c, 8);
    if !t {
        c.cycles += 3;
        c.g[V0][0] = ld_sp(c, 0x10);
        let miss = c.g[V0][0] == 0;
        restore_s0(c);
        if miss {
            return Ok(());
        }
    }
    // oy0 (y0 pair, f2 = box min y)
    c.cycles += 17;
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d20));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    st_sp(c, 0x14, 0);
    c.mtc1(2, 1);
    cvt_w(c, 1, 1);
    c.g[A0][0] = c.g[V1][0];
    c.mtc1(3, 0);
    cvt_w(c, 0, 0);
    c.lwc1(2, bx.wrapping_add(4));
    fadd(c, 1, 1, 3);
    c.f[6] = c.f[2];
    fsub(c, 0, 0, 3);
    fmul(c, 11, 1, 4);
    let ok = fcc_lt(c, 2, 11);
    fmul(c, 10, 0, 4);
    if ok {
        c.cycles += 5;
        c.lwc1(0, bx.wrapping_add(0x14));
        c.g[V0][0] = 1;
        if fcc_lt(c, 10, 0) {
            c.cycles += 1;
            let v = c.g[V0][0];
            st_sp(c, 0x14, v);
        }
    }
    // oy1 (y1 pair)
    c.cycles += 15;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(2));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d1c));
    c.mtc1(3, 1);
    cvt_w(c, 1, 1);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.mtc1(2, 0);
    cvt_w(c, 0, 0);
    c.g[FP][0] = 0;
    fadd(c, 1, 1, 2);
    fsub(c, 0, 0, 2);
    fmul(c, 9, 1, 4);
    let ok = fcc_lt(c, 6, 9);
    fmul(c, 8, 0, 4);
    if ok {
        c.cycles += 5;
        c.lwc1(0, bx.wrapping_add(0x14));
        if fcc_lt(c, 8, 0) {
            c.g[FP][0] = 1;
        }
    }
    // oy early exit.
    c.cycles += 3;
    c.g[V0][0] = ld_sp(c, 0x14);
    let t = c.g[V0][0] != 0;
    c.g[V1][0] = ld_sp(c, 0xc);
    if !t {
        c.cycles += 2;
        let miss = c.g[FP][0] == 0;
        restore_s0(c);
        if miss {
            return Ok(());
        }
    }
    // oz0 (z0 pair, f2 = box min z)
    c.cycles += 17;
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x2d18));
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    st_sp(c, 0x18, 0);
    c.mtc1(2, 1);
    cvt_w(c, 1, 1);
    c.g[A0][0] = c.g[V1][0];
    c.mtc1(3, 0);
    cvt_w(c, 0, 0);
    c.lwc1(2, bx.wrapping_add(8));
    fadd(c, 1, 1, 3);
    c.f[5] = c.f[2];
    fsub(c, 0, 0, 3);
    fmul(c, 7, 1, 4);
    let ok = fcc_lt(c, 2, 7);
    fmul(c, 3, 0, 4);
    if ok {
        c.cycles += 5;
        c.lwc1(0, bx.wrapping_add(0x18));
        c.g[V0][0] = 1;
        if fcc_lt(c, 3, 0) {
            c.cycles += 1;
            let v = c.g[V0][0];
            st_sp(c, 0x18, v);
        }
    }
    // oz1 (z1 pair)
    c.cycles += 15;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(2));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d14));
    c.mtc1(3, 1);
    cvt_w(c, 1, 1);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.mtc1(2, 0);
    cvt_w(c, 0, 0);
    st_sp(c, 0x1c, 0);
    fadd(c, 1, 1, 2);
    fsub(c, 0, 0, 2);
    fmul(c, 1, 1, 4);
    let ok = fcc_lt(c, 5, 1);
    fmul(c, 2, 0, 4);
    if ok {
        c.cycles += 5;
        c.lwc1(0, bx.wrapping_add(0x18));
        c.g[V0][0] = 1;
        if fcc_lt(c, 2, 0) {
            c.cycles += 1;
            let v = c.g[V0][0];
            st_sp(c, 0x1c, v);
        }
    }
    // oz early exit.
    c.cycles += 3;
    c.g[V0][0] = ld_sp(c, 0x18);
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = ld_sp(c, 0x1c);
    if !t {
        c.cycles += 2;
        let miss = c.g[V0][0] == 0;
        restore_s0(c);
        if miss {
            return Ok(());
        }
    }
    // cx0 (x0 containment, into s6)
    c.cycles += 4;
    let ok = fcc_lt(c, 13, 15);
    c.g[S6][0] = 0;
    if ok {
        c.cycles += 5;
        c.lwc1(0, bx.wrapping_add(0x10));
        if fcc_lt(c, 16, 0) {
            c.g[S6][0] = 1;
        }
    }
    range_flag(c, 13, 12, 0x10, 14, 0x20); // cx1
    range_flag(c, 6, 10, 0x14, 11, 0x24); // cy0
    range_flag(c, 6, 8, 0x14, 9, 0x28); // cy1
    range_flag(c, 5, 3, 0x18, 7, 0x2c); // cz0
    range_flag(c, 5, 2, 0x18, 1, 0x30); // cz1

    octant0(c)?;
    octant1(c)?;
    octant3(c)?;
    octant2(c)?;
    octant6(c)?;
    octant7(c)?;
    octant5(c)?;
    octant4(c)
}

/// The body: the node's own objects, the leaf test at level 11, then the
/// octants. `Err` when a callee unwinds (the caller returns at once).
fn run(c: &mut Cpu) -> Flow {
    let f = Frame::open(c, 0xe0);
    c.cycles += 23;
    f.save(c, S3, 0xa0);
    f.save(c, S4, 0x90);
    c.g[S3][0] = c.g[T0][0];
    f.save(c, S5, 0x80);
    c.g[S4][0] = c.g[A1][0];
    f.save(c, S0, 0xd0);
    c.g[S5][0] = c.g[A0][0];
    f.save(c, S1, 0xc0);
    c.g[S0][0] = c.g[T1][0];
    f.save(c, S2, 0xb0);
    c.g[S1][0] = c.g[A2][0];
    c.g[S2][0] = c.g[A3][0];
    f.save(c, S6, 0x70);
    f.save(c, S7, 0x60);
    c.g[A0][0] = c.g[S4][0];
    f.save(c, FP, 0x50);
    f.save_ra(c, 0x40);

    gather_objects(c, S4, S3, 0x0033_a428, 0x0033_a438, 1)?;

    // A cell one level above the leaves stops after its own objects.
    c.cycles += 4;
    c.g[A1][0] = lw_reg(c, S0, 0);
    c.g[V0][0] = 0xb;
    let leaf = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
    if leaf {
        return finish_inner(c);
    }
    octants(c)
}

/// `epilogue`: restore the callee-saved registers, `ra`, and return.
fn epilogue(c: &mut Cpu) {
    let f = frame();
    c.cycles += 11;
    f.restore(c, S1, 0xc0);
    f.restore(c, S2, 0xb0);
    f.restore(c, S3, 0xa0);
    f.restore(c, S4, 0x90);
    f.restore(c, S5, 0x80);
    f.restore(c, S6, 0x70);
    f.restore(c, S7, 0x60);
    f.restore(c, FP, 0x50);
    f.restore_ra(c, 0x40);
    f.close(c);
}

/// `Cell_GatherObjInstRecursive(flag, ctx, ctx2, box, node, cell)`
/// (0x33a3d8, `nash_privy_QueryHelper_helpX3`): an octree walk for an
/// overlap query. The node's own objects go into `ctx`. A cell above the
/// leaves (level 11 stops) visits its eight children: a child whose cell lies
/// entirely in the box gathers its whole subtree into `ctx2`; a child that
/// only partly overlaps recurses with the child's key on the stack.
pub fn cell_gather_obj_inst_recursive(c: &mut Cpu) {
    guest(c, |c| {
        run(c)?;
        epilogue(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// The query box (`tBoundBox`): min x/y/z at +0, max at +0x10.
    const BOX: u32 = DATA;
    /// Callback contexts: array pointer at +0x10, count pointer at +0x14,
    /// box pointer at +0x1c.
    const CTX1: u32 = DATA + 0x100;
    const CTX2: u32 = DATA + 0x200;
    const COUNT1: u32 = DATA + 0x300;
    const COUNT2: u32 = DATA + 0x304;
    const ARR1: u32 = DATA + 0x400;
    const ARR2: u32 = DATA + 0x800;
    /// The `flag` argument (never written by the body).
    const FLAG: u32 = DATA + 0x180;
    /// The query cell `(level, x, y, z)`.
    const ROOT: u32 = DATA + 0xc00;
    /// The node: children at +0..0x1c, its object list at +0x24.
    const NODE: u32 = DATA + 0x1000;
    /// Eight child cells, 0x40 apart, each with an object list at +0x24.
    const CHILD: u32 = DATA + 0x1100;
    /// Up to six objects, 0x200 apart: list link at +0, bounds at +0x158.
    const OBJ: u32 = DATA + 0x2000;

    fn unit(r: &mut Rng) -> f32 {
        (r.next_u32() >> 8) as f32 / (1u32 << 24) as f32
    }

    fn put_f(c: &mut Cpu, a: u32, v: f32) {
        c.bus.write32(a, v.to_bits());
    }

    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed as u64);
        scramble(c, &mut r);
        // The query box, around the origin (cells are a few thousand wide).
        for axis in 0..3u32 {
            let lo = -9000.0 + 18000.0 * unit(&mut r);
            put_f(c, BOX + 4 * axis, lo);
            put_f(c, BOX + 0x10 + 4 * axis, lo + 6000.0 * unit(&mut r));
        }
        for (ctx, arr, cnt) in [(CTX1, ARR1, COUNT1), (CTX2, ARR2, COUNT2)] {
            c.bus.write32(ctx + 0x10, arr);
            c.bus.write32(ctx + 0x14, cnt);
            c.bus.write32(ctx + 0x1c, BOX);
            c.bus.write32(cnt, 0);
        }
        // Objects: a chain of up to six, bounds around the box.
        let n = seed % 7;
        for i in 0..n {
            let o = OBJ + 0x200 * i;
            c.bus.write32(o, if i + 1 < n { o + 0x200 } else { 0 });
            for axis in 0..3u32 {
                let lo = -9000.0 + 18000.0 * unit(&mut r);
                put_f(c, o + 0x158 + 4 * axis, lo);
                put_f(c, o + 0x164 + 4 * axis, lo + 3000.0 * unit(&mut r));
            }
        }
        // The query cell: level 12 (children are leaves), or 11 (stops).
        let level = if seed % 5 == 0 { 11 } else { 12 };
        c.bus.write32(ROOT, level);
        for axis in 0..3u32 {
            let v = (r.next_u32() % 3) as i32 - 1;
            c.bus.write32(ROOT + 4 + 4 * axis, v as u32);
        }
        // The node and its children; which children exist varies per seed.
        let mask = r.next_u32();
        for k in 0..8u32 {
            let child = CHILD + 0x40 * k;
            let present = (mask >> k) & 1 != 0;
            c.bus.write32(NODE + 4 * k, if present { child } else { 0 });
            c.bus.write32(child, 0);
            c.bus.write32(child + 0x24, if n > 0 { OBJ } else { 0 });
        }
        c.bus.write32(NODE + 0x24, if n > 0 { OBJ } else { 0 });

        c.g[A0][0] = sx(FLAG);
        c.g[A1][0] = sx(CTX1);
        c.g[A2][0] = sx(CTX2);
        c.g[A3][0] = sx(BOX);
        c.g[T0][0] = sx(NODE);
        c.g[T1][0] = sx(ROOT);
        c.pc = 0x0033_a3d8;
    }

    #[test]
    fn cell_gather_obj_inst_recursive_matches() {
        check(
            ee::s11::f_0033a3d8__generated,
            cell_gather_obj_inst_recursive,
            200,
            &|c, seed| setup(c, seed),
        );
    }
}
