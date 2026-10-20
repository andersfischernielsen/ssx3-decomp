//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for DMA waiting, script hooks, rider mass/leg-IK/danger
//! state, audio trick scoring, the world-painter irradiance names and the
//! rider-base state dispatch tables, each with a differential test against
//! the generated body.
//!
//! The three `RiderBase_*` dispatchers read their jump table from `.rodata`
//! at run time (`0x450000 + 0x6b50/0x6b70/0x6c10`), exactly as the generated
//! code does; a table entry that leaves the function goes through
//! `c.jump_out` just like the original's indirect jump. `cBoardTrailFX_IsActive`
//! is a dispatcher too (the resume entries are the blocks of its state
//! `switch`), so it is written as `match c.pc` over the generated blocks.
//!
//! Tests reach the handler calls of the `RiderBase_*` tables only through
//! table entries that stay inside the function (the real handler entries
//! would run whole state updates on a dummy rider); the handler call blocks
//! themselves are transcribed from the generated code but not exercised.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_f7b0, dma_wait_to_scratchpad, "Dma_WaitToScratchpad"),
    (0x002d_1ac8, wscript_man_run_obj_script5_hook, "WScriptMan_RunObjScript5Hook"),
    (0x0034_eba0, anim_object_fire_end_script, "AnimObject_FireEndScript"),
    (0x0011_ff98, rider_get_mass, "cRider_getMass"),
    (0x0012_0d90, rider_update_leg_ik_blend, "func_00120D90"),
    (0x0012_2cf0, rider_update_danger_state, "func_00122CF0"),
    (0x0029_0f58, ssx_audio_system_set_player_trick_score, "SSXAudioSystem_SetPlayerTrickScore"),
    (0x002e_f0e8, world_painter_get_irradiance_name0, "func_002EF0E8"),
    (0x002e_f140, world_painter_get_irradiance_name1, "func_002EF140"),
    (0x0011_1408, rider_base_update_main_state, "func_00111408"),
    (0x0011_14a0, rider_base_post_update_main_state, "func_001114A0"),
    (0x0011_1728, rider_base_handle_sub_state_input, "func_00111728"),
    (0x002e_87e8, c_board_trail_fx_is_active, "cBoardTrailFX_IsActive"),
    (0x002e_f6d0, c_air_streamer_fx_update, "cAirStreamerFX_Update"),
];

// ---------------------------------------------------------------------------
// Address and register helpers.

/// `a + off` (32-bit).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `sll` on a register value (`sx` of the 32-bit result, as the generator
/// writes it).
#[inline(always)]
fn shl32(v: u64, n: u32) -> u64 {
    sx((v as u32) << n)
}

/// 32-bit wrapping add of two register values.
#[inline(always)]
fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
fn li_f(c: &mut Cpu, n: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(1, n);
}

// ---------------------------------------------------------------------------
// DMA.

/// `void Dma_WaitToScratchpad(void)` (0x38f7b0): spin on DMA channel 4's
/// `CHCR` until the transfer to the scratchpad (`0x100`) has completed.
pub fn dma_wait_to_scratchpad(c: &mut Cpu) {
    /// DMA channel 4 control register (the original polls `0x1000d400`).
    const D_CHCR4: u32 = 0x1000_d400;
    /// `CHCR.STR`-side "transfer running" bit the original waits on.
    const BUSY: u64 = 0x100;

    c.cycles += 6;
    c.g[V0][0] = 0x1000_0000;
    c.g[V0][0] |= 0xd400;
    c.g[V1][0] = Addr(D_CHCR4).lw(c);
    c.g[V1][0] &= BUSY;
    let done = c.g[V1][0] == 0;
    c.g[V1][0] = 0x1000_0000;
    if !done {
        c.cycles += 2;
        c.g[V1][0] |= 0xd400;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(D_CHCR4).lw(c);
            c.g[V0][0] &= BUSY;
            if c.g[V0][0] == 0 {
                break;
            }
            if c.cycles >= c.next_event {
                crate::events::deliver_event_here(c, 0x0038_f7d0);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Script hooks.

/// `int WScriptMan_RunObjScript5Hook(int id)` (0x2d1ac8): run object script 5
/// on the current object script manager (`gp+0xce8`).
pub fn wscript_man_run_obj_script5_hook(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0030_a688, 0x002d_1ae0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int AnimObject_FireEndScript(sAnimObject* obj)` (0x34eba0): fire the
/// animation end script of the link at `obj+0x14`, then report whether the
/// link's list head is the link itself (i.e. `list->head == link`).
pub fn anim_object_fire_end_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = abi::arg(c, 0);
        c.g[S0][0] = sx(obj);
        c.g[A0][0] = Addr(p(obj, 0x2c)).lw(c);
        frame.save_ra(c, 0);
        let link = p(obj, 0x14);
        c.g[S0][0] = sx(link);
        call_ptr(c, 0x002d_1ac8, 0x0034_ebbc)?;
        c.g[V1][0] = Addr(p(link, 0x18)).lw(c);
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
        c.g[V0][0] ^= c.g[S0][0];
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider mass and leg IK.

/// `float cRider_getMass(cRider* r)` (0x11ff98): the rider's weight from the
/// character database, scaled by the collision-toughness factor
/// (`1.5 * attrib + 1`) and by `(10 * r->massScale + 1)`.
pub fn rider_get_mass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let rider = abi::arg(c, 0);
        c.g[S0][0] = sx(rider);
        frame.save_ra(c, 0);
        // weight = getWeight(getInterface(getBE(), 2), r->aiIndex)
        call_ptr(c, 0x0014_dc80, 0x0011_ffb0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0014_dd58, 0x0011_ffbc)?;
        c.g[A1][0] = Addr(p(rider, 0x86c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_ef30, 0x0011_ffc8)?;
        c.g[S1][0] = c.g[V0][0];
        // toughness = getCollisionAttrib(getInterface(getBE(), 3), index, r->b34)
        call_ptr(c, 0x0014_dc80, 0x0011_ffd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0014_dd58, 0x0011_ffdc)?;
        c.g[A1][0] = Addr(p(rider, 0x86c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = Addr(p(rider, 0xb34)).lw(c);
        call_ptr(c, 0x0014_8f50, 0x0011_ffec)?;
        // f0 = weight * (toughness * 1.5003352 + 1) * (massScale * 10 + 1)
        c.lwc1(1, gp(c, -0x7964));
        c.mtc1(17, 3);
        c.cvt_s_w(3, 3);
        c.lwc1(2, p(rider, 0x2fc));
        c.mul_s(0, 0, 1);
        li_f(c, 4, 0x3f80_0000);
        li_f(c, 1, 0x4120_0000);
        frame.restore(c, S0, 0x20);
        c.add_s(0, 0, 4);
        frame.restore(c, S1, 0x10);
        c.mul_s(2, 2, 1);
        frame.restore_ra(c, 0);
        c.mul_s(3, 3, 0);
        c.add_s(2, 2, 4);
        c.mul_s(0, 3, 2);
        frame.close(c);
        Ok(())
    })
}

/// `void func_00120D90(cRider* r)` (0x120d90): approach the leg-IK blend
/// `r->legIKBlend` (+0x318) towards 0 for the spin/air animation ids 20..26
/// and towards 1 otherwise, at 0.05 per frame, never overshooting.
pub fn rider_update_leg_ik_blend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let rider = abi::arg(c, 0);
        c.g[S0][0] = sx(rider);
        c.g[A0][0] = Addr(p(rider, 0x784)).lw(c);
        call_ptr(c, 0x0031_1ae8, 0x0012_0dac)?;
        let anim = (r32(c, V0).wrapping_sub(0x14)) < 7;
        c.g[V0][0] = anim as u64;
        let in_air_anim = c.g[V0][0] != 0;
        c.lwc1(2, p(rider, 0x318));
        if in_air_anim {
            c.cycles += 5;
            c.lwc1(3, gp(c, -0x7934));
            c.c_lt_s(3, 2);
            if c.fcc() {
                // blend = current - step, straight out (no clamp).
                c.sub_s(1, 2, 3);
                leg_ik_blend_exit(c, frame);
                return Ok(());
            }
            c.cycles += 4;
            c.lwc1(0, gp(c, -0x7930));
            c.mtc1(0, 1);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            leg_ik_blend_clamp(c, frame);
            return Ok(());
        }
        // Not an air animation: approach 1.
        c.cycles += 5;
        c.lwc1(0, gp(c, -0x792c));
        c.lwc1(3, gp(c, -0x7928));
        c.c_lt_s(0, 2);
        if !c.fcc() {
            c.cycles += 3;
            c.lwc1(0, gp(c, -0x7924));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 1);
            leg_ik_blend_clamp(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        c.sub_s(1, 2, 3);
        leg_ik_blend_exit(c, frame);
        Ok(())
    })
}

/// The clamp of [`rider_update_leg_ik_blend`] (0x120e08): never overshoot.
fn leg_ik_blend_clamp(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    c.c_lt_s(2, 0);
    if c.fcc() {
        c.add_s(1, 2, 3);
    }
    leg_ik_blend_exit(c, frame);
}

/// The epilogue of [`rider_update_leg_ik_blend`] (0x120e18): store the blend.
fn leg_ik_blend_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    c.swc1(1, p(r32(c, S0), 0x318));
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Rider danger state.

/// `void func_00122CF0(cRider* r)` (0x122cf0): refresh `r->dangerState`
/// (+0x5b0) from the danger manager, falling back to "on the ground and
/// spinning fast" when the manager has no danger, and tell the metrics when
/// the state changed.
pub fn rider_update_danger_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        let rider = abi::arg(c, 0);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(rider);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        c.g[A1][0] = Addr(p(rider, 0x86c)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
        c.g[S1][0] = Addr(p(rider, 0x5b0)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0xa8)).lw(c);
        call_ptr(c, 0x0010_1a10, 0x0012_2d20)?;
        let danger = c.g[V0][0] != 0;
        c.sw(p(rider, 0x5b0), c.g[V0][0]);
        if danger {
            c.cycles += 1;
            c.g[A1][0] = Addr(p(rider, 0x5b0)).lw(c);
            return rider_danger_changed(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = sx(rider);
        call_ptr(c, 0x0012_2cd0, 0x0012_2d30)?;
        if c.g[V0][0] == 0 {
            c.g[A1][0] = Addr(p(rider, 0x5b0)).lw(c);
            return rider_danger_changed(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = sx(rider);
        call_ptr(c, 0x0012_2c98, 0x0012_2d40)?;
        let not_spinning = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !not_spinning {
            c.cycles += 1;
            c.sw(p(rider, 0x5b0), c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[A1][0] = Addr(p(rider, 0x5b0)).lw(c);
        rider_danger_changed(c, frame)
    })
}

/// The state-change tail of [`rider_update_danger_state`] (0x122d50).
fn rider_danger_changed(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    if c.g[S1][0] != c.g[A1][0] {
        c.cycles += 3;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x790)).lw(c);
        call_ptr(c, 0x0011_a0c0, 0x0012_2d60)?;
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Audio scoring.

/// `void SSXAudioSystem_SetPlayerTrickScore(SSXAudioSystem* app, int rider,
/// float score)` (0x290f58): store the trick score on the rider's listener
/// (rider 0 at `+0x6074`, rider 1 at `+0x6078`) when it is the active player.
pub fn ssx_audio_system_set_player_trick_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let app = abi::arg(c, 0);
        let rider = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(app);
        c.g[S1][0] = sx(rider);
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_5d98, 0x0029_0f80)?;
        if c.g[V0][0] == c.g[S1][0] {
            c.cycles += 2;
            c.swc1(20, p(app, 0x6074));
            audio_score_tail(c, frame);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_8ce8, 0x0029_0f98)?;
        c.g[V1][0] = 2;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x20);
            audio_score_exit(c, frame);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0028_5d98, 0x0029_0fb0)?;
        if c.g[V0][0] == c.g[S1][0] {
            c.swc1(20, p(app, 0x6078));
        }
        audio_score_tail(c, frame);
        Ok(())
    })
}

