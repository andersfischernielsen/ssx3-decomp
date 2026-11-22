//! Native Event Select headings, context panels and map-tab visibility.

use crate::call::{call_ptr, guest, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0020_2ad8,
    update_mode_panels,
    "EventSelect_UpdateModePanels",
)];

const HASH: u32 = 0x0031_7670;
const FIND: u32 = 0x0039_d860;
const TEXT_ID: u32 = 0x003a_1000;
const SHOW: u32 = 0x0039_e948;
const HIDE: u32 = 0x0039_e8b8;
const SPRINTF: u32 = 0x0041_7828;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}
#[inline]
fn field(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    c.lw(r(c, reg).wrapping_add(off))
}
#[inline]
fn global(c: &mut Cpu, off: u32) -> u64 {
    c.lw(r(c, GP).wrapping_sub(off))
}

fn hash(c: &mut Cpu, name: u32, back: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, HASH, back)
}

fn find_screen(c: &mut Cpu, back: u32) -> Flow {
    c.g[A0][0] = field(c, S1, 0x2e8);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, FIND, back)
}

fn state_object(c: &mut Cpu, name: u32, visible: bool, back: u32) -> Flow {
    c.g[A0][0] = field(c, S1, 0x2ec);
    c.g[A1][0] = sx(name);
    call_ptr(c, if visible { SHOW } else { HIDE }, back)
}

fn state_register(c: &mut Cpu, reg: usize, visible: bool, back: u32) -> Flow {
    let name = r(c, reg);
    state_object(c, name, visible, back)
}

fn subtitle(c: &mut Cpu, hash_back: u32, set_back: u32) -> Flow {
    call_ptr(c, HASH, hash_back)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, TEXT_ID, set_back)
}

fn restore_s0(c: &mut Cpu) {
    c.lq(S0, r(c, SP).wrapping_add(0xd0));
}

/// `void EventSelect_UpdateModePanels(EventSelect* self)` (0x202ad8).
/// Mode 0 selects peaks, 1 selects a peak goal, and 2 selects an event.
/// These panels are independent of the active UIMenu chosen by SetMode.
pub fn update_mode_panels(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S1, 0xc0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xd0);
        c.g[A0][0] = 0x0047_0000;
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x60);
        hash(c, 0x0046_f9e0, 0x0020_2b0c)?; // SubTitle
        find_screen(c, 0x0020_2b18)?;
        c.g[V1][0] = global(c, 0xb54);
        c.g[A0][0] = 1;
        let goals = c.g[V1][0] == c.g[A0][0];
        c.g[S0][0] = c.g[V0][0];
        if goals {
            goal_panels(c)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                let peaks = c.g[V1][0] == 0;
                c.g[A0][0] = 0x0047_0000;
                if peaks {
                    peak_panels(c)?;
                } else {
                    c.cycles += 2;
                    restore_s0(c);
                }
            } else {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let events = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = global(c, 0xb50);
                if events {
                    event_panels(c)?;
                } else {
                    c.cycles += 2;
                    restore_s0(c);
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore_ra(c, 0x60);
        frame.close(c);
        Ok(())
    });
}

