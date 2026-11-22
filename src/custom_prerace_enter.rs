//! Native pre-race rider setup and optional presentation-list preparation.

use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0023_41d0, enter, "GameStatePreRaceLoad_Enter")];

const RACE_MANAGER_GP: u32 = 0x488;
const APP_GP: u32 = 0x848;
const SCRIPT_ENGINE_GP: u32 = 0x84c;
const LOAD_STEP: u32 = 0x1c;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn gp(c: &Cpu, negative: u32) -> u32 {
    r(c, GP).wrapping_sub(negative)
}

/// Notify the BE interface through its native slot 1. Both race and player
/// setup use the same slot layout, with a different scratch target register.
fn apply_interface(c: &mut Cpu, owner: usize, vtable: usize, target: usize, back: u32) -> Flow {
    c.g[vtable][0] = c.lw(r(c, owner).wrapping_add(0xc));
    c.g[A0][0] = c.lh(r(c, vtable).wrapping_add(8));
    c.g[target][0] = c.lw(r(c, vtable).wrapping_add(0xc));
    let function = r(c, target);
    c.g[A0][0] = sx(r(c, owner).wrapping_add(r(c, A0)));
    call_ptr(c, function, back)
}

/// `Enter(self, mode)` (0x2341d0): set AI/BE pre-race state, reset replay,
/// copy the configured computer riders into BE, and prepare the optional
/// intro list. Mode 1 uses the normal intro unless the event is free ride;
/// mode 2 uses the restart list. Other modes prepare no script list here.
/// Rider placement is left to the subsequent load/update path.
pub fn enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 55;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S0, 0x80);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        call_ptr(c, 0x0014_dc80, 0x0023_4208)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_4214)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_421c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_4228)?;
        c.g[S5][0] = c.g[V0][0];

        c.g[A1][0] = 3;
        c.g[V0][0] = c.lw(gp(c, RACE_MANAGER_GP));
        c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0xc));
        call_ptr(c, 0x0011_3b10, 0x0023_423c)?;
        c.g[V1][0] = c.lw(gp(c, APP_GP));
        c.g[A1][0] = 0;
        c.g[V0][0] = c.lw(r(c, V1).wrapping_add(0x84));
        c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x94));
        call_ptr(c, 0x001f_36d8, 0x0023_4250)?;
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_de68, 0x0023_4258)?;
        c.g[V0][0] = c.lw(gp(c, RACE_MANAGER_GP));
        c.g[V1][0] = c.lw(gp(c, APP_GP));
        c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x28));
        c.g[S1][0] = c.lw(r(c, V1).wrapping_add(0xc0));
        call_ptr(c, 0x0026_f228, 0x0023_426c)?;

        c.g[A2][0] = c.lw(gp(c, RACE_MANAGER_GP));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.lw(r(c, S1).wrapping_add(0x10));
        c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x14));
        c.g[V1][0] = c.lw(r(c, A2).wrapping_add(0xc));
        c.g[S3][0] = sx(r(c, A1).wrapping_sub(r(c, V0)));
        c.g[S4][0] = c.lw(r(c, V1).wrapping_add(0x7c));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0014_4d98, 0x0023_4290)?;
        apply_interface(c, S0, V1, V0, 0x0023_42a4)?;

        let no_computers = (c.g[S3][0] as i64) <= 0;
        c.g[V1][0] = sx(r(c, S1).wrapping_add(0x40));
        if !no_computers {
            c.cycles += 3;
            c.g[V0][0] = sx(r(c, S4) << 2);
            c.g[S1][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
            c.g[S0][0] = sx(r(c, S2).wrapping_add(r(c, S4)));
            loop {
                c.cycles += 13;
                c.g[A2][0] = c.lw(r(c, S1).wrapping_sub(0x28));
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0014_7338, 0x0023_42c8)?;
                c.g[S2][0] = sx(r(c, S2).wrapping_add(1));
                c.g[A2][0] = c.lw(r(c, S1));
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[S1][0] = sx(r(c, S1).wrapping_add(4));
                call_ptr(c, 0x0014_73d0, 0x0023_42e0)?;
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r(c, S2).wrapping_add(r(c, S4)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_42b8);
                }
            }
        }

        c.cycles += 8;
        apply_interface(c, S5, V0, V1, 0x0023_4300)?;
        c.g[V0][0] = 1;
        let normal_intro = c.g[S6][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[S6][0] as i64) < 2) as u64;
        if normal_intro {
            c.cycles += 5;
            c.g[V1][0] = 0x0053_0000;
            c.g[V0][0] = c.lb(r(c, V1).wrapping_add(0x5c10));
            c.g[V0][0] ^= 4;
            let free_ride = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
            if !free_ride {
                c.cycles += 8;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0027_90a0, 0x0023_4368)?;
                c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0027_aaf8, 0x0023_4374)?;
                c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0027_8f38, 0x0023_4380)?;
            }
            c.cycles += 1;
            c.sw(r(c, S7).wrapping_add(LOAD_STEP), 0);
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 2;
                if c.g[S6][0] == c.g[V0][0] {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0027_90a0, 0x0023_432c)?;
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x0027_aaf8, 0x0023_4338)?;
                    c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0027_8f38, 0x0023_4344)?;
                }
            }
            c.sw(r(c, S7).wrapping_add(LOAD_STEP), 0);
        }

        c.cycles += 11;
        for (register, offset) in [
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
        ] {
            frame.restore(c, register, offset);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
