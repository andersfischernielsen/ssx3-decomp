//! Work package A2: rider getters, state hooks and the animation-sequence
//! helpers they call (`docs/investigation/phase2-candidates.md`, "A2 Rider &
//! animation"; field names from `docs/investigation/structs/rider.md`).
//!
//! Every function here is a leaf (no calls, no `c.event()` checks). Each one
//! charges the EE cycles of the blocks the original executes, writes the same
//! memory, and leaves the registers a caller (or a later spill) could observe
//! as the original does: `v0`/`v1` (including the upper 64 bits that `lq`
//! loads), `f0`/`f1`, the FPU condition bit, `LO`/`HI` of the `mult`s, and
//! for the one VU0 function its VF registers and Q. Caller-saved temporaries
//! (`at`, `a0`..`t9`, `f2`..`f19`) are not reproduced.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::{vu0_scratch, FpuF32, Vec4, VuF32};
use crate::rider::Rider;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_fe98, rider_get_main_state, "Rider_GetMainState"),
    (0x0011_fee8, rider_get_sub_state, "Rider_GetSubState"),
    (
        0x0011_ff48,
        rider_get_hips_position,
        "Rider_GetHipsPosition",
    ),
    (0x0011_d640, rider_is_loaded, "Rider_IsLoaded"),
    (0x0012_0e30, rider_update_focus_box, "Rider_UpdateFocusBox"),
    (
        0x0012_0d58,
        rider_reset_boost_meter,
        "Rider_ResetBoostMeter",
    ),
    (
        0x0011_9310,
        rider_metrix_get_score_tier,
        "RiderMetrix_GetScoreTier",
    ),
    (
        0x0013_6978,
        rider_static_state_post_update,
        "RiderStaticState_PostUpdate",
    ),
    (
        0x0031_2aa0,
        anim_controller_get_layer_anim_id,
        "AnimController_GetLayerAnimId",
    ),
    (0x0031_15c8, anim_table_get_entry, "AnimTable_GetEntry"),
    (
        0x0031_3cf0,
        anim_sequence_set_track_time,
        "AnimSequence_SetTrackTime",
    ),
    (
        0x0031_3a10,
        anim_sequence_set_weight_fade,
        "AnimSequence_SetWeightFade",
    ),
    (
        0x0031_3800,
        anim_sequence_update_weight_fade,
        "AnimSequence_UpdateWeightFade",
    ),
    (0x0031_3a20, anim_sequence_fade_out, "AnimSequence_FadeOut"),
    (
        0x0031_3d70,
        anim_sequence_get_track_blend_weight,
        "AnimSequence_GetTrackBlendWeight",
    ),
    (
        0x0031_3c50,
        anim_sequence_set_track_anim,
        "AnimSequence_SetTrackAnim",
    ),
    (
        0x0013_8b48,
        rider_handplant_state_enter,
        "RiderHandplantState_Enter",
    ),
    (
        0x0013_c5a0,
        rider_railslide_state_exit,
        "RiderRailslideState_Exit",
    ),
    (
        0x0013_1d08,
        rider_railslide_sub_enter,
        "RiderRailslideSub_Enter",
    ),
    (
        0x0012_fe98,
        rider_board_press_sub_exit,
        "RiderBoardPressSub_Exit",
    ),
];

// ---------------------------------------------------------------- registers

// ---------------------------------------------------------------- views

/// Typed accessors for the `Rider` fields this package uses (offsets from
/// `structs/rider.md`). An extension trait so it can live next to its users
/// until it moves into [`crate::rider`].
pub trait RiderFields {
    /// `+0x77c owner`: the `RiderBase` (Player / computer rider) containing
    /// this rider.
    fn owner(self, c: &mut Cpu) -> RiderBase;
    /// `+0x780 model`: the `RiderModel`.
    fn model(self, c: &mut Cpu) -> RiderModel;
    /// `+0x78c focusBox`: the FocusMan box holding a copy of the position
    /// (null until created).
    fn focus_box(self) -> Addr;
    /// `+0x880 loadStep`: 0..7, 7 = loaded.
    fn load_step(self) -> Addr;
    /// `+0x89c`: bone index of the hips.
    fn hips_bone(self) -> Addr;
    /// `+0x1e0 vel`: velocity (units/s).
    fn velocity(self) -> Addr;
    /// Slewed parameter `i` (`+0x1f0`, 19 × `{cur, maxStep, target}`).
    fn slew(self, i: u32) -> Slew;
    /// `+0x2f8 boostMeter` (0..1).
    fn boost_meter(self) -> Addr;
    /// `+0x304 boostMode`: 0 unlimited, 1 normal, 2 fast leak, 3 disabled.
    fn boost_mode(self) -> Addr;
    /// `+0xb24 boostMeterStart`.
    fn boost_meter_start(self) -> Addr;
    /// `+0x330 pressType`: board press 0 none, 1 nose, 2 tail.
    fn press_type(self) -> Addr;
    /// `+0x360 jumpRearm`: 1.0 while the jump button is up.
    fn jump_rearm(self) -> Addr;
    /// `+0x5ac`: rail flag of the take-off record.
    fn rail_flag(self) -> Addr;
}

impl RiderFields for Rider {
    fn owner(self, c: &mut Cpu) -> RiderBase {
        RiderBase(Addr(self.0.off(0x77c).read_u32(c)))
    }
    fn model(self, c: &mut Cpu) -> RiderModel {
        RiderModel(Addr(self.0.off(0x780).read_u32(c)))
    }
    fn focus_box(self) -> Addr {
        self.0.off(0x78c)
    }
    fn load_step(self) -> Addr {
        self.0.off(0x880)
    }
    fn hips_bone(self) -> Addr {
        self.0.off(0x89c)
    }
    fn velocity(self) -> Addr {
        self.0.off(0x1e0)
    }
    fn slew(self, i: u32) -> Slew {
        Slew(self.0.off(0x1f0 + 12 * i))
    }
    fn boost_meter(self) -> Addr {
        self.0.off(0x2f8)
    }
    fn boost_mode(self) -> Addr {
        self.0.off(0x304)
    }
    fn boost_meter_start(self) -> Addr {
        self.0.off(0xb24)
    }
    fn press_type(self) -> Addr {
        self.0.off(0x330)
    }
    fn jump_rearm(self) -> Addr {
        self.0.off(0x360)
    }
    fn rail_flag(self) -> Addr {
        self.0.off(0x5ac)
    }
}

