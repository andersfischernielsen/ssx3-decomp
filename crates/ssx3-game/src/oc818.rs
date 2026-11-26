//! Staging module oc818 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc818`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0017_d3b0, lobby_menu_dtor, "LobbyMenu_dtor"),
    (0x0017_da30, lobby_menu_show_error, "LobbyMenu_ShowError"),
    (
        0x0017_db18,
        lobby_menu_update_state,
        "LobbyMenu_UpdateState",
    ),
    (
        0x0017_de70,
        lobby_menu_start_net_race,
        "LobbyMenu_StartNetRace",
    ),
];

/// `dst = src + off` (`addiu`, sign-extended 32-bit result).
fn oc818_lea(c: &mut Cpu, dst: usize, src: usize, off: u32) {
    c.g[dst][0] = sx(r32(c, src).wrapping_add(off));
}

/// The destructor's loops over 0x24-byte members, walking backwards from the
/// list end in `s0` to the list head in `s2` and calling each member's
/// virtual destructor (`a0 = member + vtable->delta`, `a1 = 0`). `always_dec`:
/// the member pointer is decremented in the branch delay slot (the other
/// loop only when it goes round again).
fn oc818_dtor_walk(c: &mut Cpu, head: u32, ret: u32, always_dec: bool) -> Flow {
    loop {
        c.cycles += 8;
        c.g[A1][0] = 0;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x10));
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(8));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, ret)?;
        let more = c.g[S2][0] != c.g[S0][0];
        if always_dec || more {
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x24));
        }
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void LobbyMenu_dtor(LobbyMenu* self, int flags)` (0x17d3b0): destroy the
/// lobby menu: its three strings, the seven sub-menus and the menu system in
/// reverse order, restore the base vtable and free the object when bit 0 of
/// `flags` is set. The generated body's resume entries stay in the generated
/// twin.
pub fn lobby_menu_dtor(c: &mut Cpu) {
    if c.pc != 0x0017_d3b0 {
        return ee::s02::f_0017d3b0__entries(c);
    }
    guest(c, |c| {
        c.cycles += 59;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, S1, 0x90);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2cb0));
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        oc818_lea(c, A0, S1, 0x1c0c);
        frame.save(c, S4, 0x60);
        oc818_lea(c, S0, S1, 0x1aac);
        frame.save(c, S5, 0x50);
        oc818_lea(c, S2, S1, 0x18c0);
        frame.save(c, S3, 0x70);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S6, 0x40);
        c.g[A1][0] = 2;
        frame.save(c, S7, 0x30);
        oc818_lea(c, S4, S1, 0x1534);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.sw(r32(c, S1), c.g[V0][0]);
        c.sw(r32(c, GP).wrapping_add(0x1da0), 0);
        call_ptr(c, 0x0031_8260, 0x0017_d40c)?;
        oc818_lea(c, A0, S1, 0x1c08);
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0017_d418)?;
        oc818_lea(c, A0, S1, 0x1c04);
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0017_d424)?;
        oc818_lea(c, A0, S1, 0x1be0);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d430)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d43c)?;
        for (off, ret) in [
            (0x1a88u32, 0x0017_d448u32),
            (0x1a64, 0x0017_d454),
            (0x1a4c, 0x0017_d460),
            (0x1a30, 0x0017_d46c),
            (0x1a14, 0x0017_d478),
            (0x19f8, 0x0017_d484),
        ] {
            oc818_lea(c, A0, S1, off);
            c.g[A1][0] = 2;
            call_ptr(c, 0x002c_a280, ret)?;
        }
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d490)?;

        // Members at s1+0x16c8..0x18c0.
        oc818_lea(c, V0, S1, 0x16c8);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[S2][0];
        let mut tail = true;
        if !empty {
            c.cycles += 2;
            if c.g[V0][0] == c.g[S0][0] {
                tail = false;
            } else {
                c.cycles += 3;
                c.g[S2][0] = c.g[V0][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x24));
                oc818_dtor_walk(c, 0x0017_d4b0, 0x0017_d4c8, true)?;
            }
        }
        if tail {
            c.cycles += 1;
        }
        oc818_lea(c, A0, S4, 0x168);

        c.cycles += 31;
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d4dc)?;
        c.g[S5][0] &= 1;
        oc818_lea(c, A0, S4, 0x150);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d4ec)?;
        oc818_lea(c, S0, S1, 0x1510);
        oc818_lea(c, A0, S4, 0x134);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d4fc)?;
        oc818_lea(c, S3, S1, 0x1044);
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d50c)?;
        c.g[S6][0] = c.g[S3][0];
        let sp0 = frame.at(c, 0);
        c.sw(sp0.wrapping_add(0xc), c.g[S5][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        oc818_lea(c, S4, S1, 0xd2c);
        call_ptr(c, 0x002c_a280, 0x0017_d524)?;
        oc818_lea(c, S5, S1, 0xe94);
        oc818_lea(c, V1, S1, 0x604);
        oc818_lea(c, V0, S1, 0x11b0);
        c.sw(sp0.wrapping_add(4), c.g[V1][0]);
        oc818_lea(c, S7, S1, 0x74c);
        oc818_lea(c, V1, S1, 0x70);
        oc818_lea(c, FP, S1, 0x734);
        c.sw(sp0, c.g[V1][0]);
        oc818_lea(c, V1, S1, 8);
        let empty = c.g[V0][0] == 0;
        c.sw(sp0.wrapping_add(8), c.g[V1][0]);

        // Members at s1+0x11b0..0x1510.
        let mut tail = true;
        if !empty {
            c.cycles += 2;
            if c.g[V0][0] == c.g[S0][0] {
                tail = false;
            } else {
                c.cycles += 2;
                c.g[S2][0] = c.g[V0][0];
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x24));
                oc818_dtor_walk(c, 0x0017_d560, 0x0017_d578, false)?;
            }
        }
        if tail {
            c.cycles += 1;
        }
        oc818_lea(c, A0, S3, 0x154);

        c.cycles += 10;
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d58c)?;
        oc818_lea(c, A0, S3, 0x138);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d598)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d5a4)?;
        let none = c.g[S5][0] == 0;
        c.g[S0][0] = c.g[S6][0];

        // Members at s1+0xe94..0x1044.
        if !none {
            c.cycles += 2;
            let same = c.g[S5][0] == c.g[S0][0];
            oc818_lea(c, S2, S4, 0x168);
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x24));
                oc818_dtor_walk(c, 0x0017_d5b8, 0x0017_d5d0, true)?;
            }
        }

        c.cycles += 30;
        oc818_lea(c, A0, S4, 0x150);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d5e4)?;
        oc818_lea(c, A0, S4, 0x134);
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d5f0)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d5fc)?;
        c.g[A0][0] = c.g[S7][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x001d_d0c8, 0x0017_d608)?;
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_a280, 0x0017_d614)?;
        c.g[A0][0] = c.lw(sp0.wrapping_add(4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_aa80, 0x0017_d620)?;
        c.g[A0][0] = c.lw(sp0);
        c.g[A1][0] = 2;
        call_ptr(c, 0x001d_cea0, 0x0017_d62c)?;
        c.g[A0][0] = c.lw(sp0.wrapping_add(8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x002c_c048, 0x0017_d638)?;
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x26b8));
        c.sw(r32(c, S1), c.g[V0][0]);
        c.g[V0][0] = c.lw(sp0.wrapping_add(0xc));
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0xa0);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0017_d658)?;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_ShowError(LobbyMenu* self, int code)` (0x17da30): store
/// the error code at `+0x1c14` and switch the menu to the error page that
/// fits the current state (state - 2 indexes a jump table of eleven
/// handlers; other states leave it alone).
pub fn lobby_menu_show_error(c: &mut Cpu) {
    if c.pc != 0x0017_da30 {
        return ee::s02::f_0017da30__entries(c);
    }
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(r32(c, A2).wrapping_add(0x1c14), c.g[A1][0]);
        c.g[V1][0] = c.lw(r32(c, A2).wrapping_add(4));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(2));
        c.g[V0][0] = (c.g[V1][0] < 0xc) as u64;
        let out = c.g[V0][0] == 0;
        c.g[V0][0] = 0x46_0000;
        // What each handler does before the common tail: (extra cycles,
        // page-state value for the common push), or one of the odd exits.
        enum Exit {
            Common(u32, u64),
            Last,
            Leave,
        }
        let exit = if out {
            Exit::Last
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2e50));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            let target = r32(c, A0);
            match target {
                0x0017_da6c => Exit::Common(2, 0x12),
                0x0017_da74 => Exit::Common(2, 0x14),
                0x0017_da7c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x74));
                    let no_world_link = c.g[V0][0] == 0;
                    c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                    if no_world_link {
                        Exit::Common(2, 0x15)
                    } else {
                        c.cycles += 12;
                        c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0xc4));
                        oc818_lea(c, A0, A0, 0x48);
                        call_ptr(c, 0x0031_84e8, 0x0017_da98)?;
                        c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                        c.g[A1][0] = c.lw(r32(c, V1).wrapping_add(0xc8));
                        oc818_lea(c, A0, A0, 0x4c);
                        call_ptr(c, 0x0031_84e8, 0x0017_daac)?;
                        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                        call_ptr(c, 0x0025_c610, 0x0017_dab4)?;
                        Exit::Leave
                    }
                }
                0x0017_dac4 => Exit::Common(2, 0x16),
                0x0017_dacc => Exit::Common(2, 0x16),
                0x0017_dad4 => Exit::Common(2, 0x16),
                0x0017_dadc => Exit::Common(1, 0x17),
                0x0017_daf8 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x18;
                    oc818_lea(c, A1, A2, 0x604);
                    c.sw(r32(c, A2).wrapping_add(4), c.g[V0][0]);
                    oc818_lea(c, A0, A2, 8);
                    call_ptr(c, 0x002c_c3b8, 0x0017_db0c)?;
                    Exit::Last
                }
                0x0017_db0c => Exit::Last,
                _ => {
                    c.jump_out(target, 0x0017_da30, 0x0017_db18);
                    return Ok(());
                }
            }
        };
        match exit {
            Exit::Common(extra, state) => {
                c.cycles += extra as u64;
                c.g[V0][0] = state;
                c.cycles += 6;
                oc818_lea(c, A1, A2, 0x604);
                c.sw(r32(c, A2).wrapping_add(4), c.g[V0][0]);
                oc818_lea(c, A0, A2, 8);
                call_ptr(c, 0x002c_c3b8, 0x0017_daf0)?;
                frame.restore_ra(c, 0);
            }
            Exit::Last => {
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
            Exit::Leave => frame.restore_ra(c, 0),
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Where `LobbyMenu_UpdateState` goes after a state's handler.
#[derive(Clone, Copy)]
enum Oc818Next {
    /// Another handler of the jump table (by its target address).
    Seg(u32),
    /// The tail that reloads the state first (`+1` cycle).
    Pre,
    /// The state comparison (`v0` already holds the state).
    Cmp,
}

/// `cMenuSys_IsActive(&self->menuSys)` into `v0`, `reg = &self->menuSys`.
fn oc818_us_is_active(c: &mut Cpu, reg: usize, ret: u32) -> Flow {
    c.cycles += 5;
    oc818_lea(c, reg, S1, 8);
    c.g[A0][0] = c.g[reg][0];
    call_ptr(c, 0x002c_c098, ret)
}

/// `cMenuSys_PushPage(&self->menuSys, page)`.
fn oc818_us_push(c: &mut Cpu, ret: u32) -> Flow {
    call_ptr(c, 0x002c_c3b8, ret)
}

/// `v0 = self->state`.
fn oc818_us_state(c: &mut Cpu) {
    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
}

/// One state handler of `LobbyMenu_UpdateState`, entered at the generated
/// block `target` of its jump table.
fn oc818_us_handler(c: &mut Cpu, target: u32) -> Result<Oc818Next, Unwound> {
    use Oc818Next::*;
    Ok(match target {
        // State 0: connect the network layer.
        0x0017_db5c => {
            c.cycles += 5;
            c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x20));
            c.g[V0][0] ^= 1;
            let busy = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !busy {
                c.cycles += 3;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0025_5f50, 0x0017_db78)?;
                c.g[V0][0] = 1;
            }
            c.cycles += 2;
            c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
            Pre
        }
        // State 1: wait for the connection.
        0x0017_db84 => {
            c.cycles += 6;
            c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x20));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(3));
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0025_b6d8, 0x0017_dba8)?;
                c.g[V0][0] = 2;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            }
        }
        // State 13: push the abort page.
        0x0017_dbb4 => {
            oc818_us_is_active(c, S0, 0x0017_dbc0)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                oc818_lea(c, A1, S1, 0x1aac);
                oc818_us_push(c, 0x0017_dbd4)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 17: occupants page, state 12.
        0x0017_dbdc => {
            oc818_us_is_active(c, S0, 0x0017_dbe8)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0xc;
                c.g[A0][0] = c.g[S0][0];
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                oc818_lea(c, A1, S1, 0x1044);
                oc818_us_push(c, 0x0017_dc04)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 24: occupants page, then state 12.
        0x0017_dc0c => {
            oc818_us_is_active(c, S0, 0x0017_dc18)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                oc818_lea(c, A1, S1, 0x1044);
                oc818_us_push(c, 0x0017_dc2c)?;
                c.g[V0][0] = 0xc;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            }
        }
        // States 21 and 20: login page, state 3.
        0x0017_dc38 | 0x0017_dc5c => {
            let ret = if target == 0x0017_dc38 {
                0x0017_dc44
            } else {
                0x0017_dc68
            };
            oc818_us_is_active(c, S0, ret)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 4;
                c.g[V0][0] = 3;
                c.g[A0][0] = c.g[S0][0];
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Seg(0x0017_dda8)
            }
        }
        // States 18 and 19: back to state 0 once the menu is gone.
        0x0017_dc80 => {
            c.cycles += 4;
            oc818_lea(c, A0, S1, 8);
            call_ptr(c, 0x002c_c098, 0x0017_dc88)?;
            if c.g[V0][0] == 0 {
                c.sw(r32(c, S1).wrapping_add(4), 0);
                Pre
            } else {
                c.cycles += 2;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 22: persona page, state 6.
        0x0017_dc98 => {
            oc818_us_is_active(c, S2, 0x0017_dca4)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 10;
                oc818_lea(c, S0, S1, 0x74c);
                c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x001d_b418, 0x0017_dcbc)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                oc818_us_push(c, 0x0017_dcc8)?;
                c.g[V0][0] = 6;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            }
        }
        // State 23: room page, state 10.
        0x0017_dcd4 => {
            oc818_us_is_active(c, S0, 0x0017_dce0)?;
            if c.g[V0][0] != 0 {
                oc818_us_state(c);
                Cmp
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                oc818_lea(c, A1, S1, 0xd2c);
                oc818_us_push(c, 0x0017_dcf4)?;
                c.g[V0][0] = 0xa;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            }
        }
        // States 15 and 6: re-push the challenge / persona page when no
        // page is up.
        0x0017_dd00 | 0x0017_dd1c => {
            let (page, ret) = if target == 0x0017_dd00 {
                (0x18c0, 0x0017_dd14)
            } else {
                (0x74c, 0x0017_dd30)
            };
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            let up = c.g[V0][0] != 0;
            oc818_lea(c, A0, S1, 8);
            if up {
                Pre
            } else {
                c.cycles += 4;
                oc818_lea(c, A1, S1, page);
                oc818_us_push(c, ret)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 10: room list.
        0x0017_dd38 => {
            c.cycles += 4;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            oc818_lea(c, A0, S1, 8);
            let up = c.g[V0][0] != 0;
            oc818_lea(c, S0, S1, 0xd2c);
            if !up {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                oc818_us_push(c, 0x0017_dd50)?;
            }
            c.cycles += 5;
            c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001d_b528, 0x0017_dd5c)?;
            oc818_us_state(c);
            Cmp
        }
        // State 12: occupants list, or leave the room.
        0x0017_dd64 => {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            let up = c.g[V0][0] != 0;
            c.g[A1][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
            if !up {
                c.cycles += 7;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
                c.g[A1][0] = u64::MAX;
                c.g[A2][0] = 0;
                call_ptr(c, 0x0025_d800, 0x0017_dd80)?;
                c.g[V0][0] = 0xb;
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                Pre
            } else {
                c.cycles += 4;
                oc818_lea(c, A0, S1, 0x1044);
                call_ptr(c, 0x001d_b620, 0x0017_dd94)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 3: re-push the login page when no page is up.
        0x0017_dd9c => {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            let up = c.g[V0][0] != 0;
            oc818_lea(c, A0, S1, 8);
            if up {
                Pre
            } else {
                Seg(0x0017_dda8)
            }
        }
        // Push the login page (`a0 = &menuSys`).
        0x0017_dda8 => {
            c.cycles += 4;
            oc818_lea(c, A1, S1, 0x70);
            oc818_us_push(c, 0x0017_ddb0)?;
            oc818_us_state(c);
            Cmp
        }
        // State 14: occupants page, state 12.
        0x0017_ddb8 => {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(8));
            let up = c.g[V0][0] != 0;
            oc818_lea(c, A0, S1, 8);
            if up {
                Pre
            } else {
                c.cycles += 6;
                c.g[V0][0] = 0xc;
                oc818_lea(c, A1, S1, 0x1044);
                c.sw(r32(c, S1).wrapping_add(4), c.g[V0][0]);
                oc818_us_push(c, 0x0017_ddd4)?;
                oc818_us_state(c);
                Cmp
            }
        }
        // State 16: when the game channel is up, start the pre-game load.
        0x0017_dddc => {
            c.cycles += 5;
            c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
            c.g[A1][0] = c.lw(r32(c, S1).wrapping_add(0x1c10));
            call_ptr(c, 0x0025_5d50, 0x0017_dde8)?;
            let down = c.g[V0][0] == 0;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
            if !down {
                c.cycles += 22;
                c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(0x1c10));
                c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0xb4));
                call_ptr(c, 0x0025_6bf8, 0x0017_ddfc)?;
                c.g[A1][0] = 0x46_0000;
                c.g[A3][0] = 0;
                c.g[A2][0] = 0x100;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x31a0));
                c.g[A0][0] = 8;
                call_ptr(c, 0x0031_7d70, 0x0017_de14)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0023_2720, 0x0017_de1c)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0031_6ef8, 0x0017_de28)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0025_6038, 0x0017_de34)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(0x304));
                c.g[A1][0] = c.lw(sp(c));
                call_ptr(c, 0x0026_6d58, 0x0017_de40)?;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(0x304));
                call_ptr(c, 0x0026_6e30, 0x0017_de48)?;
            }
            Pre
        }
        // Not a handled state (a table entry of 0x17de48).
        _ => Pre,
    })
}