fn peak_panels(c: &mut Cpu) -> Flow {
    c.cycles += 25;
    c.g[A0][0] = sx(0x0047_07c0); // kT_240SelectPeak
    subtitle(c, 0x0020_2b5c, 0x0020_2b68)?;
    state_object(c, 0x0047_07d8, true, 0x0020_2b78)?; // SelectPeak
    c.g[V0][0] = 0x004a_0000;
    c.g[S2][0] = sx(0x004a_2608); // SP_Info
    state_register(c, S2, false, 0x0020_2b8c)?;
    state_object(c, 0x0047_07e8, false, 0x0020_2b9c)?; // SelectPeakGoal
    state_object(c, 0x0047_07f8, false, 0x0020_2bac)?; // SelectEvent
    c.g[V0][0] = field(c, S1, 0x2cc);
    let confirm = c.g[V0][0] != 0;
    c.g[A0][0] = field(c, S1, 0x2ec);
    if !confirm {
        c.cycles += 4;
        state_object(c, 0x004a_2610, true, 0x0020_2bc4)?; // Locks
        c.g[A0][0] = field(c, S1, 0x2ec);
    }
    c.cycles += 15;
    c.g[S0][0] = 0;
    state_object(c, 0x0047_07a0, false, 0x0020_2bd8)?; // Indicators
    state_register(c, S2, false, 0x0020_2be4)?;
    hash(c, 0x0047_0600, 0x0020_2bf0)?; // SP_PeakList
    find_screen(c, 0x0020_2bfc)?;
    if c.g[V0][0] != 0 {
        c.g[S0][0] = c.lbu(r(c, V0).wrapping_add(0x95));
    }
    c.cycles += 3;
    c.g[V0][0] = field(c, S1, 0);
    let no_info = c.g[V0][0] == 0;
    c.g[V0][0] = 3;
    if !no_info {
        c.cycles += 2;
        let mountain = c.g[S0][0] == c.g[V0][0];
        c.g[A0][0] = field(c, S1, 0x2ec);
        if mountain {
            return hide_map_panels(c);
        }
        c.cycles += 2;
        state_register(c, S2, true, 0x0020_2c20)?;
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    let mountain = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = global(c, 0xb5c);
    if mountain {
        c.cycles += 1;
        c.g[A0][0] = field(c, S1, 0x2ec);
        hide_map_panels(c)
    } else {
        c.cycles += 2;
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        finish_map_mode(c)
    }
}

fn goal_panels(c: &mut Cpu) -> Flow {
    c.cycles += 81;
    c.g[S2][0] = 0;
    c.g[A0][0] = sx(0x0047_0828); // kT_FESelPeakGoal
    subtitle(c, 0x0020_2c44, 0x0020_2c50)?;
    state_object(c, 0x0047_07d8, false, 0x0020_2c60)?;
    state_object(c, 0x0047_07e8, true, 0x0020_2c70)?;
    state_object(c, 0x0047_07f8, false, 0x0020_2c80)?;
    state_object(c, 0x004a_2610, false, 0x0020_2c90)?;
    state_object(c, 0x0047_07a0, true, 0x0020_2ca0)?;
    state_object(c, 0x0047_0840, false, 0x0020_2cb0)?;
    state_object(c, 0x0047_0850, false, 0x0020_2cc0)?;
    state_object(c, 0x0047_0860, false, 0x0020_2cd0)?;
    c.g[A2][0] = global(c, 0xb4c);
    c.g[A1][0] = sx(0x0047_0870); // peak%dindicators
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, SPRINTF, 0x0020_2ce4)?;
    state_register(c, SP, true, 0x0020_2cf0)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[S6][0] = sx(0x0047_0888); // SPG_RaceFSEvents
    state_register(c, S6, false, 0x0020_2d04)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[S5][0] = sx(0x0047_08a0); // SPG_FreerideGoals
    state_register(c, S5, false, 0x0020_2d18)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[S4][0] = sx(0x0047_08b8); // SPG_EarningsGoals
    state_register(c, S4, false, 0x0020_2d2c)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[S3][0] = sx(0x0047_08d0); // SPG_InfoBack
    state_register(c, S3, false, 0x0020_2d40)?;
    c.g[A2][0] = global(c, 0xb4c);
    c.g[S0][0] = sx(r(c, SP).wrapping_add(0x20));
    c.g[A1][0] = sx(0x0047_06d8); // SPG_Peak%dGoals
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
    call_ptr(c, SPRINTF, 0x0020_2d5c)?;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, HASH, 0x0020_2d64)?;
    find_screen(c, 0x0020_2d70)?;
    if c.g[V0][0] != 0 {
        c.g[S2][0] = c.lbu(r(c, V0).wrapping_add(0x95));
    }
    c.cycles += 3;
    c.g[V0][0] = field(c, S1, 0);
    if c.g[V0][0] != 0 {
        c.g[A0][0] = field(c, S1, 0x2ec);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        let ordinary_goal = c.g[S2][0] != c.g[V0][0];
        c.g[V0][0] = global(c, 0xb5c);
        if ordinary_goal {
            return finish_map_flow(c);
        }
        c.cycles += 1;
        c.g[A0][0] = field(c, S1, 0x2ec);
    }
    c.cycles += 5;
    state_register(c, S3, true, 0x0020_2d9c)?;
    c.g[V0][0] = 2;
    let freeride = c.g[S2][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[S2][0] as i64) < 3) as u64;
    if freeride {
        c.cycles += 5;
        state_register(c, S5, true, 0x0020_2de8)?;
        c.g[V0][0] = 3;
    } else {
        c.cycles += 2;
        let below_three = c.g[V0][0] != 0;
        c.g[V0][0] = 3;
        if below_three {
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.cycles += 5;
                state_register(c, S6, true, 0x0020_2dd4)?;
                c.g[V0][0] = 3;
            }
        } else {
            c.cycles += 2;
            let earnings = c.g[S2][0] == c.g[V0][0];
            c.g[A0][0] = field(c, S1, 0x2ec);
            if earnings {
                c.cycles += 3;
                state_register(c, S4, true, 0x0020_2df8)?;
                c.g[V0][0] = 3;
            } else {
                c.cycles += 2;
            }
        }
    }
    c.cycles += 2;
    let ordinary_goal = c.g[S2][0] != c.g[V0][0];
    c.g[V0][0] = global(c, 0xb5c);
    if ordinary_goal {
        finish_map_flow(c)
    } else {
        c.cycles += 1;
        c.g[A0][0] = field(c, S1, 0x2ec);
        hide_map_panels(c)
    }
}

