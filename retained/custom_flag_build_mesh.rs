//! Native cFlagManager_GroupBuildMesh (0x0034b228..0x0034b7b8).
//!
//! This address creates the flag grid; deformation is the separate
//! GroupComputeVerts at 0x0034bca0. Retained uninstalled: the user disabled
//! automated tests and replays. No game/window operations were performed.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::float;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0034_b228;
const END: u32 = 0x0034_b7b8;
const SEED_PHASE: u32 = 0x0034_b280;
const AFTER_RANDOM: u32 = 0x0034_b288;
const QUAD_TEMP_LOOP: u32 = 0x0034_b2d0;
const AFTER_QUAD: u32 = 0x0034_b30c;
const AFTER_CREATE_GRID: u32 = 0x0034_b384;
const AFTER_ALLOCATE_VERTICES: u32 = 0x0034_b3bc;
const GRID_TEMP_LOOP: u32 = 0x0034_b3f8;
const ROW_LOOP: u32 = 0x0034_b428;
const COLUMN_LOOP: u32 = 0x0034_b450;
const AFTER_DEFORM: u32 = 0x0034_b764;
const FINISH: u32 = 0x0034_b788;

const AMPLITUDE_THIRD: u32 = 0x1c;
const AMPLITUDE_FOURTH: u32 = 0x20;
const MESH: u32 = 0x60;
const COLUMNS: u32 = 0x64;
const ROWS: u32 = 0x68;
const COLUMN_DIVISOR: u32 = 0x6c;
const ROW_DIVISOR: u32 = 0x70;
const REST_POSITIONS: u32 = 0x74;
const PHASES: u32 = 0x78;
const UV_OFFSET_U: u32 = 0x88;
const UV_OFFSET_V: u32 = 0x8c;
const GRAPHICS_MANAGER_GP: u32 = 0x2a90;
const GRAPHICS_VTABLE: u32 = 0x10d8;
const GRID_THRESHOLD_GP: u32 = 0x2b04;
const RANDOM: u32 = 0x0031_77f0;
const ALLOCATE_ARRAY: u32 = 0x0031_7e30;
const COMPUTE_VERTICES: Callee = Callee {
    addr: 0x0034_bca0,
    f: ssx3_ee::s11::cFlagManager_GroupComputeVerts,
};

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}

