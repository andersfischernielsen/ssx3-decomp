//! Native AnimObject constructor (0x00341aa0).
//!
//! No automated tests or replays were added or run.
//! Generated EE is the authority for the guest ABI and operation order.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0034_1aa0, anim_object_ctor, "AnimObject_ctor")];

const BASE_CTOR: Callee = Callee {
    addr: 0x0034_d9b0,
    f: ssx3_ee::s11::f_0034d9b0,
};
const GET_DURATION: Callee = Callee {
    addr: 0x0035_1508,
    f: ssx3_ee::s11::f_00351508,
};

const MODE: u32 = 0;
const ACTIVE: u32 = 4;
const AT_LIMIT: u32 = 8;
const DELAY: u32 = 0xc;
const STEP: u32 = 0x10;
const START_BOUND: u32 = 0x14;
const END_BOUND: u32 = 0x18;
const POSITION: u32 = 0x1c;
const INITIAL_POSITION: u32 = 0x20;
const ANIM_BASE: u32 = 0x1c;
const VTABLE: u32 = 0x3c;
const FLAGS: u32 = 0x42;
const INTERSECT_INFO: u32 = 0x64;

const PARAM_MODE: u32 = 4;
const PARAM_DIRECTION: u32 = 8;
const PARAM_START_BOUND: u32 = 0xc;
const PARAM_END_BOUND: u32 = 0x10;
const PARAM_RATE: u32 = 0x14;
const PARAM_RATE_VARIATION: u32 = 0x18;
const PARAM_INITIAL_POSITION: u32 = 0x1c;
const PARAM_RANDOM_START: u32 = 0x20;
const PARAM_FORCE_OWNED: u32 = 0x24;

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    (c.g[register][0] as u32).wrapping_add(offset)
}

