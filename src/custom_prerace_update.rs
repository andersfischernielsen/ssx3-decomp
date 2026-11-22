//! Native pre-race rider loading and intro-list synchronization.
//!
//! The original event path is retained. Custom courses can leave the script
//! lists empty and still use the native rider setup, start-grid placement,
//! round card and countdown that follow this state.

use crate::call::{call_ptr, guest, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0023_43b0, update, "GameStatePreRaceLoad_Update")];

const STEP: u32 = 0x1c;
const PENDING_SPECIAL_RIDERS: u32 = 0x10;
const INTRO_OPTION: u32 = 0x14;
const COMPUTER_COUNT: u32 = 0x80;
const ACTOR_COUNT: u32 = 0x84;
const COMPUTERS_ACTIVE: u32 = 0x14;
const ACTORS_ACTIVE: u32 = 0x8c;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}

#[inline]
fn global(c: &Cpu, negative_offset: u32) -> u32 {
    r(c, GP).wrapping_sub(negative_offset)
}

/// Show the loading indicator before step 3, then remove it. The script
/// list is held in state 4 while riders load and released in state 3 after.
fn loading_indicator(c: &mut Cpu, before_riders_ready: bool) -> Flow {
    if before_riders_ready {
        c.cycles += 10;
        c.g[A1][0] = 2;
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[A0][0] = c.lw(at(c, V1, 0x94));
        call_ptr(c, 0x001f_36c0, 0x0023_43e4)?;
        c.g[A0][0] = c.lw(global(c, 0x84c));
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_9220, 0x0023_43f0)?;
        c.g[V1][0] = 4;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 6;
            c.g[A0][0] = c.lw(global(c, 0x84c));
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            call_ptr(c, 0x0027_9040, 0x0023_440c)?;
        }
        c.g[V1][0] = c.lw(at(c, S1, STEP));
    } else {
        c.cycles += 6;
        c.g[A0][0] = c.lw(global(c, 0x84c));
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_9220, 0x0023_4420)?;
        c.g[V1][0] = 3;
        let release_list = c.g[V0][0] == c.g[V1][0];
        c.g[V1][0] = c.lw(global(c, 0x848));
        if release_list {
            c.cycles += 5;
            c.g[A0][0] = c.lw(global(c, 0x84c));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0027_9040, 0x0023_443c)?;
            c.g[V1][0] = c.lw(global(c, 0x848));
        }
        c.cycles += 5;
        c.g[A1][0] = 2;
        c.g[V0][0] = c.lw(at(c, V1, 0x84));
        c.g[A0][0] = c.lw(at(c, V0, 0x94));
        call_ptr(c, 0x001f_36d8, 0x0023_4450)?;
        c.g[V1][0] = c.lw(at(c, S1, STEP));
    }
    Ok(())
}

