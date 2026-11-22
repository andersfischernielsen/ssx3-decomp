//! Native stream-resource resolution/insertion callback (0x3a8cd0).
//!
//! Despite the historical name, this is not a read-only resource lookup.
//! It appends a hull reference, calls the chunk resolver, and stores its
//! compressed pointer while retaining the table entry's reference count.
//! Kept uninstalled; no tests/build were run.
//!
//! Read-only Instance lookup (does not call this callback):
//! `app = *(0x4a28a8)`, `module = *(app+0x84)`, `world = *(module+0x10)`,
//! `cache = *world`, `banks = *(cache+8)`, `bank = banks[section]`.
//! Type 3 has its entries pointer at bank+0x1c and count at bank+0x20.
//! For an in-range index, a nonzero low byte means referenced;
//! decode the pointer as `(entry >> 8) << 2`, rejecting zero and the
//! `gp+0x16e0` empty-payload sentinel. Shared resources use *(cache+4).

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a_8cd0,
    world_cache_resolve_resource,
    "cWorldCache_ResolveResource",
)];

const SHARED_TABLE: u32 = 4;
const SECTION_TABLES: u32 = 8;
const CHUNK_RESOLVER: u32 = 0x3e8;
const HULL_LISTS: u32 = 0x3f8;
const LIST_STRIDE: u32 = 0x18;
const LIST_COUNT: u32 = 8;
const LIST_ENTRIES: u32 = 0xc;
const EMPTY_PAYLOAD_GP: u32 = 0x16e0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void ResolveResource(cache, hull, kind, rid, payload)`.
/// A0 = cache, A1 = hull index, A2 = resource kind, A3 = packed RID,
/// T0 = allocated payload (or gp+0x16e0 for an empty payload).
/// The RID's low byte selects the bank; the remaining bits select the
/// resource. No file/stream request is issued by this body itself.
pub fn world_cache_resolve_resource(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        c.g[T1][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        c.g[T2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A0][0] = c.g[T1][0] & 0xff;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S3][0] = sx(r(c, T1) >> 8);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0xff;
        c.g[S0][0] = c.g[T0][0];
        if c.g[A0][0] != c.g[V0][0] {
            c.cycles += 4;
            c.g[V1][0] = c.lw(at(c, T2, SECTION_TABLES));
            c.g[V0][0] = sx(r(c, A0) << 2);
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
            c.g[S2][0] = c.lw(r(c, V0));
        } else {
            c.cycles += 2;
            c.g[S2][0] = c.lw(at(c, T2, SHARED_TABLE));
        }

        // Every resolved resource is also recorded against its hull, for
        // matching release when that hull is evicted.
        c.cycles += 25;
        c.g[V1][0] = LIST_STRIDE as u64;
        c.g[A2][0] = sx(r(c, T1) >> 8);
        c.mult(0, V0, A1, V1);
        c.g[A3][0] = sx(at(c, GP, EMPTY_PAYLOAD_GP));
        c.g[V1][0] = sx(r(c, V0).wrapping_add(r(c, T2)));
        c.g[A1][0] = c.lw(at(c, V1, HULL_LISTS));
        c.g[V0][0] = c.lw(at(c, A1, LIST_COUNT));
        c.g[V1][0] = c.lw(at(c, A1, LIST_ENTRIES));
        c.g[V0][0] = sx(r(c, V0) << 2);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.sb(r(c, V0), c.g[S1][0]);
        c.g[V1][0] = c.lw(at(c, A1, LIST_COUNT));
        c.g[V0][0] = c.lw(at(c, A1, LIST_ENTRIES));
        c.g[V1][0] = sx(r(c, V1) << 2);
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.sb(at(c, V1, 1), c.g[T1][0]);
        c.g[A0][0] = c.lw(at(c, A1, LIST_COUNT));
        c.g[V0][0] = c.lw(at(c, A1, LIST_ENTRIES));
        c.g[A0][0] = sx(r(c, A0) << 2);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
        c.sh(at(c, A0, 2), c.g[A2][0]);
        c.g[V0][0] = c.lw(at(c, A1, LIST_COUNT));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
        let empty_payload = c.g[S0][0] == c.g[A3][0];
        c.sw(at(c, A1, LIST_COUNT), c.g[V0][0]);

        c.cycles += 11;
        c.g[T0][0] = c.lw(at(c, T2, CHUNK_RESOLVER));
        if empty_payload {
            c.g[A2][0] = c.g[T1][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0;
        } else {
            c.g[A3][0] = c.g[S0][0];
            c.g[A2][0] = c.g[T1][0];
            c.g[A1][0] = c.g[S1][0];
        }
        c.g[V1][0] = c.lw(at(c, T0, 8));
        c.g[A0][0] = c.lh(at(c, V1, 0x18));
        c.g[V0][0] = c.lw(at(c, V1, 0x1c));
        let target = r(c, V0);
        c.g[A0][0] = sx(r(c, T0).wrapping_add(r(c, A0)));
        call_ptr(
            c,
            target,
            if empty_payload {
                0x003a_8da8
            } else {
                0x003a_8dd4
            },
        )?;
        if !empty_payload {
            c.g[S0][0] = c.g[V0][0];
        }
        c.g[V0][0] = sx(r(c, S1) << 3);

        // SectionTable has one {entries,count} pair per resource kind.
        // A reference word stores refcount:8 | (pointer>>2):24.
        c.cycles += 16;
        c.g[A0][0] = sx(r(c, S3) << 2);
        c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
        c.g[V1][0] = sx(((r(c, S0) as i32) >> 2) as u32);
        c.g[A1][0] = c.lw(at(c, V0, 4));
        c.g[V1][0] = sx(r(c, V1) << 8);
        frame.restore(c, S0, 0x40);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = c.lbu(r(c, A0));
        frame.restore(c, S2, 0x20);
        c.g[V0][0] |= c.g[V1][0];
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        c.sw(r(c, A0), c.g[V0][0]);
        frame.close(c);
        Ok(())
    });
}
