//! Staging module oc760 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc760`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002e_c9d0,
        bolt_on_fx_interface_init_handle,
        "cBoltOnFXInterface_InitHandle",
    ),
    (
        0x0019_ec68,
        fe_module_rider_preview_release_bolt_files,
        "FEModule_RiderPreviewReleaseBoltFiles",
    ),
    (0x003d_1d28, path_service_queue, "PATH_ServiceQueue"),
    (0x0039_1788, font_parse_glyphs, "Font_ParseGlyphs"),
    (
        0x002a_fd40,
        speech_instance_load_events,
        "SpeechInstance_LoadEvents",
    ),
];

// Register-based memory helpers (base register + offset, as the compiled
// code addresses memory).
#[inline]
fn o760_ea(c: &Cpu, base: usize, off: u32) -> u32 {
    (c.g[base][0] as u32).wrapping_add(off)
}
#[inline]
fn o760_lw(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = o760_ea(c, base, off);
    Addr(a).lw(c)
}
#[inline]
fn o760_lh(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = o760_ea(c, base, off);
    Addr(a).lh(c)
}
#[inline]
fn o760_lhu(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = o760_ea(c, base, off);
    Addr(a).lhu(c)
}
#[inline]
fn o760_lb(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = o760_ea(c, base, off);
    Addr(a).lb(c)
}
#[inline]
fn o760_lbu(c: &mut Cpu, base: usize, off: u32) -> u64 {
    let a = o760_ea(c, base, off);
    Addr(a).lbu(c)
}
/// `sw val, off(base)` with the value taken from a register.
#[inline]
fn o760_sw(c: &mut Cpu, base: usize, off: u32, val: usize) {
    let a = o760_ea(c, base, off);
    let v = c.g[val][0];
    Addr(a).sw(c, v);
}
/// `sw $zero, off(base)`.
#[inline]
fn o760_sw0(c: &mut Cpu, base: usize, off: u32) {
    let a = o760_ea(c, base, off);
    Addr(a).sw(c, 0);
}
#[inline]
fn o760_sh(c: &mut Cpu, base: usize, off: u32, val: usize) {
    let a = o760_ea(c, base, off);
    let v = c.g[val][0];
    Addr(a).sh(c, v);
}
#[inline]
fn o760_sb(c: &mut Cpu, base: usize, off: u32, val: usize) {
    let a = o760_ea(c, base, off);
    let v = c.g[val][0];
    Addr(a).sb(c, v);
}
#[inline]
fn o760_sb0(c: &mut Cpu, base: usize, off: u32) {
    let a = o760_ea(c, base, off);
    Addr(a).sb(c, 0);
}
/// `addu`: 32-bit sum of two registers, sign-extended.
#[inline]
fn o760_add(c: &Cpu, a: usize, b: usize) -> u64 {
    sx((c.g[a][0] as u32).wrapping_add(c.g[b][0] as u32))
}
/// `addiu`: register plus immediate (given as the wrapped 32-bit value).
#[inline]
fn o760_addi(c: &Cpu, a: usize, imm: u32) -> u64 {
    sx((c.g[a][0] as u32).wrapping_add(imm))
}
/// `sll`: shift left, sign-extended.
#[inline]
fn o760_shl(c: &Cpu, a: usize, n: u32) -> u64 {
    sx((c.g[a][0] as u32) << n)
}
/// `srl`: logical shift right, sign-extended.
#[inline]
fn o760_shr(c: &Cpu, a: usize, n: u32) -> u64 {
    sx((c.g[a][0] as u32) >> n)
}

/// `void cBoltOnFXInterface_InitHandle(int* handle)` (0x2ec9d0): marks a
/// bolt-on FX handle as unused (`*handle = -1`) and returns the pointer.
pub fn bolt_on_fx_interface_init_handle(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    o760_sw(c, V0, 0, V1);
    ret(c);
}

