//! Staging module oc1120 (wave wave270): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1120`.

#![allow(unused_imports)]

use crate::events::back_edge_unwinds;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_6318, libmpeg_decode_slice, "libmpeg_DecodeSlice"),
    (0x0040_65d0, libmpeg_decode_macroblock, "libmpeg_DecodeMacroblock"),
    (0x0040_8278, libmpeg_copy_output_raw, "libmpeg_CopyOutputRaw"),
    (0x0040_9088, libmpeg_convert_picture_csc, "libmpeg_ConvertPictureCsc"),
];

/// `int libmpeg_DecodeSlice(DecState* dec, int n)` (0x00406318): decodes one
/// MPEG slice of `n` macroblocks. Reads the slice header (`SliceHeader`, a
/// non-zero result is returned at once), then loops per macroblock: waits for
/// the IPU (`WaitBdec`; on failure returns 4 when the abort flag `dec[0x10a0]`
/// was set, else 2), reads the address increment when it is 0 (`ShowBits(23)`,
/// `GetMbIncrement`; returns 3/1 on a flagged error `dec[0x11c]`), reports a
/// macroblock past the end (`Errorf`, 2), decodes a coded macroblock
/// (`DecodeMacroblock`, increment 1) or a skipped one (`SkippedMacroblock`),
/// sets it up (`SetupMacroblock`) and stores the previous one's prediction
/// (`AddPredAndStore`), then toggles the buffer index `dec[0x810]`. Returns 0
/// when all `n` macroblocks are done. Converted from the generated body (a
/// `match pc` body: every register, cycle and call kept); any other `pc` (a
/// resume after an unwound callee or event) runs the generated body (at
/// install: its `__generated` alias).
pub fn libmpeg_decode_slice(c: &mut Cpu) {
    if c.pc != 0x00406318 {
        return ee::s15::f_00406318__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x00406318;
    loop {
        match pc {
            0x00406318 => {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S3][0]);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S0][0]);
                c.g[S3][0] = c.g[A1][0];
                c.g[S0][0] = c.g[A0][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x44), 0u64);
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x44));
                c.g[T0][0] = c.g[SP][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S2][0]);
                c.g[RA][0] = sx(0x00406350);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                Callee { addr: 0x004061e8, f: ee::s15::libmpeg_SliceHeader }.call(c, 0x00406350)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                if t { pc = 0x004064f8; continue; }
                pc = 0x00406358; continue;
            }
            0x00406358 => {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                pc = 0x00406360; continue;
            }
            0x00406360 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810)); pc = 0x00406378; continue; }
                pc = 0x00406370; continue;
            }
            0x00406370 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x004064f4; continue; }
                pc = 0x00406378; continue;
            }
            0x00406378 => {
                c.cycles += 8;
                c.g[V1][0] = 0x140;
                c.g[A0][0] = c.g[S0][0];
                c.mult(0, 5, 2, 3);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[RA][0] = sx(0x00406390);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x6cc), 0u64);
                Callee { addr: 0x00405b70, f: ee::s15::libmpeg_WaitBdec }.call(c, 0x00406390)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                if t { pc = 0x004063ac; continue; }
                pc = 0x00406398; continue;
            }
            0x00406398 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10a0));
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x4;
                if t { pc = 0x00406414; continue; }
                pc = 0x004063a4; continue;
            }
            0x004063a4 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10a0), 0u64);
                if t { pc = 0x004064f4; continue; }
                pc = 0x004063ac; continue;
            }
            0x004063ac => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                if t { pc = 0x004063f8; continue; }
                pc = 0x004063b4; continue;
            }
            0x004063b4 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x004063c0);
                c.g[A1][0] = 0x17;
                Callee { addr: 0x004070f0, f: ee::s15::libmpeg_ShowBits }.call(c, 0x004063c0)?;
                
                if c.g[V0][0] == 0u64 { c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64); pc = 0x004063d8; continue; }
                pc = 0x004063c8; continue;
            }
            0x004063c8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                let t = c.g[V0][0] == 0u64; 
                
                if t { pc = 0x004063e0; continue; }
                pc = 0x004063d4; continue;
            }
            0x004063d4 => {
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                pc = 0x004063d8; continue;
            }
            0x004063d8 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x3;
                if t { pc = 0x004064f4; continue; }
                pc = 0x004063e0; continue;
            }
            0x004063e0 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x004063e8);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00405fb8, f: ee::s15::libmpeg_MacroblockAddressIncrement }.call(c, 0x004063e8)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
                let t = c.g[V1][0] != 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                if t { pc = 0x00406454; continue; }
                pc = 0x004063f4; continue;
            }
            0x004063f4 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                pc = 0x004063f8; continue;
            }
            0x004063f8 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                if t { pc = 0x0040641c; continue; }
                pc = 0x00406404; continue;
            }
            0x00406404 => {
                c.cycles += 4;
                c.g[A1][0] = 0x490000;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00406414);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f70));
                Callee { addr: 0x004034a8, f: ee::s15::libmpeg_Error }.call(c, 0x00406414)?;
                pc = 0x00406414; continue;
            }
            0x00406414 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x004064f4; continue; }
                pc = 0x0040641c; continue;
            }
            0x0040641c => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                if t { pc = 0x00406460; continue; }
                pc = 0x00406428; continue;
            }
            0x00406428 => {
                c.cycles += 11;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4c));
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[T0][0] = c.g[SP][0];
                c.g[T1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0040644c);
                c.g[T2][0] = c.g[S1][0];
                Callee { addr: 0x004065d0, f: ee::s15::libmpeg_DecodeMacroblock }.call(c, 0x0040644c)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                if t { pc = 0x00406484; continue; }
                pc = 0x00406454; continue;
            }
            0x00406454 => {
                c.cycles += 3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                let t = true; 
                c.g[V0][0] = 0x1;
                if t { pc = 0x004064f4; continue; }
                pc = 0x00406460; continue;
            }
            0x00406460 => {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4c));
                c.g[A3][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x00406478);
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                Callee { addr: 0x00406510, f: ee::s15::libmpeg_SkippedMacroblock }.call(c, 0x00406478)?;
                let t = c.g[V0][0] == 0u64; 
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                if t { pc = 0x004064ac; continue; }
                pc = 0x00406480; continue;
            }
            0x00406480 => {
                c.cycles += 1;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                pc = 0x00406484; continue;
            }
            0x00406484 => {
                c.cycles += 10;
                c.g[T2][0] = c.g[S2][0];
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                c.g[T3][0] = c.g[S1][0];
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                c.g[A0][0] = c.g[S0][0];
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4c));
                c.g[RA][0] = sx(0x004064a4);
                c.g[T1][0] = c.g[SP][0];
                Callee { addr: 0x00403e00, f: ee::s15::libmpeg_SetupMacroblock }.call(c, 0x004064a4)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                if t { pc = 0x004064b4; continue; }
                pc = 0x004064ac; continue;
            }
            0x004064ac => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                if t { if back_edge_unwinds(c, 0x00406414) { return Ok(()); } pc = 0x00406414; continue; }
                pc = 0x004064b4; continue;
            }
            0x004064b4 => {
                c.cycles += 2;
                
                if c.g[A0][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810)); pc = 0x004064d4; continue; }
                pc = 0x004064bc; continue;
            }
            0x004064bc => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x004064cc);
                c.g[A1][0] = c.g[A1][0] ^ 0x1;
                Callee { addr: 0x00404b78, f: ee::s15::libmpeg_AddPredAndStore }.call(c, 0x004064cc)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
                pc = 0x004064d4; continue;
            }
            0x004064d4 => {
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x810), c.g[V0][0]);
                let t = true; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                if t { if back_edge_unwinds(c, 0x00406360) { return Ok(()); } pc = 0x00406360; continue; }
                pc = 0x004064f4; continue;
            }
            0x004064f4 => {
                c.cycles += 1;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                pc = 0x004064f8; continue;
            }
            0x004064f8 => {
                c.cycles += 6;
                c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00406318, 0x00406510); return Ok(()); }
        }
    }
    });
}

