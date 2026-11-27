//! Staging module oc1096 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1096`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003b_cba0, sndenc_write_header, "SNDENC_WriteHeader"),
    (0x003b_d220, sndenc_analysis_filterbank, "SNDENC_AnalysisFilterbank"),
    (0x003b_d5b8, sndenc_encode_channel_mode1, "SNDENC_EncodeChannelMode1"),
    (0x003b_d9c0, sndenc_pack_header_mode1, "SNDENC_PackHeaderMode1"),
    (0x003b_f0f0, sndenc_encode_frame, "SNDENC_EncodeFrame"),
];

/// `SNDENC_WriteHeader` (0x003bcba0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_write_header(c: &mut Cpu) {
    if c.pc != 0x003bcba0 {
        return ee::s13::f_003bcba0__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003bcba0;
    loop {
        match pc {
            0x003bcba0 => {
                c.cycles += 31;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[A0][0] = 0xfffffffff8720000;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = c.g[A0][0] | 0x4e1f;
                c.g[S0][0] = 0x510000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5398));
                c.g[A1][0] = 0x20;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[S0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x003bcbd8);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x6), 0u64);
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcbd8)?;
                c.g[A0][0] = 0xb0000;
                c.g[A1][0] = 0x20;
                c.g[RA][0] = sx(0x003bcbe8);
                c.g[A0][0] = c.g[A0][0] | 0x3ee8;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcbe8)?;
                c.g[A0][0] = 0x7ffe0000;
                c.g[A1][0] = 0x20;
                c.g[RA][0] = sx(0x003bcbf8);
                c.g[A0][0] = c.g[A0][0] | 0x8001;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcbf8)?;
                c.g[A0][0] = 0xfc30000;
                c.g[A1][0] = 0x1c;
                c.g[RA][0] = sx(0x003bcc08);
                c.g[A0][0] = c.g[A0][0] | 0xc7dc;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc08)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = 0x1;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
                
                if c.g[V0][0] != c.g[A0][0] { c.g[A0][0] = 0x9; pc = 0x003bcc30; continue; }
                pc = 0x003bcc1c; continue;
            }
            0x003bcc1c => {
                c.cycles += 5;
                c.g[A0][0] = 0x8;
                c.g[RA][0] = sx(0x003bcc28);
                c.g[A1][0] = 0x6;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc28)?;
                let t = true; 
                c.g[A0][0] = 0x60000;
                if t { pc = 0x003bcc3c; continue; }
                pc = 0x003bcc30; continue;
            }
            0x003bcc30 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003bcc38);
                c.g[A1][0] = 0x6;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc38)?;
                c.g[A0][0] = 0x60000;
                pc = 0x003bcc3c; continue;
            }
            0x003bcc3c => {
                c.cycles += 15;
                c.g[A1][0] = 0x13;
                c.g[A0][0] = c.g[A0][0] | 0xe000;
                c.g[RA][0] = sx(0x003bcc4c);
                c.g[S0][0] = 0u64;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc4c)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = 0x21c0000;
                c.g[A1][0] = 0x1a;
                c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                c.g[RA][0] = sx(0x003bcc68);
                c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc68)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
                let t = c.g[V1][0] == 0u64; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bcca4; continue; }
                pc = 0x003bcc78; continue;
            }
            0x003bcc78 => {
                c.cycles += 2;
                c.g[A0][0] = 0x16;
                pc = 0x003bcc80; continue;
            }
            0x003bcc80 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcc88);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcc88)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x16;
                if t { if back_edge_unwinds(c, 0x003bcc80) { return Ok(()); } pc = 0x003bcc80; continue; }
                pc = 0x003bcca0; continue;
            }
            0x003bcca0 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bcca4; continue;
            }
            0x003bcca4 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bccd8; continue; }
                pc = 0x003bccb0; continue;
            }
            0x003bccb0 => {
                c.cycles += 2;
                c.g[A0][0] = 0x17;
                pc = 0x003bccb8; continue;
            }
            0x003bccb8 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bccc0);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bccc0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x17;
                if t { if back_edge_unwinds(c, 0x003bccb8) { return Ok(()); } pc = 0x003bccb8; continue; }
                pc = 0x003bccd8; continue;
            }
            0x003bccd8 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcd18; continue; }
                pc = 0x003bcce8; continue;
            }
            0x003bcce8 => {
                c.cycles += 2;
                c.g[A0][0] = 0u64;
                pc = 0x003bccf0; continue;
            }
            0x003bccf0 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bccf8);
                c.g[A1][0] = 0x3;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bccf8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0u64;
                if t { if back_edge_unwinds(c, 0x003bccf0) { return Ok(()); } pc = 0x003bccf0; continue; }
                pc = 0x003bcd10; continue;
            }
            0x003bcd10 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                pc = 0x003bcd18; continue;
            }
            0x003bcd18 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcd48; continue; }
                pc = 0x003bcd20; continue;
            }
            0x003bcd20 => {
                c.cycles += 2;
                c.g[A0][0] = 0x3;
                pc = 0x003bcd28; continue;
            }
            0x003bcd28 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcd30);
                c.g[A1][0] = 0x2;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcd30)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x3;
                if t { if back_edge_unwinds(c, 0x003bcd28) { return Ok(()); } pc = 0x003bcd28; continue; }
                pc = 0x003bcd48; continue;
            }
            0x003bcd48 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcd88; continue; }
                pc = 0x003bcd58; continue;
            }
            0x003bcd58 => {
                c.cycles += 2;
                c.g[A0][0] = 0x6;
                pc = 0x003bcd60; continue;
            }
            0x003bcd60 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcd68);
                c.g[A1][0] = 0x3;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcd68)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x6;
                if t { if back_edge_unwinds(c, 0x003bcd60) { return Ok(()); } pc = 0x003bcd60; continue; }
                pc = 0x003bcd80; continue;
            }
            0x003bcd80 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                pc = 0x003bcd88; continue;
            }
            0x003bcd88 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcdb8; continue; }
                pc = 0x003bcd90; continue;
            }
            0x003bcd90 => {
                c.cycles += 2;
                c.g[A0][0] = 0x6;
                pc = 0x003bcd98; continue;
            }
            0x003bcd98 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcda0);
                c.g[A1][0] = 0x3;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcda0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x6;
                if t { if back_edge_unwinds(c, 0x003bcd98) { return Ok(()); } pc = 0x003bcd98; continue; }
                pc = 0x003bcdb8; continue;
            }
            0x003bcdb8 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcdf0; continue; }
                pc = 0x003bcdc8; continue;
            }
            0x003bcdc8 => {
                c.cycles += 2;
                c.g[A0][0] = 0x1;
                pc = 0x003bcdd0; continue;
            }
            0x003bcdd0 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcdd8);
                c.g[A1][0] = 0x1;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcdd8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x1;
                if t { if back_edge_unwinds(c, 0x003bcdd0) { return Ok(()); } pc = 0x003bcdd0; continue; }
                pc = 0x003bcdf0; continue;
            }
            0x003bcdf0 => {
                c.cycles += 2;
                c.g[S1][0] = 0x3;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bcdf8; continue;
            }
            0x003bcdf8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bce28; continue; }
                pc = 0x003bce04; continue;
            }
            0x003bce04 => {
                c.cycles += 1;
                c.g[A0][0] = 0x3;
                pc = 0x003bce08; continue;
            }
            0x003bce08 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bce10);
                c.g[A1][0] = 0x2;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bce10)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x3;
                if t { if back_edge_unwinds(c, 0x003bce08) { return Ok(()); } pc = 0x003bce08; continue; }
                pc = 0x003bce28; continue;
            }
            0x003bce28 => {
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                let t = (c.g[S1][0] as i64) >= 0; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { if back_edge_unwinds(c, 0x003bcdf8) { return Ok(()); } pc = 0x003bcdf8; continue; }
                pc = 0x003bce34; continue;
            }
            0x003bce34 => {
                c.cycles += 1;
                c.g[S1][0] = 0x6;
                pc = 0x003bce38; continue;
            }
            0x003bce38 => {
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bce70; continue; }
                pc = 0x003bce48; continue;
            }
            0x003bce48 => {
                c.cycles += 2;
                c.g[A0][0] = 0x7;
                pc = 0x003bce50; continue;
            }
            0x003bce50 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bce58);
                c.g[A1][0] = 0x3;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bce58)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x7;
                if t { if back_edge_unwinds(c, 0x003bce50) { return Ok(()); } pc = 0x003bce50; continue; }
                pc = 0x003bce70; continue;
            }
            0x003bce70 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 11i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { if back_edge_unwinds(c, 0x003bce38) { return Ok(()); } pc = 0x003bce38; continue; }
                pc = 0x003bce7c; continue;
            }
            0x003bce7c => {
                c.cycles += 7;
                c.g[A0][0] = 0x8;
                c.g[RA][0] = sx(0x003bce88);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bce88)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
                let t = c.g[V1][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcec0; continue; }
                pc = 0x003bce98; continue;
            }
            0x003bce98 => {
                c.cycles += 2;
                c.g[A0][0] = 0u64;
                pc = 0x003bcea0; continue;
            }
            0x003bcea0 => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003bcea8);
                c.g[A1][0] = 0x18;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcea8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0u64;
                if t { if back_edge_unwinds(c, 0x003bcea0) { return Ok(()); } pc = 0x003bcea0; continue; }
                pc = 0x003bcec0; continue;
            }
            0x003bcec0 => {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = c.g[S2][0];
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcfbc; continue; }
                pc = 0x003bced4; continue;
            }
            0x003bced4 => {
                c.cycles += 1;
                c.g[S3][0] = 0x1;
                pc = 0x003bced8; continue;
            }
            0x003bced8 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
                let t = c.g[V0][0] != c.g[S3][0]; 
                c.g[S1][0] = 0u64;
                if t { pc = 0x003bcf38; continue; }
                pc = 0x003bcee8; continue;
            }
            0x003bcee8 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = 0xd;
                pc = 0x003bcef0; continue;
            }
            0x003bcef0 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003bcef8);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcef8)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 14i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0xd;
                if t { if back_edge_unwinds(c, 0x003bcef0) { return Ok(()); } pc = 0x003bcef0; continue; }
                pc = 0x003bcf08; continue;
            }
            0x003bcf08 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bcfa4; continue; }
                pc = 0x003bcf14; continue;
            }
            0x003bcf14 => {
                c.cycles += 1;
                c.g[A0][0] = 0xb;
                pc = 0x003bcf18; continue;
            }
            0x003bcf18 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003bcf20);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcf20)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0xb;
                if t { if back_edge_unwinds(c, 0x003bcf18) { return Ok(()); } pc = 0x003bcf18; continue; }
                pc = 0x003bcf30; continue;
            }
            0x003bcf30 => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bcfa4; continue; }
                pc = 0x003bcf38; continue;
            }
            0x003bcf38 => {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = 0xd;
                pc = 0x003bcf40; continue;
            }
            0x003bcf40 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003bcf48);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcf48)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0xd;
                if t { if back_edge_unwinds(c, 0x003bcf40) { return Ok(()); } pc = 0x003bcf40; continue; }
                pc = 0x003bcf58; continue;
            }
            0x003bcf58 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 14i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                if t { pc = 0x003bcf98; continue; }
                pc = 0x003bcf64; continue;
            }
            0x003bcf64 => {
                c.cycles += 1;
                c.g[A0][0] = 0xb;
                pc = 0x003bcf68; continue;
            }
            0x003bcf68 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003bcf70);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcf70)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 14i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0xb;
                if t { if back_edge_unwinds(c, 0x003bcf68) { return Ok(()); } pc = 0x003bcf68; continue; }
                pc = 0x003bcf80; continue;
            }
            0x003bcf80 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                if t { pc = 0x003bcf98; continue; }
                pc = 0x003bcf88; continue;
            }
            0x003bcf88 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003bcf90);
                c.g[A1][0] = 0x5;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcf90)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 24i64) as u64;
                pc = 0x003bcf98; continue;
            }
            0x003bcf98 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0x8;
                if t { if back_edge_unwinds(c, 0x003bcf88) { return Ok(()); } pc = 0x003bcf88; continue; }
                pc = 0x003bcfa0; continue;
            }
            0x003bcfa0 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bcfa4; continue;
            }
            0x003bcfa4 => {
                c.cycles += 4;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.g[S2][0];
                if t { if back_edge_unwinds(c, 0x003bced8) { return Ok(()); } pc = 0x003bced8; continue; }
                pc = 0x003bcfb4; continue;
            }
            0x003bcfb4 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                pc = 0x003bcfbc; continue;
            }
            0x003bcfbc => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bcff4; continue; }
                pc = 0x003bcfc4; continue;
            }
            0x003bcfc4 => {
                c.cycles += 1;
                c.g[A0][0] = 0u64;
                pc = 0x003bcfc8; continue;
            }
            0x003bcfc8 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x003bcfd0);
                c.g[A1][0] = 0x18;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcfd0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = 0u64;
                c.g[RA][0] = sx(0x003bcfe0);
                c.g[A1][0] = 0x18;
                Callee { addr: 0x003bcae8, f: ee::s13::SNDENC_PutBits }.call(c, 0x003bcfe0)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = 0u64;
                if t { if back_edge_unwinds(c, 0x003bcfc8) { return Ok(()); } pc = 0x003bcfc8; continue; }
                pc = 0x003bcff4; continue;
            }
            0x003bcff4 => {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = 0x1;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
                
                if c.g[V0][0] != c.g[V1][0] { c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8)); pc = 0x003bd030; continue; }
                pc = 0x003bd008; continue;
            }
            0x003bd008 => {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0xff;
                c.sb((c.g[V1][0] as u32).wrapping_add(0xe8), 0u64);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                c.sb((c.g[V1][0] as u32).wrapping_add(0x7cb), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                let t = true; 
                c.sb((c.g[V1][0] as u32).wrapping_add(0x7ca), c.g[A0][0]);
                if t { pc = 0x003bd05c; continue; }
                pc = 0x003bd030; continue;
            }
            0x003bd030 => {
                c.cycles += 11;
                c.g[A1][0] = 0xff;
                c.g[A2][0] = 0xf8;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x7d5));
                c.g[V0][0] = c.g[V0][0] | 0x7;
                c.sb((c.g[A0][0] as u32).wrapping_add(0x7d5), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sb((c.g[V0][0] as u32).wrapping_add(0x7d4), c.g[A1][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sb((c.g[V0][0] as u32).wrapping_add(0x7d7), c.g[A2][0]);
                pc = 0x003bd05c; continue;
            }
            0x003bd05c => {
                c.cycles += 7;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003bcba0, 0x003bd078); return Ok(()); }
        }
    }
    });
}

