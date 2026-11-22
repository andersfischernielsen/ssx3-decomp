//! Native creation of every object in an extended UI screen definition.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0039_ced0,
    ui_screen_create_objects,
    "UIScreen_CreateObjects",
)];

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Allocate the screen's object index, create definitions in file order,
/// attach each successful object to the root, then notify the owner state.
pub fn ui_screen_create_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x38));
        c.g[A0][0] = c.lw(r(c, V0));
        let empty = c.g[A0][0] == 0;
        c.g[S0][0] = sx(r(c, V0).wrapping_add(4));
        if !empty {
            c.cycles += 13;
            c.g[A1][0] = 0x0049_0000;
            c.g[A2][0] = 0x2000_0000;
            c.g[A0][0] = sx(r(c, A0) << 2);
            c.g[A1][0] = sx(r(c, A1).wrapping_add(0x3ef0));
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7e30, 0x0039_cf14)?;
            c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x38));
            c.g[A0][0] = c.g[V0][0];
            c.sw(r(c, S1).wrapping_add(0x3c), c.g[V0][0]);
            c.g[A1][0] = 0;
            c.g[A2][0] = c.lw(r(c, V1));
            c.g[A2][0] = sx(r(c, A2) << 2);
            call_ptr(c, 0x003e_6448, 0x0039_cf30)?;
        }
        c.cycles += 6;
        c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x38));
        c.g[S3][0] = 0;
        c.g[S2][0] = c.lw(r(c, S1).wrapping_add(0x3c));
        c.g[V0][0] = c.lw(r(c, V1));
        let empty = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[V1][0];
        if !empty {
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0039_d078, 0x0039_cf54)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[V1][0];
                if c.g[V1][0] != 0 {
                    c.cycles += 6;
                    c.sw(r(c, S2), c.g[V1][0]);
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(4));
                    c.g[A0][0] = sx(r(c, S1).wrapping_add(0xb4));
                    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(4));
                    c.sw(r(c, V1).wrapping_add(0x38), c.g[V0][0]);
                    call_ptr(c, 0x0039_7910, 0x0039_cf78)?;
                }
                c.cycles += 8;
                c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x38));
                c.g[S3][0] = sx(r(c, S3).wrapping_add(1));
                c.g[A0][0] = c.lhu(r(c, S0).wrapping_add(2));
                c.g[V1][0] = c.lw(r(c, V0));
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_cf48);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(r(c, A1));
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x3c));
            loop {
                c.cycles += 5;
                c.g[V0][0] = sx(r(c, S0) << 2);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                c.g[A1][0] = c.lw(r(c, V0));
                if c.g[A1][0] == 0 {
                    c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x38));
                } else {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw(r(c, S1).wrapping_add(0xd0));
                    c.g[V0][0] = c.lw(r(c, A2).wrapping_add(8));
                    c.g[A0][0] = c.lh(r(c, V0).wrapping_add(0x90));
                    c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x94));
                    let target = r(c, V1);
                    c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0039_cfd4)?;
                    c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x38));
                }
                c.cycles += 5;
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[V0][0] = c.lw(r(c, V1));
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = c.lw(r(c, S1).wrapping_add(0x3c));
                if event_due(c) {
                    deliver_event_here(c, 0x0039_cfa8);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