/// `int libmpeg_DecodeMacroblock(dec, int* mbtype, int* motion_type,
/// int* dct_type, int* pmv, int* field_select, int* dm_vector)` (0x004065d0):
/// parses one coded macroblock header. Sets the IPU quantiser-scale field of
/// `IPU_CTRL`, reads the macroblock type (the macroblock-type reader, 0 reports an
/// error and sets `dec[0x11c]`), the motion type and DCT type bits, the
/// quantiser code, then the forward / backward motion vectors (the MPEG-1
/// reader `0x406ca0` or the MPEG-2 reader `0x406b00`, chosen by `dec[0x848]`),
/// and for blocks with coefficients the IPU command (clear the block buffer,
/// wait, issue the macroblock command). Returns 0 on an error
/// (`dec[0x11c]` set) else 1, after resetting the motion-vector predictors
/// for intra and skip cases. Converted from the generated labelled-block body.
pub fn libmpeg_decode_macroblock(c: &mut Cpu) {
    guest(c, |c| {
'b_406a48: {
'b_406678: {
c.cycles += 34;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
c.g[T3][0] = 0x10000000;
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S6][0]);
c.g[T3][0] = c.g[T3][0] | 0x2010;
c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S5][0]);
c.g[V0][0] = 0xfffffffff8ff0000;
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S2][0]);
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S1][0]);
c.g[S2][0] = c.g[A1][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S0][0]);
c.g[S5][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[FP][0]);
c.g[S1][0] = c.g[T0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S7][0]);
c.g[S6][0] = c.g[T2][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S4][0]);
c.g[A1][0] = 0x1;
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S3][0]);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
c.g[V1][0] = c.lw((c.g[T3][0] as u32));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.sw((c.g[T3][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[S0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A3][0]);
c.g[RA][0] = sx(0x0040664c);
c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[T1][0]);
Callee { addr: 0x00406f78, f: ee::s15::libmpeg_IpuVdec }.call(c, 0x0040664c)?;
c.g[V1][0] = c.g[V0][0];
let t = c.g[V1][0] != 0u64;
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
if t {
break 'b_406678;
} else {
c.cycles += 8;
c.g[A1][0] = 0x490000;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00406668);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7fc0));
Callee { addr: 0x004034a8, f: ee::s15::libmpeg_Error }.call(c, 0x00406668)?;
c.g[V1][0] = 0x1;
c.g[V0][0] = 0u64;
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
if t {
break 'b_406a48;
} else {
break 'b_406678;
}
}
}
'b_4066e4: {
'b_4066e0: {
'b_4066b4: {
c.cycles += 3;
c.g[V0][0] = c.g[V1][0] & 0xc;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x3;
if t {
break 'b_4066b4;
} else {
'b_4066a4: {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
let t = c.g[V1][0] != c.g[V0][0];
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_4066a4;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x17c));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_4066a4;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S5][0] as u32), c.g[V0][0]);
if t {
break 'b_4066e0;
} else {
break 'b_4066a4;
}
}
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x004066ac);
c.g[A1][0] = 0x2;
Callee { addr: 0x004072e8, f: ee::s15::libmpeg_GetBits }.call(c, 0x004066ac)?;
let t = true;
c.sw((c.g[S5][0] as u32), c.g[V0][0]);
if t {
break 'b_4066e0;
} else {
break 'b_4066b4;
}
}
}
c.cycles += 3;
c.g[V0][0] = c.g[V1][0] & 0x1;
if c.g[V0][0] == 0u64 {
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
break 'b_4066e4;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x1;
if t {
break 'b_4066e0;
} else {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
c.g[A0][0] = 0x2;
c.g[V0][0] = c.g[V0][0] ^ 0x3;
if c.g[V0][0] == 0 { c.g[V1][0] = c.g[A0][0]; }
c.sw((c.g[S5][0] as u32), c.g[V1][0]);
break 'b_4066e0;
}
}
}
c.cycles += 1;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
break 'b_4066e4;
}
'b_406720: {
'b_40670c: {
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = c.g[A2][0] != c.g[V0][0];
c.g[A1][0] = c.lw((c.g[S5][0] as u32));
if t {
break 'b_40670c;
} else {
c.cycles += 7;
c.g[S3][0] = 0x1;
c.g[V0][0] = 0x2;
c.g[V1][0] = c.g[A1][0] ^ 0x1;
c.g[A0][0] = c.g[A1][0] ^ 0x2;
if c.g[V1][0] == 0 { c.g[S3][0] = c.g[V0][0]; }
let t = true;
c.g[S4][0] = (c.g[A0][0] < 0x1u64) as u64;
if t {
break 'b_406720;
} else {
break 'b_40670c;
}
}
}
c.cycles += 5;
c.g[V1][0] = 0x2;
c.g[S3][0] = 0x1;
c.g[S4][0] = 0u64;
c.g[V0][0] = c.g[A1][0] ^ 0x2;
if c.g[V0][0] == 0 { c.g[S3][0] = c.g[V1][0]; }
break 'b_406720;
}
'b_406738: {
c.cycles += 4;
c.g[V0][0] = c.g[A1][0] ^ 0x3;
c.g[S7][0] = 0u64;
let t = c.g[S4][0] != 0u64;
c.g[FP][0] = (c.g[V0][0] < 0x1u64) as u64;
if t {
break 'b_406738;
} else {
c.cycles += 2;
c.g[V0][0] = c.g[A2][0] ^ 0x3;
c.g[S7][0] = (c.g[V0][0] < 0x1u64) as u64;
break 'b_406738;
}
}
'b_406778: {
'b_406774: {
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = c.g[A2][0] != c.g[V0][0];
c.g[V0][0] = 0u64;
if t {
break 'b_406774;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x17c));
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0u64;
if t {
break 'b_406774;
} else {
'b_406770: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x3;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_406770;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x00406768);
c.g[A1][0] = 0x1;
Callee { addr: 0x004072e8, f: ee::s15::libmpeg_GetBits }.call(c, 0x00406768)?;
let t = true;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
if t {
break 'b_406778;
} else {
break 'b_406770;
}
}
}
c.cycles += 1;
c.g[V0][0] = 0u64;
break 'b_406774;
}
}
}
c.cycles += 1;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_406778;
}
'b_40679c: {
c.cycles += 5;
c.sw((c.g[V1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V1][0] & 0x10;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_40679c;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x00406794);
c.g[A1][0] = 0x5;
Callee { addr: 0x004072e8, f: ee::s15::libmpeg_GetBits }.call(c, 0x00406794)?;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1b4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
break 'b_40679c;
}
}
'b_406834: {
'b_4067c4: {
c.cycles += 3;
c.g[V0][0] = c.g[V1][0] & 0x8;
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x848));
break 'b_4067c4;
} else {
c.cycles += 3;
c.g[V0][0] = c.g[V1][0] & 0x1;
if c.g[V0][0] == 0u64 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
break 'b_406834;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
if c.g[V0][0] == 0u64 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
break 'b_406834;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x848));
break 'b_4067c4;
}
}
}
}
'b_40680c: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_40680c;
} else {
c.cycles += 16;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x164));
c.g[A1][0] = c.g[S1][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[FP][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[A2][0] = c.g[S6][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S7][0]);
c.g[T0][0] = 0u64;
c.g[T1][0] = c.g[S3][0];
c.g[RA][0] = sx(0x00406804);
c.g[T2][0] = c.g[S4][0];
Callee { addr: 0x00406b00, f: ee::s15::libmpeg_MotionVectors }.call(c, 0x00406804)?;
let t = true;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
if t {
break 'b_406834;
} else {
break 'b_40680c;
}
}
}
c.cycles += 10;
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x158));
c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x154));
c.g[A1][0] = c.g[S1][0];
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
c.g[A2][0] = c.g[S6][0];
c.g[T1][0] = 0u64;
c.g[T0][0] = c.g[A3][0];
c.g[RA][0] = sx(0x00406830);
c.g[T2][0] = 0u64;
Callee { addr: 0x00406ca0, f: ee::s15::libmpeg_MotionVector }.call(c, 0x00406830)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
break 'b_406834;
}
c.cycles += 2;
let t = c.g[V1][0] != 0u64;
c.g[V0][0] = 0u64;
if t {
break 'b_406a48;
} else {
'b_4068c0: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x4;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_4068c0;
} else {
'b_406898: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x848));
let t = c.g[V0][0] == 0u64;
c.g[A2][0] = c.g[S6][0];
if t {
break 'b_406898;
} else {
c.cycles += 16;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x170));
c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
c.g[T1][0] = c.g[S3][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S7][0]);
c.g[T2][0] = c.g[S4][0];
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x1));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = c.g[S1][0];
c.g[RA][0] = sx(0x00406890);
c.g[T0][0] = 0x1;
Callee { addr: 0x00406b00, f: ee::s15::libmpeg_MotionVectors }.call(c, 0x00406890)?;
let t = true;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
if t {
break 'b_4068c0;
} else {
break 'b_406898;
}
}
}
c.cycles += 10;
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x160));
c.g[T3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x15c));
c.g[A0][0] = c.g[S0][0];
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
c.g[T1][0] = 0u64;
c.g[T0][0] = c.g[A3][0];
c.g[RA][0] = sx(0x004068bc);
c.g[T2][0] = 0u64;
Callee { addr: 0x00406ca0, f: ee::s15::libmpeg_MotionVector }.call(c, 0x004068bc)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
break 'b_4068c0;
}
}
c.cycles += 2;
let t = c.g[V1][0] != 0u64;
c.g[V0][0] = 0u64;
if t {
break 'b_406a48;
} else {
'b_4068f4: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V1][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.g[V1][0] & 0x3;
if t {
break 'b_4068f4;
} else {
'b_4068f0: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_4068f0;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x004068ec);
c.g[A1][0] = 0x1;
Callee { addr: 0x004071f8, f: ee::s15::libmpeg_FlushBits }.call(c, 0x004068ec)?;
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
break 'b_4068f0;
}
}
c.cycles += 1;
c.g[V0][0] = c.g[V1][0] & 0x3;
break 'b_4068f4;
}
}
'b_406980: {
'b_406968: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x140;
if t {
break 'b_406968;
} else {
c.cycles += 27;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
c.g[A1][0] = 0x300;
c.mult(0, 4, 2, 3);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[RA][0] = sx(0x00406914);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x594));
Callee { addr: 0x00408af8, f: ee::s15::libmpeg_IpuFromDmaStart }.call(c, 0x00406914)?;
c.g[RA][0] = sx(0x0040691c);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00406e20, f: ee::s15::libmpeg_IpuWaitBusy }.call(c, 0x0040691c)?;
c.g[A1][0] = c.lw((c.g[S2][0] as u32));
c.g[A3][0] = 0x20000000;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
c.g[A0][0] = c.g[S0][0];
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b0));
c.g[A1][0] = c.g[A1][0] & 0x1;
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A1][0] = sx((c.g[A1][0] as u32) << 27);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 26);
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.g[A1][0] = c.g[A1][0] | c.g[A2][0];
c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
c.g[V0][0] = sx((c.g[V0][0] as u32) << 25);
c.g[RA][0] = sx(0x00406960);
c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
Callee { addr: 0x00406df0, f: ee::s15::libmpeg_IpuCommand }.call(c, 0x00406960)?;
let t = true;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
if t {
break 'b_406980;
} else {
break 'b_406968;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x810));
c.g[A0][0] = 0x1;
c.mult(0, 5, 2, 3);
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.sw((c.g[V0][0] as u32).wrapping_add(0x6cc), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x11c));
break 'b_406980;
}
'b_406990: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1b0), 0u64);
if t {
break 'b_406990;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = 0u64;
if t {
break 'b_406a48;
} else {
break 'b_406990;
}
}
}
'b_4069e8: {
'b_4069bc: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x1;
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
break 'b_4069bc;
} else {
c.cycles += 6;
c.g[V1][0] = 0x1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1b0), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x1;
if c.g[V0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
break 'b_4069e8;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x180));
break 'b_4069bc;
}
}
}
c.cycles += 2;
if c.g[V0][0] != 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
break 'b_4069e8;
} else {
c.cycles += 9;
c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
c.sw((c.g[S1][0] as u32), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
break 'b_4069e8;
}
}
c.cycles += 3;
c.g[V0][0] = 0x2;
let t = c.g[A0][0] != c.g[V0][0];
c.g[V0][0] = 0x1;
if t {
break 'b_406a48;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x9;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_406a48;
} else {
'b_406a44: {
'b_406a2c: {
c.cycles += 8;
c.sw((c.g[S1][0] as u32), 0u64);
c.g[V1][0] = 0x3;
c.sw((c.g[S1][0] as u32).wrapping_add(0x14), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
let t = c.g[V0][0] != c.g[V1][0];
c.g[V1][0] = 0x1;
if t {
break 'b_406a2c;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S5][0] as u32), c.g[A0][0]);
if t {
break 'b_406a44;
} else {
break 'b_406a2c;
}
}
}
c.cycles += 6;
c.sw((c.g[S5][0] as u32), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x174));
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
c.g[V0][0] = c.g[V0][0] ^ 0x2;
c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
c.sw((c.g[T0][0] as u32), c.g[V0][0]);
break 'b_406a44;
}
c.cycles += 1;
c.g[V0][0] = 0x1;
break 'b_406a48;
}
}
}
}
}
c.cycles += 12;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
c.pc = t; return Ok(());
    });
}