/// `SNDENC_AnalysisFilterbank` (0x003bd220): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_analysis_filterbank(c: &mut Cpu) {
    guest(c, |c| {
'b_3bd290: {
'b_3bd28c: {
c.cycles += 19;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.g[V0][0] = 0x510000;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S1][0] = c.g[V0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S3][0] = c.g[A0][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S4][0] = c.g[A1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5394));
c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xa3c));
let t = c.g[V0][0] == 0u64;
c.g[S5][0] = c.g[A2][0];
if t {
break 'b_3bd28c;
} else {
c.cycles += 4;
c.g[V1][0] = 0x70000000;
c.g[V0][0] = c.g[S3][0] & c.g[V1][0];
let t = c.g[V0][0] == c.g[V1][0];
c.g[FP][0] = 0u64;
if t {
break 'b_3bd290;
} else {
c.cycles += 4;
c.g[V0][0] = c.g[S5][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
let t = true;
c.g[FP][0] = (0u64 < c.g[V0][0]) as u64;
if t {
break 'b_3bd290;
} else {
break 'b_3bd28c;
}
}
}
}
c.cycles += 1;
c.g[FP][0] = 0u64;
break 'b_3bd290;
}
'b_3bd340: {
'b_3bd338: {
'b_3bd31c: {
c.cycles += 2;
let t = c.g[FP][0] == 0u64;
c.g[S0][0] = 0x510000;
if t {
break 'b_3bd31c;
} else {
c.cycles += 23;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5398));
c.g[A3][0] = 0u64;
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A0][0] = c.g[S5][0];
c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[RA][0] = sx(0x003bd2b8);
c.g[A2][0] = sx((c.g[A2][0] as u32) >> 2);
Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bd2b8)?;
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.g[S3][0];
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A2][0] = 0x200;
c.g[A1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
c.g[RA][0] = sx(0x003bd2dc);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
Callee { addr: 0x003bf440, f: ee::s13::SNDENC_PcmToFloat }.call(c, 0x003bd2dc)?;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0xd400;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = c.g[V1][0] & 0x100;
let t = c.g[V1][0] == 0u64;
c.g[V1][0] = 0x10000000;
if t {
break 'b_3bd338;
} else {
c.cycles += 1;
c.g[V1][0] = c.g[V1][0] | 0xd400;
'l_3bd2f8: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;
c.g[S2][0] = 0u64;
if t {
if event_due(c) { deliver_event_here(c, 0x003bd2f8); } continue 'l_3bd2f8;
} else {
c.cycles += 2;
let t = true;
c.g[S7][0] = 0x160;
if t {
break 'b_3bd340;
} else {
break 'b_3bd31c;
}
}
}
}
}
}
c.cycles += 7;
c.g[V0][0] = 0x510000;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[RA][0] = sx(0x003bd338);
c.g[A2][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x4));
Callee { addr: 0x003bf3e0, f: ee::s13::SNDENC_CopyBlock }.call(c, 0x003bd338)?;
break 'b_3bd338;
}
c.cycles += 2;
c.g[S2][0] = 0u64;
c.g[S7][0] = 0x160;
break 'b_3bd340;
}
c.cycles += 2;
c.g[S6][0] = 0x510000;
c.g[S0][0] = 0x510000;
'l_3bd348: loop {
'b_3bd474: {
'b_3bd3cc: {
c.cycles += 2;
let t = c.g[FP][0] == 0u64;
c.g[V1][0] = 0x510000;
if t {
break 'b_3bd3cc;
} else {
'b_3bd384: {
c.cycles += 5;
c.g[A0][0] = 0xe0;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x5398));
c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
let t = c.g[V0][0] != c.g[A0][0];
c.g[V1][0] = sx((c.g[S2][0] as u32) << 9);
if t {
break 'b_3bd384;
} else {
c.cycles += 8;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
c.g[RA][0] = sx(0x003bd37c);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
Callee { addr: 0x003bf4c0, f: ee::s13::SNDENC_WindowFilter4 }.call(c, 0x003bd37c)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd474;
} else {
break 'b_3bd384;
}
}
}
'b_3bd3ac: {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S7][0];
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd3ac;
} else {
c.cycles += 8;
c.g[V1][0] = sx((c.g[S2][0] as u32) << 9);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
c.g[RA][0] = sx(0x003bd3a4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
Callee { addr: 0x003bf5a0, f: ee::s13::SNDENC_WindowFilter6 }.call(c, 0x003bd3a4)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd474;
} else {
break 'b_3bd3ac;
}
}
}
c.cycles += 8;
c.g[V1][0] = sx((c.g[S2][0] as u32) << 9);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
c.g[RA][0] = sx(0x003bd3c4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
Callee { addr: 0x003bf6b0, f: ee::s13::SNDENC_WindowFilter8 }.call(c, 0x003bd3c4)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd474;
} else {
break 'b_3bd3cc;
}
}
}
'b_3bd454: {
'b_3bd420: {
c.cycles += 14;
c.g[A3][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.g[S3][0];
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A2][0] = 0x80;
c.g[A1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
c.g[RA][0] = sx(0x003bd3f0);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
Callee { addr: 0x003bf440, f: ee::s13::SNDENC_PcmToFloat }.call(c, 0x003bd3f0)?;
c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_sub(0x5398));
c.g[V1][0] = 0xe0;
c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
let t = c.g[V0][0] != c.g[V1][0];

