//! Native race-end presentation selection, excluded for additional courses.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0027_ac60,
    queue_race_cutscene,
    "ScriptEngine_QueueRaceCutscene",
)];

const CURRENT_RACE: u32 = 0x0053_5bc8;
const APP_GP: u32 = 0x848;
const SCRIPT_ENGINE_GP: u32 = 0x84c;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}

#[inline]
fn global(c: &Cpu, negative: u32) -> u32 {
    r(c, GP).wrapping_sub(negative)
}

/// The original frequently restores S0 or S1 in a branch delay slot.
enum Restore {
    S0,
    S1,
    Rest,
}

/// Call a rider's primary interface, already addressed by A0/V1. This
/// function only factors the native vtable-call sequence from this routine.
fn rider_call(c: &mut Cpu, slot: u32, back: u32) -> Flow {
    c.g[A1][0] = c.lh(at(c, V1, slot));
    c.g[V0][0] = c.lw(at(c, V1, slot + 4));
    let target = r(c, V0);
    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
    call_ptr(c, target, back)
}

/// Determine whether one of the human riders finished in the first three.
fn find_human_podium(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c, S2, 0x7c));
    let no_humans = (c.g[V0][0] as i64) <= 0;
    c.g[S1][0] = 0;
    if no_humans {
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = 0x0053_0000;
    c.g[S0][0] = sx(at(c, S2, 0x40));
    c.g[S3][0] = sx(0x0053_6730);
    loop {
        c.cycles += 14;
        c.g[V0][0] = c.lw(r(c, S0));
        c.g[A0][0] = c.lw(at(c, V0, 0x18));
        c.g[V1][0] = c.lw(at(c, A0, 0x6c0));
        c.g[A0][0] = sx(at(c, A0, 0x6c0));
        rider_call(c, 0x38, 0x0027_ad28)?;
        c.g[V0][0] = sx(r(c, V0) << 2);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, S3)));
        c.g[V1][0] = c.lw(r(c, V0));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 3) as u64;
        let on_podium = c.g[V1][0] != 0;
        c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
        if on_podium {
            c.cycles += 2;
            c.g[S4][0] = 1;
            if event_due(c) {
                deliver_event_here(c, 0x0027_ad50);
            }
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw(at(c, S2, 0x7c));
        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0027_ad08);
        }
    }
}

