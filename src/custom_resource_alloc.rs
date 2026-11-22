//! Native stream-resource reference tracking and first-use allocation.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003a_8a90, alloc_resource, "cWorldCache_AllocResource")];

const ALLOC_HEAP: u32 = 0x0031_7e30;
const ALLOC_SECTION: u32 = 0x003a_7878;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `void* cWorldCache::AllocResource(int hull, int kind, u32 handle, int bytes)`.
/// A handle packs the bank in its low byte and the resource ID above it.
/// Existing references append a hull handle and return zero. First use returns
/// storage, or -1 after undoing the reference when allocation fails.
pub fn alloc_resource(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[T2][0] = c.g[A3][0];
        c.g[T1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A3][0] = c.g[T2][0] & 0xff;
        frame.save(c, S2, 0x10);
        c.g[A0][0] = sx(r(c, T2) >> 8);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0xff;
        let local_bank = c.g[A3][0] != c.g[V0][0];
        c.g[T3][0] = c.g[A2][0];
        if local_bank {
            c.cycles += 4;
            c.g[V1][0] = c.lw(r(c, T1).wrapping_add(8));
            c.g[V0][0] = sx(r(c, A3) << 2);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
            c.g[V0][0] = c.lw(r(c, V0));
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lw(r(c, T1).wrapping_add(4));
        }

        c.cycles += 12;
        c.g[V0][0] = sx(r(c, V0).wrapping_add(4));
        c.g[V1][0] = sx(r(c, T3) << 3);
        c.g[S1][0] = c.g[V0][0];
        c.g[S2][0] = c.g[V1][0];
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[A0][0] = sx(r(c, A0) << 2);
        c.g[V1][0] = c.lw(r(c, V0));
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
        c.g[V0][0] = c.lbu(r(c, A0));
        let first_reference = c.g[V0][0] == 0;
        c.g[V1][0] = 1;
        c.cycles += 2;
        c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
        if first_reference {
            c.g[V1][0] = 0;
        }
        c.cycles += 2;
        c.sb(r(c, A0), c.g[V0][0]);

        if !first_reference {
            // The same shared RID may occur in several leaves. Each hull gets
            // its own handle while the resource's low-byte reference count grows.
            append_existing_handle(c);
            c.cycles += 1;
        } else {
            c.cycles += 2;
            let has_bytes = c.g[T0][0] != 0;
            c.g[V1][0] = sx(r(c, T3).wrapping_sub(1));
            if !has_bytes {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 1;
            } else {
                if !allocate_first_reference(c)? {
                    return Ok(());
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.cycles += 8;
                    c.g[V0][0] = sx(r(c, S1).wrapping_add(r(c, S2)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
                    c.g[V0][0] = c.lbu(r(c, A0));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
                    c.g[V1][0] = c.g[V0][0] & 0xff;
                    c.sb(r(c, A0), c.g[V0][0]);
                    if c.g[V1][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[V1][0];
                        c.sw(r(c, A0), c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = u64::MAX;
                    c.cycles += 1;
                }
            }
        }

        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

fn append_existing_handle(c: &mut Cpu) {
    c.cycles += 25;
    c.g[A0][0] = 0x18;
    c.g[A3][0] = sx(r(c, T2) >> 8);
    c.mult(0, V0, A1, A0);
    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, T1)));
    c.g[A2][0] = c.lw(r(c, A0).wrapping_add(0x3f8));
    c.g[V0][0] = 0;
    c.g[V1][0] = c.lw(r(c, A2).wrapping_add(8));
    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(0xc));
    c.g[V1][0] = sx(r(c, V1) << 2);
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
    c.sb(r(c, V1), c.g[T3][0]);
    c.g[A0][0] = c.lw(r(c, A2).wrapping_add(8));
    c.g[V1][0] = c.lw(r(c, A2).wrapping_add(0xc));
    c.g[A0][0] = sx(r(c, A0) << 2);
    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
    c.sb(r(c, A0).wrapping_add(1), c.g[T2][0]);
    c.g[A1][0] = c.lw(r(c, A2).wrapping_add(8));
    c.g[V1][0] = c.lw(r(c, A2).wrapping_add(0xc));
    c.g[A1][0] = sx(r(c, A1) << 2);
    c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, V1)));
    c.sh(r(c, A1).wrapping_add(2), c.g[A3][0]);
    c.g[V1][0] = c.lw(r(c, A2).wrapping_add(8));
    c.g[V1][0] = sx(r(c, V1).wrapping_add(1));
    c.sw(r(c, A2).wrapping_add(8), c.g[V1][0]);
}

