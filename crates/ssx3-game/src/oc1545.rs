//! Staging module oc1545 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1545`.

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
    (0x0025_61e0, dnas_auth_dtor, "DNASAuth_dtor"),
    (0x0025_6258, dnas_auth_start, "DNASAuth_Start"),
    (0x0025_62d0, dnas_auth_begin_auth, "DNASAuth_BeginAuth"),
    (0x0025_63a8, dnas_auth_free_buffers, "DNASAuth_FreeBuffers"),
    (0x0025_6460, dnas_auth_update_load_overlay, "DNASAuth_UpdateLoadOverlay"),
    (0x0025_6510, dnas_auth_update_load_config, "DNASAuth_UpdateLoadConfig"),
    (0x0025_65e0, dnas_auth_update_auth, "DNASAuth_UpdateAuth"),
    (0x0026_88e0, c_private_message_dtor, "cPrivateMessage_dtor"),
];

/// The address `off` bytes past the pointer in register `reg` (read after a
/// call: callee-saved registers may only be trusted from the register).
fn oc1545_field(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `void DNASAuth::~DNASAuth(this, flags)` (0x2561e0): runs the hook that
/// does nothing, then frees `this` when bit 0 of `flags` is set.
pub fn dnas_auth_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Callee {
            addr: 0x0025_6250,
            f: ee::s06::f_00256250,
        }
        .call(c, 0x0025_61fc)?;
        c.g[S0][0] &= 0x1;
        let free = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0025_6210)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth_Start(DNASAuth* self, int arg)` (0x256258): records `arg`,
/// clears the two buffer pointers, allocates the 0x195000-byte block
/// (`operator_new`) and loads the DNAS data file into +0x18.
pub fn dnas_auth_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x0400_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let arg = c.g[A1][0];
        Addr(oc1545_field(c, S0, 0x20)).sw(c, arg);
        c.g[A0][0] = 0x19_0000;
        Addr(oc1545_field(c, S0, 0x14)).sw(c, 0);
        c.g[A1][0] = 0x4a_0000;
        Addr(oc1545_field(c, S0, 0x1c)).sw(c, 0);
        c.g[A0][0] |= 0x5000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ee0));
        let one = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0)).sw(c, one);
        c.g[A2][0] |= 0x100;
        call_ptr(c, 0x0031_7e30, 0x0025_629c)?;
        c.g[A0][0] = 0x48_0000;
        c.g[A2][0] = 0x19_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e8));
        c.g[A1][0] = c.g[V0][0];
        let block = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0x4)).sw(c, block);
        c.g[A2][0] |= 0x5000;
        call_ptr(c, 0x003d_f748, 0x0025_62b8)?;
        let file = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0x18)).sw(c, file);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::BeginAuth(this)` (0x2562d0): clears the 0xa0-byte request