/// `void GroupBuildMesh(FlagGroup* group, Instance* first_member)`.
///
/// Seeds four phase values uniformly in [0,1), resets UV scrolling, and reads
/// the instance's first quad through graphics slot97. The grid is eight
/// columns by two rows when amplitudes at +0x1c/+0x20 are both below the
/// native 0.01 threshold; otherwise it has five rows. These fields alone
/// choose the grid: no variant, phase increment, scale or boolean is read.
///
/// A bilinear interpolation of that quad creates rest positions (+0x74),
/// temporary UVs and RGB colors. GroupComputeVerts then deforms those rest
/// positions before mesh slot6 receives the positions, colors and UVs.
/// Mesh creation failure returns before allocating or building the grid.
pub fn flag_manager_group_build_mesh(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x900 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 22;
                    Frame::open(c, frame.size);
                    frame.save(c, S2, 0x8c0);
                    frame.save(c, S3, 0x8b0);
                    c.g[S2][0] = 0x7f_0000;
                    frame.save_fpr(c, 20, 0x8f0);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S0, 0x8e0);
                    c.g[S2][0] |= 0xffff;
                    frame.save(c, S1, 0x8d0);
                    c.g[S0][0] = sx(at(c, S3, PHASES));
                    frame.save(c, S4, 0x8a0);
                    c.g[S1][0] = 3;
                    frame.save(c, S5, 0x890);
                    c.g[S4][0] = 0x3f80_0000;
                    frame.save(c, S6, 0x880);
                    c.g[S5][0] = c.g[A1][0];
                    frame.save(c, S7, 0x870);
                    c.g[S6][0] = sx(at(c, SP, 0x60));
                    frame.save_ra(c, 0x860);
                    c.g[S7][0] = sx(at(c, SP, 0x20));
                    c.g[AT][0] = 0x3f80_0000;
                    c.f[20] = r(c, AT);
                    pc = SEED_PHASE;
                }
                SEED_PHASE => {
                    c.cycles += 9;
                    c.g[S1][0] = sx(r(c, S1).wrapping_sub(1));
                    call_ptr(c, RANDOM, AFTER_RANDOM)?;
                    pc = AFTER_RANDOM;
                }
                AFTER_RANDOM => {
                    c.g[V0][0] &= c.g[S2][0];
                    c.g[V0][0] |= c.g[S4][0];
                    c.f[1] = r(c, V0);
                    c.f[0] = (FpuF32(c.f[1]) - FpuF32(c.f[20])).0;
                    c.swc1(0, r(c, S0));
                    let more = (c.g[S1][0] as i64) >= 0;
                    c.g[S0][0] = sx(at(c, S0, 4));
                    if more {
                        if event_due(c) { deliver_event_here(c, SEED_PHASE); }
                        pc = SEED_PHASE;
                    } else {
                        c.cycles += 11;
                        c.sw(at(c, S3, UV_OFFSET_U), 0);
                        c.g[V0][0] = 3;
                        c.sw(at(c, S3, UV_OFFSET_V), 0);
                        c.g[V1][0] = u64::MAX;
                        c.g[S0][0] = c.lw(at(c, GP, GRAPHICS_MANAGER_GP));
                        c.g[A2][0] = c.lw(at(c, S0, GRAPHICS_VTABLE));
                        c.g[V0][0] = 3;
                        c.g[V1][0] = u64::MAX;
                        c.g[V0][0] = 3;
                        c.g[V1][0] = u64::MAX;
                        pc = QUAD_TEMP_LOOP;
                    }
                }
                QUAD_TEMP_LOOP => {
                    // Constructor/destructor loop retained for native timing.
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    c.g[A1][0] = c.g[S5][0];
                    if more {
                        if event_due(c) { deliver_event_here(c, QUAD_TEMP_LOOP); }
                    } else {
                        c.cycles += 14;
                        c.g[A0][0] = c.lh(at(c, A2, 0x308));
                        c.g[V0][0] = c.lw(at(c, A2, 0x30c));
                        c.g[A3][0] = c.g[S6][0];
                        c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                        c.g[T1][0] = c.g[S7][0];
                        c.g[A2][0] = 0;
                        let target = r(c, V0);
                        c.g[T0][0] = c.g[SP][0];
                        call_ptr(c, target, AFTER_QUAD)?;
                        pc = AFTER_QUAD;
                    }
                }
                AFTER_QUAD => {
                    c.lwc1(1, r(c, GP).wrapping_sub(GRID_THRESHOLD_GP));
                    c.lwc1(0, at(c, S3, AMPLITUDE_THIRD));
                    let small = FpuF32(c.f[0]).lt(FpuF32(c.f[1]));
                    abi::set_fcc(c, small);
                    c.g[V0][0] = 8;
                    let mut two_rows = false;
                    if small {
                        c.cycles += 5;
                        c.lwc1(0, at(c, S3, AMPLITUDE_FOURTH));
                        two_rows = FpuF32(c.f[0]).lt(FpuF32(c.f[1]));
                        abi::set_fcc(c, two_rows);
                        c.g[V1][0] = 2;
                    }
                    if !two_rows {
                        c.cycles += 1;
                        c.g[V1][0] = 5;
                    }
                    c.cycles += 20;
                    c.sw(at(c, S3, COLUMNS), c.g[V0][0]);
                    c.sw(at(c, S3, ROWS), c.g[V1][0]);
                    c.g[A1][0] = c.lw(at(c, S3, COLUMNS));
                    c.g[A3][0] = 0x6000_0000;
                    c.g[A2][0] = c.lw(at(c, S3, ROWS));
                    c.g[V1][0] = sx(r(c, A1).wrapping_sub(1));
                    c.g[V0][0] = sx(r(c, A2).wrapping_sub(1));
                    c.f[0] = float::from_fixed(r(c, V1) as i32, 0);
                    c.f[1] = float::from_fixed(r(c, V0) as i32, 0);
                    c.swc1(0, at(c, S3, COLUMN_DIVISOR));
                    c.swc1(1, at(c, S3, ROW_DIVISOR));
                    c.g[V1][0] = c.lw(at(c, S0, GRAPHICS_VTABLE));
                    c.g[A0][0] = c.lh(at(c, V1, 0x260));
                    c.g[V0][0] = c.lw(at(c, V1, 0x264));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_CREATE_GRID)?;
                    pc = AFTER_CREATE_GRID;
                }
                AFTER_CREATE_GRID => {
                    let failed = c.g[V0][0] == 0;
                    c.sw(at(c, S3, MESH), c.g[V0][0]);
                    if failed {
                        pc = FINISH;
                    } else {
                        c.cycles += 17;
                        c.g[S0][0] = c.lw(at(c, S3, COLUMNS));
                        c.g[A0][0] = 12;
                        c.g[V0][0] = c.lw(at(c, S3, ROWS));
                        c.g[A1][0] = 0x49_0000;
                        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x17b0));
                        c.g[A2][0] = 0x2000_0000;
                        c.mult(0, S0, S0, V0);
                        c.g[A3][0] = 0;
                        c.mult(0, ZERO, S0, A0);
                        c.g[A0][0] = c.lo[0];
                        c.g[S1][0] = sx(at(c, S3, REST_POSITIONS));
                        call_ptr(c, ALLOCATE_ARRAY, AFTER_ALLOCATE_VERTICES)?;
                        pc = AFTER_ALLOCATE_VERTICES;
                    }
                }
                AFTER_ALLOCATE_VERTICES => {
                    c.g[S2][0] = sx(at(c, SP, 0x220));
                    c.g[S4][0] = sx(at(c, SP, 0x600));
                    c.g[S5][0] = sx(at(c, SP, 0x90));
                    let no_columns = c.g[S0][0] == 0;
                    c.g[V1][0] = sx(r(c, S0).wrapping_sub(1));
                    if !no_columns {
                        c.cycles += 2;
                        c.g[A0][0] = u64::MAX;
                    }
                    c.cycles += 8;
                    c.sw(r(c, S1), c.g[V0][0]);
                    c.g[V1][0] = u64::MAX;
                    c.g[V0][0] = 0x31;
                    c.g[A0][0] = c.lw(at(c, S3, ROWS));
                    c.g[V0][0] = 0x31;
                    c.g[V1][0] = u64::MAX;
                    pc = GRID_TEMP_LOOP;
                }
                GRID_TEMP_LOOP => {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    let more = c.g[V0][0] != c.g[V1][0];
                    c.g[T2][0] = 0;
                    if more {
                        if event_due(c) { deliver_event_here(c, GRID_TEMP_LOOP); }
                    } else {
                        c.cycles += 2;
                        let no_rows = (c.g[A0][0] as i64) <= 0;
                        c.g[V1][0] = 0;
                        if no_rows {
                            prepare_deformation(c);
                            COMPUTE_VERTICES.call(c, AFTER_DEFORM)?;
                            pc = AFTER_DEFORM;
                        } else {
                            c.cycles += 3;
                            c.g[T4][0] = sx(at(c, SP, 0x90));
                            c.g[V0][0] = c.lw(at(c, S3, COLUMNS));
                            pc = ROW_LOOP;
                        }
                    }
                }
                ROW_LOOP => {
                    c.cycles += 3;
                    c.g[T0][0] = 0;
                    let no_columns = (c.g[V0][0] as i64) <= 0;
                    c.g[T3][0] = sx(r(c, V1).wrapping_add(1));
                    if no_columns {
                        pc = finish_row(c);
                    } else {
                        c.cycles += 7;
                        c.f[16] = float::from_fixed(r(c, V1) as i32, 0);
                        c.g[V0][0] = sx(r(c, T2) << 3);
                        c.g[AT][0] = 0x3f80_0000;
                        c.f[15] = r(c, AT);
                        c.g[T1][0] = sx(r(c, T4).wrapping_add(r(c, V0)));
                        pc = COLUMN_LOOP;
                    }
                    if pc == AFTER_DEFORM {
                        prepare_deformation(c);
                        COMPUTE_VERTICES.call(c, AFTER_DEFORM)?;
                    }
                }
                COLUMN_LOOP => {
                    interpolate_vertex(c);
                    if c.g[V0][0] != 0 {
                        if event_due(c) { deliver_event_here(c, COLUMN_LOOP); }
                    } else {
                        pc = finish_row(c);
                        if pc == AFTER_DEFORM {
                            prepare_deformation(c);
                            COMPUTE_VERTICES.call(c, AFTER_DEFORM)?;
                        }
                    }
                }
                AFTER_DEFORM => {
                    c.g[T0][0] = c.lw(at(c, S3, MESH));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[V0][0] = c.lw(at(c, T0, 4));
                    c.g[A0][0] = c.lh(at(c, V0, 0x30));
                    c.g[V1][0] = c.lw(at(c, V0, 0x34));
                    let target = r(c, V1);
                    c.g[A0][0] = sx(r(c, T0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, FINISH)?;
                    pc = FINISH;
                }
                FINISH => {
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
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}

/// Advance the row using the native signed comparison and event boundary.
fn finish_row(c: &mut Cpu) -> u32 {
    c.cycles += 5;
    c.g[V0][0] = c.lw(at(c, S3, ROWS));
    c.g[V1][0] = c.g[T3][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = c.lw(at(c, S3, COLUMNS));
        if event_due(c) { deliver_event_here(c, ROW_LOOP); }
        ROW_LOOP
    } else {
        AFTER_DEFORM
    }
}

fn prepare_deformation(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = 0x31;
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S4][0];
}

