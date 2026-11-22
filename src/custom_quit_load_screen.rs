//! Native transition from the loaded world to a front-end or game load screen.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0023_0488,
    quit_to_load_screen,
    "GameModule_QuitToLoadScreen",
)];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `QuitToLoadScreen(GameModule* self, bool load_game)` (0x230488).
/// Queue world state 6, leave any network race, then install the app's next
/// loading screen: zero selects PreFELoad, nonzero selects PreGameLoad.
/// The world is still alive here; disposal follows through the queued state
/// and app transition rather than an immediate free in this function.
pub fn quit_to_load_screen(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.lw(r(c, S0).wrapping_add(0x28));
        call_ptr(c, 0x0027_06f0, 0x0023_04a8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 6;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0023_1250, 0x0023_04bc)?;

        c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x204));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0025_8ae0, 0x0023_04d0)?;
        }
        c.g[A0][0] = c.lw(r(c, GP).wrapping_add(0x304));
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, 0x0026_6df8, 0x0023_04e4)?;
        }

        c.cycles += 2;
        c.g[A2][0] = 0x100;
        if c.g[S1][0] != 0 {
            c.cycles += 12;
            c.g[A1][0] = 0x0048_0000;
            c.g[A0][0] = 8;
            c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x4890));
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0023_0500)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0023_2720, 0x0023_0508)?;
            c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_6ef8, 0x0023_0514)?;
        } else {
            c.cycles += 11;
            c.g[A1][0] = 0x0048_0000;
            c.g[A0][0] = 8;
            c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x4878));
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0023_0530)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0023_2738, 0x0023_0538)?;
            c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0031_6ef8, 0x0023_0544)?;
        }

        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
