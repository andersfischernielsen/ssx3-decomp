//! Staging module oc1043 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1043`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0024_02d0,
        mc_manager_state1d_check,
        "MCManager_State1D_Check",
    ),
    (0x0024_2288, mc_manager_ctor, "MCManager_ctor"),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_SET_MODE;

    /// `MCManager_PollCardStatus(self)`: reread the card flags into `+0x43c`;
    /// true when the card is ready.
    pub const MC_MANAGER_POLL_CARD_STATUS: Callee = Callee {
        addr: 0x0024_1cd8,
        f: ee::s06::MCManager_PollCardStatus,
    };

    /// `MCManager_SetStateNoCard(self)`: post state 4.
    pub const MC_MANAGER_SET_STATE_NO_CARD: Callee = Callee {
        addr: 0x0023_eb50,
        f: ee::s06::MCManager_SetStateNoCard,
    };

    /// `MCManager_ReturnToSlotState(self)`: post state 2 or 3.
    pub const MC_MANAGER_RETURN_TO_SLOT_STATE: Callee = Callee {
        addr: 0x0024_2050,
        f: ee::s06::MCManager_ReturnToSlotState,
    };
}

/// `self + off` for the manager in `s0` (read again after every call).
#[inline]
fn state1d_field(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S0).wrapping_add(off))
}

/// Reload `s0` and leave through the 0x50 frame's shared epilogue.
fn state1d_leave(c: &mut Cpu, frame: Frame) -> Flow {
    use crate::game::movie::movieplayer::mcus_epilogue;
    frame.restore(c, S0, 0x40);
    mcus_epilogue(c, frame);
    Ok(())
}

