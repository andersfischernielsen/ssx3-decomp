//! Staging module oc452 (wave wave224): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc452`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0013_6ad0, rider_wipeout_state_dtor, "RiderWipeoutState_dtor"),
    (0x0013_9178, rider_handplant_state_exit, "RiderHandplantState_Exit"),
    (0x0014_0680, rider_iface_dtor, "RiderIface_dtor"),
    (0x0014_0968, rider_get_heading, "Rider_GetHeading"),
    (0x0014_0cf0, rider_set_boost_mode, "Rider_SetBoostMode"),
    (0x0014_0d80, rider_ground_state_dtor, "RiderGroundState_dtor"),
    (0x0014_0ee0, rider_air_state_dtor, "RiderAirState_dtor"),
    (0x0014_1040, rider_railslide_state_dtor, "RiderRailslideState_dtor"),
    (0x0014_11c0, rider_static_state_dtor, "RiderStaticState_dtor"),
    (0x0014_1320, rider_handplant_state_dtor, "RiderHandplantState_dtor"),
    (0x0014_15c8, rider_cruise_sub_dtor, "RiderCruiseSub_dtor"),
    (0x0014_1728, rider_board_press_sub_dtor, "RiderBoardPressSub_dtor"),
    (0x0014_1888, rider_prewind_sub_dtor, "RiderPrewindSub_dtor"),
    (0x0014_19e8, rider_bump_sub_dtor, "RiderBumpSub_dtor"),
    (0x0014_1b48, rider_natural_air_sub_dtor, "RiderNaturalAirSub_dtor"),
    (0x0014_1ca8, rider_spin_sub_dtor, "RiderSpinSub_dtor"),
    (0x0014_1e08, rider_gate_sub_dtor, "RiderGateSub_dtor"),
    (0x0014_1f68, rider_railslide_sub_dtor, "RiderRailslideSub_dtor"),
];

/// The data page the generated code builds its table addresses from
/// (`lui 0x460000`, then `addiu` of a negative offset).
pub(crate) const STATE_DTOR_DATA_PAGE: u32 = 0x0046_0000;

/// `0x460000 - 0x6470`: the table the state's `+0x6d0` link points at.
pub(crate) const STATE_DTOR_LINK_K: u32 = 0x6470;

/// Callees of the state destructors (addresses and the generated shards'
/// names are in `generated/ee`).
pub(crate) const STATE_DTOR_SPHERE_TREE: u32 = 0x0032_9970;
pub(crate) const STATE_DTOR_RIDER_CALL: u32 = 0x0011_b978;
pub(crate) const STATE_DTOR_DELETE: u32 = 0x0031_7e50;

/// The layout one rider state destructor is built from. The generated
/// family (`RiderXState_dtor(this, flags)`) is one body: it copies a 0x40-byte
/// table from the data page to the stack, points the rider at it, then runs
/// the optional sphere-tree dtor and the rider and delete calls chosen by
/// the `flags` bits (`0x2`: rider call, `0x1`: delete). The constants differ
/// per state (checked against the generated bodies: only these numbers
/// change, and the addresses that follow from the entry).
#[derive(Clone, Copy)]
pub(crate) struct StateDtorLayout {
    /// Entry address of the generated function: the loops' event heads and
    /// the call return addresses are offsets from it.
    pub entry: u32,
    /// Offset of the rider pointer in the state.
    pub rider: u32,
    /// `0x460000 - table`: the table the stack copy starts from (`A2`).
    pub table: u32,
    /// `0x460000 - vtab`: the value stored at the rider's `+0x6c0`, and the
    /// base of the second table copy.
    pub vtab: u32,
    /// The `- stride` applied to the rider pointer for the stack word at +8.
    pub stride: u32,
    /// Offset of a sphere-tree pointer (wipeout only): dtor'd first, with 3.
    pub sphere: Option<u32>,
    pub ret_sphere: u32,
    pub ret_rider: u32,
    pub ret_delete: u32,
}

