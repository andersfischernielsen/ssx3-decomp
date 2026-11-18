//! Staging module oc415 (wave wave218): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc415`.

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
    0x0034_b228,
    c_flag_manager_group_build_mesh,
    "cFlagManager_GroupBuildMesh",
)];

/// `rand` (`BXrand`, 0x3177f0): the next random word.
pub(crate) const GROUP_BUILD_MESH_RAND: u32 = 0x0031_77f0;
/// `operator new[](size, name, flags, 0)` (0x317e30).
pub(crate) const GROUP_BUILD_MESH_NEW_NAMED: u32 = 0x0031_7e30;
/// `cFlagManager_GroupComputeVerts` (0x34bca0).
pub(crate) const GROUP_BUILD_MESH_COMPUTE_VERTS: Callee = Callee {
    addr: 0x0034_bca0,
    f: ee::s11::cFlagManager_GroupComputeVerts,
};
/// The flag manager pointer (`gp+0x2a90`); its mesh-manager vtable is at `+0x10d8`.
pub(crate) const GROUP_BUILD_MESH_MANAGER_GP: u32 = 0x2a90;
/// Group fields (`self`): the four wave phases (`+0x78`), the texture scroll
/// `u`/`v` (`+0x88`/`+0x8c`), the wind amounts (`+0x1c`/`+0x20`), the mesh
/// (`+0x60`), its columns/rows (`+0x64`/`+0x68`), their spans minus one as
/// floats (`+0x6c`/`+0x70`) and the vertex array (`+0x74`).
pub(crate) const GROUP_BUILD_MESH_PHASES: u32 = 0x78;
pub(crate) const GROUP_BUILD_MESH_WIND_X: u32 = 0x1c;
pub(crate) const GROUP_BUILD_MESH_WIND_Y: u32 = 0x20;
pub(crate) const GROUP_BUILD_MESH_MESH: u32 = 0x60;
pub(crate) const GROUP_BUILD_MESH_COLS: u32 = 0x64;
pub(crate) const GROUP_BUILD_MESH_ROWS: u32 = 0x68;
pub(crate) const GROUP_BUILD_MESH_COL_SPAN: u32 = 0x6c;
pub(crate) const GROUP_BUILD_MESH_ROW_SPAN: u32 = 0x70;
pub(crate) const GROUP_BUILD_MESH_VERTS: u32 = 0x74;