/// `void libmpeg_CopyOutputRaw(DecState* dec, Picture* pic)` (0x00408278):
/// copies a decoded raw picture out through the scratchpad: for each of the
/// 1 or 2 fields and each macroblock row `pic[3]`, DMA `qwc` quadwords of
/// `pic[0]` into the scratchpad (channel 9, `0x1000d400`), wait, DMA them
/// back out to the destination row at `dec[0xd8]` (channel 8, `0x1000d000`),
/// wait for the queue to drain, and advance the row by the stride from
/// `dec[0xe0]`. Interrupts are masked around each kick (`DIntr`/`EIntr`).
/// Converted from the generated labelled-block body.
pub fn libmpeg_copy_output_raw(c: &mut Cpu) {
    guest(c, |c| {
'b_40834c: {
'b_408324: {
'b_4082f8: {
'b_4082ec: {
c.cycles += 24;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.g[V0][0] = 0xfff0000;
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S6][0]);
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
c.g[S6][0] = c.g[A1][0];
c.sw((c.g[SP][0] as u32), c.g[A0][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[FP][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S7][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S5][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S4][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S3][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S1][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
c.g[A2][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = c.lw((c.g[S6][0] as u32));
c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd8));
c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x174));
c.g[S2][0] = c.g[V1][0] & c.g[V0][0];
c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
c.g[V1][0] = 0x3;
let t = c.g[A1][0] == c.g[V1][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
if t {
break 'b_4082ec;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe0));
let t = c.g[A0][0] != 0u64;
c.g[V1][0] = c.g[A0][0];
if t {
break 'b_408324;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
if t {
break 'b_4082f8;
} else {
break 'b_4082ec;
}
}
}
}
c.cycles += 3;
c.g[A3][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe0));
c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
break 'b_4082f8;
}
'b_408318: {
'b_408314: {
c.cycles += 4;
c.g[A0][0] = 0x180;
c.mult(0, 21, 2, 4);
let t = c.g[V1][0] == 0u64;
c.g[S4][0] = sx(((c.g[S5][0] as u32) as i32 >> 4) as u32);
if t {
break 'b_408314;
} else {
c.cycles += 3;
c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
let t = true;
c.mult(0, 30, 2, 4);
if t {
break 'b_408318;
} else {
break 'b_408314;
}
}
}
c.cycles += 1;
c.g[FP][0] = c.g[S5][0];
break 'b_408318;
}
c.cycles += 3;
c.g[V0][0] = 0x1;
let t = true;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
if t {
break 'b_40834c;
} else {
break 'b_408324;
}
}
c.cycles += 10;
c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
c.g[A1][0] = 0x180;
c.g[V1][0] = 0xc0;
c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.mult(0, 30, 4, 3);
c.mult(1, 21, 2, 5);
c.g[V1][0] = 0x2;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.g[S4][0] = sx(((c.g[S5][0] as u32) as i32 >> 4) as u32);
break 'b_40834c;
}
'b_4084d8: {
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
let t = c.g[A2][0] == 0u64;
c.g[A1][0] = 0u64;
if t {
break 'b_4084d8;
} else {
c.cycles += 2;
c.g[A2][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
'l_408360: loop {
'b_4084ac: {
'b_4084a4: {
c.cycles += 3;
c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
let t = (c.g[A2][0] as i64) <= 0;
c.g[S0][0] = 0u64;
if t {
break 'b_4084a4;
} else {
c.cycles += 1;
c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
'l_408370: loop {
'b_4083c0: {
c.cycles += 17;
c.g[RA][0] = sx(0x00408378);
Callee { addr: 0x0042c078, f: ee::s15::DIntr }.call(c, 0x00408378)?;
c.g[V1][0] = 0x10000000;
c.g[A1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xd480;
c.g[A1][0] = c.g[A1][0] | 0xd410;
c.sw((c.g[V1][0] as u32), 0u64);
c.g[A0][0] = 0x10000000;
c.sw((c.g[A1][0] as u32), c.g[S2][0]);
c.g[A0][0] = c.g[A0][0] | 0xd420;
c.g[A1][0] = 0x10000000;
c.sw((c.g[A0][0] as u32), c.g[S4][0]);
c.g[A1][0] = c.g[A1][0] | 0xd400;
c.g[V1][0] = 0x101;
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x10000000;
if t {
break 'b_4083c0;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x004083bc);
Callee { addr: 0x0042c0c0, f: ee::s15::EIntr }.call(c, 0x004083bc)?;
c.g[V1][0] = 0x10000000;
break 'b_4083c0;
}
}
c.cycles += 4;
c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S5][0] as u32)));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[FP][0] as u32)));
c.g[V1][0] = c.g[V1][0] | 0xd400;
'l_4083d0: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x004083d0); } continue 'l_4083d0;
} else {
'b_40843c: {
c.cycles += 17;
c.g[RA][0] = sx(0x004083f4);
Callee { addr: 0x0042c078, f: ee::s15::DIntr }.call(c, 0x004083f4)?;
c.g[V1][0] = 0x10000000;
c.g[A1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xd080;
c.g[A1][0] = c.g[A1][0] | 0xd010;
c.sw((c.g[V1][0] as u32), 0u64);
c.g[A0][0] = 0x10000000;
c.sw((c.g[A1][0] as u32), c.g[S1][0]);
c.g[A0][0] = c.g[A0][0] | 0xd020;
c.g[A1][0] = 0x10000000;
c.sw((c.g[A0][0] as u32), c.g[S4][0]);
c.g[A1][0] = c.g[A1][0] | 0xd000;
c.g[V1][0] = 0x100;
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x10000000;
if t {
break 'b_40843c;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x00408438);
Callee { addr: 0x0042c0c0, f: ee::s15::EIntr }.call(c, 0x00408438)?;
c.g[V1][0] = 0x10000000;
break 'b_40843c;
}
}
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
c.g[V1][0] = c.g[V1][0] | 0xd000;
'l_408448: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00408448); } continue 'l_408448;
} else {
c.cycles += 3;
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xd020;
'l_408470: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00408470); } continue 'l_408470;
} else {
c.cycles += 4;
c.g[S1][0] = c.g[S2][0];
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S2][0] = c.g[S3][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00408370); } continue 'l_408370;
} else {
c.cycles += 2;
let t = true;
c.g[A3][0] = c.lw((c.g[SP][0] as u32));
if t {
break 'b_4084ac;
} else {
break 'b_4084a4;
}
}
}
}
}
}
}
}
}
}
}
c.cycles += 2;
c.g[S7][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[A3][0] = c.lw((c.g[SP][0] as u32));
break 'b_4084ac;
}
c.cycles += 11;
c.g[A1][0] = c.g[S7][0];
c.g[V1][0] = 0xc0;
c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe4));
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.lo[0] = c.g[A3][0];
c.madd(0, 0, 2, 3);
c.g[A3][0] = c.lo[0];
let t = c.g[A0][0] != 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x00408360); } continue 'l_408360;
} else {
break 'b_4084d8;
}
}
}
}
c.cycles += 12;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t; return Ok(());
    });
}

