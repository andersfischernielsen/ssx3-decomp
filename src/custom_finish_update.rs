//! Native post-finish slowdown, overlay and optional cinematic sequencing.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0023_3cd8, update, "GameStateFinish_Update")];

const PHASE: u32 = 0x10;
const OVERLAY_DELAY: u32 = 0x14;
const APP_GP: u32 = 0x848;
const SCRIPT_ENGINE_GP: u32 = 0x84c;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn gp(c: &Cpu, negative: u32) -> u32 {
    r(c, GP).wrapping_sub(negative)
}

/// The generated branches restore S0/S1 in several delay slots. Some paths
/// through GotoNext repeat both loads; retain those distinctions.
enum Restore {
    Both,
    S1,
    Neither,
}

/// Find the native `finishov` state and call its visibility slot. V1 carries
/// the app on entry, as in the timer and phase-1 callers.
fn overlay_visibility(c: &mut Cpu, visible: u64, returns: [u32; 3]) -> Flow {
    c.cycles += 12;
    c.g[A0][0] = 0x0048_0000;
    c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x3ff0));
    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x84));
    c.g[S0][0] = c.lw(r(c, V0).wrapping_add(0x48));
    c.g[S0][0] = sx(r(c, S0).wrapping_add(0x18));
    call_ptr(c, 0x0031_7670, returns[0])?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_f9d8, returns[1])?;
    c.g[A3][0] = c.g[V0][0];
    let found = c.g[A3][0] != 0;
    crate::custom_finish_trace::observe(
        c, if visible != 0 { "overlay-show-lookup" } else { "overlay-hide-lookup" },
        r(c, S3), r(c, A3),
    );
    c.g[A1][0] = visible;
    if found {
        c.cycles += 6;
        c.g[V1][0] = c.lw(r(c, A3).wrapping_add(8));
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lh(r(c, V1).wrapping_add(0xc0));
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0xc4));
        let target = r(c, V0);
        c.g[A0][0] = sx(r(c, A3).wrapping_add(r(c, A0)));
        call_ptr(c, target, returns[2])?;
    }
    Ok(())
}

/// Decrement the delay and open the normal finish screen when it expires.
/// Return true for the one branch whose delay slot already loaded S0.
fn overlay_timer(c: &mut Cpu) -> Result<bool, Unwound> {
    c.g[V0][0] = c.lw(r(c, S3).wrapping_add(OVERLAY_DELAY));
    let armed = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
    if !armed {
        return Ok(false);
    }
    c.cycles += 2;
    let expired = c.g[V0][0] == 0;
    c.sw(r(c, S3).wrapping_add(OVERLAY_DELAY), c.g[V0][0]);
    if !expired {
        return Ok(false);
    }
    c.cycles += 6;
    c.g[V0][0] = c.lw(gp(c, APP_GP));
    c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x84));
    c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0x28));
    c.g[A0][0] = c.lw(r(c, A0));
    let zero_state = c.g[A0][0] == 0;
    c.g[A1][0] = 0;
    if !zero_state {
        c.cycles += 1;
        c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
    }
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        // The module has explicitly rejected this request, so no finish
        // state was created. Keep it pending for imported events instead
        // of consuming their only menu-open attempt. Retrying only this
        // pre-call rejection cannot queue duplicate finish dialogs.
        if crate::custom_tracks::active().is_some() {
            c.sw(r(c, S3).wrapping_add(OVERLAY_DELAY), 1);
        }
        crate::custom_finish_trace::observe(c, "overlay-timer-gated", r(c, S3), r(c, A0));
        c.g[S0][0] = c.lw(r(c, S3).wrapping_add(PHASE));
        return Ok(true);
    }
    c.cycles += 6;
    crate::custom_finish_trace::observe(c, "overlay-delay-expired", r(c, S3), 0);
    call_ptr(c, 0x0020_cc60, 0x0023_3d54)?;
    c.g[V1][0] = c.lw(r(c, S3).wrapping_add(PHASE));
    c.g[V0][0] = 2;
    let already_shown = c.g[V1][0] == c.g[V0][0];
    c.g[V1][0] = c.lw(gp(c, APP_GP));
    if !already_shown {
        overlay_visibility(c, 0, [0x0023_3d7c, 0x0023_3d88, 0x0023_3dac])?;
    }
    Ok(false)
}

