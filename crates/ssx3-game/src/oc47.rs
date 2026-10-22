//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the char-database singleton, world-painter lighting and
//! irradiance lookups, the bolt-on FX interface, path event queries, the audio
//! system's listener/view counts, rider spin/friction helpers, the font trail
//! reset and the input-VM entry points, each with a differential test against
//! the generated body.
//!
//! - **`BE_GetCharDBInterface`** lazily creates the `cBENetworkInterface`
//!   singleton the game keeps in `D_004A1208`.
//! - **World painter** (`WorldPainter_GetLighting28`,
//!   `WorldPainter_GetIrradiance0/1`): resolve the lighting/irradiance entry
//!   through the world painter's `sRSMEntry` table and the
//!   `IrradianceRef` accessor.
//! - **`cBoltOnFXInterface_Update`**: refresh the emitters when the interface
//!   holds a world.
//! - **`Path_QueryEventsReversed`** queries a path's events with the reversed
//!   time window `[base - a, base - b]`.
//! - **Audio** (`SSXAudioSystem_GetListenerCount/GetViewCount`): counts come
//!   from the world's audio app; a world without an audio app has one
//!   listener.
//! - **Rider helpers**: `Rider_IsSpinningFast` (spin rate past ±0.925) and
//!   `RiderGroundState_ComputeFrictionAccel` (the hand-plant friction model).
//! - **`ComputerActorRider_FindHumanRiderIndex`** scans the six rider slots
//!   for the first AI rider with both character links set.
//! - **Input** (`InputMap_EvalFloat/EvalBool`, `InputBinding_EvalFloat`): run
//!   a binding's operand list through the input VM and return its value.
//! - **`FXTrail_Reset`** clears a font/flash trail's cursor and endpoints.
//!
//! Calls follow [`crate::call`]: the same registers at each call as the
//! original, the unwind check after every call. FPU arithmetic uses
//! [`FpuF32`] in the original's operation order and leaves the same registers.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_edd8, be_get_char_db_interface, "BE_GetCharDBInterface"),
    (0x002e_eff0, world_painter_get_lighting28, "WorldPainter_GetLighting28"),
    (0x0039_2d18, fx_trail_reset, "FXTrail_Reset"),
    (0x002e_e340, world_painter_get_irradiance1, "WorldPainter_GetIrradiance1"),
    (0x002e_e318, world_painter_get_irradiance0, "WorldPainter_GetIrradiance0"),
    (0x002e_cf78, c_bolton_fx_interface_update, "cBoltOnFXInterface_Update"),
    (0x0026_a090, path_query_events_reversed, "Path_QueryEventsReversed"),
    (0x0028_8ce8, ssx_audio_system_get_listener_count, "SSXAudioSystem_GetListenerCount"),
    (0x0028_b1f8, ssx_audio_system_get_view_count, "SSXAudioSystem_GetViewCount"),
    (0x0010_d8f8, computer_actor_rider_find_human_rider_index, "ComputerActorRider_FindHumanRiderIndex"),
    (0x0012_2c98, rider_is_spinning_fast, "Rider_IsSpinningFast"),
    (0x0013_c878, rider_ground_state_compute_friction_accel, "RiderGroundState_ComputeFrictionAccel"),
    (0x0032_1108, input_map_eval_bool, "InputMap_EvalBool"),
    (0x0032_0fa8, input_map_eval_float, "InputMap_EvalFloat"),
    (0x0032_0bf0, input_binding_eval_float, "InputBinding_EvalFloat"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    /// `cMemMan_alloc` (0x317d70, another shard: `call_indirect`).
    pub const MEM_MAN_ALLOC: u32 = 0x0031_7d70;

    pub const C_BOLTON_FX_REFRESH_EMITTERS: Callee =
        Callee { addr: 0x002e_ccb8, f: ee::s09::cBoltOnFXInterface_RefreshEmitters };
    pub const IRRADIANCE_REF_GET_ENTRY: Callee =
        Callee { addr: 0x002e_e010, f: ee::s09::IrradianceRef_GetEntry };
    pub const WORLD_PAINTER_GET_IRRADIANCE_NAME0: Callee =
        Callee { addr: 0x002e_f0e8, f: ee::s09::WorldPainter_GetIrradianceName0 };
    pub const WORLD_PAINTER_GET_IRRADIANCE_NAME1: Callee =
        Callee { addr: 0x002e_f140, f: ee::s09::WorldPainter_GetIrradianceName1 };
    pub const PATH_QUERY_EVENTS: Callee =
        Callee { addr: 0x0026_aa80, f: ee::s07::Path_QueryEvents };
    pub const SSX_AUDIO_SYSTEM_GET_APP: Callee =
        Callee { addr: 0x0028_b1c0, f: ee::s07::SSXAudioSystem_GetApp };
    pub const SSX_AUDIO_SYSTEM_GET_VIEW_COUNT: Callee =
        Callee { addr: 0x0028_b1f8, f: ee::s07::SSXAudioSystem_GetViewCount };
    pub const INPUT_VM_INIT: Callee = Callee { addr: 0x0032_5250, f: ee::s10::InputVM_Init };
    pub const INPUT_VM_EXECUTE: Callee =
        Callee { addr: 0x0032_5450, f: ee::s10::InputVM_Execute };
    pub const INPUT_MAP_EVAL_FLOAT: Callee =
        Callee { addr: 0x0032_0fa8, f: ee::s10::InputMap_EvalFloat };
}

