//! Native WScriptFn_AddTexFlipModifier (0x002fe4a8), retained uninstalled.
//! No tests, builds or replays: automated validation is disabled by the user.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_e4a8,
    add_tex_flip_modifier,
    "WScriptFn_AddTexFlipModifier",
)];

const DEFAULTS: u32 = 0x004f_b818;
const PARAM_TYPES: u32 = 0x0044_6268;
const INITIALIZED_GP: u32 = 0x2530;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* AddTexFlipModifier(Vec3* out, int count, NamedArg* args)`.
/// Copy the native 24-byte defaults `[-1,0,0,0,2.0f,trailing-byte]`, then
/// apply named arguments against `[int,int,int,int,float,end]`. The first
/// argument is an instance RID; -1 selects WScriptMan's current instance.
/// With an existing node whose adjusted virtual slot +0x80 returns true,
/// call `ModObj_AddTexFlipModifier(node, params)` (0x355bf0). This wrapper
/// creates no node and does not interpret the three integer mode fields or
/// the float itself. Return the shared native zero Vec3 through `out`.
pub fn add_tex_flip_modifier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw(at(c, GP, INITIALIZED_GP));
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        let initialized = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialized {
            c.cycles += 15;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(at(c, V1, 0x14));
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x4000_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = 1;
            c.sw(r(c, A2).wrapping_sub(0x47e8), c.g[V0][0]);
            c.sb(at(c, V1, 0x14), c.g[A1][0]);
            c.swc1(0, at(c, V1, 0x10));
            c.sw(at(c, GP, INITIALIZED_GP), c.g[A0][0]);
            c.sw(at(c, V1, 4), 0);
            c.sw(at(c, V1, 8), 0);
            c.sw(at(c, V1, 0xc), 0);
            c.sb(r(c, SP), c.g[A1][0]);
        }

        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x47e8));
        c.g[A2][0] = 0x18;
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003e_6574, 0x002f_e534)?;
        let no_arguments = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_arguments {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(PARAM_TYPES);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(r(c, A2));
                c.g[V1][0] = c.lw(at(c, A2, 0xc));
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                c.g[V0][0] = c.lw(r(c, V0));
                let mut converted = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = c.lw(at(c, A2, 4));
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw(at(c, A2, 4));
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, at(c, A2, 4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r(c, A0));
                        converted = true;
                    }
                }
                if !converted {
                    c.cycles += 1;
                    c.sw(r(c, A0), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(at(c, A2, 0x10));
                if !more { break }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_e550);
                }
            }
        }

        c.cycles += 4;
        c.g[A2][0] = c.lw(r(c, S1));
        c.g[V0][0] = !c.g[A2][0];
        let explicit_instance = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw(at(c, GP, 0x16c8));
        if !explicit_instance {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, GP, 0xce8));
            c.g[V0][0] = c.lw(at(c, V0, 0x290));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r(c, V0) << 2);
            c.g[V1][0] = c.lw(r(c, A1));
            c.g[A0][0] = c.lw(at(c, V1, 8));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0));
            let absent_bank = c.g[V1][0] == 0;
            c.g[V0][0] = sx(r(c, A2) >> 8);
            if absent_bank {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = c.lw(r(c, V0));
                c.g[V0][0] = sx(r(c, V1) >> 8);
                let present = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r(c, V0) << 2);
                if !present {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
            }
        }

        c.cycles += 3;
        c.g[S0][0] = c.lw(at(c, V0, 0xc));
        let no_node = c.g[S0][0] == 0;
        c.g[V1][0] = 0x004d_0000;
        if !no_node {
            c.cycles += 7;
            c.g[V1][0] = c.lw(at(c, S0, 0xc));
            c.g[A0][0] = c.lh(at(c, V1, 0x80));
            c.g[V0][0] = c.lw(at(c, V1, 0x84));
            let target = r(c, V0);
            c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x002f_e614)?;
            let supported = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if supported {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0035_5bf0, 0x002f_e624)?;
            }
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
        }

        c.cycles += 15;
        c.g[V0][0] = c.g[S3][0];
        c.g[A3][0] = sx(r(c, V1).wrapping_sub(0x6f68));
        c.ldl(A0, at(c, A3, 7));
        c.ldr(A0, r(c, A3));
        c.g[A1][0] = c.lw(at(c, A3, 8));
        c.sdl(A0, at(c, S3, 7));
        c.sdr(A0, r(c, S3));
        c.sw(at(c, S3, 8), c.g[A1][0]);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}
