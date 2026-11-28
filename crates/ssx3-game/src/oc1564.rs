//! Staging module oc1564 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1564`.

#![allow(unused_imports, unused_assignments, unused_variables, unused_mut, unreachable_code)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `MenuSysColorSwatchItem_Draw` (0x002d0008): hand port of the whole resumable body (every entry
/// of the generated `match pc`, so a resume after an unwound callee or event
/// continues natively). Every register, cycle and call is kept.
pub fn menu_sys_color_swatch_item_draw(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x002d0008 => {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                c.g[A3][0] = 0u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[T0][0] = 0u64;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[S0][0] = c.g[A1][0];
                pc = 0x002d0020; continue;
            }
            0x002d0020 => {
                c.cycles += 133;
                c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                c.g[S1][0] = c.g[A0][0];
                c.g[RA][0] = sx(0x002d0030);
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                Callee { addr: 0x002ca988, f: ee::s09::MenuSysItem_DrawRow }.call(c, 0x002d0030)?;
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = 0x3;
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.lwc1(3, (c.g[S0][0] as u32));
                c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.add_s(3, 3, 0);
                c.g[AT][0] = 0x41800000;
                c.mtc1(1, 1);
                c.g[AT][0] = 0x40c00000;
                c.mtc1(1, 0);
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 7);
                c.add_s(5, 5, 0);
                c.g[AT][0] = 0x40000000;
                c.mtc1(1, 2);
                c.sub_s(3, 3, 1);
                c.g[AT][0] = 0x41880000;
                c.mtc1(1, 8);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.add_s(5, 5, 2);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xc8));
                c.sub_s(3, 3, 7);
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xcc));
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 2);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
                c.add_s(8, 5, 8);
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.add_s(7, 3, 7);
                c.mtc1(0, 4);
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 6);
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[V1][0] as u32));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.mul_s(0, 0, 2);
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xd8));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xdc));
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(4, (c.g[SP][0] as u32));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.lwc1(1, (c.g[V1][0] as u32));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4c));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.lwc1(1, (c.g[V1][0] as u32));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.mul_s(1, 1, 2);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x7c));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xa4));
                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 2);
                c.cvt_w_s(1, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa8));
                c.lwc1(1, (c.g[V1][0] as u32));
                c.mul_s(1, 1, 2);
                c.cvt_w_s(0, 1);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xac));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x94));
                c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xd4));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x98));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lwc1(0, (c.g[V1][0] as u32));
                c.c_eq_s(0, 6);
                let t = c.fcc(); 
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                if t { pc = 0x002d024c; continue; }
                pc = 0x002d0234; continue;
            }
            0x002d0234 => {
                c.cycles += 6;
                c.g[A1][0] = 0xffffffffffffff83;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | 0x14;
                if t { pc = 0x002d0260; continue; }
                pc = 0x002d024c; continue;
            }
            0x002d024c => {
                c.cycles += 5;
                c.g[A1][0] = 0xffffffffffffff83;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | 0x4;
                pc = 0x002d0260; continue;
            }
            0x002d0260 => {
                c.cycles += 19;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A0][0] = 0xffffffffffffffff;
                c.g[A1][0] = 0x4;
                c.g[A2][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = 0u64;
                c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x002d0298);
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x002d0298)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x002d0008, 0x002d02ac); return Ok(()); }
        }
    }
    });
}