/// `gp`-relative globals the originals address through `gp`.
const BE_NETWORK_INTERFACE: i32 = -0x1ee8; // D_004A1208
const WORLD: i32 = -0x848; // the world pointer (D_004A28A8)
const WORLD_PAINTER_TABLE: u32 = 0x004f_a370; // D_004FA370, sRSMEntry[0]
const SPIN_FAST_THRESHOLD: i32 = -0x78b4; // 0.9250819683074951f
const FRICTION_SLOPE: i32 = -0x70e8; // -0.0333333f
const BE_NETWORK_INTERFACE_VTABLE: u32 = 0x0045_ad38; // D_0045AD38
const BE_NETWORK_INTERFACE_INFO: u32 = 0x0045_a280; // D_0045A280

/// `cBENetworkInterface` fields the singleton constructor initialises.
mod network_interface {
    /// `field_0x8`.
    pub const FLAGS: u32 = 0x8;
    /// The vtable pointer.
    pub const VTABLE: u32 = 0xc;
}

/// `sRSMObj`: the object an `sRSMEntry` points at, and its vtable.
mod rsm_obj {
    /// `vt`.
    pub const VTABLE: u32 = 0x4;
    /// Entry 41's `{ short delta; short; void (*fn)(); }` pair.
    pub const SLOT_41: u32 = 0x148;
}

/// `RiderGroundState` fields used by the friction model, and the rider field
/// the model publishes its normalised slip to.
mod ground_state {
    /// `m4`: the start of the slip curve.
    pub const M4: u32 = 0x4;
    /// `m8`: the top of the slip curve.
    pub const M8: u32 = 0x8;
    /// `mRider`.
    pub const RIDER: u32 = 0x18;
    /// `p[7]`, the second friction coefficient.
    pub const COEFF_B: u32 = 0x1c;
    /// `Rider::unk_0x758`: the normalised slip published every update.
    pub const SLIP: u32 = 0x758;
}

/// `Path` field: the base time the query window is shifted by.
mod path {
    pub const BASE_TIME: u32 = 0x38;
}

/// `Rider::unk_5B4`: the current spin rate, radians per update.
const RIDER_SPIN_RATE: u32 = 0x5b4;

/// `ComputerActor`'s six `{ character, model }` rider links.
const ACTOR_RIDERS: u32 = 0x18;
const RIDER_LINKS: u32 = 6;
const RIDER_LINK_SIZE: u32 = 0x24;

/// `FXTrail` (font flash trail) fields: the count, the `{x,y,z}` cursor, and
/// the two endpoint times.
mod trail {
    pub const COUNT: u32 = 0x8;
    pub const START: u32 = 0xa0;
    pub const END: u32 = 0xa4;
}