/// The object that owns a [`Rider`] (Player / ComputerActorRider) and runs
/// its state machine.
#[derive(Clone, Copy, Debug)]
pub struct RiderBase(pub Addr);

impl RiderBase {
    /// `+0xde0 mainState`: 0 Ground, 1 Air, 2 Wipeout, 3 Static, 4 Railslide,
    /// 5 Handplant, 6 none.
    pub fn main_state(self) -> Addr {
        self.0.off(0xde0)
    }
    /// `+0xde4 subState`: 0 Cruise, 1 BoardPress, 2 Prewind, 3 Bump,
    /// 4 NaturalAir, 5 Spin, 6 Gate, 7 Railslide, 8 Wipeout, 9 Reset,
    /// 0xa FinishLine, 0xb Handplant, 0xc UberGrind, 0xd Void, 0xe none.
    pub fn sub_state(self) -> Addr {
        self.0.off(0xde4)
    }
}

/// `RiderModel` (0x160 bytes).
#[derive(Clone, Copy, Debug)]
pub struct RiderModel(pub Addr);

impl RiderModel {
    /// `+0x2c`: the bone transform array (0x20 bytes per bone: vec4
    /// translation, then the rotation quaternion).
    pub fn bones(self, c: &mut Cpu) -> Addr {
        Addr(self.0.off(0x2c).read_u32(c))
    }
}

/// One slewed rider parameter: `cur` moves toward `target` by at most
/// `maxStep` per frame (`Rider_UpdateSlewedParams`).
#[derive(Clone, Copy, Debug)]
pub struct Slew(pub Addr);

impl Slew {
    pub fn cur(self) -> Addr {
        self.0
    }
    pub fn max_step(self) -> Addr {
        self.0.off(4)
    }
    pub fn target(self) -> Addr {
        self.0.off(8)
    }
}

/// A main- or sub-state object of a `RiderBase`. Each holds a pointer to
/// the [`Rider`] (the "vbase pointer") at a state-specific offset.
#[derive(Clone, Copy, Debug)]
pub struct RiderStateObj(pub Addr);

impl RiderStateObj {
    fn rider_at(self, c: &mut Cpu, off: u32) -> Rider {
        Rider(Addr(self.0.off(off).read_u32(c)))
    }
}

/// An animation sequence: up to N tracks of 0x1c bytes from `+0`, then the
/// sequence weight / fade fields.
#[derive(Clone, Copy, Debug)]
pub struct AnimSequence(pub Addr);

impl AnimSequence {
    pub const TRACK_SIZE: i32 = 0x1c;

    /// `+0x94`: current sequence weight.
    pub fn weight(self) -> Addr {
        self.0.off(0x94)
    }
    /// `+0x98`: weight the fade moves toward.
    pub fn target_weight(self) -> Addr {
        self.0.off(0x98)
    }
    /// `+0x9c`: remaining fade time (0 = no fade running).
    pub fn fade_time(self) -> Addr {
        self.0.off(0x9c)
    }
    /// `+0xa0`: remove the sequence when the fade ends (set by a fade-out).
    pub fn remove_when_faded(self) -> Addr {
        self.0.off(0xa0)
    }
    /// `+0xa4`: length of the fade-out ramp at the end of a track.
    pub fn track_fade_out(self) -> Addr {
        self.0.off(0xa4)
    }
    /// `+0xa8`: length of the fade-in ramp at the start of a track.
    pub fn track_fade_in(self) -> Addr {
        self.0.off(0xa8)
    }
    /// `+0xc4`: dirty flag (tracks changed).
    pub fn dirty(self) -> Addr {
        self.0.off(0xc4)
    }
}

/// One track of an [`AnimSequence`].
#[derive(Clone, Copy, Debug)]
pub struct AnimTrack(pub Addr);

impl AnimTrack {
    /// `+4`: anim id (`id & 0xff` bank, `id >> 8` clip).
    pub fn anim_id(self) -> Addr {
        self.0.off(0x4)
    }
    /// `+8`: playback time.
    pub fn time(self) -> Addr {
        self.0.off(0x8)
    }
    /// `+0xc`: playback rate.
    pub fn rate(self) -> Addr {
        self.0.off(0xc)
    }
    /// `+0x10`: clip length (time).
    pub fn length(self) -> Addr {
        self.0.off(0x10)
    }
    /// `+0x14`: track weight.
    pub fn weight(self) -> Addr {
        self.0.off(0x14)
    }
    /// `+0x18`: active flag.
    pub fn active(self) -> Addr {
        self.0.off(0x18)
    }
    /// `+0x1c`: cleared on activation.
    pub fn reserved(self) -> Addr {
        self.0.off(0x1c)
    }
}

// ---------------------------------------------------------------- rider

/// `int Rider_GetMainState(Rider*)` (0x11fe98): the owner's main state.
pub fn rider_get_main_state(c: &mut Cpu) {
    c.cycles += 3;
    let owner = Rider(Addr(abi::arg(c, 0))).owner(c);
    abi::set_word(c, V1, owner.0 .0);
    let state = owner.main_state().read_u32(c);
    abi::set_ret(c, state);
    abi::ret(c);
}

/// `int Rider_GetSubState(Rider*)` (0x11fee8): the owner's sub state.
pub fn rider_get_sub_state(c: &mut Cpu) {
    c.cycles += 3;
    let owner = Rider(Addr(abi::arg(c, 0))).owner(c);
    abi::set_word(c, V1, owner.0 .0);
    let state = owner.sub_state().read_u32(c);
    abi::set_ret(c, state);
    abi::ret(c);
}