if t {
break 'b_3bd420;
} else {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd418);
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
Callee { addr: 0x003bf4c0, f: ee::s13::SNDENC_WindowFilter4 }.call(c, 0x003bd418)?;
let t = true;
c.g[A0][0] = 0x510000;
if t {
break 'b_3bd454;
} else {
break 'b_3bd420;
}
}
}
'b_3bd440: {
c.cycles += 2;
let t = c.g[V0][0] != c.g[S7][0];
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
if t {
break 'b_3bd440;
} else {
c.cycles += 6;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd438);
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
Callee { addr: 0x003bf5a0, f: ee::s13::SNDENC_WindowFilter6 }.call(c, 0x003bd438)?;
let t = true;
c.g[A0][0] = 0x510000;
if t {
break 'b_3bd454;
} else {
break 'b_3bd440;
}
}
}
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd450);
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa24));
Callee { addr: 0x003bf6b0, f: ee::s13::SNDENC_WindowFilter8 }.call(c, 0x003bd450)?;
c.g[A0][0] = 0x510000;
break 'b_3bd454;
}
c.cycles += 8;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa20));
c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.g[A0][0];
c.g[RA][0] = sx(0x003bd470);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x200));
Callee { addr: 0x003bf3e0, f: ee::s13::SNDENC_CopyBlock }.call(c, 0x003bd470)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
break 'b_3bd474;
}
c.cycles += 49;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x100));
c.g[RA][0] = sx(0x003bd484);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bf800, f: ee::s13::SNDENC_FilterStage1 }.call(c, 0x003bd484)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd494);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bf8d0, f: ee::s13::SNDENC_FilterStage2 }.call(c, 0x003bd494)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd4a4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
Callee { addr: 0x003bf960, f: ee::s13::SNDENC_FilterStage3 }.call(c, 0x003bd4a4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd4b4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bfac0, f: ee::s13::SNDENC_FilterStage4 }.call(c, 0x003bd4b4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd4c4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
Callee { addr: 0x003bfb90, f: ee::s13::SNDENC_FilterStage5 }.call(c, 0x003bd4c4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa28));
c.g[RA][0] = sx(0x003bd4d4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bfc50, f: ee::s13::SNDENC_FilterStage6 }.call(c, 0x003bd4d4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa2c));
c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd4e8);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bfd60, f: ee::s13::SNDENC_FilterStage7 }.call(c, 0x003bd4e8)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa30));
c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
c.g[RA][0] = sx(0x003bd4fc);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
Callee { addr: 0x003bfe80, f: ee::s13::SNDENC_FilterStage8 }.call(c, 0x003bd4fc)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa34));
c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd510);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
Callee { addr: 0x003bffa0, f: ee::s13::SNDENC_FilterStage9 }.call(c, 0x003bd510)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x5394));
c.g[A2][0] = c.g[S4][0];
c.g[A3][0] = 0x40;
c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa38));
c.g[RA][0] = sx(0x003bd52c);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
Callee { addr: 0x003c01a0, f: ee::s13::SNDENC_FilterStage10 }.call(c, 0x003bd52c)?;
c.g[V0][0] = ((c.g[S2][0] as i64) < 4i64) as u64;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x003bd348); } continue 'l_3bd348;
} else {
'b_3bd588: {
'b_3bd56c: {
c.cycles += 2;
let t = c.g[FP][0] == 0u64;
c.g[A0][0] = 0x510000;
if t {
break 'b_3bd56c;
} else {
c.cycles += 11;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x5398));
c.g[A1][0] = c.g[S5][0];
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[A3][0] = 0x10000;
c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x800));
c.g[RA][0] = sx(0x003bd564);
c.g[A2][0] = sx((c.g[A2][0] as u32) >> 2);
Callee { addr: 0x003bca28, f: ee::s13::SNDENC_DmaFromSpr }.call(c, 0x003bd564)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_3bd588;
} else {
break 'b_3bd56c;
}
}
}
c.cycles += 7;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5394));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x5398));
c.g[A1][0] = c.g[S5][0];
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa20));
c.g[RA][0] = sx(0x003bd584);
c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
Callee { addr: 0x003bf3e0, f: ee::s13::SNDENC_CopyBlock }.call(c, 0x003bd584)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_3bd588;
}
c.cycles += 11;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
}
}
        Ok(())
    });
}

