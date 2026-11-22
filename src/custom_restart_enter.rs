//! Native restart entry: reset the race, restore its riders and presentation.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0023_5aa0, enter, "GameStateRestartRace_Enter")];

const APP_GP: u32 = 0x848;
const RACE_MANAGER_GP: u32 = 0x488;
const SCRIPT_ENGINE_GP: u32 = 0x84c;
const CURRENT_RACE_OFFSET: u32 = 0x5bc8;
const STEP: u32 = 0x10;
const LOAD_TICK: u32 = 0x14;
const FINISH_PRESENTATION: u32 = 0x18;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

#[inline]
fn gp(c: &Cpu, negative: u32) -> u32 {
    r(c, GP).wrapping_sub(negative)
}

/// Apply one BE interface through slot 1, retaining the native scratch
/// vtable and target registers at the call site.
fn apply_interface(c: &mut Cpu, owner: usize, table: usize, target: usize, back: u32) -> Flow {
    c.g[table][0] = c.lw(r(c, owner).wrapping_add(0xc));
    c.g[A0][0] = c.lh(r(c, table).wrapping_add(8));
    c.g[target][0] = c.lw(r(c, table).wrapping_add(0xc));
    let function = r(c, target);
    c.g[A0][0] = sx(r(c, owner).wrapping_add(r(c, A0)));
    call_ptr(c, function, back)
}

/// Restore the computer characters and boards already chosen by GameModeMan.
/// A2 initially carries the global race manager, S0/S5 the BE interfaces.
fn restore_rider_configuration(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.lw(r(c, S6).wrapping_add(0x10));
    c.g[S2][0] = 0;
    c.g[V0][0] = c.lw(r(c, S6).wrapping_add(0x14));
    c.g[V1][0] = c.lw(r(c, A2).wrapping_add(0xc));
    c.g[S3][0] = sx(r(c, A1).wrapping_sub(r(c, V0)));
    c.g[S4][0] = c.lw(r(c, V1).wrapping_add(0x7c));
    c.g[A1][0] = c.g[S3][0];
    call_ptr(c, 0x0014_4d98, 0x0023_5b88)?;
    apply_interface(c, S0, V1, V0, 0x0023_5b9c)?;
    let no_computers = (c.g[S3][0] as i64) <= 0;
    c.g[V1][0] = sx(r(c, S4) << 2);
    if !no_computers {
        c.cycles += 3;
        c.g[V0][0] = sx(r(c, S6).wrapping_add(0x40));
        c.g[S1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.g[S0][0] = sx(r(c, S2).wrapping_add(r(c, S4)));
        loop {
            c.cycles += 13;
            c.g[A2][0] = c.lw(r(c, S1).wrapping_sub(0x28));
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0014_7338, 0x0023_5bc0)?;
            c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
            c.g[A2][0] = c.lw(r(c, S1));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S5][0];
            c.g[S1][0] = sx(r(c, S1).wrapping_add(4));
            call_ptr(c, 0x0014_73d0, 0x0023_5bd8)?;
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r(c, S2).wrapping_add(r(c, S4)));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_5bb0);
            }
        }
    }

    c.cycles += 12;
    apply_interface(c, S5, V0, V1, 0x0023_5bf8)?;
    c.g[V0][0] = c.lw(gp(c, APP_GP));
    c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x84));
    c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0xc));
    call_ptr(c, 0x0012_ab20, 0x0023_5c08)?;
    c.g[V0][0] = ((c.g[S3][0] as i64) < 2) as u64;
    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[V0][0] = c.lw(gp(c, APP_GP));
        c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x84));
        c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0xc));
        call_ptr(c, 0x0012_ac48, 0x0023_5c24)?;
    }
    c.sw(r(c, S7).wrapping_add(FINISH_PRESENTATION), 0);
    Ok(())
}

