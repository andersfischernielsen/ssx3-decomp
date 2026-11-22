//! Native TrackPath array loading for WorldAIP resources.

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_b310, path_data_read_track_paths, "PathData_ReadTrackPaths"),
];

const PATH_DATA_READ_INT: Callee = Callee {
    addr: 0x0026_ba68,
    f: ssx3_ee::s07::f_0026ba68,
};
const PATH_READ: Callee = Callee {
    addr: 0x0026_a180,
    f: ssx3_ee::s07::f_0026a180,
};
const ALLOC_ARRAY: u32 = 0x0031_7e30;
const TRACK_PATH_BYTES: u32 = 0x3c;
const TRACK_PATH_VTABLE: u32 = 0x0048_1448;
const COUNT: u32 = 0x10;
const PATHS: u32 = 0x14;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> Addr {
    Addr(r(c, register).wrapping_add(offset))
}

/// `int PathData_ReadTrackPaths(PathData* self, const void* source)`
/// (0x26b310): consume the path count, allocate the native 0x3c-byte
/// TrackPath array, install its vtables and read each serialized path.
/// Returns the number of bytes consumed from the WorldAIP resource.
pub fn path_data_read_track_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        c.g[S1][0] = c.g[A1][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        c.g[S2][0] = sx(r(c, S3).wrapping_add(PATHS));
        frame.save_ra(c, 0x10);
        c.g[S4][0] = c.g[S1][0];
        PATH_DATA_READ_INT.call(c, 0x0026_b344)?;

        c.g[S1][0] = sx(r(c, S1).wrapping_add(4));
        c.g[S0][0] = at(c, SP, 0).lw(c);
        c.g[A0][0] = TRACK_PATH_BYTES as u64;
        c.g[A1][0] = 0x0048_0000;
        c.g[A2][0] = 0;
        c.mult(0, ZERO, S0, A0);
        at(c, S3, COUNT).sw(c, c.g[S0][0]);
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x1438));
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        call_ptr(c, ALLOC_ARRAY, 0x0026_b370)?;

        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = sx(r(c, S0).wrapping_sub(1));
        c.g[V1][0] = c.g[A1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = 0x0048_0000;
            c.g[A2][0] = u64::MAX;
            c.g[V0][0] = sx(TRACK_PATH_VTABLE);
            loop {
                c.cycles += 7;
                at(c, V1, 0x34).sw(c, c.g[V0][0]);
                c.g[A0][0] = sx(r(c, A0).wrapping_sub(1));
                let more = c.g[A0][0] != c.g[A2][0];
                c.g[V1][0] = sx(r(c, V1).wrapping_add(TRACK_PATH_BYTES));
                if !more { break }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b390);
                }
            }
        }

        c.cycles += 4;
        at(c, S2, 0).sw(c, c.g[A1][0]);
        c.g[V0][0] = at(c, S3, COUNT).lw(c);
        let nonempty = (c.g[V0][0] as i64) > 0;
        c.g[S0][0] = 0;
        if nonempty {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 10;
                c.g[A0][0] = at(c, S3, PATHS).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S2)));
                PATH_READ.call(c, 0x0026_b3d4)?;
                c.g[S2][0] = sx(r(c, S2).wrapping_add(TRACK_PATH_BYTES));
                c.g[V1][0] = at(c, S3, COUNT).lw(c);
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.g[S1][0] = sx(r(c, S1).wrapping_add(r(c, V0)));
                if !more { break }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b3c0);
                }
            }
        }

        c.cycles += 9;
        c.g[V0][0] = sx(r(c, S1).wrapping_sub(r(c, S4)));
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}