/// The common layout of the 348-byte family: the rider call returns at
/// `entry + 0x134`, the delete at `entry + 0x148`.
const fn state_dtor_layout(entry: u32, rider: u32, table: u32, vtab: u32, stride: u32) -> StateDtorLayout {
    StateDtorLayout {
        entry,
        rider,
        table,
        vtab,
        stride,
        sphere: None,
        ret_sphere: 0,
        ret_rider: entry + 0x134,
        ret_delete: entry + 0x148,
    }
}

const RIDER_GROUND_DTOR: StateDtorLayout = state_dtor_layout(0x0014_0d80, 0x18, 0x6568, 0x6520, 0x20);
const RIDER_AIR_DTOR: StateDtorLayout = state_dtor_layout(0x0014_0ee0, 0x4, 0x6660, 0x6618, 0x10);
const RIDER_RAILSLIDE_DTOR: StateDtorLayout = state_dtor_layout(0x0014_1040, 0x50, 0x6758, 0x6710, 0x60);
const RIDER_STATIC_DTOR: StateDtorLayout = state_dtor_layout(0x0014_11c0, 0, 0x6948, 0x6900, 0x10);
const RIDER_HANDPLANT_DTOR: StateDtorLayout = state_dtor_layout(0x0014_1320, 0xa0, 0x6a40, 0x69f8, 0xb0);
const RIDER_CRUISE_SUB_DTOR: StateDtorLayout = state_dtor_layout(0x0014_15c8, 0, 0x6b38, 0x6af0, 0x10);
const RIDER_BOARD_PRESS_SUB_DTOR: StateDtorLayout = state_dtor_layout(0x0014_1728, 0x14, 0x6c30, 0x6be8, 0x20);
const RIDER_PREWIND_SUB_DTOR: StateDtorLayout = state_dtor_layout(0x0014_1888, 0, 0x6d28, 0x6ce0, 0x10);
const RIDER_BUMP_SUB_DTOR: StateDtorLayout = state_dtor_layout(0x0014_19e8, 0, 0x6e20, 0x6dd8, 0x10);
const RIDER_NATURAL_AIR_SUB_DTOR: StateDtorLayout = state_dtor_layout(0x0014_1b48, 0x14, 0x6f18, 0x6ed0, 0x20);
const RIDER_SPIN_SUB_DTOR: StateDtorLayout = state_dtor_layout(0x0014_1ca8, 0x58, 0x7010, 0x6fc8, 0x60);
const RIDER_GATE_SUB_DTOR: StateDtorLayout = state_dtor_layout(0x0014_1e08, 0x14, 0x7108, 0x70c0, 0x20);
const RIDER_RAILSLIDE_SUB_DTOR: StateDtorLayout = state_dtor_layout(0x0014_1f68, 0x8, 0x7200, 0x71b8, 0x10);

/// `RiderWipeoutState_dtor` (0x136ad0): the state family with the sphere-tree
/// dtor (`this+0x44`, flags 3) before the rider call; its returns sit 0x18
/// further on because of that extra call.
const RIDER_WIPEOUT_DTOR: StateDtorLayout = StateDtorLayout {
    entry: 0x0013_6ad0,
    rider: 0x40,
    table: 0x6850,
    vtab: 0x6808,
    stride: 0x70,
    sphere: Some(0x44),
    ret_sphere: 0x0013_6c00,
    ret_rider: 0x0013_6c18,
    ret_delete: 0x0013_6c2c,
};