/// Dispatch through the original kind table, retaining its register values.
/// `false` means the original jump table transferred outside this function.
fn allocate_first_reference(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = (c.g[V1][0] < 0xe) as u64;
    let in_table = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0049_0000;
    let target = if in_table {
        c.cycles += 6;
        c.g[V1][0] = sx(r(c, V1) << 2);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(0x4eb0));
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.lw(r(c, V1));
        r(c, A0)
    } else {
        0x003a_8c3c
    };

    match target {
        // Kinds 9 and 10: shared textures use the named heap, whereas local
        // textures use the bank's section allocator, with the same flags.
        0x003a_8bb8 | 0x003a_8bec => {
            let texture = target == 0x003a_8bb8;
            let flags = if texture { 0x4500_0000 } else { 0x2200_0000 };
            c.cycles += 3;
            c.g[V0][0] = 0xff;
            let local_bank = c.g[A3][0] != c.g[V0][0];
            c.g[A0][0] = c.lw(r(c, T1).wrapping_add(0xc));
            if local_bank {
                c.cycles += if texture { 5 } else { 4 };
                c.g[V0][0] = sx(r(c, A3) << 2);
                c.g[A1][0] = c.g[T0][0];
                c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                c.g[A2][0] = flags;
                c.cycles += 4;
                c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x3c));
                call_ptr(c, ALLOC_SECTION, 0x003a_8c24)?;
            } else {
                c.cycles += 5;
                c.g[A1][0] = 0x0049_0000;
                c.g[A0][0] = c.g[T0][0];
                c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4d40));
                c.g[A2][0] = flags;
                allocate_heap(c)?;
            }
        }
        // Kind 14 (AIPATH) always uses its own named heap allocation.
        0x003a_8c2c => {
            c.cycles += 4;
            c.g[A1][0] = 0x0049_0000;
            c.g[A0][0] = c.g[T0][0];
            c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4e98));
            c.cycles += 1;
            c.g[A2][0] = 0;
            allocate_heap(c)?;
        }
        0x003a_8c3c => {
            c.cycles += 3;
            c.g[V0][0] = 0xff;
            let local_bank = c.g[A3][0] != c.g[V0][0];
            c.g[A0][0] = c.lw(r(c, T1).wrapping_add(0xc));
            if local_bank {
                c.cycles += 6;
                c.g[V0][0] = sx(r(c, A3) << 2);
                c.g[A1][0] = c.g[T0][0];
                c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                c.g[A2][0] = 0;
                c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x3c));
                call_ptr(c, ALLOC_SECTION, 0x003a_8c80)?;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0x0049_0000;
                c.g[A0][0] = c.g[T0][0];
                c.g[A1][0] = sx(r(c, A1).wrapping_add(0x4d40));
                c.cycles += 1;
                c.g[A2][0] = 0;
                allocate_heap(c)?;
            }
        }
        other => {
            c.jump_out(other, 0x003a_8a90, 0x003a_8cd0);
            return Ok(false);
        }
    }
    Ok(true)
}

fn allocate_heap(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A3][0] = 0;
    call_ptr(c, ALLOC_HEAP, 0x003a_8c60)
}
