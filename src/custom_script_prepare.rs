//! Native preparation of a presentation list, including the empty-list path.

use crate::call::{call_ptr, guest, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0027_6270, prepare, "ScriptList_Prepare")];

const STATE: u32 = 0xa4;
const OPTION: u32 = 0xb4;
const NEXT_ENTRY: u32 = 0xbc;
const QUEUE: u32 = 0xc;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// Prepare the first usable entry of a nonempty idle list. S0 is the list,
/// S1 initially its command queue and S3 is the native loading-state value 1.
fn prepare_entries(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    call_ptr(c, 0x0027_7450, 0x0027_62dc)?;
    c.g[S2][0] = c.g[V0][0];
    let have_entry = (c.g[S2][0] as i64) >= 0;
    c.g[A0][0] = c.g[S0][0];
    if !have_entry {
        c.cycles += 4;
        c.g[A1][0] = 1;
        call_ptr(c, 0x0027_6868, 0x0027_62f0)?;
        c.g[V0][0] = c.lw(r(c, S0).wrapping_add(STATE));
        return Ok(());
    }

    c.cycles += 11;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, 0x0027_7598, 0x0027_6300)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0028_3d70, 0x0027_630c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0027_7dd8, 0x0027_631c)?;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.cycles += 2;
        let movie_loaded = (c.g[V0][0] as i64) >= 0;
        c.g[V0][0] = 2;
        if movie_loaded {
            c.cycles += 1;
            c.sw(r(c, S0).wrapping_add(STATE), c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.sw(r(c, S0).wrapping_add(STATE), c.g[S3][0]);
        }
    } else {
        c.cycles += 5;
        c.g[A1][0] = c.lw(r(c, S1).wrapping_add(0xc));
        c.g[A0][0] = c.lw(r(c, S0));
        call_ptr(c, 0x0028_2798, 0x0027_6330)?;
        let process_loading = c.g[V0][0] == c.g[S3][0];
        c.g[V0][0] = 2;
        c.cycles += 2;
        c.sw(
            r(c, S0).wrapping_add(STATE),
            if process_loading {
                c.g[S3][0]
            } else {
                c.g[V0][0]
            },
        );
    }

    c.cycles += 6;
    c.g[A1][0] = sx(r(c, S2).wrapping_add(1));
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = 0;
    call_ptr(c, 0x0027_7450, 0x0027_6364)?;
    c.sw(r(c, S0).wrapping_add(NEXT_ENTRY), c.g[V0][0]);
    c.g[V0][0] = c.lw(r(c, S0).wrapping_add(STATE));
    Ok(())
}

/// `int ScriptList_Prepare(ScriptList* list, int option)` (0x276270).
/// Poll a loading list, or prepare the first entry of an idle nonempty list.
/// An empty idle list stays at state zero and returns zero; it stores the
/// option but never prepares an entry or transitions into prepared state 2.
pub fn prepare(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save(c, S2, 0x20);
        c.g[S3][0] = 1;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(r(c, S0).wrapping_add(STATE));
        let loading = c.g[V0][0] == c.g[S3][0];
        c.g[S1][0] = c.g[A1][0];
        if loading {
            c.cycles += 3;
            call_ptr(c, 0x0027_7778, 0x0027_62a4)?;
            c.g[V0][0] = c.lw(r(c, S0).wrapping_add(STATE));
        }

        c.cycles += 2;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 6;
            c.sw(r(c, S0).wrapping_add(OPTION), c.g[S1][0]);
            c.g[S1][0] = sx(r(c, S0).wrapping_add(QUEUE));
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0028_3d28, 0x0027_62c0)?;
            let nonempty = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if nonempty {
                prepare_entries(c)?;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }

        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}
