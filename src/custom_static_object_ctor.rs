//! Native constructor for a static, modifier-backed world object.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0035_6db0, object_ctor, "Object_ctor")];

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

pub fn object_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.g[A3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[T0][0];
        c.g[A2][0] = 0x11;
        call_ptr(c, 0x0035_5280, 0x0035_6ddc)?;
        c.g[V1][0] = sx(0x0049_0e80);
        c.sw(r(c, S1).wrapping_add(0xc), c.g[V1][0]);
        let force_owned = c.g[S0][0] != 0;
        let mut make_owned = force_owned;
        if !force_owned {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x18));
            c.g[A0][0] = 3;
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(8));
            c.g[V1][0] &= 3;
            make_owned = c.g[V1][0] == c.g[A0][0];
        }
        if make_owned {
            c.cycles += 7;
            c.g[A0][0] = c.lw(r(c, S1).wrapping_add(0x18));
            c.g[V1][0] = 0xffff_ffff_ffff_fffd;
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(8));
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 4;
            c.sw(r(c, A0).wrapping_add(8), c.g[V0][0]);
        }
        c.cycles += 2;
        let special = c.g[S2][0] != 0;
        c.g[V0][0] = c.g[S1][0];
        if special {
            c.cycles += 8;
            c.g[A0][0] = c.lw(r(c, S1).wrapping_add(0x18));
            c.g[V1][0] = 0xffff_ffff_ffff_ffbf;
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(8));
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 0x20;
            c.sw(r(c, A0).wrapping_add(8), c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
