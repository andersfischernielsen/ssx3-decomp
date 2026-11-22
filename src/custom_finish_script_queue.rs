//! Native finish-presentation lists, excluded for additional-course worlds.

use crate::call::{call_ptr, guest, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0027_a860,
    queue_finish_scripts,
    "ScriptEngine_QueueFinishScripts",
)];

const ADD_CHOICE: u32 = 0x0027_8e50;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// All choices leave their three optional selector arguments unspecified.
fn add_choice(c: &mut Cpu, back: u32) -> Flow {
    c.g[T1][0] = u64::MAX;
    c.g[T2][0] = u64::MAX;
    c.g[T3][0] = u64::MAX;
    call_ptr(c, ADD_CHOICE, back)
}

/// Read the native human-player count through App -> GameModule -> AI.
/// The original leaves A1=1 and then restores A0 to the engine in S0.
fn one_player(c: &mut Cpu) -> bool {
    c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
    c.g[A1][0] = 1;
    c.g[V0][0] = c.lw(r(c, A0).wrapping_add(0x84));
    c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0xc));
    c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0x7c));
    let one = c.g[A0][0] == c.g[A1][0];
    c.g[A0][0] = c.g[S0][0];
    one
}

/// `void QueueFinishScripts(engine, online, include_intro)` (0x27a860).
///
/// Original courses queue the appropriate categories into native list 1,
/// prepare and start it, then send music event 20. Additional courses have
/// no matching finish cameras or presentation scripts in the template world,
/// so they keep this list empty and use the native result-screen fallback.
pub fn queue_finish_scripts(c: &mut Cpu) {
    if crate::custom_tracks::active().is_some() {
        crate::abi::ret(c);
        return;
    }

    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let online = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];

        if online {
            c.cycles += 2;
            let include_intro = c.g[A2][0] != 0;
            c.g[A1][0] = 1;
            if include_intro {
                c.cycles += 7;
                c.g[A2][0] = 0x12;
                c.g[A3][0] = 1;
                c.g[T0][0] = 1;
                add_choice(c, 0x0027_a898)?;
            }

            c.cycles += 16;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = 0x13;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            add_choice(c, 0x0027_a8bc)?;
            let one = one_player(c);

            c.cycles += 4;
            c.g[A2][0] = if one { 0x14 } else { 0x15 };
            c.g[A3][0] = 8;
            c.g[T0][0] = 0;
            c.cycles += 6;
            add_choice(c, 0x0027_a984)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            let include_intro = c.g[A2][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if include_intro {
                c.cycles += 8;
                c.g[A1][0] = 1;
                c.g[A2][0] = 0xc;
                c.g[A3][0] = 1;
                c.g[T0][0] = 1;
                add_choice(c, 0x0027_a920)?;
            }

            c.cycles += 18;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = 0xd;
            c.g[A3][0] = 0;
            c.g[T0][0] = 1;
            add_choice(c, 0x0027_a944)?;
            c.g[V1][0] = c.lw(r(c, GP).wrapping_sub(0x854));
            c.g[A1][0] = 1;
            c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
            c.sw(r(c, V1).wrapping_add(0xf44), c.g[A1][0]);
            c.g[V0][0] = c.lw(r(c, A0).wrapping_add(0x84));
            c.g[V1][0] = c.lw(r(c, V0).wrapping_add(0xc));
            c.g[A0][0] = c.lw(r(c, V1).wrapping_add(0x7c));
            let one = c.g[A0][0] == c.g[A1][0];
            c.g[A0][0] = c.g[S0][0];
            if one {
                c.cycles += 3;
                c.g[A2][0] = 0xe;
                c.g[A3][0] = 8;
                c.g[T0][0] = 1;
                c.cycles += 6;
                add_choice(c, 0x0027_a984)?;
            } else {
                c.cycles += 8;
                c.g[A2][0] = 0xf;
                c.g[A3][0] = 8;
                c.g[T0][0] = 0;
                add_choice(c, 0x0027_a9a8)?;
            }
            c.g[A0][0] = c.g[S0][0];
        }

        c.cycles += 17;
        c.g[A1][0] = 1;
        call_ptr(c, 0x0027_8f38, 0x0027_a9b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        c.g[A3][0] = 1;
        call_ptr(c, 0x0027_8f68, 0x0027_a9c8)?;
        call_ptr(c, 0x0028_b180, 0x0027_a9d0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x14;
        c.g[A2][0] = 1;
        call_ptr(c, 0x0028_e8c0, 0x0027_a9e0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