/// `MenuSysVectorItem_HandleInput` (0x002d06e8): hand port of the whole resumable body (every entry
/// of the generated `match pc`, so a resume after an unwound callee or event
/// continues natively). Every register, cycle and call is kept.
pub fn menu_sys_vector_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    let mut charged = false;
    loop {
        match pc {
            0x002d06e8 => {
                // The main body charges the first two blocks together (9) before
                // the call; only the `__entries` twin splits them 6 + 3.
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[RA][0] = sx(0x002d06fc);
                c.g[S0][0] = c.g[A0][0];
                Callee { addr: 0x002cfa08, f: ee::s09::MenuSysGroupItem_HandleInput }.call(c, 0x002d06fc)?;
                c.g[T1][0] = c.g[V0][0];
                charged = true;
                pc = 0x002d0700; continue;
            }
            0x002d0700 => {
                if !charged {
                    c.cycles += 3;
                }
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x670));
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b8));
                if t { pc = 0x002d07f4; continue; }
                pc = 0x002d070c; continue;
            }
            0x002d070c => {
                c.cycles += 37;
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x5dc));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5ec));
                c.g[A2][0] = c.g[V0][0];
                c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x610));
                c.lwc1(1, (c.g[A0][0] as u32));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x620));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[T0][0] = c.g[V1][0];
                c.mul_s(0, 0, 1);
                c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0x648));
                c.lwc1(2, (c.g[V0][0] as u32));
                c.g[A3][0] = c.g[A1][0];
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x5e0));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                c.mul_s(3, 3, 2);
                c.add_s(7, 0, 1);
                c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x614));
                c.lwc1(4, (c.g[V0][0] as u32));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x644));
                c.add_s(6, 3, 2);
                c.mul_s(0, 7, 7);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.mul_s(1, 1, 4);
                c.mul_s(2, 6, 6);
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.mtc1(0, 3);
                c.add_s(4, 1, 5);
                c.add_s(0, 0, 2);
                c.mul_s(1, 4, 4);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.add_s(0, 0, 1);
                c.sqrt_s(1, 0);
                c.c_eq_s(1, 3);
                
                if c.fcc() { c.swc1(7, (c.g[SP][0] as u32)); pc = 0x002d07c8; continue; }
                pc = 0x002d07a0; continue;
            }
            0x002d07a0 => {
                c.cycles += 10;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.div_s(0, 0, 1);
                c.mul_s(1, 4, 0);
                c.mul_s(2, 7, 0);
                c.mul_s(0, 6, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(2, (c.g[SP][0] as u32));
                let t = true; 
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                if t { pc = 0x002d07d0; continue; }
                pc = 0x002d07c8; continue;
            }
            0x002d07c8 => {
                c.cycles += 2;
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                pc = 0x002d07d0; continue;
            }
            0x002d07d0 => {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[V0][0] as u32));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[V1][0] as u32));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32));
                pc = 0x002d07f4; continue;
            }
            0x002d07f4 => {
                c.cycles += 5;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.g[T1][0];
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x002d06e8, 0x002d0808); return Ok(()); }
        }
    }
    });
}