/// 0x290fb8: the `+1` cycle tail and `s0` pop.
fn audio_score_tail(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x20);
    audio_score_exit(c, frame);
}

/// 0x290fbc: the epilogue (s1, ra and f20 back, `sp += 0x40`).
fn audio_score_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// World-painter irradiance names.

/// `const char* func_002EF0E8(void)` (0x2ef0e8): the name of irradiance slot
/// 0 — the built-in global name once the world painter exists, else the
/// first object's render-state name.
pub fn world_painter_get_irradiance_name0(c: &mut Cpu) {
    world_painter_irradiance_name(c, 0x002e_f128, 0x128, 0x12c);
}

/// `const char* func_002EF140(void)` (0x2ef140): the name of irradiance slot
/// 1 (see [`world_painter_get_irradiance_name0`]).
pub fn world_painter_get_irradiance_name1(c: &mut Cpu) {
    world_painter_irradiance_name(c, 0x002e_f180, 0x130, 0x134);
}

/// The shared body of the two name getters: `gp+0xa74` set means "return the
/// global name", else `vt[slot]` of the object in `D_004FA370`.
fn world_painter_irradiance_name(c: &mut Cpu, ret: u32, delta_off: u32, fn_off: u32) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp(c, 0xa74)).lw(c);
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = sx(gp(c, 0x24c0));
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 14;
            c.g[V1][0] = 0xf0;
            c.g[V0][0] = 0x50_0000;
            c.mult(0, 3, 4, 3);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x5c90));
            c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
            c.g[A1][0] = Addr(r32(c, A0)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A1), 4)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), delta_off)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), fn_off)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
            call_ptr(c, target, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider base main/sub-state dispatch (`switch` over `.rodata` jump tables).

/// One table entry of the `RiderBase_*MainState` / `..._HandleSubStateInput`
/// dispatchers: the handler a state calls, the sub-object it receives and the
/// block it falls into afterwards.
#[derive(Clone, Copy)]
struct StateCase {
    /// The state handler (a `Rider<State>_*` function).
    handler: u32,
    /// Where the handler returns.
    ret: u32,
    /// Offset from the rider base to the handler's sub-object (0: the rider).
    off: u32,
    /// Cycles charged before the call.
    cycles: u64,
    /// True for the case that falls into the `+1`-cycle tail instead of
    /// straight into the epilogue.
    tail: bool,
}

/// `RiderBase_UpdateMainState` (0x111408): the update handler per main state.
const UPDATE_CASES: [StateCase; 6] = [
    StateCase { handler: 0x0013_d818, ret: 0x0011_1444, off: 0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_9a20, ret: 0x0011_1454, off: 0x20, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6e98, ret: 0x0011_1464, off: 0x30, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6958, ret: 0x0011_1474, off: 0xa0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_af28, ret: 0x0011_1484, off: 0xb0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_91a8, ret: 0x0011_1494, off: 0x110, cycles: 2, tail: true },
];

/// `RiderBase_PostUpdateMainState` (0x1114a0): the post-update handler per
/// main state.
const POST_CASES: [StateCase; 6] = [
    StateCase { handler: 0x0013_f178, ret: 0x0011_14dc, off: 0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_9c88, ret: 0x0011_14ec, off: 0x20, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6ee0, ret: 0x0011_14fc, off: 0x30, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6978, ret: 0x0011_150c, off: 0xa0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_bfa8, ret: 0x0011_151c, off: 0xb0, cycles: 4, tail: false },
    StateCase { handler: 0x0013_9528, ret: 0x0011_152c, off: 0x110, cycles: 2, tail: true },
];

/// `RiderBase_HandleSubStateInput` (0x111728): the input handler per sub
/// state.
const SUB_CASES: [StateCase; 13] = [
    StateCase { handler: 0x0013_1620, ret: 0x0011_1764, off: 0x1c0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_fc80, ret: 0x0011_1774, off: 0x1d0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_e9b8, ret: 0x0011_1784, off: 0x1f0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_e778, ret: 0x0011_1794, off: 0x200, cycles: 4, tail: false },
    StateCase { handler: 0x0012_f730, ret: 0x0011_17a4, off: 0x210, cycles: 4, tail: false },
    StateCase { handler: 0x0013_3308, ret: 0x0011_17b4, off: 0x230, cycles: 4, tail: false },
    StateCase { handler: 0x0012_bf68, ret: 0x0011_17c4, off: 0x290, cycles: 4, tail: false },
    StateCase { handler: 0x0013_1d30, ret: 0x0011_17d4, off: 0x2b0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_cb68, ret: 0x0011_17e4, off: 0x2c0, cycles: 4, tail: false },
    StateCase { handler: 0x0012_f398, ret: 0x0011_17f4, off: 0x350, cycles: 4, tail: false },
    StateCase { handler: 0x0012_c678, ret: 0x0011_1804, off: 0x360, cycles: 4, tail: false },
    StateCase { handler: 0x0013_2a30, ret: 0x0011_1814, off: 0x370, cycles: 4, tail: false },
    StateCase { handler: 0x0013_6508, ret: 0x0011_1824, off: 0x390, cycles: 2, tail: true },
];

/// One of the state handler calls: `handler(this + off)`.
fn state_case_call(c: &mut Cpu, frame: Frame, case: StateCase) -> Flow {
    c.cycles += case.cycles;
    if case.off == 0 {
        c.g[A0][0] = c.g[A1][0];
    } else {
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(case.off));
    }
    call_ptr(c, case.handler, case.ret)?;
    if case.tail {
        c.cycles += 1;
    }
    frame.restore_ra(c, 0);
    state_case_exit(c, frame);
    Ok(())
}