fn hide_map_panels(c: &mut Cpu) -> Flow {
    c.cycles += 25;
    for (name, back) in [
        (0x004a_25b0, 0x0020_2e14),
        (0x0047_0808, 0x0020_2e24),
        (0x0047_0818, 0x0020_2e34),
        (0x004a_2618, 0x0020_2e44),
        (0x004a_2620, 0x0020_2e54),
        (0x004a_2628, 0x0020_2e64),
    ] {
        state_object(c, name, false, back)?;
    }
    restore_s0(c);
    Ok(())
}

fn finish_map_flow(c: &mut Cpu) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    finish_map_mode(c)
}

fn finish_map_mode(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let front_end = c.g[V0][0] != 0;
    restore_s0(c);
    if !front_end {
        c.cycles += 26;
        for (name, back) in [
            (0x004a_25b0, 0x0020_2e88),
            (0x0047_0808, 0x0020_2e98),
            (0x0047_0818, 0x0020_2ea8),
            (0x004a_2618, 0x0020_2eb8),
            (0x004a_2620, 0x0020_2ec8),
            (0x004a_2628, 0x0020_2ed8),
        ] {
            state_object(c, name, true, back)?;
        }
        restore_s0(c);
    }
    Ok(())
}

fn event_panels(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let tab_nonzero = c.g[V0][0] != 0;
    c.g[A1][0] = 0x004a_0000;
    if !tab_nonzero {
        c.cycles += 6;
        state_object(c, 0x004a_2610, false, 0x0020_2ef4)?;
        c.g[A0][0] = sx(0x0047_08e0); // kT_FESelRaceEvent
        event_subtitle(c)?;
    } else {
        c.cycles += 2;
        let freestyle = c.g[V0][0] == c.g[A0][0];
        c.g[A0][0] = field(c, S1, 0x2ec);
        if freestyle {
            c.cycles += 5;
            state_object(c, 0x004a_2610, false, 0x0020_2f14)?;
            c.g[A0][0] = sx(0x0047_08f8); // kT_FESelFSEvent
            event_subtitle(c)?;
        } else {
            c.cycles += 10;
            state_object(c, 0x004a_2610, false, 0x0020_2f44)?;
            hash(c, 0x0047_0908, 0x0020_2f50)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, TEXT_ID, 0x0020_2f5c)?;
            c.g[A0][0] = field(c, S1, 0x2ec);
        }
    }
    c.cycles += 47;
    state_object(c, 0x0047_07d8, false, 0x0020_2f6c)?;
    state_object(c, 0x0047_07e8, false, 0x0020_2f7c)?;
    state_object(c, 0x0047_07f8, true, 0x0020_2f8c)?;
    state_object(c, 0x0047_07a0, true, 0x0020_2f9c)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[S5][0] = sx(0x0047_0840);
    state_register(c, S5, false, 0x0020_2fb0)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[S4][0] = sx(0x0047_0850);
    state_register(c, S4, false, 0x0020_2fc4)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[S3][0] = sx(0x0047_0860);
    state_register(c, S3, false, 0x0020_2fd8)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[A2][0] = global(c, 0xb4c);
    c.g[S2][0] = sx(0x0047_0870);
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, SPRINTF, 0x0020_2ff0)?;
    state_register(c, SP, true, 0x0020_2ffc)?;
    c.g[V0][0] = 0x004a_0000;
    c.g[S0][0] = sx(0x004a_2630); // SE_Info
    state_register(c, S0, false, 0x0020_3010)?;
    c.g[V0][0] = field(c, S1, 0);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        state_register(c, S0, true, 0x0020_3028)?;
    }
    c.cycles += 20;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0020_7430, 0x0020_3030)?;
    state_register(c, S5, false, 0x0020_303c)?;
    state_register(c, S4, false, 0x0020_3048)?;
    state_register(c, S3, false, 0x0020_3054)?;
    c.g[S0][0] = sx(r(c, SP).wrapping_add(0x40));
    c.g[A2][0] = global(c, 0xb4c);
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, SPRINTF, 0x0020_3068)?;
    state_register(c, S0, true, 0x0020_3074)?;
    restore_s0(c);
    Ok(())
}

fn event_subtitle(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    subtitle(c, 0x0020_2f24, 0x0020_2f30)?;
    c.g[A0][0] = field(c, S1, 0x2ec);
    Ok(())
}
