//! Native Nash volume traversal0x335d78, retained uninstalled.
//!
//! The native and unwind generated forms describe the same EE body; the
//! unwind block layout exposes every call return explicitly. Keep all eight
//! unrolled roots: their register setup, delay loads and cycle charges differ.
//! Reference only, with no override registration or automated test.

use crate::call::{call_ptr, guest};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }
#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}

/// `int Nash_CollideVolume(world, query, hits, capacity, ignore)`.
/// Build the native context `{query,hits,capacity,count*,ignore,cache}` at
/// stack+0, copy the query box to stack+20 and clear stack+40's hit count.
/// Classify each of eight root cells; a fully covered cell visits its static
/// instance list(+24), dynamic object list(+20), and all eight child subtrees.
/// A partial cell delegates the query/cell pair to recursive collision.
/// There is no surface/query-kind filter in this traversal itself. Instance,
/// object, subtree and recursive callees retain those decisions. Ignore data
/// is passed through the context, and contact capacity is not a root filter.
/// Native context: SP+0 query, +4 hits, +8 capacity, +c count pointer,
/// +10 ignore, +14 world cache; S4 is world, S1 is the context, S2 a root node.
/// The frame is100hex bytes, with saved RA at60 and saved GPRs70..f0.
pub fn nash_collide_volume(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        match pc {
            0x00335f44 => c.cycles += 2,
            0x00335f74 => c.cycles += 1,
            _ => {}
        }
        loop {
            match pc {
                // Build the native volume-query context and classify root0.
                0x00335d78 => {
                    c.cycles += 28;
                    c.g[SP][0] = sx((r(c, SP)).wrapping_sub(0x100));
                    c.sq(S1, at(c, SP, 0xe0));
                    c.g[V1][0] = sx(at(c, SP, 0x40));
                    c.sq(S4, at(c, SP, 0xb0));
                    c.g[S1][0] = c.g[SP][0];
                    c.sq(S0, at(c, SP, 0xf0));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(S2, at(c, SP, 0xd0));
                    c.sq(S3, at(c, SP, 0xc0));
                    c.sq(S5, at(c, SP, 0xa0));
                    c.sq(S6, at(c, SP, 0x90));
                    c.sq(S7, at(c, SP, 0x80));
                    c.sq(FP, at(c, SP, 0x70));
                    c.sd(at(c, SP, 0x60), c.g[RA][0]);
                    c.lq(A0, at(c, A1, 0x20));
                    c.g[T1][0] = c.lw(at(c, S4, 0xa4));
                    c.lq(V0, at(c, A1, 0x30));
                    c.g[S2][0] = c.lw(at(c, S4, 0x10));
                    c.sw(at(c, SP, 0x4), c.g[A2][0]);
                    c.sw(at(c, SP, 0x8), c.g[A3][0]);
                    c.sw(at(c, SP, 0xc), c.g[V1][0]);
                    c.sw(at(c, SP, 0x10), c.g[T0][0]);
                    c.sw(at(c, SP, 0x14), c.g[T1][0]);
                    c.sq(V0, at(c, SP, 0x20));
                    c.sq(A0, at(c, SP, 0x30));
                    c.sw((r(c, SP)), c.g[A1][0]);
                    let t = c.g[S2][0] == 0u64; 
                    c.sw(at(c, SP, 0x40), 0u64);
                    if t { pc = 0x00335f20; continue; }
                    pc = 0x00335de8; continue;
                }
                0x00335de8 => {
                    c.cycles += 7;
                    c.g[S0][0] = sx(at(c, SP, 0x44));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x00335df8);
                    c.g[A1][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x00328360, 0x00335df8)?;
                    pc = 0x00335df8; continue;
                }
                0x00335df8 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64; 
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x00335efc; continue; }
                    pc = 0x00335e04; continue;
                }
                0x00335e04 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, 0x24));
                
                    if c.g[S0][0] == 0u64 { c.g[S0][0] = c.lw(at(c, S2, 0x20)); pc = 0x00335e30; continue; }
                    pc = 0x00335e10; continue;
                }
                0x00335e10 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00335e18; continue;
                }
                0x00335e18 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00335e20);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335960, 0x00335e20)?;
                    pc = 0x00335e20; continue;
                }
                0x00335e20 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00335e18) { return Err(crate::call::Unwound); } pc = 0x00335e18; continue; }
                    pc = 0x00335e2c; continue;
                }
                0x00335e2c => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x00335e30; continue;
                }
                0x00335e30 => {
                    c.cycles += 2;
                
                    if c.g[S0][0] == 0u64 { c.g[A0][0] = c.lw((r(c, S2))); pc = 0x00335e58; continue; }
                    pc = 0x00335e38; continue;
                }
                0x00335e38 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00335e40; continue;
                }
                0x00335e40 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00335e48);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335b90, 0x00335e48)?;
                    pc = 0x00335e48; continue;
                }
                0x00335e48 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00335e40) { return Err(crate::call::Unwound); } pc = 0x00335e40; continue; }
                    pc = 0x00335e54; continue;
                }
                0x00335e54 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((r(c, S2)));
                    pc = 0x00335e58; continue;
                }
                0x00335e58 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x00335e6c; continue; }
                    pc = 0x00335e60; continue;
                }
                0x00335e60 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335e68);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335e68)?;
                    pc = 0x00335e68; continue;
                }
                0x00335e68 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x4));
                    pc = 0x00335e6c; continue;
                }
                0x00335e6c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x00335e80; continue; }
                    pc = 0x00335e74; continue;
                }
                0x00335e74 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335e7c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335e7c)?;
                    pc = 0x00335e7c; continue;
                }
                0x00335e7c => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x8));
                    pc = 0x00335e80; continue;
                }
                0x00335e80 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x00335e94; continue; }
                    pc = 0x00335e88; continue;
                }
                0x00335e88 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335e90);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335e90)?;
                    pc = 0x00335e90; continue;
                }
                0x00335e90 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0xc));
                    pc = 0x00335e94; continue;
                }
                0x00335e94 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x00335ea8; continue; }
                    pc = 0x00335e9c; continue;
                }
                0x00335e9c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335ea4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335ea4)?;
                    pc = 0x00335ea4; continue;
                }
                0x00335ea4 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x10));
                    pc = 0x00335ea8; continue;
                }
                0x00335ea8 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x00335ebc; continue; }
                    pc = 0x00335eb0; continue;
                }
                0x00335eb0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335eb8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335eb8)?;
                    pc = 0x00335eb8; continue;
                }
                0x00335eb8 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x00335ebc; continue;
                }
                0x00335ebc => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x00335ed0; continue; }
                    pc = 0x00335ec4; continue;
                }
                0x00335ec4 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335ecc);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335ecc)?;
                    pc = 0x00335ecc; continue;
                }
                0x00335ecc => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x18));
                    pc = 0x00335ed0; continue;
                }
                0x00335ed0 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x00335ee4; continue; }
                    pc = 0x00335ed8; continue;
                }
                0x00335ed8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335ee0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335ee0)?;
                    pc = 0x00335ee0; continue;
                }
                0x00335ee0 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                    pc = 0x00335ee4; continue;
                }
                0x00335ee4 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S4, 0x24)); pc = 0x00335f24; continue; }
                    pc = 0x00335eec; continue;
                }
                0x00335eec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00335ef4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335ef4)?;
                    pc = 0x00335ef4; continue;
                }
                0x00335ef4 => {
                    let t = true; 
                    c.g[V0][0] = c.lw(at(c, S4, 0x24));
                    if t { pc = 0x00335f24; continue; }
                    pc = 0x00335efc; continue;
                }
                0x00335efc => {
                    c.cycles += 2;
                
                    if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S4, 0x24)); pc = 0x00335f24; continue; }
                    pc = 0x00335f04; continue;
                }
                0x00335f04 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    c.g[RA][0] = sx(0x00335f20);
                    c.g[T1][0] = c.g[S4][0];
                    call_ptr(c, 0x0033ccf8, 0x00335f20)?;
                    pc = 0x00335f20; continue;
                }
                0x00335f20 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S4, 0x24));
                    pc = 0x00335f24; continue;
                }
                // Prepare the seven remaining native root descriptors.
                0x00335f24 => {
                    c.cycles += 10;
                    c.g[V1][0] = sx(at(c, S4, 0x78));
                    c.sw(at(c, SP, 0x4c), c.g[V1][0]);
                    c.g[S0][0] = sx(at(c, S4, 0x14));
                    c.g[V1][0] = sx(at(c, S4, 0x8c));
                    c.g[S5][0] = sx(at(c, S4, 0x28));
                    c.g[S6][0] = sx(at(c, S4, 0x3c));
                    c.g[S7][0] = sx(at(c, S4, 0x50));
                    c.g[FP][0] = sx(at(c, S4, 0x64));
                    pc = 0x00335f44; continue;
                }
                0x00335f44 => {
                    let t = c.g[V0][0] == 0u64; 
                    c.sw(at(c, SP, 0x50), c.g[V1][0]);
                    if t { pc = 0x00336088; continue; }
                    pc = 0x00335f4c; continue;
                }
                0x00335f4c => {
                    c.cycles += 8;
                    c.g[S2][0] = c.lw(at(c, S0, 0x10));
                    c.g[S3][0] = sx(at(c, SP, 0x45));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00335f60);
                    c.g[A1][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x00328360, 0x00335f60)?;
                    pc = 0x00335f60; continue;
                }
                0x00335f60 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64; 
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x00336064; continue; }
                    pc = 0x00335f6c; continue;
                }
                0x00335f6c => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, 0x24));
                
                    if c.g[S0][0] == 0u64 { c.g[S0][0] = c.lw(at(c, S2, 0x20)); pc = 0x00335f98; continue; }
                    pc = 0x00335f78; continue;
                }
                0x00335f74 => {
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x00335f78; continue;
                }
                0x00335f78 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00335f80; continue;
                }
                0x00335f80 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00335f88);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335960, 0x00335f88)?;
                    pc = 0x00335f88; continue;
                }
                0x00335f88 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00335f80) { return Err(crate::call::Unwound); } pc = 0x00335f80; continue; }
                    pc = 0x00335f94; continue;
                }
                0x00335f94 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x00335f98; continue;
                }
                0x00335f98 => {
                    c.cycles += 2;
                
                    if c.g[S0][0] == 0u64 { c.g[A0][0] = c.lw((r(c, S2))); pc = 0x00335fc0; continue; }
                    pc = 0x00335fa0; continue;
                }
                0x00335fa0 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00335fa8; continue;
                }
                0x00335fa8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00335fb0);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335b90, 0x00335fb0)?;
                    pc = 0x00335fb0; continue;
                }
                0x00335fb0 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00335fa8) { return Err(crate::call::Unwound); } pc = 0x00335fa8; continue; }
                    pc = 0x00335fbc; continue;
                }
                0x00335fbc => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((r(c, S2)));
                    pc = 0x00335fc0; continue;
                }
                0x00335fc0 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x00335fd4; continue; }
                    pc = 0x00335fc8; continue;
                }
                0x00335fc8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335fd0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335fd0)?;
                    pc = 0x00335fd0; continue;
                }
                0x00335fd0 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x4));
                    pc = 0x00335fd4; continue;
                }
                0x00335fd4 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x00335fe8; continue; }
                    pc = 0x00335fdc; continue;
                }
                0x00335fdc => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335fe4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335fe4)?;
                    pc = 0x00335fe4; continue;
                }
                0x00335fe4 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x8));
                    pc = 0x00335fe8; continue;
                }
                0x00335fe8 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x00335ffc; continue; }
                    pc = 0x00335ff0; continue;
                }
                0x00335ff0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00335ff8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00335ff8)?;
                    pc = 0x00335ff8; continue;
                }
                0x00335ff8 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0xc));
                    pc = 0x00335ffc; continue;
                }
                0x00335ffc => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x00336010; continue; }
                    pc = 0x00336004; continue;
                }
                0x00336004 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0033600c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033600c)?;
                    pc = 0x0033600c; continue;
                }
                0x0033600c => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x10));
                    pc = 0x00336010; continue;
                }
                0x00336010 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x00336024; continue; }
                    pc = 0x00336018; continue;
                }
                0x00336018 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336020);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336020)?;
                    pc = 0x00336020; continue;
                }
                0x00336020 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x00336024; continue;
                }
                0x00336024 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x00336038; continue; }
                    pc = 0x0033602c; continue;
                }
                0x0033602c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336034);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336034)?;
                    pc = 0x00336034; continue;
                }
                0x00336034 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x18));
                    pc = 0x00336038; continue;
                }
                0x00336038 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x0033604c; continue; }
                    pc = 0x00336040; continue;
                }
                0x00336040 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336048);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336048)?;
                    pc = 0x00336048; continue;
                }
                0x00336048 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                    pc = 0x0033604c; continue;
                }
                0x0033604c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S4, 0x38)); pc = 0x0033608c; continue; }
                    pc = 0x00336054; continue;
                }
                0x00336054 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0033605c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033605c)?;
                    pc = 0x0033605c; continue;
                }
                0x0033605c => {
                    let t = true; 
                    c.g[V0][0] = c.lw(at(c, S4, 0x38));
                    if t { pc = 0x0033608c; continue; }
                    pc = 0x00336064; continue;
                }
                0x00336064 => {
                    c.cycles += 2;
                
                    if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S4, 0x38)); pc = 0x0033608c; continue; }
                    pc = 0x0033606c; continue;
                }
                0x0033606c => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[T1][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00336088);
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x0033ccf8, 0x00336088)?;
                    pc = 0x00336088; continue;
                }
                0x00336088 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S4, 0x38));
                    pc = 0x0033608c; continue;
                }
                // Root2.
                0x0033608c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64; 
                    c.g[S0][0] = sx(at(c, SP, 0x46));
                    if t { pc = 0x003361c8; continue; }
                    pc = 0x00336094; continue;
                }
                0x00336094 => {
                    c.cycles += 7;
                    c.g[S2][0] = c.lw(at(c, S5, 0x10));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x003360a4);
                    c.g[A1][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x00328360, 0x003360a4)?;
                    pc = 0x003360a4; continue;
                }
                0x003360a4 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64; 
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x003361a4; continue; }
                    pc = 0x003360b0; continue;
                }
                0x003360b0 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, 0x24));
                
                    if c.g[S0][0] == 0u64 { c.g[S0][0] = c.lw(at(c, S2, 0x20)); pc = 0x003360d8; continue; }
                    pc = 0x003360bc; continue;
                }
                0x003360bc => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x003360c0; continue;
                }
                0x003360c0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003360c8);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335960, 0x003360c8)?;
                    pc = 0x003360c8; continue;
                }
                0x003360c8 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x003360c0) { return Err(crate::call::Unwound); } pc = 0x003360c0; continue; }
                    pc = 0x003360d4; continue;
                }
                0x003360d4 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x003360d8; continue;
                }
                0x003360d8 => {
                    c.cycles += 2;
                
                    if c.g[S0][0] == 0u64 { c.g[A0][0] = c.lw((r(c, S2))); pc = 0x00336100; continue; }
                    pc = 0x003360e0; continue;
                }
                0x003360e0 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x003360e8; continue;
                }
                0x003360e8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003360f0);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335b90, 0x003360f0)?;
                    pc = 0x003360f0; continue;
                }
                0x003360f0 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x003360e8) { return Err(crate::call::Unwound); } pc = 0x003360e8; continue; }
                    pc = 0x003360fc; continue;
                }
                0x003360fc => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((r(c, S2)));
                    pc = 0x00336100; continue;
                }
                0x00336100 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x00336114; continue; }
                    pc = 0x00336108; continue;
                }
                0x00336108 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336110);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336110)?;
                    pc = 0x00336110; continue;
                }
                0x00336110 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x4));
                    pc = 0x00336114; continue;
                }
                0x00336114 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x00336128; continue; }
                    pc = 0x0033611c; continue;
                }
                0x0033611c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336124);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336124)?;
                    pc = 0x00336124; continue;
                }
                0x00336124 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x8));
                    pc = 0x00336128; continue;
                }
                0x00336128 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x0033613c; continue; }
                    pc = 0x00336130; continue;
                }
                0x00336130 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336138);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336138)?;
                    pc = 0x00336138; continue;
                }
                0x00336138 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0xc));
                    pc = 0x0033613c; continue;
                }
                0x0033613c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x00336150; continue; }
                    pc = 0x00336144; continue;
                }
                0x00336144 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0033614c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033614c)?;
                    pc = 0x0033614c; continue;
                }
                0x0033614c => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x10));
                    pc = 0x00336150; continue;
                }
                0x00336150 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x00336164; continue; }
                    pc = 0x00336158; continue;
                }
                0x00336158 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336160);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336160)?;
                    pc = 0x00336160; continue;
                }
                0x00336160 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x00336164; continue;
                }
                0x00336164 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x00336178; continue; }
                    pc = 0x0033616c; continue;
                }
                0x0033616c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336174);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336174)?;
                    pc = 0x00336174; continue;
                }
                0x00336174 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x18));
                    pc = 0x00336178; continue;
                }
                0x00336178 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x0033618c; continue; }
                    pc = 0x00336180; continue;
                }
                0x00336180 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336188);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336188)?;
                    pc = 0x00336188; continue;
                }
                0x00336188 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                    pc = 0x0033618c; continue;
                }
                0x0033618c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S4, 0x4c)); pc = 0x003361cc; continue; }
                    pc = 0x00336194; continue;
                }
                0x00336194 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0033619c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033619c)?;
                    pc = 0x0033619c; continue;
                }
                0x0033619c => {
                    let t = true; 
                    c.g[V0][0] = c.lw(at(c, S4, 0x4c));
                    if t { pc = 0x003361cc; continue; }
                    pc = 0x003361a4; continue;
                }
                0x003361a4 => {
                    c.cycles += 2;
                
                    if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S4, 0x4c)); pc = 0x003361cc; continue; }
                    pc = 0x003361ac; continue;
                }
                0x003361ac => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[T1][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003361c8);
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x0033ccf8, 0x003361c8)?;
                    pc = 0x003361c8; continue;
                }
                0x003361c8 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S4, 0x4c));
                    pc = 0x003361cc; continue;
                }
                // Root3.
                0x003361cc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64; 
                    c.g[S0][0] = sx(at(c, SP, 0x47));
                    if t { pc = 0x00336308; continue; }
                    pc = 0x003361d4; continue;
                }
                0x003361d4 => {
                    c.cycles += 7;
                    c.g[S2][0] = c.lw(at(c, S6, 0x10));
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x003361e4);
                    c.g[A1][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x00328360, 0x003361e4)?;
                    pc = 0x003361e4; continue;
                }
                0x003361e4 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64; 
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x003362e4; continue; }
                    pc = 0x003361f0; continue;
                }
                0x003361f0 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, 0x24));
                
                    if c.g[S0][0] == 0u64 { c.g[S0][0] = c.lw(at(c, S2, 0x20)); pc = 0x00336218; continue; }
                    pc = 0x003361fc; continue;
                }
                0x003361fc => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00336200; continue;
                }
                0x00336200 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00336208);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335960, 0x00336208)?;
                    pc = 0x00336208; continue;
                }
                0x00336208 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00336200) { return Err(crate::call::Unwound); } pc = 0x00336200; continue; }
                    pc = 0x00336214; continue;
                }
                0x00336214 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x00336218; continue;
                }
                0x00336218 => {
                    c.cycles += 2;
                
                    if c.g[S0][0] == 0u64 { c.g[A0][0] = c.lw((r(c, S2))); pc = 0x00336240; continue; }
                    pc = 0x00336220; continue;
                }
                0x00336220 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00336228; continue;
                }
                0x00336228 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00336230);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335b90, 0x00336230)?;
                    pc = 0x00336230; continue;
                }
                0x00336230 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00336228) { return Err(crate::call::Unwound); } pc = 0x00336228; continue; }
                    pc = 0x0033623c; continue;
                }
                0x0033623c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((r(c, S2)));
                    pc = 0x00336240; continue;
                }
                0x00336240 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x00336254; continue; }
                    pc = 0x00336248; continue;
                }
                0x00336248 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336250);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336250)?;
                    pc = 0x00336250; continue;
                }
                0x00336250 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x4));
                    pc = 0x00336254; continue;
                }
                0x00336254 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x00336268; continue; }
                    pc = 0x0033625c; continue;
                }
                0x0033625c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336264);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336264)?;
                    pc = 0x00336264; continue;
                }
                0x00336264 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x8));
                    pc = 0x00336268; continue;
                }
                0x00336268 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x0033627c; continue; }
                    pc = 0x00336270; continue;
                }
                0x00336270 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336278);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336278)?;
                    pc = 0x00336278; continue;
                }
                0x00336278 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0xc));
                    pc = 0x0033627c; continue;
                }
                0x0033627c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x00336290; continue; }
                    pc = 0x00336284; continue;
                }
                0x00336284 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0033628c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033628c)?;
                    pc = 0x0033628c; continue;
                }
                0x0033628c => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x10));
                    pc = 0x00336290; continue;
                }
                0x00336290 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x003362a4; continue; }
                    pc = 0x00336298; continue;
                }
                0x00336298 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003362a0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003362a0)?;
                    pc = 0x003362a0; continue;
                }
                0x003362a0 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x003362a4; continue;
                }
                0x003362a4 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x003362b8; continue; }
                    pc = 0x003362ac; continue;
                }
                0x003362ac => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003362b4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003362b4)?;
                    pc = 0x003362b4; continue;
                }
                0x003362b4 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x18));
                    pc = 0x003362b8; continue;
                }
                0x003362b8 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x003362cc; continue; }
                    pc = 0x003362c0; continue;
                }
                0x003362c0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003362c8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003362c8)?;
                    pc = 0x003362c8; continue;
                }
                0x003362c8 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                    pc = 0x003362cc; continue;
                }
                0x003362cc => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S4, 0x60)); pc = 0x0033630c; continue; }
                    pc = 0x003362d4; continue;
                }
                0x003362d4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003362dc);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003362dc)?;
                    pc = 0x003362dc; continue;
                }
                0x003362dc => {
                    let t = true; 
                    c.g[V0][0] = c.lw(at(c, S4, 0x60));
                    if t { pc = 0x0033630c; continue; }
                    pc = 0x003362e4; continue;
                }
                0x003362e4 => {
                    c.cycles += 2;
                
                    if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S4, 0x60)); pc = 0x0033630c; continue; }
                    pc = 0x003362ec; continue;
                }
                0x003362ec => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[T1][0] = c.g[S6][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00336308);
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x0033ccf8, 0x00336308)?;
                    pc = 0x00336308; continue;
                }
                0x00336308 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S4, 0x60));
                    pc = 0x0033630c; continue;
                }
                // Root4.
                0x0033630c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64; 
                    c.g[S0][0] = sx(at(c, SP, 0x48));
                    if t { pc = 0x00336448; continue; }
                    pc = 0x00336314; continue;
                }
                0x00336314 => {
                    c.cycles += 7;
                    c.g[S2][0] = c.lw(at(c, S7, 0x10));
                    c.g[A0][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x00336324);
                    c.g[A1][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x00328360, 0x00336324)?;
                    pc = 0x00336324; continue;
                }
                0x00336324 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64; 
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x00336424; continue; }
                    pc = 0x00336330; continue;
                }
                0x00336330 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, 0x24));
                
                    if c.g[S0][0] == 0u64 { c.g[S0][0] = c.lw(at(c, S2, 0x20)); pc = 0x00336358; continue; }
                    pc = 0x0033633c; continue;
                }
                0x0033633c => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00336340; continue;
                }
                0x00336340 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00336348);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335960, 0x00336348)?;
                    pc = 0x00336348; continue;
                }
                0x00336348 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00336340) { return Err(crate::call::Unwound); } pc = 0x00336340; continue; }
                    pc = 0x00336354; continue;
                }
                0x00336354 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x00336358; continue;
                }
                0x00336358 => {
                    c.cycles += 2;
                
                    if c.g[S0][0] == 0u64 { c.g[A0][0] = c.lw((r(c, S2))); pc = 0x00336380; continue; }
                    pc = 0x00336360; continue;
                }
                0x00336360 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00336368; continue;
                }
                0x00336368 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00336370);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335b90, 0x00336370)?;
                    pc = 0x00336370; continue;
                }
                0x00336370 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00336368) { return Err(crate::call::Unwound); } pc = 0x00336368; continue; }
                    pc = 0x0033637c; continue;
                }
                0x0033637c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((r(c, S2)));
                    pc = 0x00336380; continue;
                }
                0x00336380 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x00336394; continue; }
                    pc = 0x00336388; continue;
                }
                0x00336388 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336390);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336390)?;
                    pc = 0x00336390; continue;
                }
                0x00336390 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x4));
                    pc = 0x00336394; continue;
                }
                0x00336394 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x003363a8; continue; }
                    pc = 0x0033639c; continue;
                }
                0x0033639c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003363a4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003363a4)?;
                    pc = 0x003363a4; continue;
                }
                0x003363a4 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x8));
                    pc = 0x003363a8; continue;
                }
                0x003363a8 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x003363bc; continue; }
                    pc = 0x003363b0; continue;
                }
                0x003363b0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003363b8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003363b8)?;
                    pc = 0x003363b8; continue;
                }
                0x003363b8 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0xc));
                    pc = 0x003363bc; continue;
                }
                0x003363bc => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x003363d0; continue; }
                    pc = 0x003363c4; continue;
                }
                0x003363c4 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003363cc);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003363cc)?;
                    pc = 0x003363cc; continue;
                }
                0x003363cc => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x10));
                    pc = 0x003363d0; continue;
                }
                0x003363d0 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x003363e4; continue; }
                    pc = 0x003363d8; continue;
                }
                0x003363d8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003363e0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003363e0)?;
                    pc = 0x003363e0; continue;
                }
                0x003363e0 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x003363e4; continue;
                }
                0x003363e4 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x003363f8; continue; }
                    pc = 0x003363ec; continue;
                }
                0x003363ec => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003363f4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003363f4)?;
                    pc = 0x003363f4; continue;
                }
                0x003363f4 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x18));
                    pc = 0x003363f8; continue;
                }
                0x003363f8 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x0033640c; continue; }
                    pc = 0x00336400; continue;
                }
                0x00336400 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336408);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336408)?;
                    pc = 0x00336408; continue;
                }
                0x00336408 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                    pc = 0x0033640c; continue;
                }
                0x0033640c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S4, 0x74)); pc = 0x0033644c; continue; }
                    pc = 0x00336414; continue;
                }
                0x00336414 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0033641c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033641c)?;
                    pc = 0x0033641c; continue;
                }
                0x0033641c => {
                    let t = true; 
                    c.g[V0][0] = c.lw(at(c, S4, 0x74));
                    if t { pc = 0x0033644c; continue; }
                    pc = 0x00336424; continue;
                }
                0x00336424 => {
                    c.cycles += 2;
                
                    if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S4, 0x74)); pc = 0x0033644c; continue; }
                    pc = 0x0033642c; continue;
                }
                0x0033642c => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[T1][0] = c.g[S7][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00336448);
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x0033ccf8, 0x00336448)?;
                    pc = 0x00336448; continue;
                }
                0x00336448 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S4, 0x74));
                    pc = 0x0033644c; continue;
                }
                // Root5.
                0x0033644c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64; 
                    c.g[S0][0] = sx(at(c, SP, 0x49));
                    if t { pc = 0x00336588; continue; }
                    pc = 0x00336454; continue;
                }
                0x00336454 => {
                    c.cycles += 7;
                    c.g[S2][0] = c.lw(at(c, FP, 0x10));
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x00336464);
                    c.g[A1][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x00328360, 0x00336464)?;
                    pc = 0x00336464; continue;
                }
                0x00336464 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64; 
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x00336564; continue; }
                    pc = 0x00336470; continue;
                }
                0x00336470 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, 0x24));
                
                    if c.g[S0][0] == 0u64 { c.g[S0][0] = c.lw(at(c, S2, 0x20)); pc = 0x00336498; continue; }
                    pc = 0x0033647c; continue;
                }
                0x0033647c => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00336480; continue;
                }
                0x00336480 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00336488);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335960, 0x00336488)?;
                    pc = 0x00336488; continue;
                }
                0x00336488 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00336480) { return Err(crate::call::Unwound); } pc = 0x00336480; continue; }
                    pc = 0x00336494; continue;
                }
                0x00336494 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x00336498; continue;
                }
                0x00336498 => {
                    c.cycles += 2;
                
                    if c.g[S0][0] == 0u64 { c.g[A0][0] = c.lw((r(c, S2))); pc = 0x003364c0; continue; }
                    pc = 0x003364a0; continue;
                }
                0x003364a0 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x003364a8; continue;
                }
                0x003364a8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003364b0);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335b90, 0x003364b0)?;
                    pc = 0x003364b0; continue;
                }
                0x003364b0 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x003364a8) { return Err(crate::call::Unwound); } pc = 0x003364a8; continue; }
                    pc = 0x003364bc; continue;
                }
                0x003364bc => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((r(c, S2)));
                    pc = 0x003364c0; continue;
                }
                0x003364c0 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x003364d4; continue; }
                    pc = 0x003364c8; continue;
                }
                0x003364c8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003364d0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003364d0)?;
                    pc = 0x003364d0; continue;
                }
                0x003364d0 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x4));
                    pc = 0x003364d4; continue;
                }
                0x003364d4 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x003364e8; continue; }
                    pc = 0x003364dc; continue;
                }
                0x003364dc => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003364e4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003364e4)?;
                    pc = 0x003364e4; continue;
                }
                0x003364e4 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x8));
                    pc = 0x003364e8; continue;
                }
                0x003364e8 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x003364fc; continue; }
                    pc = 0x003364f0; continue;
                }
                0x003364f0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003364f8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003364f8)?;
                    pc = 0x003364f8; continue;
                }
                0x003364f8 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0xc));
                    pc = 0x003364fc; continue;
                }
                0x003364fc => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x00336510; continue; }
                    pc = 0x00336504; continue;
                }
                0x00336504 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0033650c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033650c)?;
                    pc = 0x0033650c; continue;
                }
                0x0033650c => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x10));
                    pc = 0x00336510; continue;
                }
                0x00336510 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x00336524; continue; }
                    pc = 0x00336518; continue;
                }
                0x00336518 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336520);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336520)?;
                    pc = 0x00336520; continue;
                }
                0x00336520 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x00336524; continue;
                }
                0x00336524 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x00336538; continue; }
                    pc = 0x0033652c; continue;
                }
                0x0033652c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336534);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336534)?;
                    pc = 0x00336534; continue;
                }
                0x00336534 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x18));
                    pc = 0x00336538; continue;
                }
                0x00336538 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x0033654c; continue; }
                    pc = 0x00336540; continue;
                }
                0x00336540 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336548);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336548)?;
                    pc = 0x00336548; continue;
                }
                0x00336548 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                    pc = 0x0033654c; continue;
                }
                0x0033654c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S4, 0x88)); pc = 0x0033658c; continue; }
                    pc = 0x00336554; continue;
                }
                0x00336554 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0033655c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033655c)?;
                    pc = 0x0033655c; continue;
                }
                0x0033655c => {
                    let t = true; 
                    c.g[V0][0] = c.lw(at(c, S4, 0x88));
                    if t { pc = 0x0033658c; continue; }
                    pc = 0x00336564; continue;
                }
                0x00336564 => {
                    c.cycles += 2;
                
                    if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S4, 0x88)); pc = 0x0033658c; continue; }
                    pc = 0x0033656c; continue;
                }
                0x0033656c => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[T1][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00336588);
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x0033ccf8, 0x00336588)?;
                    pc = 0x00336588; continue;
                }
                0x00336588 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S4, 0x88));
                    pc = 0x0033658c; continue;
                }
                // Root6, whose descriptor pointer was saved on the stack.
                0x0033658c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64; 
                    c.g[V0][0] = c.lw(at(c, SP, 0x4c));
                    if t { pc = 0x003366d0; continue; }
                    pc = 0x00336594; continue;
                }
                0x00336594 => {
                    c.cycles += 8;
                    c.g[S0][0] = sx(at(c, SP, 0x4a));
                    c.g[A1][0] = sx(at(c, SP, 0x20));
                    c.g[S2][0] = c.lw(at(c, V0, 0x10));
                    c.g[RA][0] = sx(0x003365a8);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00328360, 0x003365a8)?;
                    pc = 0x003365a8; continue;
                }
                0x003365a8 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64; 
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x003366ac; continue; }
                    pc = 0x003365b4; continue;
                }
                0x003365b4 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, 0x24));
                
                    if c.g[S0][0] == 0u64 { c.g[S0][0] = c.lw(at(c, S2, 0x20)); pc = 0x003365e0; continue; }
                    pc = 0x003365c0; continue;
                }
                0x003365c0 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x003365c8; continue;
                }
                0x003365c8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003365d0);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335960, 0x003365d0)?;
                    pc = 0x003365d0; continue;
                }
                0x003365d0 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x003365c8) { return Err(crate::call::Unwound); } pc = 0x003365c8; continue; }
                    pc = 0x003365dc; continue;
                }
                0x003365dc => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x003365e0; continue;
                }
                0x003365e0 => {
                    c.cycles += 2;
                
                    if c.g[S0][0] == 0u64 { c.g[A0][0] = c.lw((r(c, S2))); pc = 0x00336608; continue; }
                    pc = 0x003365e8; continue;
                }
                0x003365e8 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x003365f0; continue;
                }
                0x003365f0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003365f8);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335b90, 0x003365f8)?;
                    pc = 0x003365f8; continue;
                }
                0x003365f8 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x003365f0) { return Err(crate::call::Unwound); } pc = 0x003365f0; continue; }
                    pc = 0x00336604; continue;
                }
                0x00336604 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((r(c, S2)));
                    pc = 0x00336608; continue;
                }
                0x00336608 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x0033661c; continue; }
                    pc = 0x00336610; continue;
                }
                0x00336610 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336618);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336618)?;
                    pc = 0x00336618; continue;
                }
                0x00336618 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x4));
                    pc = 0x0033661c; continue;
                }
                0x0033661c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x00336630; continue; }
                    pc = 0x00336624; continue;
                }
                0x00336624 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0033662c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033662c)?;
                    pc = 0x0033662c; continue;
                }
                0x0033662c => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x8));
                    pc = 0x00336630; continue;
                }
                0x00336630 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x00336644; continue; }
                    pc = 0x00336638; continue;
                }
                0x00336638 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336640);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336640)?;
                    pc = 0x00336640; continue;
                }
                0x00336640 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0xc));
                    pc = 0x00336644; continue;
                }
                0x00336644 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x00336658; continue; }
                    pc = 0x0033664c; continue;
                }
                0x0033664c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336654);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336654)?;
                    pc = 0x00336654; continue;
                }
                0x00336654 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x10));
                    pc = 0x00336658; continue;
                }
                0x00336658 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x0033666c; continue; }
                    pc = 0x00336660; continue;
                }
                0x00336660 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336668);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336668)?;
                    pc = 0x00336668; continue;
                }
                0x00336668 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x0033666c; continue;
                }
                0x0033666c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x00336680; continue; }
                    pc = 0x00336674; continue;
                }
                0x00336674 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0033667c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033667c)?;
                    pc = 0x0033667c; continue;
                }
                0x0033667c => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x18));
                    pc = 0x00336680; continue;
                }
                0x00336680 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x00336694; continue; }
                    pc = 0x00336688; continue;
                }
                0x00336688 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336690);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336690)?;
                    pc = 0x00336690; continue;
                }
                0x00336690 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                    pc = 0x00336694; continue;
                }
                0x00336694 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S4, 0x9c)); pc = 0x003366d4; continue; }
                    pc = 0x0033669c; continue;
                }
                0x0033669c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003366a4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003366a4)?;
                    pc = 0x003366a4; continue;
                }
                0x003366a4 => {
                    let t = true; 
                    c.g[V0][0] = c.lw(at(c, S4, 0x9c));
                    if t { pc = 0x003366d4; continue; }
                    pc = 0x003366ac; continue;
                }
                0x003366ac => {
                    c.cycles += 2;
                
                    if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S4, 0x9c)); pc = 0x003366d4; continue; }
                    pc = 0x003366b4; continue;
                }
                0x003366b4 => {
                    c.cycles += 7;
                    c.g[T1][0] = c.lw(at(c, SP, 0x4c));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003366d0);
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x0033ccf8, 0x003366d0)?;
                    pc = 0x003366d0; continue;
                }
                0x003366d0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, S4, 0x9c));
                    pc = 0x003366d4; continue;
                }
                // Root7, also saved on the stack.
                0x003366d4 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64; 
                    c.g[V1][0] = c.lw(at(c, SP, 0x50));
                    if t { pc = 0x00336818; continue; }
                    pc = 0x003366dc; continue;
                }
                0x003366dc => {
                    c.cycles += 8;
                    c.g[S0][0] = sx(at(c, SP, 0x4b));
                    c.g[A1][0] = sx(at(c, SP, 0x20));
                    c.g[S2][0] = c.lw(at(c, V1, 0x10));
                    c.g[RA][0] = sx(0x003366f0);
                    c.g[A0][0] = c.g[V1][0];
                    call_ptr(c, 0x00328360, 0x003366f0)?;
                    pc = 0x003366f0; continue;
                }
                0x003366f0 => {
                    c.g[V1][0] = c.g[V0][0];
                    let t = c.g[V1][0] != 0u64; 
                    c.g[V0][0] = 0x2;
                    if t { pc = 0x003367f4; continue; }
                    pc = 0x003366fc; continue;
                }
                0x003366fc => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, S2, 0x24));
                
                    if c.g[S0][0] == 0u64 { c.g[S0][0] = c.lw(at(c, S2, 0x20)); pc = 0x00336728; continue; }
                    pc = 0x00336708; continue;
                }
                0x00336708 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00336710; continue;
                }
                0x00336710 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00336718);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335960, 0x00336718)?;
                    pc = 0x00336718; continue;
                }
                0x00336718 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00336710) { return Err(crate::call::Unwound); } pc = 0x00336710; continue; }
                    pc = 0x00336724; continue;
                }
                0x00336724 => {
                    c.cycles += 1;
                    c.g[S0][0] = c.lw(at(c, S2, 0x20));
                    pc = 0x00336728; continue;
                }
                0x00336728 => {
                    c.cycles += 2;
                
                    if c.g[S0][0] == 0u64 { c.g[A0][0] = c.lw((r(c, S2))); pc = 0x00336750; continue; }
                    pc = 0x00336730; continue;
                }
                0x00336730 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x00336738; continue;
                }
                0x00336738 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00336740);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00335b90, 0x00336740)?;
                    pc = 0x00336740; continue;
                }
                0x00336740 => {
                    c.g[S0][0] = c.lw((r(c, S0)));
                    let t = c.g[S0][0] != 0u64; 
                    c.g[A1][0] = c.g[S0][0];
                    if t { if back_edge_unwinds(c, 0x00336738) { return Err(crate::call::Unwound); } pc = 0x00336738; continue; }
                    pc = 0x0033674c; continue;
                }
                0x0033674c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((r(c, S2)));
                    pc = 0x00336750; continue;
                }
                0x00336750 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x00336764; continue; }
                    pc = 0x00336758; continue;
                }
                0x00336758 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336760);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336760)?;
                    pc = 0x00336760; continue;
                }
                0x00336760 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x4));
                    pc = 0x00336764; continue;
                }
                0x00336764 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x00336778; continue; }
                    pc = 0x0033676c; continue;
                }
                0x0033676c => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336774);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336774)?;
                    pc = 0x00336774; continue;
                }
                0x00336774 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x8));
                    pc = 0x00336778; continue;
                }
                0x00336778 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x0033678c; continue; }
                    pc = 0x00336780; continue;
                }
                0x00336780 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00336788);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x00336788)?;
                    pc = 0x00336788; continue;
                }
                0x00336788 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0xc));
                    pc = 0x0033678c; continue;
                }
                0x0033678c => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x003367a0; continue; }
                    pc = 0x00336794; continue;
                }
                0x00336794 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0033679c);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x0033679c)?;
                    pc = 0x0033679c; continue;
                }
                0x0033679c => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x10));
                    pc = 0x003367a0; continue;
                }
                0x003367a0 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x003367b4; continue; }
                    pc = 0x003367a8; continue;
                }
                0x003367a8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003367b0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003367b0)?;
                    pc = 0x003367b0; continue;
                }
                0x003367b0 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x003367b4; continue;
                }
                0x003367b4 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x003367c8; continue; }
                    pc = 0x003367bc; continue;
                }
                0x003367bc => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003367c4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003367c4)?;
                    pc = 0x003367c4; continue;
                }
                0x003367c4 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x18));
                    pc = 0x003367c8; continue;
                }
                0x003367c8 => {
                    c.cycles += 2;
                
                    if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x003367dc; continue; }
                    pc = 0x003367d0; continue;
                }
                0x003367d0 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003367d8);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003367d8)?;
                    pc = 0x003367d8; continue;
                }
                0x003367d8 => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                    pc = 0x003367dc; continue;
                }
                0x003367dc => {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64; 
                    c.g[V0][0] = c.lw(at(c, SP, 0x40));
                    if t { pc = 0x0033681c; continue; }
                    pc = 0x003367e4; continue;
                }
                0x003367e4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003367ec);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x00340fa0, 0x003367ec)?;
                    pc = 0x003367ec; continue;
                }
                0x003367ec => {
                    let t = true; 
                    c.g[V0][0] = c.lw(at(c, SP, 0x40));
                    if t { pc = 0x0033681c; continue; }
                    pc = 0x003367f4; continue;
                }
                0x003367f4 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0]; 
                    c.g[V0][0] = c.lw(at(c, SP, 0x40));
                    if t { pc = 0x0033681c; continue; }
                    pc = 0x003367fc; continue;
                }
                0x003367fc => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T1][0] = c.lw(at(c, SP, 0x50));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T0][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[A1][0];
                    c.g[RA][0] = sx(0x00336818);
                    c.g[A3][0] = sx(at(c, SP, 0x20));
                    call_ptr(c, 0x0033ccf8, 0x00336818)?;
                    pc = 0x00336818; continue;
                }
                0x00336818 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(at(c, SP, 0x40));
                    pc = 0x0033681c; continue;
                }
                // Return the shared contact count and restore the native frame.
                0x0033681c => {
                    c.cycles += 12;
                    c.lq(S0, at(c, SP, 0xf0));
                    c.lq(S1, at(c, SP, 0xe0));
                    c.lq(S2, at(c, SP, 0xd0));
                    c.lq(S3, at(c, SP, 0xc0));
                    c.lq(S4, at(c, SP, 0xb0));
                    c.lq(S5, at(c, SP, 0xa0));
                    c.lq(S6, at(c, SP, 0x90));
                    c.lq(S7, at(c, SP, 0x80));
                    c.lq(FP, at(c, SP, 0x70));
                    c.g[RA][0] = c.ld(at(c, SP, 0x60));
                    let t = r(c, RA);
                    c.g[SP][0] = sx(at(c, SP, 0x100));
                    c.pc = t; return Ok(());
                }
                _ => { c.jump_out(pc, 0x00335d78, 0x0033684c); return Ok(()); }
            }
        }
    });
}