/// `sp += 0x10; jr ra` (0x111498 / 0x111530 / 0x111828).
fn state_case_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 2;
    frame.close(c);
}

/// `RiderBase_UpdateMainState` (0x111408): run the main-state update handler
/// of `rider->mMainState` (0xde0); states past the table do nothing.
pub fn rider_base_update_main_state(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        update_main_state_at(c, frame, c.pc)
    })
}

fn update_main_state_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        0x0011_1408 => {
            c.cycles += 7;
            Frame::open(c, 0x10);
            c.g[A1][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V1][0] = Addr(p(r32(c, A1), 0xde0)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 6) as u64;
            let in_table = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0045_0000;
            if in_table {
                update_main_state_table(c, frame)
            } else {
                // Not in the table: the `+1` tail and the epilogue.
                c.cycles += 1;
                frame.restore_ra(c, 0);
                state_case_exit(c, frame);
                Ok(())
            }
        }
        0x0011_1424 => update_main_state_table(c, frame),
        0x0011_143c => state_case_call(c, frame, UPDATE_CASES[0]),
        0x0011_144c => state_case_call(c, frame, UPDATE_CASES[1]),
        0x0011_145c => state_case_call(c, frame, UPDATE_CASES[2]),
        0x0011_146c => state_case_call(c, frame, UPDATE_CASES[3]),
        0x0011_147c => state_case_call(c, frame, UPDATE_CASES[4]),
        0x0011_148c => state_case_call(c, frame, UPDATE_CASES[5]),
        0x0011_1494 => {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            state_case_exit(c, frame);
            Ok(())
        }
        0x0011_1498 => {
            state_case_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, 0x0011_1408, 0x0011_14a0);
            Ok(())
        }
    }
}

/// The jump-table read at 0x111424 (`0x450000 + 0x6b50 + state * 4`).
fn update_main_state_table(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = shl32(c.g[V1][0], 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6b50));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match r32(c, A0) {
        0x0011_143c => state_case_call(c, frame, UPDATE_CASES[0]),
        0x0011_144c => state_case_call(c, frame, UPDATE_CASES[1]),
        0x0011_145c => state_case_call(c, frame, UPDATE_CASES[2]),
        0x0011_146c => state_case_call(c, frame, UPDATE_CASES[3]),
        0x0011_147c => state_case_call(c, frame, UPDATE_CASES[4]),
        0x0011_148c => state_case_call(c, frame, UPDATE_CASES[5]),
        t => {
            c.jump_out(t, 0x0011_1408, 0x0011_14a0);
            Ok(())
        }
    }
}

/// `RiderBase_PostUpdateMainState` (0x1114a0): run the main-state
/// post-update handler of `rider->mMainState` (0xde0).
pub fn rider_base_post_update_main_state(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        post_main_state_at(c, frame, c.pc)
    })
}

fn post_main_state_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        0x0011_14a0 => {
            c.cycles += 7;
            Frame::open(c, 0x10);
            c.g[A1][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V1][0] = Addr(p(r32(c, A1), 0xde0)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 6) as u64;
            let in_table = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0045_0000;
            if in_table {
                post_main_state_table(c, frame)
            } else {
                c.cycles += 1;
                frame.restore_ra(c, 0);
                state_case_exit(c, frame);
                Ok(())
            }
        }
        0x0011_14bc => post_main_state_table(c, frame),
        0x0011_14d4 => state_case_call(c, frame, POST_CASES[0]),
        0x0011_14e4 => state_case_call(c, frame, POST_CASES[1]),
        0x0011_14f4 => state_case_call(c, frame, POST_CASES[2]),
        0x0011_1504 => state_case_call(c, frame, POST_CASES[3]),
        0x0011_1514 => state_case_call(c, frame, POST_CASES[4]),
        0x0011_1524 => state_case_call(c, frame, POST_CASES[5]),
        0x0011_152c => {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            state_case_exit(c, frame);
            Ok(())
        }
        0x0011_1530 => {
            state_case_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, 0x0011_14a0, 0x0011_1538);
            Ok(())
        }
    }
}

/// The jump-table read at 0x1114bc (`0x450000 + 0x6b70 + state * 4`).
fn post_main_state_table(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = shl32(c.g[V1][0], 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6b70));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match r32(c, A0) {
        0x0011_14d4 => state_case_call(c, frame, POST_CASES[0]),
        0x0011_14e4 => state_case_call(c, frame, POST_CASES[1]),
        0x0011_14f4 => state_case_call(c, frame, POST_CASES[2]),
        0x0011_1504 => state_case_call(c, frame, POST_CASES[3]),
        0x0011_1514 => state_case_call(c, frame, POST_CASES[4]),
        0x0011_1524 => state_case_call(c, frame, POST_CASES[5]),
        t => {
            c.jump_out(t, 0x0011_14a0, 0x0011_1538);
            Ok(())
        }
    }
}

/// `RiderBase_HandleSubStateInput` (0x111728): hand the input to the
/// sub-state handler of `rider->mSubState` (0xde4).
pub fn rider_base_handle_sub_state_input(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x10 };
        sub_state_input_at(c, frame, c.pc)
    })
}

