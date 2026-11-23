//! Hooks for the in-place edits the Tricky fork made in the race/AI, track
//! info, camera, collision, script engine and draw files of ssx3-decomp.
//!
//! Every edit is a wrapper (`Chain`) around upstream's current port. Retail
//! courses (`custom_tracks::active() == None`) take the previous function
//! unchanged (a pure pass-through or a field patch that is skipped).
//!
//! | function | address | kind | what changed | uncertain |
//! |---|---|---|---|---|
//! | `ComputerActorRider_IsFarBehindHuman` | 0x0010d938 | wrapper | the rider's `+0x4d0` limit goes through `course_rules::distance`: the field is patched before the call and restored after | field is briefly patched; restored even if the call unwinds |
//! | `ComputerActorRider_IsFarAheadOfHuman` | 0x0010d990 | wrapper | same | same |
//! | `Rider_HandleRacePathEvent` | 0x0010e5d8 | wrapper | `course_rules::path_event(...)` at entry may swallow the event (early return) | none |
//! | `Rider_UpdateRaceProgress` | 0x00112338 | wrapper | `rail_diagnostics::observe` and `course_rules::update` at entry; the elevator `boost` block (enter Air, re-prime the air predictor) runs BEFORE the previous function instead of after its first `EA_MemCopy` | the block runs before the race-entry memcpy instead of after (independent data); a callee that unwinds mid-block drops this tick's progress update |
//! | `Rider_GetDistToFinish` | 0x00113128 | wrapper | result f0 goes through `course_rules::distance` | none |
//! | `Rider_GetStartDistToFinish` | 0x00113130 | none | the fork added an override reading `+0x4d8`; upstream already has the identical port | none |
//! | `AI_Restart` | 0x00129768 | wrapper | `course_rules::restart()` first | none |
//! | `BENewRaceInt_GetTrackInfo` | 0x00144bc0 | wrapper | v0 replaced by `custom_tracks::track_info()` after the call | none |
//! | `BENewRaceInt_GetTrackName` | 0x00144bf0 | wrapper | v0 replaced by `track_info() + 4` after the call | none |
//! | `BENewRaceInt_Commit` | 0x00144928 | wrapper | `custom_tracks::configure_native_run(c)` first | none |
//! | `BEScoreInt_GetHighScoreTable` | 0x0014ab20 | wrapper | custom course + the template's event id: return null without the call | none |
//! | `GameCamera_AvoidCollision` | 0x0015ee00 | wrapper (post-step) | the imported chase-camera retraction runs after the previous function: same query as the fork's `avoid_collision_script(.., chase=true)` and the same result | cycles of the extra query are approximate (36 charged); scratch is the freed 0x400 frame (offsets as the generated code) |
//! | `CollSegment_ctor` | 0x0032e100 | wrapper | `custom_tricky_recovery::prepare_ground_query` first | none |
//! | `OverlapList_CollideObjects` | 0x00334458 | wrapper | `custom_contact_trace::query` at entry; the per-candidate trace calls (before/after `CollideVolumeObject`) are SKIPPED (diagnostics, env-gated) | none |
//! | `ScriptEngine_QueueIntroScripts` | 0x0027aaf8 | wrapper | custom course: return without queueing | none |
//! | `PS2GraphicsMan_DrawModel` (capture.rs part) | 0x0037e238 | wrapper | movie screens always take the native path; `ModObj_Render`'s draw (RA 0x3565d8, object in S4) gets the crowd/movie material table in A2 | none |
//! | `packetless::draw_model` (is_screen part) | n/a | `packetless::set_opaque_blend_hook` | `fast_allowed = .. && !source_movie` and the `(5 << 2, 5 << 7)` blend inside the model loop | [`screen_blend`] |
//! | `containers.rs` heap diagnostics | n/a | skipped | diagnostics | |

use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::Cpu;
use ssx3_game::abi;
use ssx3_game::call::call_ptr;

/// Full replacements: none, every edit is a wrapper.
pub const OVERRIDES: &[(u32, ssx3_ee_rt::funcs::GuestFn, &str)] = &[];

static PREV_FAR_BEHIND: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_FAR_AHEAD: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_PATH_EVENT: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_UPDATE_PROGRESS: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_DIST_TO_FINISH: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_AI_RESTART: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_TRACK_INFO: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_TRACK_NAME: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_COMMIT: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_HIGH_SCORE_TABLE: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_AVOID_COLLISION: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_COLL_SEGMENT_CTOR: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_OVERLAP_COLLIDE: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_QUEUE_INTRO: ssx3_plugin::Prev = ssx3_plugin::Prev::new();
static PREV_DRAW_MODEL: ssx3_plugin::Prev = ssx3_plugin::Prev::new();

