//! Native WorldObjFactory_AnimObject (0x002fbcb8).
//!
//! Used by Tricky object effects through the native world process system.
//! No tests were added or run. Generated EE is the authority.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_bcb8,
    world_obj_factory_anim_object,
    "WorldObjFactory_AnimObject",
)];

const DEFAULTS: u32 = 0x004f_b498;
const DEFAULTS_READY_GP: u32 = 0x24e8;
const PARAM_TYPES: u32 = 0x0044_5f48;
const SCRIPT_MANAGER_GP: u32 = 0x0ce8;
const WORLD_HANDLE_GP: u32 = 0x16c8;
const CURRENT_INSTANCE: u32 = 0x290;
const INSTANCE_NODE: u32 = 0x0c;
const PARAM_BYTES: u32 = 0x2c;
const PARAM_HEAD: u32 = 0x002f_bd80;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* WorldObjFactory_AnimObject(Vec3* out, int count, NamedArg* args)`.
/// Each 16-byte argument contains `(index, value, unused, type)`; types are
/// checked against the native 11-word schema. Integer arguments to float
/// slots are converted by the original FPU operation.
///
/// Parameter 0 is an instance RID, or -1 for WScriptMan's current instance.
/// Resolve only a resident instance; ask an existing node to handle kind 1,
/// otherwise allocate and construct an AnimObject. Return the native shared
/// Vec3 result through the hidden first argument, independently of creation.
pub fn world_obj_factory_anim_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
        frame.save(c, S3, 0x60);
        frame.save(c, S0, 0x90);
        c.g[S3][0] = c.g[A2][0]; // named arguments
        frame.save(c, S2, 0x70);
        c.g[S0][0] = c.g[A1][0]; // argument count
        frame.save(c, S1, 0x80);
        c.g[S2][0] = c.g[A0][0]; // hidden Vec3 return buffer
        frame.save(c, S4, 0x50);
        c.g[A2][0] = 0x0050_0000;
        frame.save_ra(c, 0x40);

        if c.g[V0][0] == 0 {
            c.cycles += 22;
            c.g[V0][0] = sx(DEFAULTS);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(at(c, V0, 0x28));
            c.g[V1][0] |= 0xffff;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 0); // -1.0
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41f0_0000;
            c.mtc1(AT, 1); // 30.0
            c.sw(r(c, A2).wrapping_sub(0x4b68), c.g[V1][0]);
            c.sb(at(c, V0, 0x28), c.g[A1][0]);
            c.swc1(1, at(c, V0, 0x14));
            c.swc1(0, at(c, V0, 0x1c));
            c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
            c.sw(at(c, V0, 4), c.g[A0][0]);
            c.sw(at(c, V0, 8), 0);
            c.swc1(0, at(c, V0, 0xc));
            c.swc1(0, at(c, V0, 0x10));
            c.sw(at(c, V0, 0x18), 0);
            c.sw(at(c, V0, 0x20), 0);
            c.sw(at(c, V0, 0x24), 0);
            c.sb(r(c, SP), c.g[A1][0]);
        }

        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(DEFAULTS);
        c.g[A2][0] = PARAM_BYTES as u64;
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_bd68)?; // copy the defaults
        let has_args = (c.g[S0][0] as i64) > 0;
        c.g[A2][0] = c.g[S3][0];
        if has_args {
            c.cycles += 4;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(PARAM_TYPES);
            c.g[A3][0] = 2; // float schema tag
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(r(c, A2));
                c.g[V1][0] = c.lw(at(c, A2, 0xc));
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                c.g[V0][0] = c.lw(r(c, V0));
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = c.lw(at(c, A2, 4));
                    c.cycles += 1;
                    c.sw(r(c, A0), c.g[V0][0]);
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw(at(c, A2, 4));
                        c.cycles += 1;
                        c.sw(r(c, A0), c.g[V0][0]);
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, at(c, A2, 4));
                        c.cvt_s_w(0, 0);
                        c.swc1(0, r(c, A0));
                    }
                }
                c.cycles += 3;
                c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                let more = c.g[A1][0] != 0;
                c.g[A2][0] = sx(at(c, A2, 0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, PARAM_HEAD);
                }
            }
        }

        c.cycles += 4;
        c.g[A2][0] = c.lw(r(c, S1));
        c.g[V0][0] = !c.g[A2][0];
        c.g[A1][0] = c.lw(at(c, GP, WORLD_HANDLE_GP));
        if c.g[V0][0] == 0 {
            // RID -1: use the instance installed by the script dispatcher.
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
            c.g[S0][0] = c.lw(at(c, V0, CURRENT_INSTANCE));
        } else {
            // RID: section bank -> type-3 references -> compressed pointer.
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r(c, V0) << 2);
            c.g[V1][0] = c.lw(r(c, A1));
            c.g[A0][0] = c.lw(at(c, V1, 8));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0));
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.g[A3][0] = 0;
            } else {
                c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                c.cycles += 7;
                c.g[V0][0] = sx(r(c, A2) >> 8);
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                c.g[V1][0] = c.lw(r(c, V0));
                c.g[V0][0] = sx(r(c, V1) >> 8);
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r(c, V0) << 2);
                if !nonzero {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A3][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A3][0];
        }

        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.lw(at(c, S0, INSTANCE_NODE));
            let has_node = c.g[A0][0] != 0;
            c.g[A1][0] = 1;
            let mut handled = false;
            if has_node {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x002f_ae38, 0x002f_be50)?;
                handled = c.g[V0][0] != 0;
            }
            if !handled {
                c.cycles += 12;
                c.g[A1][0] = 0x0049_0000;
                c.g[A0][0] = 0x6c;
                c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6958));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x002f_be70)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A3][0] = c.g[S0][0];
                c.g[T0][0] = c.g[S1][0];
                c.g[A2][0] = 1;
                call_ptr(c, 0x0034_1aa0, 0x002f_be88)?;
            }
        }

        // Return a Vec3 through the hidden buffer, using the same unaligned
        // doubleword operations as the original instead of assuming alignment.
        c.cycles += 16;
        c.g[A1][0] = sx(r(c, S4).wrapping_sub(0x6f68));
        c.ldl(V0, at(c, A1, 7));
        c.ldr(V0, r(c, A1));
        c.g[V1][0] = c.lw(at(c, A1, 8));
        c.sdl(V0, at(c, S2, 7));
        c.sdr(V0, r(c, S2));
        c.sw(at(c, S2, 8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}
