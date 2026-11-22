//! Native AnimCombo constructor (0x00358c80), retained uninstalled.
//!
//! Generated EE is the authority. No tests, replays or builds were added or
//! run.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0035_8c80, anim_combo_ctor, "AnimCombo_ctor")];

const ANIM_OBJECT_CTOR: Callee = Callee {
    addr: 0x0034_1aa0,
    f: ssx3_ee::s11::f_00341aa0,
};

const COMBO_STEP: u32 = 0;
const COMBO_ACTIVE: u32 = 4;
const COMBO_START: u32 = 8;
const COMBO_END: u32 = 0xc;
const BLEND_A: u32 = 0x14;
const BLEND_B: u32 = 0x15;
const END_MODE_NONZERO: u32 = 0x16;
const END_MODE_NEGATIVE: u32 = 0x17;
const EMBEDDED_ANIM: u32 = 0x18;
const VTABLE: u32 = 0x54;
const PART_COUNT: u32 = 0x74;
const SNAPSHOT_MATRICES: u32 = 0x84;

const PARAM_IDLE_END: u32 = 0x10;
const PARAM_COMBO_START: u32 = 0x28;
const PARAM_COMBO_END: u32 = 0x2c;
const PARAM_COMBO_RATE: u32 = 0x30;
const PARAM_COMBO_END_MODE: u32 = 0x34;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `AnimCombo* AnimCombo_ctor(this, int process_arg, Instance*, Params*)`.
///
/// Forward the parameter block unchanged to the embedded AnimObject at +0x18
/// with native type 3. Thus its idle playback uses the normal AnimObject
/// parameters at +0..0x24. The combo adds frame30 start/end at params+0x28/
/// +0x2c, frame30/sec rate at +0x30, and signed end mode at +0x34.
///
/// A negative combo start substitutes the *raw* idle-end parameter at +0x10,
/// then divides it by 30; it does not read the embedded object's resolved end.
/// A negative combo end uses MDR+0x14 duration directly (already seconds).
/// The end-mode bytes store nonzero at this+0x16 and its sign bit at +0x17.
/// Allocate one 64-byte snapshot matrix per embedded animation part, without
/// initializing the matrix payload, and clear the active/blend selectors.
pub fn anim_combo_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0]; // Instance
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0]; // allocation
        c.g[S2][0] = c.g[A3][0]; // parameter block
        c.g[A0][0] = sx(at(c, S1, EMBEDDED_ANIM));
        frame.save(c, S3, 0x10);
        c.g[A2][0] = 3;
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S2][0];
        ANIM_OBJECT_CTOR.call(c, 0x0035_8cb8)?;

        c.g[V0][0] = 0x0049_0000;
        c.mtc1(ZERO, 0);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x13d8));
        c.g[V1][0] = sx(at(c, S1, COMBO_START));
        Addr(at(c, S1, VTABLE)).sw(c, c.g[V0][0]);
        c.f[1] = Addr(at(c, S2, PARAM_COMBO_START)).read_fpu(c).0;
        abi::set_fcc(c, FpuF32(c.f[1]).lt(FpuF32(c.f[0])));
        let explicit_start = !c.fcc();
        c.g[A0][0] = Addr(at(c, S0, 0x80)).lw(c); // MDR
        if !explicit_start {
            c.cycles += 1;
            c.f[1] = Addr(at(c, S2, PARAM_IDLE_END)).read_fpu(c).0;
        }

        c.cycles += 9;
        c.f[0] = Addr(r(c, GP).wrapping_sub(0x2a7c)).read_fpu(c).0;
        c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
        Addr(r(c, V1)).write_fpu(c, FpuF32(c.f[1]));
        c.mtc1(ZERO, 0);
        c.f[1] = Addr(at(c, S2, PARAM_COMBO_END)).read_fpu(c).0;
        abi::set_fcc(c, FpuF32(c.f[1]).lt(FpuF32(c.f[0])));
        let explicit_end = !c.fcc();
        c.g[V0][0] = sx(at(c, S1, COMBO_END));
        c.cycles += 2;
        if explicit_end {
            c.f[0] = Addr(r(c, GP).wrapping_sub(0x2a78)).read_fpu(c).0;
            c.f[0] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
        } else {
            c.f[0] = Addr(at(c, A0, 0x14)).read_fpu(c).0;
        }

        c.cycles += 25;
        Addr(r(c, V0)).write_fpu(c, FpuF32(c.f[0]));
        c.g[A1][0] = 0x0049_0000;
        c.g[V0][0] = Addr(at(c, GP, 0x2a74)).lw(c);
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x15e0));
        c.f[0] = Addr(at(c, S2, PARAM_COMBO_RATE)).read_fpu(c).0;
        c.g[A2][0] = 0x2000_0000;
        c.f[1] = Addr(r(c, GP).wrapping_sub(0x2a74)).read_fpu(c).0;
        c.f[2] = Addr(at(c, V0, 0x10)).read_fpu(c).0;
        c.f[2] = FpuF32::from_i32(c.f[2] as i32).0;
        c.g[S0][0] = Addr(at(c, S1, PART_COUNT)).lw(c);
        c.g[A3][0] = 0;
        c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
        c.g[S3][0] = sx(at(c, S1, SNAPSHOT_MATRICES));
        c.g[A0][0] = sx(r(c, S0) << 6);
        c.f[0] = FpuF32(c.f[0]).div(FpuF32(c.f[2])).0;
        Addr(at(c, S1, COMBO_STEP)).write_fpu(c, FpuF32(c.f[0]));
        c.g[V0][0] = Addr(at(c, S2, PARAM_COMBO_END_MODE)).lw(c);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        Addr(at(c, S1, END_MODE_NONZERO)).sb(c, c.g[V0][0]);
        c.g[V1][0] = Addr(at(c, S2, PARAM_COMBO_END_MODE)).lw(c);
        c.g[V1][0] = sx(r(c, V1) >> 31);
        Addr(at(c, S1, END_MODE_NEGATIVE)).sb(c, c.g[V1][0]);
        call_ptr(c, 0x0031_7e30, 0x0035_8d74)?;
        let no_parts = c.g[S0][0] == 0;
        c.g[V1][0] = sx(r(c, S0).wrapping_sub(1));
        if !no_parts {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }

        c.cycles += 12;
        Addr(r(c, S3)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S1][0];
        Addr(at(c, S1, BLEND_A)).sb(c, 0);
        Addr(at(c, S1, BLEND_B)).sb(c, 0);
        Addr(at(c, S1, COMBO_ACTIVE)).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}
