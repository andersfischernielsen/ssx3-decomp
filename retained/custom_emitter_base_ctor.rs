//! Native Emitter_ctor0x370018, retained uninstalled.
//!
//! Generated EE defines registers, memory, cycles and calls. The user's
//! instruction disables tests/replays/gates; none were added or run.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0037_0018;
const AFTER_DESCRIPTOR: u32 = 0x0037_003c;
const END: u32 = 0x0037_0058;
const DESCRIPTOR: u32 = 0x10;
const VISIBLE: u32 = 0x0c;
const SPRITE_PHASE: u32 = 0x184;
const VTABLE: u32 = 0x18c;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }

/// `Emitter* Emitter_ctor(Emitter* self)`.
/// Install the emitter vtable, construct its embedded descriptor through
/// BoardSparkEmitter_ctor0x2f7a68, clear the visibility word, and initialize
/// the sprite phase to exactly0. Static imported sprites therefore start
/// at frame0 even when their animation rate is zero.
pub fn emitter_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        match c.pc {
            ENTRY => {
                c.cycles += 16;
                Frame::open(c, 0x20);
                c.g[V0][0] = 0x49_0000;
                frame.save(c, S0, 0x10);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(0x3160));
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[A0][0] = sx(r(c, S0).wrapping_add(DESCRIPTOR));
                c.g[RA][0] = sx(AFTER_DESCRIPTOR);
                c.sw(r(c, S0).wrapping_add(VTABLE), c.g[V0][0]);
                call_ptr(c, 0x002f_7a68, AFTER_DESCRIPTOR)?;
            }
            AFTER_DESCRIPTOR => {},
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }
        c.g[V0][0] = c.g[S0][0];
        c.sw(r(c, S0).wrapping_add(VISIBLE), 0);
        c.sw(r(c, S0).wrapping_add(SPRITE_PHASE), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