/// block at +0x24, copies the two header words and the MAC address into it,
/// and starts the DNAS release with `DnasRel_CallInit`.
pub fn dnas_auth_begin_auth(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0xa0;
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(oc1545_field(c, S0, 0x98));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(oc1545_field(c, S0, 0x24));
        Addr(oc1545_field(c, S0, 0x1c)).sw(c, 0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003e_6448, 0x0025_6304)?;
        c.g[V1][0] = Addr(oc1545_field(c, S0, 0xc)).lw(c);
        c.g[A0][0] = 8;
        c.g[V0][0] = Addr(oc1545_field(c, S0, 0x10)).lw(c);
        let v1 = c.g[V1][0];
        Addr(oc1545_field(c, S0, 0x6c)).sw(c, v1);
        let v0 = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0x70)).sw(c, v0);
        Addr(oc1545_field(c, S0, 0x90)).sw(c, 0);
        Addr(oc1545_field(c, S0, 0x8c)).sw(c, 0);
        c.ldl(2, (c.g[GP][0] as u32).wrapping_sub(0x211));
        c.ldr(2, (c.g[GP][0] as u32).wrapping_sub(0x218));
        c.sdl(2, oc1545_field(c, S0, 0x7b));
        c.sdr(2, oc1545_field(c, S0, 0x74));
        let eight = c.g[A0][0];
        Addr(oc1545_field(c, S0, 0x84)).sw(c, eight);
        call_ptr(c, 0x003f_3f78, 0x0025_6338)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0041_66f4, 0x0025_6344)?;
        c.g[V1][0] = 0x48_0000;
        c.g[A2][0] = 1;
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3f8));
        c.ldl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(7, c.g[A1][0] as u32);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lh(c);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xa)).lb(c);
        c.sdl(7, oc1545_field(c, S0, 0xaf));
        c.sdr(7, oc1545_field(c, S0, 0xa8));
        let half = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0xb0)).sh(c, half);
        let byte = c.g[A0][0];
        Addr(oc1545_field(c, S0, 0xb2)).sb(c, byte);
        c.g[A1][0] = 4;
        Addr(oc1545_field(c, S0, 0xb8)).sw(c, 0);
        c.g[V0][0] = Addr(oc1545_field(c, S0, 0x20)).lw(c);
        c.g[A0][0] = c.g[S2][0];
        let len = c.g[A1][0];
        Addr(oc1545_field(c, S0, 0)).sw(c, len);
        let v0 = c.g[V0][0];
        Addr(oc1545_field(c, S0, 0xbc)).sw(c, v0);
        let flag = c.g[A2][0];
        Addr(oc1545_field(c, S0, 0xc0)).sw(c, flag);
        call_ptr(c, 0x003f_d0d8, 0x0025_6390)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::FreeBuffers(this)` (0x2563a8): frees the buffer at +0xc
/// and then the one at +4, clearing each pointer that was freed.
pub fn dnas_auth_free_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(oc1545_field(c, S0, 0xc)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x4)).lw(c);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x0025_63cc)?;
            Addr(oc1545_field(c, S0, 0xc)).sw(c, 0);
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x4)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x0025_63e4)?;
            Addr(oc1545_field(c, S0, 0x4)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::UpdateLoadOverlay(this)` (0x256460): once the load at +0x18