fn sub_state_input_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        0x0011_1728 => {
            c.cycles += 7;
            Frame::open(c, 0x10);
            c.g[A2][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V1][0] = Addr(p(r32(c, A2), 0xde4)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 14) as u64;
            let in_table = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0045_0000;
            if in_table {
                sub_state_input_table(c, frame)
            } else {
                c.cycles += 1;
                frame.restore_ra(c, 0);
                state_case_exit(c, frame);
                Ok(())
            }
        }
        0x0011_1744 => sub_state_input_table(c, frame),
        0x0011_175c => sub_case_call(c, frame, 0),
        0x0011_176c => sub_case_call(c, frame, 1),
        0x0011_177c => sub_case_call(c, frame, 2),
        0x0011_178c => sub_case_call(c, frame, 3),
        0x0011_179c => sub_case_call(c, frame, 4),
        0x0011_17ac => sub_case_call(c, frame, 5),
        0x0011_17bc => sub_case_call(c, frame, 6),
        0x0011_17cc => sub_case_call(c, frame, 7),
        0x0011_17dc => sub_case_call(c, frame, 8),
        0x0011_17ec => sub_case_call(c, frame, 9),
        0x0011_17fc => sub_case_call(c, frame, 10),
        0x0011_180c => sub_case_call(c, frame, 11),
        0x0011_181c => sub_case_call(c, frame, 12),
        0x0011_1824 => {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            state_case_exit(c, frame);
            Ok(())
        }
        0x0011_1828 => {
            state_case_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, 0x0011_1728, 0x0011_1830);
            Ok(())
        }
    }
}

/// The sub-state input call; the sub-object is at `rider + off` and `a1`
/// (the pad state) is passed through.
fn sub_case_call(c: &mut Cpu, frame: Frame, i: usize) -> Flow {
    let case = SUB_CASES[i];
    c.cycles += case.cycles;
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(case.off));
    call_ptr(c, case.handler, case.ret)?;
    if case.tail {
        c.cycles += 1;
    }
    frame.restore_ra(c, 0);
    state_case_exit(c, frame);
    Ok(())
}

/// The jump-table read at 0x111744 (`0x450000 + 0x6c10 + state * 4`).
fn sub_state_input_table(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = shl32(c.g[V1][0], 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6c10));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match r32(c, A0) {
        0x0011_175c => sub_case_call(c, frame, 0),
        0x0011_176c => sub_case_call(c, frame, 1),
        0x0011_177c => sub_case_call(c, frame, 2),
        0x0011_178c => sub_case_call(c, frame, 3),
        0x0011_179c => sub_case_call(c, frame, 4),
        0x0011_17ac => sub_case_call(c, frame, 5),
        0x0011_17bc => sub_case_call(c, frame, 6),
        0x0011_17cc => sub_case_call(c, frame, 7),
        0x0011_17dc => sub_case_call(c, frame, 8),
        0x0011_17ec => sub_case_call(c, frame, 9),
        0x0011_17fc => sub_case_call(c, frame, 10),
        0x0011_180c => sub_case_call(c, frame, 11),
        0x0011_181c => sub_case_call(c, frame, 12),
        // The table's 14th slot (sub states 13..) points at the function's
        // `+1`-cycle tail block, which the original's `switch` accepts too.
        0x0011_1824 => {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            state_case_exit(c, frame);
            Ok(())
        }
        t => {
            c.jump_out(t, 0x0011_1728, 0x0011_1830);
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Board-trail wake effect.

/// `cBoardTrailFX` (a wake tail attached to a rider) field offsets.
mod board_trail {
    /// The rider (entity) the effect follows (`*(self)`).
    pub const OWNER: u32 = 0;
    /// The rider's animation controller.
    pub const ANIM: u32 = 0x784;
    /// The rider's state object.
    pub const STATE: u32 = 0x77c;
    /// The board-trail intensity the `switch` writes.
    pub const INTENSITY: u32 = 0x9c;
    /// Rider main state.
    pub const MAIN_STATE: u32 = 0x438;
}

/// The blocks of `cBoardTrailFX_IsActive`'s state `switch` (its `.rodata`
/// jump table sits at `0x480000 + 0x7b40`).
const CBFX_ENTRY: u32 = 0x002e_87e8;
const CBFX_END: u32 = 0x002e_8934;
const CBFX_TABLE_OFF: u32 = 0x7b40;
const CBFX_GET_ANIM_ID: u32 = 0x0031_2aa0;
const CBFX_GET_SEQUENCE: u32 = 0x0031_1b20;
const CBFX_TEST_FLAGS: u32 = 0x0014_46a0;

/// `bool cBoardTrailFX_IsActive(cBoardTrailFX* self)` (0x2e87e8): whether the
/// wake tail should draw this frame — the rider must be in a drawable state
/// (or hidden behind the state's flag), playing the board-trail animation
/// sequence, and the function stores the per-state trail intensity into
/// `self+0x9c` when it is active.
pub fn c_board_trail_fx_is_active(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        board_trail_fx_at(c, frame, c.pc)
    })
}

fn board_trail_fx_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        CBFX_ENTRY => cbfx_entry(c, frame),
        0x002e_880c => cbfx_state_two(c, frame),
        0x002e_8820 => {
            cbfx_pop(c, frame);
            Ok(())
        }
        0x002e_8828 => cbfx_ask_active(c, frame),
        0x002e_8838 => cbfx_can_draw(c, frame),
        0x002e_8848 => cbfx_can_draw_sub(c, frame),
        0x002e_8854 => cbfx_can_draw_flag(c, frame),
        0x002e_885c => cbfx_a1_known(c, frame),
        0x002e_8864 => cbfx_b00_check(c, frame),
        0x002e_8870 => cbfx_anim_check(c, frame),
        0x002e_888c => cbfx_sequence(c, frame),
        0x002e_88ac => cbfx_sequence_second(c, frame),
        0x002e_88cc => {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            cbfx_switch(c, frame)
        }
        0x002e_88d0 => cbfx_switch(c, frame),
        0x002e_88e0 => cbfx_table(c, frame),
        0x002e_88f8 => cbfx_case_f8(c, frame),
        0x002e_8900 => cbfx_case_00(c, frame),
        0x002e_8908 => cbfx_case_08(c, frame),
        0x002e_8910 => cbfx_case_10(c, frame),
        0x002e_8914 => {
            cbfx_store_active(c, frame);
            Ok(())
        }
        0x002e_8920 => {
            cbfx_return_false(c, frame);
            Ok(())
        }
        0x002e_8924 => {
            cbfx_return(c, frame);
            Ok(())
        }
        0x002e_8928 => {
            cbfx_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, CBFX_ENTRY, CBFX_END);
            Ok(())
        }
    }
}

/// 0x2e87e8: the entry.
fn cbfx_entry(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let fx = abi::arg(c, 0);
    c.g[S0][0] = sx(fx);
    c.g[A0][0] = Addr(p(fx, board_trail::OWNER)).lw(c);
    call_ptr(c, 0x0011_fe98, 0x002e_8800)?;
    c.g[V1][0] = 2;
    if c.g[V0][0] != c.g[V1][0] {
        cbfx_ask_active(c, frame)
    } else {
        cbfx_state_two(c, frame)
    }
}

/// 0x2e880c: state 2: inactive when the state's hidden flag is set.
fn cbfx_state_two(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, V0), board_trail::STATE)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V1), 0x30)).lw(c);
    let hidden = c.g[A0][0] != 0;
    c.g[V0][0] = 0;
    if hidden {
        cbfx_pop(c, frame);
        return Ok(());
    }
    cbfx_anim_check(c, frame)
}

/// 0x2e8820: pop `s0` and return with the caller's `v0` (0).
fn cbfx_pop(c: &mut Cpu, frame: Frame) {
    c.cycles += 2;
    frame.restore(c, S0, 0x10);
    cbfx_exit(c, frame);
}

/// 0x2e8828: the rider is in some other state: state 0 draws, others need
/// the drawable check.
fn cbfx_ask_active(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
    call_ptr(c, 0x0011_fe98, 0x002e_8830)?;
    let first_state = c.g[V0][0] == 0;
    c.g[A1][0] = 0;
    if first_state {
        cbfx_anim_check(c, frame)
    } else {
        cbfx_can_draw(c, frame)
    }
}

/// 0x2e8838: `boardTrailCanDraw` inlined (the rider's spray link).
fn cbfx_can_draw(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
    c.g[V0][0] = Addr(p(r32(c, V1), 0xac4)).lw(c);
    let link = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[V1][0];
    if link {
        cbfx_a1_known(c, frame)
    } else {
        cbfx_can_draw_sub(c, frame)
    }
}