/// `void libmpeg_ConvertPictureCsc(DecState* dec, Picture* pic)` (0x00409088):
/// converts a picture to RGB through the IPU colour-space converter. Tells
/// the callback chain (`DispatchCallback` command 2), clears an IPU error,
/// waits for the IPU, issues the `IpuCommand`, then feeds the source
/// `pic[0]` (`pic[3] * pic[4]` macroblocks, 0x18 quadwords each) through the
/// to-IPU DMA: in one go when it fits 0xffff quadwords, else in 0xffff-qwc
/// pieces with a DMAC handler (`AddDmacHandler`/`EnableDmac`, `DisableDmac`,
/// `RemoveDmacHandler`). Fewer than 1024 macroblocks use `IpuCsc`, more
/// `IpuCscDma`; finally the callback chain gets command 3. Converted from the
/// generated labelled-block body.
pub fn libmpeg_convert_picture_csc(c: &mut Cpu) {
    guest(c, |c| {
'b_4090f0: {
c.cycles += 24;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.g[A2][0] = 0x2;
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S3][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S2][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S0][0]);
c.g[S2][0] = c.g[A1][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S5][0]);
c.g[A1][0] = c.g[SP][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S4][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S1][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
c.mult(0, 19, 3, 2);
c.g[RA][0] = sx(0x004090d0);
c.sw((c.g[SP][0] as u32), c.g[A2][0]);
Callee { addr: 0x00402c30, f: ee::s15::libmpeg_DispatchCallback }.call(c, 0x004090d0)?;
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0x2010;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x4000;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x40000000;
if t {
break 'b_4090f0;
} else {
c.cycles += 2;
c.g[AT][0] = 0x10000000;
c.sw((c.g[AT][0] as u32).wrapping_add(0x2010), c.g[V0][0]);
break 'b_4090f0;
}
}
c.cycles += 4;
c.g[V1][0] = 0x10000000;
c.g[S5][0] = ((c.g[S3][0] as i64) < 1024i64) as u64;
c.g[V1][0] = c.g[V1][0] | 0x2010;
'l_409100: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
let t = (c.g[V0][0] as i64) < 0;