/// `WScriptFn_SectionEventCommand` (0x00302210): hand port of the whole resumable body (every entry
/// of the generated `match pc`, so a resume after an unwound callee or event
/// continues natively). Every register, cycle and call is kept.
pub fn w_script_fn_section_event_command(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x00302210 => {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2640));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S2][0] = c.g[A2][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S5][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A2][0] = 0x500000;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                let t = c.g[V0][0] != 0u64; 
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                if t { pc = 0x00302270; continue; }
                pc = 0x00302248; continue;
            }
            0x00302248 => {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43e0));
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = 0x1;
                c.sw((c.g[A2][0] as u32).wrapping_sub(0x43e0), c.g[V1][0]);
                c.sb((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2640), c.g[A0][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.sb((c.g[SP][0] as u32), c.g[A1][0]);
                pc = 0x00302270; continue;
            }
            0x00302270 => {
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x43e0));
                c.g[A2][0] = 0x10;
                c.g[RA][0] = sx(0x00302284);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003e6574, 0x00302284)?;
                let t = (c.g[S1][0] as i64) <= 0; 
                c.g[A2][0] = c.g[S2][0];
                if t { pc = 0x003022e8; continue; }
                pc = 0x0030228c; continue;
            }
            0x0030228c => {
                c.cycles += 5;
                c.g[V0][0] = 0x440000;
                c.g[A1][0] = c.g[S1][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x65c8));
                c.g[A3][0] = 0x2;
                pc = 0x003022a0; continue;
            }
            0x003022a0 => {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                
                if c.g[V1][0] == c.g[V0][0] { c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4)); pc = 0x003022d8; continue; }
                pc = 0x003022c0; continue;
            }
            0x003022c0 => {
                c.cycles += 2;
                
                if c.g[V0][0] != c.g[A3][0] { c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4)); pc = 0x003022d8; continue; }
                pc = 0x003022c8; continue;
            }
            0x003022c8 => {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                let t = true; 
                c.swc1(0, (c.g[A0][0] as u32));
                if t { pc = 0x003022dc; continue; }
                pc = 0x003022d8; continue;
            }
            0x003022d8 => {
                c.cycles += 1;
                c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                pc = 0x003022dc; continue;
            }
            0x003022dc => {
                c.cycles += 3;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                let t = c.g[A1][0] != 0u64; 
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                if t { if back_edge_unwinds(c, 0x003022a0) { return Ok(()); } pc = 0x003022a0; continue; }
                pc = 0x003022e8; continue;
            }
            0x003022e8 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                let t = c.g[V0][0] == c.g[V1][0]; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t { pc = 0x00302340; continue; }
                pc = 0x003022fc; continue;
            }
            0x003022fc => {
                c.cycles += 17;
                c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                if c.g[A0][0] == 0 { c.g[V1][0] = 0u64; }
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
                if c.g[V1][0] == 0 { c.g[A0][0] = 0u64; }
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00302338);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                call_ptr(c, t, 0x00302338)?;
                let t = true; 
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                if t { pc = 0x00302344; continue; }
                pc = 0x00302340; continue;
            }
            0x00302340 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x00302344; continue;
            }
            0x00302344 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S3][0] = 0u64;
                if t { pc = 0x00302448; continue; }
                pc = 0x0030234c; continue;
            }
            0x0030234c => {
                c.cycles += 13;
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[S4][0] = c.lw((c.g[S0][0] as u32));
                c.g[RA][0] = sx(0x0030235c);
                c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                call_ptr(c, 0x0014dc80, 0x0030235c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00302368);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00302368)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = (c.g[S1][0] < 0x8u64) as u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                let t = c.g[A0][0] == 0u64; 
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                if t { pc = 0x00302394; continue; }
                pc = 0x00302380; continue;
            }
            0x00302380 => {
                c.cycles += 5;
                c.g[V0][0] = 0x440000;
                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e40));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[S3][0] = c.lw((c.g[V1][0] as u32));
                pc = 0x00302394; continue;
            }
            0x00302394 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[A1][0];
                c.g[RA][0] = sx(0x003023a0);
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x00144ce0, 0x003023a0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[S2][0] < 0x6u64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x490000;
                if t { pc = 0x00302448; continue; }
                pc = 0x003023b0; continue;
            }
            0x003023b0 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6870));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
            }
            0x003023c8 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003023d4);
                c.g[A2][0] = c.g[S3][0];
                call_ptr(c, 0x0022cea8, 0x003023d4)?;
                let t = true; 
                c.g[V1][0] = 0x4d0000;
                if t { pc = 0x0030244c; continue; }
                pc = 0x003023dc; continue;
            }
            0x003023dc => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003023e8);
                c.g[A2][0] = c.g[S3][0];
                call_ptr(c, 0x0022d088, 0x003023e8)?;
                let t = true; 
                c.g[V1][0] = 0x4d0000;
                if t { pc = 0x0030244c; continue; }
                pc = 0x003023f0; continue;
            }
            0x003023f0 => {
                c.cycles += 12;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = 0xe;
                c.g[A2][0] = 0x2;
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x00302408);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                call_ptr(c, 0x00231250, 0x00302408)?;
                c.g[RA][0] = sx(0x00302410);
                call_ptr(c, 0x0028b180, 0x00302410)?;
                c.g[RA][0] = sx(0x00302418);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002a49e8, 0x00302418)?;
                let t = true; 
                c.g[V1][0] = 0x4d0000;
                if t { pc = 0x0030244c; continue; }
                pc = 0x00302420; continue;
            }
            0x00302420 => {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = 0xe;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x00302438);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                call_ptr(c, 0x00231250, 0x00302438)?;
                let t = true; 
                c.g[V1][0] = 0x4d0000;
                if t { pc = 0x0030244c; continue; }
                pc = 0x00302440; continue;
            }
            0x00302440 => {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a4), c.g[V0][0]);
                pc = 0x00302448; continue;
            }
            0x00302448 => {
                c.cycles += 1;
                c.g[V1][0] = 0x4d0000;
                pc = 0x0030244c; continue;
            }
            0x0030244c => {
                c.cycles += 17;
                c.g[V0][0] = c.g[S5][0];
                c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
                c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(4, (c.g[A3][0] as u32));
                c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[S5][0] as u32).wrapping_add(0x7));
                c.sdr(4, (c.g[S5][0] as u32));
                c.sw((c.g[S5][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00302210, 0x00302490); return Ok(()); }
        }
    }
    });
}