/// Build the three podium rider arguments and choose their native category.
fn queue_podium(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.g[S4][0] = 3;
    c.g[S3][0] = sx(r(c, V0).wrapping_add(0x6708));
    c.g[S1][0] = 0;
    c.g[S7][0] = sx(at(c, SP, 0x10));
    c.g[FP][0] = u64::MAX;
    c.g[S5][0] = sx(at(c, S2, 0x28));
    c.g[S0][0] = c.g[SP][0];
    loop {
        c.cycles += 6;
        c.g[V1][0] = c.lw(r(c, S3));
        c.sw(r(c, S0), c.g[V1][0]);
        c.g[V0][0] = c.lw(at(c, S2, 0x78));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let rider_exists = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r(c, V1) << 2);
        if !rider_exists {
            c.cycles += 2;
            c.sw(r(c, S0), c.g[FP][0]);
        } else {
            c.cycles += 9;
            c.g[V0][0] = sx(r(c, S5).wrapping_add(r(c, V0)));
            c.g[A0][0] = c.lw(r(c, V0));
            c.g[V1][0] = c.lw(at(c, A0, 0x6c0));
            c.g[A0][0] = sx(at(c, A0, 0x6c0));
            rider_call(c, 0x40, 0x0027_adb8)?;
            if c.g[V0][0] != 0 {
                c.g[S4][0] = c.g[S1][0];
            }
        }
        c.cycles += 5;
        c.g[S1][0] = sx(r(c, S1).wrapping_add(1));
        c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
        c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S3][0] = sx(r(c, S3).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0027_ad78);
        }
    }

    c.cycles += 14;
    c.g[V1][0] = 0x0048_0000;
    c.g[V0][0] = sx(r(c, S4) << 2);
    c.g[A3][0] = sx(0x0048_1e90);
    c.ldl(A0, at(c, A3, 7));
    c.ldr(A0, r(c, A3));
    c.ldl(A1, at(c, A3, 0xf));
    c.ldr(A1, at(c, A3, 8));
    c.sdl(A0, at(c, SP, 0x17));
    c.sdr(A0, at(c, SP, 0x10));
    c.sdl(A1, at(c, SP, 0x1f));
    c.sdr(A1, at(c, SP, 0x18));
    c.g[V0][0] = sx(r(c, S7).wrapping_add(r(c, V0)));
    let winner_is_human = c.g[S4][0] == 0;
    c.g[S0][0] = c.lw(r(c, V0));
    if winner_is_human {
        c.cycles += 18;
        c.g[V0][0] = c.lw(r(c, SP));
        c.g[S0][0] = 9;
        c.g[V0][0] = sx(r(c, V0) << 2);
        c.g[V0][0] = sx(r(c, S2).wrapping_add(r(c, V0)));
        c.g[A0][0] = c.lw(at(c, V0, 0x28));
        c.g[V1][0] = c.lw(at(c, A0, 0x6c0));
        c.g[A0][0] = sx(at(c, A0, 0x6c0));
        rider_call(c, 0x38, 0x0027_ae34)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_a0b0, 0x0027_ae3c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r(c, V1).wrapping_sub(10));
        c.g[V0][0] = (c.g[V0][0] < 11) as u64;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = 10;
            c.cycles += 1;
            c.g[A0][0] = c.lw(global(c, SCRIPT_ENGINE_GP));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x1c;
            let alternate_character = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = c.lw(global(c, SCRIPT_ENGINE_GP));
            if alternate_character {
                c.cycles += 1;
                c.g[S0][0] = 10;
                c.cycles += 1;
                c.g[A0][0] = c.lw(global(c, SCRIPT_ENGINE_GP));
            }
        }
    } else {
        c.cycles += 1;
        c.g[A0][0] = c.lw(global(c, SCRIPT_ENGINE_GP));
    }
    c.cycles += 8;
    c.g[A2][0] = c.g[S0][0];
    c.g[T1][0] = c.lw(r(c, SP));
    c.g[A1][0] = 0;
    c.g[T2][0] = c.lw(at(c, SP, 4));
    c.g[A3][0] = 1;
    c.g[T3][0] = c.lw(at(c, SP, 8));
    c.g[T0][0] = 0;
    call_ptr(c, 0x0027_8e50, 0x0027_ae84)
}

/// Copy the 100-byte TrackInfo, retaining the original two alignment paths.
fn copy_track_info(c: &mut Cpu) {
    let unaligned = c.g[V0][0] != 0;
    c.cycles += 2;
    c.g[S4][0] = (c.g[S6][0] < 1) as u64;
    c.g[V0][0] = sx(at(c, V1, 0x60));
    loop {
        if unaligned {
            c.cycles += 19;
            for (reg, offset) in [(T1, 0), (A0, 8), (A1, 0x10), (A3, 0x18)] {
                c.ldl(reg, at(c, V1, offset + 7));
                c.ldr(reg, at(c, V1, offset));
            }
            for (reg, offset) in [(T1, 0), (A0, 8), (A1, 0x10), (A3, 0x18)] {
                c.sdl(reg, at(c, A2, offset + 7));
                c.sdr(reg, at(c, A2, offset));
            }
        } else {
            c.cycles += 11;
            for (reg, offset) in [(T0, 0), (T1, 8), (A0, 0x10), (A1, 0x18)] {
                c.g[reg][0] = c.ld(at(c, V1, offset));
            }
            for (reg, offset) in [(T0, 0), (T1, 8), (A0, 0x10), (A1, 0x18)] {
                c.sd(at(c, A2, offset), c.g[reg][0]);
            }
        }
        c.g[V1][0] = sx(at(c, V1, 0x20));
        let more = c.g[V1][0] != c.g[V0][0];
        c.g[A2][0] = sx(at(c, A2, 0x20));
        if !more {
            if unaligned {
                c.cycles += 2;
            }
            return;
        }
        if event_due(c) {
            deliver_event_here(c, if unaligned { 0x0027_aedc } else { 0x0027_af40 });
        }
    }
}

