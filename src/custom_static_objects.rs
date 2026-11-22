//! Native Object factory used by world-script argument records.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_c0d0,
    world_obj_factory_object,
    "WorldObjFactory_Object",
)];

const DEFAULTS: u32 = 0x004f_b4f0;
const PARAMETER_TYPES: u32 = 0x0044_5f90;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `ScriptValue* WorldObjFactory_Object(out, count, Arg16* args)`
/// (0x2fc0d0). Each argument supplies a parameter index at +0, payload at
/// +4 and type tag at +0xc. The four expected tags are [1, 1, 1, 0].
/// Parameter 0 selects an instance handle, or -1 for the current factory
/// instance. The other two used words are passed to Object_ctor as an
/// integer and a boolean; this factory does not read collision geometry.
pub fn world_obj_factory_object(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw(r(c, GP).wrapping_add(0x24f0));
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A2][0] = 0x0050_0000;
        let initialized = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !initialized {
            c.cycles += 11;
            c.g[V1][0] = sx(r(c, A2).wrapping_sub(0x4b10));
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.lbu(r(c, V1).wrapping_add(0xc));
            c.g[V0][0] |= 0xffff;
            c.sw(DEFAULTS, c.g[V0][0]);
            c.g[A0][0] = 1;
            c.sb(r(c, V1).wrapping_add(0xc), c.g[A1][0]);
            c.sw(r(c, GP).wrapping_add(0x24f0), c.g[A0][0]);
            c.sw(r(c, V1).wrapping_add(4), 0);
            c.sw(r(c, V1).wrapping_add(8), 0);
            c.sb(r(c, SP), c.g[A1][0]);
        }

        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S1][0] = sx(r(c, SP).wrapping_add(0x10));
        c.g[V0][0] |= 0xffff;
        c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x4b10));
        c.g[A2][0] = 0x10;
        c.sw(r(c, SP).wrapping_add(0x10), c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[S4][0] = 0x004d_0000;
        call_ptr(c, 0x003e_6574, 0x002f_c154)?;
        let no_arguments = (c.g[S0][0] as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !no_arguments {
            c.cycles += 5;
            c.g[V0][0] = 0x0044_0000;
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = sx(PARAMETER_TYPES);
            c.g[A3][0] = 2;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(r(c, A2));
                c.g[V1][0] = c.lw(r(c, A2).wrapping_add(0xc));
                c.g[V0][0] = sx(r(c, V0) << 2);
                c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                c.g[V0][0] = c.lw(r(c, V0));
                let mut convert_integer = false;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[V0][0] = c.lw(r(c, A2).wrapping_add(4));
                } else {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw(r(c, A2).wrapping_add(4));
                    } else {
                        c.cycles += 4;
                        c.lwc1(0, r(c, A2).wrapping_add(4));
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
                c.g[A2][0] = sx(r(c, A2).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002f_c170);
                }
            }
        }

        c.cycles += 4;
        c.g[A2][0] = c.lw(r(c, S1));
        c.g[V0][0] = !c.g[A2][0];
        let explicit_instance = c.g[V0][0] != 0;
        c.g[A1][0] = c.lw(r(c, GP).wrapping_add(0x16c8));
        if !explicit_instance {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r(c, GP).wrapping_add(0xce8));
            c.g[S0][0] = c.lw(r(c, V0).wrapping_add(0x290));
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.g[V0][0] = sx(r(c, V0) << 2);
            c.g[V1][0] = c.lw(r(c, A1));
            c.g[A0][0] = c.lw(r(c, V1).wrapping_add(8));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
            c.g[V1][0] = c.lw(r(c, V0));
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.g[A2][0] = 0;
            } else {
                c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0x1c));
                c.cycles += 7;
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
                c.g[A2][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.g[S0][0] = c.g[A2][0];
        }

        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.lw(r(c, S0).wrapping_add(0xc));
            let has_owner = c.g[A0][0] != 0;
            c.g[A1][0] = 0x11;
            let mut attached = false;
            if has_owner {
                c.cycles += 4;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x002f_ae38, 0x002f_c240)?;
                attached = c.g[V0][0] != 0;
            }
            if !attached {
                c.cycles += 13;
                c.g[A1][0] = 0x004a_0000;
                c.g[A0][0] = 0x2c;
                c.g[A1][0] = sx(0x004a_3bf8);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x002f_c260)?;
                c.g[T0][0] = c.lw(r(c, S1).wrapping_add(8));
                c.g[A0][0] = c.g[V0][0];
                c.g[A3][0] = c.lw(r(c, S1).wrapping_add(4));
                c.g[A1][0] = 1;
                c.g[A2][0] = c.g[S0][0];
                c.g[T0][0] = (0 < c.g[T0][0]) as u64;
                call_ptr(c, 0x0035_6db0, 0x002f_c27c)?;
            }
        }

        c.cycles += 16;
        c.g[A1][0] = sx(r(c, S4).wrapping_sub(0x6f68));
        c.ldl(V0, r(c, A1).wrapping_add(7));
        c.ldr(V0, r(c, A1));
        c.g[V1][0] = c.lw(r(c, A1).wrapping_add(8));
        c.sdl(V0, r(c, S2).wrapping_add(7));
        c.sdr(V0, r(c, S2));
        c.sw(r(c, S2).wrapping_add(8), c.g[V1][0]);
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