/// `ssx_main` (0x0031af80): hand port of the whole resumable body (every entry
/// of the generated `match pc`, so a resume after an unwound callee or event
/// continues natively). Every register, cycle and call is kept.
pub fn ssx_main(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    // The main body charges all 34 cycles up front (the `__entries` twin
    // charges them block by block); a body entered at the start does the same.
    let mut pre = false;
    loop {
        match pc {
            0x0031af80 => {
                c.cycles += 34;
                pre = true;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x0031afa0);
                c.g[S2][0] = c.g[A1][0];
                call_ptr(c, 0x0040fc90, 0x0031afa0)?;
                pc = 0x0031afa0; continue;
            }
            0x0031afa0 => {
                if !pre {
                    c.cycles += 2;
                }
                c.g[RA][0] = sx(0x0031afa8);
                Callee { addr: 0x0031adb0, f: ee::s10::ssx_SystemInit }.call(c, 0x0031afa8)?;
                pc = 0x0031afa8; continue;
            }
            0x0031afa8 => {
                if !pre {
                    c.cycles += 7;
                }
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = 0x10000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2518));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x0031afc4);
                c.g[A0][0] = c.g[A0][0] | 0x42c0;
                Callee { addr: 0x00317d70, f: ee::s10::EA_MemAllocNamed }.call(c, 0x0031afc4)?;
                pc = 0x0031afc4; continue;
            }
            0x0031afc4 => {
                if !pre {
                    c.cycles += 3;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0031afd0);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317458, f: ee::s10::PS2ExecMan_ctor }.call(c, 0x0031afd0)?;
                pc = 0x0031afd0; continue;
            }
            0x0031afd0 => {
                if !pre {
                    c.cycles += 8;
                }
                c.g[V1][0] = 0x490000;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2498));
                c.g[A1][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0031aff0);
                c.g[A3][0] = c.g[S2][0];
                Callee { addr: 0x00316d60, f: ee::s10::App_Run_MainLoop }.call(c, 0x0031aff0)?;
                pc = 0x0031aff0; continue;
            }
            0x0031aff0 => {
                if !pre {
                    c.cycles += 6;
                }
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x0031af80, 0x0031b008); return Ok(()); }
        }
    }
    });
}



