//! Staging module oc753 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc753`.

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
    0x0037_2b78,
    bezier_patch_cache_init,
    "BezierPatchCache_Init",
)];

/// `operator new(size, name, 0, 0)` (named allocator).
const BPC_NEW: u32 = 0x0031_7e30;
/// `MEM_fill(ptr, value, size)`.
const BPC_MEM_FILL: u32 = 0x003e_6448;

// Fields of the cache object (`self`, in s3); +0x00 holds the three patch counts.
const BPC_REC_BUF: u32 = 0x0c; // [3] record arrays
const BPC_RECORDS: u32 = 0x18; // [3] pointers to them
const BPC_FREE_BUF: u32 = 0x24; // [3] free-list buffers
const BPC_FREE_LIST: u32 = 0x30; // [3]
const BPC_FREE_SP: u32 = 0x3c; // [3] free-list stack pointers
const BPC_PTS: u32 = 0x48; // [3] position caches
const BPC_UV_A: u32 = 0x54; // [3]
const BPC_UV_B: u32 = 0x60; // [3]
const BPC_BASIS4: u32 = 0x70;
const BPC_BASIS6: u32 = 0xb0;
const BPC_BASIS8: u32 = 0x110;
const BPC_BASIS: u32 = 0x190; // [3] pointers to the basis tables
const BPC_ORDER: u32 = 0x19c; // [3] 4, 6, 8
const BPC_NRM_SIZE: u32 = 0x1a8; // [3] normal cache sizes
const BPC_NRM: u32 = 0x1b4; // [3] normal caches
const BPC_NRM_FREE_BUF: u32 = 0x1c0; // [3]
const BPC_NRM_FREE_LIST: u32 = 0x1cc; // [3]
const BPC_NRM_FREE_SP: u32 = 0x1d8; // [3]
const BPC_DBASIS4: u32 = 0x1f0;
const BPC_DBASIS6: u32 = 0x230;
const BPC_DBASIS8: u32 = 0x290;
const BPC_DBASIS: u32 = 0x310; // [3]

#[inline]
fn bpc_rg(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `reg = sign_extend(v)`.
#[inline]
fn bpc_wr(c: &mut Cpu, r: usize, v: u32) {
    c.g[r][0] = sx(v);
}

/// The back-edge event check of the compiled loops.
#[inline]
fn bpc_edge(c: &mut Cpu, head: u32) {
    if event_due(c) {
        deliver_event_here(c, head);
    }
}

/// A free-list fill loop (`list[sp++] = i` for `i` in `a1..count`, the
/// stack pointer kept in memory): the loop at `head` over `count_reg`.
fn bpc_free_list_loop(c: &mut Cpu, head: u32, sp_off: u32, list_off: u32, count_reg: usize) {
    let this = bpc_rg(c, S3);
    loop {
        c.cycles += 10;
        c.g[V1][0] = c.lw(this.wrapping_add(sp_off));
        c.g[A0][0] = c.lw(this.wrapping_add(list_off));
        bpc_wr(c, V0, bpc_rg(c, V1) << 2);
        bpc_wr(c, V0, bpc_rg(c, V0).wrapping_add(bpc_rg(c, A0)));
        bpc_wr(c, V1, bpc_rg(c, V1).wrapping_add(1));
        c.sw(bpc_rg(c, V0), c.g[A1][0]);
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[count_reg][0] as i64)) as u64;
        let t = c.g[V0][0] != 0;
        c.sw(this.wrapping_add(sp_off), c.g[V1][0]);
        if t {
            bpc_edge(c, head);
            continue;
        }
        break;
    }
}

/// The element constructor loop of `new T[n]` (form with v0): entered with
/// `v1 = n - 1`, `a0 = -1`; the inner loop runs `inner` times per element.
fn bpc_ctor_loop_a(c: &mut Cpu, outer: u32, inner_head: u32, inner: u32) {
    'outer: loop {
        c.cycles += 4;
        bpc_wr(c, A1, bpc_rg(c, V1).wrapping_sub(1));
        c.g[V0][0] = inner as u64;
        bpc_wr(c, V1, inner - 1);
        loop {
            c.cycles += 8;
            c.g[V0][0] = c.g[V1][0];
            let t = c.g[V0][0] != c.g[A0][0];
            bpc_wr(c, V1, bpc_rg(c, V0).wrapping_sub(1));
            if t {
                bpc_edge(c, inner_head);
                continue;
            }
            c.cycles += 3;
            c.g[V1][0] = c.g[A1][0];
            let t = c.g[V1][0] != c.g[A0][0];
            if t {
                bpc_edge(c, outer);
                continue 'outer;
            }
            return;
        }
    }
}