/// `SNDENC_EncodeChannelMode1` (0x003bd5b8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_encode_channel_mode1(c: &mut Cpu) {
    guest(c, |c| {
c.cycles += 36;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.g[V0][0] = 0x510000;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S1][0] = 0x510000;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S4][0] = c.g[A0][0];
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5394));
c.g[S5][0] = c.g[V0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.g[A1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S2][0] = 0u64;
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5398));
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa20));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[RA][0] = sx(0x003bd610);
c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
Callee { addr: 0x003c03e0, f: ee::s13::SNDENC_FindSubbandPeaks }.call(c, 0x003bd610)?;
c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = 0x450000;
c.g[V1][0] = 0x450000;
c.g[AT][0] = 0x40800000;
c.mtc1(1, 3);
c.g[AT][0] = 0x352f0000;
c.g[AT][0] = c.g[AT][0] | 0x2507;
c.mtc1(1, 2);
c.g[A2][0] = c.g[S3][0];
c.g[T2][0] = c.g[S5][0];
c.g[T1][0] = c.g[S1][0];
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x22c8));
c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x38a0));
'l_3bd648: loop {
'b_3bd6d0: {
'b_3bd68c: {
c.cycles += 12;
c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x5394));
c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xa20));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
c.lwc1(0, (c.g[V1][0] as u32));
c.mul_s(1, 0, 1);
c.c_lt_s(1, 3);
let t = !c.fcc();