/// Configure the native computer and actor populations for the selected
/// event. S0 retains its mode across the guest setup calls.
fn setup_riders(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    call_ptr(c, 0x0014_dc80, 0x0023_449c)?;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x0023_44a8)?;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0023_4910, 0x0023_44b0)?;
    c.g[V0][0] = 0x0053_0000;
    c.g[A2][0] = 4;
    c.g[V0][0] = sx(r(c, V0).wrapping_add(0x5bc8));
    c.g[S0][0] = c.lb(at(c, V0, 0x48));
    c.g[V1][0] = c.lb(at(c, V0, 0x4a));
    let free_ride = c.g[S0][0] == c.g[A2][0];
    c.g[A1][0] = c.lb(at(c, V0, 0x49));

    let mut special_riders = free_ride;
    if !free_ride {
        c.cycles += 3;
        c.g[A0][0] = 5;
        let time_challenge = c.g[S0][0] == c.g[A0][0];
        c.g[V0][0] = 6;
        if time_challenge {
            c.cycles += 2;
            let ordinary_time_challenge = c.g[V1][0] != c.g[A2][0];
            c.g[V0][0] = c.lw(global(c, 0x488));
            if ordinary_time_challenge {
                special_riders = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 6;
            }
        }
        if !special_riders {
            c.cycles += 2;
            if c.g[S0][0] == c.g[V0][0] {
                c.cycles += 2;
                special_riders = c.g[V1][0] != c.g[A0][0];
                if special_riders {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(global(c, 0x488));
                }
            }
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = c.lw(global(c, 0x488));
    }

    if special_riders {
        c.cycles += 8;
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        call_ptr(c, 0x0012_ab20, 0x0023_4500)?;
        c.g[V0][0] = c.lw(global(c, 0x488));
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        call_ptr(c, 0x0012_ac48, 0x0023_450c)?;
        c.g[V0][0] = 1;
        c.sw(at(c, S1, PENDING_SPECIAL_RIDERS), c.g[V0][0]);
        c.cycles += 1;
        c.g[V0][0] = c.lw(global(c, 0x488));
    } else {
        c.cycles += 2;
        let skip_computers = c.g[A1][0] != 0;
        c.g[V0][0] = c.lw(global(c, 0x488));
        if !skip_computers {
            c.cycles += 7;
            c.g[A0][0] = c.lw(at(c, V0, 0xc));
            call_ptr(c, 0x0012_8958, 0x0023_4528)?;
            c.g[V0][0] = c.lw(global(c, 0x488));
            c.g[A0][0] = c.lw(at(c, V0, 0xc));
            c.g[V1][0] = c.lw(at(c, A0, COMPUTER_COUNT));
            if c.g[V1][0] != 0 {
                c.g[V1][0] = c.lw(at(c, V0, 0xc));
            } else {
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.cycles += 4;
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0012_8998, 0x0023_454c)?;
                    c.g[V0][0] = c.lw(global(c, 0x488));
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0012_8998, 0x0023_455c)?;
                    c.g[V0][0] = c.lw(global(c, 0x488));
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw(at(c, V0, 0xc));
            }
            c.cycles += 1;
            c.sw(at(c, V1, ACTORS_ACTIVE), 0);
            c.cycles += 1;
            c.g[V0][0] = c.lw(global(c, 0x488));
        }
    }

    c.cycles += 6;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.lw(at(c, V0, 0xc));
    call_ptr(c, 0x0012_8a48, 0x0023_4578)?;
    c.g[V0][0] = 5;
    let enable_race_riders = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = c.lw(global(c, 0x488));
    if enable_race_riders {
        c.cycles += 5;
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        call_ptr(c, 0x0012_8a48, 0x0023_459c)?;
        c.g[V0][0] = 2;
    } else {
        c.cycles += 2;
        let race = c.g[S0][0] == 0;
        c.g[V0][0] = 4;
        if race {
            c.cycles += 1;
            c.g[V0][0] = c.lw(global(c, 0x488));
            c.cycles += 5;
            c.g[A1][0] = 1;
            c.g[A0][0] = c.lw(at(c, V0, 0xc));
            call_ptr(c, 0x0012_8a48, 0x0023_459c)?;
            c.g[V0][0] = 2;
        } else {
            c.cycles += 2;
            let keep_only_humans = c.g[S0][0] == c.g[V0][0];
            c.g[V0][0] = c.lw(global(c, 0x488));
            if !keep_only_humans {
                c.cycles += 3;
                c.g[A1][0] = 2;
                c.g[A0][0] = c.lw(at(c, V0, 0xc));
                call_ptr(c, 0x0012_8a48, 0x0023_45b8)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 2;
        }
    }
    c.cycles += 2;
    c.sw(at(c, S1, STEP), c.g[V0][0]);
    Ok(())
}