/// The original four-component VU length, used to wait for all human riders
/// to slow down before selecting a finish presentation.
fn riders_still_moving(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 7;
    c.g[V0][0] = 0x0053_0000;
    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x66a8));
    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x84));
    c.g[S5][0] = c.lw(r(c, V1).wrapping_add(0xc0));
    c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0xc));
    // Additional worlds deliberately have no template finish-camera list.
    // Their result overlay must not wait for every rider to settle on a
    // slope, conveyor or lift. Preserve the native setup of S5/S1 above.
    let skip_speed_wait = c.g[A0][0] != 0 || crate::custom_tracks::active().is_some();
    c.g[S1][0] = c.lw(r(c, V1).wrapping_add(0x7c));
    if skip_speed_wait {
        return Ok(false);
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
    let have_players = c.g[V0][0] != 0;
    c.g[S2][0] = 0;
    if !have_players {
        return Ok(false);
    }
    c.cycles += 5;
    c.lwc1(21, gp(c, 0x5174));
    c.g[S0][0] = sx(r(c, V1).wrapping_add(0x40));
    c.lwc1(20, r(c, GP).wrapping_add(0x1e70));
    c.g[S4][0] = 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r(c, GP).wrapping_add(0x1e74));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.swc1(21, r(c, GP).wrapping_add(0x1e70));
            c.sw(r(c, GP).wrapping_add(0x1e74), c.g[S4][0]);
        }
        c.g[V0][0] = c.lw(r(c, S0));
        c.cycles += 21;
        c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x18));
        c.g[V1][0] = c.lw(r(c, A0).wrapping_add(0x6c0));
        c.g[A0][0] = sx(r(c, A0).wrapping_add(0x6c0));
        c.g[A1][0] = c.lh(r(c, V1).wrapping_add(0x10));
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x14));
        let target = r(c, V0);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
        call_ptr(c, target, 0x0023_3e54)?;
        c.lqc2(3, r(c, V0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V1, 22);
        c.mtc1(V1, 0);
        c.c_lt_s(20, 0);
        let moving = c.fcc();
        c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
        if moving {
            return Ok(true);
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
        if !more {
            return Ok(false);
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_3e20);
        }
    }
}

/// Phase zero: retain the slowdown gate, select any available presentation,
/// then advance to the native finish overlay. An empty list leaves next=7.
fn prepare_finish(c: &mut Cpu, frame: Frame) -> Result<Restore, Unwound> {
    if riders_still_moving(c)? {
        crate::custom_finish_trace::observe(c, "wait-rider-speed", r(c, S3), 0);
        return Ok(Restore::Both);
    }
    c.cycles += 7;
    c.g[V1][0] = 0x0053_0000;
    c.g[S0][0] = c.g[V1][0];
    c.g[V1][0] = sx(r(c, V1).wrapping_add(0x5bc8));
    c.g[V0][0] = c.lb(r(c, V1).wrapping_add(0x48));
    c.g[V0][0] ^= 4;
    let free_ride = c.g[V0][0] == 0;
    c.g[A0][0] = 0;
    if !free_ride {
        c.cycles += 3;
        c.g[V0][0] = c.lw(r(c, S5).wrapping_add(0x98));
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(r(c, S5).wrapping_add(0x9c));
            c.g[A0][0] = (c.g[V0][0] < 1) as u64;
        }
    }
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        crate::custom_finish_trace::observe(c, "wait-mode-results", r(c, S3), 0);
        frame.restore(c, S0, 0x60);
        return Ok(Restore::S1);
    }
    c.cycles += 13;
    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    c.g[A1][0] = 0;
    call_ptr(c, 0x0027_90a0, 0x0023_3edc)?;
    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    call_ptr(c, 0x0027_ac60, 0x0023_3ee4)?;
    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    c.g[A1][0] = 0;
    call_ptr(c, 0x0027_8f38, 0x0023_3ef0)?;
    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    c.g[A1][0] = 0;
    call_ptr(c, 0x0027_9248, 0x0023_3efc)?;
    let have_cinematic = (c.g[V0][0] as i64) > 0;
    c.g[V0][0] = 12;
    if have_cinematic {
        c.cycles += 1;
        c.sw(r(c, S3), c.g[V0][0]);
    }
    crate::custom_finish_trace::observe(c, "presentation-selected", r(c, S3), u32::from(have_cinematic));
    c.cycles += 4;
    c.g[V0][0] = sx(r(c, S0).wrapping_add(0x5bc8));
    c.g[V1][0] = c.lb(r(c, V0).wrapping_add(0x48));
    let race_mode = c.g[V1][0] == 0;
    c.g[V0][0] = 1;
    if race_mode && crate::custom_tracks::active().is_none() {
        c.cycles += 5;
        call_ptr(c, 0x0015_5a50, 0x0023_3f20)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_5e58, 0x0023_3f28)?;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    c.sw(r(c, S3).wrapping_add(PHASE), c.g[V0][0]);
    Ok(Restore::Both)
}