pub static CHAINS: &[crate::Chain] = &[
    crate::Chain { addr: 0x0010_d938, f: far_behind_human, prev: &PREV_FAR_BEHIND, name: "ComputerActorRider_IsFarBehindHuman" },
    crate::Chain { addr: 0x0010_d990, f: far_ahead_of_human, prev: &PREV_FAR_AHEAD, name: "ComputerActorRider_IsFarAheadOfHuman" },
    crate::Chain { addr: 0x0010_e5d8, f: handle_race_path_event, prev: &PREV_PATH_EVENT, name: "Rider_HandleRacePathEvent" },
    crate::Chain { addr: 0x0011_2338, f: update_race_progress, prev: &PREV_UPDATE_PROGRESS, name: "Rider_UpdateRaceProgress" },
    crate::Chain { addr: 0x0011_3128, f: get_dist_to_finish, prev: &PREV_DIST_TO_FINISH, name: "Rider_GetDistToFinish" },
    crate::Chain { addr: 0x0012_9768, f: ai_restart, prev: &PREV_AI_RESTART, name: "AI_Restart" },
    crate::Chain { addr: 0x0014_4bc0, f: get_track_info, prev: &PREV_TRACK_INFO, name: "BENewRaceInt_GetTrackInfo" },
    crate::Chain { addr: 0x0014_4bf0, f: get_track_name, prev: &PREV_TRACK_NAME, name: "BENewRaceInt_GetTrackName" },
    crate::Chain { addr: 0x0014_4928, f: commit, prev: &PREV_COMMIT, name: "BENewRaceInt_Commit" },
    crate::Chain { addr: 0x0014_ab20, f: get_high_score_table, prev: &PREV_HIGH_SCORE_TABLE, name: "BEScoreInt_GetHighScoreTable" },
    crate::Chain { addr: 0x0015_ee00, f: avoid_collision, prev: &PREV_AVOID_COLLISION, name: "GameCamera_AvoidCollision" },
    crate::Chain { addr: 0x0032_e100, f: coll_segment_ctor, prev: &PREV_COLL_SEGMENT_CTOR, name: "CollSegment_ctor" },
    crate::Chain { addr: 0x0033_4458, f: overlap_list_collide_objects, prev: &PREV_OVERLAP_COLLIDE, name: "OverlapList_CollideObjects" },
    crate::Chain { addr: 0x0027_aaf8, f: queue_intro_scripts, prev: &PREV_QUEUE_INTRO, name: "ScriptEngine_QueueIntroScripts" },
    crate::Chain { addr: 0x0037_e238, f: draw_model, prev: &PREV_DRAW_MODEL, name: "PS2GraphicsMan_DrawModel" },
];

// ---------------------------------------------------------------------------
// ai/computer.rs

/// Patch the actor's rider's `+0x4d0` (distance to finish) with the lap-aware
/// distance; the generated body reads it for the comparison. Returns what to
/// restore.
fn patch_limit(c: &mut Cpu) -> Option<(u32, u32)> {
    if crate::custom_tracks::active().is_none() {
        return None;
    }
    let actor = c.g[A0][0] as u32;
    let rider = c.bus.read32(actor.wrapping_add(0x18));
    if rider == 0 {
        return None;
    }
    let field = rider.wrapping_add(0x4d0);
    let original = c.bus.read32(field);
    let adjusted = crate::custom_course_rules::distance(c, rider, f32::from_bits(original)).to_bits();
    if adjusted == original {
        return None;
    }
    c.bus.write32(field, adjusted);
    Some((field, original))
}

fn far_comparison(c: &mut Cpu, prev: &ssx3_plugin::Prev) {
    let patch = patch_limit(c);
    prev.call(c);
    if let Some((field, original)) = patch {
        c.bus.write32(field, original);
    }
}

/// `ComputerActorRider_IsFarBehindHuman` (0x10d938).
fn far_behind_human(c: &mut Cpu) {
    far_comparison(c, &PREV_FAR_BEHIND);
}

/// `ComputerActorRider_IsFarAheadOfHuman` (0x10d990).
fn far_ahead_of_human(c: &mut Cpu) {
    far_comparison(c, &PREV_FAR_AHEAD);
}

/// `Rider_HandleRacePathEvent(rider, event, a2, a3)` (0x10e5d8): custom lap
/// rules may swallow the event.
fn handle_race_path_event(c: &mut Cpu) {
    if crate::custom_course_rules::path_event(
        c,
        c.g[A0][0] as u32,
        c.g[A1][0] as u32,
        c.g[A2][0] == 0 && c.g[A3][0] != 0,
        c.g[A3][0] != 0,
    ) {
        abi::ret(c);
        return;
    }
    PREV_PATH_EVENT.call(c);
}