/// `Vec4 *Rider_GetHipsPosition(Vec4 *out, Rider *r)` (0x11ff48): copy the
/// hips bone's translation (all four lanes) to `out`; returns `out`.
pub fn rider_get_hips_position(c: &mut Cpu) {
    c.cycles += 9;
    let out = Addr(abi::arg(c, 0));
    let rider = Rider(Addr(abi::arg(c, 1)));
    let model = rider.model(c);
    c.g[V0][0] = c.g[A0][0];
    let hips = rider.hips_bone().read_u32(c);
    let bone = model.bones(c).off(hips << 5);
    let translation = bone.read_vec4(c);
    abi::set_gpr128(c, V1, translation.to_bits());
    out.write_vec4(c, translation);
    abi::ret(c);
}

/// `bool Rider_IsLoaded(Rider*)` (0x11d640): incremental loading finished
/// (`loadStep == 7`).
pub fn rider_is_loaded(c: &mut Cpu) {
    c.cycles += 4 + 2;
    let step = Rider(Addr(abi::arg(c, 0))).load_step().read_u32(c);
    abi::set_word(c, V1, step);
    c.g[V0][0] = (step == 7) as u64;
    abi::ret(c);
}

/// `void Rider_UpdateFocusBox(Rider*)` (0x120e30): mirror the position into
/// the FocusMan box, if there is one.
pub fn rider_update_focus_box(c: &mut Cpu) {
    c.cycles += 3;
    let rider = Rider(Addr(abi::arg(c, 0)));
    let focus_box = rider.focus_box().read_u32(c);
    abi::set_word(c, V1, focus_box);
    if focus_box != 0 {
        c.cycles += 2;
        let pos = rider.position().read_vec4(c);
        abi::set_gpr128(c, V0, pos.to_bits());
        Addr(focus_box).write_vec4(c, pos);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Rider_ResetBoostMeter(Rider*)` (0x120d58): the meter starts at
/// `boostMeterStart`, except empty when boost is disabled (mode 3) and full
/// when it is unlimited (mode 0).
pub fn rider_reset_boost_meter(c: &mut Cpu) {
    c.cycles += 5;
    let rider = Rider(Addr(abi::arg(c, 0)));
    // A raw 32-bit copy (lwc1/swc1 don't touch the bits).
    let start = rider.boost_meter_start().read_fpu(c);
    c.f[0] = start.0;
    c.g[V0][0] = 3;
    let mode = rider.boost_mode().read_u32(c);
    abi::set_word(c, V1, mode);
    rider.boost_meter().write_fpu(c, start);
    match mode {
        3 => {
            c.cycles += 2;
            rider.boost_meter().write_u32(c, 0);
        }
        0 => {
            c.cycles += 2 + 3 + 2;
            c.f[0] = FpuF32::ONE.0;
            rider.boost_meter().write_fpu(c, FpuF32::ONE);
        }
        _ => c.cycles += 2 + 2,
    }
    abi::ret(c);
}

/// `int RiderMetrix_GetScoreTier(RiderMetrix*, int64 score)` (0x119310):
/// trick score → tier 0..5 (thresholds 1000, 2500, 4000, 7500, 11500).
pub fn rider_metrix_get_score_tier(c: &mut Cpu) {
    let score = c.g[A1][0] as i64;
    c.cycles += 3;
    let tier: u64 = if score < 1000 {
        c.cycles += 2;
        0
    } else if score < 2500 {
        c.cycles += 2 + 2;
        1
    } else if score < 4000 {
        c.cycles += 2 + 2 + 2;
        2
    } else if score < 7500 {
        c.cycles += 2 + 2 + 2 + 2;
        3
    } else {
        // `movn`: v1 holds the alternative.
        c.cycles += 2 + 2 + 2 + 4;
        c.g[V1][0] = 4;
        if score < 11500 {
            4
        } else {
            5
        }
    };
    c.g[V0][0] = tier;
    abi::ret(c);
}

/// `RiderStaticState_PostUpdate` (0x136978): the Static main state has
/// nothing to do after the update.
pub fn rider_static_state_post_update(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------- anim

/// `uint AnimController_GetLayerAnimId(AnimController*, int layer)`
/// (0x312aa0): the anim id playing on one layer (u32 array at `+0`).
pub fn anim_controller_get_layer_anim_id(c: &mut Cpu) {
    c.cycles += 4;
    let ids = Addr(abi::arg(c, 0));
    let layer = abi::arg(c, 1);
    let id = ids.off(layer << 2).read_u32(c);
    abi::set_ret(c, id);
    abi::ret(c);
}

/// Base of the static anim table (28-byte entries).
const ANIM_TABLE: u32 = 0x0044_6990;

/// `AnimEntry *AnimTable_GetEntry(int i)` (0x3115c8): `&ANIM_TABLE[i]`.
pub fn anim_table_get_entry(c: &mut Cpu) {
    c.cycles += 6;
    const ENTRY_SIZE: i32 = 0x1c;
    c.g[V1][0] = ENTRY_SIZE as u64;
    let offset = abi::mult(c, abi::arg(c, 0) as u32, ENTRY_SIZE as u32);
    abi::set_ret(c, ANIM_TABLE.wrapping_add(offset));
    abi::ret(c);
}

/// The track `i` of `seq` (the index multiply is a `mult`, so it also sets
/// LO/HI).
fn track(c: &mut Cpu, seq: AnimSequence, i: u32) -> AnimTrack {
    let offset = abi::mult(c, i as u32, AnimSequence::TRACK_SIZE as u32);
    AnimTrack(seq.0.off(offset))
}

/// `void AnimSequence_SetTrackTime(float t, AnimSequence*, int track)`
/// (0x313cf0): set a track's playback time and mark the sequence dirty.
pub fn anim_sequence_set_track_time(c: &mut Cpu) {
    c.cycles += 7;
    let seq = AnimSequence(Addr(abi::arg(c, 0)));
    c.g[V0][0] = AnimSequence::TRACK_SIZE as u64;
    c.g[V1][0] = 1;
    let t = track(c, seq, abi::arg(c, 1));
    t.time().write_fpu(c, abi::farg(c, 0));
    seq.dirty().write_u32(c, 1);
    abi::ret(c);
}

/// `void AnimSequence_SetWeightFade(float target, float duration,
/// AnimSequence*)` (0x313a10): start fading the sequence weight to
/// `target` over `duration` seconds (not a fade-out).
pub fn anim_sequence_set_weight_fade(c: &mut Cpu) {
    c.cycles += 4;
    let seq = AnimSequence(Addr(abi::arg(c, 0)));
    seq.target_weight().write_fpu(c, abi::farg(c, 0));
    seq.fade_time().write_fpu(c, abi::farg(c, 1));
    seq.remove_when_faded().write_u32(c, 0);
    abi::ret(c);
}

/// `int AnimSequence_UpdateWeightFade(float dt, AnimSequence*)` (0x313800):
/// advance the weight fade by `dt`. The weight moves linearly to the target;
/// when the fade ends, returns 1 if the sequence should now be removed
/// (it was a fade-out), else 0.
pub fn anim_sequence_update_weight_fade(c: &mut Cpu) {
    c.cycles += 5;
    let seq = AnimSequence(Addr(abi::arg(c, 0)));
    let dt = abi::farg(c, 0);
    c.f[1] = 0;
    let fade = seq.fade_time().read_fpu(c);
    c.f[3] = fade.0;
    let no_fade = fade.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, no_fade);
    let mut remove = false;
    if !no_fade {
        c.cycles += 4;
        let ends = fade.le(dt);
        abi::set_fcc(c, ends);
        if !ends {
            // weight += (dt / fade) * (target - weight); fade -= dt.
            c.cycles += 8;
            let cur = seq.weight().read_fpu(c);
            let step = dt.div(fade);
            let target = seq.target_weight().read_fpu(c);
            let left = fade - dt;
            let diff = target - cur;
            seq.fade_time().write_fpu(c, left);
            let new = cur + step * diff;
            seq.weight().write_fpu(c, new);
            c.f[0] = diff.0;
            c.f[1] = new.0;
            c.f[2] = (step * diff).0;
            c.f[3] = left.0;
        } else {
            // The fade is over: snap to the target.
            c.cycles += 5;
            let target = seq.target_weight().read_fpu(c);
            c.f[0] = target.0;
            let flag = seq.remove_when_faded().read_u32(c);
            seq.weight().write_fpu(c, target);
            seq.fade_time().write_fpu(c, FpuF32::ZERO);
            remove = flag != 0;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = remove as u64;
    abi::ret(c);
}

/// `void AnimSequence_FadeOut(float t, AnimSequence*)` (0x313a20): fade the
/// weight to 0 over `t` seconds and remove the sequence afterwards. A
/// fade-out already running keeps the shorter of the two times.
pub fn anim_sequence_fade_out(c: &mut Cpu) {
    c.cycles += 6;
    let seq = AnimSequence(Addr(abi::arg(c, 0)));
    let t = abi::farg(c, 0);
    c.f[1] = 0;
    let target = seq.target_weight().read_fpu(c);
    c.f[0] = target.0;
    let fading_out = target.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, fading_out);
    if !fading_out {
        seq.fade_time().write_fpu(c, t);
        c.cycles += 1;
    } else {
        c.cycles += 5;
        let fade = seq.fade_time().read_fpu(c);
        c.f[0] = fade.0;
        let idle = fade.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, idle);
        if idle {
            seq.fade_time().write_fpu(c, t);
            c.cycles += 1;
        } else {
            c.cycles += 4;
            let shorter = t.lt(fade);
            abi::set_fcc(c, shorter);
            if shorter {
                c.cycles += 1 + 1;
                seq.fade_time().write_fpu(c, t);
            }
        }
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    seq.target_weight().write_u32(c, 0);
    seq.remove_when_faded().write_u32(c, 1);
    abi::ret(c);
}

/// `float AnimSequence_GetTrackBlendWeight(AnimSequence*, int track)`
/// (0x313d70): sequence weight × track weight, limited by a linear fade-in
/// over the first `track_fade_in` seconds and a fade-out over the last
/// `track_fade_out` seconds of the clip. Returned in f0.
pub fn anim_sequence_get_track_blend_weight(c: &mut Cpu) {
    c.cycles += 13;
    let seq = AnimSequence(Addr(abi::arg(c, 0)));
    let fade_in = seq.track_fade_in().read_fpu(c);
    let t = track(c, seq, abi::arg(c, 1));
    let seq_weight = seq.weight().read_fpu(c);
    abi::set_word(c, V0, t.0 .0);
    c.g[V1][0] = c.g[V0][0];
    let time = t.time().read_fpu(c);
    let track_weight = t.weight().read_fpu(c);
    c.f[0] = track_weight.0;
    let in_fade_in = time.lt(fade_in);
    let length = t.length().read_fpu(c);
    let mut w = seq_weight * track_weight;
    if in_fade_in {
        c.cycles += 2;
        let ramp = time.div(fade_in);
        c.f[0] = ramp.0;
        w = ramp.min(w);
    }
    c.cycles += 5;
    let remaining = length - time;
    let fade_out = seq.track_fade_out().read_fpu(c);
    c.f[0] = fade_out.0;
    let in_fade_out = remaining.lt(fade_out);
    abi::set_fcc(c, in_fade_out);
    if in_fade_out {
        c.cycles += 2;
        let ramp = remaining.div(fade_out);
        w = ramp.min(w);
    }
    c.cycles += 2;
    c.f[1] = w.0;
    c.f[2] = remaining.0;
    c.f[3] = length.0;
    c.f[4] = fade_in.0;
    abi::set_fret(c, w);
    abi::ret(c);
}

/// `void AnimSequence_SetTrackAnim(AnimSequence*, int track, uint animId)`
/// (0x313c50): put anim `id` on a track and set the track length from the
/// clip's frame count; a track that wasn't active is started (time 0,
/// rate 1, weight 1) and the sequence marked dirty.
pub fn anim_sequence_set_track_anim(c: &mut Cpu) {
    c.cycles += 28;
    let seq = AnimSequence(Addr(abi::arg(c, 0)));
    let id = abi::arg(c, 2);
    let gp = Addr(c.g[GP][0] as u32);
    // gp+0xd08: the anim bank table (pointer to an array of bank pointers);
    // gp-0x3224: seconds per frame. Both are filled in at runtime.
    let banks = Addr(gp.off(0xd08).read_u32(c));
    let t = track(c, seq, abi::arg(c, 1));
    let bank_slot = banks.off((id & 0xff) << 2);
    let clip_offset = abi::mult(c, (id >> 8) as u32, 0x14 as u32);
    let frame_time = gp.off(0u32.wrapping_sub(0x3224)).read_fpu(c);
    c.f[1] = frame_time.0;
    t.anim_id().write_u32(c, id);
    let bank = Addr(bank_slot.read_u32(c));
    let clips = bank.off(8).read_u32(c);
    let clip = Addr(clips).off(clip_offset);
    let frames = clip.off(0xc).read_u16(c) as u32;
    let length = FpuF32::from_i32(frames.wrapping_sub(1) as i32) * frame_time;
    c.f[0] = length.0;
    t.length().write_fpu(c, length);
    let active = t.active().read_u32(c);
    abi::set_word(c, V0, t.0 .0);
    if active != 0 {
        abi::set_word(c, V1, clips);
    } else {
        c.cycles += 10;
        t.time().write_u32(c, 0);
        t.rate().write_fpu(c, FpuF32::ONE);
        t.weight().write_fpu(c, FpuF32::ONE);
        t.active().write_u32(c, 1);
        t.reserved().write_u32(c, 0);
        seq.dirty().write_u32(c, 1);
        c.f[0] = FpuF32::ONE.0;
        c.g[V1][0] = 1;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------- state hooks

/// `RiderHandplantState_Enter(HandplantState*)` (0x138b48): the rider
/// glides to the handplant spot, so its velocity becomes
/// `(target - pos) / duration` (state `+0x30` target, `+0x84` duration;
/// rider pointer at `+0xa0`).
///
/// VU0 macro code. It works through three quadword slots of a 0x30-byte
/// stack frame, which keep their final values below the caller's `sp`.
pub fn rider_handplant_state_enter(c: &mut Cpu) {
    c.cycles += 21;
    let state = RiderStateObj(Addr(abi::arg(c, 0)));
    let frame = Addr((c.g[SP][0] as u32).wrapping_sub(0x30));
    let rider = state.rider_at(c, 0xa0);
    abi::set_word(c, V1, rider.0 .0);
    let target = state.0.off(0x30).read_vec4(c);
    let pos = rider.position().read_vec4(c);
    let delta = Vec4 {
        x: target.x - pos.x,
        y: target.y - pos.y,
        z: target.z - pos.z,
        w: target.w - pos.w,
    };
    frame.off(0x20).write_vec4(c, delta);
    frame.off(0x10).write_vec4(c, delta);
    let duration = state.0.off(0x84).read_u32(c);
    c.f[0] = duration;
    // `mfc1 a0` then `qmtc2 a0, vf3`: lanes z/w come from a0's upper half.
    c.g[A0][0] = sx(duration);
    let a0 = c.g[A0];
    let vf3 = [
        a0[0] as u32,
        (a0[0] >> 32) as u32,
        a0[1] as u32,
        (a0[1] >> 32) as u32,
    ];
    c.bus.plat.set_vf(3, vf3);
    // Q = vf0.w / duration = 1 / duration (VU divider).
    let inv = VuF32::div(VuF32::ONE, VuF32(duration));
    let vel = Vec4 {
        x: delta.x * inv,
        y: delta.y * inv,
        z: delta.z * inv,
        w: delta.w * inv,
    };
    frame.off(0x20).write_vec4(c, vel);
    abi::set_gpr128(c, V0, vel.to_bits());
    frame.write_vec4(c, vel);
    rider.velocity().write_vec4(c, vel);
    vu0_scratch::set_vf(c, 4, 0b1111, vel);
    vu0_scratch::set_vf(c, 5, 0b1111, delta);
    vu0_scratch::set_q(c, inv);
    abi::ret(c);
}

/// Bits of -0.25.
const MINUS_QUARTER: u32 = 0xbe80_0000;

/// `RiderRailslideState_Exit(RailslideState*)` (0x13c5a0): leaving a rail
/// resets slew #9 (rail-state value) to -0.25 moving toward 1.0 at the step
/// stored at gp-0x70f8 (1/30), and clears the rail flag.
///
/// A slew's `maxStep` is a maximum per tick, so at another rate the 1/30 entry
/// is `dt * 2` (exactly 1/30 at 60; the 60 Hz path still loads the entry).
pub fn rider_railslide_state_exit(c: &mut Cpu) {
    c.cycles += 15;
    let state = RiderStateObj(Addr(abi::arg(c, 0)));
    let rider = state.rider_at(c, 0x50);
    let slew = rider.slew(9);
    slew.max_step().write_u32(c, 0);
    slew.cur().write_u32(c, MINUS_QUARTER);
    slew.target().write_u32(c, MINUS_QUARTER);
    let base = Addr(c.g[GP][0] as u32);
    let step = if crate::tick::rate(c) == 60 {
        base.off(0u32.wrapping_sub(0x70f8)).read_fpu(c)
    } else {
        FpuF32::from_f32(crate::tick::dt(c) * 2.0)
    };
    slew.target().write_fpu(c, FpuF32::ONE);
    slew.max_step().write_fpu(c, step);
    rider.rail_flag().write_u32(c, 0);
    c.f[0] = step.0;
    c.f[1] = MINUS_QUARTER;
    c.f[2] = FpuF32::ONE.0;
    abi::set_word(c, V0, rider.0 .0);
    abi::set_word(c, V1, rider.0 .0);
    abi::ret(c);
}

/// `RiderRailslideSub_Enter(RailslideSub*)` (0x131d08): reset the
/// sub-state's two locals (0, -1), clear `jumpRearm`, and let slew #1
/// (body lean) relax to 0 at the step stored at gp-0x75ac (1/15).
pub fn rider_railslide_sub_enter(c: &mut Cpu) {
    c.cycles += 10;
    let state = RiderStateObj(Addr(abi::arg(c, 0)));
    let rider = state.rider_at(c, 0x8);
    c.g[V1][0] = u64::MAX;
    state.0.write_u32(c, 0);
    rider.jump_rearm().write_u32(c, 0);
    let step = Addr(c.g[GP][0] as u32)
        .off(0u32.wrapping_sub(0x75ac))
        .read_fpu(c);
    state.0.off(4).write_u32(c, u32::MAX);
    let slew = rider.slew(1);
    slew.max_step().write_fpu(c, step);
    slew.target().write_u32(c, 0);
    c.f[0] = step.0;
    abi::set_word(c, V0, rider.0 .0);
    abi::ret(c);
}

/// `RiderBoardPressSub_Exit(BoardPressSub*)` (0x12fe98): clear the board
/// press and let slews #11 (pivot, step 1/60) and #10 (press amount, step
/// 1/30) relax to 0. The steps are the words at gp-0x76d4 / gp-0x76d0; the
/// 1/60 entry follows the rate, the 1/30 one is `dt * 2` at another rate (a
/// per-tick maximum, exactly 1/30 at 60).
pub fn rider_board_press_sub_exit(c: &mut Cpu) {
    c.cycles += 11;
    let state = RiderStateObj(Addr(abi::arg(c, 0)));
    let rider = state.rider_at(c, 0x14);
    let gp = Addr(c.g[GP][0] as u32);
    let pivot_step = gp.off(0u32.wrapping_sub(0x76d4)).read_fpu(c);
    rider.press_type().write_u32(c, 0);
    let press_step = if crate::tick::rate(c) == 60 {
        gp.off(0u32.wrapping_sub(0x76d0)).read_fpu(c)
    } else {
        FpuF32::from_f32(crate::tick::dt(c) * 2.0)
    };
    let pivot = rider.slew(11);
    pivot.max_step().write_fpu(c, pivot_step);
    pivot.target().write_u32(c, 0);
    let press = rider.slew(10);
    press.max_step().write_fpu(c, press_step);
    press.target().write_u32(c, 0);
    c.f[0] = pivot_step.0;
    c.f[1] = press_step.0;
    abi::set_word(c, V0, rider.0 .0);
    abi::set_word(c, V1, rider.0 .0);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_vec, Rng, DATA};
    use ssx3_ee::{s00, s01, s10};

    const SEEDS: u32 = 400;

    fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 20) ^ salt)
    }

    /// Random values in every GPR (both halves) except zero/sp/ra/gp, in the
    /// FPU registers and the condition bit, and in the VU0 registers, so
    /// that every register a function leaves behind is checked.
    fn scramble(c: &mut Cpu, r: &mut Rng) {
        for reg in 1..32 {
            if !matches!(reg, 28 | 29 | 31) {
                let lo = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                let hi = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[reg] = [lo, hi];
            }
        }
        for f in 0..32 {
            c.f[f] = r.next_u32();
        }
        c.fcr31 = r.next_u32() & ssx3_ee_rt::fpu::FCC;
        for reg in 1..32 {
            c.bus.plat.set_vf(
                reg,
                [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
            );
        }
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
    }

    /// A float bit pattern biased toward the cases compares and PS2 rounding
    /// care about: ±0, denormals, exponent 255, 1.0, small magnitudes.
    fn float(r: &mut Rng) -> u32 {
        match r.next_u32() % 10 {
            0 => 0,
            1 => 0x8000_0000,
            2 => r.next_u32() & 0x807f_ffff, // denormal
            3 => 0x7f80_0000 | (r.next_u32() & 0x807f_ffff),
            4 => 0x3f80_0000,
            5 => r.f32(1.0),
            6 => r.f32(10.0),
            _ => r.f32(2.0) & 0x7fff_ffff,
        }
    }

    /// A pointer into test RAM (16-byte aligned).
    fn ptr(r: &mut Rng, region: u32) -> u32 {
        DATA + region * 0x10_0000 + (r.next_u32() & 0x7fff0)
    }

    #[test]
    fn rider_get_main_and_sub_state() {
        for (generated, hand) in [
            (
                s00::f_0011fe98__generated as GuestFn,
                rider_get_main_state as GuestFn,
            ),
            (s00::f_0011fee8__generated, rider_get_sub_state),
        ] {
            check(generated, hand, SEEDS, &|c, seed| {
                let mut r = rng(seed, 1);
                scramble(c, &mut r);
                let rider = ptr(&mut r, 0);
                let owner = ptr(&mut r, 1);
                c.g[A0][0] = sx(rider);
                c.bus.write32(rider + 0x77c, owner);
                c.bus.write32(
                    owner + 0xde0,
                    r.next_u32() % 7 | (r.next_u32() & 0x8000_0000),
                );
                c.bus.write32(owner + 0xde4, r.next_u32());
            });
        }
    }

    #[test]
    fn rider_get_hips_position_matches() {
        check(
            s00::f_0011ff48__generated,
            rider_get_hips_position,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 2);
                scramble(c, &mut r);
                let out = ptr(&mut r, 0) + (r.next_u32() & 0xf) * (seed & 1);
                let rider = ptr(&mut r, 1);
                let model = ptr(&mut r, 2);
                let bones = ptr(&mut r, 3);
                let hips = r.next_u32() % 64;
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(rider);
                c.bus.write32(rider + 0x780, model);
                c.bus.write32(rider + 0x89c, hips);
                c.bus.write32(model + 0x2c, bones + (r.next_u32() & 0x10));
                for i in 0..80 {
                    put_vec(
                        c,
                        bones + i * 16,
                        [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                    );
                }
            },
        );
    }

    #[test]
    fn rider_is_loaded_matches() {
        check(
            s00::f_0011d640__generated,
            rider_is_loaded,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 3);
                scramble(c, &mut r);
                let rider = ptr(&mut r, 0);
                c.g[A0][0] = sx(rider);
                let step = match seed % 3 {
                    0 => 7,
                    1 => r.next_u32() % 8,
                    _ => r.next_u32(),
                };
                c.bus.write32(rider + 0x880, step);
            },
        );
    }

    #[test]
    fn rider_update_focus_box_matches() {
        check(
            s00::f_00120e30__generated,
            rider_update_focus_box,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 4);
                scramble(c, &mut r);
                let rider = ptr(&mut r, 0);
                c.g[A0][0] = sx(rider);
                let focus_box = if seed % 3 == 0 { 0 } else { ptr(&mut r, 1) };
                c.bus.write32(rider + 0x78c, focus_box);
                put_vec(
                    c,
                    rider + 0x110,
                    [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
                );
            },
        );
    }

    #[test]
    fn rider_reset_boost_meter_matches() {
        check(
            s00::f_00120d58__generated,
            rider_reset_boost_meter,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 5);
                scramble(c, &mut r);
                let rider = ptr(&mut r, 0);
                c.g[A0][0] = sx(rider);
                let mode = match seed % 5 {
                    4 => r.next_u32(),
                    m => m,
                };
                c.bus.write32(rider + 0x304, mode);
                c.bus.write32(rider + 0xb24, r.next_u32());
                c.bus.write32(rider + 0x2f8, r.next_u32());
            },
        );
    }

    #[test]
    fn rider_metrix_get_score_tier_matches() {
        check(
            s00::f_00119310__generated,
            rider_metrix_get_score_tier,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 6);
                scramble(c, &mut r);
                const EDGES: [i64; 5] = [1000, 2500, 4000, 7500, 11500];
                let score: i64 = match seed % 4 {
                    0 => EDGES[(seed / 4 % 5) as usize] - 1 + (seed as i64 / 20 % 2),
                    1 => (r.next_u32() % 14000) as i64,
                    2 => ((r.next_u32() as i64) << 32) | r.next_u32() as i64,
                    _ => -((r.next_u32() % 5000) as i64),
                };
                c.g[A1][0] = score as u64;
            },
        );
    }

    #[test]
    fn rider_static_state_post_update_matches() {
        check(
            s01::f_00136978__generated,
            rider_static_state_post_update,
            50,
            &|c, seed| {
                let mut r = rng(seed, 7);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn anim_controller_get_layer_anim_id_matches() {
        check(
            s10::f_00312aa0__generated,
            anim_controller_get_layer_anim_id,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 8);
                scramble(c, &mut r);
                let ctl = ptr(&mut r, 0);
                let layer = r.next_u32() % 16;
                c.g[A0][0] = sx(ctl);
                c.g[A1][0] = layer as u64 | (c.g[A1][0] & !0xffff_ffff);
                c.bus.write32(ctl + layer * 4, r.next_u32());
            },
        );
    }

    #[test]
    fn anim_table_get_entry_matches() {
        check(
            s10::f_003115c8__generated,
            anim_table_get_entry,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 9);
                scramble(c, &mut r);
                if seed % 2 == 0 {
                    c.g[A0][0] = (r.next_u32() % 600) as u64;
                }
            },
        );
    }

    /// A sequence with random tracks and fade fields.
    fn put_sequence(c: &mut Cpu, r: &mut Rng, seq: u32) {
        for off in (0..0xd0).step_by(4) {
            c.bus.write32(seq + off, float(r));
        }
    }

    #[test]
    fn anim_sequence_set_track_time_matches() {
        check(
            s10::f_00313cf0__generated,
            anim_sequence_set_track_time,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 10);
                scramble(c, &mut r);
                let seq = ptr(&mut r, 0);
                put_sequence(c, &mut r, seq);
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = (r.next_u32() % 5) as u64;
            },
        );
    }

    #[test]
    fn anim_sequence_set_weight_fade_matches() {
        check(
            s10::f_00313a10__generated,
            anim_sequence_set_weight_fade,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 11);
                scramble(c, &mut r);
                let seq = ptr(&mut r, 0);
                put_sequence(c, &mut r, seq);
                c.g[A0][0] = sx(seq);
            },
        );
    }

    #[test]
    fn anim_sequence_update_weight_fade_matches() {
        check(
            s10::f_00313800__generated,
            anim_sequence_update_weight_fade,
            600,
            &|c, seed| {
                let mut r = rng(seed, 12);
                scramble(c, &mut r);
                let seq = ptr(&mut r, 0);
                put_sequence(c, &mut r, seq);
                c.g[A0][0] = sx(seq);
                c.f[12] = float(&mut r);
                if seed % 4 == 0 {
                    c.bus.write32(seq + 0x9c, c.f[12]);
                }
                c.bus
                    .write32(seq + 0xa0, if seed % 3 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn anim_sequence_fade_out_matches() {
        check(
            s10::f_00313a20__generated,
            anim_sequence_fade_out,
            600,
            &|c, seed| {
                let mut r = rng(seed, 13);
                scramble(c, &mut r);
                let seq = ptr(&mut r, 0);
                put_sequence(c, &mut r, seq);
                c.g[A0][0] = sx(seq);
                c.f[12] = float(&mut r);
                if seed % 5 == 0 {
                    c.bus.write32(seq + 0x9c, c.f[12]);
                }
            },
        );
    }

    #[test]
    fn anim_sequence_get_track_blend_weight_matches() {
        check(
            s10::f_00313d70__generated,
            anim_sequence_get_track_blend_weight,
            600,
            &|c, seed| {
                let mut r = rng(seed, 14);
                scramble(c, &mut r);
                let seq = ptr(&mut r, 0);
                put_sequence(c, &mut r, seq);
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = (r.next_u32() % 5) as u64;
            },
        );
    }

    #[test]
    fn anim_sequence_set_track_anim_matches() {
        check(
            s10::f_00313c50__generated,
            anim_sequence_set_track_anim,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 15);
                scramble(c, &mut r);
                let gp = c.g[GP][0] as u32;
                let seq = ptr(&mut r, 0);
                let table = ptr(&mut r, 1);
                let bank = ptr(&mut r, 2);
                let clips = ptr(&mut r, 3);
                put_sequence(c, &mut r, seq);
                let track = r.next_u32() % 5;
                let bank_ix = r.next_u32() & 0xff;
                let clip_ix = r.next_u32() % 0x400;
                c.g[A0][0] = sx(seq);
                c.g[A1][0] = track as u64;
                c.g[A2][0] = sx(clip_ix << 8 | bank_ix);
                c.bus.write32(gp + 0xd08, table);
                c.bus.write32(gp.wrapping_sub(0x3224), float(&mut r));
                c.bus.write32(table + bank_ix * 4, bank);
                c.bus.write32(bank + 8, clips);
                let frames = match seed % 4 {
                    0 => 0,
                    1 => 0xffff,
                    _ => r.next_u32() & 0x3ff,
                };
                c.bus.write16(clips + clip_ix * 0x14 + 0xc, frames as u16);
                let active = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(seq + track * 0x1c + 0x18, active);
            },
        );
    }

    #[test]
    fn rider_handplant_state_enter_matches() {
        check(
            s01::f_00138b48__generated,
            rider_handplant_state_enter,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 16);
                scramble(c, &mut r);
                let state = ptr(&mut r, 0);
                let rider = ptr(&mut r, 1);
                c.g[A0][0] = sx(state);
                c.bus.write32(state + 0xa0, rider);
                put_vec(
                    c,
                    state + 0x30,
                    [r.f32(1e4), r.f32(1e4), r.f32(1e4), float(&mut r)],
                );
                put_vec(
                    c,
                    rider + 0x110,
                    [r.f32(1e4), r.f32(1e4), r.f32(1e4), float(&mut r)],
                );
                c.bus.write32(state + 0x84, float(&mut r));
            },
        );
    }

    #[test]
    fn rider_state_slew_resets_match() {
        for (generated, hand, rider_at) in [
            (
                s01::f_0013c5a0__generated as GuestFn,
                rider_railslide_state_exit as GuestFn,
                0x50,
            ),
            (s00::f_00131d08__generated, rider_railslide_sub_enter, 0x8),
            (s00::f_0012fe98__generated, rider_board_press_sub_exit, 0x14),
        ] {
            check(generated, hand, SEEDS, &|c, seed| {
                let mut r = rng(seed, 17);
                scramble(c, &mut r);
                let gp = c.g[GP][0] as u32;
                let state = ptr(&mut r, 0);
                let rider = ptr(&mut r, 1);
                c.g[A0][0] = sx(state);
                c.bus.write32(state + rider_at, rider);
                for off in (0x1f0..0x5b0).step_by(4) {
                    c.bus.write32(rider + off, r.next_u32());
                }
                for off in [0x70f8, 0x75ac, 0x76d4, 0x76d0] {
                    c.bus.write32(gp - off, float(&mut r));
                }
            });
        }
    }

    /// The two 1/30 slew steps (`RiderRailslideState_Exit`'s rail value,
    /// `RiderBoardPressSub_Exit`'s press amount) are maximum moves per tick: at
    /// 120/240 Hz they are one 60 Hz frame's worth (`1/60`, `1/120`), so a slew
    /// covers the same amount per second. The 1/60 pivot step follows the
    /// `.lit4` entry. At 60 the entries are loaded unchanged.
    #[test]
    fn rider_state_slew_steps_follow_the_rate() {
        use crate::tick;
        let gp = crate::difftest::GAME_GP;
        let steps = |hz: u32| -> (u32, u32, u32) {
            let mut c = crate::difftest::fresh();
            c.g[GP][0] = gp as u64;
            // The game's values: 1/30 for the rail and the press, 1/60 (the
            // pool entry the rate patches) for the pivot.
            c.bus
                .write32(gp.wrapping_sub(0x70f8), (1.0f32 / 30.0).to_bits());
            c.bus
                .write32(gp.wrapping_sub(0x76d0), (1.0f32 / 30.0).to_bits());
            c.bus.write32(gp.wrapping_sub(0x76d4), tick::step_bits(hz));
            tick::set_for_test(&mut c, hz);
            // Railslide: state +0x50 -> rider, slew #9 (rail value).
            let state = 0x0080_0000u32;
            let rider = 0x0080_1000u32;
            c.bus.write32(state + 0x50, rider);
            c.g[A0][0] = state as u64;
            rider_railslide_state_exit(&mut c);
            let rail = c.bus.read32(rider + 0x1f0 + 9 * 12 + 4);
            // Board press: state +0x14 -> rider, slews #11 (pivot) and #10.
            let state = 0x0080_2000u32;
            let rider = 0x0080_3000u32;
            c.bus.write32(state + 0x14, rider);
            c.g[A0][0] = state as u64;
            rider_board_press_sub_exit(&mut c);
            let pivot = c.bus.read32(rider + 0x1f0 + 11 * 12 + 4);
            let press = c.bus.read32(rider + 0x1f0 + 10 * 12 + 4);
            (rail, pivot, press)
        };
        let (rail, pivot, press) = steps(60);
        assert_eq!(rail, (1.0f32 / 30.0).to_bits(), "60 Hz rail step");
        assert_eq!(press, (1.0f32 / 30.0).to_bits(), "60 Hz press step");
        assert_eq!(pivot, tick::step_bits(60), "60 Hz pivot step");
        for hz in [120u32, 240] {
            // A 1/30 step is two 60 Hz frames per tick, i.e. `2 / hz` per tick.
            let expected = (2.0f32 / hz as f32).to_bits();
            let (rail, pivot, press) = steps(hz);
            assert_eq!(rail, expected, "{hz} Hz rail step");
            assert_eq!(press, expected, "{hz} Hz press step");
            assert_eq!(pivot, tick::step_bits(hz), "{hz} Hz pivot step");
        }
    }
}
