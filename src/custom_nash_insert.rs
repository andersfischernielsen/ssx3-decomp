//! Native spatial-hash insertion into one of a cell's three primitive lists.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0032_8c20, insert, "SpatialHash_Insert")];

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// Copy one 16-byte spatial key while retaining the EE unaligned-load state.
fn copy_key(c: &mut Cpu, from: u32, to: u32) {
    c.ldl(V0, from.wrapping_add(7));
    c.ldr(V0, from);
    c.ldl(V1, from.wrapping_add(15));
    c.ldr(V1, from.wrapping_add(8));
    c.sdl(V0, to.wrapping_add(7));
    c.sdr(V0, to);
    c.sdl(V1, to.wrapping_add(15));
    c.sdr(V1, to.wrapping_add(8));
}

/// Clear the node's three primitive-list heads. Child slots are cleared by
/// the original memset call immediately afterward.
fn clear_list_heads(c: &mut Cpu, loop_pc: u32) {
    loop {
        c.cycles += 7;
        c.sw(r(c, V0), 0);
        c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
        let more = c.g[V1][0] != c.g[A0][0];
        c.g[V0][0] = sx(r(c, V0).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, loop_pc);
        }
    }
}

/// Determine whether root->key contains the requested key. The difference
/// in levels also becomes A2 for the parent-growth path.
fn root_contains_key(c: &mut Cpu) -> bool {
    c.cycles += 5;
    c.g[V0][0] = c.lw(r(c, S3));
    c.g[V1][0] = c.lw(r(c, S5));
    c.g[A0][0] = sx(r(c, V0).wrapping_sub(r(c, V1)));
    let nonnegative = (c.g[A0][0] as i64) >= 0;
    c.g[A2][0] = c.g[V0][0];
    if !nonnegative {
        c.cycles += 2;
        c.g[V0][0] = 0;
        return false;
    }

    c.cycles += 2;
    if c.g[A0][0] == 0 {
        c.cycles += 4;
        c.g[V1][0] = c.lw(r(c, S5).wrapping_add(4));
        c.g[V0][0] = c.lw(r(c, S3).wrapping_add(4));
        let different_x = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = 0;
        if different_x {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            return false;
        }
        c.cycles += 4;
        c.g[V1][0] = c.lw(r(c, S5).wrapping_add(8));
        c.g[V0][0] = c.lw(r(c, S3).wrapping_add(8));
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.g[A0][0];
            return false;
        }
        c.cycles += 5;
        c.g[V1][0] = c.lw(r(c, S5).wrapping_add(12));
        c.g[A0][0] = c.g[FP][0];
        c.g[V0][0] = c.lw(r(c, S3).wrapping_add(12));
        c.g[V1][0] ^= c.g[V0][0];
        if c.g[V1][0] != 0 {
            c.g[A0][0] = 0;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        return c.g[V0][0] != 0;
    }

    // At different levels, compare the requested cell's ancestor with root.
    c.g[V0][0] = c.lw(r(c, S5).wrapping_add(4));
    c.cycles += 4;
    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(4));
    c.g[V0][0] = sx(((r(c, V0) as i32) >> (r(c, A0) & 31)) as u32);
    let different_x = c.g[V0][0] != c.g[V1][0];
    c.g[A1][0] = 0;
    if different_x {
        c.cycles += 1;
        c.g[V0][0] = c.g[A1][0];
        return false;
    }
    c.cycles += 5;
    c.g[V0][0] = c.lw(r(c, S5).wrapping_add(8));
    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(8));
    c.g[V0][0] = sx(((r(c, V0) as i32) >> (r(c, A0) & 31)) as u32);
    let different_y = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = c.g[A1][0];
    if different_y {
        return false;
    }
    c.cycles += 6;
    c.g[V0][0] = c.lw(r(c, S5).wrapping_add(12));
    c.g[A1][0] = c.g[FP][0];
    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(12));
    c.g[V0][0] = sx(((r(c, V0) as i32) >> (r(c, A0) & 31)) as u32);
    c.g[V0][0] ^= c.g[V1][0];
    if c.g[V0][0] != 0 {
        c.g[A1][0] = 0;
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[A1][0];
    c.g[V0][0] != 0
}

