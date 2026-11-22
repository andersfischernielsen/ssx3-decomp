//! Native `WorldObjFactory_DeadNode` (0x002fc2c0), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run.

use crate::call::{call_ptr, guest, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002f_c2c0,
    world_obj_factory_dead_node,
    "WorldObjFactory_DeadNode",
)];

const INSTANCE_FLAGS: u32 = 8;
const INSTANCE_NODE: u32 = 0xc;
const NODE_KIND: u32 = 0x10;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Delete the current node using its adjusted virtual destructor. `A2`
/// carries the WorldObj subobject, not necessarily its allocation base.
fn delete_node(c: &mut Cpu, back: u32) -> Flow {
    c.cycles += 6;
    c.g[V0][0] = c.lw(at(c, A2, 0xc));
    c.g[A1][0] = 3;
    c.g[A0][0] = c.lh(at(c, V0, 8));
    c.g[V1][0] = c.lw(at(c, V0, 0xc));
    let target = r(c, V1);
    c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
    call_ptr(c, target, back)
}

/// `void WorldObjFactory_DeadNode(Instance* instance, Params* parameters)`.
/// This internal factory reads only `parameters+4`, the mode; it does not
/// parse named arguments or resolve an instance RID.
///
/// Mode 0 deletes any non-dead owner and installs a native 0x1c-byte DeadNode.
/// Mode 1 deletes any non-dead owner and restores the static instance flags:
/// `t = flags & 0xffff0300; flags = t | (signed(t) >> 16) | 2`.
/// Mode 3 preserves existing kind 6/0x13 owners, otherwise installs the
/// alternative native 0x1c-byte node constructed at 0x350f60. Other modes do
/// nothing. A kind-6 node is preserved in every handled mode. In particular,
/// source Tricky DeadNode mode 4 is not a native mode of this function.
pub fn world_obj_factory_dead_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw(at(c, A1, 4));
        let mode_one = c.g[V1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];

        // The generated branches reload S0 either at 0x2fc40c (+1 cycle),
        // or in a branch delay slot before jumping directly to 0x2fc410.
        let mut common_restore = false;
        'mode: {
            if mode_one {
                c.cycles += 3;
                c.g[A2][0] = c.lw(at(c, S0, INSTANCE_NODE));
                let absent = c.g[A2][0] == 0;
                c.g[V0][0] = 6;
                if !absent {
                    c.cycles += 3;
                    c.g[V1][0] = c.lh(at(c, A2, NODE_KIND));
                    if c.g[V1][0] == c.g[V0][0] {
                        break 'mode;
                    }
                    delete_node(c, 0x002f_c338)?;
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw(at(c, S0, INSTANCE_FLAGS));
                c.g[V1][0] = 0xffff_ffff_ffff_0000;
                c.g[V1][0] |= 0x300;
                c.g[V0][0] &= c.g[V1][0];
                c.g[A0][0] = sx(((r(c, V0) as i32) >> 16) as u32);
                c.g[V0][0] |= c.g[A0][0];
                c.g[V0][0] |= 2;
                c.sw(at(c, S0, INSTANCE_FLAGS), c.g[V0][0]);
                common_restore = true;
                break 'mode;
            }

            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                let mode_zero = c.g[V1][0] == 0;
                c.g[A2][0] = c.lw(at(c, S0, INSTANCE_NODE));
                if !mode_zero {
                    c.cycles += 2;
                    break 'mode;
                }

                c.cycles += 2;
                let absent = c.g[A2][0] == 0;
                c.g[V0][0] = 6;
                if !absent {
                    c.cycles += 3;
                    c.g[V1][0] = c.lh(at(c, A2, NODE_KIND));
                    if c.g[V1][0] == c.g[V0][0] {
                        break 'mode;
                    }
                    delete_node(c, 0x002f_c388)?;
                }
                c.cycles += 11;
                c.g[A1][0] = 0x0049_0000;
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6948));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x002f_c3a0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0035_06d8, 0x002f_c3ac)?;
                break 'mode;
            }

            c.g[V0][0] = 3;
            c.cycles += 2;
            let mode_three = c.g[V1][0] == c.g[V0][0];
            c.g[A2][0] = c.lw(at(c, S0, INSTANCE_NODE));
            if !mode_three {
                c.cycles += 2;
                break 'mode;
            }

            c.cycles += 2;
            let absent = c.g[A2][0] == 0;
            c.g[V0][0] = 6;
            if !absent {
                c.cycles += 3;
                c.g[V1][0] = c.lh(at(c, A2, NODE_KIND));
                let dead = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x13;
                if dead {
                    common_restore = true;
                    break 'mode;
                }
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    break 'mode;
                }
                delete_node(c, 0x002f_c3e8)?;
            }

            c.cycles += 9;
            c.g[A1][0] = 0x0049_0000;
            c.g[A0][0] = 0x1c;
            c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x6938));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x002f_c400)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0035_0f60, 0x002f_c40c)?;
            common_restore = true;
        }

        if common_restore {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