/// The state destructor body (generated: the 348-byte family and the wipeout
/// one, which shares the copy and differs in the flag block).
pub(crate) fn state_dtor_body(c: &mut Cpu, l: &StateDtorLayout) -> Flow {
    c.cycles += 21;
    let frame = Frame::open(c, 0x130);
    c.g[V0][0] = STATE_DTOR_DATA_PAGE as u64;
    frame.save(c, S0, 0x120);
    c.g[A2][0] = sx(STATE_DTOR_DATA_PAGE.wrapping_sub(l.table));
    frame.save(c, S1, 0x110);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x100);
    c.g[S1][0] = c.g[A1][0];
    c.g[A0][0] = STATE_DTOR_DATA_PAGE as u64;
    c.g[A3][0] = c.g[V0][0];
    let this = c.g[S0][0] as u32;
    c.g[V1][0] = Addr(this).off(l.rider).lw(c);
    c.g[A0][0] = sx(STATE_DTOR_DATA_PAGE.wrapping_sub(STATE_DTOR_LINK_K));
    Addr((c.g[V1][0] as u32).wrapping_add(0x6e8)).sw(c, c.g[A2][0]);
    c.g[V1][0] = STATE_DTOR_DATA_PAGE as u64;
    c.g[A1][0] = Addr(this).off(l.rider).lw(c);
    c.g[T2][0] = c.g[V1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(l.vtab));
    Addr((c.g[A1][0] as u32).wrapping_add(0x6d0)).sw(c, c.g[A0][0]);
    c.g[V0][0] = Addr(this).off(l.rider).lw(c);
    let skip_copy = c.g[S1][0] != 0;
    Addr((c.g[V0][0] as u32).wrapping_add(0x6c0)).sw(c, c.g[V1][0]);
    if !skip_copy {
        copy_state_table(c, &frame, this, l);
    }
    state_dtor_flags(c, &frame, this, l)?;
    c.cycles += 4;
    frame.restore(c, S1, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
    Ok(())
}

/// The table copies of a state destructor, run when the flags are zero:
/// two loops of 0x20-byte blocks (the first copies the data-page table to the
/// stack frame, the second the rider's table) with the event checks at their
/// back edges.
fn copy_state_table(c: &mut Cpu, frame: &Frame, this: u32, l: &StateDtorLayout) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A2][0];
    c.g[V1][0] = c.g[SP][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    loop {
        c.cycles += 11;
        let src = c.g[A1][0] as u32;
        let dst = c.g[V1][0] as u32;
        c.g[A0][0] = c.ld(src);
        c.g[A2][0] = c.ld(src.wrapping_add(0x8));
        c.g[T1][0] = c.ld(src.wrapping_add(0x10));
        c.g[T3][0] = c.ld(src.wrapping_add(0x18));
        c.sd(dst, c.g[A0][0]);
        c.sd(dst.wrapping_add(0x8), c.g[A2][0]);
        c.sd(dst.wrapping_add(0x10), c.g[T1][0]);
        c.sd(dst.wrapping_add(0x18), c.g[T3][0]);
        c.g[A1][0] = sx(src.wrapping_add(0x20));
        let again = c.g[A1][0] != c.g[V0][0];
        c.g[V1][0] = sx(dst.wrapping_add(0x20));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, l.entry + 0x64);
        }
    }
    c.cycles += 14;
    c.g[V0][0] = Addr(this).off(l.rider).lw(c);
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(l.table));
    c.g[A2][0] = c.ld(c.g[A1][0] as u32);
    c.sd(c.g[V1][0] as u32, c.g[A2][0]);
    Addr((c.g[V0][0] as u32).wrapping_add(0x6e8)).sw(c, c.g[SP][0]);
    c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(l.vtab));
    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
    c.g[V0][0] = Addr(this).off(l.rider).lw(c);
    c.g[A1][0] = c.g[T0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(l.stride));
    c.g[A3][0] = sx(this.wrapping_sub(c.g[V0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    let word8 = frame.at(c, 0x8);
    c.sh(word8, c.g[V1][0]);
    loop {
        c.cycles += 11;
        let src = c.g[A2][0] as u32;
        let dst = c.g[A1][0] as u32;
        c.g[V0][0] = c.ld(src);
        c.g[V1][0] = c.ld(src.wrapping_add(0x8));
        c.g[A0][0] = c.ld(src.wrapping_add(0x10));
        c.g[T3][0] = c.ld(src.wrapping_add(0x18));
        c.sd(dst, c.g[V0][0]);
        c.sd(dst.wrapping_add(0x8), c.g[V1][0]);
        c.sd(dst.wrapping_add(0x10), c.g[A0][0]);
        c.sd(dst.wrapping_add(0x18), c.g[T3][0]);
        c.g[A2][0] = sx(src.wrapping_add(0x20));
        let again = c.g[A2][0] != c.g[T1][0];
        c.g[A1][0] = sx(dst.wrapping_add(0x20));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, l.entry + 0xc8);
        }
    }
    c.cycles += 10;
    let src = c.g[A2][0] as u32;
    let dst = c.g[A1][0] as u32;
    c.g[V0][0] = c.ld(src);
    c.g[V1][0] = c.ld(src.wrapping_add(0x8));
    c.sd(dst, c.g[V0][0]);
    c.sd(dst.wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(l.vtab));
    c.g[A0][0] = Addr(this).off(l.rider).lw(c);
    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
    Addr((c.g[A0][0] as u32).wrapping_add(0x6c0)).sw(c, c.g[T0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    let word58 = frame.at(c, 0x58);
    c.sh(word58, c.g[V1][0]);
}

/// The flag block after the copy: the sphere-tree dtor (wipeout, when its
/// pointer is set), then the rider call for `flags & 2`, then
/// `__builtin_delete(this)` for `flags & 1`, each returning to its own
/// address.
fn state_dtor_flags(c: &mut Cpu, frame: &Frame, this: u32, l: &StateDtorLayout) -> Flow {
    if let Some(sphere_off) = l.sphere {
        c.cycles += 3;
        c.g[A0][0] = Addr(this).off(sphere_off).lw(c);
        let no_sphere = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        if !no_sphere {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, STATE_DTOR_SPHERE_TREE, l.ret_sphere)?;
            c.g[V0][0] = c.g[S1][0] & 0x2;
        }
        c.cycles += 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x2;
    }
    let no_rider_call = c.g[V0][0] == 0;
    c.g[V0][0] = c.g[S1][0] & 0x1;
    if !no_rider_call {
        c.cycles += 4;
        c.g[A0][0] = Addr(this).off(l.rider).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, STATE_DTOR_RIDER_CALL, l.ret_rider)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        frame.restore(c, S0, 0x120);
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, STATE_DTOR_DELETE, l.ret_delete)?;
        frame.restore(c, S0, 0x120);
    }
    Ok(())
}