if t {
if event_due(c) { deliver_event_here(c, 0x00409100); } continue 'l_409100;
} else {
c.cycles += 7;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00409128);
c.g[A1][0] = 0u64;
Callee { addr: 0x00406df0, f: ee::s15::libmpeg_IpuCommand }.call(c, 0x00409128)?;
c.g[A0][0] = 0x10000000;
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
c.g[A0][0] = c.g[A0][0] | 0x2010;
'l_409138: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
let t = (c.g[V0][0] as i64) < 0;

if t {
if event_due(c) { deliver_event_here(c, 0x00409138); } continue 'l_409138;
} else {
'b_4092dc: {
'b_409260: {
c.cycles += 10;
c.g[V0][0] = 0x18;
c.g[S1][0] = 0xfff0000;
c.mult(0, 2, 19, 2);
c.g[S1][0] = c.g[S1][0] | 0xffff;
c.g[V1][0] = c.g[V1][0] & c.g[S1][0];
c.g[S4][0] = 0xffff;
c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
c.g[A0][0] = (c.g[S4][0] < c.g[V0][0]) as u64;
let t = c.g[A0][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
if t {
break 'b_409260;
} else {
'b_4091f8: {
c.cycles += 28;
c.g[A1][0] = 0x410000;
c.g[A0][0] = 0x4;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7050));
c.g[A2][0] = 0u64;
c.g[RA][0] = sx(0x00409194);
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
Callee { addr: 0x00423ac0, f: ee::s15::f_00423ac0 }.call(c, 0x00409194)?;
c.g[S2][0] = c.g[V0][0];
c.g[V1][0] = 0x10;
c.g[V0][0] = 0x10000000;
c.g[A0][0] = 0x4;
c.g[V0][0] = c.g[V0][0] | 0xe010;
c.g[RA][0] = sx(0x004091b0);
c.sw((c.g[V0][0] as u32), c.g[V1][0]);
Callee { addr: 0x004249b8, f: ee::s15::EnableDmac }.call(c, 0x004091b0)?;
c.g[RA][0] = sx(0x004091b8);
Callee { addr: 0x0042c078, f: ee::s15::DIntr }.call(c, 0x004091b8)?;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xb410;
c.g[A0][0] = 0x10000000;
c.sw((c.g[V1][0] as u32), c.g[A1][0]);
c.g[A0][0] = c.g[A0][0] | 0xb420;
c.sw((c.g[A0][0] as u32), c.g[S4][0]);
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xb400;
c.g[A0][0] = 0x101;
c.sw((c.g[V1][0] as u32), c.g[A0][0]);
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
if t {
break 'b_4091f8;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x004091f4);
Callee { addr: 0x0042c0c0, f: ee::s15::EIntr }.call(c, 0x004091f4)?;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
break 'b_4091f8;
}
}
'b_409244: {
'b_409234: {
c.cycles += 9;
c.g[V1][0] = 0xf0000;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.g[V1][0] | 0xfff0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[S4][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
let t = c.g[S5][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A0][0]);
if t {
break 'b_409234;
} else {
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
c.g[A2][0] = c.g[S3][0];
c.g[RA][0] = sx(0x0040922c);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00408b80, f: ee::s15::libmpeg_IpuCsc }.call(c, 0x0040922c)?;
let t = true;