/// `reg + off` as a guest address (`addiu`/load-store base).
#[inline(always)]
fn bm_ad(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `void cFlagManager_GroupBuildMesh(group, item)` (0x34b228): set up a flag
/// group's cloth mesh. Draws four random phases in [0, 1) (`+0x78`), clears
/// the scroll, asks the mesh manager (vtable slot at `+0x308`) for the
/// item's quad (positions at `sp+0x60`, texture coordinates at `sp+0x20`,
/// colours at `sp`), picks an 8x2 grid (calm wind) or an 8x5 grid, creates
/// the mesh (slot `+0x260`; none means done) and the vertex array, bilinearly
/// interpolates positions, texture coordinates and colours (alpha 1.0) over
/// the grid, lets `cFlagManager_GroupComputeVerts` animate the vertices and
/// hands them to the mesh (mesh vtable slot `+0x30`).
pub fn c_flag_manager_group_build_mesh(c: &mut Cpu) {
    /// Loop heads, for event delivery.
    const PHASE_HEAD: u32 = 0x0034_b280;
    const SPIN4_HEAD: u32 = 0x0034_b2d0;
    const SPIN50_HEAD: u32 = 0x0034_b3f8;
    const ROW_HEAD: u32 = 0x0034_b428;
    const COLUMN_HEAD: u32 = 0x0034_b450;
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x900);
        frame.save(c, S2, 0x8c0);
        frame.save(c, S3, 0x8b0);
        c.g[S2][0] = 0x7f_0000;
        frame.save_fpr(c, 20, 0x8f0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x8e0);
        c.g[S2][0] |= 0xffff;
        frame.save(c, S1, 0x8d0);
        c.g[S0][0] = sx(r32(c, S3).wrapping_add(GROUP_BUILD_MESH_PHASES));
        frame.save(c, S4, 0x8a0);
        c.g[S1][0] = 3;
        frame.save(c, S5, 0x890);
        c.g[S4][0] = 0x3f80_0000;
        frame.save(c, S6, 0x880);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S7, 0x870);
        c.g[S6][0] = sx(sp(c).wrapping_add(0x60));
        frame.save_ra(c, 0x860);
        c.g[S7][0] = sx(sp(c).wrapping_add(0x20));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 20);
        // phase[i] = rand01() = (float)(1.0 | rand & 0x7fffff) - 1.0.
        loop {
            c.cycles += 9;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            call_ptr(c, GROUP_BUILD_MESH_RAND, 0x0034_b288)?;
            c.g[V0][0] &= c.g[S2][0];
            c.g[V0][0] |= c.g[S4][0];
            c.mtc1(V0, 1);
            c.sub_s(0, 1, 20);
            c.swc1(0, r32(c, S0));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, PHASE_HEAD);
            }
        }
        c.cycles += 11;
        c.sw(bm_ad(c, S3, 0x88), 0);
        c.g[V0][0] = 3;
        c.sw(bm_ad(c, S3, 0x8c), 0);
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = c.lw(bm_ad(c, GP, GROUP_BUILD_MESH_MANAGER_GP));
        c.g[A2][0] = c.lw(bm_ad(c, S0, 0x10d8));
        c.g[V0][0] = 3;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 3;
        c.g[V1][0] = u64::MAX;
        // The compiler's constructor loop of the four quad vertices (empty).
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[A1][0] = c.g[S5][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, SPIN4_HEAD);
            }
        }
        // item->GetQuad(0, pos, uv, col).
        c.cycles += 14;
        c.g[A0][0] = c.lh(bm_ad(c, A2, 0x308));
        c.g[V0][0] = c.lw(bm_ad(c, A2, 0x30c));
        c.g[A3][0] = c.g[S6][0];
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        c.g[T1][0] = c.g[S7][0];
        c.g[A2][0] = 0;
        let target = r32(c, V0);
        c.g[T0][0] = c.g[SP][0];
        call_ptr(c, target, 0x0034_b30c)?;
        // The grid: 8x2 in a calm wind (both amounts below the threshold), else 8x5.
        c.lwc1(1, bm_ad(c, GP, 0u32.wrapping_sub(0x2b04)));
        c.lwc1(0, bm_ad(c, S3, GROUP_BUILD_MESH_WIND_X));
        c.c_lt_s(0, 1);
        let windy_x = !c.fcc();
        c.g[V0][0] = 8;
        let mut calm = false;
        if !windy_x {
            c.cycles += 5;
            c.lwc1(0, bm_ad(c, S3, GROUP_BUILD_MESH_WIND_Y));
            c.c_lt_s(0, 1);
            let calm_y = c.fcc();
            c.g[V1][0] = 2;
            calm = calm_y;
        }
        if !calm {
            c.cycles += 1;
            c.g[V1][0] = 5;
        }
        'finish: {
            c.cycles += 20;
            c.sw(bm_ad(c, S3, GROUP_BUILD_MESH_COLS), c.g[V0][0]);
            c.sw(bm_ad(c, S3, GROUP_BUILD_MESH_ROWS), c.g[V1][0]);
            c.g[A1][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_COLS));
            c.g[A3][0] = 0x6000_0000;
            c.g[A2][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_ROWS));
            c.g[V1][0] = sx(r32(c, A1).wrapping_sub(1));
            c.g[V0][0] = sx(r32(c, A2).wrapping_sub(1));
            c.mtc1(V1, 0);
            c.cvt_s_w(0, 0);
            c.mtc1(V0, 1);
            c.cvt_s_w(1, 1);
            c.swc1(0, bm_ad(c, S3, GROUP_BUILD_MESH_COL_SPAN));
            c.swc1(1, bm_ad(c, S3, GROUP_BUILD_MESH_ROW_SPAN));
            // mesh = manager->CreateMesh(cols, rows, 0x60000000).
            c.g[V1][0] = c.lw(bm_ad(c, S0, 0x10d8));
            c.g[A0][0] = c.lh(bm_ad(c, V1, 0x260));
            c.g[V0][0] = c.lw(bm_ad(c, V1, 0x264));
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0034_b384)?;
            let no_mesh = c.g[V0][0] == 0;
            c.sw(bm_ad(c, S3, GROUP_BUILD_MESH_MESH), c.g[V0][0]);
            if no_mesh {
                break 'finish;
            }
            // verts = new("Flag_Verts", 0x20000000) V3[cols * rows].
            c.cycles += 17;
            c.g[S0][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_COLS));
            c.g[A0][0] = 0xc;
            c.g[V0][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_ROWS));
            c.g[A1][0] = 0x49_0000;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x17b0));
            c.g[A2][0] = 0x2000_0000;
            c.mult(0, S0, S0, V0);
            c.g[A3][0] = 0;
            c.mult(0, 0, S0, A0);
            c.g[A0][0] = c.lo[0];
            c.g[S1][0] = sx(r32(c, S3).wrapping_add(GROUP_BUILD_MESH_VERTS));
            call_ptr(c, GROUP_BUILD_MESH_NEW_NAMED, 0x0034_b3bc)?;
            c.g[S2][0] = sx(sp(c).wrapping_add(0x220));
            c.g[S4][0] = sx(sp(c).wrapping_add(0x600));
            c.g[S5][0] = sx(sp(c).wrapping_add(0x90));
            let empty = c.g[S0][0] == 0;
            c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = u64::MAX;
            }
            c.cycles += 8;
            c.sw(r32(c, S1), c.g[V0][0]);
            c.g[V1][0] = u64::MAX;
            c.g[V0][0] = 0x31;
            c.g[A0][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_ROWS));
            c.g[V0][0] = 0x31;
            c.g[V1][0] = u64::MAX;
            // The compiler's constructor loop of the uv/colour arrays (empty).
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                let more = c.g[V0][0] != c.g[V1][0];
                c.g[T2][0] = 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, SPIN50_HEAD);
                }
            }
            // Bilinear fill: for each row, for each column,
            // pos/uv/colour = lerp(lerp(a, b, fc), lerp(c, d, fc), fr).
            'grid: {
                c.cycles += 2;
                let no_rows = (c.g[A0][0] as i64) <= 0;
                c.g[V1][0] = 0;
                if no_rows {
                    break 'grid;
                }
                c.cycles += 3;
                c.g[T4][0] = sx(sp(c).wrapping_add(0x90));
                c.g[V0][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_COLS));
                loop {
                    'columns: {
                        c.cycles += 3;
                        c.g[T0][0] = 0;
                        let no_cols = (c.g[V0][0] as i64) <= 0;
                        c.g[T3][0] = sx(r32(c, V1).wrapping_add(1));
                        if no_cols {
                            break 'columns;
                        }
                        c.cycles += 7;
                        c.mtc1(V1, 16);
                        c.cvt_s_w(16, 16);
                        c.g[V0][0] = sx(r32(c, T2) << 3);
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 15);
                        c.g[T1][0] = sx(r32(c, T4).wrapping_add(r32(c, V0)));
                        loop {
                            c.cycles += 186;
                    c.mtc1(8, 9);
                    c.cvt_s_w(9, 9);
                    c.lwc1(0, bm_ad(c, S3, 0x6c));
                    c.lwc1(14, bm_ad(c, S3, 0x70));
                    c.g[V0][0] = 0xc;
                    c.lwc1(2, bm_ad(c, SP, 0x60));
                    c.mult(0, 2, 10, 2);
                    c.div_s(9, 9, 0);
                    c.lwc1(1, bm_ad(c, SP, 0x64));
                    c.lwc1(0, bm_ad(c, SP, 0x68));
                    c.div_s(14, 16, 14);
                    c.lwc1(11, bm_ad(c, SP, 0x6c));
                    c.g[V1][0] = sx(r32(c, T2) << 4);
                    c.lwc1(12, bm_ad(c, SP, 0x70));
                    c.g[A2][0] = sx(r32(c, SP).wrapping_add(r32(c, V1)));
                    c.lwc1(13, bm_ad(c, SP, 0x74));
                    c.g[A1][0] = sx(r32(c, S2).wrapping_add(r32(c, V1)));
                    c.lwc1(4, bm_ad(c, SP, 0x78));
                    c.g[A3][0] = c.g[A2][0];
                    c.sub_s(10, 15, 9);
                    c.lwc1(5, bm_ad(c, SP, 0x7c));
                    c.lwc1(6, bm_ad(c, SP, 0x80));
                    c.mul_s(11, 11, 9);
                    c.mul_s(12, 12, 9);
                    c.lwc1(3, bm_ad(c, SP, 0x84));
                    c.mul_s(2, 2, 10);
                    c.lwc1(7, bm_ad(c, SP, 0x88));
                    c.mul_s(1, 1, 10);
                    c.lwc1(8, bm_ad(c, SP, 0x8c));
                    c.mul_s(0, 0, 10);
                    c.g[A0][0] = c.lw(bm_ad(c, S3, 0x74));
                    c.mul_s(13, 13, 9);
                    c.swc1(2, bm_ad(c, SP, 0x570));
                    c.mul_s(4, 4, 10);
                    c.swc1(1, bm_ad(c, SP, 0x574));
                    c.mul_s(5, 5, 10);
                    c.swc1(0, bm_ad(c, SP, 0x578));
                    c.mul_s(6, 6, 10);
                    c.swc1(12, bm_ad(c, SP, 0x584));
                    c.mul_s(3, 3, 9);
                    c.swc1(4, bm_ad(c, SP, 0x5b0));
                    c.mul_s(7, 7, 9);
                    c.swc1(5, bm_ad(c, SP, 0x5b4));
                    c.mul_s(8, 8, 9);
                    c.swc1(6, bm_ad(c, SP, 0x5b8));
                    c.add_s(2, 2, 11);
                    c.swc1(11, bm_ad(c, SP, 0x580));
                    c.add_s(1, 1, 12);
                    c.swc1(3, bm_ad(c, SP, 0x5c0));
                    c.add_s(0, 0, 13);
                    c.swc1(7, bm_ad(c, SP, 0x5c4));
                    c.sub_s(12, 15, 14);
                    c.swc1(2, bm_ad(c, SP, 0x560));
                    c.add_s(4, 4, 3);
                    c.swc1(1, bm_ad(c, SP, 0x564));
                    c.add_s(6, 6, 8);
                    c.swc1(0, bm_ad(c, SP, 0x568));
                    c.add_s(5, 5, 7);
                    c.swc1(8, bm_ad(c, SP, 0x5c8));
                    c.mul_s(2, 2, 12);
                    c.swc1(13, bm_ad(c, SP, 0x588));
                    c.mul_s(1, 1, 12);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.mul_s(0, 0, 12);
                    c.g[A0][0] = c.g[A2][0];
                    c.mul_s(11, 4, 14);
                    c.swc1(2, bm_ad(c, SP, 0x550));
                    c.mul_s(7, 5, 14);
                    c.swc1(1, bm_ad(c, SP, 0x554));
                    c.mul_s(3, 6, 14);
                    c.swc1(0, bm_ad(c, SP, 0x558));
                    c.add_s(2, 2, 11);
                    c.swc1(4, bm_ad(c, SP, 0x5a0));
                    c.add_s(1, 1, 7);
                    c.swc1(11, bm_ad(c, SP, 0x590));
                    c.add_s(0, 0, 3);
                    c.swc1(5, bm_ad(c, SP, 0x5a4));
                    c.swc1(2, bm_ad(c, SP, 0x540));
                    c.g[T0][0] = sx(bm_ad(c, T0, 0x1));
                    c.swc1(1, bm_ad(c, SP, 0x544));
                    c.g[T2][0] = sx(bm_ad(c, T2, 0x1));
                    c.swc1(0, bm_ad(c, SP, 0x548));
                    c.swc1(6, bm_ad(c, SP, 0x5a8));
                    c.swc1(7, bm_ad(c, SP, 0x594));
                    c.swc1(3, bm_ad(c, SP, 0x598));
                    c.ldl(3, bm_ad(c, SP, 0x547));
                    c.ldr(3, bm_ad(c, SP, 0x540));
                    c.g[T5][0] = c.lw(bm_ad(c, SP, 0x548));
                    c.sdl(3, bm_ad(c, V0, 0x7));
                    c.sdr(3, r32(c, V0));
                    c.sw(bm_ad(c, V0, 0x8), c.g[T5][0]);
                    c.lwc1(0, r32(c, SP));
                    c.lwc1(1, bm_ad(c, SP, 0x4));
                    c.lwc1(2, bm_ad(c, SP, 0x8));
                    c.mul_s(0, 0, 10);
                    c.lwc1(3, bm_ad(c, SP, 0xc));
                    c.mul_s(1, 1, 10);
                    c.mul_s(2, 2, 9);
                    c.lwc1(4, bm_ad(c, SP, 0x10));
                    c.mul_s(3, 3, 9);
                    c.lwc1(5, bm_ad(c, SP, 0x14));
                    c.lwc1(6, bm_ad(c, SP, 0x18));
                    c.mul_s(4, 4, 10);
                    c.lwc1(7, bm_ad(c, SP, 0x1c));
                    c.mul_s(5, 5, 10);
                    c.mul_s(6, 6, 9);
                    c.swc1(0, bm_ad(c, SP, 0x570));
                    c.mul_s(7, 7, 9);
                    c.swc1(1, bm_ad(c, SP, 0x574));
                    c.add_s(0, 0, 2);
                    c.swc1(2, bm_ad(c, SP, 0x580));
                    c.add_s(1, 1, 3);
                    c.swc1(3, bm_ad(c, SP, 0x584));
                    c.add_s(11, 4, 6);
                    c.swc1(4, bm_ad(c, SP, 0x5e0));
                    c.add_s(8, 5, 7);
                    c.swc1(0, bm_ad(c, SP, 0x560));
                    c.swc1(1, bm_ad(c, SP, 0x564));
                    c.mul_s(0, 0, 12);
                    c.mul_s(1, 1, 12);
                    c.swc1(5, bm_ad(c, SP, 0x5e4));
                    c.mul_s(3, 11, 14);
                    c.swc1(6, bm_ad(c, SP, 0x5f0));
                    c.mul_s(2, 8, 14);
                    c.swc1(0, bm_ad(c, SP, 0x550));
                    c.swc1(1, bm_ad(c, SP, 0x554));
                    c.add_s(0, 0, 3);
                    c.swc1(3, bm_ad(c, SP, 0x590));
                    c.add_s(1, 1, 2);
                    c.swc1(7, bm_ad(c, SP, 0x5f4));
                    c.swc1(11, bm_ad(c, SP, 0x5d0));
                    c.swc1(8, bm_ad(c, SP, 0x5d4));
                    c.swc1(2, bm_ad(c, SP, 0x594));
                    c.swc1(0, bm_ad(c, SP, 0x540));
                    c.swc1(1, bm_ad(c, SP, 0x544));
                    c.ldl(2, bm_ad(c, SP, 0x547));
                    c.ldr(2, bm_ad(c, SP, 0x540));
                    c.sdl(2, bm_ad(c, T1, 0x7));
                    c.sdr(2, r32(c, T1));
                    c.swc1(15, r32(c, A1));
                    c.g[T1][0] = sx(bm_ad(c, T1, 0x8));
                    c.lwc1(2, bm_ad(c, SP, 0x24));
                    c.lwc1(3, bm_ad(c, SP, 0x34));
                    c.lwc1(0, bm_ad(c, SP, 0x44));
                    c.mul_s(2, 2, 10);
                    c.lwc1(1, bm_ad(c, SP, 0x54));
                    c.mul_s(3, 3, 9);
                    c.mul_s(0, 0, 10);
                    c.g[V0][0] = c.lw(bm_ad(c, S3, 0x64));
                    c.mul_s(1, 1, 9);
                    c.add_s(2, 2, 3);
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.add_s(0, 0, 1);
                    c.mul_s(2, 2, 12);
                    c.mul_s(0, 0, 14);
                    c.add_s(2, 2, 0);
                    c.swc1(2, bm_ad(c, A2, 0x224));
                    c.lwc1(1, bm_ad(c, SP, 0x28));
                    c.lwc1(3, bm_ad(c, SP, 0x38));
                    c.lwc1(0, bm_ad(c, SP, 0x48));
                    c.mul_s(1, 1, 10);
                    c.lwc1(2, bm_ad(c, SP, 0x58));
                    c.mul_s(3, 3, 9);
                    c.mul_s(0, 0, 10);
                    c.mul_s(2, 2, 9);
                    c.add_s(1, 1, 3);
                    c.add_s(0, 0, 2);
                    c.mul_s(1, 1, 12);
                    c.mul_s(0, 0, 14);
                    c.add_s(1, 1, 0);
                    c.swc1(1, bm_ad(c, A3, 0x228));
                    c.lwc1(2, bm_ad(c, SP, 0x2c));
                    c.lwc1(3, bm_ad(c, SP, 0x3c));
                    c.lwc1(0, bm_ad(c, SP, 0x4c));
                    c.mul_s(2, 2, 10);
                    c.lwc1(1, bm_ad(c, SP, 0x5c));
                    c.mul_s(3, 3, 9);
                    c.mul_s(0, 0, 10);
                    c.mul_s(1, 1, 9);
                    c.add_s(2, 2, 3);
                    c.add_s(0, 0, 1);
                    c.mul_s(2, 2, 12);
                    c.mul_s(0, 0, 14);
                    c.add_s(2, 2, 0);
                            let more = c.g[V0][0] != 0;
                            c.swc1(2, bm_ad(c, A0, 0x22c));
                            if !more {
                                break 'columns;
                            }
                            if event_due(c) {
                                deliver_event_here(c, COLUMN_HEAD);
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_ROWS));
                    c.g[V1][0] = c.g[T3][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break 'grid;
                    }
                    c.g[V0][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_COLS));
                    if event_due(c) {
                        deliver_event_here(c, ROW_HEAD);
                    }
                }
            }
            // Animate the vertices once, then hand them to the mesh.
            c.cycles += 15;
            c.g[V0][0] = 0x31;
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S4][0];
            GROUP_BUILD_MESH_COMPUTE_VERTS.call(c, 0x0034_b764)?;
            c.g[T0][0] = c.lw(bm_ad(c, S3, GROUP_BUILD_MESH_MESH));
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[V0][0] = c.lw(bm_ad(c, T0, 4));
            c.g[A0][0] = c.lh(bm_ad(c, V0, 0x30));
            c.g[V1][0] = c.lw(bm_ad(c, V0, 0x34));
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0034_b788)?;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x8e0);
        frame.restore(c, S1, 0x8d0);
        frame.restore(c, S2, 0x8c0);
        frame.restore(c, S3, 0x8b0);
        frame.restore(c, S4, 0x8a0);
        frame.restore(c, S5, 0x890);
        frame.restore(c, S6, 0x880);
        frame.restore(c, S7, 0x870);
        frame.restore_ra(c, 0x860);
        frame.restore_fpr(c, 20, 0x8f0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Fixture base (clear of the fakes' log and of their hashed pointers).
    const FIX: u32 = DATA + 0x20_0000;
    const MGR: u32 = FIX + 0x1000;
    const MGR_VT: u32 = FIX + 0x3000;
    const MESH: u32 = FIX + 0x4000;
    const MESH_VT: u32 = FIX + 0x4100;
    const VERTS: u32 = FIX + 0x5000;
    const CTRL: u32 = FIX + 0x6000;
    const MAGIC: u32 = 0x4d45_5348;

    static PREV_NEW: AtomicUsize = AtomicUsize::new(0);
    static PREV_VERTS: AtomicUsize = AtomicUsize::new(0);

    /// `operator new[]` while the gate word is set: log the call, answer
    /// with the result the fixture forces.
    fn fake_new(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_NEW, ee::s10::f_00317e30__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e30, ra)
    }

    /// `cFlagManager_GroupComputeVerts` while the gate word is set.
    fn fake_compute_verts(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_VERTS, ee::s11::f_0034bca0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0034_bca0, ra)
    }

    /// `cFlagManager_GroupBuildMesh`: calm and windy flags (8x2 and 8x5
    /// grids). The item's quad getter, the mesh creation (or the real
    /// `return 0` leaf 0x1406c0: no mesh), `operator new[]`,
    /// `GroupComputeVerts` and the mesh's vertex setter are fakes (the quad
    /// is prepared on the stack); one seed in three unwinds at a fake call.
    /// The four phases come from the real `BXrand`.
    #[test]
    fn c_flag_manager_group_build_mesh_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            fake_new,
            &PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0034BCA0),
            fake_compute_verts,
            &PREV_VERTS,
        );
        covered(
            ee::s11::f_0034b228__generated,
            c_flag_manager_group_build_mesh,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x34b228, 5);
                c.pc = 0x0034_b228;
                c.bus.write32(CTRL, MAGIC);
                let group = FIX;
                // The manager and its mesh-manager vtable.
                c.bus.write32(GAME_GP + GROUP_BUILD_MESH_MANAGER_GP, MGR);
                c.bus.write32(MGR + 0x10d8, MGR_VT);
                for (slot, fake) in [(0x308u32, FAKE), (0x260, FAKE + 0x10)] {
                    c.bus.write16(MGR_VT + slot, (r.next_u32() & 0xfc) as u16);
                    c.bus.write32(MGR_VT + slot + 4, fake);
                }
                let no_mesh = seed % 6 == 0;
                if no_mesh {
                    c.bus.write32(MGR_VT + 0x264, 0x0014_06c0);
                }
                // The mesh and its vertex setter (slot 0x30).
                c.bus.write32(MESH + 4, MESH_VT);
                c.bus.write16(MESH_VT + 0x30, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(MESH_VT + 0x34, FAKE + 0x20);
                // Forced results: call 1 = the mesh, call 2 = the vertex array.
                c.bus.write32(LOG_RESULTS + 4, MESH);
                c.bus.write32(LOG_RESULTS + 8, VERTS);
                // Wind amounts against the 0.01 threshold: calm, x windy,
                // y windy, both windy.
                c.bus.write32(GAME_GP.wrapping_sub(0x2b04), 0.01f32.to_bits());
                let amount = |r: &mut Rng, windy: bool| {
                    if windy {
                        (0.02 + (r.next_u32() % 100) as f32 / 100.0).to_bits()
                    } else {
                        ((r.next_u32() % 100) as f32 / 20000.0).to_bits()
                    }
                };
                let x = amount(&mut r, seed % 4 == 1 || seed % 4 == 3);
                let y = amount(&mut r, seed % 4 == 2 || seed % 4 == 3);
                c.bus.write32(group + GROUP_BUILD_MESH_WIND_X, x);
                c.bus.write32(group + GROUP_BUILD_MESH_WIND_Y, y);
                for off in (0x60..0x80).step_by(4) {
                    c.bus.write32(group + off, r.next_u32());
                }
                // The quad the item getter would have left on the stack:
                // colours (sp+0), texture coordinates (sp+0x20), positions (sp+0x60).
                let frame = (c.g[SP][0] as u32).wrapping_sub(0x900);
                for i in 0..0x24u32 {
                    c.bus.write32(frame + 4 * i, r.f32(8.0));
                }
                c.g[A0][0] = group as u64;
                c.g[A1][0] = (FIX + 0x200) as u64;
            },
        );
    }
}