/// Wait for the intro and actor queues, place the native riders on their
/// start grid, and advance to the round card. The bool tracks the original
/// branch-delay reload of S0, which is also repeated on some completion paths.
fn finish_loading(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    c.cycles += 5;
    c.g[A0][0] = c.lw(global(c, 0x84c));
    c.g[A1][0] = 1;
    call_ptr(c, 0x0027_9220, 0x0023_4600)?;
    let intro_busy = c.g[V0][0] != 0;
    frame.restore(c, S0, 0x20);
    if intro_busy {
        return Ok(true);
    }
    c.cycles += 5;
    c.g[V0][0] = c.lw(global(c, 0x488));
    c.g[A0][0] = c.lw(at(c, V0, 0xc));
    c.g[V1][0] = c.lw(at(c, A0, ACTOR_COUNT));
    let have_actors = (c.g[V1][0] as i64) > 0;
    c.g[V0][0] = 1;
    if have_actors {
        c.cycles += 1;
        c.sw(at(c, A0, ACTORS_ACTIVE), c.g[V0][0]);
    }
    c.cycles += 5;
    c.g[V0][0] = c.lw(global(c, 0x488));
    c.g[A0][0] = c.lw(at(c, V0, 0xc));
    c.g[V1][0] = c.lw(at(c, A0, COMPUTER_COUNT));
    let have_computers = (c.g[V1][0] as i64) > 0;
    c.g[V0][0] = 1;
    if have_computers {
        c.cycles += 1;
        c.sw(at(c, A0, COMPUTERS_ACTIVE), c.g[V0][0]);
    }
    c.cycles += 6;
    c.g[A0][0] = c.lw(global(c, 0x84c));
    c.g[A1][0] = 0;
    call_ptr(c, 0x0027_91d8, 0x0023_4644)?;
    c.g[V1][0] = 2;
    let start_list = c.g[V0][0] == c.g[V1][0];
    c.g[A0][0] = c.lw(global(c, 0x84c));
    if start_list {
        c.cycles += 7;
        c.g[A2][0] = c.lw(at(c, S1, INTRO_OPTION));
        c.g[A1][0] = 0;
        c.g[A3][0] = 1;
        c.g[A2][0] = (c.g[A2][0] < 1) as u64;
        call_ptr(c, 0x0027_8f68, 0x0023_4664)?;
        frame.restore(c, S0, 0x20);
        return Ok(true);
    }
    c.cycles += 9;
    c.g[A1][0] = 0;
    call_ptr(c, 0x0027_9220, 0x0023_4674)?;
    c.g[A0][0] = c.lw(global(c, 0x84c));
    c.g[S0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    call_ptr(c, 0x0027_9148, 0x0023_4688)?;
    let list_inactive = c.g[S0][0] == 0;
    c.g[V1][0] = 0;
    if !list_inactive {
        c.cycles += 2;
        c.g[V0][0] ^= 5;
        c.g[V1][0] = (0 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    let list_busy = c.g[V1][0] != 0;
    frame.restore(c, S0, 0x20);
    if list_busy {
        return Ok(true);
    }
    c.cycles += 10;
    call_ptr(c, 0x0014_dc80, 0x0023_46a8)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0023_46b4)?;
    c.g[V1][0] = 0x0053_0000;
    c.g[A0][0] = 4;
    c.g[V0][0] = c.lb(at(c, V1, 0x5c10));
    let free_ride = c.g[V0][0] == c.g[A0][0];
    c.g[V0][0] = c.lw(global(c, 0x488));
    if !free_ride {
        c.cycles += 7;
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        call_ptr(c, 0x0012_89f0, 0x0023_46d0)?;
        c.g[V0][0] = c.lw(global(c, 0x488));
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        c.g[V1][0] = c.lw(at(c, A0, COMPUTER_COUNT));
        let have_computers = (c.g[V1][0] as i64) > 0;
        c.g[V0][0] = 1;
        if have_computers {
            c.cycles += 1;
            c.sw(at(c, A0, COMPUTERS_ACTIVE), c.g[V0][0]);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c, S1, PENDING_SPECIAL_RIDERS));
    let special_riders_pending = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(global(c, 0x488));
    if special_riders_pending {
        c.cycles += 4;
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        call_ptr(c, 0x0012_abd0, 0x0023_46fc)?;
        let first_ready = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(global(c, 0x488));
        if !first_ready {
            return Ok(false);
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        call_ptr(c, 0x0012_acf8, 0x0023_470c)?;
        let second_ready = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(global(c, 0x488));
        if !second_ready {
            return Ok(false);
        }
        c.cycles += 6;
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        call_ptr(c, 0x0012_b030, 0x0023_471c)?;
        c.g[V0][0] = c.lw(global(c, 0x488));
        c.g[A0][0] = c.lw(at(c, V0, 0xc));
        call_ptr(c, 0x0012_b000, 0x0023_4728)?;
        c.sw(at(c, S1, PENDING_SPECIAL_RIDERS), 0);
    }
    c.cycles += 4;
    c.g[V0][0] = 4;
    c.g[A0][0] = c.g[S1][0];
    c.sw(at(c, S1, STEP), c.g[V0][0]);
    call_ptr(c, 0x0023_3aa0, 0x0023_473c)?;
    Ok(false)
}

/// `void GameStatePreRaceLoad_Update(GameState* self)` (0x2343b0).
/// Steps 0..3 wait for script loading, create riders, load them one step per
/// tick, then synchronize the intro and place the field before advancing.
pub fn update(c: &mut Cpu) {
    guest(c, |c| loop {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(at(c, S1, STEP));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
        let before_riders_ready = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(global(c, 0x848));
        loading_indicator(c, before_riders_ready)?;

        c.cycles += 3;
        c.g[V0][0] = (c.g[V1][0] < 5) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x0048_0000;
        let mut s0_restored = false;
        if in_range {
            c.cycles += 6;
            c.g[V1][0] = sx(r(c, V1) << 2);
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x3fe0));
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
            c.g[A0][0] = c.lw(r(c, V1));
            match r(c, A0) {
                0x0023_43b0 => {
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_43b0);
                    }
                    continue;
                }
                0x0023_4478 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(global(c, 0x84c));
                    call_ptr(c, 0x0027_9298, 0x0023_4480)?;
                    let scripts_busy = c.g[V0][0] != 0;
                    frame.restore(c, S0, 0x20);
                    s0_restored = true;
                    if !scripts_busy {
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        c.sw(at(c, S1, STEP), c.g[V0][0]);
                    }
                }
                0x0023_4494 => setup_riders(c)?,
                0x0023_45c4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(global(c, 0x488));
                    c.g[A0][0] = c.lw(at(c, V0, 0xc));
                    call_ptr(c, 0x0012_a180, 0x0023_45d0)?;
                    let riders_loaded = c.g[V0][0] != 0;
                    c.g[V0][0] = 3;
                    if riders_loaded {
                        c.cycles += 2;
                        c.sw(at(c, S1, STEP), c.g[V0][0]);
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(global(c, 0x488));
                        c.g[A0][0] = c.lw(at(c, V0, 0xc));
                        call_ptr(c, 0x0012_96f8, 0x0023_45e4)?;
                        frame.restore(c, S0, 0x20);
                        s0_restored = true;
                    }
                }
                0x0023_45f4 => s0_restored = finish_loading(c, frame)?,
                0x0023_473c => {}
                target => {
                    c.jump_out(target, 0x0023_43b0, 0x0023_4750);
                    return Ok(());
                }
            }
        }
        if !s0_restored {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    })
}