/// Post the code in `a1` through slot 8 of the message sink in `v0` (the
/// table at `self+0x748`: `{short delta; fn}`, called with `self + delta`),
/// returning to `ret`.
fn state1d_send(c: &mut Cpu, ret: u32) -> Flow {
    let sink = r32(c, V0);
    c.g[A0][0] = Addr(sink.wrapping_add(8)).lh(c);
    c.g[V1][0] = Addr(sink.wrapping_add(0xc)).lw(c);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// The shared report tail (0x240578): charge `cycles` for the branch that
/// got here, load the sink from `+0x748`, post `a1` and leave.
fn state1d_report(c: &mut Cpu, frame: Frame, cycles: u64) -> Flow {
    c.cycles += cycles;
    c.g[V0][0] = state1d_field(c, 0x748).lw(c);
    c.cycles += 6;
    state1d_send(c, 0x0024_0588)?;
    state1d_leave(c, frame)
}

/// `void MCManager_State1D_Check(MCManager *self)` (0x2402d0): one update of
/// the card-check state (0x1d).
///
/// Does nothing while the periodic card check (`+0x338`) is set. A pending
/// request (`+0xbc` or `+0xc0`, or the `+0x78` hook firing) counts as "quit".
/// Polls the card (`MCManager_PollCardStatus`; the flags at `+0x43c`, with the
/// flags from before the poll kept in `s3`). When the card is ready and the
/// `+0x78` hook does not fire: with flag 8 set, goes to the no-card state
/// (`MCManager_SetStateNoCard`) if word `+0x124`, the `+0x78` or `+0x68` hook
/// or the quit request says so; without flag 8, posts 0x27 (flag 0x10 and word
/// `+0x120`), 5 (flag 1) or 0x17 (flag 4 and word `+0x11c`) to the message
/// sink at `+0x748`, or rescans (`MCManager_OnCardScanned`) when flag 8 was set
/// before the poll, or when the flags are now 0 and had a flag other than 4
/// before.
///
/// Then, unless quitting, handles a card change (`+0xc8`: clear it and
/// `MCManager_ReturnToSlotState`) and posts 0x31 while flag 8 is set. Runs the
/// `+0xa4` hook; if the state (`+0x130`) is still 0x1d: when the `+0x78` hook
/// fires, posts 0x1f (word `+0xd0` set) or asks the backend at `+0x434`
/// (slot 0x1a8, then 0x218 and 0x208 with the file at `+0xf8`) and posts 0x24
/// or 0x25; else when the `+0x68` hook fires and word `+0xd4` is set, calls
/// backend slot 0x70 with the file at `+0xf8` and `self + 0x234`, then posts
/// 0x18; else when the `+0x6c` hook fires, posts 1.
pub fn mc_manager_state1d_check(c: &mut Cpu) {
    use crate::game::movie::movieplayer::mcus_card_call;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = state1d_field(c, 0x338).lw(c);
        if c.g[V0][0] != 0 {
            return state1d_leave(c, frame);
        }

        // s2: quit. `+0xbc` and the hook set it in one shared block, `+0xc0`
        // in its branch's delay slot.
        c.cycles += 3;
        c.g[V0][0] = state1d_field(c, 0xbc).lw(c);
        c.g[S2][0] = 0;
        let mut shared_set = c.g[V0][0] != 0;
        let mut quit = shared_set;
        if !quit {
            c.cycles += 3;
            c.g[V0][0] = state1d_field(c, 0xc0).lw(c);
            if c.g[V0][0] != 0 {
                c.g[S2][0] = 1;
                quit = true;
            } else {
                c.cycles += 5;
                c.g[V0][0] = state1d_field(c, 0x78).lw(c);
                call_ptr(c, r32(c, V0), 0x0024_031c)?;
                shared_set = c.g[V0][0] != 0;
                quit = shared_set;
            }
        }
        if shared_set {
            c.cycles += 1;
            c.g[S2][0] = 1;
        }
        if quit {
            c.cycles += 1;
        }
        c.g[S3][0] = state1d_field(c, 0x43c).lw(c);

        // s1: the card is ready.
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        callee::MC_MANAGER_POLL_CARD_STATUS.call(c, 0x0024_0334)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = state1d_field(c, 0x78).lw(c);
        call_ptr(c, r32(c, V0), 0x0024_0344)?;
        'card: {
            if c.g[V0][0] != 0 {
                break 'card;
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                break 'card;
            }
            c.cycles += 5;
            c.g[V0][0] = state1d_field(c, 0x43c).lw(c);
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] &= 8;
            if c.g[V0][0] != 0 {
                // Flag 8: no card, unless nothing asks for the no-card state.
                c.cycles += 3;
                c.g[V0][0] = state1d_field(c, 0x124).lw(c);
                let mut no_card = c.g[V0][0] != 0;
                if !no_card {
                    c.cycles += 5;
                    c.g[V0][0] = state1d_field(c, 0x78).lw(c);
                    call_ptr(c, r32(c, V0), 0x0024_0380)?;
                    no_card = c.g[V0][0] != 0;
                }
                if !no_card {
                    c.cycles += 5;
                    c.g[V0][0] = state1d_field(c, 0x68).lw(c);
                    call_ptr(c, r32(c, V0), 0x0024_0394)?;
                    no_card = c.g[V0][0] != 0;
                }
                if !no_card {
                    c.cycles += 2;
                    no_card = c.g[S2][0] != 0;
                    if !no_card {
                        c.g[V0][0] = state1d_field(c, 0x43c).lw(c);
                    }
                }
                if no_card {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_SET_STATE_NO_CARD.call(c, 0x0024_03ac)?;
                    return state1d_leave(c, frame);
                }
            } else {
                c.g[V0][0] = state1d_field(c, 0x120).lw(c);
                // Word +0x120 and flag 0x10: post 0x27.
                c.cycles += 2;
                let off = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] & 0x10;
                if !off {
                    c.cycles += 2;
                    let clear = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x27;
                    if !clear {
                        return state1d_report(c, frame, 2);
                    }
                }
                // Flag 1: post 5.
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 1;
                let clear = c.g[V0][0] == 0;
                c.g[A1][0] = 5;
                if !clear {
                    return state1d_report(c, frame, 2);
                }
                // Word +0x11c and flag 4: post 0x17.
                c.cycles += 3;
                c.g[V0][0] = state1d_field(c, 0x11c).lw(c);
                let off = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] & 4;
                if !off {
                    c.cycles += 2;
                    let clear = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x17;
                    if !clear {
                        return state1d_report(c, frame, 2);
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = state1d_field(c, 0x43c).lw(c);
            }

            // Rescan when flag 8 went away during the poll, or when the flags
            // are now 0 and the old ones had a flag other than 4.
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            let no_card_now = c.g[V0][0] & 8 != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_fffb;
            let mut rescan = false;
            if !no_card_now {
                c.cycles += 3;
                rescan = c.g[S3][0] & 8 != 0;
            }
            if !rescan {
                c.cycles += 2;
                let flags_set = c.g[V1][0] != 0;
                c.g[V0][0] = c.g[S3][0] & c.g[V0][0];
                if flags_set {
                    break 'card;
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    break 'card;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_ON_CARD_SCANNED.call(c, 0x0024_0434)?;
            return state1d_leave(c, frame);
        }

        c.cycles += 2;
        if c.g[S2][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = state1d_field(c, 0xc8).lw(c);
            if c.g[V0][0] != 0 {
                // The card changed: back to the slot state.
                c.cycles += 4;
                state1d_field(c, 0xc8).sw(c, 0);
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_RETURN_TO_SLOT_STATE.call(c, 0x0024_045c)?;
            }
            c.g[V0][0] = state1d_field(c, 0x43c).lw(c);
            c.cycles += 3;
            c.g[V0][0] &= 8;
            let clear = c.g[V0][0] == 0;
            c.g[A1][0] = 0x31;
            if !clear {
                return state1d_report(c, frame, 2);
            }
            c.cycles += 1;
        }
        c.g[V1][0] = state1d_field(c, 0xa4).lw(c);
        c.cycles += 6;
        call_ptr(c, r32(c, V1), 0x0024_0480)?;
        c.g[V0][0] = state1d_field(c, 0x130).lw(c) ^ 0x1d;
        if c.g[V0][0] != 0 {
            return state1d_leave(c, frame);
        }

        c.cycles += 5;
        c.g[V0][0] = state1d_field(c, 0x78).lw(c);
        call_ptr(c, r32(c, V0), 0x0024_049c)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = state1d_field(c, 0xd0).lw(c);
            let clear = c.g[V0][0] == 0;
            c.g[A1][0] = 0x1f;
            if !clear {
                return state1d_report(c, frame, 2);
            }
            c.cycles += 8;
            c.g[A1][0] = state1d_field(c, 0x434).lw(c);
            mcus_card_call(c, A1, V1, V0, 0x1a8, 0x0024_04d0)?;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = 0x24;
                return state1d_report(c, frame, 3);
            }
            c.g[A2][0] = state1d_field(c, 0x434).lw(c);
            c.cycles += 8;
            c.g[A1][0] = state1d_field(c, 0xf8).lw(c);
            mcus_card_call(c, A2, V1, V0, 0x218, 0x0024_04fc)?;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = 0x25;
                return state1d_report(c, frame, 3);
            }
            c.g[A2][0] = state1d_field(c, 0x434).lw(c);
            c.cycles += 8;
            c.g[A1][0] = state1d_field(c, 0xf8).lw(c);
            mcus_card_call(c, A2, V1, V0, 0x208, 0x0024_0528)?;
            let clear = c.g[V0][0] == 0;
            c.g[A1][0] = 0x25;
            if clear {
                c.cycles += 1;
                return state1d_leave(c, frame);
            }
            return state1d_report(c, frame, 2);
        }

        c.g[V0][0] = state1d_field(c, 0x68).lw(c);
        c.cycles += 4;
        call_ptr(c, r32(c, V0), 0x0024_0540)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = state1d_field(c, 0xd4).lw(c);
            let clear = c.g[V0][0] == 0;
            c.g[A2][0] = sx(r32(c, S0).wrapping_add(0x234));
            if clear {
                c.cycles += 1;
                return state1d_leave(c, frame);
            }
            c.cycles += 9;
            c.g[A3][0] = state1d_field(c, 0x434).lw(c);
            c.g[A1][0] = state1d_field(c, 0xf8).lw(c);
            mcus_card_call(c, A3, V0, V1, 0x70, 0x0024_0570)?;
            c.g[A1][0] = 0x18;
            return state1d_report(c, frame, 0);
        }

        c.g[V0][0] = state1d_field(c, 0x6c).lw(c);
        c.cycles += 4;
        call_ptr(c, r32(c, V0), 0x0024_0598)?;
        let clear = c.g[V0][0] == 0;
        c.g[A1][0] = 1;
        if !clear {
            c.cycles += 5;
            c.g[V0][0] = state1d_field(c, 0x748).lw(c);
            state1d_send(c, 0x0024_05b4)?;
        }
        c.cycles += 1;
        state1d_leave(c, frame)
    })
}

