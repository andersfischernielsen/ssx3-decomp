//! Native `WScriptFn_MakeDeadNode` (0x002fc420), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_c420,
    wscript_fn_make_dead_node,
    "WScriptFn_MakeDeadNode",
)];

const DEFAULTS: u32 = 0x004f_b500;
const DEFAULTS_READY_GP: u32 = 0x24f4;
const PARAM_TYPES_GP: u32 = 0xb10;
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

/// `Vec3* WScriptFn_MakeDeadNode(Vec3* out, int count, NamedArg* args)`.
/// The named scalar schema at gp+0xb10 is `[1,1]`: parameter 0 is an
/// instance RID (-1 selects WScriptMan's current instance), parameter 1 is
/// the native dead-node mode. Defaults are `[-1,0]`, with a preserved byte
/// at +8 in the copied 12-byte block. No mode translation happens here.
///
/// Resolve the resident instance, ask an existing node to accept modifier
/// kind 6, then forward Instance* and the raw parameter block to the
/// internal `WorldObjFactory_DeadNode` (0x2fc2c0) if it was not handled.
pub fn wscript_fn_make_dead_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x0050_0000;
        frame.save_ra(c, 0x20);

        if c.g[V0][0] == 0 {
            c.cycles += 10;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(at(c, V1, 8));
            c.g[V0][0] |= 0xffff;
            c.sw(r(c, A2).wrapping_sub(0x4b00), c.g[V0][0]);
            c.g[A0][0] = 1;
            c.sb(at(c, V1, 8), c.g[A1][0]);
            c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
            c.sw(at(c, V1, 4), 0);
            c.sb(r(c, SP), c.g[A1][0]);
        }

        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x4b00));
        c.g[A2][0] = 0xc;
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_c4a0)?;
        let have_arguments = (c.g[S0][0] as i64) > 0;
        c.g[A2][0] = c.g[S3][0];
        if have_arguments {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(at(c, GP, PARAM_TYPES_GP));
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(r(c, A2));
                c.g[V1][0] = c.lw(at(c, A2, 0xc));
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                c.g[V0][0] = c.lw(r(c, V0));
                let mut convert_integer = false;
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
                        convert_integer = true;
                    }
                }
                if !convert_integer {
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
                    deliver_event_here(c, 0x002f_c4b8);
                }
            }
        }

        c.cycles += 4;
        c.g[A2][0] = c.lw(r(c, S1));
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = c.lw(at(c, GP, WORLD_HANDLE_GP));
        if c.g[V0][0] == 0 {
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
            let absent_bank = c.g[V1][0] == 0;
            c.g[A0][0] = 0;
            if !absent_bank {
                c.cycles += 8;
                c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                c.g[V0][0] = sx(r(c, A2) >> 8);
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
                c.cycles += 1;
                c.g[A0][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A0][0];
        }

        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.lw(at(c, S0, INSTANCE_NODE));
            let has_owner = c.g[A0][0] != 0;
            c.g[A1][0] = 6;
            let mut handled = false;
            if has_owner {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x002f_ae38, 0x002f_c584)?;
                handled = c.g[V0][0] != 0;
            }
            if !handled {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x002f_c2c0, 0x002f_c598)?;
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
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}