if t {
break 'b_3bd68c;
} else {
c.cycles += 5;
c.cvt_w_s(0, 1);
c.mfc1(2, 0);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
let t = true;
c.g[A0][0] = c.lb((c.g[V0][0] as u32));
if t {
break 'b_3bd6d0;
} else {
break 'b_3bd68c;
}
}
}
c.cycles += 16;
c.mfc1(3, 1);
c.g[V0][0] = c.g[V1][0];
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.mul_s(0, 0, 2);
c.cvt_w_s(4, 0);
c.mfc1(3, 4);
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b8));
c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.lwc1(0, (c.g[V0][0] as u32));
c.cvt_s_w(0, 0);
c.c_lt_s(1, 0);
let t = !c.fcc();
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2b7));
if t {
break 'b_3bd6d0;
} else {
c.cycles += 1;
c.g[A0][0] = c.g[A1][0];
break 'b_3bd6d0;
}
}
c.cycles += 5;
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
let t = c.g[V0][0] != 0u64;
c.sb((c.g[V1][0] as u32), c.g[A0][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x003bd648); } continue 'l_3bd648;
} else {
c.cycles += 11;
c.g[V0][0] = 0xdc;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x5398));
c.mult(0, 2, 20, 2);
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V1][0] = 0x450000;
c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa00));
c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x38a0));
c.g[S2][0] = 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xea));
c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
'l_3bd710: loop {
c.cycles += 110;
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 20);
c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
c.g[A2][0] = sx((c.g[S2][0] as u32) << 6);
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
c.g[A1][0] = c.g[S0][0];
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.lwc1(12, (c.g[V0][0] as u32));
c.cvt_s_w(12, 12);
c.div_s(12, 20, 12);
c.g[RA][0] = sx(0x003bd74c);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
Callee { addr: 0x003c0564, f: ee::s13::SNDENC_Quantize10 }.call(c, 0x003bd74c)?;
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
c.g[A0][0] = ((c.g[S2][0] as i64) < 14i64) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x3), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
c.sb((c.g[S1][0] as u32).wrapping_add(0x5), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xa));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x6), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xe));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x9), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
c.sb((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.sb((c.g[S1][0] as u32).wrapping_add(0x371), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x12));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x370), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x12));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x373), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x16));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x372), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x16));
c.sb((c.g[S1][0] as u32).wrapping_add(0x375), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.sb((c.g[S1][0] as u32).wrapping_add(0x374), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1a));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x377), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1a));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x376), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1e));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sb((c.g[S1][0] as u32).wrapping_add(0x379), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1e));
c.sb((c.g[S1][0] as u32).wrapping_add(0x378), c.g[V1][0]);
let t = c.g[A0][0] != 0u64;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa));
if t {
if event_due(c) { deliver_event_here(c, 0x003bd710); } continue 'l_3bd710;
} else {
'b_3bd998: {
c.cycles += 3;
c.g[V0][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x450000;
if t {
break 'b_3bd998;
} else {
c.cycles += 1;
c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x38a0));
'l_3bd8d8: loop {
c.cycles += 48;
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
c.g[A0][0] = sx((c.g[S2][0] as u32) << 6);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.lwc1(12, (c.g[V0][0] as u32));
c.cvt_s_w(12, 12);
c.div_s(12, 20, 12);
c.g[RA][0] = sx(0x003bd90c);
c.g[A1][0] = c.g[S0][0];
Callee { addr: 0x003c05e0, f: ee::s13::SNDENC_Quantize8 }.call(c, 0x003bd90c)?;
c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
c.g[A0][0] = ((c.g[S2][0] as i64) < 24i64) as u64;
c.sb((c.g[S1][0] as u32).wrapping_add(0x1), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x1));
c.sb((c.g[S1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x2));
c.sb((c.g[S1][0] as u32).wrapping_add(0x3), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x3));
c.sb((c.g[S1][0] as u32).wrapping_add(0x2), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x4));
c.sb((c.g[S1][0] as u32).wrapping_add(0x5), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x5));
c.sb((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x6));
c.sb((c.g[S1][0] as u32).wrapping_add(0x7), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x7));
c.sb((c.g[S1][0] as u32).wrapping_add(0x6), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x8));
c.sb((c.g[S1][0] as u32).wrapping_add(0x371), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x9));
c.sb((c.g[S1][0] as u32).wrapping_add(0x370), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xa));
c.sb((c.g[S1][0] as u32).wrapping_add(0x373), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xb));
c.sb((c.g[S1][0] as u32).wrapping_add(0x372), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xc));
c.sb((c.g[S1][0] as u32).wrapping_add(0x375), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xd));
c.sb((c.g[S1][0] as u32).wrapping_add(0x374), c.g[V0][0]);
c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xe));
c.sb((c.g[S1][0] as u32).wrapping_add(0x377), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf));
c.sb((c.g[S1][0] as u32).wrapping_add(0x376), c.g[V0][0]);
let t = c.g[A0][0] != 0u64;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
if t {
if event_due(c) { deliver_event_here(c, 0x003bd8d8); } continue 'l_3bd8d8;
} else {
break 'b_3bd998;
}
}
}
}
c.cycles += 10;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t; return Ok(());
}
}
}
}
        Ok(())
    });
}