/// is done, reads its size into +8 and, if there is data, loads the config
/// buffer (+0xc) with `DnasRel_Load`'s result checked and allocates it.
pub fn dnas_auth_update_load_overlay(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(oc1545_field(c, S0, 0x18)).lw(c);
        call_ptr(c, 0x003d_f980, 0x0025_6478)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 8;
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x18)).lw(c);
            c.g[A1][0] = 0;
            Addr(oc1545_field(c, S0, 0x8)).sw(c, 0);
            c.g[A2][0] = sx(oc1545_field(c, S0, 0x8));
            call_ptr(c, 0x003d_f9d8, 0x0025_6498)?;
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x4)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = Addr(oc1545_field(c, S0, 0x8)).lw(c);
                if (c.g[A1][0] as i64) > 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x003f_cfd8, 0x0025_64b8)?;
                    let failed = (c.g[V0][0] as i64) < 0;
                    c.g[A1][0] = 0x48_0000;
                    if !failed {
                        c.cycles += 15;
                        c.g[A2][0] = 0x400_0000;
                        c.g[A0][0] = 0x1_0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x428));
                        c.g[A2][0] |= 0x100;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7e30, 0x0025_64d8)?;
                        c.g[A0][0] = 0x48_0000;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x438));
                        let buf = c.g[V0][0];
                        Addr(oc1545_field(c, S0, 0xc)).sw(c, buf);
                        c.g[A2][0] = 0x1_0000;
                        call_ptr(c, 0x003d_f748, 0x0025_64f0)?;
                        c.g[V1][0] = 2;
                        let file = c.g[V0][0];
                        Addr(oc1545_field(c, S0, 0x18)).sw(c, file);
                        let state = c.g[V1][0];
                        Addr(oc1545_field(c, S0, 0)).sw(c, state);
                    }
                    c.cycles += 1;
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::UpdateLoadConfig(this)` (0x256510): once the config load
/// at +0x18 is done, reads its entry count into +0x10 and XORs that many bytes
/// of the buffer at +0xc with a stack-derived key (a byte loop with event
/// checks at its back edge).
pub fn dnas_auth_update_load_config(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(oc1545_field(c, S0, 0x18)).lw(c);
        call_ptr(c, 0x003d_f980, 0x0025_6528)?;
        c.g[V1][0] = 1;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 8;
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0x18)).lw(c);
            c.g[A1][0] = 0;
            Addr(oc1545_field(c, S0, 0x10)).sw(c, 0);
            c.g[A2][0] = sx(oc1545_field(c, S0, 0x10));
            call_ptr(c, 0x003d_f9d8, 0x0025_6548)?;
            c.g[A0][0] = Addr(oc1545_field(c, S0, 0xc)).lw(c);
            if c.g[A0][0] != 0 {
                'loop_block: {
                    c.cycles += 3;
                    c.g[A3][0] = Addr(oc1545_field(c, S0, 0x10)).lw(c);
                    let empty = (c.g[A3][0] as i64) <= 0;
                    c.g[V0][0] = 3;
                    if empty {
                        break 'loop_block;
                    }
                    c.cycles += 10;
                    c.g[V1][0] = 0x4a_0000;
                    let mode = c.g[V0][0];
                    Addr(oc1545_field(c, S0, 0)).sw(c, mode);
                    c.g[A2][0] = c.g[A0][0];
                    c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2ee8));
                    c.lwl(2, (c.g[T1][0] as u32).wrapping_add(0x3));
                    c.lwr(2, c.g[T1][0] as u32);
                    c.swl(2, sp(c).wrapping_add(0x3));
                    c.swr(2, sp(c));
                    let none = (c.g[A3][0] as i64) <= 0;
                    c.g[A1][0] = 0;
                    if none {
                        break 'loop_block;
                    }
                    c.cycles += 2;
                    c.g[A3][0] = u64::MAX;
                    loop {
                        c.cycles += 16;
                        c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3));
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = c.g[A1][0];
                        }
                        c.g[A0][0] = Addr(c.g[A2][0] as u32).lbu(c);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V1][0] = sx(sp(c).wrapping_add(c.g[V0][0] as u32));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
                        c.g[A0][0] ^= c.g[V0][0];
                        let key = c.g[A0][0];
                        Addr(c.g[A2][0] as u32).sb(c, key);
                        c.g[V0][0] = Addr(oc1545_field(c, S0, 0x10)).lw(c);
                        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        if !more {
                            break 'loop_block;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_6590);
                        }
                    }
                }
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void DNASAuth::UpdateAuth(this)` (0x2565e0): runs the release's update
/// step; once it reports done, clears the three strings of the release, calls
/// `DnasRel_CallExit`, copies three strings and sets the state to 5.
pub fn dnas_auth_update_auth(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(oc1545_field(c, S3, 0x14)).lw(c);
        call_ptr(c, 0x003f_d100, 0x0025_6604)?;
        if c.g[V0][0] == 0 {
            c.cycles += 24;
            c.g[S0][0] = sx(oc1545_field(c, S3, 0xc4));
            c.g[A2][0] = 0x38c;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6448, 0x0025_6620)?;
            c.g[S1][0] = sx(oc1545_field(c, S3, 0x650));
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx(oc1545_field(c, S3, 0x850));
            call_ptr(c, 0x003f_d128, 0x0025_6630)?;
            Addr(oc1545_field(c, S3, 0x450)).sh(c, 0);
            c.g[S0][0] = c.g[V0][0];
            Addr(oc1545_field(c, S3, 0x650)).sh(c, 0);
            c.g[A0][0] = sx(oc1545_field(c, S3, 0x450));
            Addr(oc1545_field(c, S3, 0x850)).sh(c, 0);
            c.g[A1][0] = sx(oc1545_field(c, S3, 0xcc));
            call_ptr(c, 0x003d_be20, 0x0025_664c)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx(oc1545_field(c, S3, 0x10c));
            call_ptr(c, 0x003d_be20, 0x0025_6658)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx(oc1545_field(c, S3, 0x20c));
            call_ptr(c, 0x003d_be20, 0x0025_6664)?;
            if c.g[S0][0] != 0 {
                Addr(oc1545_field(c, S3, 0x1c)).sw(c, 0);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                let ready = c.g[V0][0];
                Addr(oc1545_field(c, S3, 0x1c)).sw(c, ready);
            }
            c.cycles += 3;
            c.g[V0][0] = 5;
            let stage = c.g[V0][0];
            Addr(oc1545_field(c, S3, 0)).sw(c, stage);
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cPrivateMessage::~cPrivateMessage(this, flags)` (0x2688e0): sets the
/// vtable word, destroys the three strings at +8, +4 and +0 and frees `this`
/// when bit 0 of `flags` is set.
pub fn c_private_message_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2740));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        Addr(oc1545_field(c, S1, 0xc)).sw(c, vtable);
        c.g[A0][0] = sx(oc1545_field(c, S1, 0x8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8910)?;
        c.g[A0][0] = sx(oc1545_field(c, S1, 0x4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_891c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8928)?;
        c.g[S0][0] &= 0x1;
        let free = c.g[S0][0] != 0;
        // The `lq s0` sits in the branch delay slot: s0 is already restored
        // when `operator_delete` is called.
        frame.restore(c, S0, 0x20);
        if free {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_893c)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (own address and magic in the
    /// machine's RAM, so other tests' machines run the real bodies). Mailbox
    /// `i` is a flag word at `+0x10 + 8i` and a value word at `+0x14 + 8i`:
    /// a set flag makes the fake return the value (sign-extended).
    const OC1545_CTRL: u32 = DATA + 0x2d_0000;
    const OC1545_MAGIC: u32 = 0x1545_f00d;
    const OC1545_OBJ: u32 = DATA + 0x1000;
    const OC1545_BUF: u32 = DATA + 0x1200;

    /// Mailbox indices of the callee fakes.
    const MB_NEW: u32 = 0;
    const MB_IS_DONE: u32 = 5;
    /// `AsyncFile_GetResult`: its value is written to the pointer in `a2`.
    const MB_RESULT: u32 = 6;
    const MB_CALL_UPDATE: u32 = 12;
    const MB_CALL_EXIT: u32 = 13;
    const MB_LOAD_DNAS: u32 = 14;

    /// The logged call; a set mailbox flag makes it return the mailbox value.
    fn oc1545_fake(c: &mut Cpu, target: u32, idx: u32) {
        fakes::fake_call(c, target);
        if c.bus.read32(OC1545_CTRL + 0x10 + 8 * idx) != 0 {
            c.g[V0][0] = sx(c.bus.read32(OC1545_CTRL + 0x14 + 8 * idx));
        }
    }

    /// Forces the result of the callee with mailbox `idx`.
    fn oc1545_result(c: &mut Cpu, idx: u32, value: u32) {
        c.bus.write32(OC1545_CTRL + 0x10 + 8 * idx, 1);
        c.bus.write32(OC1545_CTRL + 0x14 + 8 * idx, value);
    }

    fn oc1545_fake_317e30(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_317e30), ee::s10::f_00317e30__generated);
        }
        oc1545_fake(c, 0x0031_7e30, MB_NEW);
    }

    fn oc1545_fake_317e50(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_317e50), ee::s10::f_00317e50__generated);
        }
        oc1545_fake(c, 0x0031_7e50, 1);
    }

    fn oc1545_fake_317e98(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_317e98), ee::s10::f_00317e98__generated);
        }
        oc1545_fake(c, 0x0031_7e98, 2);
    }

    fn oc1545_fake_318260(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_318260), ee::s10::f_00318260__generated);
        }
        oc1545_fake(c, 0x0031_8260, 3);
    }

    fn oc1545_fake_3df748(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3df748), ee::s14::f_003df748__generated);
        }
        oc1545_fake(c, 0x003d_f748, 4);
    }

    fn oc1545_fake_3df980(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3df980), ee::s14::f_003df980__generated);
        }
        oc1545_fake(c, 0x003d_f980, MB_IS_DONE);
    }

    fn oc1545_fake_3df9d8(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3df9d8), ee::s14::f_003df9d8__generated);
        }
        let out = c.g[A2][0] as u32;
        let value = c.bus.read32(OC1545_CTRL + 0x14 + 8 * MB_RESULT);
        oc1545_fake(c, 0x003d_f9d8, MB_RESULT);
        c.bus.write32(out, value);
    }

    fn oc1545_fake_3e6448(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3e6448), ee::s14::f_003e6448__generated);
        }
        oc1545_fake(c, 0x003e_6448, 7);
    }

    fn oc1545_fake_3f3f78(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3f3f78), ee::s14::f_003f3f78__generated);
        }
        oc1545_fake(c, 0x003f_3f78, 8);
    }

    fn oc1545_fake_3dbe20(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3dbe20), ee::s14::f_003dbe20__generated);
        }
        oc1545_fake(c, 0x003d_be20, 9);
    }

    fn oc1545_fake_4166f4(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_4166f4), ee::s15::f_004166f4__generated);
        }
        oc1545_fake(c, 0x0041_66f4, 10);
    }

    fn oc1545_fake_3fd0d8(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3fd0d8), ee::s15::f_003fd0d8__generated);
        }
        oc1545_fake(c, 0x003f_d0d8, 11);
    }

    fn oc1545_fake_3fd100(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3fd100), ee::s15::f_003fd100__generated);
        }
        oc1545_fake(c, 0x003f_d100, MB_CALL_UPDATE);
    }

    fn oc1545_fake_3fd128(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3fd128), ee::s15::f_003fd128__generated);
        }
        oc1545_fake(c, 0x003f_d128, MB_CALL_EXIT);
    }

    fn oc1545_fake_3fcfd8(c: &mut Cpu) {
        if c.bus.read32(OC1545_CTRL) != OC1545_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1545_fake_3fcfd8), ee::s15::f_003fcfd8__generated);
        }
        oc1545_fake(c, 0x003f_cfd8, MB_LOAD_DNAS);
    }

    /// Puts every fake in its override slot (once per process).
    fn oc1545_hook() {
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00317E30), oc1545_fake_317e30, fakes::prev_cell(oc1545_fake_317e30));
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00317E50), oc1545_fake_317e50, fakes::prev_cell(oc1545_fake_317e50));
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00317E98), oc1545_fake_317e98, fakes::prev_cell(oc1545_fake_317e98));
        fakes::hook(addr_of_mut!(ee::s10::OVERRIDE_00318260), oc1545_fake_318260, fakes::prev_cell(oc1545_fake_318260));
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003DF748), oc1545_fake_3df748, fakes::prev_cell(oc1545_fake_3df748));
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003DF980), oc1545_fake_3df980, fakes::prev_cell(oc1545_fake_3df980));
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003DF9D8), oc1545_fake_3df9d8, fakes::prev_cell(oc1545_fake_3df9d8));
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003E6448), oc1545_fake_3e6448, fakes::prev_cell(oc1545_fake_3e6448));
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003F3F78), oc1545_fake_3f3f78, fakes::prev_cell(oc1545_fake_3f3f78));
        fakes::hook(addr_of_mut!(ee::s14::OVERRIDE_003DBE20), oc1545_fake_3dbe20, fakes::prev_cell(oc1545_fake_3dbe20));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_004166F4), oc1545_fake_4166f4, fakes::prev_cell(oc1545_fake_4166f4));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_003FD0D8), oc1545_fake_3fd0d8, fakes::prev_cell(oc1545_fake_3fd0d8));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_003FD100), oc1545_fake_3fd100, fakes::prev_cell(oc1545_fake_3fd100));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_003FD128), oc1545_fake_3fd128, fakes::prev_cell(oc1545_fake_3fd128));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_003FCFD8), oc1545_fake_3fcfd8, fakes::prev_cell(oc1545_fake_3fcfd8));
    }

    /// Hooked fakes, scrambled registers, the gate on.
    fn oc1545_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc1545_hook();
        let _ = machine(c, seed, salt, calls);
        c.bus.write32(OC1545_CTRL, OC1545_MAGIC);
    }

    /// `DNASAuth::~DNASAuth`: both flag values, so both paths free or not.
    #[test]
    fn dnas_auth_dtor_matches() {
        check(ee::s06::f_002561e0__generated, dnas_auth_dtor, 200, &|c, seed| {
            oc1545_machine(c, seed, 0x2561e0, 3);
            c.g[A0][0] = OC1545_OBJ as u64;
            c.g[A1][0] = seed as u64;
        });
    }

    /// `DNASAuth_Start`: the operator_new result set or the default.
    #[test]
    fn dnas_auth_start_matches() {
        check(ee::s06::f_00256258__generated, dnas_auth_start, 200, &|c, seed| {
            oc1545_machine(c, seed, 0x256258, 3);
            if seed % 3 == 0 {
                oc1545_result(c, MB_NEW, OC1545_OBJ + 0x2000);
            }
            c.g[A0][0] = OC1545_OBJ as u64;
            c.g[A1][0] = seed as u64 * 0x101;
        });
    }

    /// `DNASAuth_BeginAuth`: random request fields, so every copy is exercised.
    #[test]
    fn dnas_auth_begin_auth_matches() {
        check(ee::s06::f_002562d0__generated, dnas_auth_begin_auth, 200, &|c, seed| {
            oc1545_machine(c, seed, 0x2562d0, 3);
            let mut r = Rng(0x2562_d0 ^ seed as u64);
            fill(c, &mut r, OC1545_OBJ, 0x80);
            c.g[A0][0] = OC1545_OBJ as u64;
        });
    }

    /// `DNASAuth_FreeBuffers`: each of the two pointers is either set or not.
    #[test]
    fn dnas_auth_free_buffers_matches() {
        check(ee::s06::f_002563a8__generated, dnas_auth_free_buffers, 200, &|c, seed| {
            oc1545_machine(c, seed, 0x2563a8, 3);
            c.bus.write32(OC1545_OBJ + 0xc, if seed & 1 != 0 { DATA + 0x3000 } else { 0 });
            c.bus.write32(OC1545_OBJ + 0x4, if seed & 2 != 0 { DATA + 0x3100 } else { 0 });
            c.g[A0][0] = OC1545_OBJ as u64;
        });
    }

    /// `DNASAuth_UpdateLoadOverlay`: the load done or not, the size zero or
    /// positive, the buffer set or not, `DnasRel_Load` failing or not.
    #[test]
    fn dnas_auth_update_load_overlay_matches() {
        check(ee::s06::f_00256460__generated, dnas_auth_update_load_overlay, 240, &|c, seed| {
            oc1545_machine(c, seed, 0x256460, 3);
            let mut r = Rng(0x2564_60 ^ seed as u64);
            fill(c, &mut r, OC1545_OBJ, 0x10);
            c.bus.write32(OC1545_OBJ + 0x4, if seed & 1 != 0 { DATA + 0x3000 } else { 0 });
            oc1545_result(c, MB_IS_DONE, if seed % 4 == 0 { 0 } else { 1 });
            oc1545_result(c, MB_RESULT, seed % 5);
            oc1545_result(c, MB_LOAD_DNAS, if seed % 3 == 0 { 0xffff_ffff } else { 1 });
            c.g[A0][0] = OC1545_OBJ as u64;
        });
    }

    /// `DNASAuth_UpdateLoadConfig`: the entry count 0..11 (the byte loop runs
    /// for the non-zero ones), the buffer set or not.
    #[test]
    fn dnas_auth_update_load_config_matches() {
        check(ee::s06::f_00256510__generated, dnas_auth_update_load_config, 240, &|c, seed| {
            oc1545_machine(c, seed, 0x256510, 3);
            let mut r = Rng(0x2565_10 ^ seed as u64);
            fill(c, &mut r, OC1545_BUF, 0x20);
            c.bus.write32(OC1545_OBJ + 0xc, if seed & 1 != 0 { OC1545_BUF } else { 0 });
            oc1545_result(c, MB_IS_DONE, if seed % 4 == 0 { 0 } else { 1 });
            oc1545_result(c, MB_RESULT, seed % 12);
            c.g[A0][0] = OC1545_OBJ as u64;
        });
    }

    /// `DNASAuth_UpdateAuth`: the update step busy or done, the exit result
    /// zero or not.
    #[test]
    fn dnas_auth_update_auth_matches() {
        check(ee::s06::f_002565e0__generated, dnas_auth_update_auth, 200, &|c, seed| {
            oc1545_machine(c, seed, 0x2565e0, 3);
            oc1545_result(c, MB_CALL_UPDATE, if seed % 3 == 0 { 1 } else { 0 });
            oc1545_result(c, MB_CALL_EXIT, seed % 2);
            c.g[A0][0] = OC1545_OBJ as u64;
        });
    }

    /// `cPrivateMessage::~cPrivateMessage`: with and without the free.
    #[test]
    fn c_private_message_dtor_matches() {
        check(ee::s07::f_002688e0__generated, c_private_message_dtor, 200, &|c, seed| {
            oc1545_machine(c, seed, 0x2688e0, 3);
            c.g[A0][0] = OC1545_OBJ as u64;
            c.g[A1][0] = seed as u64;
        });
    }
}
