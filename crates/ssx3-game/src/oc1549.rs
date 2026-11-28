//! Staging module oc1549 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1549`.

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
    (0x002b_aee8, cworld_sound_global_ctors, "cWorldSound_GlobalCtors"),
    (0x002b_afb0, world_painter_static_init, "WorldPainter_StaticInit"),
    (0x002b_b0e0, world_painter_global_ctors, "WorldPainter_GlobalCtors"),
    (0x002b_b140, cworld_painter_man_dtor, "cWorldPainterMan_dtor"),
    (0x002b_b2b8, cworld_painter_man_update, "cWorldPainterMan_Update"),
    (0x002b_b318, cworld_painter_man_load_file, "cWorldPainterMan_LoadFile"),
    (0x002b_b390, wpfile_dtor, "WPFile_dtor"),
    (0x002b_b3f0, wpfile_poll_load, "WPFile_PollLoad"),
];

/// `cWorldSound_GlobalCtors` (0x2baee8): the static constructor of the world
/// sound system, `cWorldSound_StaticInit(1, 0xffff)`.
pub fn cworld_sound_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1;
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x002b_8818,
            f: ee::s08::f_002b8818,
        }
        .call(c, 0x002b_aefc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `WorldPainter_StaticInit(int initialize, int priority)` (0x2bafb0): when
/// `priority` is the default (0xffff) and `initialize` is set, it fills the
/// world painter's static tables at gp+0x2200.. (flat stores of constants and
/// index bytes). The stores are transcribed from the generated body in its
/// order: they are a table, so the statements stay as generated.
pub fn world_painter_static_init(c: &mut Cpu) {
'b_2bb0d8: {
c.cycles += 3;
c.g[V0][0] = 0xffff;
let t = c.g[A1][0] != c.g[V0][0];
if t {
break 'b_2bb0d8;
} else {
c.cycles += 2;
let t = c.g[A0][0] == 0u64;
c.g[V1][0] = 0x2;
if t {
break 'b_2bb0d8;
} else {
c.cycles += 69;
c.g[A1][0] = 0x4;
c.g[A2][0] = 0x5;
c.sb((c.g[GP][0] as u32).wrapping_add(0x2208), c.g[V1][0]);
c.sb((c.g[GP][0] as u32).wrapping_add(0x2218), c.g[A1][0]);
c.g[A0][0] = 0x3;
c.sb((c.g[GP][0] as u32).wrapping_add(0x2220), c.g[A2][0]);
c.g[V0][0] = 0x1;
c.g[T0][0] = 0x7;
c.g[T1][0] = 0x8;
c.g[T2][0] = 0x9;
c.sb((c.g[GP][0] as u32).wrapping_add(0x2200), c.g[V0][0]);
c.sb((c.g[GP][0] as u32).wrapping_add(0x2210), c.g[A0][0]);
c.g[A3][0] = 0x6;
c.sb((c.g[GP][0] as u32).wrapping_add(0x2230), c.g[T0][0]);
c.g[T3][0] = c.g[V1][0] & 0xff;
c.sb((c.g[GP][0] as u32).wrapping_add(0x2238), c.g[T1][0]);
c.g[T0][0] = c.g[A2][0] & 0xff;
c.sb((c.g[GP][0] as u32).wrapping_add(0x2240), c.g[T2][0]);
c.g[V0][0] = 0xa;
c.sb((c.g[GP][0] as u32).wrapping_add(0x2228), c.g[A3][0]);
c.g[T2][0] = c.g[A1][0] & 0xff;
c.sb((c.g[GP][0] as u32).wrapping_add(0x21f8), 0u64);
c.g[T4][0] = c.g[A0][0] & 0xff;
c.g[T1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2200));
c.g[A3][0] = c.g[A3][0] & 0xff;
c.g[A2][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2230));
c.g[T3][0] = c.g[T3][0] | 0x200;
c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2238));
c.g[T1][0] = c.g[T1][0] | 0x100;
c.g[V1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2240));
c.g[T4][0] = c.g[T4][0] | 0x300;
c.sb((c.g[GP][0] as u32).wrapping_add(0x2248), c.g[V0][0]);
c.g[T2][0] = c.g[T2][0] | 0x400;
c.g[A0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x21f8));
c.g[V1][0] = c.g[V1][0] | 0x900;
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[T0][0] = c.g[T0][0] | 0x500;
c.g[A3][0] = c.g[A3][0] | 0x600;
c.g[A2][0] = c.g[A2][0] | 0x700;
c.g[A1][0] = c.g[A1][0] | 0x800;
c.sw((c.g[GP][0] as u32).wrapping_add(0x21f8), c.g[A0][0]);
c.sw((c.g[GP][0] as u32).wrapping_add(0x2240), c.g[V1][0]);
c.g[V0][0] = c.g[V0][0] | 0xa00;
c.sw((c.g[GP][0] as u32).wrapping_add(0x2200), c.g[T1][0]);
c.g[T7][0] = 0xb;
c.sw((c.g[GP][0] as u32).wrapping_add(0x2208), c.g[T3][0]);
c.g[T6][0] = 0xc;
c.sw((c.g[GP][0] as u32).wrapping_add(0x2210), c.g[T4][0]);
c.g[T5][0] = 0xd;
c.sw((c.g[GP][0] as u32).wrapping_add(0x2218), c.g[T2][0]);
c.sw((c.g[GP][0] as u32).wrapping_add(0x2220), c.g[T0][0]);
c.sw((c.g[GP][0] as u32).wrapping_add(0x2228), c.g[A3][0]);
c.sw((c.g[GP][0] as u32).wrapping_add(0x2230), c.g[A2][0]);
c.sw((c.g[GP][0] as u32).wrapping_add(0x2238), c.g[A1][0]);
c.sw((c.g[GP][0] as u32).wrapping_add(0x21f4), 0u64);
c.sw((c.g[GP][0] as u32).wrapping_add(0x21f0), 0u64);
c.sw((c.g[GP][0] as u32).wrapping_add(0x2248), c.g[V0][0]);
c.sb((c.g[GP][0] as u32).wrapping_add(0x2250), c.g[T7][0]);
c.sb((c.g[GP][0] as u32).wrapping_add(0x2258), c.g[T6][0]);
c.sb((c.g[GP][0] as u32).wrapping_add(0x2260), c.g[T5][0]);
c.g[V0][0] = c.g[T7][0] & 0xff;
c.g[V1][0] = c.g[T6][0] & 0xff;
c.g[V0][0] = c.g[V0][0] | 0xb00;
c.g[A0][0] = c.g[T5][0] & 0xff;
c.g[V1][0] = c.g[V1][0] | 0xc00;
c.sw((c.g[GP][0] as u32).wrapping_add(0x2250), c.g[V0][0]);
c.g[A0][0] = c.g[A0][0] | 0xd00;
c.sw((c.g[GP][0] as u32).wrapping_add(0x2258), c.g[V1][0]);
c.sw((c.g[GP][0] as u32).wrapping_add(0x2260), c.g[A0][0]);
break 'b_2bb0d8;
}
}
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.pc = t; return;
}