/// The audio app / view objects: `world + 0x84` is the audio system, its
/// `+0x84` the app, and the app's `+0x10` the view count.
mod audio {
    pub const APP: u32 = 0x84;
    pub const LISTENER_COUNT: u32 = 0x10;
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// FPU register `n`.
#[inline(always)]
fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

// ---------------------------------------------------------------------------
// Char database.

/// `cBENetworkInterface *BE_GetCharDBInterface()` (0x14edd8): lazily create
/// the character-database network interface and return the singleton. The
/// object is 0x10 bytes with the type-info word at `+0xc`.
pub fn be_get_char_db_interface(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let slot = gp(c, BE_NETWORK_INTERFACE);
        let cached = Addr(slot).lw(c);
        c.g[V0][0] = cached;
        frame.save_ra(c, 0);
        if cached == 0 {
            // `cMemMan_alloc(0x10, type_info, 0, 0)`, then initialise it.
            c.cycles += 11;
            c.g[A1][0] = 0x0046_0000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx(BE_NETWORK_INTERFACE_INFO);
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, callee::MEM_MAN_ALLOC, 0x0014_ee00)?;
            let mem = c.g[V0][0] as u32;
            c.g[V1][0] = 0x0046_0000;
            Addr(mem).off(network_interface::FLAGS).sw(c, 0);
            c.g[V1][0] = sx(BE_NETWORK_INTERFACE_VTABLE);
            c.sw(mem.wrapping_add(network_interface::VTABLE), c.g[V1][0]);
            c.sw(slot, c.g[V0][0]);
        }
        c.cycles += 4;
        let instance = Addr(slot).lw(c);
        c.g[V0][0] = instance;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// World painter.

/// `float WorldPainter_GetLighting28(int entry)` (0x2eeff0): call slot 41 of
/// the `sRSMEntry` at index `entry` (0xf0 bytes each) and return the float it
/// points at.
pub fn world_painter_get_lighting28(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0xf0;
        let index = abi::arg(c, 0);
        // `mult a0, a0, v1`: the entry index scaled by sizeof(sRSMEntry)
        // (leaves LO/HI as the original does).
        let offset = abi::mult(c, index, 0xf0);
        c.g[V0][0] = 0x0050_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WORLD_PAINTER_TABLE);
        let entry = offset.wrapping_add(WORLD_PAINTER_TABLE);
        c.g[A0][0] = sx(entry);
        let p1c = Addr(entry).off(0x1c).lw(c);
        c.g[V1][0] = p1c;
        let obj = Addr(p1c as u32).lw(c);
        c.g[A2][0] = obj;
        let vt = Addr(obj as u32).off(rsm_obj::VTABLE).lw(c);
        c.g[A1][0] = vt;
        let delta = Addr(vt as u32).off(rsm_obj::SLOT_41).lh(c);
        c.g[A0][0] = delta;
        let method = Addr(vt as u32).off(rsm_obj::SLOT_41 + 4).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx((obj as u32).wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x002e_f028)?;
        // The method returns a pointer to the value.
        let value = c.g[V0][0] as u32;
        c.lwc1(0, value);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldPainter_GetIrradiance0()` (0x2ee318): look up entry 0 of the
/// irradiance table by name.
pub fn world_painter_get_irradiance0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME0.call(c, 0x002e_e328)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e330)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldPainter_GetIrradiance1()` (0x2ee340): look up entry 1 of the
/// irradiance table by name.
pub fn world_painter_get_irradiance1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::WORLD_PAINTER_GET_IRRADIANCE_NAME1.call(c, 0x002e_e350)?;
        c.g[A0][0] = c.g[V0][0];
        callee::IRRADIANCE_REF_GET_ENTRY.call(c, 0x002e_e358)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt-on FX.

/// `void cBoltOnFXInterface_Update(cBoltOnFXInterface *self)` (0x2ecf78):
/// refresh the emitters when the interface is attached to a world.
pub fn c_bolton_fx_interface_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let selfp = Addr(abi::arg(c, 0));
        let attached = selfp.lw(c);
        c.g[V0][0] = attached;
        frame.save_ra(c, 0);
        if attached != 0 {
            c.cycles += 2;
            callee::C_BOLTON_FX_REFRESH_EMITTERS.call(c, 0x002e_cf90)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Path events.

/// `int Path_QueryEventsReversed(Path *path, void *out, int maxOut,
/// float a, float b)` (0x26a090): query the events whose time lies in
/// `[base - a, base - b]`.
pub fn path_query_events_reversed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let path = Addr(abi::arg(c, 0));
        c.lwc1(0, path.off(path::BASE_TIME).0);
        let base = fr(c, 0);
        // The window is measured backwards from the path's base time.
        let upper = fr(c, 13);
        setf(c, 13, base - upper);
        let lower = fr(c, 12);
        setf(c, 12, base - lower);
        callee::PATH_QUERY_EVENTS.call(c, 0x0026_a0a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio system.

/// `int SSXAudioSystem_GetViewCount()` (0x28b1f8): `world->audio->app->count`.
pub fn ssx_audio_system_get_view_count(c: &mut Cpu) {
    c.cycles += 5;
    let world = Addr(gp(c, WORLD)).lw(c);
    c.g[V0][0] = world;
    let system = Addr(world as u32).off(audio::APP).lw(c);
    c.g[V1][0] = system;
    let app = Addr(system as u32).off(audio::APP).lw(c);
    c.g[A0][0] = app;
    let count = Addr(app as u32).off(audio::LISTENER_COUNT).lw(c);
    c.g[V0][0] = count;
    abi::ret(c);
}

/// `int SSXAudioSystem_GetListenerCount()` (0x288ce8): the app's view count,
/// or 1 when the world has no audio app.
pub fn ssx_audio_system_get_listener_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SSX_AUDIO_SYSTEM_GET_APP.call(c, 0x0028_8cf8)?;
        let world = c.g[V0][0] as u32;
        let system = Addr(world).off(audio::APP).lw(c);
        c.g[V1][0] = system;
        c.g[V0][0] = 1;
        if system != 0 {
            c.cycles += 2;
            callee::SSX_AUDIO_SYSTEM_GET_VIEW_COUNT.call(c, 0x0028_8d0c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider helpers.

/// `int Rider_IsSpinningFast(cRider *rider)` (0x122c98): true when the spin
/// rate is at least +0.925 or below -0.925 rad/update.
pub fn rider_is_spinning_fast(c: &mut Cpu) {
    c.cycles += 6;
    let rider = Addr(abi::arg(c, 0));
    c.lwc1(1, rider.off(RIDER_SPIN_RATE).0);
    c.lwc1(0, gp(c, SPIN_FAST_THRESHOLD));
    c.c_le_s(0, 1); // threshold <= rate
    c.g[V0][0] = 0;
    let mut fast = 0u64;
    if c.fcc() {
        c.cycles += 1;
        fast = 1;
    } else {
        c.cycles += 4;
        c.neg_s(0, 0); // -threshold
        c.c_lt_s(1, 0); // rate < -threshold
        if c.fcc() {
            c.cycles += 1;
            fast = 1;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = fast;
    abi::ret(c);
}

/// `float RiderGroundState_ComputeFrictionAccel(RiderGroundState *gs,
/// const float *p, float a, float b)` (0x13c878): the hand-plant ground
/// friction model. Returns the friction acceleration and publishes the
/// normalised slip (`max(-a / m8, 0)`) to `rider + 0x758`.
pub fn rider_ground_state_compute_friction_accel(c: &mut Cpu) {
    const M4: u32 = ground_state::M4;
    const M8: u32 = ground_state::M8;

    let gs = Addr(abi::arg(c, 0));
    let p = Addr(abi::arg(c, 1));
    let a = abi::farg(c, 0); // f12
    let b = abi::farg(c, 1); // f13

    c.cycles += 5;
    c.mtc1(ZERO, 1); // f1 = 0.0
    let zero = fr(c, 1);
    let m8;
    let r;
    if abi::c_lt_s(c, zero, a) {
        // a > 0: sliding at constant speed, `r = p0 * a * -1/30`.
        c.cycles += 7;
        let slope = Addr(gp(c, FRICTION_SLOPE)).read_fpu(c);
        setf(c, 0, slope);
        let b_positive = abi::c_lt_s(c, zero, b);
        let p0 = p.read_fpu(c);
        setf(c, 1, p0);
        let scaled = a * slope;
        setf(c, 0, scaled);
        m8 = gs.off(M8).read_fpu(c);
        setf(c, 7, m8);
        let product = p0 * scaled;
        setf(c, 5, product);
        if b_positive {
            c.cycles += 4;
            let p7 = p.off(ground_state::COEFF_B).read_fpu(c);
            setf(c, 0, p7);
            let drag = p7 * b;
            setf(c, 0, drag);
            setf(c, 5, product - drag);
        }
        r = fr(c, 5);
    } else {
        let m4 = gs.off(M4).read_fpu(c);
        setf(c, 6, m4);
        c.cycles += 5;
        let neg_m4 = -m4;
        setf(c, 0, neg_m4);
        if abi::c_lt_s(c, neg_m4, a) {
            // -m4 < a <= 0: `r = -p0 * a / m4`.
            c.cycles += 8;
            let p0 = p.read_fpu(c);
            setf(c, 0, p0);
            let p7 = p.off(ground_state::COEFF_B).read_fpu(c);
            setf(c, 1, p7);
            let neg_p0 = -p0;
            setf(c, 0, neg_p0);
            m8 = gs.off(M8).read_fpu(c);
            setf(c, 7, m8);
            let scaled = neg_p0 * a;
            setf(c, 0, scaled);
            let quotient = scaled.div(m4);
            setf(c, 5, quotient);
            let drag = p7 * b;
            setf(c, 1, drag);
            c.cycles += 1;
            setf(c, 5, quotient - drag);
            r = fr(c, 5);
        } else {
            // a <= -m4: the deep-slip curve from m4 to m8. `f0` (m8) is
            // loaded before the block, as the original does, and both the
            // load and the clamp read it.
            m8 = gs.off(M8).read_fpu(c);
            setf(c, 0, m8);
            c.cycles += 14;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 2); // f2 = 1.0
            setf(c, 3, -m8);
            setf(c, 7, m8);
            setf(c, 4, m8 - m4);
            let p7 = p.off(ground_state::COEFF_B).read_fpu(c);
            setf(c, 1, p7);
            let p0 = p.read_fpu(c);
            setf(c, 5, p0);
            let clamped = fr(c, 3).max(fr(c, 12));
            setf(c, 12, clamped);
            let drag = p7 * b;
            setf(c, 1, drag);
            let sum = clamped + m4;
            setf(c, 0, sum);
            let doubled = sum + sum;
            setf(c, 0, doubled);
            let span = fr(c, 4);
            let ratio = doubled.div(span);
            setf(c, 0, ratio);
            let one = fr(c, 2);
            let remaining = one - ratio;
            setf(c, 2, remaining);
            let curve = p0 * remaining;
            setf(c, 5, curve);
            c.cycles += 1;
            setf(c, 5, curve - drag);
            r = fr(c, 5);
        }
    }

    c.cycles += 8;
    let a_clamped = fr(c, 12);
    setf(c, 1, -a_clamped);
    let rider = gs.off(ground_state::RIDER).lw(c);
    c.g[V0][0] = rider;
    c.mtc1(ZERO, 2); // f2 = 0.0
    setf(c, 0, r);
    let neg_a = fr(c, 1);
    let slip_raw = neg_a.div(m8);
    setf(c, 1, slip_raw);
    let floor = fr(c, 2);
    setf(c, 1, slip_raw.max(floor));
    let slip = (rider as u32).wrapping_add(ground_state::SLIP);
    c.swc1(1, slip);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Computer riders.

/// `int ComputerActorRider_FindHumanRiderIndex(ComputerActor *self)`
/// (0x10d8f8): the first of the six rider links whose character and model
/// are both set, or -1.
pub fn computer_actor_rider_find_human_rider_index(c: &mut Cpu) {
    c.cycles += 2;
    let actor = Addr(abi::arg(c, 0));
    c.g[V1][0] = actor.off(ACTOR_RIDERS).lw(c);
    c.g[A0][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4).lw(c);
            if c.g[V0][0] == 0 {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[A0][0];
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < RIDER_LINKS as i64) as u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(RIDER_LINK_SIZE));
        if c.g[V0][0] != 0 {
            if event_due(c) {
                deliver_event_here(c, 0x0010_d900);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
        return;
    }
}

// ---------------------------------------------------------------------------
// Input VM.

/// `float InputMap_EvalFloat(InputMap *map, int value, int binding)`
/// (0x320fa8): run the binding's operand list through the input VM and return
/// its result.
pub fn input_map_eval_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S1, 0x1b0);
        frame.save(c, S0, 0x1c0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x1a0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x190);
        c.g[S2][0] = c.g[A1][0];
        callee::INPUT_VM_INIT.call(c, 0x0032_0fd0)?;
        // The operand list of binding `a2`: `ops + 4 * table[a2]`.
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        callee::INPUT_VM_EXECUTE.call(c, 0x0032_0ff8)?;
        frame.restore(c, S0, 0x1c0);
        frame.restore(c, S1, 0x1b0);
        frame.restore(c, S2, 0x1a0);
        frame.restore_ra(c, 0x190);
        frame.close(c);
        Ok(())
    })
}

/// `bool InputMap_EvalBool(InputMap *map, int value, int binding)`
/// (0x321108): [`input_map_eval_float`] with the result compared against 0.
pub fn input_map_eval_bool(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S1, 0x1b0);
        frame.save(c, S0, 0x1c0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x1a0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x190);
        c.g[S2][0] = c.g[A1][0];
        callee::INPUT_VM_INIT.call(c, 0x0032_1130)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        callee::INPUT_VM_EXECUTE.call(c, 0x0032_1158)?;
        c.mtc1(ZERO, 1); // f1 = 0.0
        c.c_eq_s(0, 1);
        let mut result = 1u64;
        if c.fcc() {
            // The VM's value is exactly 0.
            c.cycles += 1;
            result = 0;
        }
        c.g[V0][0] = result;
        c.cycles += 6;
        frame.restore(c, S0, 0x1c0);
        frame.restore(c, S1, 0x1b0);
        frame.restore(c, S2, 0x1a0);
        frame.restore_ra(c, 0x190);
        frame.close(c);
        Ok(())
    })
}

