//! Native registration and relocation of resources loaded from a world SSB.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_ae68,
    resolve_world_resource,
    "cPS2ChunkResolve_Resolve",
)];

const DISPATCH_TABLE: u32 = 0x0049_4fb0;
const RESOLVER_NASH: u32 = 0xc;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

fn call_resolver_slot(c: &mut Cpu, ret: u32) -> Flow {
    let target = r(c, V0);
    c.g[A0][0] = sx(r(c, S3).wrapping_add(r(c, A0)));
    call_ptr(c, target, ret)
}

fn write_bounds(c: &mut Cpu) {
    c.swc1(5, r(c, SP));
    c.swc1(0, r(c, SP).wrapping_add(4));
    c.swc1(1, r(c, SP).wrapping_add(8));
    c.swc1(2, r(c, SP).wrapping_add(0x10));
    c.swc1(3, r(c, SP).wrapping_add(0x14));
    c.swc1(4, r(c, SP).wrapping_add(0x18));
    c.swc1(6, r(c, SP).wrapping_add(0x1c));
    c.swc1(6, r(c, SP).wrapping_add(0xc));
}

/// Select the Nash octant from the signed box key returned on the stack.
fn select_cell(c: &mut Cpu) {
    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x24));
    c.g[A3][0] = u64::MAX;
    c.g[V1][0] = sx(r(c, S0).wrapping_add(0x50));
    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x28));
    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A2][0] = c.lw(r(c, SP).wrapping_add(0x2c));
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.g[S0][0];
    }
    c.g[A1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
    c.g[A0][0] = sx(r(c, V1).wrapping_add(0x28));
    c.g[A3][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
    if c.g[A1][0] != 0 {
        c.g[A0][0] = c.g[V1][0];
    }
    c.g[A2][0] = c.g[S2][0];
    c.g[V0][0] = sx(r(c, A0).wrapping_add(0x14));
}

/// `void* Resolve(resolver, unsigned kind, unsigned handle, void* payload)`
/// (0x3aae68). Resolve type-specific data, publish textures and world-side
/// files, and insert patches, instances and effect primitives into Nash.
/// The PC dispatcher also preserves the spline loop's event resume point.
pub fn resolve_world_resource(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xc0 };
        let mut pc = c.pc;
        loop {
            match pc {
                0x003a_ae68 => {
                    c.cycles += 15;
                    let frame = Frame::open(c, 0xc0);
                    c.g[V0][0] = (c.g[A1][0] < 0x17) as u64;
                    frame.save(c, S0, 0xa0);
                    frame.save(c, S2, 0x80);
                    c.g[S0][0] = c.g[A2][0];
                    frame.save(c, S3, 0x70);
                    c.g[S2][0] = c.g[A3][0];
                    frame.save(c, S1, 0x90);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S4, 0x60);
                    frame.save(c, S5, 0x50);
                    frame.save(c, S6, 0x40);
                    frame.save_ra(c, 0x30);
                    let out_of_range = c.g[V0][0] == 0;
                    frame.save_fpr(c, 20, 0xb0);
                    pc = if out_of_range {
                        0x003a_b464
                    } else {
                        0x003a_aea4
                    };
                }
                0x003a_aea4 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x0049_0000;
                    c.g[V1][0] = sx(r(c, A1) << 2);
                    c.g[V0][0] = sx(DISPATCH_TABLE);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.g[A0][0] = c.lw(r(c, V1));
                    pc = r(c, A0);
                    if pc == 0x003a_ae68 && event_due(c) {
                        deliver_event_here(c, 0x003a_ae68);
                    }
                }
                // kind 1: patches, including the shared lightmap ID base.
                0x003a_aec0 => {
                    c.cycles += 39;
                    c.g[V1][0] = c.lhu(r(c, S3).wrapping_add(0x10));
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.g[V0][0] = c.lhu(r(c, S2).wrapping_add(0x1a2));
                    c.g[A1][0] = c.g[SP][0];
                    c.lwc1(5, r(c, S2).wrapping_add(0x158));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.lwc1(0, r(c, S2).wrapping_add(0x15c));
                    c.sh(r(c, S2).wrapping_add(0x1a2), c.g[V0][0]);
                    c.lwc1(1, r(c, S2).wrapping_add(0x160));
                    c.lwc1(2, r(c, S2).wrapping_add(0x164));
                    c.lwc1(3, r(c, S2).wrapping_add(0x168));
                    c.lwc1(4, r(c, S2).wrapping_add(0x16c));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 6);
                    c.g[S0][0] = c.lw(r(c, S3).wrapping_add(RESOLVER_NASH));
                    write_bounds(c);
                    call_ptr(c, 0x0032_8f28, 0x003a_af20)?;
                    select_cell(c);
                    c.g[A1][0] = 1;
                    pc = 0x003a_b2a8;
                }
                // kinds 9/10: shape and lightmap texture publication.
                0x003a_af5c => {
                    c.cycles += 11;
                    c.g[T2][0] = c.lw(r(c, GP).wrapping_add(0x2a90));
                    c.g[A2][0] = 0x0049_0000;
                    c.g[A2][0] = sx(0x0049_4f88);
                    c.g[T1][0] = sx(r(c, S0) >> 8);
                    c.g[V1][0] = c.lw(r(c, T2).wrapping_add(0x10d8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 0;
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x170));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x174));
                    pc = 0x003a_afb8;
                }
                0x003a_af88 => {
                    c.cycles += 12;
                    c.g[T2][0] = c.lw(r(c, GP).wrapping_add(0x2a90));
                    c.g[V0][0] = sx(r(c, S0) >> 8);
                    c.g[T1][0] = c.lw(r(c, S3).wrapping_add(0x10));
                    c.g[A2][0] = 0x0049_0000;
                    c.g[V1][0] = c.lw(r(c, T2).wrapping_add(0x10d8));
                    c.g[A2][0] = sx(0x0049_4f98);
                    c.g[T1][0] = sx(r(c, V0).wrapping_add(r(c, T1)));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x170));
                    c.g[A3][0] = 0;
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x174));
                    c.g[T0][0] = 0;
                    pc = 0x003a_afb8;
                }
                0x003a_afb8 => {
                    c.cycles += 4;
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, T2).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x003a_afc0)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                // kind 3: resolve its model/colour chain, then spatially link it.
                0x003a_afc8 => {
                    c.cycles += 41;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x68));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x6c));
                    call_resolver_slot(c, 0x003a_afe0)?;
                    c.lwc1(5, r(c, S2).wrapping_add(0x60));
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.lwc1(0, r(c, S2).wrapping_add(0x64));
                    c.g[A1][0] = c.g[SP][0];
                    c.lwc1(1, r(c, S2).wrapping_add(0x68));
                    c.lwc1(2, r(c, S2).wrapping_add(0x6c));
                    c.lwc1(3, r(c, S2).wrapping_add(0x70));
                    c.lwc1(4, r(c, S2).wrapping_add(0x74));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 6);
                    c.g[S0][0] = c.lw(r(c, S3).wrapping_add(RESOLVER_NASH));
                    write_bounds(c);
                    call_ptr(c, 0x0032_8f28, 0x003a_b030)?;
                    select_cell(c);
                    c.g[A1][0] = 0;
                    pc = 0x003a_b2a8;
                }
                // kinds 2/0: MDR model and material relocation slots.
                0x003a_b06c => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x28));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x2c));
                    pc = 0x003a_b2cc;
                }
                0x003a_b080 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x48));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x4c));
                    pc = 0x003a_b2cc;
                }
                // Additional spatial primitives (world light/effect data).
                0x003a_b094 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0xa8));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xac));
                    call_resolver_slot(c, 0x003a_b0ac)?;
                    let failed = c.g[V0][0] == 0;
                    c.g[V0][0] = 6;
                    if failed {
                        pc = 0x003a_b464;
                        continue;
                    }
                    c.cycles += 10;
                    c.lwc1(5, r(c, S2).wrapping_add(0x44));
                    c.sw(r(c, S2).wrapping_add(8), c.g[V0][0]);
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.lwc1(0, r(c, S2).wrapping_add(0x48));
                    c.g[A1][0] = c.g[SP][0];
                    c.lwc1(1, r(c, S2).wrapping_add(0x4c));
                    c.lwc1(2, r(c, S2).wrapping_add(0x50));
                    c.lwc1(3, r(c, S2).wrapping_add(0x54));
                    c.lwc1(4, r(c, S2).wrapping_add(0x58));
                    pc = 0x003a_b240;
                }
                0x003a_b0dc => {
                    c.cycles += 17;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xa4));
                    call_resolver_slot(c, 0x003a_b0f4)?;
                    c.g[V1][0] = 8;
                    c.lwc1(5, r(c, S2).wrapping_add(0x28));
                    c.sw(r(c, S2).wrapping_add(8), c.g[V1][0]);
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.lwc1(0, r(c, S2).wrapping_add(0x2c));
                    c.g[A1][0] = c.g[SP][0];
                    c.lwc1(1, r(c, S2).wrapping_add(0x30));
                    c.lwc1(2, r(c, S2).wrapping_add(0x34));
                    c.lwc1(3, r(c, S2).wrapping_add(0x38));
                    c.lwc1(4, r(c, S2).wrapping_add(0x3c));
                    pc = 0x003a_b240;
                }
                0x003a_b120 => {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[S4][0] = 0;
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x90));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x94));
                    call_resolver_slot(c, 0x003a_b13c)?;
                    c.g[V1][0] = c.lw(r(c, S2).wrapping_add(0x20));
                    let none = (c.g[V1][0] as i64) <= 0;
                    c.g[S1][0] = c.lw(r(c, S2).wrapping_add(0x24));
                    if none {
                        pc = 0x003a_b464;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 20);
                    c.g[S6][0] = 1;
                    c.g[S5][0] = u64::MAX;
                    pc = 0x003a_b158;
                }
                0x003a_b158 => {
                    c.cycles += 40;
                    c.lwc1(0, r(c, S1).wrapping_add(0x6c));
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.sw(r(c, S1).wrapping_add(8), c.g[S6][0]);
                    c.g[A1][0] = c.g[SP][0];
                    c.swc1(0, r(c, SP));
                    c.g[S4][0] = sx(r(c, S4).wrapping_add(1));
                    c.lwc1(0, r(c, S1).wrapping_add(0x70));
                    c.g[S0][0] = c.lw(r(c, S3).wrapping_add(RESOLVER_NASH));
                    c.swc1(0, r(c, SP).wrapping_add(4));
                    c.lwc1(1, r(c, S1).wrapping_add(0x74));
                    c.swc1(20, r(c, SP).wrapping_add(0xc));
                    c.swc1(1, r(c, SP).wrapping_add(8));
                    c.lwc1(0, r(c, S1).wrapping_add(0x78));
                    c.swc1(0, r(c, SP).wrapping_add(0x10));
                    c.lwc1(1, r(c, S1).wrapping_add(0x7c));
                    c.swc1(1, r(c, SP).wrapping_add(0x14));
                    c.lwc1(0, r(c, S1).wrapping_add(0x80));
                    c.swc1(20, r(c, SP).wrapping_add(0x1c));
                    c.swc1(0, r(c, SP).wrapping_add(0x18));
                    call_ptr(c, 0x0032_8f28, 0x003a_b1a8)?;
                    c.g[V0][0] = c.lw(r(c, SP).wrapping_add(0x24));
                    c.g[V1][0] = sx(r(c, S0).wrapping_add(0x50));
                    c.g[A1][0] = c.lw(r(c, SP).wrapping_add(0x28));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw(r(c, SP).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[A0][0] = sx(r(c, V1).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S5][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A1][0] != 0 {
                        c.g[A0][0] = c.g[V1][0];
                    }
                    c.g[A3][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.g[V0][0] = sx(r(c, A0).wrapping_add(0x14));
                    c.g[A1][0] = 2;
                    if c.g[T0][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    call_ptr(c, 0x0032_8c20, 0x003a_b1e8)?;
                    c.g[V0][0] = c.lw(r(c, S2).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = c.lw(r(c, S1).wrapping_add(0x64));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_b158);
                        }
                        pc = 0x003a_b158;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S2][0];
                        pc = 0x003a_b468;
                    }
                }
                0x003a_b200 => {
                    c.cycles += 16;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x88));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x8c));
                    call_resolver_slot(c, 0x003a_b218)?;
                    c.g[V1][0] = 7;
                    c.lwc1(5, r(c, S2).wrapping_add(0x68));
                    c.sw(r(c, S2).wrapping_add(8), c.g[V1][0]);
                    c.g[A0][0] = sx(r(c, SP).wrapping_add(0x20));
                    c.lwc1(0, r(c, S2).wrapping_add(0x6c));
                    c.g[A1][0] = c.g[SP][0];
                    c.lwc1(1, r(c, S2).wrapping_add(0x70));
                    c.lwc1(2, r(c, S2).wrapping_add(0x74));
                    c.lwc1(3, r(c, S2).wrapping_add(0x78));
                    c.lwc1(4, r(c, S2).wrapping_add(0x7c));
                    pc = 0x003a_b240;
                }
                0x003a_b240 => {
                    c.cycles += 26;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 6);
                    c.g[S0][0] = c.lw(r(c, S3).wrapping_add(RESOLVER_NASH));
                    write_bounds(c);
                    call_ptr(c, 0x0032_8f28, 0x003a_b270)?;
                    select_cell(c);
                    c.g[A1][0] = 2;
                    pc = 0x003a_b2a8;
                }
                0x003a_b2a8 => {
                    c.cycles += 5;
                    if c.g[A3][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    c.g[A3][0] = sx(r(c, SP).wrapping_add(0x20));
                    call_ptr(c, 0x0032_8c20, 0x003a_b2b4)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b2bc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x70));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x74));
                    pc = 0x003a_b2cc;
                }
                0x003a_b2cc => {
                    c.cycles += 4;
                    call_resolver_slot(c, 0x003a_b2d4)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                // kind 12: the virtual collision-data loader owns its layout.
                0x003a_b2dc => {
                    c.cycles += 9;
                    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0x98));
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x9c));
                    call_resolver_slot(c, 0x003a_b2f8)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b300 => {
                    c.cycles += 8;
                    call_ptr(c, 0x0028_b1c8, 0x003a_b308)?;
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x4c));
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x002b_6b08, 0x003a_b318)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b320 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0016_cec8, 0x003a_b32c)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b334 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, GP).wrapping_add(0x16c8));
                    let missing_world = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S0][0] & 0xff;
                    if missing_world {
                        pc = 0x003a_b464;
                        continue;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(8));
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x002b_b348, 0x003a_b34c)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b354 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r(c, GP).wrapping_add(0x16c8));
                    let missing_world = c.g[V0][0] == 0;
                    c.g[S0][0] &= 0xff;
                    if missing_world {
                        pc = 0x003a_b464;
                        continue;
                    }
                    c.cycles += 11;
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(4));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x003a_ca10, 0x003a_b370)?;
                    call_ptr(c, 0x003a_d188, 0x003a_b378)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_0430, 0x003a_b384)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b38c => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x84c));
                    c.g[A1][0] = c.g[S0][0] & 0xff;
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x0027_8a10, 0x003a_b39c)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b3a4 => {
                    c.cycles += 8;
                    call_ptr(c, 0x0028_b180, 0x003a_b3ac)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x0028_6ca8, 0x003a_b3bc)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b3c4 => {
                    c.cycles += 12;
                    call_ptr(c, 0x0014_dc80, 0x003a_b3cc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xa;
                    call_ptr(c, 0x0014_dd58, 0x003a_b3d8)?;
                    c.g[A3][0] = c.lw(r(c, S2));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0] & 0xff;
                    c.g[A2][0] = sx(r(c, S2).wrapping_add(4));
                    call_ptr(c, 0x0015_47d8, 0x003a_b3ec)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b3f4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0021_05d0, 0x003a_b3fc)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b404 => {
                    c.cycles += 7;
                    call_ptr(c, 0x003a_d198, 0x003a_b40c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0024_2540, 0x003a_b418)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                // kind 14: register AI/race paths, freeing rejected payloads.
                0x003a_b420 => {
                    c.cycles += 9;
                    c.g[V1][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                    c.g[A2][0] = sx(r(c, S0) >> 8);
                    c.g[A1][0] = c.g[S0][0] & 0xff;
                    c.g[A3][0] = c.g[S2][0];
                    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x84));
                    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0xc));
                    call_ptr(c, 0x0012_a340, 0x003a_b43c)?;
                    let accepted = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[S2][0];
                    if accepted {
                        pc = 0x003a_b468;
                        continue;
                    }
                    c.cycles += 2;
                    let null = c.g[S2][0] == 0;
                    frame.restore(c, S0, 0xa0);
                    if null {
                        pc = 0x003a_b46c;
                        continue;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_7e98, 0x003a_b454)?;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b45c => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002d_9fb8, 0x003a_b464)?;
                    pc = 0x003a_b464;
                }
                0x003a_b464 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                    pc = 0x003a_b468;
                }
                0x003a_b468 => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0xa0);
                    pc = 0x003a_b46c;
                }
                0x003a_b46c => {
                    c.cycles += 10;
                    frame.restore(c, S1, 0x90);
                    frame.restore(c, S2, 0x80);
                    frame.restore(c, S3, 0x70);
                    frame.restore(c, S4, 0x60);
                    frame.restore(c, S5, 0x50);
                    frame.restore(c, S6, 0x40);
                    frame.restore_ra(c, 0x30);
                    frame.restore_fpr(c, 20, 0xb0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003a_ae68, 0x003a_b494);
                    return Ok(());
                }
            }
        }
    })
}
