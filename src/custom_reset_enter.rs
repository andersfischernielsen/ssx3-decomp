//! Native reset-state setup and restart of existing world presentation lists.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0023_55c0, enter, "GameStateReset_Enter")];

const SCRIPT_ENGINE_GP: u32 = 0x84c;
const APP_GP: u32 = 0x848;
const RACE_MANAGER_GP: u32 = 0x488;
const WSCRIPT_MAN_GP: u32 = 0xce8;
const CURRENT_RACE: u32 = 0x0053_5bc8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn gp(c: &Cpu, negative: u32) -> u32 {
    r(c, GP).wrapping_sub(negative)
}

/// `Enter(GameStateReset* self)` (0x2355c0).
///
/// Reset the world-script and BE state, choose the following game state,
/// and resume prepared presentation lists when requested. This function
/// does not enqueue an intro or place riders on the starting grid.
pub fn enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.sw(r(c, S1).wrapping_add(0x1c), 0);
        c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
        call_ptr(c, 0x0027_aa80, 0x0023_55f0)?;
        let busy = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if busy {
            c.cycles += 1;
            c.sw(r(c, S1).wrapping_add(0x1c), c.g[V0][0]);
        }

        c.cycles += 17;
        call_ptr(c, 0x0014_dc80, 0x0023_5604)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_5610)?;
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_5618)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0023_5624)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_8e30, 0x0023_562c)?;
        c.g[V0][0] = 0x0053_0000;
        c.g[V0][0] = sx(CURRENT_RACE);
        c.g[S3][0] = c.lb(r(c, V0).wrapping_add(0x49));
        let flagged = c.g[S3][0] != 0;
        c.g[S4][0] = c.lb(r(c, V0).wrapping_add(0x48));

        let mut pre_race_load = flagged;
        if flagged {
            c.cycles += 1;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 8;
            c.g[V0][0] = c.lw(gp(c, APP_GP));
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0x84));
            c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0x78));
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(0x1bc));
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(0xe));
            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
            pre_race_load = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !pre_race_load {
                c.cycles += 4;
                c.g[V0][0] = 4;
                c.sw(r(c, S1).wrapping_add(4), 0);
                c.sw(r(c, S1), c.g[V0][0]);
            }
        }
        if pre_race_load {
            c.cycles += 4;
            c.g[V1][0] = (c.g[S3][0] < 1) as u64;
            c.sw(r(c, S1), c.g[V0][0]);
            c.sw(r(c, S1).wrapping_add(4), c.g[V1][0]);
            c.g[V0][0] = 4;
        }

        c.cycles += 2;
        if c.g[S4][0] == c.g[V0][0] {
            c.cycles += 5;
            c.g[V0][0] = c.lw(gp(c, RACE_MANAGER_GP));
            c.g[A1][0] = 3;
            c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0xc));
            call_ptr(c, 0x0011_3b10, 0x0023_569c)?;
        }
        c.g[A0][0] = 0;

        c.cycles += 13;
        c.g[S5][0] = 1;
        call_ptr(c, 0x0014_de68, 0x0023_56a8)?;
        c.g[A0][0] = c.lw(r(c, GP).wrapping_add(WSCRIPT_MAN_GP));
        call_ptr(c, 0x0030_9030, 0x0023_56b0)?;
        c.g[A0][0] = c.lw(r(c, GP).wrapping_add(WSCRIPT_MAN_GP));
        call_ptr(c, 0x0030_9f18, 0x0023_56b8)?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_4be0, 0x0023_56c0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_4cc0, 0x0023_56cc)?;
        let one_rider = c.g[V0][0] == c.g[S5][0];
        c.g[V0][0] = c.lw(gp(c, RACE_MANAGER_GP));
        if one_rider {
            c.cycles += 18;
            call_ptr(c, 0x0014_dc80, 0x0023_56dc)?;
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x0023_56e8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0014_4be0, 0x0023_56f4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_6e10, 0x0023_5704)?;
            c.g[V0][0] = c.lw(r(c, S0).wrapping_add(0xc));
            c.g[A0][0] = c.lh(r(c, V0).wrapping_add(8));
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0xc));
            let function = r(c, V1);
            c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
            call_ptr(c, function, 0x0023_5718)?;
            c.g[V0][0] = c.lw(gp(c, RACE_MANAGER_GP));
        }

        c.cycles += 11;
        c.g[V1][0] = c.g[S4][0] ^ 4;
        c.g[A2][0] = c.lw(gp(c, APP_GP));
        c.g[V1][0] = (0 < c.g[V1][0]) as u64;
        c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0xc));
        c.g[A1][0] = 0;
        c.sw(r(c, A0).wrapping_add(0x14), c.g[V1][0]);
        c.g[V0][0] = c.lw(r(c, A2).wrapping_add(0x84));
        c.g[A0][0] = c.lw(r(c, V0).wrapping_add(0x94));
        call_ptr(c, 0x001f_36d8, 0x0023_5740)?;

        if c.g[S3][0] != 0 {
            c.cycles += 7;
            c.sw(r(c, S1), c.g[S5][0]);
            c.sw(r(c, S1).wrapping_add(4), 0);
            c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(0x550));
            c.g[V0][0] ^= 2;
            let inactive = c.g[V0][0] != 0;
            c.g[A1][0] = 1;
            if !inactive {
                c.cycles += 8;
                c.g[A2][0] = 0;
                c.g[A3][0] = 1;
                call_ptr(c, 0x0027_8f68, 0x0023_5770)?;
                c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
                c.g[V0][0] = c.lw(r(c, A0).wrapping_add(0x550));
                c.g[V0][0] ^= 2;
                let inactive = c.g[V0][0] != 0;
                c.g[A1][0] = 0;
                if !inactive {
                    c.cycles += 5;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = 1;
                    call_ptr(c, 0x0027_8f68, 0x0023_5790)?;
                }
            }
        } else {
            c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0x14));
            c.cycles += 2;
            let no_intro = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
            if !no_intro {
                c.cycles += 4;
                c.g[A1][0] = 1;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0027_8f68, 0x0023_57b0)?;
            }
        }

        c.cycles += 17;
        call_ptr(c, 0x0028_b180, 0x0023_57b8)?;
        c.g[A0][0] = sx(r(c, V0).wrapping_add(0x118));
        call_ptr(c, 0x002b_3a98, 0x0023_57c0)?;
        call_ptr(c, 0x0028_b180, 0x0023_57c8)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002a_4b68, 0x0023_57d0)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
