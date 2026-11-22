//! Native WorldObjFactory_AnimCombo (0x002fb270), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run:
//! the user's explicit ban overrides the normal differential-test workflow.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_b270,
    world_obj_factory_anim_combo,
    "WorldObjFactory_AnimCombo",
)];

const DEFAULTS: u32 = 0x004f_b3c0;
const DEFAULTS_READY_GP: u32 = 0x24d4;
const PARAM_TYPES: u32 = 0x0044_5e98;
const SCRIPT_MANAGER_GP: u32 = 0x0ce8;
const WORLD_HANDLE_GP: u32 = 0x16c8;
const CURRENT_INSTANCE: u32 = 0x290;
const INSTANCE_NODE: u32 = 0x0c;
const PARAM_BYTES: u32 = 0x3c;
const PARAM_HEAD: u32 = 0x002f_b350;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* WorldObjFactory_AnimCombo(Vec3* out, int count, NamedArg* args)`.
/// Each 16-byte argument holds `(index, value, unused, type)`. The native
/// parameter schema is `[int,int,int,float,float,float,float,float,int,int,
/// float,float,float,int,int]`; incompatible types for a float slot undergo
/// the original integer-to-float conversion. Parameter zero is an instance
/// RID, or -1 for the current instance in WScriptMan.
///
/// Reuse an owner through AttachModifier(kind=3), or allocate 0x88 bytes and
/// call AnimCombo_ctor (0x358c80) with the resolved instance and parameter
/// block. The hidden Vec3 result is the native shared script return value,
/// independently of whether a process was constructed.
pub fn world_obj_factory_anim_combo(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
        frame.save(c, S4, 0x60);
        frame.save(c, S0, 0xa0);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x70);
        c.g[A2][0] = 0x0050_0000;
        frame.save_ra(c, 0x50);

        if c.g[V0][0] == 0 {
            c.cycles += 28;
            c.g[V0][0] = sx(DEFAULTS);
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(at(c, V0, 0x38));
            c.g[V1][0] |= 0xffff;
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 2); // -1.0
            c.g[A0][0] = 1;
            c.g[AT][0] = 0x41f0_0000;
            c.mtc1(AT, 0); // 30.0
            c.g[AT][0] = 0x41f8_0000;
            c.mtc1(AT, 1); // 31.0
            c.sw(r(c, A2).wrapping_sub(0x4c40), c.g[V1][0]);
            c.sb(at(c, V0, 0x38), c.g[A1][0]);
            c.swc1(1, at(c, V0, 0x28));
            c.swc1(2, at(c, V0, 0x2c));
            c.swc1(0, at(c, V0, 0x30));
            c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
            c.sw(at(c, V0, 4), c.g[A0][0]);
            c.sw(at(c, V0, 8), 0);
            c.swc1(2, at(c, V0, 0xc));
            c.swc1(0, at(c, V0, 0x10));
            c.swc1(0, at(c, V0, 0x14));
            c.sw(at(c, V0, 0x18), 0);
            c.swc1(2, at(c, V0, 0x1c));
            c.sw(at(c, V0, 0x20), 0);
            c.sw(at(c, V0, 0x24), 0);
            c.sw(at(c, V0, 0x34), 0);
            c.sb(r(c, SP), c.g[A1][0]);
        }

        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(at(c, SP, 0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x4c40));
        c.g[A2][0] = PARAM_BYTES as u64;
        c.sw(at(c, SP, 0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S3][0] = 0x004d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_b338)?;
        let has_args = (c.g[S0][0] as i64) > 0;
        c.g[A2][0] = c.g[S4][0];
        if has_args {
            c.cycles += 4;
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
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = sx(r(c, V0) << 2);
                if !nonzero {
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
            let has_node = c.g[A0][0] != 0;
            c.g[A1][0] = 3;
            let mut handled = false;
            if has_node {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x002f_ae38, 0x002f_b420)?;
                handled = c.g[V0][0] != 0;
            }
            if !handled {
                c.cycles += 11;
                c.g[A1][0] = 0x0049_0000;
                c.g[A0][0] = 0x88;
                c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6998));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x002f_b440)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, 0x0035_8c80, 0x002f_b454)?;
            }
        }

        c.cycles += 16;
        c.g[A1][0] = sx(r(c, S3).wrapping_sub(0x6f68));
        c.ldl(V0, at(c, A1, 7));
        c.ldr(V0, r(c, A1));
        c.g[V1][0] = c.lw(at(c, A1, 8));
        c.sdl(V0, at(c, S2, 7));
        c.sdr(V0, r(c, S2));
        c.sw(at(c, S2, 8), c.g[V1][0]);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}