fn show_finish(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    c.g[A1][0] = 0;
    call_ptr(c, 0x0027_91d8, 0x0023_3f40)?;
    let loading = c.g[V0][0] == c.g[S0][0];
    c.g[V1][0] = c.lw(gp(c, APP_GP));
    if loading {
        crate::custom_finish_trace::observe(c, "wait-presentation", r(c, S3), r(c, V0));
        return Ok(());
    }
    overlay_visibility(c, 1, [0x0023_3f60, 0x0023_3f6c, 0x0023_3f90])?;
    c.cycles += 8;
    c.g[V0][0] = c.lw(gp(c, APP_GP));
    c.g[A1][0] = 4;
    c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x84));
    c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0x94));
    call_ptr(c, 0x001f_36d8, 0x0023_3fa4)?;
    c.g[V0][0] = 2;
    c.sw(r(c, S3).wrapping_add(PHASE), c.g[V0][0]);
    Ok(())
}

/// Phase two waits for the finish overlay. Only the original free-ride
/// special case advances immediately here; event UI actions advance races.
fn finish_visible(c: &mut Cpu, frame: Frame) -> Result<Restore, Unwound> {
    crate::custom_finish_trace::observe(c, "finish-visible", r(c, S3), 0);
    c.cycles += 5;
    c.g[V0][0] = 0x0053_0000;
    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x5bc8));
    c.g[V1][0] = c.lb(r(c, V0).wrapping_add(0x49));
    let flagged = c.g[V1][0] != 0;
    frame.restore(c, S1, 0x50);
    if flagged {
        return Ok(Restore::Neither);
    }
    c.cycles += 4;
    c.g[V1][0] = c.lb(r(c, V0).wrapping_add(0x48));
    c.g[V0][0] = 4;
    if c.g[V1][0] != c.g[V0][0] {
        return Ok(Restore::Neither);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0023_3aa0, 0x0023_3fdc)?;
    Ok(Restore::Both)
}

/// `void GameStateFinish_Update(GameStateFinish* self)` (0x233cd8).
/// Keep the native finish delay, rider slowdown, result overlay, and optional
/// race presentation. Imported courses bypass the speed wait and career
/// mutations; native mode completion and UI actions still own progression.
pub fn update(c: &mut Cpu) {
    crate::custom_finish_trace::observe(c, "finish-update", r(c, A0), 0);
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S3, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x78);
        frame.save_fpr(c, 20, 0x70);
        call_ptr(c, 0x0014_dc80, 0x0023_3d08)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_3d14)?;
        if !overlay_timer(c)? {
            c.cycles += 1;
            c.g[S0][0] = c.lw(r(c, S3).wrapping_add(PHASE));
        }

        c.cycles += 3;
        c.g[V0][0] = 1;
        let showing = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
        let restore = if showing {
            show_finish(c)?;
            Restore::Both
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                let preparing = c.g[S0][0] == 0;
                c.g[V1][0] = c.lw(gp(c, APP_GP));
                if preparing {
                    prepare_finish(c, frame)?
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x60);
                    Restore::S1
                }
            } else {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let visible = c.g[S0][0] == c.g[V0][0];
                frame.restore(c, S0, 0x60);
                if visible {
                    finish_visible(c, frame)?
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x50);
                    Restore::Neither
                }
            }
        };
        if matches!(restore, Restore::Both) {
            c.cycles += 1;
            frame.restore(c, S0, 0x60);
        }
        if !matches!(restore, Restore::Neither) {
            c.cycles += 1;
            frame.restore(c, S1, 0x50);
        }
        c.cycles += 9;
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    });
}