/// The same loop as the normal-cache arrays compile it (a2/a0 for the
/// counters, `a1 = -1`): entered with `v1 = n - 1`, `a1 = -1`.
fn bpc_ctor_loop_b(c: &mut Cpu, outer: u32, inner_head: u32, inner: u32) {
    'outer: loop {
        c.cycles += 4;
        bpc_wr(c, A2, bpc_rg(c, V1).wrapping_sub(1));
        c.g[V1][0] = inner as u64;
        bpc_wr(c, A0, inner - 1);
        loop {
            c.cycles += 8;
            c.g[V1][0] = c.g[A0][0];
            if c.g[V1][0] != c.g[A1][0] {
                bpc_wr(c, A0, bpc_rg(c, V1).wrapping_sub(1));
                bpc_edge(c, inner_head);
                continue;
            }
            c.cycles += 3;
            c.g[V1][0] = c.g[A2][0];
            let t = c.g[V1][0] != c.g[A1][0];
            if t {
                bpc_edge(c, outer);
                continue 'outer;
            }
            return;
        }
    }
}

/// After an allocation in the first two cache families: `a2 = v0` (kept for
/// the next stage's store), then the constructor loop over `count_reg` unless
/// the count is 0 (`count - 1 == -1`).
fn bpc_ctor_tail(
    c: &mut Cpu,
    count_reg: usize,
    else_cycles: u64,
    outer: u32,
    inner_head: u32,
    inner: u32,
) {
    c.g[A2][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    let t = c.g[count_reg][0] == c.g[V0][0];
    c.g[V1][0] = c.g[count_reg][0];
    if t {
        return;
    }
    c.cycles += else_cycles;
    c.g[A0][0] = u64::MAX;
    bpc_ctor_loop_a(c, outer, inner_head, inner);
}

/// The same for the third family (v1 loaded before v0 is set to -1, no
/// extra cycles).
fn bpc_ctor_tail_late(c: &mut Cpu, count_reg: usize, outer: u32, inner_head: u32, inner: u32) {
    c.g[A2][0] = c.g[V0][0];
    c.g[V1][0] = c.g[count_reg][0];
    c.g[V0][0] = u64::MAX;
    let t = c.g[V1][0] == c.g[V0][0];
    c.g[A0][0] = u64::MAX;
    if t {
        return;
    }
    bpc_ctor_loop_a(c, outer, inner_head, inner);
}

/// The basis rows `(t^3, t^2, t, 1)` for `t = i * step` (step in f4, loaded
/// by the caller), `n` rows from `a0`, through the stack quadword.
fn bpc_basis_loop(c: &mut Cpu, head: u32, n: i64) {
    loop {
        c.cycles += 15;
        c.mtc1(5, 1);
        c.cvt_s_w(1, 1);
        c.swc1(3, sp(c).wrapping_add(0xc));
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < n) as u64;
        c.mul_s(2, 1, 4);
        c.mul_s(0, 2, 2);
        c.swc1(2, sp(c).wrapping_add(0x8));
        c.mul_s(1, 0, 2);
        c.swc1(0, sp(c).wrapping_add(0x4));
        c.swc1(1, sp(c));
        c.lq(2, sp(c));
        c.sq(2, bpc_rg(c, A0));
        let t = c.g[V1][0] != 0;
        bpc_wr(c, A0, bpc_rg(c, A0).wrapping_add(0x10));
        if t {
            bpc_edge(c, head);
            continue;
        }
        return;
    }
}

/// The basis derivative rows `(3t^2, 2t, 1, 0)` (f4 = 3.0, f5 = step).
fn bpc_dbasis_loop(c: &mut Cpu, head: u32, n: i64) {
    loop {
        c.cycles += 16;
        c.mtc1(5, 1);
        c.cvt_s_w(1, 1);
        c.swc1(3, sp(c).wrapping_add(0x8));
        c.sw(sp(c).wrapping_add(0xc), 0);
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < n) as u64;
        c.mul_s(2, 1, 5);
        c.mul_s(0, 2, 4);
        c.add_s(1, 2, 2);
        c.mul_s(0, 2, 0);
        c.swc1(1, sp(c).wrapping_add(0x4));
        c.swc1(0, sp(c));
        c.lq(2, sp(c));
        c.sq(2, bpc_rg(c, A0));
        let t = c.g[V1][0] != 0;
        bpc_wr(c, A0, bpc_rg(c, A0).wrapping_add(0x10));
        if t {
            bpc_edge(c, head);
            continue;
        }
        return;
    }
}

