//! Native `WScriptFn_SetRailEnabled` (0x00300b20), retained uninstalled.
//!
//! Generated EE is the authority. No tests, builds or replays are run here
//! because automated validation is disabled by the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0030_0b20,
    wscript_fn_set_rail_enabled,
    "WScriptFn_SetRailEnabled",
)];

const DEFAULTS: u32 = 0x004f_bb48;
const DEFAULTS_READY_GP: u32 = 0x25b4;
const PARAM_TYPES_GP: u32 = 0xb60;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* WScriptFn_SetRailEnabled(Vec3* out, int count, NamedArg* args)`.
/// The native named-argument schema is `[1,1]` with defaults `[-1,1]`.
/// Parameter 0 is forwarded unchanged to 0x358b28; this wrapper does not
/// resolve an instance or substitute the current script owner. A value of
/// -1 skips the operation. Parameter 1 must be exactly 1 (enable) or 0
/// (disable); every other value is a no-op. The downstream function owns
/// interpretation of the rail identifier and its resource lookup.
pub fn wscript_fn_set_rail_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x0050_0000;

        if c.g[V0][0] == 0 {
            c.cycles += 10;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(at(c, V1, 8));
            c.g[V0][0] |= 0xffff;
            c.sw(r(c, A2).wrapping_sub(0x44b8), c.g[V0][0]);
            c.g[A0][0] = 1;
            c.sb(at(c, V1, 8), c.g[A1][0]);
            c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
            c.sw(at(c, V1, 4), c.g[A0][0]);
            c.sb(r(c, SP), c.g[A1][0]);
        }

        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x44b8));
        c.g[A2][0] = 0xc;
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_0b98)?;
        let have_arguments = (c.g[S1][0] as i64) > 0;
        c.g[A2][0] = c.g[S2][0];
        if have_arguments {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            c.g[T0][0] = sx(at(c, GP, PARAM_TYPES_GP));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(r(c, A2));
                c.g[V1][0] = c.lw(at(c, A2, 0xc));
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
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
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_0bb0);
                }
            }
        }

        c.cycles += 4;
        c.g[A0][0] = c.lw(r(c, S0));
        c.g[V0][0] = !c.g[A0][0];
        let no_rail = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if no_rail {
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
        } else {
            c.cycles += 3;
            c.g[V1][0] = c.lw(at(c, S0, 4));
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0035_8b28, 0x0030_0c1c)?;
                c.g[V1][0] = 0x004d_0000;
            } else {
                c.cycles += 2;
                let unsupported_mode = c.g[V1][0] != 0;
                c.g[V1][0] = 0x004d_0000;
                if !unsupported_mode {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0035_8b28, 0x0030_0c34)?;
                    c.cycles += 1;
                    c.g[V1][0] = 0x004d_0000;
                }
            }
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
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}