/// 0x2e8848.
fn cbfx_can_draw_sub(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(p(r32(c, A0), 0xad0)).lw(c);
    if c.g[V0][0] != 0 {
        cbfx_a1_known(c, frame)
    } else {
        cbfx_can_draw_flag(c, frame)
    }
}

/// 0x2e8854.
fn cbfx_can_draw_flag(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.g[V0][0] = Addr(p(r32(c, A0), 0xafc)).lw(c);
    c.g[A1][0] = (0 < c.g[V0][0]) as u64;
    cbfx_a1_known(c, frame)
}

/// 0x2e885c: `a1` says whether the rider is in a spray-tracked state.
fn cbfx_a1_known(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let not_tracked = c.g[A1][0] == 0;
    c.g[V0][0] = 0;
    if not_tracked {
        cbfx_return(c, frame);
        return Ok(());
    }
    cbfx_b00_check(c, frame)
}

/// 0x2e8864.
fn cbfx_b00_check(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(p(r32(c, A0), 0xb00)).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if empty {
        cbfx_return(c, frame);
        return Ok(());
    }
    cbfx_anim_check(c, frame)
}

/// 0x2e8870: is the rider playing the board-trail animation (0x16)?
fn cbfx_anim_check(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = 2;
    c.g[A0][0] = Addr(p(r32(c, V0), board_trail::ANIM)).lw(c);
    call_ptr(c, CBFX_GET_ANIM_ID, 0x002e_8880)?;
    c.g[V1][0] = 0x16;
    let other_anim = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    if other_anim {
        cbfx_switch(c, frame)
    } else {
        cbfx_sequence(c, frame)
    }
}

/// 0x2e888c: the trail sequence's first flag.
fn cbfx_sequence(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    c.g[A1][0] = 2;
    c.g[A0][0] = Addr(p(r32(c, V0), board_trail::ANIM)).lw(c);
    call_ptr(c, CBFX_GET_SEQUENCE, 0x002e_8898)?;
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xb0));
    c.g[A1][0] = 0;
    call_ptr(c, CBFX_TEST_FLAGS, 0x002e_88a4)?;
    let unset = c.g[V0][0] == 0;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    if unset {
        cbfx_switch(c, frame)
    } else {
        cbfx_sequence_second(c, frame)
    }
}

/// 0x2e88ac: the trail sequence's second flag.
fn cbfx_sequence_second(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    c.g[A1][0] = 2;
    c.g[A0][0] = Addr(p(r32(c, V0), board_trail::ANIM)).lw(c);
    call_ptr(c, CBFX_GET_SEQUENCE, 0x002e_88b8)?;
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(0xb0));
    c.g[A1][0] = 1;
    call_ptr(c, CBFX_TEST_FLAGS, 0x002e_88c4)?;
    let unset = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if unset {
        cbfx_return(c, frame);
        return Ok(());
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    cbfx_switch(c, frame)
}

/// 0x2e88d0: the per-state trail intensity `switch` (state < 14).
fn cbfx_switch(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = Addr(p(r32(c, V0), board_trail::MAIN_STATE)).lw(c);
    c.g[V1][0] = (c.g[A0][0] < 0xe) as u64;
    let in_table = c.g[V1][0] != 0;
    c.g[V0][0] = 0x0048_0000;
    if !in_table {
        cbfx_return_false(c, frame);
        return Ok(());
    }
    cbfx_table(c, frame)
}

/// 0x2e88e0: the jump-table read; only the switch's own case blocks are
/// accepted (any other target is an indirect jump out of the function).
fn cbfx_table(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = shl32(c.g[A0][0], 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(CBFX_TABLE_OFF));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    match r32(c, A0) {
        0x002e_88f8 => cbfx_case_f8(c, frame),
        0x002e_8900 => cbfx_case_00(c, frame),
        0x002e_8908 => cbfx_case_08(c, frame),
        0x002e_8910 => cbfx_case_10(c, frame),
        0x002e_8920 => {
            cbfx_return_false(c, frame);
            Ok(())
        }
        t => {
            c.jump_out(t, CBFX_ENTRY, CBFX_END);
            Ok(())
        }
    }
}

/// 0x2e88f8.
fn cbfx_case_f8(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.lwc1(0, gp(c, 0x14a4));
    cbfx_store_active(c, frame);
    Ok(())
}

/// 0x2e8900.
fn cbfx_case_00(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.lwc1(0, gp(c, 0x14a0));
    cbfx_store_active(c, frame);
    Ok(())
}

/// 0x2e8908.
fn cbfx_case_08(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.lwc1(0, gp(c, 0x14a8));
    cbfx_store_active(c, frame);
    Ok(())
}

/// 0x2e8910.
fn cbfx_case_10(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.lwc1(0, gp(c, 0x14ac));
    cbfx_store_active(c, frame);
    Ok(())
}

/// 0x2e8914: store the intensity and report active.
fn cbfx_store_active(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    c.g[V0][0] = 1;
    c.swc1(0, p(r32(c, S0), board_trail::INTENSITY));
    cbfx_return(c, frame);
}

/// 0x2e8920: report inactive.
fn cbfx_return_false(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    c.g[V0][0] = 0;
    cbfx_return(c, frame);
}

/// 0x2e8924: pop `s0`.
fn cbfx_return(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x10);
    cbfx_exit(c, frame);
}