if t {
break 'b_409244;
} else {
break 'b_409234;
}
}
}
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
c.g[A2][0] = c.g[S3][0];
c.g[RA][0] = sx(0x00409244);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00408df0, f: ee::s15::libmpeg_IpuCscDma }.call(c, 0x00409244)?;
break 'b_409244;
}
c.cycles += 7;
c.g[RA][0] = sx(0x0040924c);
c.g[A0][0] = 0x4;
Callee { addr: 0x00424950, f: ee::s15::DisableDmac }.call(c, 0x0040924c)?;
c.g[A1][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00409258);
c.g[A0][0] = 0x4;
Callee { addr: 0x00423ad0, f: ee::s15::RemoveDmacHandler }.call(c, 0x00409258)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
if t {
break 'b_4092dc;
} else {
break 'b_409260;
}
}
}
'b_4092a8: {
c.cycles += 16;
c.g[RA][0] = sx(0x00409268);
Callee { addr: 0x0042c078, f: ee::s15::DIntr }.call(c, 0x00409268)?;
c.g[A1][0] = c.lw((c.g[S2][0] as u32));
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xb410;
c.g[A2][0] = 0x10000000;
c.g[A1][0] = c.g[A1][0] & c.g[S1][0];
c.g[A2][0] = c.g[A2][0] | 0xb420;
c.sw((c.g[V1][0] as u32), c.g[A1][0]);
c.g[A0][0] = 0x10000000;
c.g[A0][0] = c.g[A0][0] | 0xb400;
c.g[A1][0] = 0x101;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[A2][0] as u32), c.g[V1][0]);
let t = c.g[V0][0] == 0u64;
c.sw((c.g[A0][0] as u32), c.g[A1][0]);
if t {
break 'b_4092a8;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x004092a8);
Callee { addr: 0x0042c0c0, f: ee::s15::EIntr }.call(c, 0x004092a8)?;
break 'b_4092a8;
}
}
'b_4092c8: {
c.cycles += 2;
let t = c.g[S5][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
if t {
break 'b_4092c8;
} else {
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
c.g[A2][0] = c.g[S3][0];
c.g[RA][0] = sx(0x004092c0);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00408b80, f: ee::s15::libmpeg_IpuCsc }.call(c, 0x004092c0)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
if t {
break 'b_4092dc;
} else {
break 'b_4092c8;
}
}
}
c.cycles += 5;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xd8));
c.g[A2][0] = c.g[S3][0];
c.g[RA][0] = sx(0x004092d8);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x00408df0, f: ee::s15::libmpeg_IpuCscDma }.call(c, 0x004092d8)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x858));
break 'b_4092dc;
}
c.cycles += 13;
c.g[V0][0] = 0x3;
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[RA][0] = sx(0x004092ec);
c.g[A1][0] = c.g[SP][0];
Callee { addr: 0x00402c30, f: ee::s15::libmpeg_DispatchCallback }.call(c, 0x004092ec)?;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
}
}
}
}
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (in the machine's RAM): with it
    /// unset every fake runs the body it replaced.
    pub(crate) const OC1120_CTRL: u32 = DATA + 0xf_9000;

    pub(crate) const OC1120_MAGIC: u32 = 0x1120_c0de;

    /// Mailbox of fake `k`: call count, 8 results, 8 side stores, two outputs.
    pub(crate) const OC1120_MB: u32 = OC1120_CTRL + 0x100;

    pub(crate) const OC1120_MB_SIZE: u32 = 0x80;

    /// The decoder state (`dec`) and the picture record of the fixtures.
    pub(crate) const OC1120_DEC: u32 = DATA + 0x2000;

    pub(crate) const OC1120_REC: u32 = DATA + 0x6000;

    /// Source and destination pictures of the DMA fixtures.
    pub(crate) const OC1120_SRC: u32 = DATA + 0x1_0000;

    pub(crate) const OC1120_DST: u32 = DATA + 0x2_0000;

    /// Output areas of `DecodeMacroblock` (mbtype, motion type, dct type, pmv,
    /// field select, dm vector).
    pub(crate) const OC1120_OUT: u32 = DATA + 0x8000;

    /// `dec[0x11c]`: the error flag the callees may set.
    pub(crate) const OC1120_ERR: u32 = 0x11c;

    /// The index of each fake's mailbox.
    pub(crate) const K_SLICE_HEADER: u32 = 0;
    pub(crate) const K_WAIT_BDEC: u32 = 1;
    pub(crate) const K_SHOW_BITS: u32 = 2;
    pub(crate) const K_MB_INCREMENT: u32 = 3;
    pub(crate) const K_ERRORF: u32 = 4;
    pub(crate) const K_DECODE_MB: u32 = 5;
    pub(crate) const K_SKIPPED_MB: u32 = 6;
    pub(crate) const K_SETUP_MB: u32 = 7;
    pub(crate) const K_ADD_PRED: u32 = 8;
    pub(crate) const K_GET_BITS: u32 = 9;
    pub(crate) const K_MV1: u32 = 10;
    pub(crate) const K_MV2: u32 = 11;
    pub(crate) const K_CLEAR: u32 = 12;
    pub(crate) const K_71F8: u32 = 13;
    pub(crate) const K_MB_TYPE: u32 = 14;
    pub(crate) const K_BLOCK_WAIT: u32 = 15;
    pub(crate) const K_IPU_COMMAND: u32 = 16;
    pub(crate) const K_DINTR: u32 = 17;
    pub(crate) const K_EINTR: u32 = 18;
    pub(crate) const K_CSC_DMA: u32 = 19;
    pub(crate) const K_CSC: u32 = 20;
    pub(crate) const K_DISPATCH: u32 = 21;
    pub(crate) const K_ENABLE_DMAC: u32 = 22;
    pub(crate) const K_DISABLE_DMAC: u32 = 23;
    pub(crate) const K_REMOVE_HANDLER: u32 = 24;
    pub(crate) const K_ADD_HANDLER: u32 = 25;

    /// A callee fake: logs the call (every register, cycles, target), then
    /// takes its result from the mailbox's next entry (zero-extended like the
    /// plain fakes), optionally stores the entry's side word minus one to
    /// `a0[0x11c]` (0: no store) and, for `SliceHeader`, the two output words
    /// through `a2` and `a3`.
    pub(crate) fn oc1120_fake(c: &mut Cpu, addr: u32, k: u32, side: bool, outs: bool) {
        let a0 = c.g[A0][0] as u32;
        let a2 = c.g[A2][0] as u32;
        let a3 = c.g[A3][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let mb = OC1120_MB + OC1120_MB_SIZE * k;
        let n = c.bus.read32(mb);
        c.bus.write32(mb, n + 1);
        let i = 4 * (n % 8);
        let result = c.bus.read32(mb + 4 + i);
        c.g[V0][0] = result as u64;
        if side {
            let s = c.bus.read32(mb + 0x24 + i);
            if s != 0 {
                c.bus.write32(a0 + OC1120_ERR, s - 1);
            }
        }
        if outs {
            let (o0, o1) = (c.bus.read32(mb + 0x44), c.bus.read32(mb + 0x48));
            c.bus.write32(a2, o0);
            c.bus.write32(a3, o1);
        }
    }

    macro_rules! oc1120_fakes {
        ($(($fake:ident, $addr:expr, $slot:ident, $gen:ident, $k:expr, $side:expr, $outs:expr)),* $(,)?) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1120_CTRL) != OC1120_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::s15::$gen);
                    }
                    oc1120_fake(c, $addr, $k, $side, $outs)
                }
            )*

            /// Put every fake in its override slot (each once per process).
            pub(crate) fn oc1120_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::s15::$slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc1120_fakes!(
        (oc1120_fake_slice_header, 0x004061e8, OVERRIDE_004061E8, f_004061e8__generated, K_SLICE_HEADER, false, true),
        (oc1120_fake_wait_bdec, 0x00405b70, OVERRIDE_00405B70, f_00405b70__generated, K_WAIT_BDEC, false, false),
        (oc1120_fake_show_bits, 0x004070f0, OVERRIDE_004070F0, f_004070f0__generated, K_SHOW_BITS, true, false),
        (oc1120_fake_mb_increment, 0x00405fb8, OVERRIDE_00405FB8, f_00405fb8__generated, K_MB_INCREMENT, true, false),
        (oc1120_fake_errorf, 0x004034a8, OVERRIDE_004034A8, f_004034a8__generated, K_ERRORF, false, false),
        (oc1120_fake_decode_mb, 0x004065d0, OVERRIDE_004065D0, f_004065d0__generated, K_DECODE_MB, false, false),
        (oc1120_fake_skipped_mb, 0x00406510, OVERRIDE_00406510, f_00406510__generated, K_SKIPPED_MB, false, false),
        (oc1120_fake_setup_mb, 0x00403e00, OVERRIDE_00403E00, f_00403e00__generated, K_SETUP_MB, false, false),
        (oc1120_fake_add_pred, 0x00404b78, OVERRIDE_00404B78, f_00404b78__generated, K_ADD_PRED, false, false),
        (oc1120_fake_get_bits, 0x004072e8, OVERRIDE_004072E8, f_004072e8__generated, K_GET_BITS, false, false),
        (oc1120_fake_mv1, 0x00406ca0, OVERRIDE_00406CA0, f_00406ca0__generated, K_MV1, true, false),
        (oc1120_fake_mv2, 0x00406b00, OVERRIDE_00406B00, f_00406b00__generated, K_MV2, true, false),
        (oc1120_fake_clear, 0x00408af8, OVERRIDE_00408AF8, f_00408af8__generated, K_CLEAR, false, false),
        (oc1120_fake_71f8, 0x004071f8, OVERRIDE_004071F8, f_004071f8__generated, K_71F8, false, false),
        (oc1120_fake_mb_type, 0x00406f78, OVERRIDE_00406F78, f_00406f78__generated, K_MB_TYPE, false, false),
        (oc1120_fake_block_wait, 0x00406e20, OVERRIDE_00406E20, f_00406e20__generated, K_BLOCK_WAIT, true, false),
        (oc1120_fake_ipu_command, 0x00406df0, OVERRIDE_00406DF0, f_00406df0__generated, K_IPU_COMMAND, false, false),
        (oc1120_fake_dintr, 0x0042c078, OVERRIDE_0042C078, f_0042c078__generated, K_DINTR, false, false),
        (oc1120_fake_eintr, 0x0042c0c0, OVERRIDE_0042C0C0, f_0042c0c0__generated, K_EINTR, false, false),
        (oc1120_fake_csc_dma, 0x00408df0, OVERRIDE_00408DF0, f_00408df0__generated, K_CSC_DMA, false, false),
        (oc1120_fake_csc, 0x00408b80, OVERRIDE_00408B80, f_00408b80__generated, K_CSC, false, false),
        (oc1120_fake_dispatch, 0x00402c30, OVERRIDE_00402C30, f_00402c30__generated, K_DISPATCH, false, false),
        (oc1120_fake_enable_dmac, 0x004249b8, OVERRIDE_004249B8, f_004249b8__generated, K_ENABLE_DMAC, false, false),
        (oc1120_fake_disable_dmac, 0x00424950, OVERRIDE_00424950, f_00424950__generated, K_DISABLE_DMAC, false, false),
        (oc1120_fake_remove_handler, 0x00423ad0, OVERRIDE_00423AD0, f_00423ad0__generated, K_REMOVE_HANDLER, false, false),
        (oc1120_fake_add_handler, 0x00423ac0, OVERRIDE_00423AC0, f_00423ac0__generated, K_ADD_HANDLER, false, false),
    );

    /// A machine with the fakes on and every mailbox cleared.
    pub(crate) fn oc1120_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1120_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1120_CTRL, OC1120_MAGIC);
        for k in 0..26 {
            for w in 0..0x20 {
                c.bus.write32(OC1120_MB + OC1120_MB_SIZE * k + 4 * w, 0);
            }
        }
        r
    }

    /// Fill fake `k`'s eight (result, side) entries from `f(i)`.
    pub(crate) fn oc1120_mb(c: &mut Cpu, k: u32, mut f: impl FnMut(u32) -> (u32, u32)) {
        let mb = OC1120_MB + OC1120_MB_SIZE * k;
        for i in 0..8 {
            let (result, side) = f(i);
            c.bus.write32(mb + 4 + 4 * i, result);
            c.bus.write32(mb + 0x24 + 4 * i, side);
        }
    }

    /// The two output words `SliceHeader` stores (mbAddress, mbIncrement).
    pub(crate) fn oc1120_mb_outs(c: &mut Cpu, k: u32, o0: u32, o1: u32) {
        let mb = OC1120_MB + OC1120_MB_SIZE * k;
        c.bus.write32(mb + 0x44, o0);
        c.bus.write32(mb + 0x48, o1);
    }

    /// `true` for about `num` in `den` draws.
    pub(crate) fn oc1120_chance(r: &mut Rng, num: u32, den: u32) -> bool {
        r.next_u32() % den < num
    }

    /// How many different cycle totals the generated body reaches over the
    /// seeds (each total is a different path through the function).
    pub(crate) fn oc1120_paths(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    // ------------------------------------------------------------------
    // libmpeg_DecodeSlice

    /// `SliceHeader` returns 0 (rarely an error) and stores the first
    /// macroblock address (0..n+1) and increment (0..3); then each macroblock
    /// draws its callee results (success 4 in 5), `ShowBits` and
    /// `GetMbIncrement` may flag an error in `dec[0x11c]`.
    pub(crate) fn oc1120_slice_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1120_machine(c, seed, 0x406318, 14);
        fill(c, &mut r, OC1120_DEC, 0x440);
        let n = 1 + seed % 5;
        c.bus.write32(OC1120_DEC + 0x810, seed / 5 % 2);
        let abort = if seed / 10 % 3 == 0 { 1 + r.next_u32() % 9 } else { 0 };
        c.bus.write32(OC1120_DEC + 0x10a0, abort);
        oc1120_mb(c, K_SLICE_HEADER, |_| {
            (if oc1120_chance(&mut r, 1, 6) { 1 + r.next_u32() % 4 } else { 0 }, 0)
        });
        let (a, inc) = (r.next_u32() % (n + 2), r.next_u32() % 4);
        oc1120_mb_outs(c, K_SLICE_HEADER, a, inc);
        oc1120_mb(c, K_WAIT_BDEC, |_| (oc1120_chance(&mut r, 4, 5) as u32, 0));
        oc1120_mb(c, K_SHOW_BITS, |_| {
            let side = [0, 0, 0, 1, 2][(r.next_u32() % 5) as usize];
            (oc1120_chance(&mut r, 4, 5) as u32, side)
        });
        oc1120_mb(c, K_MB_INCREMENT, |_| {
            let side = [0, 0, 0, 1, 2][(r.next_u32() % 5) as usize];
            ([0, 1, 1, 2, 3][(r.next_u32() % 5) as usize], side)
        });
        oc1120_mb(c, K_DECODE_MB, |_| (oc1120_chance(&mut r, 5, 6) as u32, 0));
        oc1120_mb(c, K_SKIPPED_MB, |_| (oc1120_chance(&mut r, 5, 6) as u32, 0));
        oc1120_mb(c, K_SETUP_MB, |_| (oc1120_chance(&mut r, 5, 6) as u32, 0));
        c.g[A0][0] = OC1120_DEC as u64;
        c.g[A1][0] = n as u64;
        c.pc = 0x0040_6318;
    }

    /// Slice-header error, the loop ending by count, a failed `WaitBdec`
    /// (with and without the abort flag), `ShowBits` zero or flagged, the
    /// increment read (and its flag), a macroblock past the end, coded and
    /// skipped macroblocks failing or not, setup failing, and the
    /// `AddPredAndStore` call from the second macroblock on.
    #[test]
    fn libmpeg_decode_slice_matches() {
        oc1120_install_fakes();
        check_lohi(
            ee::s15::f_00406318__generated,
            libmpeg_decode_slice,
            600,
            &oc1120_slice_setup,
        );
        let paths = oc1120_paths(ee::s15::f_00406318__generated, 600, &oc1120_slice_setup);
        assert!(paths >= 10, "only {paths} paths");
    }

    // ------------------------------------------------------------------
    // libmpeg_DecodeMacroblock

    pub(crate) fn oc1120_mb_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1120_machine(c, seed, 0x4065d0, 14);
        fill(c, &mut r, OC1120_DEC, 0x440);
        fill(c, &mut r, OC1120_OUT, 0x100);
        let d = OC1120_DEC;
        let ptype = 1 + seed % 3;
        c.bus.write32(d + 0x150, ptype);
        c.bus.write32(d + 0x174, 1 + seed / 3 % 3);
        c.bus.write32(d + 0x17c, seed / 9 % 2);
        c.bus.write32(d + 0x180, seed / 18 % 2);
        c.bus.write32(d + 0x848, seed / 36 % 2);
        c.bus.write32(d + 0x1b0, seed / 72 % 2);
        c.bus.write32(d + 0x810, seed / 144 % 2);
        c.bus.write32(d + OC1120_ERR, 0);
        for off in [0x154u32, 0x158, 0x15c, 0x160, 0x164, 0x168, 0x16c, 0x170] {
            c.bus.write32(d + off, 1 + r.next_u32() % 9);
        }
        oc1120_mb(c, K_MB_TYPE, |_| {
            let t = if oc1120_chance(&mut r, 1, 8) { 0 } else { r.next_u32() % 0x20 };
            (t, 0)
        });
        oc1120_mb(c, K_GET_BITS, |i| ([r.next_u32() % 4, r.next_u32() % 2, r.next_u32() % 32][(i % 3) as usize], 0));
        for k in [K_MV1, K_MV2, K_BLOCK_WAIT] {
            oc1120_mb(c, k, |_| (r.next_u32(), if oc1120_chance(&mut r, 1, 6) { 2 } else { 0 }));
        }
        c.g[A0][0] = d as u64;
        c.g[A1][0] = (OC1120_OUT) as u64;
        c.g[A2][0] = (OC1120_OUT + 0x10) as u64;
        c.g[A3][0] = (OC1120_OUT + 0x20) as u64;
        c.g[T0][0] = (OC1120_OUT + 0x40) as u64;
        c.g[T1][0] = (OC1120_OUT + 0x80) as u64;
        c.g[T2][0] = (OC1120_OUT + 0xa0) as u64;
    }

    /// Every macroblock type (0 reports the error), the three picture
    /// structures, both MV readers, the motion-type / DCT / quantiser reads,
    /// intra and inter blocks, the flags set by the readers' side stores, and
    /// the predictor resets (the `ptype == 2` and non-intra-no-motion cases).
    #[test]
    fn libmpeg_decode_macroblock_matches() {
        oc1120_install_fakes();
        check_lohi(
            ee::s15::f_004065d0__generated,
            libmpeg_decode_macroblock,
            1200,
            &oc1120_mb_setup,
        );
        let paths = oc1120_paths(ee::s15::f_004065d0__generated, 1200, &oc1120_mb_setup);
        assert!(paths >= 12, "only {paths} paths");
    }

    // ------------------------------------------------------------------
    // libmpeg_CopyOutputRaw

    pub(crate) fn oc1120_copy_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1120_machine(c, seed, 0x408278, 0);
        fill(c, &mut r, OC1120_SRC, 0x800);
        fill(c, &mut r, OC1120_DST, 0x800);
        let d = OC1120_DEC;
        c.bus.write32(d + 0xd8, OC1120_DST + 16 * (seed % 4));
        c.bus.write32(d + 0x174, 1 + seed / 4 % 3);
        c.bus.write32(d + 0xe0, [0u32, 16, 32, 48][(seed / 12 % 4) as usize]);
        c.bus.write32(d + 0xe4, seed / 48 % 4);
        let rec = OC1120_REC;
        c.bus.write32(rec, OC1120_SRC + 16 * (seed % 3));
        c.bus.write32(rec + 0xc, seed / 3 % 4);
        c.bus.write32(rec + 0x10, [1u32, 2, 3, 4][(seed / 5 % 4) as usize]);
        oc1120_mb(c, K_DINTR, |_| (oc1120_chance(&mut r, 1, 2) as u32, 0));
        c.g[A0][0] = d as u64;
        c.g[A1][0] = rec as u64;
    }

    /// One and two fields (`dec[0x174] == 3` or `dec[0xe0] == 0` means one),
    /// with and without a destination stride, 0 to 3 rows, interrupts
    /// enabled or not when masked.
    #[test]
    fn libmpeg_copy_output_raw_matches() {
        oc1120_install_fakes();
        check_lohi(
            ee::s15::f_00408278__generated,
            libmpeg_copy_output_raw,
            384,
            &oc1120_copy_setup,
        );
        let paths = oc1120_paths(ee::s15::f_00408278__generated, 384, &oc1120_copy_setup);
        assert!(paths >= 6, "only {paths} paths");
    }

    // ------------------------------------------------------------------
    // libmpeg_ConvertPictureCsc

    pub(crate) fn oc1120_csc_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1120_machine(c, seed, 0x409088, 14);
        fill(c, &mut r, OC1120_DEC, 0x440);
        let rec = OC1120_REC;
        let count = [5u32, 1023, 1024, 2730, 2731, 5000][(seed % 6) as usize];
        c.bus.write32(rec, OC1120_SRC + 16 * (seed / 6 % 4));
        c.bus.write32(rec + 0xc, count);
        c.bus.write32(rec + 0x10, 1);
        if seed / 12 % 2 == 1 {
            // An invalid VDEC command leaves ECD set in IPU_CTRL.
            c.bus.write32(0x1000_2000, 0x3400_0000);
        }
        let intr = seed / 24 % 2;
        oc1120_mb(c, K_DINTR, |_| (intr, 0));
        oc1120_mb(c, K_ADD_HANDLER, |_| (1 + r.next_u32() % 100, 0));
        c.g[A0][0] = OC1120_DEC as u64;
        c.g[A1][0] = rec as u64;
    }

    /// Fewer and more than 1023 macroblocks (`IpuCsc` / `IpuCscDma`), a
    /// transfer within 0xffff quadwords and one split with a DMAC handler,
    /// interrupts enabled or not, an IPU error cleared first.
    #[test]
    fn libmpeg_convert_picture_csc_matches() {
        oc1120_install_fakes();
        check_lohi(
            ee::s15::f_00409088__generated,
            libmpeg_convert_picture_csc,
            288,
            &oc1120_csc_setup,
        );
        let paths = oc1120_paths(ee::s15::f_00409088__generated, 288, &oc1120_csc_setup);
        assert!(paths >= 6, "only {paths} paths");
    }
}
