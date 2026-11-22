//! Native Fence constructor0x0034cab8, retained uninstalled.
//!
//! Generated EE owns registers, cycles and call order. Automated tests,
//! builds and game runs are disabled for this inspection writer.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0034_cab8;
const AFTER_WORLD_OBJECT: u32 = 0x0034_cae4;
const AFTER_BUILD_MESH: u32 = 0x0034_cb48;
const AFTER_IMPULSE: u32 = 0x0034_cb54;
const END: u32 = 0x0034_cb80;

const VTABLE: u32 = 0x0c;
const DESCRIPTOR: u32 = 0x1c;
const MOTION: u32 = 0x2c;
const STRENGTH: u32 = 0x30;
const MOTION_EXTRA: u32 = 0x34;
const LEFT_DELAY: u32 = 0x38;
const RIGHT_DELAY: u32 = 0x3c;

fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Fence* Fence_ctor(self, process_arg, Instance*, Descriptor*, unused,
/// float initial_impulse)` (0x34cab8).
/// Attach a native WorldObj of type9 to the instance, install Fence's
/// vtable48fa00, copy the16-byte descriptor to+1c, force its first word to-1,
/// clear motion/strength, build the native deformable mesh and apply F12's
/// initial impulse. The two neighbour delays start at4. Incoming T0 is not
/// consumed. The constructor itself allocates no object or descriptor.
pub fn fence_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 49;
                    Frame::open(c, 0x40);
                    frame.save(c, S1, 0x10);
                    frame.save(c, S0, 0x20);
                    c.g[S1][0] = c.g[A3][0];
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_fpr(c, 20, 0x30);
                    c.g[A3][0] = c.g[A2][0];
                    frame.save_ra(c, 0);
                    c.mov_s(20, 12);
                    c.g[A2][0] = 9;
                    call_ptr(c, 0x0034_fb00, AFTER_WORLD_OBJECT)?;
                    pc = AFTER_WORLD_OBJECT;
                }
                AFTER_WORLD_OBJECT => {
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V1][0] = 0x0049_0000;
                    c.g[V0][0] |= 0xffff;
                    c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x600));
                    c.sw(at(c, S0, DESCRIPTOR), c.g[V0][0]);
                    c.g[A0][0] = u64::MAX;
                    c.sw(at(c, S0, VTABLE), c.g[V1][0]);
                    c.g[V0][0] = 15;
                    // The generated body has no element-initializer loop:
                    // its sixteen empty constructors were optimized away.
                    c.ldl(V1, at(c, S1, 7));
                    c.ldr(V1, r(c, S1));
                    c.ldl(A0, at(c, S1, 15));
                    c.ldr(A0, at(c, S1, 8));
                    c.sdl(V1, at(c, S0, DESCRIPTOR + 7));
                    c.sdr(V1, at(c, S0, DESCRIPTOR));
                    c.sdl(A0, at(c, S0, DESCRIPTOR + 15));
                    c.sdr(A0, at(c, S0, DESCRIPTOR + 8));
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[V0][0] |= 0xffff;
                    c.sw(at(c, S0, STRENGTH), 0);
                    c.sw(at(c, S0, MOTION), 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(at(c, S0, MOTION_EXTRA), 0);
                    c.sw(at(c, S0, DESCRIPTOR), c.g[V0][0]);
                    call_ptr(c, 0x0034_d1e8, AFTER_BUILD_MESH)?;
                    pc = AFTER_BUILD_MESH;
                }
                AFTER_BUILD_MESH => {
                    c.mov_s(12, 20);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0034_d6f0, AFTER_IMPULSE)?;
                    pc = AFTER_IMPULSE;
                }
                AFTER_IMPULSE => {
                    c.g[V1][0] = 4;
                    c.g[V0][0] = c.g[S0][0];
                    c.sw(at(c, S0, RIGHT_DELAY), c.g[V1][0]);
                    c.sw(at(c, S0, LEFT_DELAY), c.g[V1][0]);
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.restore_fpr(c, 20, 0x30);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