/// `SNDENC_PackHeaderMode1` (0x003bd9c0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_pack_header_mode1(c: &mut Cpu) {
    guest(c, |c| {
'b_3bde00: {
c.cycles += 7;
c.g[A1][0] = 0x510000;
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x5398));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
let t = c.g[A0][0] == 0u64;
c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8d));
if t {
break 'b_3bde00;
} else {
c.cycles += 3;
c.g[T1][0] = c.g[A1][0];
c.g[T0][0] = 0x4;
'l_3bd9e8: loop {
c.cycles += 262;
c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_sub(0x1), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x2));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x2), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x3));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x1), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x3));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x4), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x5));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x3), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x5));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x6));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x6), c.g[A0][0]);
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x6));
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x7));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x5), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x9));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x9));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xa));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x7), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xa));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xb));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0xa), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xb));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x9), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xd));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0xc), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xd));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xe));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0xb), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xe));
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xf));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0xe), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x11));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0xd), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x11));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x12));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x12));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x13));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0xf), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x13));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x14));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x12), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x15));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x11), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x15));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x16));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x14), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x16));
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x17));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x13), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x16), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x2));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x15), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x3));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x3));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x17), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x5));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x1a), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x5));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x6));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x19), c.g[A0][0]);
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x6));
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x7));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x9));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x1b), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x9));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xa));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x1e), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xa));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xb));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x1d), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xb));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xd));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x1f), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xd));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xe));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x22), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xe));
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xf));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x21), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x11));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x24), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x11));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x12));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x23), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x12));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x13));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x26), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x13));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x14));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x25), c.g[V1][0]);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x15));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x28), c.g[A0][0]);
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x15));
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x16));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x27), c.g[V1][0]);
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x16));
c.g[V1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x17));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A2][0] as u32).wrapping_add(0x2a), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0x5398));
c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2a));
if t {
if event_due(c) { deliver_event_here(c, 0x003bd9e8); } continue 'l_3bd9e8;
} else {
break 'b_3bde00;
}
}
}
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.pc = t; return Ok(());
        Ok(())
    });
}