/// `Rider_UpdateRaceProgress(rider)` (0x112338).
fn update_race_progress(c: &mut Cpu) {
    let rider = c.g[A0][0] as u32;
    crate::custom_rail_diagnostics::observe(c, rider);
    if let Some(boost) = crate::custom_course_rules::update(c, rider) {
        if !apply_boost(c, rider, boost.take_off) {
            return;
        }
    }
    PREV_UPDATE_PROGRESS.call(c);
}

/// The fork's boost block: ground motion projects away an elevator's vertical
/// velocity, so enter native Air before it processes the authored impulse,
/// and re-prime the air predictor like `Rider_AddVelocity`. Registers, the
/// float registers and `pc` are restored. False when a callee unwound.
fn apply_boost(c: &mut Cpu, rider: u32, take_off: bool) -> bool {
    const RET: u32 = 0x0011_2378;
    let (registers, floats, pc) = (c.g, c.f, c.pc);
    if take_off {
        c.g[A0][0] = sx(rider);
        c.g[A1][0] = 1;
        if call_ptr(c, 0x0011_fe78, RET).is_err() {
            return false;
        }
        c.g[A0][0] = sx(rider);
        c.g[A1][0] = 0;
        if call_ptr(c, 0x0011_fec8, RET).is_err() {
            return false;
        }
    }
    // Rider_AddVelocity primes the predictor from its saved trajectory, so a
    // velocity/altitude edit must re-prime it too.
    let owner = c.bus.read32(rider + 0x77c);
    if owner != 0 && c.bus.read32(owner + 0xde0) == 1 {
        c.g[A0][0] = c.lw(rider + 0x788);
        c.g[A1][0] = sx(rider + 0x110);
        c.g[A2][0] = sx(rider + 0x1e0);
        c.f[12] = c.bus.read32((registers[GP][0] as u32).wrapping_sub(0x7880));
        if call_ptr(c, 0x0011_35b8, RET).is_err() {
            return false;
        }
    }
    c.g = registers;
    c.f = floats;
    c.pc = pc;
    true
}

/// `Rider_GetDistToFinish(rider)` (0x113128): custom lap races include the
/// future circuits in this query.
fn get_dist_to_finish(c: &mut Cpu) {
    let rider = c.g[A0][0] as u32;
    PREV_DIST_TO_FINISH.call(c);
    if crate::custom_tracks::active().is_some() {
        let local = f32::from_bits(c.f[0]);
        c.f[0] = crate::custom_course_rules::distance(c, rider, local).to_bits();
    }
}

// ---------------------------------------------------------------------------
// ai/ai.rs

/// `AI_Restart(ai, state)` (0x129768).
fn ai_restart(c: &mut Cpu) {
    crate::custom_course_rules::restart();
    PREV_AI_RESTART.call(c);
}

// ---------------------------------------------------------------------------
// be/beintnewrace.rs, be/belibrary.rs, state.rs

/// `BENewRaceInt_GetTrackInfo()` (0x144bc0).
fn get_track_info(c: &mut Cpu) {
    PREV_TRACK_INFO.call(c);
    if let Some(info) = crate::custom_tracks::track_info() {
        abi::set_ret(c, info);
    }
}

/// `BENewRaceInt_GetTrackName(index)` (0x144bf0).
fn get_track_name(c: &mut Cpu) {
    PREV_TRACK_NAME.call(c);
    if let Some(info) = crate::custom_tracks::track_info() {
        c.g[V0][0] = sx(info + 4);
    }
}

/// `BENewRaceInt_Commit()` (0x144928).
fn commit(c: &mut Cpu) {
    crate::custom_tracks::configure_native_run(c);
    PREV_COMMIT.call(c);
}

/// `BEScoreInt_GetHighScoreTable(a, b)` (0x14ab20): the bounded template
/// course's official leaderboard does not describe the active custom course.
fn get_high_score_table(c: &mut Cpu) {
    if crate::custom_tracks::active().is_some() && abi::arg(c, 0) == c.bus.read32(0x0053_05f0) {
        abi::set_ret(c, 0);
        abi::ret(c);
        return;
    }
    PREV_HIGH_SCORE_TABLE.call(c);
}

// ---------------------------------------------------------------------------
// camera/camera.rs