/// Prepare native finish/next-heat presentation lists and restart loading.
/// The race-specific main list 1 is independent of QueueIntroScripts.
fn prepare_presentation(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = sx(r(c, FP).wrapping_add(CURRENT_RACE_OFFSET));
    c.g[V1][0] = c.lb(r(c, V0).wrapping_add(0x48));
    let ordinary_race = c.g[V1][0] == 0;
    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    if ordinary_race {
        c.cycles += 6;
        c.g[V0][0] = 1;
        c.sw(r(c, S7).wrapping_add(FINISH_PRESENTATION), c.g[V0][0]);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        call_ptr(c, 0x0027_a860, 0x0023_5c4c)?;
        c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    }

    c.cycles += 18;
    c.g[A1][0] = 0;
    call_ptr(c, 0x0027_90a0, 0x0023_5c58)?;
    c.g[V1][0] = c.lw(r(c, S6).wrapping_add(0x98));
    c.g[V0][0] = 2;
    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    c.g[A1][0] = 3;
    if c.g[V1][0] == 0 {
        c.g[A1][0] = c.g[V0][0];
    }
    call_ptr(c, 0x0027_aaf8, 0x0023_5c70)?;
    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
    c.g[A1][0] = 0;
    call_ptr(c, 0x0027_8f38, 0x0023_5c7c)?;
    c.g[V1][0] = c.lw(gp(c, APP_GP));
    c.g[A1][0] = 3;
    c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x84));
    c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x94));
    call_ptr(c, 0x001f_36c0, 0x0023_5c90)?;
    c.sw(r(c, S7).wrapping_add(STEP), 0);
    c.sw(r(c, S7).wrapping_add(LOAD_TICK), 0);
    Ok(())
}

/// `Enter(GameStateRestartRace* self)` (0x235aa0).
/// Reset the current native game-mode/race state, restore configured AI
/// riders, and prepare its presentation and loading sequence. Challenge
/// modes 5/6 and currentRace flag +0x49 instead start directly at step 3.
/// All start-grid/path placement is delegated to the normal race reset path.
pub fn enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S7, 0x20);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, FP, 0x10);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_de68, 0x0023_5ad8)?;
        c.g[A0][0] = c.lw(r(c, GP).wrapping_add(0xce8));
        call_ptr(c, 0x0030_b7f8, 0x0023_5ae0)?;
        call_ptr(c, 0x0014_dc80, 0x0023_5ae8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_5af4)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_5afc)?;
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0023_5b08)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(gp(c, APP_GP));
        c.g[S6][0] = c.lw(r(c, V0).wrapping_add(0xc0));
        c.g[A0][0] = c.g[S6][0];
        call_ptr(c, 0x0023_82d8, 0x0023_5b1c)?;
        c.g[V0][0] = c.lw(gp(c, APP_GP));
        c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x84));
        call_ptr(c, 0x0023_0180, 0x0023_5b28)?;
        c.g[V0][0] = 0x0053_0000;
        c.g[V1][0] = 5;
        c.g[A1][0] = sx(r(c, V0).wrapping_add(CURRENT_RACE_OFFSET));
        c.g[A0][0] = c.lb(r(c, A1).wrapping_add(0x48));
        let challenge_five = c.g[A0][0] == c.g[V1][0];
        c.g[FP][0] = c.g[V0][0];
        let mut skip_loading = challenge_five;
        if challenge_five {
            c.cycles += 1;
            c.g[V0][0] = 3;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 6;
            let challenge_six = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            skip_loading = challenge_six;
            if !challenge_six {
                c.cycles += 3;
                c.g[V0][0] = c.lb(r(c, A1).wrapping_add(0x49));
                let flagged = c.g[V0][0] != 0;
                c.g[A2][0] = c.lw(gp(c, RACE_MANAGER_GP));
                if flagged {
                    c.cycles += 1;
                    c.g[V0][0] = 3;
                    skip_loading = true;
                }
            }
        }
        if skip_loading {
            c.cycles += 2;
            c.sw(r(c, S7).wrapping_add(STEP), c.g[V0][0]);
        } else {
            restore_rider_configuration(c)?;
            prepare_presentation(c)?;
        }

        c.cycles += 12;
        for (reg, offset) in [
            (S0, 0x90),
            (S1, 0x80),
            (S2, 0x70),
            (S3, 0x60),
            (S4, 0x50),
            (S5, 0x40),
            (S6, 0x30),
            (S7, 0x20),
            (FP, 0x10),
        ] {
            frame.restore(c, reg, offset);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