/// Queue first-time presentation choices and mark their profile flag. The
/// additional-course guard excludes these persistent writes as well.
fn queue_first_time(c: &mut Cpu, frame: Frame) -> Result<Restore, crate::call::Unwound> {
    c.cycles += 12;
    call_ptr(c, 0x0014_dc80, 0x0027_aeac)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    call_ptr(c, 0x0014_dd58, 0x0027_aeb8)?;
    c.g[A0][0] = c.lw(at(c, SP, 0x90));
    c.g[S3][0] = c.g[V0][0];
    call_ptr(c, 0x0014_4bc0, 0x0027_aec4)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = c.g[V1][0] & 7;
    c.g[A2][0] = sx(at(c, SP, 0x20));
    copy_track_info(c);
    c.cycles += 18;
    c.g[A3][0] = c.lw(r(c, V1));
    c.g[A0][0] = c.g[S3][0];
    c.sw(r(c, A2), c.g[A3][0]);
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_6e98, 0x0027_af80)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A2][0] = c.lw(at(c, SP, 0x74));
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S4][0];
    call_ptr(c, 0x0014_6150, 0x0027_af98)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0014_a0b0, 0x0027_afa4)?;
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(10));
    c.g[V0][0] = (c.g[V0][0] < 11) as u64;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x130);
        return Ok(Restore::S1);
    }

    c.cycles += 11;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0014_7398, 0x0027_afc0)?;
    c.g[A3][0] = c.lw(at(c, SP, 0x74));
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[T0][0] = c.g[S4][0];
    call_ptr(c, 0x0014_64d0, 0x0027_afd8)?;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x130);
        return Ok(Restore::S1);
    }
    c.cycles += 2;
    let already_seen = c.g[S0][0] != 0;
    frame.restore(c, S0, 0x130);
    if already_seen {
        return Ok(Restore::S1);
    }

    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c, S2, 0x80));
    let have_computer = (c.g[V0][0] as i64) > 0;
    c.g[S0][0] = u64::MAX;
    let have_other_rider;
    if have_computer {
        c.cycles += 2;
        c.g[V0][0] = c.lw(at(c, S2, 0x48));
        have_other_rider = true;
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw(at(c, S2, 0x84));
        have_other_rider = (c.g[V0][0] as i64) > 0;
        if !have_other_rider {
            c.g[A0][0] = c.lw(global(c, SCRIPT_ENGINE_GP));
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.lw(at(c, S2, 0x5c));
        }
    }
    if have_other_rider {
        c.cycles += 9;
        c.g[A0][0] = c.lw(at(c, V0, 0x18));
        c.g[V1][0] = c.lw(at(c, A0, 0x6c0));
        c.g[A0][0] = sx(at(c, A0, 0x6c0));
        rider_call(c, 0x38, 0x0027_b028)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lw(global(c, SCRIPT_ENGINE_GP));
    }
    c.cycles += 23;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x18;
    c.g[A3][0] = 1;
    c.g[T0][0] = 0;
    c.g[T1][0] = c.g[S0][0];
    c.g[T2][0] = u64::MAX;
    c.g[T3][0] = u64::MAX;
    call_ptr(c, 0x0027_8e50, 0x0027_b050)?;
    c.g[A0][0] = c.lw(global(c, SCRIPT_ENGINE_GP));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x19;
    c.g[A3][0] = 3;
    c.g[T0][0] = 0;
    c.g[T1][0] = c.g[S0][0];
    c.g[T2][0] = u64::MAX;
    c.g[T3][0] = u64::MAX;
    call_ptr(c, 0x0027_8e50, 0x0027_b074)?;
    c.g[A2][0] = c.lw(at(c, SP, 0x74));
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S4][0];
    c.g[T0][0] = 1;
    call_ptr(c, 0x0014_6320, 0x0027_b08c)?;
    Ok(Restore::S0)
}