/// The table `+0x748` points at (its slot 8 posts a state).
pub(crate) const MC_MANAGER_CTOR_SINK: u32 = 0x0047_cbd8;

/// The default hook stubs: `void` no-op, return 0, return 1, return 0.
pub(crate) const MC_MANAGER_CTOR_NOP: u32 = 0x0024_4860;
pub(crate) const MC_MANAGER_CTOR_FALSE: u32 = 0x0024_4868;
pub(crate) const MC_MANAGER_CTOR_TRUE: u32 = 0x0024_4870;
pub(crate) const MC_MANAGER_CTOR_FALSE_2: u32 = 0x0024_4878;

/// Hook words the constructor fills after `+0x1c`/`+0x34`, in the compiled
/// order, with the register holding the stub (`v0` no-op, `v1` and `a0`
/// return 0, `a1` return 1).
const MC_MANAGER_CTOR_HOOKS: [(u32, usize); 43] = [
    (0x00, V0),
    (0x04, A0),
    (0x08, V0),
    (0x0c, V0),
    (0x10, V0),
    (0x14, V0),
    (0x18, V0),
    (0x20, V1),
    (0x24, V0),
    (0x28, V0),
    (0x2c, V0),
    (0x30, A1),
    (0x38, V0),
    (0x3c, V0),
    (0x40, V0),
    (0x44, V0),
    (0x48, V0),
    (0x4c, V0),
    (0x50, V0),
    (0x54, V0),
    (0x58, V0),
    (0x5c, V0),
    (0x60, V0),
    (0x64, V1),
    (0x68, V1),
    (0x6c, V1),
    (0xac, V0),
    (0xb0, V1),
    (0x70, V1),
    (0x74, V1),
    (0x78, V1),
    (0x7c, V1),
    (0x80, V1),
    (0x84, V1),
    (0x88, V1),
    (0x8c, V0),
    (0x90, V0),
    (0x94, V0),
    (0x98, V0),
    (0x9c, V0),
    (0xa0, V0),
    (0xa4, V0),
    (0xa8, V0),
];