/// 0x2e8928: the epilogue.
fn cbfx_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, put_words, scramble, Rng, DATA, GAME_GP};
    use ssx3_ee as ee;

    /// Point the world chain at dummies: `gp-0x848 -> +0x84` (race manager)
    /// `-> +0xc` (its object). Returns the world address.
    fn put_world(c: &mut Cpu, world: u32, man: u32, ctx: u32) {
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, man);
        c.bus.write32(man + 0xc, ctx);
    }

    #[test]
    fn dma_wait_to_scratchpad_matches() {
        check(ee::s12::f_0038f7b0__generated, dma_wait_to_scratchpad, 64, &|c, seed| {
            let mut r = Rng(0x38f7_b0 ^ (seed as u64) << 9 | 1);
            scramble(c, &mut r);
            // DMA channel 4 is idle (CHCR bit 8 clear) on a fresh machine; a
            // busy channel spins until the DMAC clears it, which no test
            // machine does, so only the idle path is exercised.
        });
    }

    #[test]
    fn wscript_man_run_obj_script5_hook_matches() {
        check(ee::s09::f_002d1ac8__generated, wscript_man_run_obj_script5_hook, 200, &|c, seed| {
            machine(c, seed, 0x2d1ac8, 2);
            let man = DATA + 0x2000;
            let id = DATA + 0x1000;
            c.bus.write32(GAME_GP + 0xce8, man);
            c.bus.write32(man + 0x28c, 0);
            c.bus.write32(id + 0x78, 0);
            c.bus.write32(id + 0x88, 0);
            c.g[A0][0] = sx(id);
        });
    }

    #[test]
    fn anim_object_fire_end_script_matches() {
        check(ee::s11::f_0034eba0__generated, anim_object_fire_end_script, 200, &|c, seed| {
            machine(c, seed, 0x34eba0, 3);
            let obj = DATA;
            let list = DATA + 0x1000;
            let man = DATA + 0x2000;
            c.bus.write32(GAME_GP + 0xce8, man);
            c.bus.write32(man + 0x28c, 0);
            c.bus.write32(list + 0x78, 0);
            c.bus.write32(list + 0x88, 0);
            c.bus.write32(obj + 0x2c, list);
            // `list->head == link` decides the result.
            c.bus.write32(list + 0xc, if seed % 2 == 0 { obj + 0x14 } else { obj + 0x40 });
            c.g[A0][0] = sx(obj);
        });
    }

    #[test]
    fn rider_get_mass_matches() {
        check(ee::s00::f_0011ff98__generated, rider_get_mass, 250, &|c, seed| {
            let mut r = machine(c, seed, 0x11ff98, 5);
            // The back end's singleton exists (else it builds itself from
            // disc); its interface table starts empty, so the interface call
            // goes through a null vtable slot answered by the fake hook.
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x3000);
            c.bus.write32(GAME_GP - 0x1efc, 0);
            let rider = DATA;
            c.bus.write32(rider + 0x86c, r.next_u32() % 4);
            c.bus.write32(rider + 0xb34, [0u32, 1, 0xffff_ffff, 5][(seed % 4) as usize]);
            c.bus.write32(rider + 0x2fc, r.f32(0.4));
            c.pc = 0x0011_ff98;
            c.g[A0][0] = sx(rider);
        });
    }

    #[test]
    fn rider_update_leg_ik_blend_matches() {
        check(ee::s00::f_00120d90__generated, rider_update_leg_ik_blend, 250, &|c, seed| {
            machine(c, seed, 0x120d90, 1);
            let rider = DATA;
            let anim = DATA + 0x200;
            let table = 0x0044_6990u32;
            let idx = (seed % 4) as u32;
            c.bus.write32(rider + 0x784, anim);
            // `AnimTable_GetEntry` indexes the table with the raw id word.
            c.bus.write32(anim + 8, if seed % 8 == 7 { 0x1b6 } else { idx * 4 });
            if seed % 8 != 7 {
                let id = if seed % 8 < 4 { 0x14 + seed % 7 } else { 0x30 };
                c.bus.write32(table + idx * 4, id);
            }
            // +-1 makes the four approach paths (in/out of an air animation,
            // below/above the step) deterministic: both are reached.
            c.bus.write32(rider + 0x318, if seed % 2 == 0 { 0xbf80_0000 } else { 0x3f80_0000 });
            // The five approach constants (`riderApproach`'s arguments).
            put_words(
                c,
                GAME_GP - 0x7934,
                &[0x3c23_d70a, 0x3f80_0000, 0x3dcc_cccd, 0x3c23_d70a, 0x3f80_0000],
            );
            c.g[A0][0] = sx(rider);
        });
    }

    #[test]
    fn rider_update_danger_state_matches() {
        check(ee::s00::f_00122cf0__generated, rider_update_danger_state, 250, &|c, seed| {
            machine(c, seed, 0x122cf0, 5);
            let rider = DATA;
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            let ctx = DATA + 0x300;
            let danger = DATA + 0x400;
            put_world(c, world, man, ctx);
            c.bus.write32(ctx + 0xa8, danger);
            let index = (seed % 4) as u32;
            c.bus.write32(rider + 0x86c, index);
            c.bus.write32(
                danger + index + 0x710,
                if seed % 3 == 0 { 0 } else { 1 + seed % 5 },
            );
            c.bus.write32(rider + 0x5b0, [0u32, 1, 2][(seed % 3) as usize]);
            c.bus.write32(rider + 0x790, DATA + 0x800);
            let state = DATA + 0x500;
            c.bus.write32(rider + 0x77c, state);
            c.bus.write32(state + 0xde0, [0u32, 1, 2][(seed % 3) as usize]);
            c.bus.write32(state + 0x30, if seed % 4 == 2 { 1 } else { 0 });
            c.bus.write32(rider + 0x5b4, if seed % 2 == 0 { 0xbf80_0000 } else { 0x3f80_0000 });
            put_words(c, GAME_GP - 0x78b4, &[0x3f80_0000]);
            c.g[A0][0] = sx(rider);
        });
    }

    #[test]
    fn ssx_audio_system_set_player_trick_score_matches() {
        check(ee::s07::f_00290f58__generated, ssx_audio_system_set_player_trick_score, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x290f58, 6);
            let app = DATA;
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            let racman = DATA + 0x300;
            let view = DATA + 0x400;
            put_world(c, world, man, racman);
            c.bus.write32(man + 0x28, 0); // no replay
            c.bus.write32(man + 0x84, view);
            c.bus.write32(view + 0x10, if seed % 4 == 1 { 2 } else { 1 });
            // An empty player table: FindRider answers 0 for both players.
            c.bus.write32(racman + 0x40, 0);
            c.bus.write32(racman + 0x78, 0);
            c.bus.write32(racman + 0x7c, 0);
            let rider = [0u32, 1, 2, 0xffff_ffff][(seed % 4) as usize];
            c.f[12] = r.f32(10.0);
            c.g[A0][0] = sx(app);
            c.g[A1][0] = sx(rider);
        });
    }

    #[test]
    fn world_painter_get_irradiance_name0_matches() {
        world_painter_name_matches(
            ee::s09::f_002ef0e8__generated,
            world_painter_get_irradiance_name0,
            0x2ef0e8,
            0x128,
        );
    }

    #[test]
    fn world_painter_get_irradiance_name1_matches() {
        world_painter_name_matches(
            ee::s09::f_002ef140__generated,
            world_painter_get_irradiance_name1,
            0x2ef140,
            0x130,
        );
    }

    /// The two irradiance-name getters differ only in the vtable slot.
    fn world_painter_name_matches(
        generated: GuestFn,
        hand: GuestFn,
        salt: u64,
        slot: u32,
    ) {
        check(generated, hand, 200, &|c, seed| {
            let mut r = machine(c, seed, salt, 2);
            let entry = 0x004f_a460u32;
            let obj = DATA + 0x100;
            let cell = DATA + 0x200;
            // The painter exists: return the built-in name.
            c.bus.write32(GAME_GP + 0xa74, if seed % 3 == 0 { 0 } else { 1 });
            if seed % 3 == 0 {
                return;
            }
            if seed % 3 == 1 {
                // Empty object table: the call target is 0 (fake).
                c.bus.write32(entry + 0x1c, 0);
                return;
            }
            let vt = fakes::vtable(c, &mut r, &[slot]);
            c.bus.write32(cell, obj);
            c.bus.write32(obj + 4, vt);
            c.bus.write32(entry + 0x1c, cell);
        });
    }

    #[test]
    fn rider_base_update_main_state_matches() {
        check(ee::s00::f_00111408__generated, rider_base_update_main_state, 200, &|c, seed| {
            rider_state_table_test(c, seed, 0x0011_1408, 0xde0, 0x0045_6b50, 0x0011_1494);
        });
    }

    #[test]
    fn rider_base_post_update_main_state_matches() {
        check(ee::s00::f_001114a0__generated, rider_base_post_update_main_state, 200, &|c, seed| {
            rider_state_table_test(c, seed, 0x0011_14a0, 0xde0, 0x0045_6b70, 0x0011_152c);
        });
    }

    #[test]
    fn rider_base_handle_sub_state_input_matches() {
        check(ee::s00::f_00111728__generated, rider_base_handle_sub_state_input, 200, &|c, seed| {
            rider_state_table_test(c, seed, 0x0011_1728, 0xde4, 0x0045_6c10, 0x0011_1824);
        });
    }

    /// The shared setup of the three state tables: a rider whose state
    /// selects a table entry, and a table that either stays inside the
    /// function (its tail / epilogue block) or is empty (an indirect jump
    /// out, answered by the fake jump hook).
    fn rider_state_table_test(c: &mut Cpu, seed: u32, salt: u64, state_off: u32, table: u32, tail: u32) {
        machine(c, seed, salt, 0);
        let rider = DATA;
        let states = [0u32, 1, 2, 3, 4, 5, 6, 13, 14, 0xffff_ffff];
        c.bus.write32(rider + state_off, states[(seed as usize) % states.len()]);
        let case = if seed % 3 == 0 {
            0
        } else if seed % 3 == 1 {
            tail
        } else {
            tail + 4
        };
        put_words(c, table, &[case; 14]);
        c.pc = salt as u32;
        c.g[A0][0] = sx(rider);
    }

    #[test]
    fn c_air_streamer_fx_update_matches() {
        check(ee::s09::f_002ef6d0__generated, c_air_streamer_fx_update, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2ef6d0, 2);
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            let ctx = DATA + 0x300;
            put_world(c, world, man, ctx);
            c.bus.write32(ctx + 8, if seed % 2 == 0 { 0 } else { 1 });
            let fx = DATA;
            let owner = DATA + 0x1000;
            let state = DATA + 0x1400;
            c.bus.write32(fx, owner);
            c.bus.write32(owner + 0x77c, state);
            c.bus.write32(state + 0xde0, [1u32, 0, 2, 3, 1][(seed % 5) as usize]);
            c.bus.write32(fx + 0x4, DATA + 0x2000); // ring
            c.bus.write32(fx + 0x8, seed % 0x19); // head
            c.bus.write32(fx + 0xc, [0u32, 1, 2, 4, 0x20][(seed % 5) as usize]); // count
            c.bus.write32(fx + 0x14, [1u32, 2, 3][(seed % 3) as usize]); // len
            c.bus.write32(fx + 0x10, if seed % 2 == 0 { 0xbf80_0000 } else { r.f32(2.0) });
            c.bus.write32(owner + 0x780, DATA + 0x1800);
            c.bus.write32(owner + 0x89c, 1);
            c.bus.write32(owner + 0x8a4, 2);
            c.bus.write32(owner + 0x1e0, r.next_u32());
            put_words(c, GAME_GP - 0x393c, &[0x3c88_8889]);
            c.pc = 0x002e_f6d0;
            c.g[A0][0] = sx(fx);
        });
    }

    #[test]
    fn c_board_trail_fx_is_active_matches() {
        check(ee::s09::f_002e87e8__generated, c_board_trail_fx_is_active, 300, &|c, seed| {
            machine(c, seed, 0x2e87e8, 3);
            let fx = DATA;
            let owner = DATA + 0x100;
            let state = DATA + 0x400;
            let anim = DATA + 0x500;
            let seq = DATA + 0x600;
            c.bus.write32(fx, owner);
            c.bus.write32(owner + 0x77c, state);
            c.bus.write32(state + 0xde0, [0u32, 1, 2, 3, 4, 13, 14, 22][(seed % 8) as usize]);
            c.bus.write32(state + 0x30, if seed % 3 == 0 { 1 } else { 0 });
            c.bus.write32(owner + 0x438, [0u32, 1, 2, 3, 4, 5, 13, 14, 0x30][(seed % 9) as usize]);
            c.bus.write32(owner + 0x784, anim);
            c.bus.write32(anim + 8, if seed % 4 == 1 { 0x16 } else { 0x10 });
            c.bus.write32(anim + 0x50, seq);
            c.bus.write32(seq + 0x14, seq); // a one-element sequence list
            c.bus.write32(seq + 0xb0, if seed % 2 == 0 { 1 } else { 0 });
            c.bus.write32(seq + 0xb4, if seed % 5 == 2 { 1 } else { 0 });
            c.bus.write32(owner + 0xac4, if seed % 3 == 1 { 1 } else { 0 });
            c.bus.write32(owner + 0xad0, if seed % 2 == 0 { 1 } else { 0 });
            c.bus.write32(owner + 0xafc, if seed % 7 == 3 { 0 } else { 1 });
            c.bus.write32(owner + 0xb00, if seed % 4 == 0 { 0 } else { 1 });
            // The state switch's table (all entries are blocks of the
            // function itself: every state below 14 reaches one of them).
            let targets = [0x002e_88f8u32, 0x002e_8900, 0x002e_8908, 0x002e_8910, 0x002e_8920];
            for i in 0..14u32 {
                c.bus.write32(0x0048_7b40 + 4 * i, targets[(i % 5) as usize]);
            }
            put_words(
                c,
                GAME_GP + 0x14a0,
                &[0x3f80_0000, 0x3f80_0000, 0x4000_0000, 0x3f00_0000, 0x3f40_0000],
            );
            c.pc = 0x002e_87e8;
            c.g[A0][0] = sx(fx);
        });
    }
}

