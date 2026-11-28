//! Staging module oc1476 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1476`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_e840,
    wof_add_particle_data,
    "WorldObjFactory_AddParticleData",
)];

/// `WorldObjFactory_AddParticleData` (0x002fe840): adds one particle object of a
/// type from the entry table: copies the static record, fills the slots, looks up
/// the type through the index chain, probes the source and the viewer distances,
/// and hands the object to the modifier list / particle constructor or deletes it.
/// The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn wof_add_particle_data(c: &mut Cpu) {
    if c.pc != 0x002fe840 {
        return ee::s10::f_002fe840__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x002fe840;
    loop {
        match pc {
            0x002fe840 => {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2538));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S0][0] = 0x500000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S1][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S5][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                let t = c.g[V0][0] != 0u64; 
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                if t { pc = 0x002fe9a0; continue; }
                pc = 0x002fe880; continue;
            }
            0x002fe880 => {
                c.cycles += 72;
                c.g[AT][0] = 0xffffffffbf800000;
                c.mtc1(1, 1);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x47a8));
                c.g[AT][0] = 0x40800000;
                c.mtc1(1, 2);
                c.g[A0][0] = 0x1;
                c.mtc1(0, 0);
                c.g[V1][0] = 0xffffffffffff0000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 3);
                c.g[A2][0] = 0x10;
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.sw((c.g[S0][0] as u32).wrapping_sub(0x47a8), c.g[A0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x24));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x28));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x40));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x48));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x54));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x58));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x60));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x68));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x6c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x70));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x74));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x78));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x7c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x80));
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 1);
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xdc));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xb4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A2][0]);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xcc));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xd4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2538), c.g[A0][0]);
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x88));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8c));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x90));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x94));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x98));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x9c));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xa0));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xa4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xa8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xac));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb0));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xbc));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc0));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[A0][0]);
                c.sb((c.g[SP][0] as u32), c.g[A1][0]);
                pc = 0x002fe9a0; continue;
            }
            0x002fe9a0 => {
                c.cycles += 17;
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = 0xe0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68c0));
                c.g[A2][0] = 0x20000000;
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x002fe9bc);
                c.g[S6][0] = 0x4d0000;
                Callee { addr: 0x00317d70, f: ee::s10::EA_MemAllocNamed }.call(c, 0x002fe9bc)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[A2][0] = 0xe0;
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x47a8));
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x002fe9dc);
                c.sw((c.g[S3][0] as u32).wrapping_add(0xd8), c.g[V0][0]);
                call_ptr(c, 0x003e6574, 0x002fe9dc)?;
                let t = (c.g[S1][0] as i64) <= 0; 
                c.g[A2][0] = c.g[S2][0];
                if t { pc = 0x002fea40; continue; }
                pc = 0x002fe9e4; continue;
            }
            0x002fe9e4 => {
                c.cycles += 5;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62a0));
                c.g[A3][0] = 0x2;
                pc = 0x002fe9f8; continue;
            }
            0x002fe9f8 => {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                
                if c.g[V1][0] == c.g[V0][0] { c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4)); pc = 0x002fea30; continue; }
                pc = 0x002fea18; continue;
            }
            0x002fea18 => {
                c.cycles += 2;
                
                if c.g[V0][0] != c.g[A3][0] { c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4)); pc = 0x002fea30; continue; }
                pc = 0x002fea20; continue;
            }
            0x002fea20 => {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                let t = true; 
                c.swc1(0, (c.g[A0][0] as u32));
                if t { pc = 0x002fea34; continue; }
                pc = 0x002fea30; continue;
            }
            0x002fea30 => {
                c.cycles += 1;
                c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                pc = 0x002fea34; continue;
            }
            0x002fea34 => {
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let t = c.g[A1][0] != 0u64; 
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if t { if back_edge_unwinds(c, 0x002fe9f8) { return Ok(()); } pc = 0x002fe9f8; continue; }
                pc = 0x002fea40; continue;
            }
            0x002fea40 => {
                c.cycles += 6;
                c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x1c));
                c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x14));
                c.c_lt_s(1, 0);
                
                if c.fcc() { c.swc1(1, (c.g[S3][0] as u32).wrapping_add(0x1c)); pc = 0x002fea58; continue; }
                pc = 0x002fea58; continue;
            }
            0x002fea58 => {
                c.cycles += 4;
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd8));
                c.g[V0][0] = !(0u64 | c.g[A2][0]);
                let t = c.g[V0][0] != 0u64; 
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                if t { pc = 0x002fea74; continue; }
                pc = 0x002fea68; continue;
            }
            0x002fea68 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                let t = true; 
                c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x290));
                if t { pc = 0x002feac4; continue; }
                pc = 0x002fea74; continue;
            }
            0x002fea74 => {
                c.cycles += 8;
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                
                if c.g[V1][0] != 0u64 { c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c)); pc = 0x002fea9c; continue; }
                pc = 0x002fea94; continue;
            }
            0x002fea94 => {
                c.cycles += 2;
                let t = true; 
                c.g[A2][0] = 0u64;
                if t { pc = 0x002feac0; continue; }
                pc = 0x002fea9c; continue;
            }
            0x002fea9c => {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                if t { pc = 0x002feabc; continue; }
                pc = 0x002feab8; continue;
            }
            0x002feab8 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x002feabc; continue;
            }
            0x002feabc => {
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
                pc = 0x002feac0; continue;
            }
            0x002feac0 => {
                c.cycles += 1;
                c.g[S4][0] = c.g[A2][0];
                pc = 0x002feac4; continue;
            }
            0x002feac4 => {
                c.cycles += 3;
                c.g[S2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                
                if c.g[S2][0] == 0u64 { c.lq(2, (c.g[S4][0] as u32).wrapping_add(0x40)); pc = 0x002fec90; continue; }
                pc = 0x002fead0; continue;
            }
            0x002fead0 => {
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002feae4);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x002feae4)?;
                let t = c.g[V0][0] == 0u64; 
                
                if t { pc = 0x002fee44; continue; }
                pc = 0x002feaec; continue;
            }
            0x002feaec => {
                c.cycles += 12;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002feb00);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x002feb00)?;
                c.lq(3, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                c.g[V0][0] = c.g[V0][0] ^ 0x2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t { pc = 0x002feb24; continue; }
                pc = 0x002feb1c; continue;
            }
            0x002feb1c => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x1;
                if t { pc = 0x002fec78; continue; }
                pc = 0x002feb24; continue;
            }
            0x002feb24 => {
                c.cycles += 6;
                c.g[S0][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                let t = (c.g[S1][0] as i64) <= 0; 
                c.g[A1][0] = c.g[V0][0];
                if t { pc = 0x002febcc; continue; }
                pc = 0x002feb3c; continue;
            }
            0x002feb3c => {
                c.cycles += 1;
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x337c));
                pc = 0x002feb40; continue;
            }
            0x002feb40 => {
                c.cycles += 32;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002feb70);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                call_ptr(c, t, 0x002feb70)?;
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(4, (c.g[V0][0] as u32));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(0, 20);
                let t = c.fcc(); 
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x002feb1c) { return Ok(()); } pc = 0x002feb1c; continue; }
                pc = 0x002febc0; continue;
            }
            0x002febc0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t { if back_edge_unwinds(c, 0x002feb40) { return Ok(()); } pc = 0x002feb40; continue; }
                pc = 0x002febcc; continue;
            }
            0x002febcc => {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                let t = (c.g[S1][0] as i64) <= 0; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x002fec74; continue; }
                pc = 0x002febe0; continue;
            }
            0x002febe0 => {
                c.cycles += 2;
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3378));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                pc = 0x002febe8; continue;
            }
            0x002febe8 => {
                c.cycles += 32;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002fec18);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                call_ptr(c, t, 0x002fec18)?;
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(4, (c.g[V0][0] as u32));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(0, 20);
                let t = c.fcc(); 
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x002feb1c) { return Ok(()); } pc = 0x002feb1c; continue; }
                pc = 0x002fec68; continue;
            }
            0x002fec68 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t { if back_edge_unwinds(c, 0x002febe8) { return Ok(()); } pc = 0x002febe8; continue; }
                pc = 0x002fec74; continue;
            }
            0x002fec74 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x002fec78; continue;
            }
            0x002fec78 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[A0][0] = c.g[S2][0];
                if t { pc = 0x002fee44; continue; }
                pc = 0x002fec80; continue;
            }
            0x002fec80 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002fec88);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x00355cb0, 0x002fec88)?;
                let t = true; 
                
                if t { pc = 0x002fee4c; continue; }
                pc = 0x002fec90; continue;
            }
            0x002fec90 => {
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                c.sq(2, (c.g[SP][0] as u32));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                c.g[V1][0] = c.g[V1][0] ^ 0x2;
                let t = c.g[V1][0] == 0u64; 
                c.g[S2][0] = c.g[SP][0];
                if t { pc = 0x002fecb0; continue; }
                pc = 0x002feca8; continue;
            }
            0x002feca8 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x1;
                if t { pc = 0x002fee08; continue; }
                pc = 0x002fecb0; continue;
            }
            0x002fecb0 => {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[S0][0] = 0u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                let t = (c.g[S1][0] as i64) <= 0; 
                c.g[A1][0] = c.g[V0][0];
                if t { pc = 0x002fed5c; continue; }
                pc = 0x002feccc; continue;
            }
            0x002feccc => {
                c.cycles += 1;
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3374));
                pc = 0x002fecd0; continue;
            }
            0x002fecd0 => {
                c.cycles += 32;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002fed00);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                call_ptr(c, t, 0x002fed00)?;
                c.lqc2(3, (c.g[S2][0] as u32));
                c.lqc2(4, (c.g[V0][0] as u32));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(0, 20);
                let t = c.fcc(); 
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x002feca8) { return Ok(()); } pc = 0x002feca8; continue; }
                pc = 0x002fed50; continue;
            }
            0x002fed50 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t { if back_edge_unwinds(c, 0x002fecd0) { return Ok(()); } pc = 0x002fecd0; continue; }
                pc = 0x002fed5c; continue;
            }
            0x002fed5c => {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                let t = (c.g[S1][0] as i64) <= 0; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x002fee04; continue; }
                pc = 0x002fed70; continue;
            }
            0x002fed70 => {
                c.cycles += 2;
                c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x3370));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                pc = 0x002fed78; continue;
            }
            0x002fed78 => {
                c.cycles += 32;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002feda8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                call_ptr(c, t, 0x002feda8)?;
                c.lqc2(3, (c.g[S2][0] as u32));
                c.lqc2(4, (c.g[V0][0] as u32));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(0, 20);
                let t = c.fcc(); 
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x002feca8) { return Ok(()); } pc = 0x002feca8; continue; }
                pc = 0x002fedf8; continue;
            }
            0x002fedf8 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t { if back_edge_unwinds(c, 0x002fed78) { return Ok(()); } pc = 0x002fed78; continue; }
                pc = 0x002fee04; continue;
            }
            0x002fee04 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x002fee08; continue;
            }
            0x002fee08 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = 0x490000;
                if t { pc = 0x002fee44; continue; }
                pc = 0x002fee10; continue;
            }
            0x002fee10 => {
                c.cycles += 13;
                c.g[A0][0] = 0x2c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x68d0));
                c.g[A2][0] = 0x20000000;
                c.g[RA][0] = sx(0x002fee24);
                c.g[A3][0] = 0u64;
                Callee { addr: 0x00317d70, f: ee::s10::EA_MemAllocNamed }.call(c, 0x002fee24)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x002fee3c);
                c.g[T0][0] = 0u64;
                call_ptr(c, 0x003578a8, 0x002fee3c)?;
                let t = true; 
                
                if t { pc = 0x002fee4c; continue; }
                pc = 0x002fee44; continue;
            }
            0x002fee44 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002fee4c);
                c.g[A0][0] = c.g[S3][0];
                Callee { addr: 0x00317e50, f: ee::s10::__builtin_delete }.call(c, 0x002fee4c)?;
                pc = 0x002fee4c; continue;
            }
            0x002fee4c => {
                c.cycles += 19;
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6f68));
                c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[A1][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(2, (c.g[S5][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[S5][0] as u32));
                c.sw((c.g[S5][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.g[V0][0] = c.g[S5][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x002fe840, 0x002fee98); return Ok(()); }
        }
    }
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, check_reaching};
    use crate::difftest::{check, elf_image, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    pub(crate) const OC1476_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const OC1476_MAGIC: u32 = 0x1476_f00d;

    /// Fixture objects, clear of the allocator pools, the arena and the call log.
    pub(crate) const OC1476_BASE: u32 = DATA + 0x9_0000;

    /// The static particle-type record the function copies into its new object.
    pub(crate) const OC1476_STATIC: u32 = 0x004b_8858;

    macro_rules! oc1476_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1476_CTRL) != OC1476_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1476_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1476_fakes! {
        oc1476_fake_add_modifier, 0x0035_5cb0, ee::s11::OVERRIDE_00355CB0, ee::s11::f_00355cb0__generated;
        oc1476_fake_particle_ctor, 0x0035_78a8, ee::s11::OVERRIDE_003578A8, ee::s11::f_003578a8__generated;
    }

    /// A fixture for `WorldObjFactory_AddParticleData`: the static-init flag, the
    /// type lookup (direct or through the index chain, or an empty entry), the
    /// object with or without a source, the two virtual probes (the first one
    /// may answer 0), the viewer-distance loops over the world's two object
    /// lists, and the entry table. The seed's hash picks each choice.
    pub(crate) fn oc1476_setup(c: &mut Cpu, seed: u32) -> crate::difftest::Rng {
        oc1476_hook();
        let mut r = fakes::machine_rt(c, seed, 0x1476, 8);
        elf_image(c);
        c.bus.write32(OC1476_CTRL, OC1476_MAGIC);
        let h = fakes::mix(seed as u64 ^ 0x1476_0000);
        let bit = |n: u32| (h >> n) & 1 != 0;
        let gp = GAME_GP;
        let b = OC1476_BASE;
        c.bus.write32(gp + 0x2538, bit(0) as u32);
        // Type lookup.
        let obj4 = b + 0x1000;
        let s2obj = b + 0x2000;
        let vt = b + 0x3000;
        let (p16, root, tab, ent, subt, g) = (
            b + 0x6000,
            b + 0x6100,
            b + 0x6200,
            b + 0x6300,
            b + 0x6400,
            b + 0x6500,
        );
        let a2val: u32 = if bit(1) { 0xffff_ffff } else { 0x201 };
        c.bus.write32(OC1476_STATIC + 0xd8, a2val);
        c.bus.write32(gp + 0x16c8, p16);
        c.bus.write32(p16, root);
        c.bus.write32(root + 8, tab);
        c.bus.write32(tab + 4, if bit(2) { 0 } else { ent });
        c.bus.write32(ent + 0x1c, subt);
        c.bus.write32(subt + 8, (obj4 >> 2) << 8);
        c.bus.write32(gp + 0xce8, g);
        c.bus.write32(g + 0x290, obj4);
        // The type object and its source.
        c.bus.write32(obj4 + 0xc, if bit(3) { 0 } else { s2obj });
        for i in 0..4 {
            let q = r.f32(50.0);
            c.bus.write32(obj4 + 0x40 + 4 * i, q);
        }
        c.bus.write32(s2obj + 0xc, vt);
        c.bus.write16(vt + 0x80, 0);
        c.bus.write32(vt + 0x84, if bit(4) { 0x0014_06c0 } else { fakes::FAKE + 0x10 });
        c.bus.write16(vt + 0xc0, 0);
        c.bus.write32(vt + 0xc4, fakes::FAKE + 0x20);
        // The renderer flag object and the world's object lists.
        let o = b + 0x4000;
        c.bus.write32(gp.wrapping_sub(0x84c), o);
        c.bus.write32(o + 0x550, if bit(5) { 2 } else { 0 });
        let world = b + 0x5000;
        let (wa, wb, fobj, xobj) = (b + 0x5100, b + 0x5200, b + 0x5800, b + 0x5900);
        c.bus.write32(gp.wrapping_sub(0x848), world);
        c.bus.write32(world + 0x84, wa);
        c.bus.write32(wa + 0xc, wb);
        c.bus.write32(wb + 0x7c, ((h >> 8) % 4) as u32);
        c.bus.write32(wb + 0x88, ((h >> 10) % 4) as u32);
        // Three entries per list at most: slot 3 of the second list (wb+0x7c) is the first count.
        for i in 0..3u32 {
            c.bus.write32(wb + 0x40 + 4 * i, b + 0x5400 + 0x40 * i);
            c.bus.write32(wb + 0x70 + 4 * i, b + 0x5600 + 0x40 * i);
            c.bus.write32(b + 0x5400 + 0x40 * i + 0x18, fobj);
            c.bus.write32(b + 0x5600 + 0x40 * i + 0x18, fobj);
        }
        c.bus.write32(fobj + 0x6c0, xobj);
        c.bus.write16(xobj + 0x28, 0);
        c.bus.write32(xobj + 0x2c, fakes::FAKE + 0x30);
        // The distance thresholds the loops load into f20 (a pool constant per
        // loop): huge ends the loop at once, negative lets it run through.
        let lim = |on: bool| if on { 1.0e30f32.to_bits() } else { (-1.0f32).to_bits() };
        for (off, on) in [(0x337c, bit(6)), (0x3378, bit(7)), (0x3374, bit(6)), (0x3370, bit(7))] {
            c.bus.write32(gp.wrapping_sub(off), lim(on));
        }
        // The entry table (16-byte entries: slot, value, -, expected table word).
        let count = 1 + ((h >> 12) % 3) as u32;
        let entries = b + 0x7100;
        let tbl = 0x0044_62a0u32;
        let mut idx = [0u32; 3];
        for slot in idx.iter_mut().take(count as usize) {
            *slot = r.next_u32() % 8;
            let tv = [1u32, 2, 5][(r.next_u32() % 3) as usize];
            c.bus.write32(tbl + 4 * *slot, tv);
        }
        for i in 0..count {
            let e = entries + 16 * i;
            let tv = c.bus.read32(tbl + 4 * idx[i as usize]);
            c.bus.write32(e, idx[i as usize]);
            let v = r.next_u32();
            c.bus.write32(e + 4, v);
            c.bus.write32(e + 0xc, if r.next_u32() % 2 == 0 { tv } else { tv + 1 });
        }
        c.g[A0][0] = (b + 0x7000) as u64;
        c.g[A1][0] = count as u64;
        c.g[A2][0] = entries as u64;
        c.pc = 0x002f_e840;
        r
    }

    /// Static init, type lookup (direct, chained, empty), both loop pairs with
    /// early exits, the "no source" static path, and the delete / hand-off exits.
    #[test]
    fn wof_add_particle_data_matches() {
        check_reaching(
            ee::s10::f_002fe840__generated,
            wof_add_particle_data,
            240,
            &[
                0x002f_eae4,
                0x002f_eb00,
                0x002f_eb70,
                0x002f_ec18,
                0x002f_ec88,
                0x002f_ed00,
                0x002f_eda8,
                0x002f_ee3c,
            ],
            &|c, seed| {
                oc1476_setup(c, seed);
            },
        );
    }

    /// The seeds take many distinct paths.
    #[test]
    fn wof_add_particle_data_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..64 {
            let mut c = crate::difftest::fresh();
            oc1476_setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s10::f_002fe840__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 8, "only {} distinct paths", seen.len());
    }

    /// Events delivered or unwinding at the loop back edges.
    #[test]
    fn wof_add_particle_data_events() {
        check_events(
            ee::s10::f_002fe840__generated,
            wof_add_particle_data,
            120,
            &|c, seed| {
                let mut r = oc1476_setup(c, seed);
                schedule(c, &mut r, seed, 0x002f_e840)
            },
        );
    }
}