/// `void FEModule_RiderPreviewReleaseBoltFiles(FEModule* self)` (0x19ec68):
/// when the preview rider has a bolt file (`self+4` = rider index, `self+0`
/// = bolt file id, both >= 0), release every texture and model of the rider's
/// bolt-file index entry from the BoltFile and mark the slots free (-1).
pub fn fe_module_rider_preview_release_bolt_files(c: &mut Cpu) {
    guest(c, |c| {
        'out: {
            c.cycles += 10;
            let frame = Frame::open(c, 0x50);
            frame.save(c, 16, 0x40);
            frame.save(c, 17, 0x30);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, 18, 0x20);
            frame.save(c, 19, 0x10);
            frame.save_ra(c, 0);
            c.g[V0][0] = o760_lw(c, S0, 0x4);
            if (c.g[V0][0] as i64) < 0 {
                frame.restore(c, 16, 0x40);
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = o760_lw(c, S0, 0);
            if (c.g[V0][0] as i64) < 0 {
                frame.restore(c, 16, 0x40);
                break 'out;
            }
            'released: {
                c.cycles += 15;
                call_ptr(c, 0x0014_dc80, 0x0019_eca4)?; // cBE_getBE
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x9;
                call_ptr(c, 0x0014_dd58, 0x0019_ecb0)?;
                c.g[V1][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x848));
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = o760_lw(c, S0, 0x4);
                c.g[V0][0] = o760_lw(c, V1, 0x7c);
                c.g[A2][0] = o760_lw(c, S0, 0);
                c.g[S3][0] = o760_addi(c, V0, 0x1a70); // the BoltFile
                call_ptr(c, 0x0014_ad28, 0x0019_eccc)?;
                c.g[V1][0] = o760_lw(c, V0, 0x28c); // entry count
                let empty = (c.g[V1][0] as i64) <= 0;
                c.g[S1][0] = o760_addi(c, V0, 0x290); // entries, 4 bytes each
                if empty {
                    break 'released;
                }
                c.cycles += 2;
                c.g[S2][0] = c.g[V1][0];
                loop {
                    // Texture slot of this entry.
                    c.cycles += 6;
                    c.g[A0][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x1788));
                    c.g[A1][0] = o760_lw(c, S0, 0x4);
                    c.g[A2][0] = o760_lh(c, S1, 0);
                    Callee {
                        addr: 0x0019_cce8,
                        f: ee::s03::BoltFileIndexManager_GetTextureSlot,
                    }
                    .call(c, 0x0019_ecf0)?;
                    let none = (c.g[V0][0] as i64) < 0;
                    c.g[A2][0] = c.g[V0][0];
                    if !none {
                        c.cycles += 8;
                        c.g[A1][0] = o760_lw(c, S0, 0);
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x0019_dc20,
                            f: ee::s03::BoltFile_Release,
                        }
                        .call(c, 0x0019_ed04)?;
                        c.g[A0][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x1788));
                        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
                        c.g[A1][0] = o760_lw(c, S0, 0x4);
                        c.g[A2][0] = o760_lh(c, S1, 0);
                        Callee {
                            addr: 0x0019_cd38,
                            f: ee::s03::BoltFileIndexManager_SetTextureSlot,
                        }
                        .call(c, 0x0019_ed18)?;
                    }
                    // Model slot of this entry.
                    c.cycles += 6;
                    c.g[A0][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x1788));
                    c.g[A1][0] = o760_lw(c, S0, 0x4);
                    c.g[A2][0] = o760_lh(c, S1, 0);
                    Callee {
                        addr: 0x0019_cbe0,
                        f: ee::s03::BoltFileIndexManager_GetModelSlot,
                    }
                    .call(c, 0x0019_ed28)?;
                    if (c.g[V0][0] as i64) < 0 {
                        c.g[S2][0] = o760_addi(c, S2, 0u32.wrapping_sub(1));
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = o760_lw(c, S0, 0);
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x0019_dc20,
                            f: ee::s03::BoltFile_Release,
                        }
                        .call(c, 0x0019_ed40)?;
                        c.g[A0][0] = o760_lw(c, GP, 0u32.wrapping_sub(0x1788));
                        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
                        c.g[A1][0] = o760_lw(c, S0, 0x4);
                        c.g[A2][0] = o760_lh(c, S1, 0);
                        Callee {
                            addr: 0x0019_cc30,
                            f: ee::s03::BoltFileIndexManager_SetModelSlot,
                        }
                        .call(c, 0x0019_ed54)?;
                        c.g[S2][0] = o760_addi(c, S2, 0u32.wrapping_sub(1));
                    }
                    c.cycles += 2;
                    let more = c.g[S2][0] != 0;
                    c.g[S1][0] = o760_addi(c, S1, 0x4);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_ece0);
                        }
                        continue;
                    }
                    break 'released;
                }
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x40);
        }
        // Epilogue.
        let frame = Frame { size: 0x50 };
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_ServiceQueue(int queue)` (0x3d1d28): runs the pending actions of
/// path queue `queue` (`queue_table[queue] + 0xdc`). The queue state word at
/// `+0xc` holds the head (bits 22..27) and tail count (bits 16..21); each entry
/// (16 bytes from `+0x10`) has a due time (`+4`), flags (`+8`, bit 26 = due)
/// and an action executed with `PATH_ExecuteAction`. Returns how many entries
/// were consumed (or the count so far when an action asks to stop).
pub fn path_service_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[V1][0] = 0x450000;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(0x0045_0000u32.wrapping_sub(0xbe0)).lw(c); // queue table
        c.g[A0][0] = o760_shl(c, A0, 2);
        frame.save(c, 18, 0x50);
        frame.save(c, 22, 0x10);
        c.g[V0][0] = o760_add(c, V0, A0);
        frame.save(c, 16, 0x70);
        frame.save(c, 19, 0x40);
        frame.save(c, 20, 0x30);
        frame.save(c, 21, 0x20);
        frame.save_ra(c, 0);
        frame.save(c, 17, 0x60);
        c.g[S1][0] = o760_lw(c, V0, 0xdc); // queue state
        c.g[V1][0] = o760_lw(c, S1, 0xc);
        c.g[V0][0] = o760_shr(c, V1, 16);
        c.g[S2][0] = o760_shr(c, V1, 22);
        c.g[V0][0] &= 0x3f;
        c.g[S2][0] &= 0x3f;
        c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
        let nothing_queued = c.g[V0][0] == 0;
        c.g[S6][0] = 0;
        'epilogue: {
            'drained: {
                if nothing_queued {
                    break 'drained;
                }
                c.cycles += 8;
                c.g[V0][0] = o760_shl(c, S2, 4);
                c.g[S3][0] = 0xffff_ffff_f03f_0000;
                c.g[V0][0] = o760_addi(c, V0, 0x10);
                c.g[S5][0] = 0x450000;
                c.g[S0][0] = o760_add(c, V0, S1); // entry
                c.g[S4][0] = 0x400_0000; // "due" flag
                c.g[S3][0] |= 0xffff;
                loop {
                    'flags: {
                        c.cycles += 3;
                        c.g[V1][0] = o760_lw(c, S0, 0x4); // due time
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = o760_lw(c, S0, 0x8);
                            break 'flags;
                        }
                        'checked: {
                            c.cycles += 4;
                            c.g[V0][0] = o760_lw(c, S1, 0);
                            c.g[V0][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                            let t = c.g[V0][0] == 0;
                            c.g[A0][0] = 0x450000;
                            if t {
                                break 'checked;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = o760_lw(c, S5, 0u32.wrapping_sub(0xbc8)); // now
                            c.g[V0][0] = o760_add(c, V1, V0);
                            o760_sw(c, S0, 0x4, V0);
                        }
                        c.cycles += 5;
                        c.g[V1][0] = o760_lw(c, A0, 0u32.wrapping_sub(0xbc8));
                        c.g[V0][0] = o760_lw(c, S0, 0x4);
                        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = o760_lw(c, S0, 0x8);
                        if t {
                            break 'flags;
                        }
                        c.cycles += 3;
                        c.g[V0][0] |= c.g[S4][0];
                        o760_sw(c, S0, 0x8, V0);
                        c.g[V0][0] = o760_lw(c, S0, 0x8);
                    }
                    'ran: {
                        c.cycles += 3;
                        c.g[V0][0] &= c.g[S4][0];
                        if c.g[V0][0] != 0 {
                            c.g[A0][0] = o760_lw(c, S1, 0xc);
                            break 'ran;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        Callee {
                            addr: 0x003c_fc30,
                            f: ee::s14::PATH_ExecuteAction,
                        }
                        .call(c, 0x003d_1e00)?;
                        let stop = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[S6][0];
                        if stop {
                            break 'epilogue;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = o760_lw(c, S1, 0xc);
                    }
                    c.cycles += 17;
                    c.g[S2][0] = o760_addi(c, S2, 1);
                    c.g[V1][0] = o760_lw(c, S5, 0u32.wrapping_sub(0xbc8));
                    c.g[S6][0] = o760_addi(c, S6, 1);
                    c.g[V0][0] = o760_shr(c, A0, 22);
                    c.g[S0][0] = o760_addi(c, S0, 0x10);
                    c.g[V0][0] &= 0x3f;
                    c.g[A0][0] &= c.g[S3][0];
                    c.g[V0][0] = o760_addi(c, V0, 1);
                    o760_sw(c, S1, 0x8, V1);
                    c.g[V0][0] &= 0x3f;
                    c.g[V0][0] = o760_shl(c, V0, 22);
                    c.g[A0][0] |= c.g[V0][0];
                    c.g[V1][0] = o760_shr(c, A0, 16);
                    c.g[V1][0] &= 0x3f;
                    c.g[V1][0] = (c.g[S2][0] < c.g[V1][0]) as u64;
                    let more = c.g[V1][0] != 0;
                    o760_sw(c, S1, 0xc, A0);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_1da0);
                        }
                        continue;
                    }
                    break 'drained;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S6][0];
        }
        c.cycles += 10;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Font_ParseGlyphs(Font* font, const FontFile* file)` (0x391788,
/// reference `cFont_linkFont`): builds the font's glyph table from a font
/// file. Allocates `count` 12-byte glyph records (`font+4`), copies the 11
/// meaningful bytes of each record from the file, extends the run of
/// consecutive codes `font+0x10`, points `font+8` at the glyph of code 0xac,
/// remembers the last glyph of code 0x20 (space) and gives a glyph of code
/// 0xa0 (no-break space) that space's metrics, tracks the tallest glyph
/// extent (`font+0x14`), then uploads the font texture.
pub fn font_parse_glyphs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = 0xc;
        c.g[A1][0] = 0x4a0000;
        c.g[V1][0] = o760_lb(c, S1, 0x10);
        c.g[A1][0] = o760_addi(c, A1, 0x46f8);
        c.g[A3][0] = o760_lb(c, S1, 0x11);
        c.mtc1(3, 0);
        c.cvt_s_w(0, 0);
        c.g[V0][0] = o760_lw(c, S1, 0x14);
        c.mtc1(7, 1);
        c.cvt_s_w(1, 1);
        c.g[S2][0] = o760_add(c, S1, V0); // the file's glyph records
        c.g[A3][0] = 0;
        let a = frame.at(c, 0);
        c.swc1(0, a);
        let a = frame.at(c, 4);
        c.swc1(1, a);
        let a = frame.at(c, 7);
        c.ldl(2, a);
        let a = frame.at(c, 0);
        c.ldr(2, a);
        let a = o760_ea(c, S0, 0x1f);
        c.sdl(2, a);
        let a = o760_ea(c, S0, 0x18);
        c.sdr(2, a);
        c.g[V0][0] = o760_lh(c, S1, 0xa); // glyph count
        c.mult(0, 0, 2, 4);
        c.g[A0][0] = c.lo[0];
        o760_sw(c, S0, 0, V0);
        call_ptr(c, 0x0031_7e30, 0x0039_1800)?; // alloc(count * 12)
        o760_sw(c, S0, 0x4, V0);
        c.g[T2][0] = 0;
        o760_sw0(c, S0, 0x14);
        c.g[T1][0] = 0;
        c.g[V1][0] = o760_lw(c, S0, 0);
        c.g[V0][0] = o760_lhu(c, S2, 0);
        o760_sw(c, S0, 0x10, V0);
        let no_glyphs = (c.g[V1][0] as i64) <= 0;
        o760_sw(c, S0, 0xc, V0);
        'glyphs: {
            if no_glyphs {
                break 'glyphs;
            }
            c.cycles += 5;
            c.g[T4][0] = 0xac;
            c.g[T3][0] = 0xa0;
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = 0;
            loop {
                // Copy the 11 bytes of glyph t1.
                'copy: {
                    c.cycles += 33;
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A3, 0);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    o760_sh(c, V0, 0, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A3, 0x2);
                    c.g[V1][0] = o760_add(c, T0, V1);
                    o760_sb(c, V1, 0x2, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A3, 0x3);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    o760_sb(c, V0, 0x3, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A3, 0x4);
                    c.g[V1][0] = o760_add(c, T0, V1);
                    o760_sh(c, V1, 0x4, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A3, 0x6);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    o760_sh(c, V0, 0x6, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A3, 0x8);
                    c.g[V1][0] = o760_add(c, T0, V1);
                    o760_sb(c, V1, 0x8, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A3, 0x9);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    o760_sb(c, V0, 0x9, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[V0][0] = o760_lbu(c, A3, 0xa);
                    c.g[V1][0] = o760_add(c, T0, V1);
                    let first = c.g[T1][0] == 0;
                    o760_sb(c, V1, 0xa, V0);
                    if first {
                        break 'copy;
                    }
                    // Extend the run of consecutive codes (font+0x10).
                    c.cycles += 7;
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[V1][0] = o760_lw(c, S0, 0x10);
                    c.g[V0][0] = o760_add(c, T0, V0);
                    c.g[V0][0] = o760_lhu(c, V0, 0);
                    c.g[V1][0] = o760_addi(c, V1, 1);
                    if c.g[V0][0] == c.g[V1][0] {
                        o760_sw(c, S0, 0x10, V0);
                    }
                }
                // Code 0xac: font+8 points at its record; code 0x20: remember the index (t2).
                'space: {
                    c.cycles += 8;
                    c.g[A1][0] = 0xc;
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.mult(0, 6, 9, 5);
                    c.g[A0][0] = o760_add(c, A2, V0);
                    c.g[V1][0] = o760_lhu(c, A0, 0);
                    c.g[V0][0] = c.g[V1][0] ^ 0x20;
                    let not_0xac = c.g[V1][0] != c.g[T4][0];
                    if c.g[V0][0] == 0 {
                        c.g[T2][0] = c.g[T1][0];
                    }
                    if not_0xac {
                        break 'space;
                    }
                    c.cycles += 1;
                    o760_sw(c, S0, 0x8, A0);
                }
                // A no-break space (0xa0) takes the metrics of the last space (record t2).
                'nbsp: {
                    c.cycles += 5;
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_add(c, A2, V0);
                    c.g[V1][0] = o760_lhu(c, A0, 0);
                    if c.g[V1][0] != c.g[T3][0] {
                        c.g[V1][0] = o760_lw(c, S0, 0x4);
                        break 'nbsp;
                    }
                    c.cycles += 29;
                    c.mult(0, 2, 10, 5);
                    c.g[A1][0] = o760_add(c, V0, S2);
                    c.g[V0][0] = o760_lbu(c, A1, 0x2);
                    o760_sb(c, A0, 0x2, V0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A1, 0x3);
                    c.g[V1][0] = o760_add(c, A2, V1);
                    o760_sb(c, V1, 0x3, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A1, 0x4);
                    c.g[V0][0] = o760_add(c, A2, V0);
                    o760_sh(c, V0, 0x4, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lhu(c, A1, 0x6);
                    c.g[V1][0] = o760_add(c, A2, V1);
                    o760_sh(c, V1, 0x6, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A1, 0x8);
                    c.g[V0][0] = o760_add(c, A2, V0);
                    o760_sb(c, V0, 0x8, A0);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                    c.g[A0][0] = o760_lbu(c, A1, 0x9);
                    c.g[V1][0] = o760_add(c, A2, V1);
                    o760_sb(c, V1, 0x9, A0);
                    c.g[V0][0] = o760_lw(c, S0, 0x4);
                    c.g[V1][0] = o760_lbu(c, A1, 0xa);
                    c.g[V0][0] = o760_add(c, A2, V0);
                    o760_sb(c, V0, 0xa, V1);
                    c.g[V1][0] = o760_lw(c, S0, 0x4);
                }
                // Track the greatest glyph extent (y offset + height) in font+0x14.
                {
                    c.cycles += 8;
                    c.g[A0][0] = o760_lw(c, S0, 0x14);
                    c.g[V1][0] = o760_add(c, A2, V1);
                    c.g[A1][0] = o760_lb(c, V1, 0xa);
                    c.g[V0][0] = o760_lbu(c, V1, 0x3);
                    c.g[V0][0] = o760_add(c, V0, A1);
                    c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        o760_sw(c, S0, 0x14, V0);
                    }
                }
                c.cycles += 6;
                c.g[V0][0] = o760_lw(c, S0, 0);
                c.g[T1][0] = o760_addi(c, T1, 1);
                c.g[A3][0] = o760_addi(c, A3, 0xc);
                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T0][0] = o760_addi(c, T0, 0xc);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0039_1838);
                    }
                    continue;
                }
                break 'glyphs;
            }
        }
        c.cycles += 10;
        c.g[A1][0] = o760_lw(c, S1, 0x1c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = o760_add(c, S1, A1);
        Callee {
            addr: 0x0039_1a48,
            f: ee::s12::Font_UploadTexture,
        }
        .call(c, 0x0039_19cc)?;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int SpeechInstance_LoadEvents(SpeechInstance* self, int mode, char* name, int tableFlags)`
/// (0x2afd40): registers the speech event table named `name` with the sound
/// system and the speech banks it refers to. The handle is kept at
/// `self+0x8d4 + 4*(mode==0) + 8*self->idx` (idx = `self+0xec`); the sub-name
/// list (`|`-separated, for `mode != 0`) and the bank entries are resolved
/// through the string and event-table callees, and each changed bank is
/// added with `SpeechInstance_AddBank`. Returns 1 on success, 0 when a
/// table could not be loaded.
pub fn speech_instance_load_events(c: &mut Cpu) {
    guest(c, |c| {
        'done: {
            'ok: {
                'no_handle: {
                    c.cycles += 35;
                    let frame = Frame::open(c, 0x170);
                    frame.save(c, 16, 0x160);
                    frame.save(c, 17, 0x150);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 18, 0x140);
                    c.g[S1][0] = c.g[A2][0];
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, 30, 0xe0);
                    frame.save(c, 19, 0x130);
                    c.g[A0][0] = c.g[S1][0];
                    frame.save(c, 20, 0x120);
                    c.g[FP][0] = c.g[A3][0];
                    frame.save(c, 21, 0x110);
                    c.g[A1][0] = 0;
                    frame.save(c, 22, 0x100);
                    c.g[S5][0] = 0x1;
                    frame.save(c, 23, 0xf0);
                    frame.save_ra(c, 0xd0);
                    c.g[S4][0] = o760_addi(c, S2, 0x8d4);
                    call_ptr(c, 0x003e_1908, 0x002a_fd90)?;
                    c.g[V1][0] = o760_lw(c, S2, 0xec);
                    c.g[A0][0] = 0;
                    if c.g[S0][0] == 0 {
                        c.g[A0][0] = c.g[S5][0];
                    }
                    c.g[S3][0] = o760_shl(c, A0, 2);
                    c.g[V1][0] = o760_shl(c, V1, 3);
                    c.g[V1][0] = o760_add(c, S3, V1);
                    c.g[V1][0] = o760_add(c, S4, V1);
                    o760_sw(c, V1, 0, V0);
                    c.g[V0][0] = o760_lw(c, S2, 0xec);
                    c.g[V0][0] = o760_shl(c, V0, 3);
                    c.g[V0][0] = o760_add(c, S3, V0);
                    c.g[V0][0] = o760_add(c, S4, V0);
                    c.g[V1][0] = o760_lw(c, V0, 0);
                    let missing = c.g[V1][0] == 0;
                    c.g[A1][0] = 0x480000;
                    if missing {
                        break 'no_handle;
                    }
                    'handled: {
                        'direct: {
                            c.cycles += 2;
                            let direct = c.g[S0][0] == 0;
                            o760_sb0(c, SP, 0);
                            if direct {
                                break 'direct;
                            }
                            // mode != 0: split the name at '|' and register the parts.
                            c.cycles += 13;
                            c.g[A1][0] = 0x480000;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = o760_addi(c, A1, 0x32c0);
                            c.g[A2][0] = o760_addi(c, S2, 0x6c);
                            call_ptr(c, 0x0041_7828, 0x002a_fde8)?;
                            c.g[S5][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0041_6810, 0x002a_fdf4)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[S0][0] = 0;
                            c.g[S6][0] = o760_addi(c, S2, 0xcc);
                            c.g[S7][0] = o760_addi(c, S2, 0xd0);
                            'lp: loop {
                                'found: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                    let end = c.g[V0][0] == 0;
                                    c.g[V0][0] = o760_add(c, S1, S0);
                                    if end {
                                        break 'found;
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = 0x7c;
                                    c.g[V1][0] = o760_lb(c, V0, 0);
                                    let other = c.g[V1][0] != c.g[A0][0];
                                    c.g[S0][0] = o760_addi(c, S0, 1);
                                    if other {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002a_fe08);
                                        }
                                        continue 'lp;
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A0][0] = c.g[SP][0];
                                    c.g[A2][0] = c.g[S0][0];
                                    call_ptr(c, 0x0041_6ce0, 0x002a_fe34)?;
                                    c.g[S5][0] = o760_add(c, S1, S0);
                                    c.g[V0][0] = o760_add(c, SP, S0);
                                    o760_sb0(c, V0, 0);
                                }
                                'registered: {
                                    c.cycles += 21;
                                    c.g[V0][0] = o760_lw(c, S2, 0xec);
                                    c.g[A1][0] = c.g[S5][0];
                                    c.g[V0][0] = o760_shl(c, V0, 3);
                                    c.g[V0][0] = o760_add(c, S3, V0);
                                    c.g[V0][0] = o760_add(c, S4, V0);
                                    c.g[A0][0] = o760_lw(c, V0, 0);
                                    call_ptr(c, 0x003e_2df0, 0x002a_fe5c)?;
                                    c.g[V1][0] = o760_lw(c, S2, 0xec);
                                    c.g[V1][0] = o760_shl(c, V1, 3);
                                    c.g[V1][0] = o760_add(c, S6, V1);
                                    o760_sw(c, V1, 0, V0);
                                    c.g[V0][0] = o760_lw(c, S2, 0xec);
                                    c.g[V0][0] = o760_shl(c, V0, 3);
                                    c.g[V0][0] = o760_add(c, S7, V0);
                                    o760_sw0(c, V0, 0);
                                    c.g[V1][0] = o760_lw(c, S2, 0xec);
                                    c.g[V1][0] = o760_shl(c, V1, 3);
                                    c.g[V0][0] = o760_add(c, S6, V1);
                                    c.g[A0][0] = o760_lw(c, V0, 0);
                                    let ok = c.g[A0][0] != 0;
                                    c.g[V0][0] = o760_add(c, S3, V1);
                                    if ok {
                                        break 'registered;
                                    }
                                    // The table could not be registered: free the handle, fail.
                                    c.cycles += 10;
                                    c.g[V0][0] = o760_add(c, S4, V0);
                                    c.g[A0][0] = o760_lw(c, V0, 0);
                                    call_ptr(c, 0x0025_23a8, 0x002a_fea0)?;
                                    c.g[V1][0] = o760_lw(c, S2, 0xec);
                                    c.g[V0][0] = 0;
                                    c.g[V1][0] = o760_shl(c, V1, 3);
                                    c.g[V1][0] = o760_add(c, S3, V1);
                                    c.g[V1][0] = o760_add(c, S4, V1);
                                    o760_sw0(c, V1, 0);
                                    break 'done;
                                }
                                c.cycles += 4;
                                c.g[A1][0] = c.g[FP][0];
                                call_ptr(c, 0x003d_66c0, 0x002a_fec4)?;
                                c.g[V0][0] = o760_lw(c, S2, 0xec);
                                break 'handled;
                            }
                        }
                        // mode == 0: the name is the single string "|".
                        c.cycles += 4;
                        c.g[V0][0] = 0x7c;
                        o760_sb0(c, SP, 1);
                        o760_sb(c, SP, 0, V0);
                        c.g[V0][0] = o760_lw(c, S2, 0xec);
                    }
                    c.cycles += 9;
                    c.g[S0][0] = 0;
                    c.g[V0][0] = o760_shl(c, V0, 3);
                    c.g[V0][0] = o760_add(c, S3, V0);
                    c.g[V0][0] = o760_add(c, S4, V0);
                    c.g[A0][0] = o760_lw(c, V0, 0);
                    call_ptr(c, 0x003e_2ee0, 0x002a_fef4)?;
                    c.g[S6][0] = c.g[V0][0];
                    let none = (c.g[S6][0] as i64) <= 0;
                    c.g[S5][0] = c.g[S3][0];
                    if none {
                        break 'ok;
                    }
                    c.cycles += 4;
                    c.g[S1][0] = o760_addi(c, SP, 0x60);
                    c.g[S7][0] = o760_addi(c, S2, 0xcc);
                    c.g[FP][0] = 0x4a0000;
                    c.g[V0][0] = o760_lw(c, S2, 0xec);
                    loop {
                        'bank_done: {
                            c.cycles += 22;
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A1][0] = 0;
                            c.g[A3][0] = o760_addi(c, SP, 0xc0);
                            c.g[V0][0] = o760_shl(c, V0, 3);
                            c.g[T0][0] = 0;
                            c.g[V0][0] = o760_add(c, S5, V0);
                            c.g[S3][0] = o760_addi(c, S0, 1);
                            c.g[V0][0] = o760_add(c, S4, V0);
                            c.g[A0][0] = o760_lw(c, V0, 0);
                            call_ptr(c, 0x003e_2768, 0x002a_ff38)?;
                            c.g[V1][0] = o760_lw(c, S2, 0xec);
                            c.g[A3][0] = c.g[V0][0];
                            c.g[A2][0] = o760_lw(c, SP, 0xc0);
                            c.g[V1][0] = o760_shl(c, V1, 3);
                            c.g[A0][0] = o760_add(c, S5, V1);
                            c.g[A0][0] = o760_add(c, S4, A0);
                            c.g[V1][0] = o760_add(c, S7, V1);
                            c.g[V0][0] = o760_lw(c, A0, 0);
                            c.g[A1][0] = o760_lw(c, V1, 0);
                            c.g[S0][0] = o760_add(c, V0, A2);
                            let unchanged = c.g[S0][0] == c.g[A1][0];
                            c.g[A1][0] = o760_addi(c, FP, 0x36e0);
                            if unchanged {
                                break 'bank_done;
                            }
                            // Rebuild the bank path (name up to the last '\\') and add it.
                            'path_end: {
                                'no_slash: {
                                    c.cycles += 11;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A2][0] = c.g[SP][0];
                                    call_ptr(c, 0x0041_7828, 0x002a_ff74)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x0041_6810, 0x002a_ff7c)?;
                                    c.g[V0][0] = o760_addi(c, V0, 0u32.wrapping_sub(4));
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[V1][0] = o760_add(c, S1, V0);
                                    c.g[A0][0] = c.g[V0][0];
                                    let neg = (c.g[A0][0] as i64) < 0;
                                    o760_sb0(c, V1, 0);
                                    if neg {
                                        break 'no_slash;
                                    }
                                    c.cycles += 3;
                                    c.g[A3][0] = c.g[A1][0];
                                    c.g[A2][0] = 0x5c;
                                    c.g[A0][0] = o760_addi(c, A0, 0u32.wrapping_sub(1));
                                    loop {
                                        c.cycles += 4;
                                        c.g[V0][0] = o760_add(c, A3, A0);
                                        c.g[V1][0] = o760_lb(c, V0, 0);
                                        if c.g[V1][0] == c.g[A2][0] {
                                            c.g[A2][0] = o760_add(c, A1, A0);
                                            break 'path_end;
                                        }
                                        c.cycles += 2;
                                        if (c.g[A0][0] as i64) >= 0 {
                                            c.g[A0][0] = o760_addi(c, A0, 0u32.wrapping_sub(1));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002a_ffa0);
                                            }
                                            continue;
                                        }
                                        break 'no_slash;
                                    }
                                }
                                c.cycles += 1;
                                c.g[A2][0] = o760_add(c, A1, A0);
                            }
                            c.cycles += 4;
                            c.g[A3][0] = c.g[S0][0];
                            c.g[A2][0] = o760_addi(c, A2, 1);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x002b_0348,
                                f: ee::s08::SpeechInstance_AddBank,
                            }
                            .call(c, 0x002a_ffcc)?;
                        }
                        c.cycles += 4;
                        c.g[S0][0] = c.g[S3][0];
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = o760_lw(c, S2, 0xec);
                            if event_due(c) {
                                deliver_event_here(c, 0x002a_ff10);
                            }
                            continue;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        break 'done;
                    }
                }
                // No handle yet: register the default table and keep it.
                c.cycles += 18;
                c.g[A2][0] = o760_addi(c, S2, 0x6c);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = o760_addi(c, A1, 0x32c0);
                call_ptr(c, 0x0041_7828, 0x002a_fff4)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x003e_1908, 0x002b_0000)?;
                c.g[V1][0] = o760_lw(c, S2, 0xec);
                c.g[A2][0] = o760_addi(c, S2, 0xcc);
                c.g[V1][0] = o760_shl(c, V1, 3);
                c.g[V1][0] = o760_add(c, A2, V1);
                o760_sw(c, V1, 0, V0);
                c.g[V0][0] = o760_lw(c, S2, 0xec);
                c.g[A0][0] = o760_shl(c, V0, 3);
                c.g[V1][0] = o760_add(c, A2, A0);
                c.g[V0][0] = o760_lw(c, V1, 0);
                let loaded = c.g[V0][0] != 0;
                c.g[V1][0] = o760_add(c, S2, A0);
                if !loaded {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    break 'done;
                }
                c.cycles += 7;
                c.g[A1][0] = c.g[FP][0];
                o760_sw(c, V1, 0xd0, S5);
                c.g[V0][0] = o760_lw(c, S2, 0xec);
                c.g[V0][0] = o760_shl(c, V0, 3);
                c.g[V0][0] = o760_add(c, A2, V0);
                c.g[A0][0] = o760_lw(c, V0, 0);
                call_ptr(c, 0x003d_66c0, 0x002b_0050)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        }
        // Epilogue.
        let frame = Frame { size: 0x170 };
        c.cycles += 12;
        frame.restore(c, 16, 0x160);
        frame.restore(c, 17, 0x150);
        frame.restore(c, 18, 0x140);
        frame.restore(c, 19, 0x130);
        frame.restore(c, 20, 0x120);
        frame.restore(c, 21, 0x110);
        frame.restore(c, 22, 0x100);
        frame.restore(c, 23, 0xf0);
        frame.restore(c, 30, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP, SP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects live in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    const OC760_CTRL: u32 = DATA + 0x30_0000;
    const OC760_MAGIC: u32 = 0x4f43_3736;
    /// Forced `v0` per fake call index: `m - 3` (0 = no override; 3 = zero, 2 = -1).
    const OC760_RES: u32 = DATA + 0x30_0100;
    /// Forced `v0` per callee address (`OC760_MAIL + (addr & 0xffc)`), same encoding.
    const OC760_MAIL: u32 = DATA + 0x30_1000;
    const OC760_OBJ: u32 = DATA + 0x20_0000;
    const OC760_BLOB: u32 = DATA + 0x21_0000;
    const OC760_FILE: u32 = DATA + 0x22_0000;
    const OC760_WORLD: u32 = DATA + 0x23_0000;
    const OC760_VT: u32 = DATA + 0x24_0000;
    const OC760_STR: u32 = DATA + 0x25_0000;

    fn oc760_mail(c: &mut Cpu, addr: u32, m: u32) {
        c.bus.write32(OC760_MAIL + (addr & 0xffc), m);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then applies the
    /// forced result (by call index, else by callee address) unless it unwound.
    fn oc760_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC760_MAIL + (addr & 0xffc));
        fakes::fake_call(c, addr);
        if c.pc == ra {
            let n = c.bus.read32(fakes::LOG_COUNT).wrapping_sub(1);
            let by_index = if n < 64 {
                c.bus.read32(OC760_RES + 4 * n)
            } else {
                0
            };
            let m = if by_index != 0 { by_index } else { mail };
            if m != 0 {
                c.g[V0][0] = sx(m.wrapping_sub(3));
            }
        }
    }

    macro_rules! oc760_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC760_CTRL) != OC760_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc760_fake_call(c, $addr);
                }
            )*
            fn oc760_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc760_fakes!(
        (
            oc760_fake_14dc80,
            OC760_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc760_fake_14dd58,
            OC760_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc760_fake_14ad28,
            OC760_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc760_fake_19cce8,
            OC760_PREV_19CCE8,
            ee::s03::OVERRIDE_0019CCE8,
            ee::s03::f_0019cce8__generated,
            0x0019_cce8
        ),
        (
            oc760_fake_19dc20,
            OC760_PREV_19DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20
        ),
        (
            oc760_fake_19cd38,
            OC760_PREV_19CD38,
            ee::s03::OVERRIDE_0019CD38,
            ee::s03::f_0019cd38__generated,
            0x0019_cd38
        ),
        (
            oc760_fake_19cbe0,
            OC760_PREV_19CBE0,
            ee::s03::OVERRIDE_0019CBE0,
            ee::s03::f_0019cbe0__generated,
            0x0019_cbe0
        ),
        (
            oc760_fake_19cc30,
            OC760_PREV_19CC30,
            ee::s03::OVERRIDE_0019CC30,
            ee::s03::f_0019cc30__generated,
            0x0019_cc30
        ),
        (
            oc760_fake_317e30,
            OC760_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc760_fake_317e98,
            OC760_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc760_fake_3e6448,
            OC760_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc760_fake_3e6574,
            OC760_PREV_3E6574,
            ee::s14::OVERRIDE_003E6574,
            ee::s14::f_003e6574__generated,
            0x003e_6574
        ),
        (
            oc760_fake_3e1908,
            OC760_PREV_3E1908,
            ee::s14::OVERRIDE_003E1908,
            ee::s14::f_003e1908__generated,
            0x003e_1908
        ),
        (
            oc760_fake_3e2df0,
            OC760_PREV_3E2DF0,
            ee::s14::OVERRIDE_003E2DF0,
            ee::s14::f_003e2df0__generated,
            0x003e_2df0
        ),
        (
            oc760_fake_3e2ee0,
            OC760_PREV_3E2EE0,
            ee::s14::OVERRIDE_003E2EE0,
            ee::s14::f_003e2ee0__generated,
            0x003e_2ee0
        ),
        (
            oc760_fake_3e2768,
            OC760_PREV_3E2768,
            ee::s14::OVERRIDE_003E2768,
            ee::s14::f_003e2768__generated,
            0x003e_2768
        ),
        (
            oc760_fake_417828,
            OC760_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc760_fake_416810,
            OC760_PREV_416810,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810
        ),
        (
            oc760_fake_416ce0,
            OC760_PREV_416CE0,
            ee::s15::OVERRIDE_00416CE0,
            ee::s15::f_00416ce0__generated,
            0x0041_6ce0
        ),
        (
            oc760_fake_2523a8,
            OC760_PREV_2523A8,
            ee::s06::OVERRIDE_002523A8,
            ee::s06::f_002523a8__generated,
            0x0025_23a8
        ),
        (
            oc760_fake_2b0348,
            OC760_PREV_2B0348,
            ee::s08::OVERRIDE_002B0348,
            ee::s08::f_002b0348__generated,
            0x002b_0348
        ),
    );

    /// `*handle = -1` and the pointer comes back in v0.
    #[test]
    fn bolt_on_fx_interface_init_handle_matches() {
        check(
            ee::s09::f_002ec9d0__generated,
            bolt_on_fx_interface_init_handle,
            50,
            &|c, seed| {
                let mut r = Rng(0x2e_c9d0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let handle = DATA + (r.next_u32() & 0xf_fff0);
                c.g[A0][0] = sx(handle);
                c.bus.write32(handle, r.next_u32());
            },
        );
    }

    /// Release the bolt-file textures and models of the preview rider: covers
    /// the two early exits (negative rider / file), empty and full entry lists,
    /// present and absent texture/model slots, and unwinding callees.
    #[test]
    fn fe_module_rider_preview_release_bolt_files_matches() {
        check(
            ee::s03::f_0019ec68__generated,
            fe_module_rider_preview_release_bolt_files,
            200,
            &|c, seed| {
                oc760_install_fakes();
                let mut r = machine(c, seed, 0x19_ec68, 30);
                c.bus.write32(OC760_CTRL, OC760_MAGIC);
                let neg = 0x8000_0000 | r.next_u32();
                let (file, rider) = match seed % 6 {
                    0 => (5, neg),
                    1 => (neg, 5),
                    _ => (r.next_u32() & 0x3ff, r.next_u32() & 0x3ff),
                };
                c.bus.write32(OC760_OBJ, file);
                c.bus.write32(OC760_OBJ + 4, rider);
                c.g[A0][0] = sx(OC760_OBJ);
                c.bus.write32(GAME_GP - 0x848, OC760_WORLD);
                c.bus.write32(OC760_WORLD + 0x7c, r.next_u32() & 0xf_fff0);
                c.bus.write32(GAME_GP - 0x1788, r.next_u32());
                // The rider's index entry: a count (0, negative or 1..5) and
                // 4-byte entries holding a short index.
                let count = match (seed / 6) % 6 {
                    0 => 0,
                    1 => 0xffff_fffe,
                    n => n,
                };
                c.bus.write32(OC760_BLOB + 0x28c, count);
                for i in 0..8 {
                    c.bus.write32(OC760_BLOB + 0x290 + 4 * i, r.next_u32());
                }
                oc760_mail(c, 0x0014_ad28, OC760_BLOB + 3);
                for i in 3..40 {
                    let m = [0, 2, 9][(r.next_u32() % 3) as usize];
                    c.bus.write32(OC760_RES + 4 * i, m);
                }
            },
        );
    }

    /// Service a path queue: entries with and without due times, flagged
    /// (skip the action) and unflagged ones (run `PATH_ExecuteAction` on an
    /// empty action), 1 to 4 entries.
    #[test]
    fn path_service_queue_matches() {
        check(
            ee::s14::f_003d1d28__generated,
            path_service_queue,
            300,
            &|c, seed| {
                let mut r = Rng(0x3d_1d28 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let queue_no = seed % 4;
                let state = OC760_BLOB;
                c.bus.write32(0x0045_0000 - 0xbe0, DATA);
                c.bus.write32(DATA + 0xdc + 4 * queue_no, state);
                c.bus.write32(0x0045_0000 - 0xbc8, r.next_u32() & 0xff); // now
                c.bus.write32(state, r.next_u32() & 0xff); // time limit
                c.bus.write32(state + 0x8, r.next_u32());
                let tail = seed % 3;
                let head = tail + 1 + (seed / 3) % 4;
                c.bus.write32(
                    state + 0xc,
                    (tail << 22) | (head << 16) | (r.next_u32() & 0xf000_ffff),
                );
                for i in 0..8u32 {
                    let e = state + 0x10 + 16 * i;
                    let time = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() & 0xff
                    };
                    let flags =
                        if r.next_u32() % 2 == 0 { 0x400_0000 } else { 0 } | (r.next_u32() & 0xff);
                    c.bus.write32(e, 0); // empty action mask
                    c.bus.write32(e + 4, time);
                    c.bus.write32(e + 8, flags);
                    c.bus.write32(e + 0xc, 0);
                }
                c.g[A0][0] = sx(queue_no);
            },
        );
    }

    /// Build a font file with `n` glyph records and a texture header, the
    /// world object the texture upload reads, and the fakes' gate.
    #[test]
    fn font_parse_glyphs_matches() {
        check(
            ee::s12::f_00391788__generated,
            font_parse_glyphs,
            200,
            &|c, seed| {
                oc760_install_fakes();
                let mut r = machine(c, seed, 0x39_1788, 12);
                c.bus.write32(OC760_CTRL, OC760_MAGIC);
                let font = OC760_OBJ;
                let file = OC760_FILE;
                let n = 1 + seed % 7;
                c.g[A0][0] = sx(font);
                c.g[A1][0] = sx(file);
                c.bus.write16(file + 0xa, n as u16);
                c.bus.write8(file + 0x10, r.next_u32() as u8);
                c.bus.write8(file + 0x11, r.next_u32() as u8);
                c.bus.write32(file + 0x14, 0x100); // glyph records
                c.bus.write32(file + 0x1c, 0x400); // texture header
                for i in 0..n {
                    let g = file + 0x100 + 12 * i;
                    // Codes: space, no-break space, the 0xac fallback, consecutive, random.
                    let code = match r.next_u32() % 5 {
                        0 => 0x20,
                        1 => 0xa0,
                        2 => 0xac,
                        3 => i,
                        _ => r.next_u32() & 0xffff,
                    };
                    c.bus.write16(g, code as u16);
                    for b in 2..11 {
                        // Heights and offsets: bytes, some negative.
                        c.bus.write8(g + b, r.next_u32() as u8);
                    }
                }
                // The texture header.
                let tex = file + 0x400;
                c.bus.write16(tex + 4, 8 + 2 * (r.next_u32() & 7) as u16);
                c.bus.write16(tex + 6, 1 + (r.next_u32() & 0x3f) as u16);
                c.bus
                    .write32(tex + 0xc, if seed % 2 == 0 { 0x1000 } else { 0 });
                c.bus.write32(tex + 0x10, 0x40);
                // gp+0x2a90: the world; world+0x10d8: its vtable block.
                c.bus.write32(GAME_GP + 0x2a90, OC760_WORLD);
                c.bus.write32(OC760_WORLD + 0x10d8, OC760_VT);
                c.bus.write16(OC760_VT + 0x178, r.next_u32() as u16 & 0xff);
                c.bus.write32(OC760_VT + 0x17c, fakes::FAKE);
            },
        );
    }

    /// Register a speech event table: with and without an existing handle,
    /// `mode` 0 and non-zero (name split at '|'), failing and succeeding table
    /// lookups, unchanged and changed banks.
    #[test]
    fn speech_instance_load_events_matches() {
        check(
            ee::s08::f_002afd40__generated,
            speech_instance_load_events,
            300,
            &|c, seed| {
                oc760_install_fakes();
                let mut r = machine(c, seed, 0x2a_fd40, 14);
                c.bus.write32(OC760_CTRL, OC760_MAGIC);
                let this = OC760_OBJ;
                let idx = seed % 3;
                c.bus.write32(this + 0xec, idx);
                let mode_zero = (seed / 3) % 2 == 0;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if mode_zero {
                    0
                } else {
                    1 + (r.next_u32() & 0xff) as u64
                };
                c.g[A2][0] = sx(OC760_STR);
                c.g[A3][0] = sx(8 + (r.next_u32() & 0xff)); // fp >= 8: SPCH_RegisterEventTable stays out
                                                            // The name: 4..12 characters, a '|' somewhere (or not).
                let len = 4 + r.next_u32() % 9;
                for i in 0..len {
                    let ch = match r.next_u32() % 5 {
                        0 => b'|',
                        1 => b'\\',
                        _ => b'a' + (r.next_u32() % 26) as u8,
                    };
                    c.bus.write8(OC760_STR + i, ch);
                }
                c.bus.write8(OC760_STR + len, 0);
                oc760_mail(c, 0x0041_6810, len + 3); // strlen
                                                     // Handles and results.
                let handle = 0x1000 + (r.next_u32() & 0xfff) * 16;
                oc760_mail(c, 0x003e_1908, handle + 3);
                let fail_path = (seed / 6) % 4 == 1;
                if fail_path {
                    c.bus.write32(OC760_RES, 3); // first table lookup: null
                    if seed % 2 == 0 {
                        c.bus.write32(OC760_RES + 8, 3); // second one too
                    }
                }
                let stack_word = r.next_u32() & 0xffff;
                let equal = seed % 2 == 0;
                let registered = if equal {
                    handle.wrapping_add(stack_word)
                } else {
                    r.next_u32()
                };
                let reg_ret = if (seed / 5) % 5 == 2 {
                    3
                } else {
                    registered.wrapping_add(3)
                };
                oc760_mail(c, 0x003e_2df0, reg_ret);
                oc760_mail(c, 0x003e_2ee0, [3, 2, 4, 5, 6][(r.next_u32() % 5) as usize]);
                oc760_mail(c, 0x003e_2768, r.next_u32() | 1);
                let slot = this + 0xcc + 8 * idx;
                c.bus.write32(slot, registered);
                // Stack words the function reads back (the callee would have written them).
                let frame = SP - 0x170;
                c.bus.write32(frame + 0xc0, stack_word);
                for i in 0..0x20 {
                    let b = if r.next_u32() % 4 == 0 {
                        b'\\'
                    } else {
                        b'a' + (r.next_u32() % 26) as u8
                    };
                    c.bus.write8(frame + 0x60 + i, b);
                }
            },
        );
    }
}