/// `void LobbyMenu_UpdateState(LobbyMenu* self)` (0x17db18): the lobby
/// menu's state machine. Run the handler of the current state (a jump table
/// of 25 entries: connect, log in, persona and room pages, occupants, start
/// of the pre-game load); repeat while a handler changed the state.
pub fn lobby_menu_update_state(c: &mut Cpu) {
    if c.pc != 0x0017_db18 {
        return ee::s02::f_0017db18__entries(c);
    }
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.lw(r32(c, S1).wrapping_add(4));
        loop {
            c.cycles += 3;
            c.g[V0][0] = (c.g[S3][0] < 0x19) as u64;
            let out = c.g[V0][0] == 0;
            c.g[V0][0] = 0x46_0000;
            let mut next = if out {
                Oc818Next::Pre
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, S3) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x2e20));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let target = r32(c, A0);
                match target {
                    0x0017_db5c | 0x0017_db84 | 0x0017_dbb4 | 0x0017_dbdc | 0x0017_dc0c
                    | 0x0017_dc38 | 0x0017_dc5c | 0x0017_dc80 | 0x0017_dc98 | 0x0017_dcd4
                    | 0x0017_dd00 | 0x0017_dd1c | 0x0017_dd38 | 0x0017_dd64 | 0x0017_dd9c
                    | 0x0017_ddb8 | 0x0017_dddc => Oc818Next::Seg(target),
                    0x0017_de48 => Oc818Next::Pre,
                    _ => {
                        c.jump_out(target, 0x0017_db18, 0x0017_de70);
                        return Ok(());
                    }
                }
            };
            loop {
                match next {
                    Oc818Next::Seg(target) => next = oc818_us_handler(c, target)?,
                    Oc818Next::Pre => {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
                        break;
                    }
                    Oc818Next::Cmp => break,
                }
            }
            c.cycles += 2;
            if c.g[V0][0] != c.g[S3][0] {
                c.g[S3][0] = c.lw(r32(c, S1).wrapping_add(4));
                if event_due(c) {
                    deliver_event_here(c, 0x0017_db38);
                }
                continue;
            }
            c.cycles += 7;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void LobbyMenu_StartNetRace(LobbyMenu* self)` (0x17de70): configure the
/// new-race interface (net game, mode 4, no AI), the player interface
/// (players 0 and 1, riders 2..5 cleared) and the network interface (host or
/// guest from the lobby manager), open the game channel to the opponent's
/// address and enter state 16.
pub fn lobby_menu_start_net_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S4, 0x110);
        frame.save(c, S0, 0x150);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        frame.save(c, S3, 0x120);
        c.g[S2][0] = 2;
        frame.save_ra(c, 0x100);
        c.g[S3][0] = 0x53_0000;
        call_ptr(c, 0x0014_dc80, 0x0017_de9c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0017_dea8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_5108, 0x0017_deb8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_4df0, 0x0017_dec4)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_4d98, 0x0017_ded0)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 1;
        c.sw(r32(c, V1).wrapping_add(0x5c08), 0);
        c.sw(r32(c, GP).wrapping_sub(0x1f38), c.g[A0][0]);
        oc818_start_net_race_vcall(c, S0, 0x0017_def4)?;
        call_ptr(c, 0x0014_dc80, 0x0017_defc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0017_df08)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        loop {
            c.cycles += 11;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            call_ptr(c, 0x0014_7220, 0x0017_df1c)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            call_ptr(c, 0x0014_7338, 0x0017_df30)?;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 6) as u64;
            let again = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S2][0];
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0017_df10);
            }
        }
        c.cycles += 31;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        oc818_lea(c, S0, S3, 0x4b30);
        call_ptr(c, 0x0014_72c8, 0x0017_df50)?;
        c.g[A2][0] = u64::MAX;
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0014_72c8, 0x0017_df60)?;
        oc818_start_net_race_vcall(c, S1, 0x0017_df74)?;
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        c.g[A1][0] = 7;
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x78));
        call_ptr(c, 0x0014_dd58, 0x0017_df84)?;
        c.g[V1][0] = 1;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
        c.sw(r32(c, S3).wrapping_add(0x4b30), c.g[V1][0]);
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0xb4));
        c.sw(r32(c, S0).wrapping_add(8), c.g[V1][0]);
        c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        c.g[A0][0] = c.lh(r32(c, A1).wrapping_add(8));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0xc));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0017_dfac)?;
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(8));
        let host = c.g[V0][0] == 0;
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0xc8));
        if host {
            c.cycles += 1;
            c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0xa8));
        } else {
            c.cycles += 2;
            c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0xac));
        }
        c.cycles += 27;
        c.g[T1][0] = c.g[A2][0] & 0xff;
        c.g[A3][0] = sx(r32(c, A2) >> 16);
        c.g[T0][0] = sx(r32(c, A2) >> 8);
        c.g[A1][0] = 0x46_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] &= 0xff;
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x3068));
        c.g[A3][0] &= 0xff;
        c.g[A2][0] = sx(r32(c, A2) >> 24);
        call_ptr(c, 0x0041_7828, 0x0017_dfec)?;
        oc818_lea(c, V1, S3, 0x4b30);
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x238));
        c.g[A3][0] = c.lw(r32(c, V1).wrapping_add(8));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.lhu(r32(c, V1).wrapping_add(0xc));
        call_ptr(c, 0x0025_5bf8, 0x0017_e004)?;
        c.g[V1][0] = 0x10;
        c.sw(r32(c, S4).wrapping_add(0x1c10), c.g[V0][0]);
        c.sw(r32(c, S4).wrapping_add(4), c.g[V1][0]);
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// A virtual call on the object at `*(reg + 0xc)`: `a0 = reg + vtable->delta`
/// (halfword at `vtable + 8`), target at `vtable + 0xc`, returning to `ret`.
fn oc818_start_net_race_vcall(c: &mut Cpu, reg: usize, ret: u32) -> Flow {
    c.g[V0][0] = c.lw(r32(c, reg).wrapping_add(0xc));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(8));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, reg).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The lobby menu object (0x1c18 bytes).
    const OC818_OBJ: u32 = DATA + 0x40_0000;
    /// The comm system, a channel and its vtable.
    const OC818_COMM: u32 = DATA + 0x41_0000;
    const OC818_WORLD: u32 = DATA + 0x42_0000;
    const OC818_LOBBY_MAN: u32 = DATA + 0x43_0000;
    const OC818_VT: u32 = DATA + 0x44_0000;
    const OC818_CHAN: u32 = DATA + 0x45_0000;
    const OC818_CHAN_VT: u32 = DATA + 0x46_0000;
    const OC818_VOIP: u32 = DATA + 0x47_0000;
    /// The gate of the callee fakes (a word in the machine's own RAM).
    const OC818_CTRL: u32 = DATA + 0x20_0000;
    const OC818_MAGIC: u32 = 0x0c81_8818;
    /// Forced results of the fakes: one word per fake at `FORCE + (addr & 0xffffc)`.
    const OC818_FORCE: u32 = DATA + 0x30_0000;

    fn oc818_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Make the fake for `addr` return 0.
    fn oc818_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(OC818_FORCE + (addr & 0xf_fffc), 1);
    }

    /// The gated fake callee: logs the call like `fakes::fake_call`, returns
    /// 0 when its mailbox says so.
    fn oc818_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC818_FORCE + (addr & 0xf_fffc));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc818_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC818_CTRL) != OC818_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc818_fake_call(c, $addr);
                }
            )*
            fn oc818_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc818_fakes!(
        (
            oc818_fake_318260,
            OC818_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x00318260
        ),
        (
            oc818_fake_317e50,
            OC818_PREV_317E50,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated,
            0x00317e50
        ),
        (
            oc818_fake_2ca280,
            OC818_PREV_2CA280,
            ee::s09::OVERRIDE_002CA280,
            ee::s09::f_002ca280__generated,
            0x002ca280
        ),
        (
            oc818_fake_2caa80,
            OC818_PREV_2CAA80,
            ee::s09::OVERRIDE_002CAA80,
            ee::s09::f_002caa80__generated,
            0x002caa80
        ),
        (
            oc818_fake_2cc048,
            OC818_PREV_2CC048,
            ee::s09::OVERRIDE_002CC048,
            ee::s09::f_002cc048__generated,
            0x002cc048
        ),
        (
            oc818_fake_2cc3b8,
            OC818_PREV_2CC3B8,
            ee::s09::OVERRIDE_002CC3B8,
            ee::s09::f_002cc3b8__generated,
            0x002cc3b8
        ),
        (
            oc818_fake_2cc098,
            OC818_PREV_2CC098,
            ee::s09::OVERRIDE_002CC098,
            ee::s09::f_002cc098__generated,
            0x002cc098
        ),
        (
            oc818_fake_1dd0c8,
            OC818_PREV_1DD0C8,
            ee::s04::OVERRIDE_001DD0C8,
            ee::s04::f_001dd0c8__generated,
            0x001dd0c8
        ),
        (
            oc818_fake_1dcea0,
            OC818_PREV_1DCEA0,
            ee::s04::OVERRIDE_001DCEA0,
            ee::s04::f_001dcea0__generated,
            0x001dcea0
        ),
        (
            oc818_fake_1db418,
            OC818_PREV_1DB418,
            ee::s04::OVERRIDE_001DB418,
            ee::s04::f_001db418__generated,
            0x001db418
        ),
        (
            oc818_fake_1db528,
            OC818_PREV_1DB528,
            ee::s04::OVERRIDE_001DB528,
            ee::s04::f_001db528__generated,
            0x001db528
        ),
        (
            oc818_fake_1db620,
            OC818_PREV_1DB620,
            ee::s04::OVERRIDE_001DB620,
            ee::s04::f_001db620__generated,
            0x001db620
        ),
        (
            oc818_fake_25b6d8,
            OC818_PREV_25B6D8,
            ee::s06::OVERRIDE_0025B6D8,
            ee::s06::f_0025b6d8__generated,
            0x0025b6d8
        ),
        (
            oc818_fake_25d800,
            OC818_PREV_25D800,
            ee::s06::OVERRIDE_0025D800,
            ee::s06::f_0025d800__generated,
            0x0025d800
        ),
        (
            oc818_fake_232720,
            OC818_PREV_232720,
            ee::s06::OVERRIDE_00232720,
            ee::s06::f_00232720__generated,
            0x00232720
        ),
        (
            oc818_fake_25c610,
            OC818_PREV_25C610,
            ee::s06::OVERRIDE_0025C610,
            ee::s06::f_0025c610__generated,
            0x0025c610
        ),
        (
            oc818_fake_317d70,
            OC818_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc818_fake_316ef8,
            OC818_PREV_316EF8,
            ee::s10::OVERRIDE_00316EF8,
            ee::s10::f_00316ef8__generated,
            0x00316ef8
        ),
        (
            oc818_fake_3184e8,
            OC818_PREV_3184E8,
            ee::s10::OVERRIDE_003184E8,
            ee::s10::f_003184e8__generated,
            0x003184e8
        ),
        (
            oc818_fake_266d58,
            OC818_PREV_266D58,
            ee::s07::OVERRIDE_00266D58,
            ee::s07::f_00266d58__generated,
            0x00266d58
        ),
        (
            oc818_fake_266e30,
            OC818_PREV_266E30,
            ee::s07::OVERRIDE_00266E30,
            ee::s07::f_00266e30__generated,
            0x00266e30
        ),
        (
            oc818_fake_3f3cc0,
            OC818_PREV_3F3CC0,
            ee::s14::OVERRIDE_003F3CC0,
            ee::s14::f_003f3cc0__generated,
            0x003f3cc0
        ),
        (
            oc818_fake_14dc80,
            OC818_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc818_fake_14dd58,
            OC818_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc818_fake_145108,
            OC818_PREV_145108,
            ee::s01::OVERRIDE_00145108,
            ee::s01::f_00145108__generated,
            0x00145108
        ),
        (
            oc818_fake_144df0,
            OC818_PREV_144DF0,
            ee::s01::OVERRIDE_00144DF0,
            ee::s01::f_00144df0__generated,
            0x00144df0
        ),
        (
            oc818_fake_144d98,
            OC818_PREV_144D98,
            ee::s01::OVERRIDE_00144D98,
            ee::s01::f_00144d98__generated,
            0x00144d98
        ),
        (
            oc818_fake_147220,
            OC818_PREV_147220,
            ee::s01::OVERRIDE_00147220,
            ee::s01::f_00147220__generated,
            0x00147220
        ),
        (
            oc818_fake_147338,
            OC818_PREV_147338,
            ee::s01::OVERRIDE_00147338,
            ee::s01::f_00147338__generated,
            0x00147338
        ),
        (
            oc818_fake_1472c8,
            OC818_PREV_1472C8,
            ee::s01::OVERRIDE_001472C8,
            ee::s01::f_001472c8__generated,
            0x001472c8
        ),
        (
            oc818_fake_417828,
            OC818_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
    );

    /// The common machine: gated fakes on, the jump tables the game has in
    /// its data segment, the menu object at `OC818_OBJ`.
    fn oc818_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        fakes::machine(c, seed, salt, 0);
        c.pc = entry;
        c.bus.write32(OC818_CTRL, OC818_MAGIC);
        c.g[A0][0] = OC818_OBJ as u64;
        oc818_w(c, GAME_GP.wrapping_sub(0x238), OC818_COMM);
        oc818_w(c, GAME_GP.wrapping_sub(0xc8), OC818_LOBBY_MAN);
        oc818_w(c, GAME_GP.wrapping_sub(0x848), OC818_WORLD);
    }

    fn oc818_table(c: &mut Cpu, base: u32, targets: &[u32]) {
        for (i, t) in targets.iter().enumerate() {
            c.bus.write32(base + 4 * i as u32, *t);
        }
    }

    /// The members of the destructor's three lists: a vtable pointer at +0x10
    /// whose slot (+8 delta, +0xc function) is a fake.
    fn oc818_dtor_members(c: &mut Cpu, seed: u32) {
        c.bus.write16(OC818_VT + 8, ((seed & 7) * 4) as u16);
        c.bus.write32(OC818_VT + 0xc, fakes::FAKE);
        for (from, to) in [(0x16c8u32, 0x18c0u32), (0x11b0, 0x1510), (0xe94, 0x1044)] {
            let mut m = from;
            while m < to {
                oc818_w(c, OC818_OBJ + m + 0x10, OC818_VT);
                m += 0x24;
            }
        }
    }

    /// `LobbyMenu_dtor`: all three member lists, both values of the
    /// `delete` flag.
    #[test]
    fn lobby_menu_dtor_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017d3b0__generated,
            lobby_menu_dtor,
            20,
            &|c, seed| {
                oc818_machine(c, seed, 0x17d3b0, 0x0017_d3b0);
                oc818_dtor_members(c, seed);
                c.g[A1][0] = [0u64, 1, 2, 3, 0xffff_fffe][seed as usize % 5];
            },
        );
    }

    const OC818_SHOW_ERROR_TABLE: [u32; 12] = [
        0x0017_da6c,
        0x0017_db0c,
        0x0017_da74,
        0x0017_da7c,
        0x0017_db0c,
        0x0017_dac4,
        0x0017_dacc,
        0x0017_dad4,
        0x0017_db0c,
        0x0017_dadc,
        0x0017_db0c,
        0x0017_daf8,
    ];

    /// `LobbyMenu_ShowError`: every state (in and out of the jump table's
    /// range), with the world link present and absent for state 5.
    #[test]
    fn lobby_menu_show_error_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017da30__generated,
            lobby_menu_show_error,
            40,
            &|c, seed| {
                oc818_machine(c, seed, 0x17da30, 0x0017_da30);
                oc818_table(c, 0x45_d1b0, &OC818_SHOW_ERROR_TABLE);
                let state = match seed % 18 {
                    16 => 0xffff_fffe,
                    17 => 0x40,
                    s => s,
                };
                oc818_w(c, OC818_OBJ + 4, state);
                oc818_w(c, OC818_WORLD + 0x74, (seed / 18) & 1);
            },
        );
    }

    const OC818_UPDATE_STATE_TABLE: [u32; 25] = [
        0x0017_db5c,
        0x0017_db84,
        0x0017_de48,
        0x0017_dd9c,
        0x0017_de48,
        0x0017_de48,
        0x0017_dd1c,
        0x0017_de48,
        0x0017_de48,
        0x0017_de48,
        0x0017_dd38,
        0x0017_de48,
        0x0017_dd64,
        0x0017_dbb4,
        0x0017_ddb8,
        0x0017_dd00,
        0x0017_dddc,
        0x0017_dbdc,
        0x0017_dc80,
        0x0017_dc80,
        0x0017_dc5c,
        0x0017_dc38,
        0x0017_dc98,
        0x0017_dcd4,
        0x0017_dc0c,
    ];

    /// The fixture of the state machine: the state, whether a page is up
    /// (`+8`), whether `cMenuSys_IsActive` says so, the comm system's mode
    /// word and the game channel (virtual `isConnected` returning `connected`).
    fn oc818_update_state_fixture(c: &mut Cpu, seed: u32, state: u32, connected: u32) {
        oc818_machine(c, seed, 0x17db18, 0x0017_db18);
        oc818_table(c, 0x45_d1e0, &OC818_UPDATE_STATE_TABLE);
        let variant = seed / 27;
        oc818_w(c, OC818_OBJ + 4, state);
        oc818_w(c, OC818_OBJ + 8, variant & 1);
        if variant & 2 != 0 {
            oc818_force_zero(c, 0x002c_c098);
        }
        oc818_w(c, OC818_OBJ + 0x1c10, 0x10);
        // Never 1: that would call the real `cCommSystem_netConnConnect`.
        let mode = [3u32, 0, 4, 2, 5, 0][(seed / 2) as usize % 6];
        oc818_w(c, OC818_COMM + 0x20, mode);
        oc818_w(c, OC818_COMM + 0x40, OC818_CHAN);
        oc818_w(c, OC818_CHAN + 8, OC818_CHAN_VT);
        c.bus.write16(OC818_CHAN_VT + 0x20, (seed & 3) as u16 * 8);
        oc818_w(c, OC818_CHAN_VT + 0x24, connected);
        oc818_w(c, GAME_GP.wrapping_add(0x304), OC818_VOIP);
    }

    /// `LobbyMenu_UpdateState` over every state, with a page up and none,
    /// the menu system active and not. The game channel is never connected
    /// here (the pre-game load has its own test).
    #[test]
    fn lobby_menu_update_state_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017db18__generated,
            lobby_menu_update_state,
            108,
            &|c, seed| {
                let state = match seed % 27 {
                    26 => 0xffff_fff0,
                    s => s,
                };
                oc818_update_state_fixture(c, seed, state, 0x0014_06c0);
            },
        );
    }

    /// State 16 with a connected game channel: build the game comm and
    /// start the pre-game load. The real `cGameComm_construct` runs here on
    /// fake memory.
    #[test]
    fn lobby_menu_update_state_game_start_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017db18__generated,
            lobby_menu_update_state,
            8,
            &|c, seed| {
                oc818_update_state_fixture(c, seed, 16, fakes::FAKE);
                oc818_w(c, OC818_LOBBY_MAN + 0xb4, seed);
            },
        );
    }

    /// `LobbyMenu_StartNetRace`: host and guest, with a comm system whose
    /// only channel slot is taken so `cCommSystem_openDSockChannel` returns -1.
    #[test]
    fn lobby_menu_start_net_race_matches() {
        oc818_install_fakes();
        check(
            ee::s02::f_0017de70__generated,
            lobby_menu_start_net_race,
            20,
            &|c, seed| {
                oc818_machine(c, seed, 0x17de70, 0x0017_de70);
                oc818_w(c, OC818_COMM, 0x0090_1230);
                oc818_w(c, OC818_WORLD + 0x78, 0x1234);
                oc818_w(c, OC818_LOBBY_MAN + 0xb4, seed & 1);
                oc818_w(c, OC818_LOBBY_MAN + 0xa8, 0xc0a8_0001 ^ seed);
                oc818_w(c, OC818_LOBBY_MAN + 0xac, 0x0a00_00fe ^ (seed << 8));
            },
        );
    }
}