/// `BezierPatchCache_Init(self, n4, n6, n8, nn4, nn6, nn8)` (0x372b78): sets
/// up the patch cache for 4x4, 6x6 and 8x8 Bezier patches. Allocates the
/// record, free-list, position and two UV arrays per size (zero-filling the
/// records and constructing the position/UV arrays), the free lists, the
/// basis tables for `t = i / (order - 1)`, then the same for the normal caches
/// (`nn4`, `nn6`, `nn8` entries) with the derivative basis tables.
///
/// Registers: s3 = self, s4/s6/s5 = n4/n6/n8, s7/fp = nn4/nn6, nn8 on the
/// stack at 0x10. Stack frame 0x100 with locals 0x10..0x54.
pub fn bezier_patch_cache_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 94;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xf0);
        frame.save(c, S4, 0xb0);
        c.g[S0][0] = 0xc;
        frame.save(c, S2, 0xd0);
        c.g[S4][0] = c.g[A1][0];
        c.mult(0, S2, S4, S0);
        frame.save(c, S3, 0xc0);
        frame.save(c, S5, 0xa0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S6, 0x90);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S1, 0xe0);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S7, 0x80);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, FP, 0x70);
        c.g[S7][0] = c.g[T0][0];
        frame.save_ra(c, 0x60);
        c.g[FP][0] = c.g[T1][0];
        c.sw(bpc_rg(c, S3), c.g[S4][0]);
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2728));
        c.sw(bpc_rg(c, S3).wrapping_add(0x4), c.g[S6][0]);
        c.g[A0][0] = c.g[S2][0];
        c.sw(bpc_rg(c, S3).wrapping_add(0x8), c.g[S5][0]);
        c.g[A2][0] = 0;
        c.sw(sp(c).wrapping_add(0x10), c.g[T2][0]);
        c.g[A3][0] = 0;
        // m4x4 records
        call_ptr(c, BPC_NEW, 0x0037_2bec)?;
        c.mult(0, S1, S6, S0);
        bpc_wr(c, A1, 0x49_0000u32.wrapping_add(0x2738));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.sw(bpc_rg(c, S3).wrapping_add(BPC_REC_BUF), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        // m6x6 records
        call_ptr(c, BPC_NEW, 0x0037_2c0c)?;
        c.mult(0, S0, S5, S0);
        bpc_wr(c, A1, 0x49_0000u32.wrapping_add(0x2748));
        c.g[A3][0] = 0;
        c.g[A2][0] = 0;
        c.sw(bpc_rg(c, S3).wrapping_add(BPC_REC_BUF + 4), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        // m8x8 records
        call_ptr(c, BPC_NEW, 0x0037_2c2c)?;
        let this = bpc_rg(c, S3);
        c.g[V1][0] = c.lw(this.wrapping_add(BPC_REC_BUF + 4));
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.lw(this.wrapping_add(BPC_REC_BUF));
        c.g[A1][0] = 0;
        c.sw(this.wrapping_add(BPC_RECORDS + 4), c.g[V1][0]);
        c.g[A0][0] = c.g[A3][0];
        c.sw(this.wrapping_add(BPC_RECORDS + 8), c.g[V0][0]);
        c.sw(this.wrapping_add(BPC_REC_BUF + 8), c.g[V0][0]);
        c.sw(this.wrapping_add(BPC_RECORDS), c.g[A3][0]);
        call_ptr(c, BPC_MEM_FILL, 0x0037_2c54)?;
        let this = bpc_rg(c, S3);
        c.g[A0][0] = c.lw(this.wrapping_add(BPC_REC_BUF + 4));
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        call_ptr(c, BPC_MEM_FILL, 0x0037_2c64)?;
        let this = bpc_rg(c, S3);
        c.g[A0][0] = c.lw(this.wrapping_add(BPC_REC_BUF + 8));
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, BPC_MEM_FILL, 0x0037_2c74)?;
        // free-list buffers
        bpc_wr(c, A0, bpc_rg(c, S4) << 2);
        bpc_wr(c, A1, 0x49_0000u32.wrapping_add(0x2758));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, BPC_NEW, 0x0037_2c8c)?;
        bpc_wr(c, A0, bpc_rg(c, S6) << 2);
        bpc_wr(c, A1, 0x49_0000u32.wrapping_add(0x2768));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.sw(bpc_rg(c, S3).wrapping_add(BPC_FREE_BUF), c.g[V0][0]);
        call_ptr(c, BPC_NEW, 0x0037_2ca8)?;
        bpc_wr(c, A0, bpc_rg(c, S5) << 2);
        bpc_wr(c, A1, 0x49_0000u32.wrapping_add(0x2778));
        c.sw(bpc_rg(c, S3).wrapping_add(BPC_FREE_BUF + 4), c.g[V0][0]);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, BPC_NEW, 0x0037_2cc4)?;
        let this = bpc_rg(c, S3);
        c.g[V1][0] = c.lw(this.wrapping_add(BPC_FREE_BUF));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lw(this.wrapping_add(BPC_FREE_BUF + 4));
        c.sw(this.wrapping_add(BPC_FREE_LIST), c.g[V1][0]);
        c.sw(this.wrapping_add(BPC_FREE_LIST + 4), c.g[A0][0]);
        c.sw(this.wrapping_add(BPC_FREE_LIST + 8), c.g[V0][0]);
        c.sw(this.wrapping_add(BPC_FREE_BUF + 8), c.g[V0][0]);
        c.sw(this.wrapping_add(BPC_FREE_SP), 0);
        c.sw(this.wrapping_add(BPC_FREE_SP + 4), 0);
        let skip = (c.g[S4][0] as i64) <= 0;
        c.sw(this.wrapping_add(BPC_FREE_SP + 8), 0);
        if !skip {
            bpc_free_list_loop(c, 0x0037_2cf0, BPC_FREE_SP, BPC_FREE_LIST, S4);
        }

        // Addresses of the cache slots, kept in the frame for the stores
        // after the allocations.
        c.cycles += 39;
        let this = bpc_rg(c, S3);
        macro_rules! sl {
            ($c:expr, $off:expr, $v:expr) => {{
                let v = $v;
                $c.sw(sp($c).wrapping_add($off), v as u64)
            }};
        }
        bpc_wr(c, V0, this.wrapping_add(BPC_PTS));
        bpc_wr(c, V1, this.wrapping_add(BPC_PTS + 4));
        sl!(c, 0x2c, bpc_rg(c, V0));
        bpc_wr(c, A0, this.wrapping_add(BPC_PTS + 8));
        sl!(c, 0x30, bpc_rg(c, V1));
        bpc_wr(c, V0, this.wrapping_add(BPC_UV_A));
        bpc_wr(c, V1, bpc_rg(c, S4) << 7);
        sl!(c, 0x34, bpc_rg(c, A0));
        sl!(c, 0x38, bpc_rg(c, V0));
        bpc_wr(c, A0, this.wrapping_add(BPC_UV_A + 4));
        sl!(c, 0x24, bpc_rg(c, V1));
        bpc_wr(c, V0, this.wrapping_add(BPC_UV_A + 8));
        bpc_wr(c, V1, bpc_rg(c, S5) << 9);
        sl!(c, 0x3c, bpc_rg(c, A0));
        sl!(c, 0x40, bpc_rg(c, V0));
        bpc_wr(c, A0, this.wrapping_add(BPC_UV_B));
        sl!(c, 0x28, bpc_rg(c, V1));
        bpc_wr(c, V0, this.wrapping_add(BPC_UV_B + 4));
        bpc_wr(c, V1, this.wrapping_add(BPC_UV_B + 8));
        sl!(c, 0x44, bpc_rg(c, A0));
        sl!(c, 0x48, bpc_rg(c, V0));
        bpc_wr(c, A0, this.wrapping_add(BPC_BASIS4));
        sl!(c, 0x4c, bpc_rg(c, V1));
        bpc_wr(c, V0, this.wrapping_add(BPC_BASIS6));
        bpc_wr(c, V1, this.wrapping_add(BPC_BASIS8));
        sl!(c, 0x50, bpc_rg(c, A0));
        sl!(c, 0x14, bpc_rg(c, V0));
        bpc_wr(c, A0, this.wrapping_add(BPC_DBASIS4));
        sl!(c, 0x1c, bpc_rg(c, V1));
        bpc_wr(c, V0, this.wrapping_add(BPC_DBASIS6));
        bpc_wr(c, V1, this.wrapping_add(BPC_DBASIS8));
        c.g[A1][0] = 0;
        bpc_wr(c, S0, bpc_rg(c, S4).wrapping_sub(1));
        bpc_wr(c, S1, bpc_rg(c, S6).wrapping_sub(1));
        bpc_wr(c, S2, bpc_rg(c, S5).wrapping_sub(1));
        sl!(c, 0x54, bpc_rg(c, A0));
        sl!(c, 0x18, bpc_rg(c, V0));
        let skip = (c.g[S6][0] as i64) <= 0;
        sl!(c, 0x20, bpc_rg(c, V1));
        if !skip {
            c.cycles += 1;
            bpc_free_list_loop(c, 0x0037_2db8, BPC_FREE_SP + 4, BPC_FREE_LIST + 4, S6);
        }
        c.cycles += 2;
        let skip = (c.g[S5][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !skip {
            bpc_free_list_loop(c, 0x0037_2de8, BPC_FREE_SP + 8, BPC_FREE_LIST + 8, S5);
        }

        // Position cache arrays (4x4, 6x6, 8x8) with their constructors.
        c.cycles += 12;
        c.g[A0][0] = 0xc0;
        c.mult(0, 0, S4, A0);
        c.g[A2][0] = 0;
        bpc_wr(c, A1, 0x49_0000u32.wrapping_add(0x2788));
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        call_ptr(c, BPC_NEW, 0x0037_2e30)?;
        bpc_ctor_tail(c, S0, 2, 0x0037_2e48, 0x0037_2e58, 3);

        c.cycles += 14;
        c.g[A0][0] = 0x1b0;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x2c));
        c.mult(0, 0, S6, A0);
        c.g[A1][0] = 0x49_0000;
        c.sw(bpc_rg(c, V0), c.g[A2][0]);
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2798));
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        call_ptr(c, BPC_NEW, 0x0037_2eac)?;
        bpc_ctor_tail(c, S1, 1, 0x0037_2ec0, 0x0037_2ed0, 5);

        c.cycles += 14;
        c.g[A0][0] = 0x300;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x30));
        c.mult(0, 0, S5, A0);
        c.g[A1][0] = 0x49_0000;
        c.sw(bpc_rg(c, V0), c.g[A2][0]);
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x27a8));
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        call_ptr(c, BPC_NEW, 0x0037_2f24)?;
        bpc_ctor_tail(c, S2, 1, 0x0037_2f38, 0x0037_2f48, 7);

        // UV A arrays.
        c.cycles += 12;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x34));
        c.g[A1][0] = 0x49_0000;
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x27b8));
        c.g[A3][0] = 0;
        c.sw(bpc_rg(c, V0), c.g[A2][0]);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw(sp(c).wrapping_add(0x24));
        call_ptr(c, BPC_NEW, 0x0037_2f94)?;
        bpc_ctor_tail(c, S0, 1, 0x0037_2fa8, 0x0037_2fb8, 3);

        c.cycles += 14;
        c.g[A0][0] = 0x120;
        c.g[V1][0] = c.lw(sp(c).wrapping_add(0x38));
        c.mult(0, 0, S6, A0);
        c.g[A1][0] = 0x49_0000;
        c.sw(bpc_rg(c, V1), c.g[A2][0]);
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x27c8));
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        call_ptr(c, BPC_NEW, 0x0037_300c)?;
        bpc_ctor_tail(c, S1, 1, 0x0037_3020, 0x0037_3030, 5);

        c.cycles += 12;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x3c));
        c.g[A1][0] = 0x49_0000;
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x27d8));
        c.g[A3][0] = 0;
        c.sw(bpc_rg(c, V0), c.g[A2][0]);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw(sp(c).wrapping_add(0x28));
        call_ptr(c, BPC_NEW, 0x0037_307c)?;
        bpc_ctor_tail(c, S2, 1, 0x0037_3090, 0x0037_30a0, 7);

        // UV B arrays.
        c.cycles += 13;
        c.g[V1][0] = c.lw(sp(c).wrapping_add(0x40));
        c.g[A1][0] = 0x49_0000;
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x27e8));
        c.g[A3][0] = 0;
        c.sw(bpc_rg(c, V1), c.g[A2][0]);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw(sp(c).wrapping_add(0x24));
        call_ptr(c, BPC_NEW, 0x0037_30ec)?;
        bpc_ctor_tail_late(c, S0, 0x0037_3100, 0x0037_3110, 3);

        c.cycles += 15;
        c.g[A0][0] = 0x120;
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x44));
        c.mult(0, 0, S6, A0);
        c.g[A3][0] = 0;
        c.sw(bpc_rg(c, A1), c.g[A2][0]);
        c.g[A1][0] = 0x49_0000;
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lo[0];
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x27f8));
        call_ptr(c, BPC_NEW, 0x0037_3164)?;
        bpc_ctor_tail_late(c, S1, 0x0037_3178, 0x0037_3188, 5);

        c.cycles += 13;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x48));
        c.g[A1][0] = 0x49_0000;
        bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2808));
        c.g[A3][0] = 0;
        c.sw(bpc_rg(c, V0), c.g[A2][0]);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lw(sp(c).wrapping_add(0x28));
        call_ptr(c, BPC_NEW, 0x0037_31d4)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = c.g[S2][0];
        c.g[V0][0] = u64::MAX;
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
            bpc_ctor_loop_a(c, 0x0037_31f0, 0x0037_3200, 7);
            c.cycles += 1;
        }
        c.g[V1][0] = c.lw(sp(c).wrapping_add(0x4c));

        // Basis tables: rows (t^3, t^2, t, 1), t = i * step.
        c.cycles += 6;
        c.g[A1][0] = 0;
        c.lwc1(4, bpc_rg(c, GP).wrapping_sub(0x2810));
        bpc_wr(c, A0, bpc_rg(c, S3).wrapping_add(BPC_BASIS4));
        c.sw(bpc_rg(c, V1), c.g[A2][0]);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        bpc_basis_loop(c, 0x0037_3248, 4);

        c.cycles += 5;
        c.lwc1(4, bpc_rg(c, GP).wrapping_sub(0x280c));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        bpc_wr(c, A0, bpc_rg(c, S3).wrapping_add(BPC_BASIS6));
        bpc_basis_loop(c, 0x0037_3298, 6);

        c.cycles += 5;
        c.lwc1(4, bpc_rg(c, GP).wrapping_sub(0x2808));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        bpc_wr(c, A0, bpc_rg(c, S3).wrapping_add(BPC_BASIS8));
        bpc_basis_loop(c, 0x0037_32e8, 8);

        // Basis table pointers, orders and normal cache sizes.
        c.cycles += 19;
        let this = bpc_rg(c, S3);
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x50));
        c.g[V1][0] = 0x6;
        c.g[A0][0] = 0x8;
        c.g[V0][0] = 0x4;
        c.sw(this.wrapping_add(BPC_BASIS), c.g[A1][0]);
        c.g[A2][0] = 0;
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x14));
        c.sw(this.wrapping_add(BPC_BASIS + 4), c.g[A1][0]);
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x1c));
        c.sw(this.wrapping_add(BPC_ORDER + 4), c.g[V1][0]);
        c.sw(this.wrapping_add(BPC_BASIS + 8), c.g[A1][0]);
        c.sw(this.wrapping_add(BPC_ORDER + 8), c.g[A0][0]);
        c.sw(this.wrapping_add(BPC_NRM_SIZE), c.g[S7][0]);
        c.sw(this.wrapping_add(BPC_NRM_SIZE + 4), c.g[FP][0]);
        c.sw(this.wrapping_add(BPC_ORDER), c.g[V0][0]);
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
        c.sw(this.wrapping_add(BPC_NRM_SIZE + 8), c.g[V0][0]);
        bpc_wr(c, A1, bpc_rg(c, A2) << 2);

        // Mark every record's field 8 as -1: for each size k, records[k][j].
        loop {
            let this = bpc_rg(c, S3);
            c.cycles += 5;
            c.g[T1][0] = sx(bpc_rg(c, A2).wrapping_add(1));
            bpc_wr(c, V1, this.wrapping_add(bpc_rg(c, A1)));
            c.g[V0][0] = c.lw(bpc_rg(c, V1));
            let skip = (c.g[V0][0] as i64) <= 0;
            c.g[A2][0] = 0;
            if !skip {
                c.cycles += 5;
                bpc_wr(c, T0, this.wrapping_add(BPC_RECORDS));
                c.g[T2][0] = u64::MAX;
                c.g[A3][0] = 0;
                bpc_wr(c, V1, bpc_rg(c, T0).wrapping_add(bpc_rg(c, A1)));
                loop {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[V0][0] = c.lw(bpc_rg(c, V1));
                    bpc_wr(c, A0, this.wrapping_add(bpc_rg(c, A0)));
                    bpc_wr(c, A2, bpc_rg(c, A2).wrapping_add(1));
                    bpc_wr(c, V0, bpc_rg(c, A3).wrapping_add(bpc_rg(c, V0)));
                    c.sw(bpc_rg(c, V0).wrapping_add(8), c.g[T2][0]);
                    c.g[V0][0] = c.lw(bpc_rg(c, A0));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    bpc_wr(c, A3, bpc_rg(c, A3).wrapping_add(0xc));
                    if t {
                        bpc_edge(c, 0x0037_3398);
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 4;
            c.g[A2][0] = c.g[T1][0];
            c.g[V0][0] = ((c.g[A2][0] as i64) < 3) as u64;
            if c.g[V0][0] != 0 {
                bpc_wr(c, A1, bpc_rg(c, A2) << 2);
                bpc_edge(c, 0x0037_3370);
                continue;
            }
            break;
        }

        // Normal position caches (3 sizes), only when their count is > 0.
        c.cycles += 2;
        let skip = (c.g[S7][0] as i64) <= 0;
        c.g[A0][0] = 0xc0;
        if !skip {
            c.cycles += 10;
            c.g[A1][0] = 0x49_0000;
            c.mult(0, 0, S7, A0);
            bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2818));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lo[0];
            bpc_wr(c, S0, bpc_rg(c, S3).wrapping_add(BPC_NRM));
            call_ptr(c, BPC_NEW, 0x0037_33f8)?;
            let zero = c.g[S7][0] == 0;
            bpc_wr(c, V1, bpc_rg(c, S7).wrapping_sub(1));
            if !zero {
                c.cycles += 2;
                c.g[A1][0] = u64::MAX;
                bpc_ctor_loop_b(c, 0x0037_3408, 0x0037_3418, 3);
            }
            c.cycles += 1;
            c.sw(bpc_rg(c, S0), c.g[V0][0]);
        }

        c.cycles += 2;
        let skip = (c.g[FP][0] as i64) <= 0;
        c.g[A0][0] = 0x1b0;
        if !skip {
            c.cycles += 10;
            c.g[A1][0] = 0x49_0000;
            c.mult(0, 0, FP, A0);
            bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2828));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lo[0];
            bpc_wr(c, S0, bpc_rg(c, S3).wrapping_add(BPC_NRM + 4));
            call_ptr(c, BPC_NEW, 0x0037_3470)?;
            let zero = c.g[FP][0] == 0;
            bpc_wr(c, V1, bpc_rg(c, FP).wrapping_sub(1));
            if !zero {
                c.cycles += 2;
                c.g[A1][0] = u64::MAX;
                bpc_ctor_loop_b(c, 0x0037_3480, 0x0037_3490, 5);
            }
            c.cycles += 1;
            c.sw(bpc_rg(c, S0), c.g[V0][0]);
        }

        c.cycles += 3;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
        let skip = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = 0x300;
        if !skip {
            c.cycles += 11;
            c.g[A1][0] = 0x49_0000;
            c.mult(0, 0, V0, A0);
            bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2838));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[A0][0] = c.lo[0];
            bpc_wr(c, S0, bpc_rg(c, S3).wrapping_add(BPC_NRM + 8));
            call_ptr(c, BPC_NEW, 0x0037_34ec)?;
            c.g[A0][0] = c.lw(sp(c).wrapping_add(0x10));
            let zero = c.g[A0][0] == 0;
            bpc_wr(c, V1, bpc_rg(c, A0).wrapping_sub(1));
            if !zero {
                c.cycles += 2;
                c.g[A1][0] = u64::MAX;
                bpc_ctor_loop_b(c, 0x0037_3500, 0x0037_3510, 7);
            }
            c.cycles += 1;
            c.sw(bpc_rg(c, S0), c.g[V0][0]);
        }

        // Normal free-list buffers.
        c.cycles += 2;
        let skip = (c.g[S7][0] as i64) <= 0;
        c.g[A1][0] = 0x49_0000;
        if !skip {
            c.cycles += 6;
            bpc_wr(c, A0, bpc_rg(c, S7) << 2);
            bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2848));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, BPC_NEW, 0x0037_355c)?;
            c.sw(bpc_rg(c, S3).wrapping_add(BPC_NRM_FREE_BUF), c.g[V0][0]);
        }
        c.cycles += 2;
        let skip = (c.g[FP][0] as i64) <= 0;
        c.g[A1][0] = 0x49_0000;
        if !skip {
            c.cycles += 6;
            bpc_wr(c, A0, bpc_rg(c, FP) << 2);
            bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2858));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, BPC_NEW, 0x0037_357c)?;
            c.sw(bpc_rg(c, S3).wrapping_add(BPC_NRM_FREE_BUF + 4), c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
        let skip = (c.g[A1][0] as i64) <= 0;
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
        if !skip {
            c.cycles += 7;
            c.g[A1][0] = 0x49_0000;
            bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(0x2868));
            c.g[A2][0] = 0;
            bpc_wr(c, A0, bpc_rg(c, V0) << 2);
            c.g[A3][0] = 0;
            call_ptr(c, BPC_NEW, 0x0037_35a4)?;
            c.sw(bpc_rg(c, S3).wrapping_add(BPC_NRM_FREE_BUF + 8), c.g[V0][0]);
        }

        // Normal free lists.
        c.cycles += 11;
        let this = bpc_rg(c, S3);
        c.g[V0][0] = c.lw(this.wrapping_add(BPC_NRM_FREE_BUF));
        c.g[A1][0] = 0;
        c.g[V1][0] = c.lw(this.wrapping_add(BPC_NRM_FREE_BUF + 4));
        c.g[A0][0] = c.lw(this.wrapping_add(BPC_NRM_FREE_BUF + 8));
        c.sw(this.wrapping_add(BPC_NRM_FREE_LIST), c.g[V0][0]);
        c.sw(this.wrapping_add(BPC_NRM_FREE_LIST + 4), c.g[V1][0]);
        c.sw(this.wrapping_add(BPC_NRM_FREE_LIST + 8), c.g[A0][0]);
        c.sw(this.wrapping_add(BPC_NRM_FREE_SP), 0);
        c.sw(this.wrapping_add(BPC_NRM_FREE_SP + 4), 0);
        let skip = (c.g[S7][0] as i64) <= 0;
        c.sw(this.wrapping_add(BPC_NRM_FREE_SP + 8), 0);
        if !skip {
            c.cycles += 1;
            bpc_free_list_loop(c, 0x0037_35d8, BPC_NRM_FREE_SP, BPC_NRM_FREE_LIST, S7);
        }
        c.cycles += 2;
        let skip = (c.g[FP][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !skip {
            bpc_free_list_loop(
                c,
                0x0037_3608,
                BPC_NRM_FREE_SP + 4,
                BPC_NRM_FREE_LIST + 4,
                FP,
            );
        }
        c.cycles += 3;
        c.g[V1][0] = c.lw(sp(c).wrapping_add(0x10));
        let skip = (c.g[V1][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !skip {
            c.cycles += 1;
            let this = bpc_rg(c, S3);
            c.g[V1][0] = c.lw(this.wrapping_add(BPC_NRM_FREE_SP + 8));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.lw(this.wrapping_add(BPC_NRM_FREE_LIST + 8));
                bpc_wr(c, V0, bpc_rg(c, V1) << 2);
                bpc_wr(c, V0, bpc_rg(c, V0).wrapping_add(bpc_rg(c, A0)));
                bpc_wr(c, V1, bpc_rg(c, V1).wrapping_add(1));
                c.sw(bpc_rg(c, V0), c.g[A1][0]);
                c.sw(this.wrapping_add(BPC_NRM_FREE_SP + 8), c.g[V1][0]);
                bpc_wr(c, A1, bpc_rg(c, A1).wrapping_add(1));
                c.g[A0][0] = c.lw(sp(c).wrapping_add(0x10));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.lw(this.wrapping_add(BPC_NRM_FREE_SP + 8));
                    bpc_edge(c, 0x0037_3640);
                    continue;
                }
                break;
            }
        }

        // Derivative basis tables: rows (3t^2, 2t, 1, 0).
        c.cycles += 7;
        c.lwc1(5, bpc_rg(c, GP).wrapping_sub(0x2804));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(1, 4);
        bpc_wr(c, A0, bpc_rg(c, S3).wrapping_add(BPC_DBASIS4));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        bpc_dbasis_loop(c, 0x0037_3688, 4);

        c.cycles += 8;
        c.lwc1(5, bpc_rg(c, GP).wrapping_sub(0x2800));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(1, 4);
        bpc_wr(c, A0, bpc_rg(c, S3).wrapping_add(BPC_DBASIS6));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        bpc_dbasis_loop(c, 0x0037_36e8, 6);

        c.cycles += 8;
        c.lwc1(5, bpc_rg(c, GP).wrapping_sub(0x27fc));
        c.g[A1][0] = 0;
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(1, 4);
        bpc_wr(c, A0, bpc_rg(c, S3).wrapping_add(BPC_DBASIS8));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 3);
        bpc_dbasis_loop(c, 0x0037_3748, 8);

        // dbasis[] pointers, then the epilogue.
        c.cycles += 18;
        let this = bpc_rg(c, S3);
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x20));
        frame.restore(c, S0, 0xf0);
        c.sw(this.wrapping_add(BPC_DBASIS + 8), c.g[A1][0]);
        frame.restore(c, S1, 0xe0);
        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x54));
        frame.restore(c, S2, 0xd0);
        c.sw(this.wrapping_add(BPC_DBASIS), c.g[V0][0]);
        frame.restore(c, S4, 0xb0);
        c.g[V1][0] = c.lw(sp(c).wrapping_add(0x18));
        frame.restore(c, S5, 0xa0);
        c.sw(this.wrapping_add(BPC_DBASIS + 4), c.g[V1][0]);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S6, 0x90);
        frame.restore(c, S7, 0x80);
        frame.restore(c, FP, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, fakes, DATA, GAME_GP};
    use crate::game::be::beintstat::tests::oc673_machine;

    /// The cache object (outside the fakes' allocation range
    /// `DATA..DATA+0x40000`).
    const BPC_OBJ: u32 = DATA + 0x8_0000;

    /// A cache init with random patch counts 0..4 per size and normal-cache
    /// counts -2..3 (negative and zero skip the normal caches); the basis
    /// steps 1/3, 1/5, 1/7 in the global pool. All allocations and fills are
    /// fakes; one of them unwinds on every third seed.
    fn bpc_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc673_machine(c, seed, 0x372b78, 24, 0x0037_2b78);
        for (off, v) in [
            (0x2810, 0x3eaa_aaab),
            (0x280c, 0x3e4c_cccd),
            (0x2808, 0x3e12_4925),
            (0x2804, 0x3eaa_aaab),
            (0x2800, 0x3e4c_cccd),
            (0x27fc, 0x3e12_4925),
        ] {
            c.bus.write32(GAME_GP - off, v);
        }
        c.g[A0][0] = sx(BPC_OBJ);
        for reg in [A1, A2, A3] {
            c.g[reg][0] = sx(r.next_u32() % 5);
        }
        for reg in [T0, T1, T2] {
            c.g[reg][0] = sx((r.next_u32() % 6).wrapping_sub(2));
        }
    }

    /// `BezierPatchCache_Init` against the generated body: patch counts 0 to
    /// 4 per size (empty arrays, the constructor loops, the free lists, the
    /// record marking), normal caches absent, zero or present, basis and
    /// derivative tables, unwinds in each allocation and fill call.
    #[test]
    fn bezier_patch_cache_init_matches() {
        fakes::covered(
            ee::s12::f_00372b78__generated,
            bezier_patch_cache_init,
            150,
            &bpc_setup,
        );
    }

    /// The same fixture with LO/HI compared (the size multiplies).
    #[test]
    fn bezier_patch_cache_init_lohi_matches() {
        check_lohi(
            ee::s12::f_00372b78__generated,
            bezier_patch_cache_init,
            60,
            &bpc_setup,
        );
    }
}