/// One vertex of the bilinear grid. Keep the original FPU operation order,
/// scratch registers and stack temporaries: the caller/callee/event state is
/// observable even though most intermediate values do not belong to the mesh.
fn interpolate_vertex(c: &mut Cpu) {
    c.cycles += 186;
    c.f[9] = r(c, T0);
    c.f[9] = float::from_fixed(c.f[9] as i32, 0);
    c.lwc1(0, at(c, S3, 0x6c));
    c.lwc1(14, at(c, S3, 0x70));
    // Original position grid: u=f9, v=f14, 1-u=f10, 1-v=f12.
    c.g[V0][0] = 12;
    c.lwc1(2, at(c, SP, 0x60));
    c.mult(0, V0, T2, V0);
    c.f[9] = FpuF32(c.f[9]).div(FpuF32(c.f[0])).0;
    c.lwc1(1, at(c, SP, 0x64));
    c.lwc1(0, at(c, SP, 0x68));
    c.f[14] = FpuF32(c.f[16]).div(FpuF32(c.f[14])).0;
    c.lwc1(11, at(c, SP, 0x6c));
    c.g[V1][0] = sx(r(c, T2) << 4);
    c.lwc1(12, at(c, SP, 0x70));
    c.g[A2][0] = sx(r(c, SP).wrapping_add(r(c, V1)));
    c.lwc1(13, at(c, SP, 0x74));
    c.g[A1][0] = sx(r(c, S2).wrapping_add(r(c, V1)));
    c.lwc1(4, at(c, SP, 0x78));
    c.g[A3][0] = c.g[A2][0];
    c.f[10] = (FpuF32(c.f[15]) - FpuF32(c.f[9])).0;
    c.lwc1(5, at(c, SP, 0x7c));
    c.lwc1(6, at(c, SP, 0x80));
    c.f[11] = (FpuF32(c.f[11]) * FpuF32(c.f[9])).0;
    c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[9])).0;
    c.lwc1(3, at(c, SP, 0x84));
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[10])).0;
    c.lwc1(7, at(c, SP, 0x88));
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[10])).0;
    c.lwc1(8, at(c, SP, 0x8c));
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[10])).0;
    c.g[A0][0] = c.lw(at(c, S3, 0x74));
    c.f[13] = (FpuF32(c.f[13]) * FpuF32(c.f[9])).0;
    c.swc1(2, at(c, SP, 0x570));
    c.f[4] = (FpuF32(c.f[4]) * FpuF32(c.f[10])).0;
    c.swc1(1, at(c, SP, 0x574));
    c.f[5] = (FpuF32(c.f[5]) * FpuF32(c.f[10])).0;
    c.swc1(0, at(c, SP, 0x578));
    c.f[6] = (FpuF32(c.f[6]) * FpuF32(c.f[10])).0;
    c.swc1(12, at(c, SP, 0x584));
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[9])).0;
    c.swc1(4, at(c, SP, 0x5b0));
    c.f[7] = (FpuF32(c.f[7]) * FpuF32(c.f[9])).0;
    c.swc1(5, at(c, SP, 0x5b4));
    c.f[8] = (FpuF32(c.f[8]) * FpuF32(c.f[9])).0;
    c.swc1(6, at(c, SP, 0x5b8));
    c.f[2] = (FpuF32(c.f[2]) + FpuF32(c.f[11])).0;
    c.swc1(11, at(c, SP, 0x580));
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[12])).0;
    c.swc1(3, at(c, SP, 0x5c0));
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[13])).0;
    c.swc1(7, at(c, SP, 0x5c4));
    c.f[12] = (FpuF32(c.f[15]) - FpuF32(c.f[14])).0;
    c.swc1(2, at(c, SP, 0x560));
    c.f[4] = (FpuF32(c.f[4]) + FpuF32(c.f[3])).0;
    c.swc1(1, at(c, SP, 0x564));
    c.f[6] = (FpuF32(c.f[6]) + FpuF32(c.f[8])).0;
    c.swc1(0, at(c, SP, 0x568));
    c.f[5] = (FpuF32(c.f[5]) + FpuF32(c.f[7])).0;
    c.swc1(8, at(c, SP, 0x5c8));
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[12])).0;
    c.swc1(13, at(c, SP, 0x588));
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[12])).0;
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[12])).0;
    c.g[A0][0] = c.g[A2][0];
    c.f[11] = (FpuF32(c.f[4]) * FpuF32(c.f[14])).0;
    c.swc1(2, at(c, SP, 0x550));
    c.f[7] = (FpuF32(c.f[5]) * FpuF32(c.f[14])).0;
    c.swc1(1, at(c, SP, 0x554));
    c.f[3] = (FpuF32(c.f[6]) * FpuF32(c.f[14])).0;
    c.swc1(0, at(c, SP, 0x558));
    c.f[2] = (FpuF32(c.f[2]) + FpuF32(c.f[11])).0;
    c.swc1(4, at(c, SP, 0x5a0));
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[7])).0;
    c.swc1(11, at(c, SP, 0x590));
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[3])).0;
    c.swc1(5, at(c, SP, 0x5a4));
    c.swc1(2, at(c, SP, 0x540));
    c.g[T0][0] = sx(at(c, T0, 0x1));
    c.swc1(1, at(c, SP, 0x544));
    c.g[T2][0] = sx(at(c, T2, 0x1));
    c.swc1(0, at(c, SP, 0x548));
    c.swc1(6, at(c, SP, 0x5a8));
    c.swc1(7, at(c, SP, 0x594));
    c.swc1(3, at(c, SP, 0x598));
    c.ldl(V1, at(c, SP, 0x547));
    c.ldr(V1, at(c, SP, 0x540));
    c.g[T5][0] = c.lw(at(c, SP, 0x548));
    c.sdl(V1, at(c, V0, 0x7));
    c.sdr(V1, r(c, V0));
    c.sw(at(c, V0, 0x8), c.g[T5][0]);
    // Bilinear texture coordinates, preserving stack temporaries.
    c.lwc1(0, r(c, SP));
    c.lwc1(1, at(c, SP, 0x4));
    c.lwc1(2, at(c, SP, 0x8));
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[10])).0;
    c.lwc1(3, at(c, SP, 0xc));
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[10])).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[9])).0;
    c.lwc1(4, at(c, SP, 0x10));
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[9])).0;
    c.lwc1(5, at(c, SP, 0x14));
    c.lwc1(6, at(c, SP, 0x18));
    c.f[4] = (FpuF32(c.f[4]) * FpuF32(c.f[10])).0;
    c.lwc1(7, at(c, SP, 0x1c));
    c.f[5] = (FpuF32(c.f[5]) * FpuF32(c.f[10])).0;
    c.f[6] = (FpuF32(c.f[6]) * FpuF32(c.f[9])).0;
    c.swc1(0, at(c, SP, 0x570));
    c.f[7] = (FpuF32(c.f[7]) * FpuF32(c.f[9])).0;
    c.swc1(1, at(c, SP, 0x574));
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[2])).0;
    c.swc1(2, at(c, SP, 0x580));
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[3])).0;
    c.swc1(3, at(c, SP, 0x584));
    c.f[11] = (FpuF32(c.f[4]) + FpuF32(c.f[6])).0;
    c.swc1(4, at(c, SP, 0x5e0));
    c.f[8] = (FpuF32(c.f[5]) + FpuF32(c.f[7])).0;
    c.swc1(0, at(c, SP, 0x560));
    c.swc1(1, at(c, SP, 0x564));
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[12])).0;
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[12])).0;
    c.swc1(5, at(c, SP, 0x5e4));
    c.f[3] = (FpuF32(c.f[11]) * FpuF32(c.f[14])).0;
    c.swc1(6, at(c, SP, 0x5f0));
    c.f[2] = (FpuF32(c.f[8]) * FpuF32(c.f[14])).0;
    c.swc1(0, at(c, SP, 0x550));
    c.swc1(1, at(c, SP, 0x554));
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[3])).0;
    c.swc1(3, at(c, SP, 0x590));
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[2])).0;
    c.swc1(7, at(c, SP, 0x5f4));
    c.swc1(11, at(c, SP, 0x5d0));
    c.swc1(8, at(c, SP, 0x5d4));
    c.swc1(2, at(c, SP, 0x594));
    c.swc1(0, at(c, SP, 0x540));
    c.swc1(1, at(c, SP, 0x544));
    c.ldl(V0, at(c, SP, 0x547));
    c.ldr(V0, at(c, SP, 0x540));
    c.sdl(V0, at(c, T1, 0x7));
    c.sdr(V0, r(c, T1));
    // Bilinear RGB; alpha is always one.
    c.swc1(15, r(c, A1));
    c.g[T1][0] = sx(at(c, T1, 0x8));
    c.lwc1(2, at(c, SP, 0x24));
    c.lwc1(3, at(c, SP, 0x34));
    c.lwc1(0, at(c, SP, 0x44));
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[10])).0;
    c.lwc1(1, at(c, SP, 0x54));
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[9])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[10])).0;
    c.g[V0][0] = c.lw(at(c, S3, 0x64));
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[9])).0;
    c.f[2] = (FpuF32(c.f[2]) + FpuF32(c.f[3])).0;
    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[12])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[14])).0;
    c.f[2] = (FpuF32(c.f[2]) + FpuF32(c.f[0])).0;
    c.swc1(2, at(c, A2, 0x224));
    c.lwc1(1, at(c, SP, 0x28));
    c.lwc1(3, at(c, SP, 0x38));
    c.lwc1(0, at(c, SP, 0x48));
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[10])).0;
    c.lwc1(2, at(c, SP, 0x58));
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[9])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[10])).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[9])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[3])).0;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[2])).0;
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[12])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[14])).0;
    c.f[1] = (FpuF32(c.f[1]) + FpuF32(c.f[0])).0;
    c.swc1(1, at(c, A3, 0x228));
    c.lwc1(2, at(c, SP, 0x2c));
    c.lwc1(3, at(c, SP, 0x3c));
    c.lwc1(0, at(c, SP, 0x4c));
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[10])).0;
    c.lwc1(1, at(c, SP, 0x5c));
    c.f[3] = (FpuF32(c.f[3]) * FpuF32(c.f[9])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[10])).0;
    c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[9])).0;
    c.f[2] = (FpuF32(c.f[2]) + FpuF32(c.f[3])).0;
    c.f[0] = (FpuF32(c.f[0]) + FpuF32(c.f[1])).0;
    c.f[2] = (FpuF32(c.f[2]) * FpuF32(c.f[12])).0;
    c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[14])).0;
    c.f[2] = (FpuF32(c.f[2]) + FpuF32(c.f[0])).0;
    c.swc1(2, at(c, A0, 0x22c));
}