/// `SNDENC_EncodeFrame` (0x003bf0f0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sndenc_encode_frame(c: &mut Cpu) {
    if c.pc != 0x003bf0f0 {
        return ee::s13::f_003bf0f0__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003bf0f0;
    loop {
        match pc {
            0x003bf0f0 => {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[T3][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S6][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.g[S1][0] & 0x1;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[V0][0] == 0u64; 
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t { pc = 0x003bf130; continue; }
                pc = 0x003bf128; continue;
            }
            0x003bf128 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0xfffffffffffffffd;
                if t { pc = 0x003bf3b8; continue; }
                pc = 0x003bf130; continue;
            }
            0x003bf130 => {
                c.cycles += 14;
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
                c.g[A0][0] = 0x70000000;
                c.g[S0][0] = c.g[V0][0] | 0xf;
                c.g[V1][0] = 0x510000;
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.g[T3][0] & c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_sub(0x5394), c.g[T3][0]);
                c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.sb((c.g[T3][0] as u32).wrapping_add(0xa3c), c.g[V0][0]);
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x5394));
                c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0xa3c));
                let t = c.g[V0][0] == 0u64; 
                c.g[S5][0] = c.g[V1][0];
                if t { pc = 0x003bf24c; continue; }
                pc = 0x003bf168; continue;
            }
            0x003bf168 => {
                c.cycles += 8;
                c.g[V0][0] = 0x510000;
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa41));
                c.sw((c.g[A1][0] as u32).wrapping_add(0xa24), c.g[T3][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = 0xe0;
                c.g[V1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] != c.g[A0][0]; 
                c.g[S4][0] = c.g[V0][0];
                if t { pc = 0x003bf1a8; continue; }
                pc = 0x003bf188; continue;
            }
            0x003bf188 => {
                c.cycles += 8;
                c.g[A0][0] = 0x450000;
                c.g[A1][0] = c.g[T3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3500));
                c.g[A2][0] = 0x40;
                c.g[RA][0] = sx(0x003bf1a0);
                c.g[A3][0] = 0u64;
                Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bf1a0)?;
                let t = true; 
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe41));
                if t { pc = 0x003bf1ec; continue; }
                pc = 0x003bf1a8; continue;
            }
            0x003bf1a8 => {
                c.cycles += 3;
                c.g[V0][0] = 0x160;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x450000;
                if t { pc = 0x003bf1d4; continue; }
                pc = 0x003bf1b4; continue;
            }
            0x003bf1b4 => {
                c.cycles += 8;
                c.g[A0][0] = 0x450000;
                c.g[A1][0] = c.g[T3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3100));
                c.g[A2][0] = 0x60;
                c.g[RA][0] = sx(0x003bf1cc);
                c.g[A3][0] = 0u64;
                Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bf1cc)?;
                let t = true; 
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1041));
                if t { pc = 0x003bf1ec; continue; }
                pc = 0x003bf1d4; continue;
            }
            0x003bf1d4 => {
                c.cycles += 6;
                c.g[A1][0] = c.g[T3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2b00));
                c.g[A2][0] = 0x80;
                c.g[RA][0] = sx(0x003bf1e8);
                c.g[A3][0] = 0u64;
                Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bf1e8)?;
                c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1241));
                pc = 0x003bf1ec; continue;
            }
            0x003bf1ec => {
                c.cycles += 24;
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                c.g[A0][0] = 0x450000;
                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3680));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa20), c.g[T3][0]);
                c.g[A2][0] = 0x12;
                c.g[A3][0] = 0x10000;
                c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x200));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[T0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x10));
                c.g[T1][0] = sx((c.g[T3][0] as u32).wrapping_add(0x30));
                c.g[T2][0] = sx((c.g[T3][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(0xc0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa38), c.g[V0][0]);
                c.g[A1][0] = c.g[T3][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa2c), c.g[T0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa30), c.g[T1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa34), c.g[T2][0]);
                c.g[RA][0] = sx(0x003bf244);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa28), c.g[T3][0]);
                Callee { addr: 0x003bc968, f: ee::s13::SNDENC_DmaToSpr }.call(c, 0x003bf244)?;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bf2e8; continue; }
                pc = 0x003bf24c; continue;
            }
            0x003bf24c => {
                c.cycles += 9;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa41));
                c.g[V1][0] = 0x510000;
                c.sw((c.g[A1][0] as u32).wrapping_add(0xa20), c.g[V0][0]);
                c.g[S4][0] = c.g[V1][0];
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = 0xe0;
                c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
                
                if c.g[V1][0] != c.g[A0][0] { c.g[V0][0] = 0x160; pc = 0x003bf27c; continue; }
                pc = 0x003bf270; continue;
            }
            0x003bf270 => {
                c.cycles += 3;
                c.g[V0][0] = 0x450000;
                let t = true; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3500));
                if t { pc = 0x003bf2a0; continue; }
                pc = 0x003bf27c; continue;
            }
            0x003bf27c => {
                c.cycles += 2;
                
                if c.g[V1][0] != c.g[V0][0] { c.g[V0][0] = 0x1e0; pc = 0x003bf290; continue; }
                pc = 0x003bf284; continue;
            }
            0x003bf284 => {
                c.cycles += 3;
                c.g[V0][0] = 0x450000;
                let t = true; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3100));
                if t { pc = 0x003bf2a0; continue; }
                pc = 0x003bf290; continue;
            }
            0x003bf290 => {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                if t { pc = 0x003bf2a8; continue; }
                pc = 0x003bf298; continue;
            }
            0x003bf298 => {
                c.cycles += 2;
                c.g[V0][0] = 0x450000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2b00));
                pc = 0x003bf2a0; continue;
            }
            0x003bf2a0 => {
                c.cycles += 2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0xa24), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                pc = 0x003bf2a8; continue;
            }
            0x003bf2a8 => {
                c.cycles += 16;
                c.g[A0][0] = 0x450000;
                c.g[A1][0] = 0x450000;
                c.g[A2][0] = 0x450000;
                c.g[A3][0] = 0x450000;
                c.g[V0][0] = 0x450000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3680));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3670));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3650));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3620));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x35c0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa28), c.g[A0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa38), c.g[V0][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa2c), c.g[A1][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa30), c.g[A2][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa34), c.g[A3][0]);
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bf2e8; continue;
            }
            0x003bf2e8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                
                if c.g[V0][0] == c.g[S1][0] { c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398)); pc = 0x003bf304; continue; }
                pc = 0x003bf2f4; continue;
            }
            0x003bf2f4 => {
                c.cycles += 4;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                c.g[RA][0] = sx(0x003bf300);
                c.sh((c.g[V1][0] as u32).wrapping_add(0x6), 0u64);
                Callee { addr: 0x003bcba0, f: ee::s13::SNDENC_WriteHeader }.call(c, 0x003bf300)?;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bf304; continue;
            }
            0x003bf304 => {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[V0][0] == 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003bf388; continue; }
                pc = 0x003bf310; continue;
            }
            0x003bf310 => {
                c.cycles += 4;
                c.g[S2][0] = c.g[S4][0];
                c.g[S3][0] = 0x1;
                c.g[S1][0] = c.g[S6][0];
                pc = 0x003bf320; continue;
            }
            0x003bf320 => {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                c.g[V0][0] = c.g[A0][0] & 0xf;
                let t = c.g[V0][0] != 0u64; 
                c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
                if t { if back_edge_unwinds(c, 0x003bf128) { return Ok(()); } pc = 0x003bf128; continue; }
                pc = 0x003bf330; continue;
            }
            0x003bf330 => {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_sub(0x5394));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[RA][0] = sx(0x003bf348);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                Callee { addr: 0x003bd220, f: ee::s13::SNDENC_AnalysisFilterbank }.call(c, 0x003bf348)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x5398));
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                let t = c.g[V1][0] != c.g[S3][0]; 
                
                if t { pc = 0x003bf368; continue; }
                pc = 0x003bf358; continue;
            }
            0x003bf358 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003bf360);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x003bd5b8, f: ee::s13::SNDENC_EncodeChannelMode1 }.call(c, 0x003bf360)?;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                if t { pc = 0x003bf374; continue; }
                pc = 0x003bf368; continue;
            }
            0x003bf368 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003bf370);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x003bde08, f: ee::s13::SNDENC_EncodeChannelMode2 }.call(c, 0x003bf370)?;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                pc = 0x003bf374; continue;
            }
            0x003bf374 => {
                c.cycles += 5;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x003bf320) { return Ok(()); } pc = 0x003bf320; continue; }
                pc = 0x003bf388; continue;
            }
            0x003bf388 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_sub(0x5398));
                c.g[A0][0] = 0x1;
                c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                let t = c.g[V1][0] != c.g[A0][0]; 
                
                if t { pc = 0x003bf3ac; continue; }
                pc = 0x003bf39c; continue;
            }
            0x003bf39c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003bf3a4);
                Callee { addr: 0x003bd9c0, f: ee::s13::SNDENC_PackHeaderMode1 }.call(c, 0x003bf3a4)?;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x003bf3b8; continue; }
                pc = 0x003bf3ac; continue;
            }
            0x003bf3ac => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003bf3b4);
                c.g[A0][0] = c.g[S6][0];
                Callee { addr: 0x003be6e0, f: ee::s13::SNDENC_PackHeaderMode2 }.call(c, 0x003bf3b4)?;
                c.g[V0][0] = 0u64;
                pc = 0x003bf3b8; continue;
            }
            0x003bf3b8 => {
                c.cycles += 10;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003bf0f0, 0x003bf3e0); return Ok(()); }
        }
    }
    });
}

@@TESTS@@
