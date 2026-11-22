//! Native `WScriptFn_Modifier9` (0x002fcc20), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x002f_cc20, wscript_fn_modifier9, "WScriptFn_Modifier9")];

const DEFAULTS: u32 = 0x004f_b5b0;
const DEFAULTS_READY_GP: u32 = 0x2504;
const PARAM_TYPES: u32 = 0x0044_6028;
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

/// `Vec3* WScriptFn_Modifier9(Vec3* out, int count, NamedArg* args)`.
/// Named arguments are 16-byte `(index, value, unused, type)` records; the
/// schema is `[1,1,2,0]`. The 16-byte raw defaults are `[-1,0,0.5f]` plus
/// a preserved byte. Index 0 resolves an explicit resident instance RID,
/// or -1 selects WScriptMan's current instance. Index 1 is an integer and
/// index 2 a float, both forwarded without scaling or interpretation here.
///
/// This wrapper does not allocate a node or invoke a constructor. A resolved
/// instance must already have a WorldObj at +0xc; that owner receives
/// `AttachModifier(owner,9,params)`. The factory name alone does not establish
/// the meaning of type 9 or its parameters. Return the shared native Vec3
/// through the hidden first argument regardless of whether anything attached.
pub fn wscript_fn_modifier9(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
        frame.save(c, S3, 0x40);
        frame.save(c, S1, 0x60);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x0050_0000;
        frame.save_ra(c, 0x20);
        if c.g[V0][0] == 0 {
            c.cycles += 13;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(at(c, V1, 0xc));
            c.g[V0][0] |= 0xffff;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 0);
            c.g[A0][0] = 1;
            c.sw(r(c, A2).wrapping_sub(0x4a50), c.g[V0][0]);
            c.sb(at(c, V1, 0xc), c.g[A1][0]);
            c.swc1(0, at(c, V1, 8));
            c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
            c.sw(at(c, V1, 4), 0);
            c.sb(r(c, SP), c.g[A1][0]);
        }

        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x4a50));
        c.g[A2][0] = 0x10;
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_ccac)?;
        let no_args = (c.g[S2][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !no_args {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = sx(PARAM_TYPES);
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
                    deliver_event_here(c, 0x002f_ccc8);
                }
            }
        }

        c.cycles += 4;
        c.g[A2][0] = c.lw(r(c, S0));
        c.g[V0][0] = !c.g[A2][0];
        let explicit_rid = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw(at(c, GP, WORLD_HANDLE_GP));
        if !explicit_rid {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
            c.g[V0][0] = c.lw(at(c, V0, CURRENT_INSTANCE));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r(c, V0) << 2);
            c.g[V1][0] = c.lw(r(c, A1));
            c.g[A0][0] = c.lw(at(c, V1, 8));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0));
            let bank_absent = c.g[V1][0] == 0;
            c.g[V0][0] = sx(r(c, A2) >> 8);
            if bank_absent {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.lw(at(c, V1, 0x1c));
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
            }
        }

        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.lw(at(c, V0, INSTANCE_NODE));
            let has_owner = c.g[A0][0] != 0;
            c.g[A2][0] = c.g[S0][0];
            if has_owner {
                c.cycles += 2;
                c.g[A1][0] = 9;
                call_ptr(c, 0x002f_ae38, 0x002f_cd88)?;
            }
        }

        c.cycles += 16;
        c.g[A1][0] = sx(r(c, S4).wrapping_sub(0x6f68));
        c.ldl(V0, at(c, A1, 7));
        c.ldr(V0, r(c, A1));
        c.g[V1][0] = c.lw(at(c, A1, 8));
        c.sdl(V0, at(c, S1, 7));
        c.sdr(V0, r(c, S1));
        c.sw(at(c, S1, 8), c.g[V1][0]);
        c.g[V0][0] = c.g[S1][0];
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
