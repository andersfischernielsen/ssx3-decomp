//! Native relocation of static collision meshes and sphere-tree bodies.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003a_aa08, collision_resolve, "cPS2ChunkResolve_v19")];

const MODELS: u32 = 4;
const RUNTIME_LIST: u32 = 8;
const RUNTIME_ITEMS: u32 = 12;
const BODY_BYTES: u32 = 0x140;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `int cPS2ChunkResolve_v19(resolver, handle, collision)` (0x3aaa08).
///
/// Fix up the three relative header pointers, then build the runtime list.
/// Type 1 contains sequential mesh headers and their indexed geometry;
/// type 3 contains serialized bodies read into 0x140-byte runtime slots.
pub fn collision_resolve(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S0, 0x50);
        frame.save(c, S4, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw(r(c, S0).wrapping_add(MODELS));
        let absent = c.g[V1][0] == c.g[V0][0];
        c.g[S4][0] = c.g[A1][0];
        if absent {
            c.cycles += 2;
            c.sw(r(c, S0).wrapping_add(MODELS), 0);
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
            c.sw(r(c, S0).wrapping_add(MODELS), c.g[V0][0]);
        }

        c.cycles += 4;
        c.g[V1][0] = c.lw(r(c, S0).wrapping_add(RUNTIME_LIST));
        c.g[V0][0] = u64::MAX;
        let absent = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
        if absent {
            c.cycles += 2;
            c.sw(r(c, S0).wrapping_add(RUNTIME_LIST), 0);
        } else {
            c.cycles += 1;
            c.sw(r(c, S0).wrapping_add(RUNTIME_LIST), c.g[V0][0]);
        }

        c.cycles += 4;
        c.g[V1][0] = c.lw(r(c, S0).wrapping_add(RUNTIME_ITEMS));
        c.g[V0][0] = u64::MAX;
        let absent = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
        if absent {
            c.cycles += 2;
            c.sw(r(c, S0).wrapping_add(RUNTIME_ITEMS), 0);
        } else {
            c.cycles += 1;
            c.sw(r(c, S0).wrapping_add(RUNTIME_ITEMS), c.g[V0][0]);
        }

        c.cycles += 4;
        c.g[V1][0] = c.lhu(r(c, S0));
        c.g[V0][0] = 3;
        let sphere_tree = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if sphere_tree {
            c.cycles += 9;
            c.g[S1][0] = c.lw(r(c, S0).wrapping_add(RUNTIME_LIST));
            c.g[S2][0] = 0;
            c.sw(r(c, S1).wrapping_add(4), 0);
            c.sw(r(c, S1), 0);
            c.g[V1][0] = c.lhu(r(c, S0).wrapping_add(2));
            c.sw(r(c, S1).wrapping_add(4), c.g[V1][0]);
            c.g[V0][0] = c.lw(r(c, S0).wrapping_add(RUNTIME_ITEMS));
            let empty = c.g[V1][0] == 0;
            c.sw(r(c, S1), c.g[V0][0]);
            if !empty {
                c.cycles += 1;
                c.g[S3][0] = 0;
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S3)));
                    call_ptr(c, 0x0032_c540, 0x003a_aac8)?;
                    c.g[V0][0] = c.lw(r(c, S1).wrapping_add(4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S3][0] = sx(r(c, S3).wrapping_add(BODY_BYTES));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_aab8);
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw(r(c, S1).wrapping_add(4));
            c.g[S2][0] = 0;
            let empty = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = c.lw(r(c, S0).wrapping_add(MODELS));
            if empty {
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0;
                loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw(r(c, S1));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S3)));
                    call_ptr(c, 0x0032_7da8, 0x003a_ab08)?;
                    c.g[S3][0] = sx(r(c, S3).wrapping_add(BODY_BYTES));
                    c.g[V1][0] = c.lw(r(c, S1).wrapping_add(4));
                    c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_aaf0);
                    }
                }
                c.cycles += 2;
                frame.restore(c, S0, 0x50);
            }
        } else {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x50);
            } else {
                c.cycles += 10;
                c.g[T0][0] = c.lw(r(c, S0).wrapping_add(RUNTIME_LIST));
                c.g[A3][0] = 0;
                c.sw(r(c, T0).wrapping_add(4), 0);
                c.sw(r(c, T0), 0);
                c.g[V1][0] = c.lhu(r(c, S0).wrapping_add(2));
                c.sw(r(c, T0).wrapping_add(4), c.g[V1][0]);
                c.g[V0][0] = c.lw(r(c, S0).wrapping_add(RUNTIME_ITEMS));
                c.sw(r(c, T0), c.g[V0][0]);
                let empty = c.g[V1][0] == 0;
                c.g[A2][0] = c.lw(r(c, S0).wrapping_add(MODELS));
                if !empty {
                    c.cycles += 1;
                    loop {
                        c.cycles += 23;
                        c.g[V1][0] = c.lw(r(c, T0));
                        c.g[V0][0] = sx(r(c, A3) << 2);
                        c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                        c.sw(r(c, V0), c.g[A2][0]);
                        c.g[V1][0] = c.lw(r(c, A2).wrapping_add(4));
                        c.g[V0][0] = c.lw(r(c, A2).wrapping_add(8));
                        c.g[A0][0] = c.lw(r(c, A2).wrapping_add(12));
                        c.g[V1][0] = sx(r(c, A2).wrapping_add(r(c, V1)));
                        c.g[A1][0] = c.lw(r(c, A2).wrapping_add(16));
                        c.g[V0][0] = sx(r(c, A2).wrapping_add(r(c, V0)));
                        c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                        c.sw(r(c, A2).wrapping_add(8), c.g[V0][0]);
                        c.g[A1][0] = sx(r(c, A2).wrapping_add(r(c, A1)));
                        c.sw(r(c, A2).wrapping_add(4), c.g[V1][0]);
                        c.sw(r(c, A2).wrapping_add(12), c.g[A0][0]);
                        c.sw(r(c, A2).wrapping_add(16), c.g[A1][0]);
                        c.g[V0][0] = c.lhu(r(c, A2));
                        c.g[V1][0] = c.lw(r(c, T0).wrapping_add(4));
                        c.g[V0][0] = sx(r(c, V0) << 4);
                        c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        let more = c.g[V1][0] != 0;
                        c.g[A2][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003a_ab58);
                        }
                    }
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x50);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