/// `WorldPainter_GlobalCtors` (0x2bb0e0): the static constructor of the world
/// painter, `WorldPainter_StaticInit(1, 0xffff)`.
pub fn world_painter_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x1;
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x002b_afb0,
            f: ee::s08::f_002bafb0,
        }
        .call(c, 0x002b_b0f4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldPainterMan::~cWorldPainterMan(int flags)` (0x2bb140): destroys the
/// WPFile entries of the array at `this+4` (their count is the word before
/// the array, 12-byte stride, last entry first), frees the array (0x317e98),
/// and deletes `this` (0x317e50) when flags bit 0 is set.
pub fn cworld_painter_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        let empty = c.g[A1][0] == 0u64;
        c.g[V0][0] = 0xc;
        'done: {
            if empty {
                break 'done;
            }
            c.cycles += 5;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(0x10)).lw(c);
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] != c.g[S0][0] {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x002b_b390,
                        f: ee::s08::f_002bb390,
                    }
                    .call(c, 0x002b_b18c)?;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xc));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002b_b180);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    break;
                }
            } else {
                c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x002b_b1a4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        let delete_this = c.g[V0][0] != 0u64;
        frame.restore(c, S0, 0x30);
        if delete_this {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002b_b1b8)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldPainterMan::Update(this)` (0x2bb2b8): polls each of the WPFile
/// entries (the count is the word at `this`, the array at `this+4`, 12 bytes
/// apart) through `WPFile_PollLoad`.
pub fn cworld_painter_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0u64;
        c.g[S0][0] = 0u64;
        'done: {
            if empty {
                break 'done;
            }
            c.cycles += 1;
            c.g[S2][0] = 0u64;
            loop {
                c.cycles += 8;
                c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                Callee {
                    addr: 0x002b_b3f0,
                    f: ee::s08::f_002bb3f0,
                }
                .call(c, 0x002b_b2f0)?;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0u64;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_b2e0);
                    }
                    continue;
                }
                break 'done;
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cWorldPainterMan::LoadFile(this, index, flags)` (0x2bb318): starts the
/// load of WPFile entry `index` (12-byte stride in the array at `this+4`)
/// through `WPFile_BeginLoad`.
pub fn cworld_painter_man_load_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc;
        frame.save_ra(c, 0);
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x4)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Callee {
            addr: 0x002b_b4d8,
            f: ee::s08::f_002bb4d8,
        }
        .call(c, 0x002b_b338)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `WPFile::~WPFile(int flags)` (0x2bb390): frees the buffer at +8 (0x317e98)
/// when there is one and the short at +2 is zero, and deletes the object
/// (0x317e50) when flags bit 0 is set.
pub fn wpfile_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
        let no_buffer = c.g[A0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        'b3c8: {
            'b3c4: {
                if no_buffer {
                    break 'b3c4;
                }
                c.cycles += 3;
                let short_set = Addr((c.g[S0][0] as u32).wrapping_add(0x2)).lh(c) != 0u64;
                c.g[V0][0] = c.g[S1][0] & 0x1;
                if short_set {
                    break 'b3c8;
                }
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x002b_b3c4)?;
                break 'b3c4;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }
        'b3dc: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                frame.restore(c, S0, 0x20);
                break 'b3dc;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002b_b3d8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `WPFile::PollLoad(this)` (0x2bb3f0): while the file's state (the short at
/// `this`) is 1 and its async load is done, takes the result (a buffer and its
/// size, through `AsyncFile_GetResult`), allocates a buffer for it and copies
/// the data in (`this+8` receives the buffer); the state is then set to 2 when
/// there is no data, or to 2 after the copy.
pub fn wpfile_poll_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x1;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).lh(c);
        'exit: {
            if c.g[S0][0] != c.g[V0][0] {
                break 'exit;
            }
            c.cycles += 4;
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
            call_ptr(c, 0x003d_f980, 0x002b_b41c)?;
            if c.g[V0][0] != c.g[S0][0] {
                break 'exit;
            }
            'store: {
                'result: {
                    c.cycles += 9;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[A1][0] = c.g[SP][0];
                    Addr(sp(c)).sw(c, 0);
                    c.g[A2][0] = c.g[SP][0] | 0x4;
                    Addr(sp(c).wrapping_add(0x4)).sw(c, 0);
                    call_ptr(c, 0x003d_f9d8, 0x002b_b43c)?;
                    c.g[A0][0] = Addr(sp(c)).lw(c);
                    let buffer_null = c.g[A0][0] == 0u64;
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x4)).lw(c);
                    if buffer_null {
                        break 'result;
                    }
                    c.cycles += 2;
                    let size_le_0 = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = 0x2;
                    if size_le_0 {
                        break 'store;
                    }
                    'copy: {
                        'size: {
                            c.cycles += 4;
                            call_ptr(c, 0x003b_4818, 0x002b_b458)?;
                            let unpacked_le_0 = (c.g[V0][0] as i64) <= 0;
                            c.g[A1][0] = 0x4a_0000;
                            if unpacked_le_0 {
                                break 'size;
                            }
                            c.cycles += 11;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3868));
                            c.g[A2][0] = 0x2000_0000;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x0031_7e30, 0x002b_b474)?;
                            c.g[A0][0] = Addr(sp(c)).lw(c);
                            c.g[A1][0] = c.g[V0][0];
                            let buffer = c.g[V0][0];
                            Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, buffer);
                            call_ptr(c, 0x003b_47f8, 0x002b_b484)?;
                            break 'copy;
                        }
                        c.cycles += 10;
                        c.g[A0][0] = Addr(sp(c).wrapping_add(0x4)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3868));
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x0031_7e30, 0x002b_b4a0)?;
                        c.g[A2][0] = Addr(sp(c).wrapping_add(0x4)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = Addr(sp(c)).lw(c);
                        let buffer = c.g[V0][0];
                        Addr((c.g[S1][0] as u32).wrapping_add(0x8)).sw(c, buffer);
                        call_ptr(c, 0x003e_6574, 0x002b_b4b4)?;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x002b_b4bc);
                    c.g[A0][0] = Addr(sp(c)).lw(c);
                    call_ptr(c, 0x0025_23a8, 0x002b_b4bc)?;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x2;
            }
            c.cycles += 2;
            let state = c.g[V0][0];
            Addr(c.g[S1][0] as u32).sh(c, state);
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word value: a test's fakes run only while their gate word holds it.
    const OC1549_MAGIC: u32 = 0x1549_c0de;

    /// Gate words of this module's tests (one per test, in the fixture area
    /// above the call log and below the stack).
    const OC1549_SS_GATE: u32 = DATA + 0x2e_0000;
    const OC1549_MD_GATE: u32 = DATA + 0x2e_0100;
    const OC1549_DT_GATE: u32 = DATA + 0x2e_0200;
    const OC1549_UP_GATE: u32 = DATA + 0x2e_0300;
    const OC1549_LF_GATE: u32 = DATA + 0x2e_0400;
    const OC1549_PL_GATE: u32 = DATA + 0x2e_0500;

    /// A gated fake of one callee's override slot. With its gate word set, it
    /// logs the call to `target` and returns the word at `gate + mail` in v0;
    /// with the gate off it runs the body the slot held before.
    macro_rules! oc1549_gated_fake {
        ($fake:ident, $prev:ident, $gate:expr, $mail:expr, $gen:path, $target:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($gate) != OC1549_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $target);
                c.g[V0][0] = c.bus.read32($gate + $mail) as u64;
            }
        };
    }

    // WorldPainter_StaticInit's callee cWorldSound_StaticInit.
    oc1549_gated_fake!(
        oc1549_ss_init,
        OC1549_SS_PREV,
        OC1549_SS_GATE,
        4,
        ee::s08::f_002b8818__generated,
        0x002b_8818
    );

    fn oc1549_ss_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B8818),
            oc1549_ss_init,
            &OC1549_SS_PREV,
        );
    }

    /// `cWorldSound_GlobalCtors` against the generated body, with the static
    /// init faked (gate on) so the test does not run it.
    #[test]
    fn cworld_sound_global_ctors_matches() {
        check(
            ee::s08::f_002baee8__generated,
            cworld_sound_global_ctors,
            200,
            &|c, seed| {
                oc1549_ss_hook();
                machine(c, seed, 0x2b_aee8, 2);
                c.bus.write32(OC1549_SS_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_SS_GATE + 4, seed);
                c.pc = 0x002b_aee8;
            },
        );
    }

    /// `WorldPainter_StaticInit` (a leaf of flat stores) for both branches:
    /// priority other than 0xffff, and a null or non-null first argument.
    #[test]
    fn world_painter_static_init_matches() {
        check(
            ee::s08::f_002bafb0__generated,
            world_painter_static_init,
            200,
            &|c, seed| {
                machine(c, seed, 0x2b_afb0, 0);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { DATA as u64 };
                c.g[A1][0] = if seed % 3 == 0 {
                    0xffff
                } else {
                    seed as u64
                };
                c.pc = 0x002b_afb0;
            },
        );
    }

    /// `WorldPainter_GlobalCtors` calls the real static init (a leaf).
    #[test]
    fn world_painter_global_ctors_matches() {
        check(
            ee::s08::f_002bb0e0__generated,
            world_painter_global_ctors,
            200,
            &|c, seed| {
                machine(c, seed, 0x2b_b0e0, 0);
                c.pc = 0x002b_b0e0;
            },
        );
    }

    // cWorldPainterMan_dtor's callees: WPFile_dtor, the free and the delete.
    oc1549_gated_fake!(
        oc1549_md_wpfile_dtor,
        OC1549_MD_PREV_DTOR,
        OC1549_MD_GATE,
        4,
        ee::s08::f_002bb390__generated,
        0x002b_b390
    );
    oc1549_gated_fake!(
        oc1549_md_free,
        OC1549_MD_PREV_FREE,
        OC1549_MD_GATE,
        8,
        ee::s10::f_00317e98__generated,
        0x0031_7e98
    );
    oc1549_gated_fake!(
        oc1549_md_delete,
        OC1549_MD_PREV_DELETE,
        OC1549_MD_GATE,
        12,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );

    fn oc1549_md_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB390),
            oc1549_md_wpfile_dtor,
            &OC1549_MD_PREV_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1549_md_free,
            &OC1549_MD_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1549_md_delete,
            &OC1549_MD_PREV_DELETE,
        );
    }

    /// `cWorldPainterMan_dtor` for 0..3 entries (the empty array, the array
    /// of one, and the walk over several), with and without the delete.
    #[test]
    fn cworld_painter_man_dtor_matches() {
        check(
            ee::s08::f_002bb140__generated,
            cworld_painter_man_dtor,
            300,
            &|c, seed| {
                oc1549_md_hook();
                machine(c, seed, 0x2b_b140, 4);
                c.bus.write32(OC1549_MD_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_MD_GATE + 4, 0);
                c.bus.write32(OC1549_MD_GATE + 8, 0);
                c.bus.write32(OC1549_MD_GATE + 12, 0);
                let this = DATA + 0x1000;
                let arr = if seed % 5 == 0 { 0 } else { DATA + 0x2010 };
                c.bus.write32(this + 4, arr);
                if arr != 0 {
                    c.bus.write32(arr - 0x10, seed % 4);
                }
                c.bus.write32(this, seed);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x002b_b140;
            },
        );
    }

    // WPFile_dtor's callees: the buffer free and the delete.
    oc1549_gated_fake!(
        oc1549_dt_free,
        OC1549_DT_PREV_FREE,
        OC1549_DT_GATE,
        4,
        ee::s10::f_00317e98__generated,
        0x0031_7e98
    );
    oc1549_gated_fake!(
        oc1549_dt_delete,
        OC1549_DT_PREV_DELETE,
        OC1549_DT_GATE,
        8,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );

    fn oc1549_dt_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1549_dt_free,
            &OC1549_DT_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1549_dt_delete,
            &OC1549_DT_PREV_DELETE,
        );
    }

    /// `WPFile_dtor` over the buffer and flag combinations.
    #[test]
    fn wpfile_dtor_matches() {
        check(
            ee::s08::f_002bb390__generated,
            wpfile_dtor,
            300,
            &|c, seed| {
                oc1549_dt_hook();
                machine(c, seed, 0x2b_b390, 3);
                c.bus.write32(OC1549_DT_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_DT_GATE + 4, 0);
                c.bus.write32(OC1549_DT_GATE + 8, 0);
                let file = DATA + 0x1000;
                let buffer = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
                c.bus.write32(file + 8, buffer);
                c.bus.write16(file + 2, if seed % 2 == 0 { 0 } else { 5 });
                c.g[A0][0] = sx(file);
                c.g[A1][0] = seed as u64;
                c.pc = 0x002b_b390;
            },
        );
    }

    // cWorldPainterMan_Update's callee WPFile_PollLoad (the poll is faked).
    oc1549_gated_fake!(
        oc1549_up_poll,
        OC1549_UP_PREV,
        OC1549_UP_GATE,
        4,
        ee::s08::f_002bb3f0__generated,
        0x002b_b3f0
    );

    fn oc1549_up_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB3F0),
            oc1549_up_poll,
            &OC1549_UP_PREV,
        );
    }

    /// `cWorldPainterMan_Update` for 0..3 entries.
    #[test]
    fn cworld_painter_man_update_matches() {
        check(
            ee::s08::f_002bb2b8__generated,
            cworld_painter_man_update,
            300,
            &|c, seed| {
                oc1549_up_hook();
                machine(c, seed, 0x2b_b2b8, 3);
                c.bus.write32(OC1549_UP_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_UP_GATE + 4, 0);
                let this = DATA + 0x1000;
                c.bus.write32(this, seed % 4);
                c.bus.write32(this + 4, DATA + 0x2000);
                c.g[A0][0] = sx(this);
                c.pc = 0x002b_b2b8;
            },
        );
    }

    // cWorldPainterMan_LoadFile's callee WPFile_BeginLoad (faked).
    oc1549_gated_fake!(
        oc1549_lf_begin,
        OC1549_LF_PREV,
        OC1549_LF_GATE,
        4,
        ee::s08::f_002bb4d8__generated,
        0x002b_b4d8
    );

    fn oc1549_lf_hook() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002BB4D8),
            oc1549_lf_begin,
            &OC1549_LF_PREV,
        );
    }

    /// `cWorldPainterMan_LoadFile` for several indices and flag words.
    #[test]
    fn cworld_painter_man_load_file_matches() {
        check(
            ee::s08::f_002bb318__generated,
            cworld_painter_man_load_file,
            300,
            &|c, seed| {
                oc1549_lf_hook();
                machine(c, seed, 0x2b_b318, 2);
                c.bus.write32(OC1549_LF_GATE, OC1549_MAGIC);
                c.bus.write32(OC1549_LF_GATE + 4, 0);
                let this = DATA + 0x1000;
                c.bus.write32(this + 4, DATA + 0x2000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = seed as u64 % 7;
                c.g[A2][0] = (seed as u64 * 3) % 5;
                c.pc = 0x002b_b318;
            },
        );
    }

    // WPFile_PollLoad's callees. The async-file queries and the allocator are
    // gated fakes; the query results come from the gate's mailbox words.
    oc1549_gated_fake!(
        oc1549_pl_isdone,
        OC1549_PL_PREV_ISDONE,
        OC1549_PL_GATE,
        4,
        ee::s14::f_003df980__generated,
        0x003d_f980
    );
    oc1549_gated_fake!(
        oc1549_pl_size,
        OC1549_PL_PREV_SIZE,
        OC1549_PL_GATE,
        8,
        ee::s13::f_003b4818__generated,
        0x003b_4818
    );
    oc1549_gated_fake!(
        oc1549_pl_unpack,
        OC1549_PL_PREV_UNPACK,
        OC1549_PL_GATE,
        4,
        ee::s13::f_003b47f8__generated,
        0x003b_47f8
    );
    oc1549_gated_fake!(
        oc1549_pl_new,
        OC1549_PL_PREV_NEW,
        OC1549_PL_GATE,
        0xc,
        ee::s10::f_00317e30__generated,
        0x0031_7e30
    );
    oc1549_gated_fake!(
        oc1549_pl_memcpy,
        OC1549_PL_PREV_MEMCPY,
        OC1549_PL_GATE,
        4,
        ee::s14::f_003e6574__generated,
        0x003e_6574
    );
    oc1549_gated_fake!(
        oc1549_pl_free,
        OC1549_PL_PREV_FREE,
        OC1549_PL_GATE,
        4,
        ee::s06::f_002523a8__generated,
        0x0025_23a8
    );

    static OC1549_PL_PREV_RESULT: AtomicUsize = AtomicUsize::new(0);

    /// `AsyncFile_GetResult`: with the gate on, writes the two result words
    /// (gate + 0x10 and + 0x14) through its out pointers `a1` and `a2`.
    fn oc1549_pl_result(c: &mut Cpu) {
        if c.bus.read32(OC1549_PL_GATE) != OC1549_MAGIC {
            return fakes::fall_back(
                c,
                &OC1549_PL_PREV_RESULT,
                ee::s14::f_003df9d8__generated,
            );
        }
        let out_a = c.g[A1][0] as u32;
        let out_b = c.g[A2][0] as u32;
        let a = c.bus.read32(OC1549_PL_GATE + 0x10);
        let b = c.bus.read32(OC1549_PL_GATE + 0x14);
        c.bus.write32(out_a, a);
        c.bus.write32(out_b, b);
        fakes::fake_call(c, 0x003d_f9d8);
    }

    fn oc1549_pl_hook() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF980),
            oc1549_pl_isdone,
            &OC1549_PL_PREV_ISDONE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF9D8),
            oc1549_pl_result,
            &OC1549_PL_PREV_RESULT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B4818),
            oc1549_pl_size,
            &OC1549_PL_PREV_SIZE,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B47F8),
            oc1549_pl_unpack,
            &OC1549_PL_PREV_UNPACK,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1549_pl_new,
            &OC1549_PL_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            oc1549_pl_memcpy,
            &OC1549_PL_PREV_MEMCPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1549_pl_free,
            &OC1549_PL_PREV_FREE,
        );
    }

    /// `WPFile_PollLoad` over the state, the query results and the size:
    /// every exit (not loading, not done, no result, empty result, the
    /// allocate-and-copy path and the skipped copy).
    #[test]
    fn wpfile_poll_load_matches() {
        check(
            ee::s08::f_002bb3f0__generated,
            wpfile_poll_load,
            300,
            &|c, seed| {
                oc1549_pl_hook();
                machine(c, seed, 0x2b_b3f0, 3);
                c.bus.write32(OC1549_PL_GATE, OC1549_MAGIC);
                c.bus
                    .write32(OC1549_PL_GATE + 4, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(
                    OC1549_PL_GATE + 8,
                    if (seed / 3) % 2 == 0 { 0 } else { 0x40 },
                );
                c.bus.write32(OC1549_PL_GATE + 0xc, DATA + 0x5000);
                c.bus.write32(
                    OC1549_PL_GATE + 0x10,
                    if seed % 4 == 0 { 0 } else { 0x2_0000 },
                );
                c.bus
                    .write32(OC1549_PL_GATE + 0x14, [0, 0xffff_ffff, 0x30][(seed % 3) as usize]);
                let file = DATA + 0x1000;
                c.bus
                    .write16(file, if seed % 7 == 0 { 0 } else { 1 });
                c.bus.write32(file + 4, DATA + 0x3000);
                c.bus.write32(file + 8, 0);
                c.g[A0][0] = sx(file);
                c.pc = 0x002b_b3f0;
            },
        );
    }
}
