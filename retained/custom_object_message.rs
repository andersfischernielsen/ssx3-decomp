//! Native WScriptFn_ObjectSlot36 (0x002ff9a8), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x002f_f9a8, object_message, "WScriptFn_ObjectSlot36")];

const DEFAULTS: u32 = 0x004f_baf8;
const PARAM_TYPES: u32 = 0x0044_6540;
const DEFAULTS_READY_GP: u32 = 0x2550;
const WORLD_HANDLE_GP: u32 = 0x16c8;
const SCRIPT_MANAGER_GP: u32 = 0x0ce8;
const CURRENT_INSTANCE: u32 = 0x290;
const INSTANCE_NODE: u32 = 0x0c;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* ObjectSlot36(Vec3* out, int count, NamedArg* arguments)`.
/// The 16-byte named records are `(index, bits, unused, type)`. The native
/// schema is `[1,1,2,0]`: instance RID, message ID, float value, unused byte.
/// Defaults are RID -1, message -1 and value zero; the final byte is preserved.
/// RID -1 means the current WScriptMan instance; an explicit RID uses the
/// section's resident type-3 reference table, without starting a disc load.
///
/// When the instance has a node, call its adjusted virtual slot +0x120:
/// A1 receives the message ID unchanged and F12 receives the value unchanged.
/// This wrapper neither translates source command IDs nor scales the value.
/// Return the shared native zero Vec3 through the hidden result buffer.
pub fn object_message(c: &mut Cpu) {
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
        let initialized = c.g[V0][0] != 0;
        c.g[A2][0] = 0x0050_0000;
        if !initialized {
            c.cycles += 12;
            c.g[V1][0] = sx(DEFAULTS);
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(at(c, V1, 0xc));
            c.g[V0][0] |= 0xffff;
            c.sw(r(c, A2).wrapping_sub(0x4508), c.g[V0][0]);
            c.g[A0][0] = u64::MAX;
            c.sb(at(c, V1, 0xc), c.g[A1][0]);
            c.g[V0][0] = 1;
            c.sw(at(c, V1, 4), c.g[A0][0]);
            c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[V0][0]);
            c.sw(at(c, V1, 8), 0);
            c.sb(r(c, SP), c.g[A1][0]);
        }

        c.cycles += 10;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x4508));
        c.g[A2][0] = 0x10;
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x002f_fa28)?;
        let no_arguments = (c.g[S1][0] as i64) <= 0;
        c.g[A2][0] = c.g[S2][0];
        if !no_arguments {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S1][0];
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
                    deliver_event_here(c, 0x002f_fa40);
                }
            }
        }

        c.cycles += 4;
        c.g[A2][0] = c.lw(r(c, S0));
        c.g[V0][0] = !c.g[A2][0];
        let explicit_instance = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw(at(c, GP, WORLD_HANDLE_GP));
        if !explicit_instance {
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
        c.g[A2][0] = c.lw(at(c, V0, INSTANCE_NODE));
        let no_node = c.g[A2][0] == 0;
        c.g[V1][0] = 0x004d_0000;
        if !no_node {
            c.cycles += 8;
            c.g[V0][0] = c.lw(at(c, A2, 0xc));
            c.lwc1(12, at(c, S0, 8));
            c.g[A0][0] = c.lh(at(c, V0, 0x120));
            c.g[V1][0] = c.lw(at(c, V0, 0x124));
            c.g[A1][0] = c.lw(at(c, S0, 4));
            let target = r(c, V1);
            c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x002f_fb0c)?;
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
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}
