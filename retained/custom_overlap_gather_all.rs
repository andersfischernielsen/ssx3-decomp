//! Native OverlapList_GatherAll0x332db8, retained uninstalled.
//!
//! No tests, builds, replays or game runs were added or executed.
//! Generated EE is authoritative for the unrolled octants, delay-slot loads,
//! scratch registers and block cycle charges. The C++ reference supplies
//! the native list/box vocabulary only.

use crate::call::{call_ptr, guest};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 { r(c, register).wrapping_add(offset) }

/// `void OverlapList_GatherAll(OverlapList* self, const Box* box)`.
/// Rebuild three independent native lists: patch count+8/items+c (64),
/// world instances count+10c/items+110 (64), rails/general primitives
/// count+210/items+214 (512). The stack contexts carry those nominal
/// capacities, but this function's own append paths do not truncate.
///
/// All eight spatial octants are visited. SpatialCell_Classify328360
/// returns0 for a contained cell (collect its own lists and whole child
/// subtrees),2 for a partial overlap (Cell_GatherAllRecursive33b748), or
/// another code to skip. Primitive list+28 is copied without checking
/// type, surface or rail-enabled flags; nearest-rail filtering happens later.
/// The grid mutation stamp at+4 is updated, but no cache early return exists.
pub fn overlap_list_gather_all(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            // Native list/context setup; every count is reset.
            0x00332db8 => {
                c.cycles += 46;
                c.g[SP][0] = sx(r(c, SP).wrapping_sub(0xf0));
                c.g[A3][0] = 0x40;
                c.sq(16, at(c, SP, 0xe0));
                c.g[V1][0] = sx(at(c, A0, 0xc));
                c.sq(18, at(c, SP, 0xc0));
                c.g[A2][0] = sx(at(c, A0, 0x110));
                c.sq(20, at(c, SP, 0xa0));
                c.g[S2][0] = sx(at(c, SP, 0x10));
                c.sq(21, at(c, SP, 0x90));
                c.g[T0][0] = sx(at(c, A0, 0x214));
                c.sq(17, at(c, SP, 0xd0));
                c.g[S5][0] = c.g[A1][0];
                c.sq(19, at(c, SP, 0xb0));
                c.g[A1][0] = sx(at(c, A0, 0x8));
                c.sq(22, at(c, SP, 0x80));
                c.g[S4][0] = sx(at(c, SP, 0x20));
                c.sq(23, at(c, SP, 0x70));
                c.sq(30, at(c, SP, 0x60));
                c.sd(at(c, SP, 0x50), c.g[RA][0]);
                c.g[V0][0] = c.lw(r(c, A0));
                c.g[T1][0] = c.lw(at(c, V0, 0xa0));
                c.g[V0][0] = sx(at(c, A0, 0x10c));
                c.sw(r(c, SP), c.g[V1][0]);
                c.sw(at(c, SP, 0x4), c.g[A1][0]);
                c.g[V1][0] = 0x200;
                c.sw(at(c, SP, 0x10), c.g[A2][0]);
                c.g[A1][0] = sx(at(c, A0, 0x210));
                c.sw(at(c, SP, 0x14), c.g[V0][0]);
                c.sw(at(c, SP, 0x18), c.g[A3][0]);
                c.sw(at(c, SP, 0x8), c.g[A3][0]);
                c.sw(at(c, SP, 0xc), c.g[S5][0]);
                c.sw(at(c, SP, 0x1c), c.g[S5][0]);
                c.sw(at(c, A0, 0x4), c.g[T1][0]);
                c.g[V0][0] = c.lw(at(c, S2, 0x4));
                c.sw(at(c, A0, 0x8), 0u64);
                c.sw(r(c, V0), 0u64);
                c.sw(at(c, SP, 0x20), c.g[T0][0]);
                c.sw(at(c, SP, 0x24), c.g[A1][0]);
                c.sw(at(c, SP, 0x28), c.g[V1][0]);
                c.sw(at(c, SP, 0x2c), c.g[S5][0]);
                c.g[V0][0] = c.lw(at(c, S4, 0x4));
                c.sw(r(c, V0), 0u64);
                c.g[S6][0] = c.lw(r(c, A0));
                c.g[S1][0] = c.lw(at(c, S6, 0x10));
                let t = c.g[S1][0] == 0u64; 
                c.g[S0][0] = c.g[SP][0];
                if t { pc = 0x00333064; continue; }
                pc = 0x00332e70; continue;
            }
            0x00332e70 => {
                c.cycles += 7;
                c.g[S3][0] = sx(at(c, SP, 0x30));
                c.g[A0][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x00332e80);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x00328360, 0x00332e80)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00333040; continue; }
                pc = 0x00332e8c; continue;
            }
            0x00332e8c => {
                c.cycles += 6;
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x00332e98);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003309d8, 0x00332e98)?;
                c.g[A2][0] = c.lw(at(c, S1, 0x24));
                let t = c.g[A2][0] == 0u64; 
                c.g[T0][0] = c.g[S4][0];
                if t { pc = 0x00332f5c; continue; }
                pc = 0x00332ea4; continue;
            }
            0x00332ea4 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S2, 0xc));
                pc = 0x00332ea8; continue;
            }
            // First-octant world-instance AABB loop.
            0x00332ea8 => {
                c.cycles += 6;
                c.lwc1(1, at(c, A2, 0x164));
                c.lwc1(0, r(c, V0));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V1][0] = 0u64;
                if t { pc = 0x00332f28; continue; }
                pc = 0x00332ec0; continue;
            }
            0x00332ec0 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x10));
                c.lwc1(0, at(c, A2, 0x158));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00332f28; continue; }
                pc = 0x00332ed4; continue;
            }
            0x00332ed4 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x4));
                c.lwc1(0, at(c, A2, 0x168));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00332f28; continue; }
                pc = 0x00332ee8; continue;
            }
            0x00332ee8 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x14));
                c.lwc1(0, at(c, A2, 0x15c));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00332f28; continue; }
                pc = 0x00332efc; continue;
            }
            0x00332efc => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x8));
                c.lwc1(0, at(c, A2, 0x16c));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00332f28; continue; }
                pc = 0x00332f10; continue;
            }
            0x00332f10 => {
                c.cycles += 6;
                c.lwc1(1, at(c, V0, 0x18));
                c.lwc1(0, at(c, A2, 0x160));
                c.c_le_s(0, 1);
                
                if c.fcc() { c.g[V1][0] = 0x1; pc = 0x00332f28; continue; }
                pc = 0x00332f28; continue;
            }
            0x00332f28 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[A2][0] = c.lw(r(c, A2)); pc = 0x00332f54; continue; }
                pc = 0x00332f30; continue;
            }
            0x00332f30 => {
                c.cycles += 9;
                c.g[A1][0] = c.lw(at(c, S2, 0x4));
                c.g[A0][0] = c.lw(r(c, S2));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                pc = 0x00332f54; continue;
            }
            0x00332f54 => {
                c.cycles += 2;
                
                if c.g[A2][0] != 0u64 { c.g[V0][0] = c.lw(at(c, S2, 0xc)); if event_due(c) { deliver_event_here(c, 0x00332ea8); } pc = 0x00332ea8; continue; }
                pc = 0x00332f5c; continue;
            }
            0x00332f5c => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S1, 0x28));
                
                if c.g[A2][0] == 0u64 { c.g[A0][0] = c.lw(r(c, S1)); pc = 0x00332f9c; continue; }
                pc = 0x00332f68; continue;
            }
            0x00332f68 => {
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, T0, 0x4));
                pc = 0x00332f70; continue;
            }
            // First-octant rail/general-primitive linked-list append.
            0x00332f70 => {
                c.cycles += 10;
                c.g[A0][0] = c.lw(r(c, T0));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                
                if c.g[A2][0] != 0u64 { c.g[A1][0] = c.lw(at(c, T0, 0x4)); if event_due(c) { deliver_event_here(c, 0x00332f70); } pc = 0x00332f70; continue; }
                pc = 0x00332f98; continue;
            }
            0x00332f98 => {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S1));
                pc = 0x00332f9c; continue;
            }
            0x00332f9c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x4)); pc = 0x00332fb0; continue; }
                pc = 0x00332fa4; continue;
            }
            0x00332fa4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00332fac);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00332fac)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x4));
                pc = 0x00332fb0; continue;
            }
            0x00332fb0 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x8)); pc = 0x00332fc4; continue; }
                pc = 0x00332fb8; continue;
            }
            0x00332fb8 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00332fc0);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00332fc0)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x8));
                pc = 0x00332fc4; continue;
            }
            0x00332fc4 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0xc)); pc = 0x00332fd8; continue; }
                pc = 0x00332fcc; continue;
            }
            0x00332fcc => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00332fd4);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00332fd4)?;
                c.g[A0][0] = c.lw(at(c, S1, 0xc));
                pc = 0x00332fd8; continue;
            }
            0x00332fd8 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x10)); pc = 0x00332fec; continue; }
                pc = 0x00332fe0; continue;
            }
            0x00332fe0 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00332fe8);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00332fe8)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x10));
                pc = 0x00332fec; continue;
            }
            0x00332fec => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x14)); pc = 0x00333000; continue; }
                pc = 0x00332ff4; continue;
            }
            0x00332ff4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00332ffc);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00332ffc)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x14));
                pc = 0x00333000; continue;
            }
            0x00333000 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x18)); pc = 0x00333014; continue; }
                pc = 0x00333008; continue;
            }
            0x00333008 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333010);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333010)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x18));
                pc = 0x00333014; continue;
            }
            0x00333014 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x1c)); pc = 0x00333028; continue; }
                pc = 0x0033301c; continue;
            }
            0x0033301c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333024);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333024)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x1c));
                pc = 0x00333028; continue;
            }
            0x00333028 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S6, 0x24)); pc = 0x00333068; continue; }
                pc = 0x00333030; continue;
            }
            0x00333030 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00333038);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333038)?;
                let t = true; 
                c.g[V0][0] = c.lw(at(c, S6, 0x24));
                if t { pc = 0x00333068; continue; }
                pc = 0x00333040; continue;
            }
            0x00333040 => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S6, 0x24)); pc = 0x00333068; continue; }
                pc = 0x00333048; continue;
            }
            0x00333048 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S3][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x00333064);
                c.g[T1][0] = c.g[S6][0];
                call_ptr(c, 0x0033b748, 0x00333064)?;
                pc = 0x00333064; continue;
            }
            0x00333064 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S6, 0x24));
                pc = 0x00333068; continue;
            }
            // Publish pointers to the other seven octants in their native
            // saved registers/stack words; following blocks are unrolled.
            0x00333068 => {
                c.cycles += 11;
                c.g[V1][0] = sx(at(c, S6, 0x64));
                c.sw(at(c, SP, 0x38), c.g[V1][0]);
                c.g[S1][0] = sx(at(c, S6, 0x14));
                c.g[V1][0] = sx(at(c, S6, 0x78));
                c.g[S4][0] = sx(at(c, S6, 0x28));
                c.sw(at(c, SP, 0x3c), c.g[V1][0]);
                c.g[S7][0] = sx(at(c, S6, 0x3c));
                c.g[V1][0] = sx(at(c, S6, 0x8c));
                c.g[FP][0] = sx(at(c, S6, 0x50));
                let t = c.g[V0][0] == 0u64; 
                c.sw(at(c, SP, 0x40), c.g[V1][0]);
                if t { pc = 0x00333294; continue; }
                pc = 0x00333094; continue;
            }
            0x00333094 => {
                c.cycles += 8;
                c.g[S2][0] = c.lw(at(c, S1, 0x10));
                c.g[S3][0] = sx(at(c, SP, 0x31));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003330a8);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x00328360, 0x003330a8)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00333270; continue; }
                pc = 0x003330b4; continue;
            }
            0x003330b4 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003330c0);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x003309d8, 0x003330c0)?;
                c.g[A2][0] = c.lw(at(c, S2, 0x24));
                c.g[A3][0] = sx(at(c, S0, 0x10));
                let t = c.g[A2][0] == 0u64; 
                c.g[T0][0] = sx(at(c, S0, 0x20));
                if t { pc = 0x0033318c; continue; }
                pc = 0x003330d0; continue;
            }
            0x003330d0 => {
                c.cycles += 2;
                c.g[V0][0] = c.lw(at(c, A3, 0xc));
                pc = 0x003330d8; continue;
            }
            0x003330d8 => {
                c.cycles += 6;
                c.lwc1(1, at(c, A2, 0x164));
                c.lwc1(0, r(c, V0));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V1][0] = 0u64;
                if t { pc = 0x00333158; continue; }
                pc = 0x003330f0; continue;
            }
            0x003330f0 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x10));
                c.lwc1(0, at(c, A2, 0x158));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333158; continue; }
                pc = 0x00333104; continue;
            }
            0x00333104 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x4));
                c.lwc1(0, at(c, A2, 0x168));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333158; continue; }
                pc = 0x00333118; continue;
            }
            0x00333118 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x14));
                c.lwc1(0, at(c, A2, 0x15c));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333158; continue; }
                pc = 0x0033312c; continue;
            }
            0x0033312c => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x8));
                c.lwc1(0, at(c, A2, 0x16c));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333158; continue; }
                pc = 0x00333140; continue;
            }
            0x00333140 => {
                c.cycles += 6;
                c.lwc1(1, at(c, V0, 0x18));
                c.lwc1(0, at(c, A2, 0x160));
                c.c_le_s(0, 1);
                
                if c.fcc() { c.g[V1][0] = 0x1; pc = 0x00333158; continue; }
                pc = 0x00333158; continue;
            }
            0x00333158 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[A2][0] = c.lw(r(c, A2)); pc = 0x00333184; continue; }
                pc = 0x00333160; continue;
            }
            0x00333160 => {
                c.cycles += 9;
                c.g[A1][0] = c.lw(at(c, A3, 0x4));
                c.g[A0][0] = c.lw(r(c, A3));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                pc = 0x00333184; continue;
            }
            0x00333184 => {
                c.cycles += 2;
                
                if c.g[A2][0] != 0u64 { c.g[V0][0] = c.lw(at(c, A3, 0xc)); if event_due(c) { deliver_event_here(c, 0x003330d8); } pc = 0x003330d8; continue; }
                pc = 0x0033318c; continue;
            }
            0x0033318c => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S2, 0x28));
                
                if c.g[A2][0] == 0u64 { c.g[A0][0] = c.lw(r(c, S2)); pc = 0x003331cc; continue; }
                pc = 0x00333198; continue;
            }
            0x00333198 => {
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, T0, 0x4));
                pc = 0x003331a0; continue;
            }
            0x003331a0 => {
                c.cycles += 10;
                c.g[A0][0] = c.lw(r(c, T0));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                
                if c.g[A2][0] != 0u64 { c.g[A1][0] = c.lw(at(c, T0, 0x4)); if event_due(c) { deliver_event_here(c, 0x003331a0); } pc = 0x003331a0; continue; }
                pc = 0x003331c8; continue;
            }
            0x003331c8 => {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S2));
                pc = 0x003331cc; continue;
            }
            0x003331cc => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x4)); pc = 0x003331e0; continue; }
                pc = 0x003331d4; continue;
            }
            0x003331d4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003331dc);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x003331dc)?;
                c.g[A0][0] = c.lw(at(c, S2, 0x4));
                pc = 0x003331e0; continue;
            }
            0x003331e0 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x8)); pc = 0x003331f4; continue; }
                pc = 0x003331e8; continue;
            }
            0x003331e8 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003331f0);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x003331f0)?;
                c.g[A0][0] = c.lw(at(c, S2, 0x8));
                pc = 0x003331f4; continue;
            }
            0x003331f4 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0xc)); pc = 0x00333208; continue; }
                pc = 0x003331fc; continue;
            }
            0x003331fc => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333204);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333204)?;
                c.g[A0][0] = c.lw(at(c, S2, 0xc));
                pc = 0x00333208; continue;
            }
            0x00333208 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x10)); pc = 0x0033321c; continue; }
                pc = 0x00333210; continue;
            }
            0x00333210 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333218);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333218)?;
                c.g[A0][0] = c.lw(at(c, S2, 0x10));
                pc = 0x0033321c; continue;
            }
            0x0033321c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x14)); pc = 0x00333230; continue; }
                pc = 0x00333224; continue;
            }
            0x00333224 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0033322c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x0033322c)?;
                c.g[A0][0] = c.lw(at(c, S2, 0x14));
                pc = 0x00333230; continue;
            }
            0x00333230 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x18)); pc = 0x00333244; continue; }
                pc = 0x00333238; continue;
            }
            0x00333238 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333240);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333240)?;
                c.g[A0][0] = c.lw(at(c, S2, 0x18));
                pc = 0x00333244; continue;
            }
            0x00333244 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S2, 0x1c)); pc = 0x00333258; continue; }
                pc = 0x0033324c; continue;
            }
            0x0033324c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333254);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333254)?;
                c.g[A0][0] = c.lw(at(c, S2, 0x1c));
                pc = 0x00333258; continue;
            }
            0x00333258 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S6, 0x38)); pc = 0x00333298; continue; }
                pc = 0x00333260; continue;
            }
            0x00333260 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00333268);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333268)?;
                let t = true; 
                c.g[V0][0] = c.lw(at(c, S6, 0x38));
                if t { pc = 0x00333298; continue; }
                pc = 0x00333270; continue;
            }
            0x00333270 => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S6, 0x38)); pc = 0x00333298; continue; }
                pc = 0x00333278; continue;
            }
            0x00333278 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S3][0];
                c.g[T0][0] = c.g[S2][0];
                c.g[T1][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00333294);
                c.g[A3][0] = c.g[S5][0];
                call_ptr(c, 0x0033b748, 0x00333294)?;
                pc = 0x00333294; continue;
            }
            0x00333294 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S6, 0x38));
                pc = 0x00333298; continue;
            }
            0x00333298 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S2][0] = sx(at(c, SP, 0x32));
                if t { pc = 0x0033349c; continue; }
                pc = 0x003332a0; continue;
            }
            0x003332a0 => {
                c.cycles += 7;
                c.g[S1][0] = c.lw(at(c, S4, 0x10));
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x003332b0);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x00328360, 0x003332b0)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00333478; continue; }
                pc = 0x003332bc; continue;
            }
            0x003332bc => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003332c8);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003309d8, 0x003332c8)?;
                c.g[A2][0] = c.lw(at(c, S1, 0x24));
                c.g[A3][0] = sx(at(c, S0, 0x10));
                let t = c.g[A2][0] == 0u64; 
                c.g[T0][0] = sx(at(c, S0, 0x20));
                if t { pc = 0x00333394; continue; }
                pc = 0x003332d8; continue;
            }
            0x003332d8 => {
                c.cycles += 2;
                c.g[V0][0] = c.lw(at(c, A3, 0xc));
                pc = 0x003332e0; continue;
            }
            0x003332e0 => {
                c.cycles += 6;
                c.lwc1(1, at(c, A2, 0x164));
                c.lwc1(0, r(c, V0));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V1][0] = 0u64;
                if t { pc = 0x00333360; continue; }
                pc = 0x003332f8; continue;
            }
            0x003332f8 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x10));
                c.lwc1(0, at(c, A2, 0x158));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333360; continue; }
                pc = 0x0033330c; continue;
            }
            0x0033330c => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x4));
                c.lwc1(0, at(c, A2, 0x168));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333360; continue; }
                pc = 0x00333320; continue;
            }
            0x00333320 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x14));
                c.lwc1(0, at(c, A2, 0x15c));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333360; continue; }
                pc = 0x00333334; continue;
            }
            0x00333334 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x8));
                c.lwc1(0, at(c, A2, 0x16c));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333360; continue; }
                pc = 0x00333348; continue;
            }
            0x00333348 => {
                c.cycles += 6;
                c.lwc1(1, at(c, V0, 0x18));
                c.lwc1(0, at(c, A2, 0x160));
                c.c_le_s(0, 1);
                
                if c.fcc() { c.g[V1][0] = 0x1; pc = 0x00333360; continue; }
                pc = 0x00333360; continue;
            }
            0x00333360 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[A2][0] = c.lw(r(c, A2)); pc = 0x0033338c; continue; }
                pc = 0x00333368; continue;
            }
            0x00333368 => {
                c.cycles += 9;
                c.g[A1][0] = c.lw(at(c, A3, 0x4));
                c.g[A0][0] = c.lw(r(c, A3));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                pc = 0x0033338c; continue;
            }
            0x0033338c => {
                c.cycles += 2;
                
                if c.g[A2][0] != 0u64 { c.g[V0][0] = c.lw(at(c, A3, 0xc)); if event_due(c) { deliver_event_here(c, 0x003332e0); } pc = 0x003332e0; continue; }
                pc = 0x00333394; continue;
            }
            0x00333394 => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S1, 0x28));
                
                if c.g[A2][0] == 0u64 { c.g[A0][0] = c.lw(r(c, S1)); pc = 0x003333d4; continue; }
                pc = 0x003333a0; continue;
            }
            0x003333a0 => {
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, T0, 0x4));
                pc = 0x003333a8; continue;
            }
            0x003333a8 => {
                c.cycles += 10;
                c.g[A0][0] = c.lw(r(c, T0));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                
                if c.g[A2][0] != 0u64 { c.g[A1][0] = c.lw(at(c, T0, 0x4)); if event_due(c) { deliver_event_here(c, 0x003333a8); } pc = 0x003333a8; continue; }
                pc = 0x003333d0; continue;
            }
            0x003333d0 => {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S1));
                pc = 0x003333d4; continue;
            }
            0x003333d4 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x4)); pc = 0x003333e8; continue; }
                pc = 0x003333dc; continue;
            }
            0x003333dc => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003333e4);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x003333e4)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x4));
                pc = 0x003333e8; continue;
            }
            0x003333e8 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x8)); pc = 0x003333fc; continue; }
                pc = 0x003333f0; continue;
            }
            0x003333f0 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003333f8);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x003333f8)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x8));
                pc = 0x003333fc; continue;
            }
            0x003333fc => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0xc)); pc = 0x00333410; continue; }
                pc = 0x00333404; continue;
            }
            0x00333404 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0033340c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x0033340c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0xc));
                pc = 0x00333410; continue;
            }
            0x00333410 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x10)); pc = 0x00333424; continue; }
                pc = 0x00333418; continue;
            }
            0x00333418 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333420);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333420)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x10));
                pc = 0x00333424; continue;
            }
            0x00333424 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x14)); pc = 0x00333438; continue; }
                pc = 0x0033342c; continue;
            }
            0x0033342c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333434);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333434)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x14));
                pc = 0x00333438; continue;
            }
            0x00333438 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x18)); pc = 0x0033344c; continue; }
                pc = 0x00333440; continue;
            }
            0x00333440 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333448);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333448)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x18));
                pc = 0x0033344c; continue;
            }
            0x0033344c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x1c)); pc = 0x00333460; continue; }
                pc = 0x00333454; continue;
            }
            0x00333454 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0033345c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x0033345c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x1c));
                pc = 0x00333460; continue;
            }
            0x00333460 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S6, 0x4c)); pc = 0x003334a0; continue; }
                pc = 0x00333468; continue;
            }
            0x00333468 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00333470);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333470)?;
                let t = true; 
                c.g[V0][0] = c.lw(at(c, S6, 0x4c));
                if t { pc = 0x003334a0; continue; }
                pc = 0x00333478; continue;
            }
            0x00333478 => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S6, 0x4c)); pc = 0x003334a0; continue; }
                pc = 0x00333480; continue;
            }
            0x00333480 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[T1][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0033349c);
                c.g[A3][0] = c.g[S5][0];
                call_ptr(c, 0x0033b748, 0x0033349c)?;
                pc = 0x0033349c; continue;
            }
            0x0033349c => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S6, 0x4c));
                pc = 0x003334a0; continue;
            }
            0x003334a0 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S2][0] = sx(at(c, SP, 0x33));
                if t { pc = 0x003336a4; continue; }
                pc = 0x003334a8; continue;
            }
            0x003334a8 => {
                c.cycles += 7;
                c.g[S1][0] = c.lw(at(c, S7, 0x10));
                c.g[A0][0] = c.g[S7][0];
                c.g[RA][0] = sx(0x003334b8);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x00328360, 0x003334b8)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00333680; continue; }
                pc = 0x003334c4; continue;
            }
            0x003334c4 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003334d0);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003309d8, 0x003334d0)?;
                c.g[A2][0] = c.lw(at(c, S1, 0x24));
                c.g[A3][0] = sx(at(c, S0, 0x10));
                let t = c.g[A2][0] == 0u64; 
                c.g[T0][0] = sx(at(c, S0, 0x20));
                if t { pc = 0x0033359c; continue; }
                pc = 0x003334e0; continue;
            }
            0x003334e0 => {
                c.cycles += 2;
                c.g[V0][0] = c.lw(at(c, A3, 0xc));
                pc = 0x003334e8; continue;
            }
            0x003334e8 => {
                c.cycles += 6;
                c.lwc1(1, at(c, A2, 0x164));
                c.lwc1(0, r(c, V0));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V1][0] = 0u64;
                if t { pc = 0x00333568; continue; }
                pc = 0x00333500; continue;
            }
            0x00333500 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x10));
                c.lwc1(0, at(c, A2, 0x158));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333568; continue; }
                pc = 0x00333514; continue;
            }
            0x00333514 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x4));
                c.lwc1(0, at(c, A2, 0x168));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333568; continue; }
                pc = 0x00333528; continue;
            }
            0x00333528 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x14));
                c.lwc1(0, at(c, A2, 0x15c));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333568; continue; }
                pc = 0x0033353c; continue;
            }
            0x0033353c => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x8));
                c.lwc1(0, at(c, A2, 0x16c));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333568; continue; }
                pc = 0x00333550; continue;
            }
            0x00333550 => {
                c.cycles += 6;
                c.lwc1(1, at(c, V0, 0x18));
                c.lwc1(0, at(c, A2, 0x160));
                c.c_le_s(0, 1);
                
                if c.fcc() { c.g[V1][0] = 0x1; pc = 0x00333568; continue; }
                pc = 0x00333568; continue;
            }
            0x00333568 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[A2][0] = c.lw(r(c, A2)); pc = 0x00333594; continue; }
                pc = 0x00333570; continue;
            }
            0x00333570 => {
                c.cycles += 9;
                c.g[A1][0] = c.lw(at(c, A3, 0x4));
                c.g[A0][0] = c.lw(r(c, A3));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                pc = 0x00333594; continue;
            }
            0x00333594 => {
                c.cycles += 2;
                
                if c.g[A2][0] != 0u64 { c.g[V0][0] = c.lw(at(c, A3, 0xc)); if event_due(c) { deliver_event_here(c, 0x003334e8); } pc = 0x003334e8; continue; }
                pc = 0x0033359c; continue;
            }
            0x0033359c => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S1, 0x28));
                
                if c.g[A2][0] == 0u64 { c.g[A0][0] = c.lw(r(c, S1)); pc = 0x003335dc; continue; }
                pc = 0x003335a8; continue;
            }
            0x003335a8 => {
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, T0, 0x4));
                pc = 0x003335b0; continue;
            }
            0x003335b0 => {
                c.cycles += 10;
                c.g[A0][0] = c.lw(r(c, T0));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                
                if c.g[A2][0] != 0u64 { c.g[A1][0] = c.lw(at(c, T0, 0x4)); if event_due(c) { deliver_event_here(c, 0x003335b0); } pc = 0x003335b0; continue; }
                pc = 0x003335d8; continue;
            }
            0x003335d8 => {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S1));
                pc = 0x003335dc; continue;
            }
            0x003335dc => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x4)); pc = 0x003335f0; continue; }
                pc = 0x003335e4; continue;
            }
            0x003335e4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003335ec);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x003335ec)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x4));
                pc = 0x003335f0; continue;
            }
            0x003335f0 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x8)); pc = 0x00333604; continue; }
                pc = 0x003335f8; continue;
            }
            0x003335f8 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333600);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333600)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x8));
                pc = 0x00333604; continue;
            }
            0x00333604 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0xc)); pc = 0x00333618; continue; }
                pc = 0x0033360c; continue;
            }
            0x0033360c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333614);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333614)?;
                c.g[A0][0] = c.lw(at(c, S1, 0xc));
                pc = 0x00333618; continue;
            }
            0x00333618 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x10)); pc = 0x0033362c; continue; }
                pc = 0x00333620; continue;
            }
            0x00333620 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333628);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333628)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x10));
                pc = 0x0033362c; continue;
            }
            0x0033362c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x14)); pc = 0x00333640; continue; }
                pc = 0x00333634; continue;
            }
            0x00333634 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0033363c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x0033363c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x14));
                pc = 0x00333640; continue;
            }
            0x00333640 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x18)); pc = 0x00333654; continue; }
                pc = 0x00333648; continue;
            }
            0x00333648 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333650);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333650)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x18));
                pc = 0x00333654; continue;
            }
            0x00333654 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x1c)); pc = 0x00333668; continue; }
                pc = 0x0033365c; continue;
            }
            0x0033365c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333664);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333664)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x1c));
                pc = 0x00333668; continue;
            }
            0x00333668 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S6, 0x60)); pc = 0x003336a8; continue; }
                pc = 0x00333670; continue;
            }
            0x00333670 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00333678);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333678)?;
                let t = true; 
                c.g[V0][0] = c.lw(at(c, S6, 0x60));
                if t { pc = 0x003336a8; continue; }
                pc = 0x00333680; continue;
            }
            0x00333680 => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S6, 0x60)); pc = 0x003336a8; continue; }
                pc = 0x00333688; continue;
            }
            0x00333688 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[T1][0] = c.g[S7][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003336a4);
                c.g[A3][0] = c.g[S5][0];
                call_ptr(c, 0x0033b748, 0x003336a4)?;
                pc = 0x003336a4; continue;
            }
            0x003336a4 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S6, 0x60));
                pc = 0x003336a8; continue;
            }
            0x003336a8 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S2][0] = sx(at(c, SP, 0x34));
                if t { pc = 0x003338ac; continue; }
                pc = 0x003336b0; continue;
            }
            0x003336b0 => {
                c.cycles += 7;
                c.g[S1][0] = c.lw(at(c, FP, 0x10));
                c.g[A0][0] = c.g[FP][0];
                c.g[RA][0] = sx(0x003336c0);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x00328360, 0x003336c0)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00333888; continue; }
                pc = 0x003336cc; continue;
            }
            0x003336cc => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003336d8);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003309d8, 0x003336d8)?;
                c.g[A2][0] = c.lw(at(c, S1, 0x24));
                c.g[A3][0] = sx(at(c, S0, 0x10));
                let t = c.g[A2][0] == 0u64; 
                c.g[T0][0] = sx(at(c, S0, 0x20));
                if t { pc = 0x003337a4; continue; }
                pc = 0x003336e8; continue;
            }
            0x003336e8 => {
                c.cycles += 2;
                c.g[V0][0] = c.lw(at(c, A3, 0xc));
                pc = 0x003336f0; continue;
            }
            0x003336f0 => {
                c.cycles += 6;
                c.lwc1(1, at(c, A2, 0x164));
                c.lwc1(0, r(c, V0));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V1][0] = 0u64;
                if t { pc = 0x00333770; continue; }
                pc = 0x00333708; continue;
            }
            0x00333708 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x10));
                c.lwc1(0, at(c, A2, 0x158));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333770; continue; }
                pc = 0x0033371c; continue;
            }
            0x0033371c => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x4));
                c.lwc1(0, at(c, A2, 0x168));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333770; continue; }
                pc = 0x00333730; continue;
            }
            0x00333730 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x14));
                c.lwc1(0, at(c, A2, 0x15c));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333770; continue; }
                pc = 0x00333744; continue;
            }
            0x00333744 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x8));
                c.lwc1(0, at(c, A2, 0x16c));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333770; continue; }
                pc = 0x00333758; continue;
            }
            0x00333758 => {
                c.cycles += 6;
                c.lwc1(1, at(c, V0, 0x18));
                c.lwc1(0, at(c, A2, 0x160));
                c.c_le_s(0, 1);
                
                if c.fcc() { c.g[V1][0] = 0x1; pc = 0x00333770; continue; }
                pc = 0x00333770; continue;
            }
            0x00333770 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[A2][0] = c.lw(r(c, A2)); pc = 0x0033379c; continue; }
                pc = 0x00333778; continue;
            }
            0x00333778 => {
                c.cycles += 9;
                c.g[A1][0] = c.lw(at(c, A3, 0x4));
                c.g[A0][0] = c.lw(r(c, A3));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                pc = 0x0033379c; continue;
            }
            0x0033379c => {
                c.cycles += 2;
                
                if c.g[A2][0] != 0u64 { c.g[V0][0] = c.lw(at(c, A3, 0xc)); if event_due(c) { deliver_event_here(c, 0x003336f0); } pc = 0x003336f0; continue; }
                pc = 0x003337a4; continue;
            }
            0x003337a4 => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S1, 0x28));
                
                if c.g[A2][0] == 0u64 { c.g[A0][0] = c.lw(r(c, S1)); pc = 0x003337e4; continue; }
                pc = 0x003337b0; continue;
            }
            0x003337b0 => {
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, T0, 0x4));
                pc = 0x003337b8; continue;
            }
            0x003337b8 => {
                c.cycles += 10;
                c.g[A0][0] = c.lw(r(c, T0));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                
                if c.g[A2][0] != 0u64 { c.g[A1][0] = c.lw(at(c, T0, 0x4)); if event_due(c) { deliver_event_here(c, 0x003337b8); } pc = 0x003337b8; continue; }
                pc = 0x003337e0; continue;
            }
            0x003337e0 => {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S1));
                pc = 0x003337e4; continue;
            }
            0x003337e4 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x4)); pc = 0x003337f8; continue; }
                pc = 0x003337ec; continue;
            }
            0x003337ec => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003337f4);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x003337f4)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x4));
                pc = 0x003337f8; continue;
            }
            0x003337f8 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x8)); pc = 0x0033380c; continue; }
                pc = 0x00333800; continue;
            }
            0x00333800 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333808);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333808)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x8));
                pc = 0x0033380c; continue;
            }
            0x0033380c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0xc)); pc = 0x00333820; continue; }
                pc = 0x00333814; continue;
            }
            0x00333814 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0033381c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x0033381c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0xc));
                pc = 0x00333820; continue;
            }
            0x00333820 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x10)); pc = 0x00333834; continue; }
                pc = 0x00333828; continue;
            }
            0x00333828 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333830);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333830)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x10));
                pc = 0x00333834; continue;
            }
            0x00333834 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x14)); pc = 0x00333848; continue; }
                pc = 0x0033383c; continue;
            }
            0x0033383c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333844);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333844)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x14));
                pc = 0x00333848; continue;
            }
            0x00333848 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x18)); pc = 0x0033385c; continue; }
                pc = 0x00333850; continue;
            }
            0x00333850 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333858);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333858)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x18));
                pc = 0x0033385c; continue;
            }
            0x0033385c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x1c)); pc = 0x00333870; continue; }
                pc = 0x00333864; continue;
            }
            0x00333864 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0033386c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x0033386c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x1c));
                pc = 0x00333870; continue;
            }
            0x00333870 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S6, 0x74)); pc = 0x003338b0; continue; }
                pc = 0x00333878; continue;
            }
            0x00333878 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00333880);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333880)?;
                let t = true; 
                c.g[V0][0] = c.lw(at(c, S6, 0x74));
                if t { pc = 0x003338b0; continue; }
                pc = 0x00333888; continue;
            }
            0x00333888 => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S6, 0x74)); pc = 0x003338b0; continue; }
                pc = 0x00333890; continue;
            }
            0x00333890 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[T1][0] = c.g[FP][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003338ac);
                c.g[A3][0] = c.g[S5][0];
                call_ptr(c, 0x0033b748, 0x003338ac)?;
                pc = 0x003338ac; continue;
            }
            0x003338ac => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S6, 0x74));
                pc = 0x003338b0; continue;
            }
            0x003338b0 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = c.lw(at(c, SP, 0x38));
                if t { pc = 0x00333ab4; continue; }
                pc = 0x003338b8; continue;
            }
            0x003338b8 => {
                c.cycles += 8;
                c.g[S2][0] = sx(at(c, SP, 0x35));
                c.g[A1][0] = c.g[S5][0];
                c.g[S1][0] = c.lw(at(c, V0, 0x10));
                c.g[RA][0] = sx(0x003338cc);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00328360, 0x003338cc)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00333a90; continue; }
                pc = 0x003338d8; continue;
            }
            0x003338d8 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003338e4);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003309d8, 0x003338e4)?;
                c.g[A2][0] = c.lw(at(c, S1, 0x24));
                c.g[A3][0] = sx(at(c, S0, 0x10));
                let t = c.g[A2][0] == 0u64; 
                c.g[T0][0] = sx(at(c, S0, 0x20));
                if t { pc = 0x003339ac; continue; }
                pc = 0x003338f4; continue;
            }
            0x003338f4 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, A3, 0xc));
                pc = 0x003338f8; continue;
            }
            0x003338f8 => {
                c.cycles += 6;
                c.lwc1(1, at(c, A2, 0x164));
                c.lwc1(0, r(c, V0));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V1][0] = 0u64;
                if t { pc = 0x00333978; continue; }
                pc = 0x00333910; continue;
            }
            0x00333910 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x10));
                c.lwc1(0, at(c, A2, 0x158));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333978; continue; }
                pc = 0x00333924; continue;
            }
            0x00333924 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x4));
                c.lwc1(0, at(c, A2, 0x168));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333978; continue; }
                pc = 0x00333938; continue;
            }
            0x00333938 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x14));
                c.lwc1(0, at(c, A2, 0x15c));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333978; continue; }
                pc = 0x0033394c; continue;
            }
            0x0033394c => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x8));
                c.lwc1(0, at(c, A2, 0x16c));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333978; continue; }
                pc = 0x00333960; continue;
            }
            0x00333960 => {
                c.cycles += 6;
                c.lwc1(1, at(c, V0, 0x18));
                c.lwc1(0, at(c, A2, 0x160));
                c.c_le_s(0, 1);
                
                if c.fcc() { c.g[V1][0] = 0x1; pc = 0x00333978; continue; }
                pc = 0x00333978; continue;
            }
            0x00333978 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[A2][0] = c.lw(r(c, A2)); pc = 0x003339a4; continue; }
                pc = 0x00333980; continue;
            }
            0x00333980 => {
                c.cycles += 9;
                c.g[A1][0] = c.lw(at(c, A3, 0x4));
                c.g[A0][0] = c.lw(r(c, A3));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                pc = 0x003339a4; continue;
            }
            0x003339a4 => {
                c.cycles += 2;
                
                if c.g[A2][0] != 0u64 { c.g[V0][0] = c.lw(at(c, A3, 0xc)); if event_due(c) { deliver_event_here(c, 0x003338f8); } pc = 0x003338f8; continue; }
                pc = 0x003339ac; continue;
            }
            0x003339ac => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S1, 0x28));
                
                if c.g[A2][0] == 0u64 { c.g[A0][0] = c.lw(r(c, S1)); pc = 0x003339ec; continue; }
                pc = 0x003339b8; continue;
            }
            0x003339b8 => {
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, T0, 0x4));
                pc = 0x003339c0; continue;
            }
            0x003339c0 => {
                c.cycles += 10;
                c.g[A0][0] = c.lw(r(c, T0));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                
                if c.g[A2][0] != 0u64 { c.g[A1][0] = c.lw(at(c, T0, 0x4)); if event_due(c) { deliver_event_here(c, 0x003339c0); } pc = 0x003339c0; continue; }
                pc = 0x003339e8; continue;
            }
            0x003339e8 => {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S1));
                pc = 0x003339ec; continue;
            }
            0x003339ec => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x4)); pc = 0x00333a00; continue; }
                pc = 0x003339f4; continue;
            }
            0x003339f4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003339fc);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x003339fc)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x4));
                pc = 0x00333a00; continue;
            }
            0x00333a00 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x8)); pc = 0x00333a14; continue; }
                pc = 0x00333a08; continue;
            }
            0x00333a08 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333a10);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333a10)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x8));
                pc = 0x00333a14; continue;
            }
            0x00333a14 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0xc)); pc = 0x00333a28; continue; }
                pc = 0x00333a1c; continue;
            }
            0x00333a1c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333a24);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333a24)?;
                c.g[A0][0] = c.lw(at(c, S1, 0xc));
                pc = 0x00333a28; continue;
            }
            0x00333a28 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x10)); pc = 0x00333a3c; continue; }
                pc = 0x00333a30; continue;
            }
            0x00333a30 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333a38);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333a38)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x10));
                pc = 0x00333a3c; continue;
            }
            0x00333a3c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x14)); pc = 0x00333a50; continue; }
                pc = 0x00333a44; continue;
            }
            0x00333a44 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333a4c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333a4c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x14));
                pc = 0x00333a50; continue;
            }
            0x00333a50 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x18)); pc = 0x00333a64; continue; }
                pc = 0x00333a58; continue;
            }
            0x00333a58 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333a60);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333a60)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x18));
                pc = 0x00333a64; continue;
            }
            0x00333a64 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x1c)); pc = 0x00333a78; continue; }
                pc = 0x00333a6c; continue;
            }
            0x00333a6c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333a74);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333a74)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x1c));
                pc = 0x00333a78; continue;
            }
            0x00333a78 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S6, 0x88)); pc = 0x00333ab8; continue; }
                pc = 0x00333a80; continue;
            }
            0x00333a80 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00333a88);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333a88)?;
                let t = true; 
                c.g[V0][0] = c.lw(at(c, S6, 0x88));
                if t { pc = 0x00333ab8; continue; }
                pc = 0x00333a90; continue;
            }
            0x00333a90 => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S6, 0x88)); pc = 0x00333ab8; continue; }
                pc = 0x00333a98; continue;
            }
            0x00333a98 => {
                c.cycles += 7;
                c.g[T1][0] = c.lw(at(c, SP, 0x38));
                c.g[A0][0] = c.g[S2][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00333ab4);
                c.g[A3][0] = c.g[S5][0];
                call_ptr(c, 0x0033b748, 0x00333ab4)?;
                pc = 0x00333ab4; continue;
            }
            0x00333ab4 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S6, 0x88));
                pc = 0x00333ab8; continue;
            }
            0x00333ab8 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = c.lw(at(c, SP, 0x3c));
                if t { pc = 0x00333cbc; continue; }
                pc = 0x00333ac0; continue;
            }
            0x00333ac0 => {
                c.cycles += 8;
                c.g[S2][0] = sx(at(c, SP, 0x36));
                c.g[A1][0] = c.g[S5][0];
                c.g[S1][0] = c.lw(at(c, V1, 0x10));
                c.g[RA][0] = sx(0x00333ad4);
                c.g[A0][0] = c.g[V1][0];
                call_ptr(c, 0x00328360, 0x00333ad4)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00333c98; continue; }
                pc = 0x00333ae0; continue;
            }
            0x00333ae0 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00333aec);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003309d8, 0x00333aec)?;
                c.g[A2][0] = c.lw(at(c, S1, 0x24));
                c.g[A3][0] = sx(at(c, S0, 0x10));
                let t = c.g[A2][0] == 0u64; 
                c.g[T0][0] = sx(at(c, S0, 0x20));
                if t { pc = 0x00333bb4; continue; }
                pc = 0x00333afc; continue;
            }
            0x00333afc => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, A3, 0xc));
                pc = 0x00333b00; continue;
            }
            0x00333b00 => {
                c.cycles += 6;
                c.lwc1(1, at(c, A2, 0x164));
                c.lwc1(0, r(c, V0));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V1][0] = 0u64;
                if t { pc = 0x00333b80; continue; }
                pc = 0x00333b18; continue;
            }
            0x00333b18 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x10));
                c.lwc1(0, at(c, A2, 0x158));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333b80; continue; }
                pc = 0x00333b2c; continue;
            }
            0x00333b2c => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x4));
                c.lwc1(0, at(c, A2, 0x168));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333b80; continue; }
                pc = 0x00333b40; continue;
            }
            0x00333b40 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x14));
                c.lwc1(0, at(c, A2, 0x15c));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333b80; continue; }
                pc = 0x00333b54; continue;
            }
            0x00333b54 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x8));
                c.lwc1(0, at(c, A2, 0x16c));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333b80; continue; }
                pc = 0x00333b68; continue;
            }
            0x00333b68 => {
                c.cycles += 6;
                c.lwc1(1, at(c, V0, 0x18));
                c.lwc1(0, at(c, A2, 0x160));
                c.c_le_s(0, 1);
                
                if c.fcc() { c.g[V1][0] = 0x1; pc = 0x00333b80; continue; }
                pc = 0x00333b80; continue;
            }
            0x00333b80 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[A2][0] = c.lw(r(c, A2)); pc = 0x00333bac; continue; }
                pc = 0x00333b88; continue;
            }
            0x00333b88 => {
                c.cycles += 9;
                c.g[A1][0] = c.lw(at(c, A3, 0x4));
                c.g[A0][0] = c.lw(r(c, A3));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                pc = 0x00333bac; continue;
            }
            0x00333bac => {
                c.cycles += 2;
                
                if c.g[A2][0] != 0u64 { c.g[V0][0] = c.lw(at(c, A3, 0xc)); if event_due(c) { deliver_event_here(c, 0x00333b00); } pc = 0x00333b00; continue; }
                pc = 0x00333bb4; continue;
            }
            0x00333bb4 => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S1, 0x28));
                
                if c.g[A2][0] == 0u64 { c.g[A0][0] = c.lw(r(c, S1)); pc = 0x00333bf4; continue; }
                pc = 0x00333bc0; continue;
            }
            0x00333bc0 => {
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, T0, 0x4));
                pc = 0x00333bc8; continue;
            }
            0x00333bc8 => {
                c.cycles += 10;
                c.g[A0][0] = c.lw(r(c, T0));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                
                if c.g[A2][0] != 0u64 { c.g[A1][0] = c.lw(at(c, T0, 0x4)); if event_due(c) { deliver_event_here(c, 0x00333bc8); } pc = 0x00333bc8; continue; }
                pc = 0x00333bf0; continue;
            }
            0x00333bf0 => {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S1));
                pc = 0x00333bf4; continue;
            }
            0x00333bf4 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x4)); pc = 0x00333c08; continue; }
                pc = 0x00333bfc; continue;
            }
            0x00333bfc => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333c04);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333c04)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x4));
                pc = 0x00333c08; continue;
            }
            0x00333c08 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x8)); pc = 0x00333c1c; continue; }
                pc = 0x00333c10; continue;
            }
            0x00333c10 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333c18);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333c18)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x8));
                pc = 0x00333c1c; continue;
            }
            0x00333c1c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0xc)); pc = 0x00333c30; continue; }
                pc = 0x00333c24; continue;
            }
            0x00333c24 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333c2c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333c2c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0xc));
                pc = 0x00333c30; continue;
            }
            0x00333c30 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x10)); pc = 0x00333c44; continue; }
                pc = 0x00333c38; continue;
            }
            0x00333c38 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333c40);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333c40)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x10));
                pc = 0x00333c44; continue;
            }
            0x00333c44 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x14)); pc = 0x00333c58; continue; }
                pc = 0x00333c4c; continue;
            }
            0x00333c4c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333c54);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333c54)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x14));
                pc = 0x00333c58; continue;
            }
            0x00333c58 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x18)); pc = 0x00333c6c; continue; }
                pc = 0x00333c60; continue;
            }
            0x00333c60 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333c68);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333c68)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x18));
                pc = 0x00333c6c; continue;
            }
            0x00333c6c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x1c)); pc = 0x00333c80; continue; }
                pc = 0x00333c74; continue;
            }
            0x00333c74 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333c7c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333c7c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x1c));
                pc = 0x00333c80; continue;
            }
            0x00333c80 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw(at(c, S6, 0x9c)); pc = 0x00333cc0; continue; }
                pc = 0x00333c88; continue;
            }
            0x00333c88 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00333c90);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333c90)?;
                let t = true; 
                c.g[V0][0] = c.lw(at(c, S6, 0x9c));
                if t { pc = 0x00333cc0; continue; }
                pc = 0x00333c98; continue;
            }
            0x00333c98 => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = c.lw(at(c, S6, 0x9c)); pc = 0x00333cc0; continue; }
                pc = 0x00333ca0; continue;
            }
            0x00333ca0 => {
                c.cycles += 7;
                c.g[T1][0] = c.lw(at(c, SP, 0x3c));
                c.g[A0][0] = c.g[S2][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00333cbc);
                c.g[A3][0] = c.g[S5][0];
                call_ptr(c, 0x0033b748, 0x00333cbc)?;
                pc = 0x00333cbc; continue;
            }
            0x00333cbc => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, S6, 0x9c));
                pc = 0x00333cc0; continue;
            }
            0x00333cc0 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = c.lw(at(c, SP, 0x40));
                if t { pc = 0x00333ec4; continue; }
                pc = 0x00333cc8; continue;
            }
            0x00333cc8 => {
                c.cycles += 8;
                c.g[S2][0] = sx(at(c, SP, 0x37));
                c.g[A1][0] = c.g[S5][0];
                c.g[S1][0] = c.lw(at(c, V0, 0x10));
                c.g[RA][0] = sx(0x00333cdc);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00328360, 0x00333cdc)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00333ea0; continue; }
                pc = 0x00333ce8; continue;
            }
            0x00333ce8 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00333cf4);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003309d8, 0x00333cf4)?;
                c.g[A2][0] = c.lw(at(c, S1, 0x24));
                c.g[A3][0] = sx(at(c, S0, 0x10));
                let t = c.g[A2][0] == 0u64; 
                c.g[T0][0] = sx(at(c, S0, 0x20));
                if t { pc = 0x00333dbc; continue; }
                pc = 0x00333d04; continue;
            }
            0x00333d04 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw(at(c, A3, 0xc));
                pc = 0x00333d08; continue;
            }
            0x00333d08 => {
                c.cycles += 6;
                c.lwc1(1, at(c, A2, 0x164));
                c.lwc1(0, r(c, V0));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                c.g[V1][0] = 0u64;
                if t { pc = 0x00333d88; continue; }
                pc = 0x00333d20; continue;
            }
            0x00333d20 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x10));
                c.lwc1(0, at(c, A2, 0x158));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333d88; continue; }
                pc = 0x00333d34; continue;
            }
            0x00333d34 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x4));
                c.lwc1(0, at(c, A2, 0x168));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333d88; continue; }
                pc = 0x00333d48; continue;
            }
            0x00333d48 => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x14));
                c.lwc1(0, at(c, A2, 0x15c));
                c.c_le_s(0, 1);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333d88; continue; }
                pc = 0x00333d5c; continue;
            }
            0x00333d5c => {
                c.cycles += 5;
                c.lwc1(1, at(c, V0, 0x8));
                c.lwc1(0, at(c, A2, 0x16c));
                c.c_le_s(1, 0);
                let t = !c.fcc(); 
                
                if t { pc = 0x00333d88; continue; }
                pc = 0x00333d70; continue;
            }
            0x00333d70 => {
                c.cycles += 6;
                c.lwc1(1, at(c, V0, 0x18));
                c.lwc1(0, at(c, A2, 0x160));
                c.c_le_s(0, 1);
                
                if c.fcc() { c.g[V1][0] = 0x1; pc = 0x00333d88; continue; }
                pc = 0x00333d88; continue;
            }
            0x00333d88 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[A2][0] = c.lw(r(c, A2)); pc = 0x00333db4; continue; }
                pc = 0x00333d90; continue;
            }
            0x00333d90 => {
                c.cycles += 9;
                c.g[A1][0] = c.lw(at(c, A3, 0x4));
                c.g[A0][0] = c.lw(r(c, A3));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                pc = 0x00333db4; continue;
            }
            0x00333db4 => {
                c.cycles += 2;
                
                if c.g[A2][0] != 0u64 { c.g[V0][0] = c.lw(at(c, A3, 0xc)); if event_due(c) { deliver_event_here(c, 0x00333d08); } pc = 0x00333d08; continue; }
                pc = 0x00333dbc; continue;
            }
            0x00333dbc => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S1, 0x28));
                
                if c.g[A2][0] == 0u64 { c.g[A0][0] = c.lw(r(c, S1)); pc = 0x00333dfc; continue; }
                pc = 0x00333dc8; continue;
            }
            0x00333dc8 => {
                c.cycles += 2;
                c.g[A1][0] = c.lw(at(c, T0, 0x4));
                pc = 0x00333dd0; continue;
            }
            0x00333dd0 => {
                c.cycles += 10;
                c.g[A0][0] = c.lw(r(c, T0));
                c.g[V1][0] = c.lw(r(c, A1));
                c.g[V0][0] = sx(r(c, V1) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = sx(at(c, V1, 0x1));
                c.sw(r(c, V0), c.g[A2][0]);
                c.sw(r(c, A1), c.g[V1][0]);
                c.g[A2][0] = c.lw(r(c, A2));
                
                if c.g[A2][0] != 0u64 { c.g[A1][0] = c.lw(at(c, T0, 0x4)); if event_due(c) { deliver_event_here(c, 0x00333dd0); } pc = 0x00333dd0; continue; }
                pc = 0x00333df8; continue;
            }
            0x00333df8 => {
                c.cycles += 1;
                c.g[A0][0] = c.lw(r(c, S1));
                pc = 0x00333dfc; continue;
            }
            0x00333dfc => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x4)); pc = 0x00333e10; continue; }
                pc = 0x00333e04; continue;
            }
            0x00333e04 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333e0c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333e0c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x4));
                pc = 0x00333e10; continue;
            }
            0x00333e10 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x8)); pc = 0x00333e24; continue; }
                pc = 0x00333e18; continue;
            }
            0x00333e18 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333e20);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333e20)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x8));
                pc = 0x00333e24; continue;
            }
            0x00333e24 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0xc)); pc = 0x00333e38; continue; }
                pc = 0x00333e2c; continue;
            }
            0x00333e2c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333e34);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333e34)?;
                c.g[A0][0] = c.lw(at(c, S1, 0xc));
                pc = 0x00333e38; continue;
            }
            0x00333e38 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x10)); pc = 0x00333e4c; continue; }
                pc = 0x00333e40; continue;
            }
            0x00333e40 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333e48);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333e48)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x10));
                pc = 0x00333e4c; continue;
            }
            0x00333e4c => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x14)); pc = 0x00333e60; continue; }
                pc = 0x00333e54; continue;
            }
            0x00333e54 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333e5c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333e5c)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x14));
                pc = 0x00333e60; continue;
            }
            0x00333e60 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x18)); pc = 0x00333e74; continue; }
                pc = 0x00333e68; continue;
            }
            0x00333e68 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333e70);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333e70)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x18));
                pc = 0x00333e74; continue;
            }
            0x00333e74 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[A0][0] = c.lw(at(c, S1, 0x1c)); pc = 0x00333e88; continue; }
                pc = 0x00333e7c; continue;
            }
            0x00333e7c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00333e84);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333e84)?;
                c.g[A0][0] = c.lw(at(c, S1, 0x1c));
                pc = 0x00333e88; continue;
            }
            0x00333e88 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.lq(16, at(c, SP, 0xe0)); pc = 0x00333ec8; continue; }
                pc = 0x00333e90; continue;
            }
            0x00333e90 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00333e98);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00340dc0, 0x00333e98)?;
                let t = true; 
                c.lq(16, at(c, SP, 0xe0));
                if t { pc = 0x00333ec8; continue; }
                pc = 0x00333ea0; continue;
            }
            0x00333ea0 => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.lq(16, at(c, SP, 0xe0)); pc = 0x00333ec8; continue; }
                pc = 0x00333ea8; continue;
            }
            0x00333ea8 => {
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.g[T1][0] = c.lw(at(c, SP, 0x40));
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00333ec4);
                c.g[A2][0] = c.g[A1][0];
                call_ptr(c, 0x0033b748, 0x00333ec4)?;
                pc = 0x00333ec4; continue;
            }
            0x00333ec4 => {
                c.cycles += 1;
                c.lq(16, at(c, SP, 0xe0));
                pc = 0x00333ec8; continue;
            }
            0x00333ec8 => {
                c.cycles += 11;
                c.lq(17, at(c, SP, 0xd0));
                c.lq(18, at(c, SP, 0xc0));
                c.lq(19, at(c, SP, 0xb0));
                c.lq(20, at(c, SP, 0xa0));
                c.lq(21, at(c, SP, 0x90));
                c.lq(22, at(c, SP, 0x80));
                c.lq(23, at(c, SP, 0x70));
                c.lq(30, at(c, SP, 0x60));
                c.g[RA][0] = c.ld(at(c, SP, 0x50));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx(at(c, SP, 0xf0));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00332db8, 0x00333ef4); return Ok(()); }
        }
    }
    });
}