/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_0008, menu_sys_color_swatch_item_draw, "MenuSysColorSwatchItem_Draw"),
    (0x002d_06e8, menu_sys_vector_item_handle_input, "MenuSysVectorItem_HandleInput"),
    (0x0030_2210, w_script_fn_section_event_command, "WScriptFn_SectionEventCommand"),
    (0x0031_af80, ssx_main, "ssx_main"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{
        elf_image, fakes, Rng, DATA, GAME_GP, RA as TEST_RA, SP as TEST_SP,
    };
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (a unique cell of the machine's
    /// RAM): the fakes answer only when a fixture sets it.
    const OC1564_GATE: u32 = DATA + 0x20_0000;
    const OC1564_MAGIC: u32 = 0x1564_f001;

    /// A gated fake for the real callee at `$addr`: with the gate off it runs
    /// whatever the slot held before (`fakes::fall_back`), with the gate on it
    /// logs the call and returns (or unwinds) like `fakes::fake_call`.
    macro_rules! oc1564_fake {
        ($name:ident, $gen:path, $addr:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1564_GATE) != OC1564_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    oc1564_fake!(oc1564_fake_draw_row, ee::s09::f_002ca988__generated, 0x002c_a988);
    oc1564_fake!(oc1564_fake_group_input, ee::s09::f_002cfa08__generated, 0x002c_fa08);
    oc1564_fake!(oc1564_fake_mem_copy, ee::s14::f_003e6574__generated, 0x003e_6574);
    oc1564_fake!(oc1564_fake_14dc80, ee::s01::f_0014dc80__generated, 0x0014_dc80);
    oc1564_fake!(oc1564_fake_14dd58, ee::s01::f_0014dd58__generated, 0x0014_dd58);
    oc1564_fake!(oc1564_fake_144ce0, ee::s01::f_00144ce0__generated, 0x0014_4ce0);
    oc1564_fake!(oc1564_fake_22cea8, ee::s05::f_0022cea8__generated, 0x0022_cea8);
    oc1564_fake!(oc1564_fake_22d088, ee::s05::f_0022d088__generated, 0x0022_d088);
    oc1564_fake!(oc1564_fake_231250, ee::s06::f_00231250__generated, 0x0023_1250);
    oc1564_fake!(oc1564_fake_28b180, ee::s07::f_0028b180__generated, 0x0028_b180);
    oc1564_fake!(oc1564_fake_2a49e8, ee::s08::f_002a49e8__generated, 0x002a_49e8);
    oc1564_fake!(oc1564_fake_40fc90, ee::s15::f_0040fc90__generated, 0x0040_fc90);
    oc1564_fake!(oc1564_fake_31adb0, ee::s10::f_0031adb0__generated, 0x0031_adb0);
    oc1564_fake!(oc1564_fake_317d70, ee::s10::f_00317d70__generated, 0x0031_7d70);
    oc1564_fake!(oc1564_fake_317458, ee::s10::f_00317458__generated, 0x0031_7458);
    oc1564_fake!(oc1564_fake_316d60, ee::s10::f_00316d60__generated, 0x0031_6d60);

    /// Hook every fake of this module into its callee's slot (once per process).
    fn oc1564_install() {
        macro_rules! hook {
            ($slot:path, $fake:ident) => {
                fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
            };
        }
        hook!(ee::s09::OVERRIDE_002CA988, oc1564_fake_draw_row);
        hook!(ee::s09::OVERRIDE_002CFA08, oc1564_fake_group_input);
        hook!(ee::s14::OVERRIDE_003E6574, oc1564_fake_mem_copy);
        hook!(ee::s01::OVERRIDE_0014DC80, oc1564_fake_14dc80);
        hook!(ee::s01::OVERRIDE_0014DD58, oc1564_fake_14dd58);
        hook!(ee::s01::OVERRIDE_00144CE0, oc1564_fake_144ce0);
        hook!(ee::s05::OVERRIDE_0022CEA8, oc1564_fake_22cea8);
        hook!(ee::s05::OVERRIDE_0022D088, oc1564_fake_22d088);
        hook!(ee::s06::OVERRIDE_00231250, oc1564_fake_231250);
        hook!(ee::s07::OVERRIDE_0028B180, oc1564_fake_28b180);
        hook!(ee::s08::OVERRIDE_002A49E8, oc1564_fake_2a49e8);
        hook!(ee::s15::OVERRIDE_0040FC90, oc1564_fake_40fc90);
        hook!(ee::s10::OVERRIDE_0031ADB0, oc1564_fake_31adb0);
        hook!(ee::s10::OVERRIDE_00317D70, oc1564_fake_317d70);
        hook!(ee::s10::OVERRIDE_00317458, oc1564_fake_317458);
        hook!(ee::s10::OVERRIDE_00316D60, oc1564_fake_316d60);
    }

    /// `fakes::machine` with this module's fake gate switched on, and (when
    /// `image`) the game image loaded for tables the function reads.
    fn oc1564_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, image: bool) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        if image {
            elf_image(c);
        }
        c.bus.write32(OC1564_GATE, OC1564_MAGIC);
        r
    }

    /// `void MenuSysColorSwatchItem_Draw(item, rect)`: draws the item row, then
    /// a colour swatch quad (four vertices from the item's colour) through the
    /// render list's vtable call. Entries: the start, and the resume points at
    /// the vertex setup, the two alpha arms and the vtable call.
    #[test]
    fn menu_sys_color_swatch_item_draw_matches() {
        oc1564_install();
        const ENTRIES: [u32; 8] = [
            0x002d_0008,
            0x002d_0008,
            0x002d_0008,
            0x002d_0008,
            0x002d_0020,
            0x002d_0234,
            0x002d_024c,
            0x002d_0260,
        ];
        fakes::covered(
            ee::s09::f_002d0008__generated,
            menu_sys_color_swatch_item_draw,
            96,
            &|c, seed| {
                let mut r = oc1564_machine(c, seed, 0x002d_0008, 2, false);
                let (rect, item, color) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                let (pool, ent, queue) = (DATA + 0x2000, DATA + 0x5000, DATA + 0x5400);
                for i in 0..4 {
                    c.bus.write32(color + 4 * i, r.f32(1.0));
                    c.bus.write32(rect + 4 * i, r.f32(100.0));
                }
                if seed % 2 == 0 {
                    // alpha (word 0) exactly 1.0: the opaque arm
                    c.bus.write32(color, 0x3f80_0000);
                }
                c.bus.write32(item + 0x14, color);
                c.bus.write32(item + 0x18, r.next_u32());
                c.bus.write32(GAME_GP.wrapping_sub(0x854), pool);
                c.bus.write32(pool + 0xe84, ent);
                c.bus.write32(ent + 4, r.next_u32());
                c.bus.write32(pool + 0x10d8, queue);
                c.bus.write16(queue + 0x238, (r.next_u32() % 8) as u16);
                c.bus.write32(queue + 0x23c, fakes::FAKE);
                c.g[A0][0] = item as u64;
                c.g[A1][0] = rect as u64;
                c.g[S0][0] = rect as u64;
                c.g[S1][0] = item as u64;
                let entry = ENTRIES[(seed % 8) as usize];
                match entry {
                    0x002d_0234 | 0x002d_024c => c.g[V1][0] = pool as u64,
                    0x002d_0260 => {
                        c.g[A0][0] = ent as u64;
                        c.g[V0][0] = r.next_u32() as u64;
                    }
                    _ => {}
                }
                if entry != 0x002d_0008 {
                    c.bus.write64(TEST_SP + 0xe0, TEST_RA as u64);
                }
                c.pc = entry;
            },
        );
    }

    /// `u32 MenuSysVectorItem_HandleInput(item)`: runs the group handler, then,
    /// when the vector item is active (+0x670), normalises the three edited
    /// components and writes them back through their value pointers. Entries:
    /// the start and every resume point of the body.
    #[test]
    fn menu_sys_vector_item_handle_input_matches() {
        oc1564_install();
        const ENTRIES: [u32; 10] = [
            0x002d_06e8,
            0x002d_06e8,
            0x002d_06e8,
            0x002d_06e8,
            0x002d_0700,
            0x002d_070c,
            0x002d_07a0,
            0x002d_07c8,
            0x002d_07d0,
            0x002d_07f4,
        ];
        fakes::covered(
            ee::s09::f_002d06e8__generated,
            menu_sys_vector_item_handle_input,
            100,
            &|c, seed| {
                let mut r = oc1564_machine(c, seed, 0x002d_06e8, 1, false);
                let obj = DATA + 0x1000;
                let zero = seed % 5 == 1;
                c.bus.write32(obj + 0x670, if seed % 4 == 0 { 0 } else { 1 });
                for off in [0x5dc, 0x5e0, 0x610, 0x614, 0x644, 0x648] {
                    c.bus.write32(obj + off, if zero { 0 } else { r.f32(4.0) });
                }
                // three value records: +0x18 points at the float they edit
                for (i, rec) in [0x5b8u32, 0x5ec, 0x620].into_iter().enumerate() {
                    let p = DATA + 0x3000 + 0x10 * i as u32;
                    c.bus.write32(obj + rec + 0x18, p);
                    c.bus.write32(p, if zero { 0 } else { r.f32(4.0) });
                }
                c.g[A0][0] = obj as u64;
                c.g[S0][0] = obj as u64;
                c.g[A2][0] = (obj + 0x5b8) as u64;
                c.g[T0][0] = (obj + 0x5ec) as u64;
                c.g[A3][0] = (obj + 0x620) as u64;
                c.g[V0][0] = (obj + 0x5b8) as u64;
                for i in 0..3 {
                    c.bus.write32(TEST_SP + 4 * i, r.f32(4.0));
                }
                let entry = ENTRIES[(seed % 10) as usize];
                if entry != 0x002d_06e8 {
                    c.bus.write64(TEST_SP + 0x20, TEST_RA as u64);
                }
                c.pc = entry;
            },
        );
    }

    const WS_PARAMS: u32 = DATA + 0x3000;
    const WS_OUT: u32 = DATA + 0x6000;
    const WS_UI: u32 = DATA + 0x1300;

    /// `Luno WScriptFn_SectionEventCommand(out, n, params)`: copies the
    /// command's argument block (`n` index/value pairs applied over the
    /// defaults), and for a human rider's being runs the section command
    /// (load, activate, DJ/user select, sky swap). Entries: the start (with
    /// and without the first-time init, with and without parameters) and every
    /// resume point of the body, including the jump-table arms.
    #[test]
    fn w_script_fn_section_event_command_matches() {
        oc1564_install();
        const ENTRIES: [u32; 20] = [
            0x0030_2210,
            0x0030_2210,
            0x0030_2210,
            0x0030_2210,
            0x0030_2210,
            0x0030_2270,
            0x0030_228c,
            0x0030_22a0,
            0x0030_22dc,
            0x0030_22e8,
            0x0030_234c,
            0x0030_2394,
            0x0030_23b0,
            0x0030_23c8,
            0x0030_23dc,
            0x0030_23f0,
            0x0030_2420,
            0x0030_2440,
            0x0030_2448,
            0x0030_244c,
        ];
        fakes::covered(
            ee::s10::f_00302210__generated,
            w_script_fn_section_event_command,
            160,
            &|c, seed| {
                let mut r = oc1564_machine(c, seed, 0x0030_2210, 8, true);
                let entry = ENTRIES[(seed % 20) as usize];
                let start = entry == 0x0030_2210;
                let n: u32 = match seed / 20 % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    _ => (-1i32) as u32,
                };
                let frame = if start { TEST_SP - 0x90 } else { TEST_SP };
                let vp = frame + 0x10;
                // argument block defaults the (faked) copy leaves in place
                c.bus.write32(vp, r.next_u32());
                c.bus.write32(vp + 4, (seed / 3) % 7);
                c.bus.write32(vp + 8, (seed / 5) % 12);
                // parameters: idx, value, -, kind
                for i in 0..3 {
                    let e = WS_PARAMS + 16 * i;
                    c.bus.write32(e, r.next_u32() % 4);
                    c.bus.write32(e + 4, r.next_u32() % 20);
                    c.bus.write32(e + 12, 1 + r.next_u32() % 2);
                }
                for k in 0..4 {
                    c.bus.write32(0x0044_65c8 + 4 * k, 1 + r.next_u32() % 2);
                }
                c.bus.write32(GAME_GP + 0x2640, (seed / 2) % 2);
                // the rider being: index word (or -1: none) -> world -> AI table
                let idx = if seed % 7 == 0 { 0xffff_ffff } else { r.next_u32() % 4 };
                let (world, ai, table, lst, ride, qobj) = (
                    DATA + 0x1000,
                    DATA + 0x1100,
                    DATA + 0x1200,
                    DATA + 0x7000,
                    DATA + 0x1800,
                    DATA + 0x2000,
                );
                c.bus.write32(GAME_GP + 0xce8, lst);
                c.bus.write32(lst, idx);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, ai);
                c.bus.write32(ai + 0xc, table);
                c.bus.write32(ai + 0x78, WS_UI);
                for i in 0..4 {
                    let e = DATA + 0x1400 + 0x40 * i;
                    c.bus.write32(table + 4 * i, e);
                    c.bus.write32(e + 0x28, if seed % 11 == 3 { 0 } else { ride });
                }
                c.bus.write32(ride + 0x6c0, qobj);
                c.bus.write16(qobj + 0x48, 0);
                c.bus.write32(qobj + 0x4c, fakes::FAKE);
                // the "is a computer rider" vtable call returns 1 (not human)
                // for some seeds (v0 ^ 1 == 0), a pointer otherwise
                let call = if start || entry == 0x0030_2270 { 1 } else { 0 };
                if seed % 4 == 1 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * call, 1);
                }
                c.g[A0][0] = WS_OUT as u64;
                c.g[A1][0] = sx(n);
                c.g[A2][0] = WS_PARAMS as u64;
                c.g[S5][0] = WS_OUT as u64;
                if !start {
                    c.bus.write64(TEST_SP + 0x20, TEST_RA as u64);
                    c.g[S0][0] = vp as u64;
                    c.g[S1][0] = sx(n);
                    c.g[S2][0] = WS_PARAMS as u64;
                    c.g[S3][0] = r.next_u32() as u64;
                    c.g[S4][0] = r.next_u32() as u64;
                }
                match entry {
                    0x0030_2270 => c.g[A2][0] = 0x0050_0000,
                    0x0030_228c => {
                        c.g[S1][0] = if n as i32 > 0 { n as u64 } else { 2 };
                    }
                    0x0030_22a0 => {
                        c.g[A1][0] = if n as i32 > 0 { n as u64 } else { 2 };
                        c.g[T0][0] = 0x0044_65c8;
                        c.g[A3][0] = 2;
                    }
                    0x0030_22dc => {
                        c.g[A1][0] = if n as i32 > 0 { n as u64 } else { 1 };
                        c.g[T0][0] = 0x0044_65c8;
                        c.g[A3][0] = 2;
                    }
                    0x0030_2394 | 0x0030_23b0 | 0x0030_23c8 | 0x0030_23dc | 0x0030_23f0
                    | 0x0030_2420 | 0x0030_2440 => {
                        c.g[S0][0] = WS_UI as u64;
                        c.g[S2][0] = (seed % 6) as u64;
                        c.g[V0][0] = 0x0049_0000;
                    }
                    0x0030_244c => c.g[V1][0] = 0x004d_0000,
                    _ => {}
                }
                c.pc = entry;
            },
        );
    }

    /// `ssx_main(argc, argv)`: the program entry; initialises the system,
    /// builds the `PS2ExecMan` and runs the application main loop. Entries:
    /// the start and every return point after a call.
    #[test]
    fn ssx_main_matches() {
        oc1564_install();
        const ENTRIES: [u32; 6] = [
            0x0031_af80,
            0x0031_afa0,
            0x0031_afa8,
            0x0031_afc4,
            0x0031_afd0,
            0x0031_aff0,
        ];
        fakes::covered(ee::s10::f_0031af80__generated, ssx_main, 60, &|c, seed| {
            oc1564_machine(c, seed, 0x0031_af80, 5, false);
            let entry = ENTRIES[(seed % 6) as usize];
            if entry != 0x0031_af80 {
                c.bus.write64(TEST_SP, TEST_RA as u64);
            }
            c.pc = entry;
        });
    }
}