/// `SpatialHash_Insert(root, list_kind, primitive, target_key)` (0x328c20).
///
/// Grow octree ancestors until the root contains the requested cell, then
/// link the primitive into its selected list. Primitive fields +0/+4 are
/// intrusive next/previous links; this function does not assign any render
/// or collision flags at +0xa/+0xc/+0xe.
pub fn insert(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S3, 0x90);
        frame.save(c, S5, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S7, 0x50);
        c.g[S2][0] = sx(r(c, S3).wrapping_add(0x10));
        frame.save(c, S0, 0xc0);
        c.g[S7][0] = c.g[A2][0];
        frame.save(c, S1, 0xb0);
        frame.save(c, S4, 0x80);
        frame.save(c, S6, 0x60);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        c.sw(frame.at(c, 0x20), c.g[A1][0]);
        c.g[V0][0] = c.lw(r(c, S3).wrapping_add(0x10));
        let root_exists = c.g[V0][0] != 0;
        c.g[S5][0] = c.g[A3][0];

        if !root_exists {
            c.cycles += 20;
            let (source, root) = (r(c, S5), r(c, S3));
            copy_key(c, source, root);
            // In the EE order A1 is loaded between the final two stores;
            // neither store aliases the register value.
            c.g[A1][0] = 0x004a_0000;
            c.g[A1][0] = sx(r(c, A1).wrapping_add(0x3fc0));
            c.g[A0][0] = 0x2c;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0032_8ca0)?;
            c.g[T0][0] = c.lw(frame.at(c, 0x20));
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = u64::MAX;
            c.g[V0][0] = sx(r(c, S0).wrapping_add(0x20));
            c.g[S1][0] = sx(r(c, T0) << 2);
            c.g[V1][0] = 2;
            clear_list_heads(c, 0x0032_8cb8);
            c.cycles += 11;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20;
            call_ptr(c, 0x003e_6448, 0x0032_8ce4)?;
            c.sw(r(c, S2), c.g[S0][0]);
            c.g[V0][0] = c.lw(r(c, S3).wrapping_add(0x10));
            c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, V0)));
            c.g[A0][0] = sx(r(c, A0).wrapping_add(0x20));
            c.g[V0][0] = c.lw(r(c, A0));
            if c.g[V0][0] != 0 {
                c.sw(r(c, V0).wrapping_add(4), c.g[S7][0]);
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(r(c, A0));
            c.sw(r(c, S7).wrapping_add(4), 0);
            c.sw(r(c, S7), c.g[V0][0]);
            c.sw(r(c, A0), c.g[S7][0]);
        } else {
            c.cycles += 2;
            c.g[FP][0] = 1;
            loop {
                let contains = root_contains_key(c);
                c.cycles += 2;
                c.g[A1][0] = c.lw(frame.at(c, 0x20));
                if contains {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_8d20);
                }

                // Expand the root one level, retaining the old node in the
                // octant selected by its old coordinates.
                c.cycles += 32;
                c.g[A1][0] = c.lw(r(c, S3).wrapping_add(4));
                c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
                c.g[V0][0] = c.lw(r(c, S3).wrapping_add(8));
                c.g[T1][0] = 0x004a_0000;
                c.g[V1][0] = c.lw(r(c, S3).wrapping_add(12));
                c.g[A1][0] = sx(((r(c, A1) as i32) >> 1) as u32);
                c.g[V0][0] = sx(((r(c, V0) as i32) >> 1) as u32);
                c.sw(frame.at(c, 0x14), c.g[A1][0]);
                c.g[V1][0] = sx(((r(c, V1) as i32) >> 1) as u32);
                c.sw(frame.at(c, 0x10), c.g[A2][0]);
                c.sw(frame.at(c, 0x1c), c.g[V1][0]);
                c.g[A1][0] = sx(r(c, T1).wrapping_add(0x3fc0));
                c.sw(frame.at(c, 0x18), c.g[V0][0]);
                c.g[A0][0] = 0x2c;
                c.g[A2][0] = 0x2000_0000;
                let (source, destination) = (frame.at(c, 0x10), frame.at(c, 0));
                copy_key(c, source, destination);
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, 0x0032_8d84)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[S6][0] = c.lw(r(c, S3).wrapping_add(0x10));
                c.g[A2][0] = c.g[SP][0];
                c.g[A0][0] = u64::MAX;
                c.g[V0][0] = sx(r(c, S4).wrapping_add(0x20));
                c.g[V1][0] = 2;
                clear_list_heads(c, 0x0032_8da0);

                c.cycles += 32;
                c.g[A3][0] = c.lw(r(c, A2).wrapping_add(12));
                c.g[A0][0] = c.g[S4][0];
                c.g[V1][0] = c.lw(r(c, A2).wrapping_add(4));
                c.g[A1][0] = 0;
                c.g[V0][0] = c.lw(r(c, A2).wrapping_add(8));
                c.g[A3][0] = sx(r(c, A3) << 1);
                c.g[S2][0] = c.lw(r(c, S3).wrapping_add(4));
                c.g[V1][0] = sx(r(c, V1) << 1);
                c.g[S1][0] = c.lw(r(c, S3).wrapping_add(8));
                c.g[V0][0] = sx(r(c, V0) << 1);
                c.g[S0][0] = c.lw(r(c, S3).wrapping_add(12));
                c.g[S2][0] = sx(r(c, S2).wrapping_sub(r(c, V1)));
                c.g[S1][0] = sx(r(c, S1).wrapping_sub(r(c, V0)));
                c.g[A2][0] = 0x20;
                c.g[S0][0] = sx(r(c, S0).wrapping_sub(r(c, A3)));
                call_ptr(c, 0x003e_6448, 0x0032_8dfc)?;
                c.g[S0][0] = sx(r(c, S0) << 2);
                c.g[S1][0] = sx(r(c, S1) << 3);
                let (source, root) = (frame.at(c, 0), r(c, S3));
                copy_key(c, source, root);
                c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, S1)));
                c.g[S2][0] = sx(r(c, S2) << 4);
                c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, S2)));
                c.sw(r(c, S3).wrapping_add(0x10), c.g[S4][0]);
                c.g[S0][0] = sx(r(c, S4).wrapping_add(r(c, S0)));
                c.sw(r(c, S0), c.g[S6][0]);
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw(r(c, S3).wrapping_add(0x10));
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[T0][0] = c.g[S3][0];
            call_ptr(c, 0x0032_8660, 0x0032_8ef8)?;
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA, GAME_GP, RA as TEST_RA};
    use crate::game::fe::feasyncfile::tests::{globals_12, heap_init_12};

    /// A far-away finite collider can require 31 ancestors and another 31
    /// descendants. Keep the real allocation/insert callees bounded without
    /// exhausting the usual eight-node test pools.
    fn pools(c: &mut Cpu) {
        const CLASSES: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        c.bus.write32(GAME_GP + 0xdd8, 1);
        for (index, (slot, head)) in CLASSES.into_iter().enumerate() {
            let index = index as u32;
            let pool = DATA + 0x40_0000 + index * 0x4_0000;
            let base = DATA + 0x80_0000 + index * 0x4_0000;
            for n in 0..128 {
                c.bus.write32(
                    base + n * 0x400,
                    if n == 127 { 0 } else { base + (n + 1) * 0x400 },
                );
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    #[test]
    fn spatial_hash_insert_matches() {
        check(
            ssx3_ee::s10::f_00328c20__generated,
            insert,
            33,
            &|c, seed| {
                machine(c, seed, 0x328c_20, 0);
                globals_12(c);
                heap_init_12(c);
                pools(c);
                let (root, cell, primitive, old_head) =
                    (DATA + 0x100, DATA + 0x1000, DATA + 0x3000, DATA + 0x4000);
                // Vary alignment too: the original copies keys with LDL/LDR.
                let target = DATA + 0x200 + seed % 4;
                let kind = seed / 11;
                let (old_level, old_xyz, new_level, new_xyz): (u32, [i32; 3], u32, [i32; 3]) =
                    match seed % 11 {
                        0 => (11, [0, 0, 0], 11, [0, 0, 0]), // empty root
                        1 => (11, [1, 2, 3], 11, [1, 2, 3]), // exact existing key
                        2 => (11, [0, 0, 0], 11, [1, 0, 0]),
                        3 => (11, [-1, -1, -1], 11, [-2, -1, -1]),
                        4 => (11, [1, 0, 0], 11, [3, 0, 0]),
                        5 => (11, [0, 0, 0], 11, [0, 2, 0]),
                        6 => (11, [0, 0, 0], 11, [0, 0, 1]),
                        7 => (11, [9, 9, 9], 14, [1, 1, 1]), // coarser requested key
                        8 => (14, [-1, -1, -1], 11, [-7, -3, -1]), // contained descendant
                        9 if kind % 2 == 0 => (11, [0, 0, 0], 11, [i32::MAX; 3]),
                        9 => (11, [-1, -1, -1], 11, [i32::MIN; 3]),
                        _ => (11, [1, 2, 3], 11, [1, 2, 3]), // duplicate insertion
                    };
                c.bus.write32(root, old_level);
                c.bus.write32(target, new_level);
                for axis in 0..3 {
                    c.bus
                        .write32(root + 4 + axis * 4, old_xyz[axis as usize] as u32);
                    c.bus
                        .write32(target + 4 + axis * 4, new_xyz[axis as usize] as u32);
                }
                let populated = seed % 11 != 0;
                c.bus.write32(root + 0x10, if populated { cell } else { 0 });
                if populated {
                    // Child slots start empty; the selected primitive list may
                    // contain a distinct head whose previous link is updated.
                    // Inserting an already-linked primitive is invalid input,
                    // but both implementations must still perform the original
                    // stores (which make next=self); deduplication belongs to
                    // the caller's remove/move protocol, not this leaf.
                    c.bus.write32(
                        cell + 0x20 + kind * 4,
                        if seed % 11 == 10 { primitive } else { old_head },
                    );
                }
                c.bus.write32(old_head, 0);
                c.bus.write32(old_head + 4, 0);
                c.bus.write32(primitive, 0);
                c.bus.write32(primitive + 4, 0);
                c.bus.write32(primitive + 8, 0x2225);
                c.g[A0][0] = sx(root);
                c.g[A1][0] = kind as u64;
                c.g[A2][0] = sx(primitive);
                c.g[A3][0] = sx(target);
                c.g[RA][0] = sx(TEST_RA);
                c.pc = 0x0032_8c20;
            },
        );
    }
}
