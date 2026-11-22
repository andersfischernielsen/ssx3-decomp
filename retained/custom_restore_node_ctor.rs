//! Native RestoreNode constructor (0x00350f60), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, build, replay or game execution
//! was added or run: the user has disabled automated validation.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_0f60;
const AFTER_WORLD_OBJ_CTOR: u32 = 0x0035_0f80;
const END: u32 = 0x0035_0fd8;
const WORLD_OBJ_CTOR: u32 = 0x0034_fb00;
const RESTORE_NODE_VTABLE: u32 = 0x0049_1680;
const INSTANCE: u32 = 0x18;
const INSTANCE_FLAGS: u32 = 8;
const NODE_VTABLE: u32 = 0xc;

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    (c.g[register][0] as u32).wrapping_add(offset)
}

/// `RestoreNode* RestoreNode_ctor(RestoreNode* self, Instance* instance)`.
/// Construct a WorldObj of kind19/process argument1, install the RestoreNode
/// vtable and promote its instance to dynamic drawing while clearing bits
/// 0x60 and0x2. The instance's static/high-half flags and bit0 are preserved.
/// The temporary native frame survives the WorldObj constructor's yields.
pub fn restore_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        match c.pc {
            ENTRY => {
                c.cycles += 29;
                Frame::open(c, 0x20);
                c.g[A3][0] = c.g[A1][0];
                frame.save(c, S0, 0x10);
                c.g[A1][0] = 1;
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[A2][0] = 0x13;
                call_ptr(c, WORLD_OBJ_CTOR, AFTER_WORLD_OBJ_CTOR)?;
            }
            AFTER_WORLD_OBJ_CTOR => {}
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        }

        c.g[V1][0] = 0x0049_0000;
        c.g[A2][0] = c.lw(at(c, S0, INSTANCE));
        c.g[V1][0] = sx(RESTORE_NODE_VTABLE);
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.sw(at(c, S0, NODE_VTABLE), c.g[V1][0]);
        c.g[A0][0] |= 0xff9f;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = c.lw(at(c, A2, INSTANCE_FLAGS));
        c.g[A1][0] |= 0xfffd;
        frame.restore_ra(c, 0);
        c.g[V1][0] &= c.g[A0][0];
        c.sw(at(c, A2, INSTANCE_FLAGS), c.g[V1][0]);
        c.g[A0][0] = c.lw(at(c, S0, INSTANCE));
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = c.lw(at(c, A0, INSTANCE_FLAGS));
        c.g[V1][0] &= c.g[A1][0];
        c.g[V1][0] |= 4;
        c.sw(at(c, A0, INSTANCE_FLAGS), c.g[V1][0]);
        frame.close(c);
        Ok(())
    });
}
