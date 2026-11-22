//! Native Emitter_Setup (0x003705e0), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game runs or replays were
//! added or run, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    emitter_setup,
    "Emitter_Setup",
)];

const ENTRY: u32 = 0x0037_05e0;
const AFTER_INITIALIZE: u32 = 0x0037_06cc;
const AFTER_BUILD: u32 = 0x0037_06e8;
const END: u32 = 0x0037_0704;

const COUNT: u32 = 0;
const TRAIL_COPIES: u32 = 4;
const EMISSION_WINDOW: u32 = 8;
const TIME_SCALE: u32 = 0x0c;
const LIFE_CENTER: u32 = 0x14;
const LIFE_SPAN: u32 = 0x1c;
const TRAIL_SPACING: u32 = 0x20;
const GRAVITY: u32 = 0x78;
const SPRITE: u32 = 0xc4;
const BLEND: u32 = 0xc8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void Emitter_Setup(Emitter* self, Matrix* matrix, Description* desc)`.
/// Copy sprite/blend words unchanged. A negative emission window remains an
/// indefinite emitter lifetime; otherwise add the life centre and half its
/// span. Initialize the descriptor at self+0x10 with count, trail copies,
/// time scale, trail spacing and gravity, then build it from the complete
/// source description and matrix, with output at self+0x160.
///
/// Colours at desc+0x84..0xc0 are only forwarded, not interpreted here.
/// This function neither resolves a sprite image nor remaps blend IDs.
pub fn emitter_setup(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 22;
                    Frame::open(c, 0x60);
                    c.mtc1(ZERO, 1);
                    frame.save(c, S1, 0x40);
                    frame.save(c, S2, 0x30);
                    c.g[S1][0] = c.g[A2][0];
                    frame.save(c, S3, 0x20);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S0, 0x50);
                    frame.save_ra(c, 0x10);
                    c.g[V0][0] = c.lw(at(c, S1, 0xd0));
                    c.sw(at(c, S2, 0x180), c.g[V0][0]);
                    c.lwc1(0, at(c, S1, 0xd4));
                    c.swc1(0, at(c, S2, 0x188));
                    c.g[V0][0] = c.lw(at(c, S1, SPRITE));
                    c.sw(at(c, S2, 4), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S1, BLEND));
                    c.sw(at(c, S2, 8), c.g[V1][0]);
                    c.lwc1(3, at(c, S1, EMISSION_WINDOW));
                    c.c_lt_s(3, 1);
                    let indefinite = c.fcc();
                    c.g[S3][0] = c.g[A1][0];
                    if indefinite {
                        c.cycles += 2;
                        c.mov_s(0, 3);
                    } else {
                        c.cycles += 7;
                        c.lwc1(0, at(c, S1, LIFE_CENTER));
                        c.lwc1(1, at(c, S1, LIFE_SPAN));
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 2);
                        c.add_s(0, 3, 0);
                        c.mul_s(1, 1, 2);
                        c.add_s(0, 0, 1);
                    }

                    c.cycles += 7;
                    c.swc1(0, r(c, S2));
                    c.mtc1(ZERO, 0);
                    c.lwc1(12, at(c, S1, EMISSION_WINDOW));
                    c.c_le_s(0, 12);
                    if !c.fcc() {
                        c.cycles += 7;
                        c.lwc1(0, at(c, S1, LIFE_SPAN));
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 1);
                        c.lwc1(2, at(c, S1, LIFE_CENTER));
                        c.mul_s(0, 0, 1);
                        c.add_s(12, 2, 0);
                    }
                    c.lwc1(2, at(c, S1, GRAVITY));

                    c.cycles += 28;
                    c.g[S0][0] = sx(at(c, S2, 0x10));
                    c.lwc1(1, at(c, S1, GRAVITY + 4));
                    c.g[A0][0] = c.g[S0][0];
                    c.lwc1(0, at(c, S1, GRAVITY + 8));
                    c.g[A3][0] = c.g[SP][0];
                    c.g[A1][0] = c.lw(at(c, S1, COUNT));
                    c.g[A2][0] = c.lw(at(c, S1, TRAIL_COPIES));
                    c.lwc1(13, at(c, S1, TRAIL_SPACING));
                    c.lwc1(14, at(c, S1, TIME_SCALE));
                    c.swc1(2, r(c, SP));
                    c.swc1(1, at(c, SP, 4));
                    c.swc1(0, at(c, SP, 8));
                    c.sw(at(c, SP, 0x0c), 0);
                    call_ptr(c, 0x0036_cbf8, AFTER_INITIALIZE)?;
                    pc = AFTER_INITIALIZE;
                }
                AFTER_INITIALIZE => {
                    c.g[AT][0] = sx(0xbf80_0000);
                    c.mtc1(AT, 12);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = sx(at(c, S2, 0x160));
                    call_ptr(c, 0x0037_0058, AFTER_BUILD)?;
                    pc = AFTER_BUILD;
                }
                AFTER_BUILD => {
                    frame.restore(c, S0, 0x50);
                    frame.restore(c, S1, 0x40);
                    frame.restore(c, S2, 0x30);
                    frame.restore(c, S3, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
