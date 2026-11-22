//! Native `WorldObjFactory_Debounce` (0x002fc7d0), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_c7d0,
    world_obj_factory_debounce,
    "WorldObjFactory_Debounce",
)];

const DEFAULTS: u32 = 0x004f_b528;
const DEFAULTS_READY_GP: u32 = 0x24fc;
const PARAM_TYPES: u32 = 0x0044_5fb0;
const WORLD_HANDLE_GP: u32 = 0x16c8;
const SCRIPT_MANAGER_GP: u32 = 0xce8;
const CURRENT_INSTANCE: u32 = 0x290;
const INSTANCE_NODE: u32 = 0xc;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* WorldObjFactory_Debounce(Vec3* out, int count, NamedArg* args)`.
/// Named arguments are 16-byte `(index, value, unused, type)` records; the
/// native schema is `[1,2,1,1,1]`. Defaults are `[-1,1.0f,1,-1]`, followed
/// by a preserved byte. Index 0 resolves an explicit resident instance RID,
/// or -1 selects WScriptMan's current instance. Index 1 is forwarded as a
/// float without any scaling here; its units are owned by the constructor.
///
/// Ask an existing owner to accept modifier kind 0. If it does not, allocate
/// 0x34 bytes and invoke `Debounce_ctor(allocated,1,instance,params,0)` at
/// 0x342c08. Return the native shared Vec3 through the hidden first argument.
pub fn world_obj_factory_debounce(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
        frame.save(c, S0, 0x80);
        c.g[A3][0] = 0x0050_0000;
        frame.save(c, S2, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S4, 0x40);
        frame.save_ra(c, 0x30);

        if c.g[V0][0] == 0 {
            c.cycles += 15;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A2][0] = c.lbu(at(c, V1, 0x10));
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = u64::MAX;
            c.sw(r(c, A3).wrapping_sub(0x4ad8), c.g[V0][0]);
            c.g[A1][0] = 1;
            c.sb(at(c, V1, 0x10), c.g[A2][0]);
            c.swc1(0, at(c, V1, 4));
            c.sw(at(c, V1, 0xc), c.g[A0][0]);
            c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A1][0]);
            c.sw(at(c, V1, 8), c.g[A1][0]);
            c.sb(r(c, SP), c.g[A2][0]);
        }

        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 0x14;
        c.g[A1][0] = sx(r(c, A3).wrapping_sub(0x4ad8));
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_c864)?;
        let no_args = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !no_args {
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
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c880);
                }
            }
        }

        c.cycles += 4;
        c.g[A2][0] = c.lw(r(c, S1));
        c.g[V0][0] = !c.g[A2][0];
        let explicit_rid = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw(at(c, GP, WORLD_HANDLE_GP));
        if !explicit_rid {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
            c.g[S0][0] = c.lw(at(c, V0, CURRENT_INSTANCE));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r(c, V0) << 2);
            c.g[V1][0] = c.lw(r(c, A1));
            c.g[A0][0] = c.lw(at(c, V1, 8));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0));
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.g[A2][0] = 0;
            } else {
                c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                c.cycles += 7;
                c.g[V0][0] = sx(r(c, A2) >> 8);
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = c.lw(r(c, V0));
                c.g[V0][0] = sx(r(c, V1) >> 8);
                let live = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r(c, V0) << 2);
                if !live {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }

        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.lw(at(c, S0, INSTANCE_NODE));
            let has_owner = c.g[A0][0] != 0;
            c.g[A1][0] = 0;
            let mut handled = false;
            if has_owner {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x002f_ae38, 0x002f_c950)?;
                handled = c.g[V0][0] != 0;
            }
            if !handled {
                c.cycles += 12;
                c.g[A1][0] = 0x0049_0000;
                c.g[A0][0] = 0x34;
                c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6918));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x002f_c970)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = 0;
                call_ptr(c, 0x0034_2c08, 0x002f_c988)?;
            }
        }

        c.cycles += 16;
        c.g[A1][0] = sx(r(c, S4).wrapping_sub(0x6f68));
        c.ldl(V0, at(c, A1, 7));
        c.ldr(V0, r(c, A1));
        c.g[V1][0] = c.lw(at(c, A1, 8));
        c.sdl(V0, at(c, S2, 7));
        c.sdr(V0, r(c, S2));
        c.sw(at(c, S2, 8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}