/// `void ScriptEngine_QueueRaceCutscene(ScriptEngine* engine)` (0x27ac60).
///
/// Select the podium and first-time character presentations using native
/// placings, character choices and profile flags. Additional courses have
/// no presentation paths for the template world, so return without queueing
/// choices or changing the template's first-time profile flags. The caller
/// does not consume a return value; the native function is void.
pub fn queue_race_cutscene(c: &mut Cpu) {
    if crate::custom_tracks::active().is_some() {
        crate::abi::ret(c);
        return;
    }
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x140);
        for (reg, offset) in [
            (S6, 0xd0),
            (S0, 0x130),
            (S1, 0x120),
            (S2, 0x110),
            (S3, 0x100),
            (S4, 0xf0),
            (S5, 0xe0),
            (S7, 0xc0),
            (FP, 0xb0),
        ] {
            frame.save(c, reg, offset);
        }
        frame.save_ra(c, 0xa0);
        call_ptr(c, 0x0014_dc80, 0x0027_ac94)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0027_aca0)?;
        c.sw(at(c, SP, 0x90), c.g[V0][0]);
        c.g[V0][0] = 0x0053_0000;
        c.g[A2][0] = sx(CURRENT_RACE);
        c.g[S6][0] = c.lb(at(c, A2, 0x48));
        c.g[V0][0] = 4;
        let free_ride = c.g[S6][0] == c.g[V0][0];
        c.g[A0][0] = c.lw(global(c, APP_GP));
        let restore = 'body: {
            if free_ride {
                break 'body Restore::S0;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw(at(c, A0, 0xc0));
            c.g[V1][0] = c.lw(at(c, V0, 0x98));
            let presentation_disabled = c.g[V1][0] == 0;
            c.g[V0][0] = 1;
            if presentation_disabled {
                break 'body Restore::S0;
            }
            c.cycles += 5;
            c.g[A1][0] = c.lw(at(c, A0, 0x84));
            c.g[V1][0] = c.lw(at(c, A1, 0x84));
            c.g[A0][0] = c.lw(at(c, V1, 0x10));
            let split_screen = c.g[A0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x130);
            if split_screen {
                break 'body Restore::S1;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lb(at(c, A2, 0x49));
            c.g[S4][0] = 0;
            let flagged_event = c.g[V0][0] != 0;
            c.g[S2][0] = c.lw(at(c, A1, 0xc));
            if !flagged_event {
                find_human_podium(c)?;
            }
            c.cycles += 2;
            let have_podium = c.g[S4][0] != 0;
            c.g[V0][0] = 0x0053_0000;
            if have_podium {
                queue_podium(c)?;
            }
            c.cycles += 5;
            c.g[T0][0] = 0x0053_0000;
            c.g[V0][0] = sx(CURRENT_RACE);
            c.g[V1][0] = c.lb(at(c, V0, 0x49));
            let flagged_event = c.g[V1][0] != 0;
            frame.restore(c, S0, 0x130);
            if flagged_event {
                break 'body Restore::S1;
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[S6][0] < 4) as u64;
            if c.g[V0][0] == 0 {
                frame.restore(c, S1, 0x120);
                break 'body Restore::Rest;
            }
            queue_first_time(c, frame)?
        };
        if matches!(restore, Restore::S0) {
            c.cycles += 1;
            frame.restore(c, S0, 0x130);
        }
        if !matches!(restore, Restore::Rest) {
            c.cycles += 1;
            frame.restore(c, S1, 0x120);
        }
        c.cycles += 10;
        for (reg, offset) in [
            (S2, 0x110),
            (S3, 0x100),
            (S4, 0xf0),
            (S5, 0xe0),
            (S6, 0xd0),
            (S7, 0xc0),
            (FP, 0xb0),
        ] {
            frame.restore(c, reg, offset);
        }
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    });
}