/// `self + off` for the manager in `s2`.
#[inline]
fn mc_manager_ctor_field(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S2).wrapping_add(off))
}

/// `MCManager *MCManager_ctor(MCManager *self)` (0x242288): set up a new
/// memory-card manager and return it.
///
/// Clears the request words (`+0xbc`..`+0xc4`), sets word `+0xb4` to 2 and the
/// message sink `+0x748` to the table at 0x47cbd8, enters mode 1
/// (`MCManager_SetMode`), then clears the state (`+0x130`), the file cursor
/// (`+0xf8`) and the words `+0xc8`..`+0xf4`, `+0x100`..`+0x10c`,
/// `+0x118`..`+0x124` and `+0x12c`, sets `+0xfc`, `+0x424` and `+0x444` to -1,
/// `+0x128` to 1, `+0x110` to 1.0 and `+0x114` to the float at `gp-0x5120`;
/// clears the first device slot at `+0x344` (0xe0 bytes; the compiled
/// one-pass loop clears it a second time), the 0x42 bytes at
/// `+0x234`, the slot and card words (`+0x428`, `+0x42c`, `+0x434`..`+0x440`)
/// and the check words (`+0x338`, `+0x340`), sets `+0x33c` to 5, clears the
/// global at `gp-0x478`, and points the 45 hook words `+0x00`..`+0xb0` at the
/// default stubs.
pub fn mc_manager_ctor(c: &mut Cpu) {
    use crate::game::movie::movieplayer::mcus_callee::MEM_FILL;
    guest(c, |c| {
        c.cycles += 128;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S2, 0x30);
        c.g[V1][0] = 2;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[V0][0] = sx(MC_MANAGER_CTOR_SINK);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xffff_ffff_ffff_ffff;
        mc_manager_ctor_field(c, 0xbc).sw(c, 0);
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x344));
        mc_manager_ctor_field(c, 0xc0).sw(c, 0);
        c.g[S3][0] = c.g[S0][0];
        mc_manager_ctor_field(c, 0xc4).sw(c, 0);
        c.g[S4][0] = 0;
        mc_manager_ctor_field(c, 0xb4).sw(c, c.g[V1][0]);
        mc_manager_ctor_field(c, 0x748).sw(c, c.g[V0][0]);
        callee::MC_MANAGER_SET_MODE.call(c, 0x0024_22e0)?;

        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.g[V0][0] = 1;
        let k = r32(c, GP).wrapping_sub(0x5120);
        c.lwc1(0, k);
        c.g[A0][0] = c.g[S0][0];
        mc_manager_ctor_field(c, 0x130).sw(c, 0);
        c.g[A1][0] = 0;
        mc_manager_ctor_field(c, 0xf8).sw(c, 0);
        c.g[A2][0] = 0xe0;
        mc_manager_ctor_field(c, 0xfc).sw(c, c.g[S1][0]);
        for off in [
            0x10c, 0x104, 0x108, 0xc8, 0xcc, 0xd0, 0xd4, 0xd8, 0xe0, 0xf0, 0xe8, 0xe4, 0xf4, 0xdc,
            0xec,
        ] {
            mc_manager_ctor_field(c, off).sw(c, 0);
        }
        mc_manager_ctor_field(c, 0x424).sw(c, c.g[S1][0]);
        for off in [0x100, 0x12c, 0x118, 0x11c, 0x120, 0x124] {
            mc_manager_ctor_field(c, off).sw(c, 0);
        }
        mc_manager_ctor_field(c, 0x128).sw(c, c.g[V0][0]);
        let at = r32(c, S2).wrapping_add(0x110);
        c.swc1(1, at);
        let at = r32(c, S2).wrapping_add(0x114);
        c.swc1(0, at);
        // MEM_fill(&self->+0x344, 0, 0xe0)
        call_ptr(c, MEM_FILL, 0x0024_2370)?;

        mc_manager_ctor_field(c, 0x444).sw(c, c.g[S1][0]);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(0x234));
        mc_manager_ctor_field(c, 0x428).sw(c, 0);
        c.g[A1][0] = 0;
        mc_manager_ctor_field(c, 0x42c).sw(c, 0);
        c.g[A2][0] = 0x42;
        mc_manager_ctor_field(c, 0x43c).sw(c, 0);
        mc_manager_ctor_field(c, 0x440).sw(c, 0);
        // MEM_fill(&self->+0x234, 0, 0x42)
        call_ptr(c, MEM_FILL, 0x0024_2394)?;

        c.g[V1][0] = 5;
        mc_manager_ctor_field(c, 0x434).sw(c, 0);
        mc_manager_ctor_field(c, 0x33c).sw(c, c.g[V1][0]);
        c.g[V0][0] = 0x24_0000;
        mc_manager_ctor_field(c, 0x340).sw(c, 0);
        c.g[A0][0] = 0x24_0000;
        mc_manager_ctor_field(c, 0x338).sw(c, 0);
        c.g[V1][0] = 0x24_0000;
        c.g[A1][0] = 0x24_0000;
        c.g[V0][0] = sx(MC_MANAGER_CTOR_NOP);
        Addr(r32(c, GP).wrapping_sub(0x478)).sw(c, 0);
        c.g[V1][0] = sx(MC_MANAGER_CTOR_FALSE);
        c.g[A0][0] = sx(MC_MANAGER_CTOR_FALSE_2);
        c.g[A1][0] = sx(MC_MANAGER_CTOR_TRUE);
        mc_manager_ctor_field(c, 0x1c).sw(c, c.g[A0][0]);
        mc_manager_ctor_field(c, 0x34).sw(c, c.g[A1][0]);
        mc_manager_ctor_field(c, 0x438).sw(c, 0);
        for (off, reg) in MC_MANAGER_CTOR_HOOKS {
            mc_manager_ctor_field(c, off).sw(c, c.g[reg][0]);
        }
        c.g[A0][0] = c.g[S3][0];

        // The compiled slot loop (0x242488): its counter s4 starts at 0, so
        // it runs once and its back edge is never taken. It clears the slot at
        // +0x344 again (a0 still holds it; s3 moves on to +0x424 first).
        c.cycles += 7;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0xe0;
        c.g[S3][0] = sx(r32(c, S3).wrapping_add(0xe0));
        call_ptr(c, MEM_FILL, 0x0024_2498)?;
        c.g[S4][0] = sx(r32(c, S4).wrapping_sub(1));
        c.g[A0][0] = c.g[S3][0];

        c.cycles += 9;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake_call, machine, FAKE};
    use crate::difftest::{check, check_paths, fill, DATA, GAME_GP};

    // ------------------------------------------------- MCManager_State1D_Check

    /// Stand-in ids: stand-in `k` is the call target `FAKE + 0x10 * k`.
    const STATE1D_CB78: usize = 0;
    const STATE1D_CB68: usize = 1;
    const STATE1D_CB6C: usize = 2;
    const STATE1D_TICK: usize = 3;
    const STATE1D_STATUS: usize = 4;
    const STATE1D_SCANNED: usize = 5;
    const STATE1D_SINK: usize = 6;
    const STATE1D_BUSY: usize = 7;
    const STATE1D_VT1A8: usize = 8;
    const STATE1D_VT218: usize = 9;
    const STATE1D_VT208: usize = 10;
    const STATE1D_VT70: usize = 11;
    /// `MCManager_ReadCardStatus`'s backend slots 0x188, 0x198, 0x1d8,
    /// 0x68, 0xa0, 0xb8, 0x1b8, 0x1d0 (its 0x1a8 is `STATE1D_VT1A8`).
    const STATE1D_RCS: usize = 12;
    const STATE1D_STAND_INS: usize = 20;

    /// The backend vtable slots and their stand-ins.
    const STATE1D_SLOTS: [(u32, usize); 13] = [
        (0x58, STATE1D_BUSY),
        (0x1a8, STATE1D_VT1A8),
        (0x218, STATE1D_VT218),
        (0x208, STATE1D_VT208),
        (0x70, STATE1D_VT70),
        (0x188, STATE1D_RCS),
        (0x198, STATE1D_RCS + 1),
        (0x1d8, STATE1D_RCS + 2),
        (0x68, STATE1D_RCS + 3),
        (0xa0, STATE1D_RCS + 4),
        (0xb8, STATE1D_RCS + 5),
        (0x1b8, STATE1D_RCS + 6),
        (0x1d0, STATE1D_RCS + 7),
    ];

    /// `[script, calls]` per stand-in, at `STATE1D_SCRIPTS + 8 * k`.
    const STATE1D_SCRIPTS: u32 = DATA + 0x3000;
    /// Cycles every stand-in call costs.
    const STATE1D_CALL_COST: u64 = 4;

    /// The miss hook of this fixture: logs the call like [`fake_call`], then
    /// gives stand-in `k` a fixed cost and answers its n-th call with bit n
    /// of its script word, so every arm has one cycle total.
    fn state1d_stand_in(c: &mut Cpu, target: u32) {
        let at = c.cycles;
        fake_call(c, target);
        c.cycles = at + STATE1D_CALL_COST;
        let k = target.wrapping_sub(FAKE) / 0x10;
        if (k as usize) < STATE1D_STAND_INS {
            let word = STATE1D_SCRIPTS + 8 * k;
            let n = c.bus.read32(word + 4);
            c.bus.write32(word + 4, n + 1);
            c.g[V0][0] = ((c.bus.read32(word) >> n.min(31)) & 1) as u64;
        }
    }

    /// One arm: the manager words it needs (0 elsewhere), the hooks' and
    /// backend slots' script words (bit n: the n-th call's result) and the
    /// generated body's cycle total for it.
    #[derive(Clone, Copy)]
    struct State1dCase {
        /// `+0x338`, `+0xbc`, `+0xc0`.
        check: u32,
        save: u32,
        del: u32,
        /// Scripts of the `+0x78`, `+0x68` and `+0x6c` hooks.
        cb78: u32,
        cb68: u32,
        cb6c: u32,
        /// `+0x43c` before the poll; how the poll reads the card: 0 does not
        /// (`+0x42c != +0x428`, the flags stay), 1 reads flags 0x12, 2 reads 0.
        held: u32,
        card: u32,
        /// `+0x440` (non-zero: the card is not ready unless the flags change).
        status: u32,
        w11c: u32,
        w120: u32,
        w124: u32,
        /// `+0xc8`, `+0x130`, `+0xd0`, `+0xd4`.
        changed: u32,
        state: u32,
        d0: u32,
        d4: u32,
        /// Scripts of backend slots 0x1a8, 0x218 and 0x208.
        vt1a8: u32,
        vt218: u32,
        vt208: u32,
        cycles: u64,
    }

    const STATE1D_NONE: State1dCase = State1dCase {
        check: 0,
        save: 0,
        del: 0,
        cb78: 0,
        cb68: 0,
        cb6c: 0,
        held: 0,
        card: 0,
        status: 0,
        w11c: 0,
        w120: 0,
        w124: 0,
        changed: 0,
        state: 0,
        d0: 0,
        d4: 0,
        vt1a8: 0,
        vt218: 0,
        vt208: 0,
        cycles: 0,
    };

    const STATE1D_ALL: u32 = 0xffff_ffff;

    /// Every arm, by `seed % 28`.
    const STATE1D_CASES: [State1dCase; 28] = [
        // 0: the periodic check is running.
        State1dCase { check: 1, cycles: 16, ..STATE1D_NONE },
        // 1-4: flag 8, no card by +0x124, the second +0x78 call, +0x68, quit.
        State1dCase { held: 8, w124: 1, cycles: 111, ..STATE1D_NONE },
        State1dCase { held: 8, cb78: 0b100, cycles: 120, ..STATE1D_NONE },
        State1dCase { held: 8, cb68: 1, cycles: 129, ..STATE1D_NONE },
        State1dCase { held: 8, save: 1, cycles: 121, ..STATE1D_NONE },
        // 5-6: flag 8 kept: post 0x31 (6: after a card change).
        State1dCase { held: 8, cycles: 138, ..STATE1D_NONE },
        State1dCase { held: 8, changed: 1, cycles: 173, ..STATE1D_NONE },
        // 7-10: posts 0x27, 5 (+0x120 set and not), 0x17.
        State1dCase { held: 0x10, w120: 1, cycles: 105, ..STATE1D_NONE },
        State1dCase { held: 1, w120: 1, cycles: 108, ..STATE1D_NONE },
        State1dCase { held: 1, cycles: 106, ..STATE1D_NONE },
        State1dCase { held: 4, w11c: 1, w120: 1, cycles: 113, ..STATE1D_NONE },
        // 11: flags set, +0x11c without flag 4: no rescan.
        State1dCase { held: 0x10, w11c: 1, cycles: 128, ..STATE1D_NONE },
        // 12: flags 0 before and after: no rescan, then a card change.
        State1dCase { changed: 1, cycles: 163, ..STATE1D_NONE },
        // 13: flag 8 went away (8 -> 0x12): rescan.
        State1dCase { held: 8, card: 1, cycles: 228, ..STATE1D_NONE },
        // 14: flags dropped to 0 (2 -> 0): rescan.
        State1dCase { held: 2, card: 2, cycles: 254, ..STATE1D_NONE },
        // 15: the card is not ready.
        State1dCase { status: 1, cycles: 101, ..STATE1D_NONE },
        // 16: +0xc0 pending and the +0x78 hook fires: quit, the state moved on.
        State1dCase { del: 1, cb78: 1, cycles: 86, ..STATE1D_NONE },
        // 17-21: still state 0x1d with the +0x78 hook firing: post 0x1f,
        // 0x24, 0x25 (slot 0x218), nothing (slot 0x208), 0x25.
        State1dCase { del: 1, cb78: STATE1D_ALL, state: 0x1d, d0: 1, cycles: 110, ..STATE1D_NONE },
        State1dCase { cb78: STATE1D_ALL, state: 0x1d, cycles: 133, ..STATE1D_NONE },
        State1dCase { cb78: STATE1D_ALL, state: 0x1d, vt1a8: 1, cycles: 145, ..STATE1D_NONE },
        State1dCase {
            del: 1,
            cb78: STATE1D_ALL,
            state: 0x1d,
            vt1a8: 1,
            vt218: 1,
            cycles: 135,
            ..STATE1D_NONE
        },
        State1dCase {
            cb78: STATE1D_ALL,
            state: 0x1d,
            vt1a8: 1,
            vt218: 1,
            vt208: 1,
            cycles: 156,
            ..STATE1D_NONE
        },
        // 22-25: state 0x1d, +0x78 quiet: +0x68 without and with +0xd4 (slot
        // 0x70, post 0x18), +0x6c (post 1), nothing.
        State1dCase { del: 1, status: 1, cb68: 1, state: 0x1d, cycles: 107, ..STATE1D_NONE },
        State1dCase {
            save: 1,
            status: 1,
            cb68: 1,
            state: 0x1d,
            d4: 1,
            cycles: 127,
            ..STATE1D_NONE
        },
        State1dCase { save: 1, status: 1, cb6c: 1, state: 0x1d, cycles: 119, ..STATE1D_NONE },
        State1dCase { del: 1, status: 1, state: 0x1d, cycles: 112, ..STATE1D_NONE },
        // 26-27: no quit, state 0x1d: nothing fires; +0x78 fires from its
        // second call on (post 0x1f).
        State1dCase { state: 0x1d, cycles: 154, ..STATE1D_NONE },
        State1dCase { cb78: 0b110, state: 0x1d, d0: 1, cycles: 125, ..STATE1D_NONE },
    ];

    /// The manager at `DATA+0x400` (the `mgr` fixture) with every hook and
    /// backend slot a scripted stand-in, set up for arm `seed % 28`;
    /// `seed / 28 % 2` adds `this` deltas to the backend and sink slots and
    /// moves the file cursor.
    fn state1d_fixture(c: &mut Cpu, seed: u32) {
        use crate::game::movie::movieplayer::tests::mgr;
        let n = STATE1D_CASES.len() as u32;
        let case = STATE1D_CASES[(seed % n) as usize];
        let variant = seed / n % 2;
        let _ = mgr(c, seed, 0x2402d0, FAKE);
        c.hooks.miss = state1d_stand_in;
        let stand_in = |k: usize| FAKE + 0x10 * k as u32;
        let this = DATA + 0x400;
        let vt = DATA + 0x1100;
        let sink = DATA + 0x1200;
        for (off, k) in [
            (0x78, STATE1D_CB78),
            (0x68, STATE1D_CB68),
            (0x6c, STATE1D_CB6C),
            (0xa4, STATE1D_TICK),
            (0xb0, STATE1D_STATUS),
            (0xac, STATE1D_SCANNED),
        ] {
            c.bus.write32(this + off, stand_in(k));
        }
        for (off, k) in STATE1D_SLOTS {
            c.bus.write16(vt + off, (variant * 8) as u16);
            c.bus.write32(vt + off + 4, stand_in(k));
        }
        c.bus.write16(sink + 8, (variant * 0x10) as u16);
        c.bus.write32(sink + 0xc, stand_in(STATE1D_SINK));

        let mut script = [0u32; STATE1D_STAND_INS];
        script[STATE1D_CB78] = case.cb78;
        script[STATE1D_CB68] = case.cb68;
        script[STATE1D_CB6C] = case.cb6c;
        // The card read's own 0x1a8 call comes first (bit 0 = 1: flag 8 clear).
        script[STATE1D_VT1A8] = case.vt1a8 | (case.card != 0) as u32;
        script[STATE1D_VT218] = case.vt218;
        script[STATE1D_VT208] = case.vt208;
        // The card read: flag 1 = 0x188, flag 2 = 0x198 || 0x1d8, flag 4 =
        // !0x68 && 0xa0 < 0xb8, flag 0x10 = !0x1b8 || !0x1d0. Case 1 gives
        // 0x12, case 2 gives 0.
        script[STATE1D_RCS + 1] = (case.card == 1) as u32;
        script[STATE1D_RCS + 3] = 1;
        script[STATE1D_RCS + 6] = (case.card == 2) as u32;
        script[STATE1D_RCS + 7] = 1;
        for (k, bits) in script.iter().enumerate() {
            c.bus.write32(STATE1D_SCRIPTS + 8 * k as u32, *bits);
            c.bus.write32(STATE1D_SCRIPTS + 8 * k as u32 + 4, 0);
        }

        for (off, v) in [
            (0x338, case.check),
            (0xbc, case.save),
            (0xc0, case.del),
            (0x43c, case.held),
            (0x42c, (case.card == 0) as u32),
            (0x440, case.status),
            (0x11c, case.w11c),
            (0x120, case.w120),
            (0x124, case.w124),
            (0xc8, case.changed),
            (0x130, case.state),
            (0xd0, case.d0),
            (0xd4, case.d4),
            (0xf8, 2 + variant),
        ] {
            c.bus.write32(this + off, v);
        }
        c.pc = 0x0024_02d0;
    }

    #[test]
    fn mc_manager_state1d_check_matches() {
        let paths: Vec<u64> = STATE1D_CASES.iter().map(|k| k.cycles).collect();
        check_paths(
            ee::s06::f_002402d0__generated,
            mc_manager_state1d_check,
            2 * STATE1D_CASES.len() as u32,
            &paths,
            &state1d_fixture,
        );
    }

    // --------------------------------------------------------- MCManager_ctor

    #[test]
    fn mc_manager_ctor_matches() {
        check(ee::s06::f_00242288__generated, mc_manager_ctor, 32, &|c, seed| {
            let mut r = machine(c, seed, 0x242288, 0);
            // Junk over the object and around it, so every store shows.
            let this = DATA + 0x400 + (seed % 4) * 0x40;
            fill(c, &mut r, DATA + 0x300, 0x260);
            // The floats SetMode and the constructor load, and the global it
            // clears.
            fill(c, &mut r, GAME_GP.wrapping_sub(0x5130), 8);
            c.bus.write32(GAME_GP.wrapping_sub(0x478), r.next_u32() | 1);
            c.g[A0][0] = sx(this);
            c.pc = 0x0024_2288;
        });
    }
}