pub fn rider_ground_state_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_GROUND_DTOR))
}

pub fn rider_air_state_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_AIR_DTOR))
}

pub fn rider_railslide_state_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_RAILSLIDE_DTOR))
}

pub fn rider_static_state_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_STATIC_DTOR))
}

pub fn rider_handplant_state_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_HANDPLANT_DTOR))
}

pub fn rider_cruise_sub_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_CRUISE_SUB_DTOR))
}

pub fn rider_board_press_sub_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_BOARD_PRESS_SUB_DTOR))
}

pub fn rider_prewind_sub_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_PREWIND_SUB_DTOR))
}

pub fn rider_bump_sub_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_BUMP_SUB_DTOR))
}

pub fn rider_natural_air_sub_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_NATURAL_AIR_SUB_DTOR))
}

pub fn rider_spin_sub_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_SPIN_SUB_DTOR))
}

pub fn rider_gate_sub_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_GATE_SUB_DTOR))
}

pub fn rider_railslide_sub_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_RAILSLIDE_SUB_DTOR))
}

pub fn rider_wipeout_state_dtor(c: &mut Cpu) {
    guest(c, |c| state_dtor_body(c, &RIDER_WIPEOUT_DTOR))
}

/// `void RiderHandplantState_Exit(RiderHandplantState *self)` (0x139178):
/// runs `RiderHandplantState_UpdateVelocity` and stores the v1 quad it
/// leaves in `this+0x10` to the rider's `+0x1e0`.
pub fn rider_handplant_state_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        RIDER_HANDPLANT_UPDATE_VELOCITY.call(c, 0x0013_918c)?;
        let this = c.g[S0][0] as u32;
        c.lq(V1, this.wrapping_add(0x10));
        c.g[V0][0] = Addr(this).off(0xa0).lw(c);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.sq(V1, (c.g[V0][0] as u32).wrapping_add(0x1e0));
        frame.close(c);
        Ok(())
    })
}

/// `RiderHandplantState_UpdateVelocity` (0x139548), called by the Exit.
const RIDER_HANDPLANT_UPDATE_VELOCITY: Callee = Callee {
    addr: 0x0013_9548,
    f: ee::s01::f_00139548,
};

