//! Native WScriptFn_PlaceRiderAtObject (0x00300948), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0030_0948,
    place_rider_at_object,
    "WScriptFn_PlaceRiderAtObject",
)];

const DEFAULTS_GP: u32 = 0x25a8;
const DEFAULTS_READY_GP: u32 = 0x25b0;
const PARAM_TYPES_GP: u32 = 0x0b58;
const WORLD_HANDLE_GP: u32 = 0x16c8;
const SCRIPT_MANAGER_GP: u32 = 0x0ce8;
const CURRENT_INSTANCE: u32 = 0x290;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* PlaceRiderAtObject(Vec3* out, int count, NamedArg* arguments)`.
/// Native schema at gp+0xb58 is `[1,0]`: instance RID and unused trailing
/// byte. The RID defaults to -1 (WScriptMan's current instance); explicit
/// handles resolve only an already resident type-3 reference.
///
/// The script manager's first word is a signed rider INDEX, not a Rider*.
/// For a valid index and instance, obtain its placement matrix from the
/// static instance (+0x10), or its node's adjusted virtual slot +0xc0. Then
/// select `AI->riders[index]` (AI+0x28) and invoke the rider interface's
/// adjusted slot +0x60 with that matrix as A1. This body makes no direct
/// velocity, state, camera or race-path edits; those belong to that virtual
/// callee. Return the shared native zero Vec3 through the hidden buffer.
pub fn place_rider_at_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        let initialized = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !initialized {
            c.cycles += 7;
            c.g[V0][0] = c.lbu(at(c, GP, DEFAULTS_GP + 4));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] |= 0xffff;
            c.g[A0][0] = 1;
            c.sw(at(c, GP, DEFAULTS_GP), c.g[V1][0]);
            c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
            c.sb(r(c, SP), c.g[V0][0]);
        }

        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A2][0] = 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(at(c, GP, DEFAULTS_GP));
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        call_ptr(c, 0x003e_6574, 0x0030_09b0)?;
        let no_arguments = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_arguments {
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
                    deliver_event_here(c, 0x0030_09c8);
                }
            }
        }

        c.cycles += 4;
        c.g[A2][0] = c.lw(r(c, S0));
        c.g[V0][0] = !c.g[A2][0];
        let explicit_instance = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw(at(c, GP, WORLD_HANDLE_GP));
        if !explicit_instance {
            c.cycles += 4;
            c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
            c.g[A2][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(at(c, V0, CURRENT_INSTANCE));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r(c, V0) << 2);
            c.g[V1][0] = c.lw(r(c, A1));
            c.g[A0][0] = c.lw(at(c, V1, 8));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0));
            let missing_bank = c.g[V1][0] == 0;
            c.g[V0][0] = sx(r(c, A2) >> 8);
            if missing_bank {
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
            c.cycles += 1;
            c.g[A2][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
        }

        c.cycles += 3;
        c.g[S0][0] = c.lw(r(c, A2)); // current rider index
        let invalid_rider = (c.g[S0][0] as i64) < 0;
        c.g[V1][0] = 0x004d_0000;
        let mut common_return = true;
        if !invalid_rider {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.g[S3][0];
                common_return = false;
            } else {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, V0, 0xc));
                let no_node = c.g[A2][0] == 0;
                c.g[V0][0] = sx(at(c, V0, 0x10));
                if !no_node {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c, A2, 0xc));
                    c.g[A0][0] = c.lh(at(c, V1, 0xc0));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc4));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0030_0aa8)?;
                }
                c.cycles += 14;
                c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                c.g[A1][0] = c.g[V0][0]; // placement matrix
                c.g[A2][0] = sx(r(c, S0) << 2);
                c.g[V1][0] = c.lw(at(c, A0, 0x84));
                c.g[V0][0] = c.lw(at(c, V1, 0xc));
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A2)));
                c.g[A0][0] = c.lw(at(c, V0, 0x28));
                c.g[V1][0] = c.lw(at(c, A0, 0x6c0));
                c.g[A0][0] = sx(at(c, A0, 0x6c0));
                c.g[V0][0] = c.lh(at(c, V1, 0x60));
                c.g[A2][0] = c.lw(at(c, V1, 0x64));
                let target = r(c, A2);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                call_ptr(c, target, 0x0030_0adc)?;
                c.g[V1][0] = 0x004d_0000;
            }
        }
        if common_return {
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 14;
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
    })
}