/// `float InputBinding_EvalFloat(InputBinding *binding, float value)`
/// (0x320bf0): evaluate the binding's map with the bound value.
pub fn input_binding_eval_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
        callee::INPUT_MAP_EVAL_FLOAT.call(c, 0x0032_0c08)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Font / flash trails.

/// `void FXTrail_Reset(FXTrail *trail)` (0x392d18): clear the trail's count,
/// its `{x,y,z}` cursor and the two endpoint times.
pub fn fx_trail_reset(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x10);
    // The original builds a zeroed {x,y,z} on the stack and moves it to the
    // trail as one doubleword; keep the same stack traffic and registers.
    let tmp = frame.at(c, 0);
    Addr(tmp).sw(c, 0);
    Addr(tmp + 4).sw(c, 0);
    Addr(tmp + 8).sw(c, 0);
    let trail = abi::arg(c, 0);
    // (+0xc cleared too, before the copy.)
    c.sw(trail.wrapping_add(0xc), 0);
    c.ldl(V0, tmp + 7);
    c.ldr(V0, tmp);
    c.g[V1][0] = Addr(tmp + 8).lw(c);
    c.sdl(V0, trail.wrapping_add(7));
    c.sdr(V0, trail);
    c.sw(trail.wrapping_add(trail::COUNT), c.g[V1][0]);
    c.sw(trail.wrapping_add(trail::END), 0);
    c.sw(trail.wrapping_add(trail::START), 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP, SP};

    const SEEDS: u32 = 200;

    /// The allocator mutex (`gp`-relative globals; see oc25's fixtures).
    const MEM_MUTEX: u32 = 0x0053_8b00;

    // ------------------------------------------------- char database singleton

    #[test]
    fn be_get_char_db_interface_matches() {
        check(ee::s01::f_0014edd8__generated, be_get_char_db_interface, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xedd8, 0);
            // Mark the allocator's mutex as held by "this thread" (the fake
            // `GetThreadId` returns 0x42) so the real allocator takes its fast
            // path.
            c.bus.write32(MEM_MUTEX + 4, 0x42);
            c.bus.write32(LOG_RESULTS, 0x42);
            c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
            let slot = GAME_GP.wrapping_sub(0x1ee8);
            // Null hits the allocation path, non-null returns the singleton.
            c.bus.write32(slot, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        });
    }

    // ---------------------------------------------------------- world painter

    /// `WorldPainter_GetLighting28` dispatches through slot 41 of a vtable:
    /// point it at [`FAKE`] so the call lands in the fake-call hook.
    #[test]
    fn world_painter_get_lighting28_matches() {
        check(ee::s09::f_002eeff0__generated, world_painter_get_lighting28, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xeff0, 0);
            let index = if seed % 2 == 0 { 1 } else { 1 + r.next_u32() % 0x20 };
            c.g[A0][0] = index as u64; // entry index (0xf0 bytes each)
            let entry = 0x004f_a370u32.wrapping_add(index * 0xf0);
            c.bus.write32(entry + 0x1c, DATA + 0x100); // entry->p1C
            c.bus.write32(DATA + 0x100, DATA + 0x200); // the object
            c.bus.write32(DATA + 0x204, DATA + 0x400); // o->vt
            c.bus.write16(DATA + 0x400 + 0x148, 0xfff8); // slot 41: delta -8
            c.bus.write32(DATA + 0x400 + 0x14c, FAKE); // slot 41: method
        });
    }

    /// `WorldPainter_GetIrradianceNameN` returns the static name buffer while
    /// its cache word is set, which keeps its vtable path out of the test.
    fn irradiance_fixture(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_add(0xa74), 1);
    }

    #[test]
    fn world_painter_get_irradiance0_matches() {
        check(ee::s09::f_002ee318__generated, world_painter_get_irradiance0, SEEDS, &|c, seed| {
            let mut r = Rng(0x002e_e318 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            irradiance_fixture(c);
        });
    }

    #[test]
    fn world_painter_get_irradiance1_matches() {
        check(ee::s09::f_002ee340__generated, world_painter_get_irradiance1, SEEDS, &|c, seed| {
            let mut r = Rng(0x002e_e340 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            irradiance_fixture(c);
        });
    }

    // --------------------------------------------------------------- bolt-on FX

    #[test]
    fn c_bolton_fx_interface_update_matches() {
        check(ee::s09::f_002ecf78__generated, c_bolton_fx_interface_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xcf78, 0);
            let selfp = DATA + 0x1000;
            c.g[A0][0] = selfp as u64;
            // The refresh walks the interface fields; a detached interface
            // (all zero past `+0`) keeps it on its short path.
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            // `BEBoltInt_GetInstance` returns this cached object, so the
            // refresh does not allocate.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f28), DATA + 0x200);
            c.bus.write32(selfp, if seed % 3 == 0 { 0 } else { 1 });
            let _ = r.next_u32();
        });
    }

    // ------------------------------------------------------------------- paths

    #[test]
    fn path_query_events_reversed_matches() {
        check(ee::s07::f_0026a090__generated, path_query_events_reversed, SEEDS, &|c, seed| {
            let mut r = Rng(0x0026_a090 ^ seed as u64);
            scramble(c, &mut r);
            let path = DATA + 0x1000;
            c.g[A0][0] = path as u64;
            c.g[A1][0] = (DATA + 0x3000) as u64; // out
            c.g[A2][0] = (seed % 4) as u64; // maxOut
            let count = seed % 4;
            c.bus.write32(path + 0x0, count);
            c.bus.write32(path + 0x4, DATA + 0x2000);
            // base 100, window [base-100, base-99] = [0, 1].
            c.bus.write32(path + 0x38, 100.0f32.to_bits());
            c.f[12] = 100.0f32.to_bits();
            c.f[13] = 99.0f32.to_bits();
            for i in 0..count {
                let e = DATA + 0x2000 + 16 * i;
                c.bus.write32(e + 8, 0.5f32.to_bits());
                c.bus.write32(e + 0xc, 0.75f32.to_bits());
            }
            let _ = r.next_u32();
        });
    }

    // ------------------------------------------------------------------- audio

    /// `world -> +0x84 -> +0x84 -> +0x10` (audio system, app, count).
    fn audio_world(c: &mut Cpu) -> u32 {
        let world = DATA + 0x1000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        world
    }

    #[test]
    fn ssx_audio_system_get_view_count_matches() {
        check(ee::s07::f_0028b1f8__generated, ssx_audio_system_get_view_count, SEEDS, &|c, seed| {
            let mut r = Rng(0x0028_b1f8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = audio_world(c);
            c.bus.write32(world + 0x84, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 0x10, r.next_u32());
        });
    }

    #[test]
    fn ssx_audio_system_get_listener_count_matches() {
        check(ee::s07::f_00288ce8__generated, ssx_audio_system_get_listener_count, SEEDS, &|c, seed| {
            let mut r = Rng(0x0028_8ce8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = audio_world(c);
            if seed % 2 == 0 {
                // No audio system: the listener count is the literal 1.
                c.bus.write32(world + 0x84, 0);
            } else {
                c.bus.write32(world + 0x84, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 0x10, r.next_u32());
            }
        });
    }

    // ------------------------------------------------------------------- riders

    #[test]
    fn computer_actor_rider_find_human_rider_index_matches() {
        check(ee::s00::f_0010d8f8__generated, computer_actor_rider_find_human_rider_index, SEEDS, &|c, seed| {
            let mut r = Rng(0x0010_d8f8 ^ seed as u64);
            scramble(c, &mut r);
            let actor = DATA + 0x1000;
            c.g[A0][0] = actor as u64;
            let riders = DATA + 0x2000;
            c.bus.write32(actor + 0x18, riders);
            // Slot `seed % 8` is the first one with both links set; 6 and 7
            // leave none, so the scan falls out of the table with -1.
            let hit = seed % 8;
            for i in 0..6u32 {
                let e = riders + 0x24 * i;
                let (first, second) = if i < hit {
                    if seed % 2 == 0 {
                        (r.next_u32(), 0)
                    } else {
                        (0, r.next_u32())
                    }
                } else if i == hit {
                    (r.next_u32() | 1, r.next_u32() | 1)
                } else {
                    (0, 0)
                };
                c.bus.write32(e, first);
                c.bus.write32(e + 4, second);
            }
        });
    }

    #[test]
    fn rider_is_spinning_fast_matches() {
        check(ee::s00::f_00122c98__generated, rider_is_spinning_fast, SEEDS, &|c, seed| {
            let mut r = Rng(0x0012_2c98 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let rider = DATA + 0x1000;
            c.g[A0][0] = rider as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x78b4), 0.925_081_97f32.to_bits());
            let rate: f32 = match seed % 6 {
                0 => 2.0,   // at or past +threshold
                1 => -2.0,  // below -threshold
                2 => 0.0,
                3 => 0.5,
                4 => 1.0,
                _ => -0.5,
            };
            c.bus.write32(rider + 0x5b4, rate.to_bits());
        });
    }

    /// `RiderGroundState_ComputeFrictionAccel` publishes a slip value and
    /// leaves caller-saved FPU registers that the race's caller may spill, so
    /// compare the soft registers as well, over varied `m4`/`m8`/`p` and every
    /// branch — including both sides of the deep-slip `max(-m8, a)` clamp.
    #[test]
    fn rider_ground_state_compute_friction_accel_matches() {
        for seed in 0..crate::difftest::seed_count(SEEDS) {
            let r = crate::difftest::compare(
                ee::s01::f_0013c878__generated,
                rider_ground_state_compute_friction_accel,
                &|c| friction_setup(c, seed),
            );
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} (soft): {:#?}", r.soft);
        }
    }

    fn friction_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0xc878, 0);
        let gs = DATA + 0x1000;
        let p = DATA + 0x1100;
        c.g[A0][0] = gs as u64;
        c.g[A1][0] = p as u64;
        let m4 = [0.25f32, 0.5, 1.0][(seed % 3) as usize];
        let m8 = m4 + [1.0f32, 2.5, 4.0][((seed / 3) % 3) as usize];
        let p0 = [0.25f32, -0.5, 1.5][(seed % 3) as usize];
        let p7 = [0.75f32, 0.125, -0.5][((seed / 2) % 3) as usize];
        c.bus.write32(gs + 0x4, m4.to_bits()); // m4
        c.bus.write32(gs + 0x8, m8.to_bits()); // m8
        c.bus.write32(gs + 0x18, DATA + 0x1200); // mRider
        c.bus.write32(p, p0.to_bits()); // p[0]
        c.bus.write32(p + 0x1c, p7.to_bits()); // p[7]
        c.bus.write32(GAME_GP.wrapping_sub(0x70e8), (-1.0f32 / 30.0).to_bits());
        let (a, b): (f32, f32) = match seed % 5 {
            0 => (2.0 * m4, 0.5), // a > 0, b > 0
            1 => (2.0 * m4, -0.5), // a > 0, b <= 0
            2 => (-0.5 * m4, 0.5), // -m4 < a <= 0
            3 => (-1.5 * m4, -0.5), // a <= -m4, clamp inactive
            _ => (-3.0 * m8, 1.0), // a <= -m8, clamp to -m8
        };
        c.f[12] = a.to_bits();
        c.f[13] = b.to_bits();
    }

    // --------------------------------------------------------------- input VM

    /// The input VM's opcode dispatch table and the interpreter's `stop`
    /// opcode handler. The handler is a label inside the interpreter itself,
    /// so the dispatcher continues there instead of tail-calling a separate
    /// function: one operand word ends the run.
    const INPUT_VM_TABLE: u32 = 0x0048_e330;
    const INPUT_VM_STOP: u32 = 0x0032_5ef8;
    const STOP_OPCODE: u32 = 0x20;

    /// `InputMap`: `+4` is the per-binding int array (indices into `+0x14`,
    /// the operand array). Every operand word stops the VM at once.
    fn input_map(c: &mut Cpu, map: u32) {
        c.bus.write32(map + 0x4, map + 0x80);
        c.bus.write32(map + 0x14, map + 0x100);
        for i in 0..4 {
            c.bus.write32(map + 0x80 + 4 * i, i); // operand offset in words
            c.bus.write32(map + 0x100 + 4 * i, STOP_OPCODE);
        }
        c.bus.write32(INPUT_VM_TABLE + 4 * STOP_OPCODE, INPUT_VM_STOP);
    }

    #[test]
    fn input_map_eval_float_matches() {
        check(ee::s10::f_00320fa8__generated, input_map_eval_float, SEEDS, &|c, seed| {
            let mut r = Rng(0x0032_0fa8 ^ seed as u64);
            scramble(c, &mut r);
            let map = DATA + 0x1000;
            c.g[A0][0] = map as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = (seed % 4) as u64; // binding
            input_map(c, map);
        });
    }

    #[test]
    fn input_map_eval_bool_matches() {
        check(ee::s10::f_00321108__generated, input_map_eval_bool, SEEDS, &|c, seed| {
            let mut r = Rng(0x0032_1108 ^ seed as u64);
            scramble(c, &mut r);
            let map = DATA + 0x1000;
            c.g[A0][0] = map as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = (seed % 4) as u64; // binding
            input_map(c, map);
            // The stopped VM leaves `*(parser + 8)` in f0 (parser = EvalBool's
            // frame): cover both compare outcomes.
            let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
            c.bus.write32(SP - 0x1d0 + 8, value.to_bits());
        });
    }

    #[test]
    fn input_binding_eval_float_matches() {
        check(ee::s10::f_00320bf0__generated, input_binding_eval_float, SEEDS, &|c, seed| {
            let mut r = Rng(0x0032_0bf0 ^ seed as u64);
            scramble(c, &mut r);
            let binding = DATA + 0x1000;
            let map = DATA + 0x2000;
            // The binding's second argument becomes the map's binding index.
            c.g[A0][0] = binding as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.bus.write32(binding, 0); // the map's binding index
            c.bus.write32(binding + 4, map);
            input_map(c, map);
        });
    }

    // --------------------------------------------------------- font FX trails

    #[test]
    fn fx_trail_reset_matches() {
        check(ee::s13::f_00392d18__generated, fx_trail_reset, SEEDS, &|c, seed| {
            let mut r = Rng(0x0039_2d18 ^ seed as u64);
            scramble(c, &mut r);
            let trail = DATA + 0x1000;
            c.g[A0][0] = trail as u64;
            // Old contents the reset must clear: the {x,y,z} cursor, the two
            // endpoint times, and the scratch doubleword on the stack.
            fill(c, &mut r, trail, 3);
            fill(c, &mut r, trail + 0xa0, 2);
            fill(c, &mut r, SP - 0x10, 4);
        });
    }
}
