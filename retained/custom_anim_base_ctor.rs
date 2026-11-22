//! Native AnimObject base constructor (0x0034d9b0), retained uninstalled.
//!
//! No automated tests or replays were added or run.
//! Generated EE is the authority for registers, cycles and call order.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0034_d9b0, anim_object_base_ctor, "AnimObject_BaseCtor")];

const WORLD_OBJ: u32 = 0x14;
const WORLD_OBJ_VTABLE: u32 = 0x20;
const MATRIX_ARRAY: u32 = 0x44;
const NODE_INFO: u32 = 0x48;
const INSTANCE_OF_WORLD_OBJ: u32 = 0x18;
const INSTANCE_MODEL: u32 = 0x80;
const INSTANCE_FLAGS: u32 = 8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `AnimBase* AnimObject_BaseCtor(AnimBase* self, int process_arg,
/// int type, Instance* instance, int force_owned)`.
///
/// Construct the modifier/WorldObj at self+0x14 via ModObj_ctor, allocate
/// a 64-byte matrix per resident MDR part at self+0x44, then allocate the
/// animation node information at self+0x48. Initialize published/current
/// time from the first node's native duration, event windows to the native
/// 1e10 sentinel and the dirty word at +0x10 to 1. When forced or when the
/// instance has flag 1, replace instance flag 2 with flag 4. Finally run
/// AnimObject_v52 to initialize its matrices/flag geometry.
///
/// This body reads no script-manager/current-task global. Registration,
/// Instance backlinks and any ownership/list setup are delegated to
/// ModObj_ctor; this function itself neither links a list nor clones an
/// Instance. Its WorldObj Instance field is self+0x2c.
pub fn anim_object_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx(at(c, S1, WORLD_OBJ));
        frame.save(c, S2, 0x20);
        c.g[A0][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T0][0];
        call_ptr(c, 0x0035_5280, 0x0034_d9dc)?; // ModObj_ctor

        c.g[S2][0] = sx(at(c, S1, MATRIX_ARRAY));
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = 0x0049_0000;
        c.g[V1][0] = sx(at(c, V1, 0x0cc8));
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x18a8));
        c.sw(at(c, S1, WORLD_OBJ_VTABLE), c.g[V1][0]);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = c.lw(at(c, S0, INSTANCE_OF_WORLD_OBJ));
        c.g[V1][0] = c.lw(at(c, V0, INSTANCE_MODEL));
        c.g[S0][0] = c.lw(at(c, V1, 4)); // MDR part count
        c.g[A0][0] = sx(r(c, S0) << 6);
        call_ptr(c, 0x0031_7e30, 0x0034_da10)?; // native array allocation
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = sx(r(c, S0).wrapping_sub(1));
        if !empty {
            // Empty element constructor: the generated body retains only
            // these loop-end scratch values and its two-cycle charge.
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
        }

        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        c.sw(r(c, S2), c.g[V0][0]);
        call_ptr(c, 0x0034_e448, 0x0034_da2c)?; // AnimObject_AllocNodeInfo
        c.g[A0][0] = c.lw(at(c, S1, NODE_INFO));
        c.mtc1(ZERO, 1);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0035_1508, 0x0034_da44)?; // IntersectInfo_GetDuration
            c.mov_s(1, 0);
        }

        c.cycles += 9;
        c.lwc1(0, r(c, GP).wrapping_sub(0x2aa8)); // 1e10
        c.g[V0][0] = 1;
        c.swc1(1, at(c, S1, 4));
        c.g[A0][0] = sx(at(c, S1, WORLD_OBJ));
        c.sw(at(c, S1, 0x10), c.g[V0][0]);
        c.swc1(0, at(c, S1, 0xc));
        c.swc1(1, r(c, S1));
        let force_owned = c.g[S3][0] != 0;
        c.swc1(0, at(c, S1, 8));
        let mut make_owned = force_owned;
        if !force_owned {
            c.cycles += 5;
            c.g[V1][0] = c.lw(at(c, A0, INSTANCE_OF_WORLD_OBJ));
            c.g[V0][0] = c.lw(at(c, V1, INSTANCE_FLAGS));
            c.g[V0][0] &= 1;
            make_owned = c.g[V0][0] != 0;
        }
        if make_owned {
            c.cycles += 7;
            c.g[A0][0] = c.lw(at(c, A0, INSTANCE_OF_WORLD_OBJ));
            c.g[V1][0] = 0xffff_ffff_ffff_0000;
            c.g[V1][0] |= 0xfffd;
            c.g[V0][0] = c.lw(at(c, A0, INSTANCE_FLAGS));
            c.g[V0][0] &= c.g[V1][0];
            c.g[V0][0] |= 4;
            c.sw(at(c, A0, INSTANCE_FLAGS), c.g[V0][0]);
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0034_e348, 0x0034_daa4)?; // AnimObject_v52
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
