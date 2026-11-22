//! Native Debounce construction (0x00342c08), retained uninstalled.
//!
//! Generated EE is the authority. No automated tests or replays were added
//! or run.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0034_2c08, debounce_ctor, "Debounce_ctor_2")];

const INSTANCE: u32 = 0x18;
const INSTANCE_FLAGS: u32 = 8;
const VTABLE: u32 = 0xc;
const DELAY_TICKS: u32 = 0x2c;
const PARAM_8_COPY: u32 = 0x30;
const PARAM_SECONDS: u32 = 4;
const PARAM_8: u32 = 8;
const PARAM_VISIBILITY: u32 = 0xc;

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    (c.g[register][0] as u32).wrapping_add(offset)
}

/// `Debounce* Debounce_ctor(self, process_arg, Instance*, Params*, owner)`.
/// Construct a kind-0 ModObj, install the debounce vtable, convert the raw
/// float at params+4 from seconds to ticks using the game's current rate and
/// CVT.W.S, and copy params+8 unchanged into the node's +0x30 word. The
/// incoming T0 is forwarded to ModObj_TakeOverObject as its second argument.
///
/// Params+0xc selects the instance flag adjustment: 0 clears its low nibble;
/// 1 sets visibility bit 0, clears bit 1 and sets bit 2; other values convert
/// bits 1 -> 2 only when the original low two bits equal 3. Params+0 and +0x10
/// are not read here. This constructor does not itself process a contact or
/// decide whether a subsequent contact may run its script.
pub fn debounce_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        call_ptr(c, 0x0035_5280, 0x0034_2c34)?;

        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = c.lw(at(c, GP, 0x2a74));
        c.g[V1][0] = sx(at(c, V1, 0x6f0));
        c.g[A1][0] = c.g[S1][0];
        c.sw(at(c, S2, VTABLE), c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(1, at(c, V0, 0x10));
        c.cvt_s_w(1, 1);
        c.lwc1(0, at(c, S0, PARAM_SECONDS));
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.swc1(1, at(c, S2, DELAY_TICKS));
        c.g[V0][0] = c.lw(at(c, S0, PARAM_8));
        c.sw(at(c, S2, PARAM_8_COPY), c.g[V0][0]);
        call_ptr(c, 0x0035_5f10, 0x0034_2c70)?;

        c.g[V1][0] = c.lw(at(c, S0, PARAM_VISIBILITY));
        c.g[V0][0] = 1;
        let mut common_result = true;
        if c.g[V1][0] == 0 {
            c.cycles += 7;
            c.g[A0][0] = c.lw(at(c, S2, INSTANCE));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] |= 0xfff0;
            c.g[V0][0] = c.lw(at(c, A0, INSTANCE_FLAGS));
            c.g[V0][0] &= c.g[V1][0];
            c.sw(at(c, A0, INSTANCE_FLAGS), c.g[V0][0]);
        } else {
            c.cycles += 2;
            let mode_one = c.g[V1][0] == c.g[V0][0];
            c.g[A1][0] = c.lw(at(c, S2, INSTANCE));
            if mode_one {
                c.cycles += 11;
                c.g[V1][0] = 0xffff_ffff_ffff_0000;
                c.g[V1][0] |= 0xfffd;
                c.g[V0][0] = c.lw(at(c, A1, INSTANCE_FLAGS));
                c.g[V0][0] |= 1;
                c.sw(at(c, A1, INSTANCE_FLAGS), c.g[V0][0]);
                c.g[A0][0] = c.lw(at(c, S2, INSTANCE));
                c.g[V0][0] = c.lw(at(c, A0, INSTANCE_FLAGS));
                c.g[V0][0] &= c.g[V1][0];
                c.g[V0][0] |= 4;
                c.sw(at(c, A0, INSTANCE_FLAGS), c.g[V0][0]);
            } else {
                c.cycles += 5;
                c.g[V1][0] = 3;
                c.g[A0][0] = c.lw(at(c, A1, INSTANCE_FLAGS));
                c.g[V0][0] = c.g[A0][0] & 3;
                let convert = c.g[V0][0] == c.g[V1][0];
                c.g[V0][0] = c.g[S2][0];
                if convert {
                    c.cycles += 5;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] |= 0xfffd;
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
                    c.g[V0][0] |= 4;
                    c.sw(at(c, A1, INSTANCE_FLAGS), c.g[V0][0]);
                } else {
                    // This branch jumps directly to 0x342cf8, omitting the
                    // one-cycle common result load at 0x342cf4.
                    common_result = false;
                }
            }
        }
        if common_result {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
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