const CAM_LOOK_AT: u32 = 0xe0;
const CAM_EYE: u32 = 0x100;
const CAM_CONTROLLER: u32 = 0xa8;
const CAM_SCRIPT_EYE: u32 = 0x490;
const CAM_SCRIPT_BLOCKED: u32 = 0x4a0;
const AVOID_UP: u32 = 0x004f_f160;
const COLL_SEGMENT_CTOR: u32 = 0x0032_e100;
const NASH_FIND_CLOSEST_CONTACT: u32 = 0x0033_6850;
const IS_SCRIPT_CAMERA_ACTIVE: u32 = 0x0015_d020;

/// `GameCamera_AvoidCollision(cam)` (0x15ee00). Upstream runs unchanged; for
/// an imported ordinary chase view (no script camera) the line-of-sight query
/// of the script camera is then run on the finished eye and the eye is pulled
/// back to the rider's side of the first obstruction.
fn avoid_collision(c: &mut Cpu) {
    if crate::custom_tracks::active().is_none() {
        PREV_AVOID_COLLISION.call(c);
        return;
    }
    let (cam, ret, sp0) = (c.g[A0][0] as u32, c.g[RA][0] as u32, c.g[SP][0] as u32);
    PREV_AVOID_COLLISION.call(c);
    if c.pc != ret || c.g[SP][0] as u32 != sp0 || !imported_chase_camera(c, cam) {
        return;
    }
    chase_obstruction(c, cam, sp0);
}

/// Active chase controller on an imported course.
fn imported_chase_camera(c: &mut Cpu, cam: u32) -> bool {
    let chase = c.bus.read32(cam + CAM_CONTROLLER);
    chase != 0 && c.bus.read32(cam + 0xa0) == chase
}

fn read_vec4(c: &mut Cpu, at: u32) -> [f32; 4] {
    std::array::from_fn(|i| f32::from_bits(c.bus.read32(at + i as u32 * 4)))
}

fn write_vec4(c: &mut Cpu, at: u32, v: [f32; 4]) {
    for (i, value) in v.iter().enumerate() {
        c.bus.write32(at + i as u32 * 4, value.to_bits());
    }
}

/// The fork's `avoid_collision_script(.., chase_obstruction = true)`: the
/// segment from 100 above the look-at point to the eye, nearest contact (the
/// reference fraction 0), the eye retracted along the sight line with the
/// native 50 cm margin, then the eye saved at +0x490 and +0x4a0 cleared.
fn chase_obstruction(c: &mut Cpu, cam: u32, sp0: u32) {
    let (registers, floats, pc) = (c.g, c.f, c.pc);
    // The frame the generated function just released (0x400 bytes): its
    // scratch layout (segment at 0x1f0, contact record at 0x2a0, the points
    // at 0x1c0/0x1d0) is reused; the saved registers at 0x370.. are not.
    let frame = sp0.wrapping_sub(0x400);
    let at = |offset: u32| frame.wrapping_add(offset);
    c.g[SP][0] = sx(frame);
    c.cycles += 36;
    let finished = (|| {
        // Only without a script camera (that case is upstream's own path).
        c.g[A0][0] = c.lw(cam + CAM_CONTROLLER);
        if call_ptr(c, IS_SCRIPT_CAMERA_ACTIVE, 0x0015_f2a0).is_err() {
            return false;
        }
        if c.g[V0][0] != 0 {
            return true;
        }
        let up = read_vec4(c, AVOID_UP);
        let look = read_vec4(c, cam + CAM_LOOK_AT);
        let eye = read_vec4(c, cam + CAM_EYE);
        let elevated: [f32; 4] = std::array::from_fn(|i| look[i] + up[i] * 100.0);
        write_vec4(c, at(0x1c0), elevated);
        write_vec4(c, at(0x1d0), eye);
        c.g[A0][0] = sx(at(0x1f0));
        c.g[A1][0] = sx(at(0x1c0));
        c.g[A2][0] = sx(at(0x1d0));
        c.g[A3][0] = 2;
        c.f[12] = 0; // the preferred contact fraction: nearest to the rider
        if call_ptr(c, COLL_SEGMENT_CTOR, 0x0015_f308).is_err() {
            return false;
        }
        let gp = c.g[GP][0] as u32;
        let manager = c.lw(gp.wrapping_sub(0x848)) as u32;
        let game = c.bus.read32(manager + 0x84);
        let world = c.lw(game + 0x20) as u32;
        c.g[A0][0] = sx(world);
        c.g[A1][0] = sx(at(0x1f0));
        c.g[A2][0] = sx(at(0x2a0));
        c.g[A3][0] = 0;
        if call_ptr(c, NASH_FIND_CLOSEST_CONTACT, 0x0015_f324).is_err() {
            return false;
        }
        let fraction = f32::from_bits(c.f[0]);
        if (0. ..=1.).contains(&fraction) {
            let eye = read_vec4(c, cam + CAM_EYE);
            let mut target = read_vec4(c, cam + CAM_LOOK_AT);
            target[2] += 100.; // the same rider-side endpoint as the query
            let delta: [f32; 3] = std::array::from_fn(|i| eye[i] - target[i]);
            let length = delta.iter().map(|v| v * v).sum::<f32>().sqrt();
            if length.is_finite() && length > 0. {
                let t = (fraction - 50. / length).max(0.);
                for axis in 0..3usize {
                    c.bus.write32(
                        cam + CAM_EYE + axis as u32 * 4,
                        (target[axis] + delta[axis] * t).to_bits(),
                    );
                }
            }
        }
        for axis in 0..4u32 {
            let value = c.bus.read32(cam + CAM_EYE + axis * 4);
            c.bus.write32(cam + CAM_SCRIPT_EYE + axis * 4, value);
        }
        c.sw(cam + CAM_SCRIPT_BLOCKED, 0);
        true
    })();
    if finished {
        c.g = registers;
        c.f = floats;
        c.pc = pc;
    }
}