// ---------------------------------------------------------------------------
// Air streamer wake ribbon.

/// `cAirStreamerFX` (a wake ribbon attached to a rider) fields.
mod air_streamer {
    /// The owner entity.
    pub const OWNER: u32 = 0;
    /// The ring of ribbon points (25 slots of two vec4s).
    pub const RING: u32 = 0x4;
    /// Ring slot index, 0..25.
    pub const HEAD: u32 = 0x8;
    /// Live slot count, approached towards 0 or 1.
    pub const COUNT: u32 = 0xc;
    /// How many slots the ribbon may use.
    pub const LEN: u32 = 0x14;
    /// The ribbon's texture start, a float wrapped to [0, 1].
    pub const START: u32 = 0x10;
}

/// `void cAirStreamerFX_Update(cAirStreamerFX* self)` (0x2ef6d0): append the
/// rider's wake points to the ribbon (once per other AI frame), keep the live
/// slot count near two, and advance the texture start by the wake length,
/// wrapped to [0, 1]. VU0 does the vector scaling and subtraction, as the
/// original does.
pub fn c_air_streamer_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x90 };
        air_streamer_at(c, frame, c.pc)
    })
}

fn air_streamer_at(c: &mut Cpu, frame: Frame, pc: u32) -> Flow {
    match pc {
        0x002e_f6d0 => {
            c.cycles += 9;
            Frame::open(c, 0x90);
            frame.save(c, S0, 0x80);
            frame.save_ra(c, 0x70);
            let fx = abi::arg(c, 0);
            c.g[S0][0] = sx(fx);
            c.g[A0][0] = Addr(fx).lw(c);
            call_ptr(c, 0x0011_fe98, 0x002e_f6e8)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[A0][0] = Addr(p(fx, air_streamer::COUNT)).lw(c);
                air_approach_count(c, frame)
            } else {
                air_build(c, frame)
            }
        }
        0x002e_f6f4 => air_build(c, frame),
        0x002e_f7bc => air_build_second(c, frame),
        0x002e_f82c => air_step(c, frame),
        0x002e_f848 => air_limit(c, frame),
        0x002e_f860 => air_tail_bump(c, frame),
        0x002e_f868 => air_copy(c, frame),
        0x002e_f894 => air_approach_count(c, frame),
        0x002e_f8a4 => air_approach_count_neg(c, frame),
        0x002e_f8b4 => air_store_count(c, frame),
        0x002e_f8b8 => air_load_owner(c, frame),
        0x002e_f8bc => air_length(c, frame),
        0x002e_f908 => air_start_increase(c, frame),
        0x002e_f918 => air_start_clamp(c, frame),
        0x002e_f934 => air_start_subtract(c, frame),
        0x002e_f940 => {
            air_exit(c, frame);
            Ok(())
        }
        other => {
            c.jump_out(other, 0x002e_f6d0, 0x002e_f94c);
            Ok(())
        }
    }
}