/// `void RiderIface_dtor(RiderIface *self, int flags)` (0x140680): stores the
/// interface's vtable word and deletes `self` for `flags & 1`.
pub fn rider_iface_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(STATE_DTOR_DATA_PAGE.wrapping_sub(0x6338));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0x1;
        let no_delete = c.g[A1][0] == 0;
        Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
        if !no_delete {
            c.cycles += 2;
            call_ptr(c, STATE_DTOR_DELETE, 0x0014_06a4)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float Rider_GetHeading(Rider *r)` (0x140968): the heading of the
/// rider's vector at `+0x1b0` (x, then y): `atan(y / x)` with the quadrant
/// corrections from the constant table at gp, and the axis cases for `x == 0`
/// (`0` for `y == 0`, else one of two constants). The arctangent is
/// `Math_Atan` (0x31c228).
pub fn rider_get_heading(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0x20);
        c.mtc1(0, 22);
        let rider = c.g[A0][0] as u32;
        c.lq(V0, rider.wrapping_add(0x1b0));
        let at10 = frame.at(c, 0x10);
        c.sq(V0, at10);
        let at0 = frame.at(c, 0);
        c.sq(V0, at0);
        let at10 = frame.at(c, 0x10);
        c.lwc1(20, at10);
        let x_zero = FpuF32(c.f[20]).eq_s(FpuF32(c.f[22]));
        abi::set_fcc(c, x_zero);
        let at4 = frame.at(c, 0x4);
        c.lwc1(21, at4);
        let to_exit_b = if !x_zero {
            // x != 0: atan(y / x) with the quadrant correction.
            c.cycles += 7;
            c.f[12] = FpuF32(c.f[21]).div(FpuF32(c.f[20])).0;
            call_ptr(c, HEADING_ATAN, 0x0014_09d8)?;
            let x_neg = abi::c_lt_s(c, FpuF32(c.f[20]), FpuF32(c.f[22]));
            c.f[12] = c.f[0];
            if x_neg {
                c.cycles += 3;
                let y_pos = abi::c_lt_s(c, FpuF32(c.f[22]), FpuF32(c.f[21]));
                if y_pos {
                    c.cycles += 3;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6e30));
                    c.f[12] = (FpuF32(c.f[12]) + FpuF32(c.f[0])).0;
                } else {
                    c.cycles += 2;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6e2c));
                    c.f[12] = (FpuF32(c.f[12]) - FpuF32(c.f[0])).0;
                }
            }
            c.cycles += 1;
            c.f[0] = c.f[12];
            true
        } else {
            // x == 0: 0 for y == 0, else one of the two axis constants.
            c.cycles += 4;
            let y_zero = FpuF32(c.f[21]).eq_s(FpuF32(c.f[22]));
            abi::set_fcc(c, y_zero);
            c.f[0] = c.f[21];
            if y_zero {
                true
            } else {
                c.cycles += 4;
                let y_nonneg = FpuF32(c.f[22]).le(FpuF32(c.f[21]));
                abi::set_fcc(c, y_nonneg);
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6e38));
                frame.restore_ra(c, 0x20);
                if !y_nonneg {
                    true
                } else {
                    c.cycles += 3;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6e34));
                    frame.restore_fpr(c, 22, 0x40);
                    false
                }
            }
        };
        if to_exit_b {
            c.cycles += 2;
            frame.restore_ra(c, 0x20);
            frame.restore_fpr(c, 22, 0x40);
        }
        c.cycles += 4;
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `Math_Atan` (0x31c228) for `Rider_GetHeading`.
const HEADING_ATAN: u32 = 0x0031_c228;

/// `void Rider_SetBoostMode(Rider *r, int mode)` (0x140cf0): stores the mode
/// at `+0x304` and resets the boost meter (`Rider_ResetBoostMeter`).
pub fn rider_set_boost_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Addr((c.g[A0][0] as u32).wrapping_add(0x304)).sw(c, c.g[A1][0]);
        call_ptr(c, BOOST_METER_RESET, 0x0014_0d00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Rider_ResetBoostMeter` (0x120d58), called by `Rider_SetBoostMode`.
const BOOST_METER_RESET: u32 = 0x0012_0d58;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, DATA, GAME_GP, SP as STACK_TOP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 120;
    /// The gate word of the callee fakes, in the test's own machine.
    const CTRL: u32 = DATA + 0xf_f000;
    const MAGIC: u32 = 0x4f43_3435;
    const THIS: u32 = DATA + 0x400;
    const RIDER: u32 = DATA + 0x8000;
    const SPHERE: u32 = DATA + 0xa000;

    static PREV_RIDER_CALL: AtomicUsize = AtomicUsize::new(0);
    static PREV_DELETE: AtomicUsize = AtomicUsize::new(0);
    static PREV_SPHERE: AtomicUsize = AtomicUsize::new(0);
    static PREV_BOOST: AtomicUsize = AtomicUsize::new(0);

    /// Gated fakes: with the gate off they run the generated body they
    /// replaced (the callees are real game code and would walk the fixture).
    fn fake_rider_call(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_RIDER_CALL, ee::s00::f_0011b978__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, STATE_DTOR_RIDER_CALL, ret);
    }

    fn fake_delete(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_DELETE, ee::s10::f_00317e50__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, STATE_DTOR_DELETE, ret);
    }

    fn fake_sphere(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_SPHERE, ee::s10::f_00329970__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, STATE_DTOR_SPHERE_TREE, ret);
    }

    fn fake_boost(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &PREV_BOOST, ee::s00::f_00120d58__generated);
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, BOOST_METER_RESET, ret);
    }

    fn install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B978),
            fake_rider_call,
            &PREV_RIDER_CALL,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            fake_delete,
            &PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00329970),
            fake_sphere,
            &PREV_SPHERE,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_00120D58),
            fake_boost,
            &PREV_BOOST,
        );
    }

    /// The machine of a state destructor: the object at `THIS` with its
    /// rider (and sphere) pointers, flags covering all four combinations.
    fn dtor_setup(c: &mut Cpu, seed: u32, entry: u32, rider_off: u32, sphere: Option<u32>) {
        let mut r = machine(c, seed, entry as u64, 2);
        c.g[SP][0] = STACK_TOP as u64;
        c.pc = entry;
        c.bus.write32(CTRL, MAGIC);
        fill(c, &mut r, THIS, 0x40);
        c.bus.write32(THIS + rider_off, RIDER);
        if let Some(off) = sphere {
            c.bus.write32(THIS + off, if seed % 3 == 0 { 0 } else { SPHERE });
        }
        fill(c, &mut r, RIDER, 0x200);
        c.g[A0][0] = THIS as u64;
        c.g[A1][0] = (seed % 4) as u64;
    }

    /// `RiderXState_dtor(this, flags)` against the generated body.
    fn check_dtor(f: GuestFn, hand: GuestFn, entry: u32, rider_off: u32, sphere: Option<u32>) {
        install_fakes();
        check(f, hand, SEEDS, &|c, seed| {
            dtor_setup(c, seed, entry, rider_off, sphere);
        });
    }

    #[test]
    fn rider_wipeout_state_dtor_matches() {
        check_dtor(
            ee::s01::f_00136ad0__generated,
            rider_wipeout_state_dtor,
            0x0013_6ad0,
            0x40,
            Some(0x44),
        );
    }

    #[test]
    fn rider_ground_state_dtor_matches() {
        check_dtor(ee::s01::f_00140d80__generated, rider_ground_state_dtor, 0x0014_0d80, 0x18, None);
    }

    #[test]
    fn rider_air_state_dtor_matches() {
        check_dtor(ee::s01::f_00140ee0__generated, rider_air_state_dtor, 0x0014_0ee0, 0x4, None);
    }

    #[test]
    fn rider_railslide_state_dtor_matches() {
        check_dtor(ee::s01::f_00141040__generated, rider_railslide_state_dtor, 0x0014_1040, 0x50, None);
    }

    #[test]
    fn rider_static_state_dtor_matches() {
        check_dtor(ee::s01::f_001411c0__generated, rider_static_state_dtor, 0x0014_11c0, 0, None);
    }

    #[test]
    fn rider_handplant_state_dtor_matches() {
        check_dtor(ee::s01::f_00141320__generated, rider_handplant_state_dtor, 0x0014_1320, 0xa0, None);
    }

    #[test]
    fn rider_cruise_sub_dtor_matches() {
        check_dtor(ee::s01::f_001415c8__generated, rider_cruise_sub_dtor, 0x0014_15c8, 0, None);
    }

    #[test]
    fn rider_board_press_sub_dtor_matches() {
        check_dtor(ee::s01::f_00141728__generated, rider_board_press_sub_dtor, 0x0014_1728, 0x14, None);
    }

    #[test]
    fn rider_prewind_sub_dtor_matches() {
        check_dtor(ee::s01::f_00141888__generated, rider_prewind_sub_dtor, 0x0014_1888, 0, None);
    }

    #[test]
    fn rider_bump_sub_dtor_matches() {
        check_dtor(ee::s01::f_001419e8__generated, rider_bump_sub_dtor, 0x0014_19e8, 0, None);
    }

    #[test]
    fn rider_natural_air_sub_dtor_matches() {
        check_dtor(ee::s01::f_00141b48__generated, rider_natural_air_sub_dtor, 0x0014_1b48, 0x14, None);
    }

    #[test]
    fn rider_spin_sub_dtor_matches() {
        check_dtor(ee::s01::f_00141ca8__generated, rider_spin_sub_dtor, 0x0014_1ca8, 0x58, None);
    }

    #[test]
    fn rider_gate_sub_dtor_matches() {
        check_dtor(ee::s01::f_00141e08__generated, rider_gate_sub_dtor, 0x0014_1e08, 0x14, None);
    }

    #[test]
    fn rider_railslide_sub_dtor_matches() {
        check_dtor(ee::s01::f_00141f68__generated, rider_railslide_sub_dtor, 0x0014_1f68, 0x8, None);
    }

    #[test]
    fn rider_handplant_state_exit_matches() {
        install_fakes();
        check(
            ee::s01::f_00139178__generated,
            rider_handplant_state_exit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x139178, 2);
                c.g[SP][0] = STACK_TOP as u64;
                c.pc = 0x0013_9178;
                c.bus.write32(CTRL, MAGIC);
                c.g[A0][0] = THIS as u64;
                fill(c, &mut r, THIS, 0x40);
                c.bus.write32(THIS + 0xa0, RIDER);
                fill(c, &mut r, RIDER, 0x200);
            },
        );
    }

    #[test]
    fn rider_iface_dtor_matches() {
        install_fakes();
        check(ee::s01::f_00140680__generated, rider_iface_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x140680, 2);
            c.g[SP][0] = STACK_TOP as u64;
            c.pc = 0x0014_0680;
            c.bus.write32(CTRL, MAGIC);
            c.g[A0][0] = THIS as u64;
            c.g[A1][0] = (seed % 2) as u64 | (r.next_u32() as u64) << 8;
            fill(c, &mut r, THIS, 0x10);
        });
    }

    #[test]
    fn rider_set_boost_mode_matches() {
        install_fakes();
        check(ee::s01::f_00140cf0__generated, rider_set_boost_mode, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x140cf0, 2);
            c.g[SP][0] = STACK_TOP as u64;
            c.pc = 0x0014_0cf0;
            c.bus.write32(CTRL, MAGIC);
            c.g[A0][0] = THIS as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn rider_get_heading_matches() {
        check(ee::s01::f_00140968__generated, rider_get_heading, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x140968, 0);
            c.g[SP][0] = STACK_TOP as u64;
            c.pc = 0x0014_0968;
            c.g[A0][0] = THIS as u64;
            // The vector: x, then y. Axis cases and the quadrants by seed.
            let (x, y) = match seed % 6 {
                0 => (0, r.f32(3.0)),
                1 => (0, 0),
                2 => (r.f32(3.0), 0),
                _ => (r.f32(3.0), r.f32(3.0)),
            };
            c.bus.write32(THIS + 0x1b0, x);
            c.bus.write32(THIS + 0x1b4, y);
            c.bus.write32(GAME_GP - 0x6e38, r.next_u32());
            c.bus.write32(GAME_GP - 0x6e34, r.next_u32());
            c.bus.write32(GAME_GP - 0x6e30, r.next_u32());
            c.bus.write32(GAME_GP - 0x6e2c, r.next_u32());
        });
    }
}