// ---------------------------------------------------------------------------
// intersect/riderspheretree.rs, intersect/worldsphtree.rs

/// `CollSegment_ctor(seg, p0, p1, state, reference)` (0x32e100).
fn coll_segment_ctor(c: &mut Cpu) {
    crate::custom_tricky_recovery::prepare_ground_query(c);
    PREV_COLL_SEGMENT_CTOR.call(c);
}

/// `OverlapList_CollideObjects(list, query, ..)` (0x334458): the optional
/// object-volume trace of the query. (The per-candidate trace around
/// `CollideVolumeObject` is not carried over: it is a diagnostic.)
fn overlap_list_collide_objects(c: &mut Cpu) {
    crate::custom_contact_trace::query(
        c,
        c.g[A0][0] as u32,
        c.g[A1][0] as u32,
        if c.g[T0][0] == 0 { 1 } else { 2 },
    );
    PREV_OVERLAP_COLLIDE.call(c);
}

// ---------------------------------------------------------------------------
// scripter/ssxscriptengine.rs

/// `ScriptEngine_QueueIntroScripts(..)` (0x27aaf8): the template's fly-over,
/// approach and gate cameras are authored for another world; a custom course
/// continues through the ordinary rider-loading and countdown path.
fn queue_intro_scripts(c: &mut Cpu) {
    if crate::custom_tracks::active().is_some() {
        abi::ret(c);
        return;
    }
    PREV_QUEUE_INTRO.call(c);
}

// ---------------------------------------------------------------------------
// capture.rs

/// The return address of `ModObj_Render`'s draw call (graphics manager slot
/// 0x2f8 = `PS2GraphicsMan_DrawModel`); `ModObj_Render` keeps its object in S4.
const MOD_OBJ_DRAW_RETURN: u32 = 0x0035_65d8;

/// `PS2GraphicsMan_DrawModel` (0x37e238), two Tricky edits:
/// - `ModObj_Render`'s draw of a crowd or movie-screen object gets the
///   course's private material table (the fork patched `A2` inside
///   `ModObj_Render`; the draw call's entry sees the same registers);
/// - imported movies use their source alpha equation through the native model
///   path even when the optional packet-capture fallback is selected (the
///   blend itself: [`screen_blend`], installed as the engine's opaque-blend
///   hook).
fn draw_model(c: &mut Cpu) {
    if c.pc == 0x0037_e238 && c.g[RA][0] as u32 == MOD_OBJ_DRAW_RETURN {
        let object = c.g[S4][0] as u32;
        let table = crate::custom_crowds::materials(c, object)
            .or_else(|| crate::custom_movies::materials(c, object));
        if let Some(table) = table {
            c.g[A2][0] = table as i32 as i64 as u64;
        }
    }
    if crate::custom_movies::is_screen(c, c.g[A1][0] as u32, c.g[A2][0] as u32) {
        ssx3_game::packetless::draw_model(c);
        return;
    }
    PREV_DRAW_MODEL.call(c);
}

/// The engine's opaque-blend hook (`packetless::set_opaque_blend_hook`): a
/// movie screen keeps the source alpha equation `(Cs - Cd) * As + Cd`
/// (blend preset 5) instead of the opaque preset.
pub fn screen_blend(c: &mut Cpu, instance: u32, table: u32) -> Option<(u32, u32)> {
    crate::custom_movies::is_screen(c, instance, table).then_some((5 << 2, 5 << 7))
}