/// 0x2ef6f4: fetch the three bone points of the ribbon (the rider's two wake
/// bones and their up axis) to the stack, scaled by 90 degrees.
fn air_build(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 50;
    c.g[A2][0] = Addr(r32(c, S0)).lw(c);
    c.lwc1(1, gp(c, -0x3940));
    c.g[V0][0] = Addr(p(r32(c, A2), 0x780)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, A2), 0x89c)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, V0), 0x30)).lw(c);
    c.g[V1][0] = shl32(c.g[V1][0], 6);
    c.g[AT][0] = 0x42b4_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
    c.lq(2, p(r32(c, A0), 0x30));
    c.sq(2, sp(c));
    c.g[A1][0] = Addr(p(r32(c, A2), 0x780)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, A2), 0x8a4)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, A1), 0x30)).lw(c);
    c.g[V1][0] = shl32(c.g[V1][0], 6);
    c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
    c.lq(2, p(r32(c, A0), 0x30));
    c.sq(2, p(sp(c), 0x10));
    c.g[A1][0] = Addr(p(r32(c, A2), 0x780)).lw(c);
    c.g[V1][0] = Addr(p(r32(c, A2), 0x8a4)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, A1), 0x34)).lw(c);
    c.g[V1][0] = shl32(c.g[V1][0], 6);
    c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
    c.lq(2, r32(c, A0));
    c.sq(2, p(sp(c), 0x20));
    c.mfc1(3, 1);
    c.lqc2(4, sp(c));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x50));
    c.lq(2, p(sp(c), 0x50));
    c.sq(2, p(sp(c), 0x40));
    c.lqc2(3, p(sp(c), 0x10));
    c.lqc2(4, p(sp(c), 0x40));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0x50));
    c.lq(2, p(sp(c), 0x50));
    c.g[A0][0] = Addr(p(r32(c, S0), air_streamer::HEAD)).lw(c);
    c.sq(2, p(sp(c), 0x30));
    c.g[A0][0] = shl32(c.g[A0][0], 5);
    c.g[V1][0] = Addr(p(r32(c, S0), air_streamer::RING)).lw(c);
    c.mfc1(5, 0);
    c.lqc2(4, p(sp(c), 0x20));
    c.qmtc2(5, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x60));
    c.lq(2, p(sp(c), 0x60));
    c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
    c.sq(2, p(sp(c), 0x50));
    c.lqc2(3, p(sp(c), 0x30));
    air_build_second(c, frame)
}

/// 0x2ef7bc: write the new point and the difference vector into the ribbon,
/// then let the AI frame counter decide whether to append.
fn air_build_second(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 28;
    c.lqc2(4, p(sp(c), 0x50));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0x60));
    c.lq(2, p(sp(c), 0x60));
    c.sq(2, p(sp(c), 0x40));
    c.sq(2, r32(c, A0));
    c.g[V1][0] = Addr(p(r32(c, S0), air_streamer::HEAD)).lw(c);
    c.g[A0][0] = Addr(p(r32(c, S0), air_streamer::RING)).lw(c);
    c.mfc1(2, 0);
    c.lqc2(4, p(sp(c), 0x20));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x60));
    c.g[V1][0] = shl32(c.g[V1][0], 5);
    c.lq(2, p(sp(c), 0x60));
    c.g[V1][0] = add32(c.g[V1][0], c.g[A0][0]);
    c.sq(2, p(sp(c), 0x50));
    c.lqc2(3, p(sp(c), 0x30));
    c.lqc2(4, p(sp(c), 0x50));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(sp(c), 0x60));
    c.lq(2, p(sp(c), 0x60));
    c.sq(2, p(sp(c), 0x40));
    c.sq(2, p(r32(c, V1), 0x10));
    call_ptr(c, 0x0012_98c8, 0x002e_f820)?;
    c.g[V0][0] &= 1;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        air_length(c, frame)
    } else {
        air_step(c, frame)
    }
}

/// 0x2ef82c: advance the tail index.
fn air_step(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[A1][0] = Addr(p(r32(c, S0), air_streamer::HEAD)).lw(c);
    c.g[V1][0] = 0x19;
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(1));
    c.div(0, 2, 3);
    c.sw(p(r32(c, S0), air_streamer::HEAD), c.g[V0][0]);
    air_limit(c, frame)
}

/// 0x2ef848: wrap the head index at the limit.
fn air_limit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V1][0] = Addr(p(r32(c, S0), air_streamer::COUNT)).lw(c);
    c.g[V0][0] = Addr(p(r32(c, S0), air_streamer::LEN)).lw(c);
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A0][0] = c.hi[0];
    let room = c.g[V0][0] != 0;
    c.sw(p(r32(c, S0), air_streamer::HEAD), c.g[A0][0]);
    if room {
        air_tail_bump(c, frame)
    } else {
        air_copy(c, frame)
    }
}

/// 0x2ef860.
fn air_tail_bump(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
    c.sw(p(r32(c, S0), air_streamer::COUNT), c.g[V0][0]);
    air_copy(c, frame)
}

/// 0x2ef868: copy the head point onto the tail point.
fn air_copy(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 11;
    c.g[V0][0] = Addr(p(r32(c, S0), air_streamer::RING)).lw(c);
    c.g[A1][0] = shl32(c.g[A1][0], 5);
    c.g[V1][0] = Addr(p(r32(c, S0), air_streamer::HEAD)).lw(c);
    c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
    c.lq(4, r32(c, A1));
    c.g[V1][0] = shl32(c.g[V1][0], 5);
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.sq(4, r32(c, V1));
    c.lq(2, p(r32(c, A1), 0x10));
    c.sq(2, p(r32(c, V1), 0x10));
    air_load_owner(c, frame)
}

/// 0x2ef894: `Approach(count, 2)` — the slot count jumps by two when it is
/// long enough, else falls back to the negative side (0x2ef8a4).
fn air_approach_count(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = 2;
    let over = 2i64 < c.g[A0][0] as i64;
    if over {
        c.g[V1][0] = sx(r32(c, A0).wrapping_sub(0x2));
    } else {
        c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x2));
    }
    c.g[V0][0] = over as u64;
    if !over {
        // 0x2ef8a4: the negative half, reached by the branch.
        c.cycles += 4;
        let neg = -3i64 < c.g[A0][0] as i64;
        c.g[V0][0] = neg as u64;
        if neg {
            c.g[V1][0] = 0;
        }
    }
    air_store_count(c, frame)
}

/// 0x2ef8a4: the negative half of the index adjustment (reached by hand).
fn air_approach_count_neg(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_ffff_fffd;
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(0x2));
    let neg = -3i64 < c.g[A0][0] as i64;
    c.g[V0][0] = neg as u64;
    if neg {
        c.g[V1][0] = 0;
    }
    air_store_count(c, frame)
}

/// 0x2ef8b4.
fn air_store_count(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.sw(p(r32(c, S0), air_streamer::COUNT), c.g[V1][0]);
    air_load_owner(c, frame)
}

/// 0x2ef8b8.
fn air_load_owner(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
    air_length(c, frame)
}

/// 0x2ef8bc: advance the texture start by the wake length (a VU0 dot product
/// and vsqrt).
fn air_length(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 19;
    c.lwc1(2, gp(c, -0x393c));
    c.lqc2(3, p(r32(c, V0), 0x1e0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0);
    c.lwc1(3, p(r32(c, S0), air_streamer::START));
    c.mul_s(0, 0, 2);
    c.mtc1(0, 1);
    c.sub_s(3, 3, 0);
    c.c_lt_s(3, 1);
    let shrinking = !c.fcc();
    c.swc1(3, p(r32(c, S0), air_streamer::START));
    if shrinking {
        air_start_clamp(c, frame)
    } else {
        air_start_increase(c, frame)
    }
}

/// 0x2ef908.
fn air_start_increase(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.add_s(0, 3, 0);
    c.swc1(0, p(r32(c, S0), air_streamer::START));
    air_start_clamp(c, frame)
}

/// 0x2ef918: wrap the texture start to [0, 1].
fn air_start_clamp(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.lwc1(0, p(r32(c, S0), air_streamer::START));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        frame.restore(c, S0, 0x80);
        air_exit(c, frame);
        return Ok(());
    }
    air_start_subtract(c, frame)
}

/// 0x2ef934.
fn air_start_subtract(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.sub_s(0, 0, 1);
    c.swc1(0, p(r32(c, S0), air_streamer::START));
    frame.restore(c, S0, 0x80);
    air_exit(c, frame)
}

/// 0x2ef940: the epilogue.
fn air_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0x70);
    frame.close(c);
    Ok(())
}
