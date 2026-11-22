//! Native Rider_PlaceAtMatrix (0x001234d0), retained uninstalled.
//!
//! No automated tests or replays were added or run.
//! Generated EE is the authority for registers, cycles and call ordering.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0012_34d0, rider_place_at_matrix, "Rider_PlaceAtMatrix")];

const APP_GP_NEG: u32 = 0x848;
const LOCAL_X: u32 = 0x004f_f140;
const RIDER_OWNER: u32 = 0x77c;
const RIDER_METRIX: u32 = 0x790;
const RIDER_TAG: u32 = 0x86c;
const LOCAL_VIEW_INDEX: u32 = 0x870;
const ATTACHED_EMITTER_FX: u32 = 0x88c;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void Rider_PlaceAtMatrix(Rider* rider, const Matrix4* destination)`.
///
/// Place at the matrix translation, facing its transformed local X axis.
/// Rider_Place receives animation 5 and ground clearance zero, so this helper
/// performs the native ground probe rather than preserving the exact pivot.
/// It notifies the replay/controller and reset metrics, selects main state 3
/// and substate 13 before placement, sets owner+0x290 to 2, then selects
/// substate 6 and clears the screen transition, rider FX and attached emitter
/// FX. A local view (rider+0x870 below 2) receives its adjusted virtual callback.
/// No path, event cursor or race-clock field is written directly here.
pub fn rider_place_at_matrix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 58;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = c.lw(r(c, GP).wrapping_sub(APP_GP_NEG));
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lw(at(c, V1, 0x84));
        c.g[A1][0] = c.lw(at(c, S0, RIDER_TAG));
        c.g[A0][0] = c.lw(at(c, V0, 0x28));
        call_ptr(c, 0x0027_0970, 0x0012_34fc)?;

        // Keep the source VU0 sequence, including all W lanes and scratch
        // vector-register results. LOCAL_X is the initialized (1,0,0,0).
        c.lq(V0, at(c, S1, 0x30));
        c.g[A1][0] = 1;
        c.sq(V0, r(c, SP));
        c.g[V0][0] = 0x0050_0000;
        c.g[V0][0] = sx(LOCAL_X);
        c.lqc2(8, r(c, V0));
        c.lqc2(4, r(c, S1));
        c.lqc2(5, at(c, S1, 0x10));
        c.lqc2(6, at(c, S1, 0x20));
        c.lqc2(7, at(c, S1, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, at(c, SP, 0x20));
        c.lq(V0, at(c, SP, 0x20));
        c.g[A0][0] = c.lw(at(c, S0, RIDER_METRIX));
        c.sq(V0, at(c, SP, 0x10));
        call_ptr(c, 0x0011_9368, 0x0012_3548)?; // RiderMetrix_OnRiderReset

        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0011_fe78, 0x0012_3554)?; // Rider_SetMainState
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 13;
        call_ptr(c, 0x0011_fec8, 0x0012_3560)?; // Rider_SetSubState

        c.mtc1(ZERO, 12);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = sx(at(c, SP, 0x10));
        c.g[A3][0] = 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0011_d660, 0x0012_3578)?; // Rider_Place

        c.g[V1][0] = c.lw(at(c, S0, RIDER_OWNER));
        c.g[V0][0] = 2;
        c.g[A1][0] = 6;
        c.g[A0][0] = c.g[S0][0];
        c.sw(at(c, V1, 0x290), c.g[V0][0]);
        call_ptr(c, 0x0011_fec8, 0x0012_3590)?; // Rider_SetSubState
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0012_5038, 0x0012_3598)?; // Rider_StopTransition
        c.g[A0][0] = c.lw(at(c, S0, RIDER_OWNER));
        call_ptr(c, 0x0011_1890, 0x0012_35a0)?; // RiderBase_ResetFX
        c.g[A0][0] = c.lw(at(c, S0, ATTACHED_EMITTER_FX));
        call_ptr(c, 0x002f_1a00, 0x0012_35a8)?; // AttachedEmitterFX_Reset

        c.g[A1][0] = c.lw(at(c, S0, LOCAL_VIEW_INDEX));
        c.g[V0][0] = (c.g[A1][0] < 2) as u64;
        let has_local_view = c.g[V0][0] != 0;
        c.g[A1][0] = sx(r(c, A1) << 2);
        if has_local_view {
            c.cycles += 11;
            c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(APP_GP_NEG));
            c.g[V1][0] = c.lw(at(c, A0, 0x84));
            c.g[V0][0] = c.lw(at(c, V1, 0x84));
            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
            c.g[V1][0] = c.lw(at(c, V0, 4));
            c.g[A1][0] = c.lw(at(c, V1, 0xa8));
            c.g[V0][0] = c.lw(at(c, A1, 0x14));
            c.g[A0][0] = c.lh(at(c, V0, 0x20));
            c.g[V1][0] = c.lw(at(c, V0, 0x24));
            let target = r(c, V1);
            c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0012_35e4)?;
        }

        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}
