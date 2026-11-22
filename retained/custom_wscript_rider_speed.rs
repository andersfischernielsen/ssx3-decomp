//! Native WScriptFn_GetRiderSpeed (0x00303380), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replays were run
//! by this writer, in accordance with the user's development instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(ENTRY, wscript_fn_get_rider_speed, "WScriptFn_GetRiderSpeed")];

const ENTRY: u32 = 0x0030_3380;
const GET_VELOCITY_RETURN: u32 = 0x0030_33dc;
const GET_SCALAR_RETURN: u32 = 0x0030_3414;
const END: u32 = 0x0030_3430;

/// `LunoValue* WScriptFn_GetRiderSpeed(LunoValue* out, ...)`.
/// Resolve WScriptMan's current rider index through AI+0x28, then ask its
/// Rider primary interface (Rider+0x6c0) virtual slot2 for the velocity vector.
/// The ordinary Rider_GetVelocity getter returns Rider+0x1e0. Compute the
/// original VU0 XYZ length, with no unit conversion or world-rate multiplier.
/// Store a native Luno float (kind2 at out+8, bits at *GetFloat(out)); return
/// out in V0. The speed is not returned through F0 by this wrapper.
pub fn wscript_fn_get_rider_speed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x30 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 44;
                    Frame::open(c, 0x30);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_fpr(c, 20, 0x20);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    if c.g[A0][0] == 0 { c.g[V0][0] = 0; }
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
                    if c.g[V0][0] == 0 { c.g[A0][0] = 0; }
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let get_velocity = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, get_velocity, GET_VELOCITY_RETURN)?;
                    pc = GET_VELOCITY_RETURN;
                }
                GET_VELOCITY_RETURN => {
                    c.lqc2(3, c.g[V0][0] as u32);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(V1, 22);
                    c.mtc1(V1, 20);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = 2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V0][0]);
                    call_ptr(c, 0x0022_6618, GET_SCALAR_RETURN)?;
                    pc = GET_SCALAR_RETURN;
                }
                GET_SCALAR_RETURN => {
                    c.swc1(20, c.g[V0][0] as u32);
                    c.g[V0][0] = c.g[S0][0];
                    frame.restore_ra(c, 0);
                    frame.restore(c, S0, 0x10);
                    frame.restore_fpr(c, 20, 0x20);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
