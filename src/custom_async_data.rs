//! Native front-end texture-file slot loading.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001a_3cd8,
    fe_async_manager_load_data_file,
    "cFEAsyncManager_LoadDataFile",
)];

const SLOT_BYTES: u32 = 0x11c;
const DATA: u32 = 0x100;
const ASYNC_HANDLE: u32 = 0x104;
const ASYNC: u32 = 0x108;
const SIZE: u32 = 0x10c;
const SET_STATE: u32 = 0x001a_39d8;

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `void cFEAsyncManager_LoadDataFile(cFEAsyncManager* self, int slot)`
/// (0x1a3cd8). Allocate a texture-file buffer, then load the slot's filename
/// synchronously or asynchronously according to its request flag. The file
/// APIs receive the filename at the beginning of the slot unchanged; this
/// routine does not open a BIG archive or interpret the `|` prefix.
pub fn fe_async_manager_load_data_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = SLOT_BYTES as u64;
        frame.save(c, S4, 0x10);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S4][0] = c.g[A1][0];
        c.mult(0, S1, S4, V0);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[A2][0] = sx(r(c, S0).wrapping_add(SIZE));
        c.g[V0][0] = 0x10000;
        frame.save(c, S2, 0x30);
        c.g[V1][0] = sx(r(c, S0).wrapping_add(DATA));
        frame.save_ra(c, 0);
        c.g[S3][0] = sx(r(c, A2).wrapping_add(r(c, S1)));
        c.g[V0][0] |= 0x9a0;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x10000;
        c.g[S2][0] = sx(r(c, V1).wrapping_add(r(c, S1)));
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x1430));
        c.g[A0][0] |= 0x9a0;
        c.g[A2][0] = 0x25000000;
        c.sw(r(c, S3), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x001a_3d3c)?;
        c.sw(r(c, S2), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = 2;
        call_ptr(c, SET_STATE, 0x001a_3d50)?;
        c.g[V0][0] = sx(r(c, S0).wrapping_add(r(c, S1)));
        c.g[V1][0] = c.lw(r(c, V0).wrapping_add(ASYNC));
        let asynchronous = c.g[V1][0] != 0;
        c.g[A1][0] = c.g[S4][0];
        if asynchronous {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 2;
            call_ptr(c, SET_STATE, 0x001a_3d6c)?;
            c.g[A1][0] = c.lw(r(c, S2));
            c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, S1)));
            c.g[A2][0] = c.lw(r(c, S3));
            call_ptr(c, 0x003d_f748, 0x001a_3d7c)?;
            c.g[V1][0] = sx(r(c, S0).wrapping_add(r(c, S1)));
            c.sw(r(c, V1).wrapping_add(ASYNC_HANDLE), c.g[V0][0]);
        } else {
            c.cycles += 8;
            c.g[A1][0] = c.lw(r(c, S2));
            c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, S1)));
            c.g[A2][0] = c.lw(r(c, S3));
            call_ptr(c, 0x003e_1bb8, 0x001a_3d98)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = 3;
            call_ptr(c, SET_STATE, 0x001a_3da8)?;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