/// `AnimObject* AnimObject_ctor(AnimObject* self, int process_arg, int type,
/// Instance* instance, const AnimObjectParams* params)`.
///
/// The 44-byte factory parameter block has an instance handle at +0 (already
/// resolved before this call), mode at +4 (low u16), direction at +8, start
/// and end frames at +0xc/+0x10, rate and its variation at +0x14/+0x18,
/// initial frame at +0x1c, random-start flag at +0x20, and a force-owned flag
/// at +0x24. The final +0x28 word is not read by this constructor.
/// Mode 1 loops, mode 2 ping-pongs, and other modes use the clamp step.
/// Frame values use the original 30-fps scale; rate is divided by game ticks.
/// A negative start uses the current intersection animation's duration;
/// a negative end uses the resident MDR header's +0x14 duration. A negative
/// initial frame uses the start bound, then is clamped to both bounds.
///
/// BaseCtor receives self+0x1c and constructs the modifier/WorldObj subobject
/// at self+0x30 with the supplied Instance. Its Instance field is self+0x48,
/// and intersection animation info is self+0x64. Keep this native chain so
/// ownership, world registration, node matrices and Instance backlinks use
/// the real game object machinery.
pub fn anim_object_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[T0][0];
        c.g[A0][0] = sx(at(c, S1, ANIM_BASE));
        c.g[T0][0] = Addr(at(c, S2, PARAM_FORCE_OWNED)).lw(c);
        c.g[T0][0] = (c.g[T0][0] != 0) as u64;
        BASE_CTOR.call(c, 0x0034_1ad0)?;

        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = Addr(at(c, S1, INTERSECT_INFO)).lw(c);
        c.g[V0][0] = sx(0x0049_0b10);
        c.f[1] = 0;
        Addr(at(c, S1, VTABLE)).sw(c, c.g[V0][0]);
        let has_intersection = c.g[A0][0] != 0;
        c.g[S0][0] = Addr(at(c, S0, 0x80)).lw(c); // resident MDR
        if has_intersection {
            c.cycles += 3;
            GET_DURATION.call(c, 0x0034_1af4)?;
            c.f[1] = c.f[0];
        }

        // Start bound: frames / 30, or the intersection's current duration.
        c.cycles += 8;
        c.g[V0][0] = Addr(at(c, S2, PARAM_MODE)).lhu(c);
        c.f[0] = 0;
        Addr(at(c, S1, MODE)).sh(c, c.g[V0][0]);
        c.f[2] = Addr(at(c, S2, PARAM_START_BOUND)).read_fpu(c).0;
        abi::set_fcc(c, FpuF32(c.f[0]).le(FpuF32(c.f[2])));
        c.g[V1][0] = sx(at(c, S1, START_BOUND));
        if c.fcc() {
            c.cycles += 3;
            c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x2b94))
                .read_fpu(c)
                .0;
            c.f[0] = (FpuF32(c.f[2]) * FpuF32(c.f[0])).0;
        } else {
            c.cycles += 1;
            c.f[0] = c.f[1];
        }

        // End bound: frames / 30, or the complete MDR animation duration.
        c.cycles += 7;
        Addr(at(c, V1, 0)).write_fpu(c, FpuF32(c.f[0]));
        c.f[0] = 0;
        c.f[1] = Addr(at(c, S2, PARAM_END_BOUND)).read_fpu(c).0;
        abi::set_fcc(c, FpuF32(c.f[1]).lt(FpuF32(c.f[0])));
        c.g[V0][0] = sx(at(c, S1, END_BOUND));
        if c.fcc() {
            c.cycles += 2;
            c.f[0] = Addr(at(c, S0, 0x14)).read_fpu(c).0;
        } else {
            c.cycles += 2;
            c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x2b90))
                .read_fpu(c)
                .0;
            c.f[0] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
        }

        c.cycles += 7;
        Addr(at(c, V0, 0)).write_fpu(c, FpuF32(c.f[0]));
        c.f[0] = 0;
        c.f[1] = Addr(at(c, S2, PARAM_INITIAL_POSITION)).read_fpu(c).0;
        abi::set_fcc(c, FpuF32(c.f[1]).lt(FpuF32(c.f[0])));
        if c.fcc() {
            c.cycles += 3;
            c.f[1] = Addr(at(c, S1, START_BOUND)).read_fpu(c).0;
            c.f[12] = c.f[1];
        } else {
            c.f[12] = Addr(at(c, S1, START_BOUND)).read_fpu(c).0;
            c.cycles += 2;
            c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x2b8c))
                .read_fpu(c)
                .0;
            c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[0])).0;
        }

        c.cycles += 4;
        abi::set_fcc(c, FpuF32(c.f[1]).lt(FpuF32(c.f[12])));
        if c.fcc() {
            c.f[1] = c.f[12];
        }
        c.cycles += 5;
        c.f[13] = Addr(at(c, S1, END_BOUND)).read_fpu(c).0;
        abi::set_fcc(c, FpuF32(c.f[13]).lt(FpuF32(c.f[1])));
        if c.fcc() {
            c.f[1] = c.f[13];
        }

        c.cycles += 3;
        c.g[V0][0] = Addr(at(c, S2, PARAM_RANDOM_START)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            call_ptr(c, 0x0031_7830, 0x0034_1bbc)?; // AIrandf(min, max)
            c.f[1] = c.f[0];
        }
        Addr(at(c, S1, POSITION)).write_fpu(c, FpuF32(c.f[1]));

        // Rate and optional variation, both converted from 30-fps frames.
        c.cycles += 6;
        c.f[0] = 0;
        c.f[13] = Addr(at(c, S2, PARAM_RATE_VARIATION)).read_fpu(c).0;
        abi::set_fcc(c, FpuF32(c.f[13]).eq_s(FpuF32(c.f[0])));
        if c.fcc() {
            c.f[0] = Addr(at(c, S2, PARAM_RATE)).read_fpu(c).0;
            c.cycles += 7;
            c.g[V0][0] = Addr(at(c, GP, 0x2a74)).lw(c);
            c.f[1] = Addr((c.g[GP][0] as u32).wrapping_sub(0x2b84))
                .read_fpu(c)
                .0;
            c.f[2] = Addr(at(c, V0, 0x10)).read_fpu(c).0;
            c.f[2] = FpuF32::from_i32(c.f[2] as i32).0;
            c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[1])).0;
            c.f[0] = FpuF32(c.f[0]).div(FpuF32(c.f[2])).0;
        } else {
            c.cycles += 11;
            c.f[0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x2b88))
                .read_fpu(c)
                .0;
            c.f[12] = Addr(at(c, S2, PARAM_RATE)).read_fpu(c).0;
            c.f[13] = (FpuF32(c.f[13]) * FpuF32(c.f[0])).0;
            c.f[12] = (FpuF32(c.f[12]) * FpuF32(c.f[0])).0;
            call_ptr(c, 0x0031_7890, 0x0034_1bf0)?;
            c.g[V0][0] = Addr(at(c, GP, 0x2a74)).lw(c);
            c.f[1] = Addr(at(c, V0, 0x10)).read_fpu(c).0;
            c.f[1] = FpuF32::from_i32(c.f[1] as i32).0;
            c.f[0] = FpuF32(c.f[0]).div(FpuF32(c.f[1])).0;
        }
        Addr(at(c, S1, STEP)).write_fpu(c, FpuF32(c.f[0]));

        c.cycles += 4;
        c.g[V0][0] = Addr(at(c, S2, PARAM_DIRECTION)).lw(c);
        c.g[A0][0] = 1;
        if c.g[V0][0] == c.g[A0][0] {
            c.cycles += 4;
            c.f[0] = Addr(at(c, S1, STEP)).read_fpu(c).0;
            c.f[0] ^= 0x8000_0000; // neg.s
            Addr(at(c, S1, STEP)).write_fpu(c, FpuF32(c.f[0]));
        }
        c.g[V1][0] = Addr(at(c, S1, FLAGS)).lhu(c);

        c.cycles += 14;
        c.g[V0][0] = c.g[S1][0];
        c.f[0] = Addr(at(c, S1, POSITION)).read_fpu(c).0;
        c.g[V1][0] |= 1;
        Addr(at(c, S1, ACTIVE)).sw(c, c.g[A0][0]);
        Addr(at(c, S1, INITIAL_POSITION)).write_fpu(c, FpuF32(c.f[0]));
        Addr(at(c, S1, FLAGS)).sh(c, c.g[V1][0]);
        Addr(at(c, S1, DELAY)).sw(c, 0);
        Addr(at(c, S1, AT_LIMIT)).sw(c, 0);
        trace_animation(c, c.g[S1][0] as u32);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

fn trace_animation(c: &mut Cpu, object: u32) {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ENABLED.get_or_init(|| std::env::var_os("SSX3_TRACE_NATIVE_ANIM").is_some()) {
        return;
    }
    let instance = c.bus.read32(object + 0x48);
    let rid = c.bus.read32(instance + 0x78);
    if rid & 255 == 49 {
        eprintln!("[tricky] native AnimObject rid {rid:08x}, object {object:08x}, instance {instance:08x}");
    }
}
