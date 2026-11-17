//! Staging module oc1042 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1042`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::game::movie::movieplayer::{mcus, mcus_at, mcus_card_call, mcus_load_card};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0023_e540,
        mc_manager_state03_check,
        "MCManager_State03_Check",
    ),
    (
        0x0023_ec00,
        mc_manager_state06_check,
        "MCManager_State06_Check",
    ),
    (
        0x0023_f698,
        mc_manager_state07_check,
        "MCManager_State07_Check",
    ),
    (
        0x0024_0130,
        mc_manager_state20_load,
        "MCManager_State20_Load",
    ),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;

    pub const MC_MANAGER_POLL_CARD_STATUS: Callee = Callee {
        addr: 0x0024_1cd8,
        f: ee::s06::f_00241cd8,
    };

    pub const MC_MANAGER_SET_STATE_NO_CARD: Callee = Callee {
        addr: 0x0023_eb50,
        f: ee::s06::f_0023eb50,
    };

    pub const MC_MANAGER_RETURN_TO_SLOT_STATE: Callee = Callee {
        addr: 0x0024_2050,
        f: ee::s06::f_00242050,
    };

    pub const MC_MANAGER_READ_CURRENT_DIR_INFO: Callee = Callee {
        addr: 0x0024_1fb0,
        f: ee::s06::f_00241fb0,
    };

    pub const MC_MANAGER_FINISH_LOAD: Callee = Callee {
        addr: 0x0024_1240,
        f: ee::s06::f_00241240,
    };

    pub const MC_MANAGER_CARD_SET_STATE_1E: Callee = Callee {
        addr: 0x0023_ca70,
        f: ee::s06::f_0023ca70,
    };

    /// `UStr_Cmp(a, b)` (through the function table).
    pub const USTR_CMP: u32 = 0x003d_bdb0;
}

/// Post the code in `a1` to the manager's message sink: `v0` holds the sink
/// (`self->+0x748`); its slot 8 (`{short delta; fn}`) is called with
/// `self + delta`, `self` in `this`.
pub(crate) fn mc_check_report(c: &mut Cpu, this: usize, ret: u32) -> crate::call::Flow {
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(8)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, this).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// The "quit" test the check states share: `quit = mSave (+0xbc) || mDelete
/// (+0xc0) || the load-pressed hook (+0x78)()`, as 0/1 into `quit`. The caller
/// has loaded `v0 = self->+0xbc` and charged its cycles; leaves `v0 =
/// self->+0x128`. `self` is in `this`.
pub(crate) fn mc_check_quit(c: &mut Cpu, this: usize, quit: usize, ret: u32) -> crate::call::Flow {
    c.g[quit][0] = 0;
    'set: {
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, this), mcus::DELETE).lw(c);
            if c.g[V0][0] != 0 {
                c.g[quit][0] = 1;
                c.cycles += 1;
                break 'set;
            }
            c.cycles += 5;
            c.g[V0][0] = mcus_at(r32(c, this), mcus::CB_LOAD_PRESSED).lw(c);
            let hook = r32(c, V0);
            call_ptr(c, hook, ret)?;
            if c.g[V0][0] == 0 {
                break 'set;
            }
        }
        c.cycles += 2;
        c.g[quit][0] = 1;
    }
    c.g[V0][0] = mcus_at(r32(c, this), 0x128).lw(c);
    Ok(())
}

/// The card-ready probe of the check states: backend slots 0x1b8 and 0x1d0
/// answer non-zero, 0x198 and 0x1d8 zero, stopping at the first that does
/// not. `a1` holds the backend (`self->+0x434`) and the first call's cycles
/// are charged by the caller; each later probe reloads it. A failed probe
/// loads `v0 = self->fail` when given. `self` is in `this`; `rets` are the
/// four return addresses.
pub(crate) fn mc_check_card_ready(
    c: &mut Cpu,
    this: usize,
    rets: [u32; 4],
    fail: Option<u32>,
) -> Result<bool, Unwound> {
    let probes = [
        (mcus::VT_CAN_LOAD, true),
        (0x1d0, true),
        (0x198, false),
        (0x1d8, false),
    ];
    for (i, (slot, want)) in probes.into_iter().enumerate() {
        if i > 0 {
            c.cycles += 8;
            mcus_load_card(c, r32(c, this), A1);
        }
        mcus_card_call(c, A1, V1, V0, slot, rets[i])?;
        if (c.g[V0][0] != 0) != want {
            if let Some(off) = fail {
                c.g[V0][0] = mcus_at(r32(c, this), off).lw(c);
            }
            return Ok(false);
        }
    }
    Ok(true)
}

/// How `MCManager_State03_Check` leaves its body.
#[derive(Clone, Copy)]
enum State03End {
    /// Straight to the epilogue.
    Return,
    /// Post `a1` to the sink in `v0`.
    Report,
    /// Post 0x1d ("no card") to the sink in `v0`.
    ReportNoCard,
}

/// `void MCManager_State03_Check(MCManager *self)` (0x23e540): the card check
/// after the boot wait. Returns at once while the backend (`+0x434`, slot
/// 0x58) is busy. `quit` = save pending (`+0xbc`), delete pending (`+0xc0`) or
/// the load-pressed hook (`+0x78`). With the word at `+0x128` set, a changed
/// card (`+0xc8`) while quitting posts 0x38. Then `MCManager_PollCardStatus`;
/// when it reports a change the status flags at `+0x43c` (old flags kept in
/// `s2`) decide: flag 8 with `quit` or the word at `+0x124` →
/// `MCManager_SetStateNoCard`; flag 1 → post 5; flag 0x10 with the word at
/// `+0x120` → post 0x27; flag 8 dropped, or all flags cleared while others
/// than 4 were set → `MCManager_OnCardScanned`. Without `quit`, a changed
/// card → clear `+0xc8` and `MCManager_ReturnToSlotState`, flag 8 → post
/// 0x31. Last the card-ready probe: ready → post 6, else post 7 with a save
/// pending or 0x1d without.
pub fn mc_manager_state03_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let end = 'body: {
            mcus_load_card(c, r32(c, S0), A1);
            mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_e570)?;
            if c.g[V0][0] != 0 {
                break 'body State03End::Return;
            }
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::SAVE).lw(c);
            mc_check_quit(c, S0, S1, 0x0023_e59c)?;
            // A changed card while quitting, with +0x128 set: post 0x38.
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_CHANGED).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = 0x38;
                    if c.g[S1][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                        break 'body State03End::Report;
                    }
                    c.cycles += 1;
                }
            }
            // The flags before the poll.
            c.g[S2][0] = mcus_at(r32(c, S0), 0x43c).lw(c);
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_POLL_CARD_STATUS.call(c, 0x0023_e5dc)?;
            if c.g[V0][0] != 0 {
                // The new flags go to v1; `release` arms leave v0 = !4.
                let scan = 'flags: {
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(r32(c, S0), 0x124).lw(c);
                    let mut no_card_check = c.g[V0][0] != 0;
                    c.g[V0][0] = mcus_at(r32(c, S0), 0x43c).lw(c);
                    if !no_card_check {
                        c.cycles += 2;
                        no_card_check = c.g[S1][0] != 0;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] &= 8;
                    if no_card_check {
                        let no_card = c.g[V0][0] != 0;
                        c.g[V0][0] = c.g[V1][0] & 1;
                        if no_card {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            callee::MC_MANAGER_SET_STATE_NO_CARD.call(c, 0x0023_e610)?;
                            break 'body State03End::Return;
                        }
                    } else {
                        let no_card = c.g[V0][0] != 0;
                        c.g[V0][0] = !4;
                        if no_card {
                            break 'flags false;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[V1][0] & 1;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 5;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                        break 'body State03End::Report;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(r32(c, S0), 0x120).lw(c);
                    let skip = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] & 0x10;
                    if !skip {
                        c.cycles += 2;
                        c.g[A1][0] = 0x27;
                        if c.g[V0][0] != 0 {
                            c.cycles += 2;
                            c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                            break 'body State03End::Report;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 8;
                    let no_card = c.g[V0][0] != 0;
                    c.g[V0][0] = !4;
                    if no_card {
                        break 'flags false;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & 8;
                    let lost = c.g[V0][0] != 0;
                    c.g[V0][0] = !4;
                    lost
                };
                // `release`: no new flags while some old ones besides 4 were set.
                let scan = scan || {
                    c.cycles += 2;
                    let any = c.g[V1][0] != 0;
                    c.g[V0][0] &= c.g[S2][0];
                    if any {
                        false
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] != 0
                    }
                };
                if scan {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_ON_CARD_SCANNED.call(c, 0x0023_e688)?;
                    break 'body State03End::Return;
                }
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_CHANGED).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    mcus_at(r32(c, S0), mcus::CARD_CHANGED).sw(c, 0);
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_RETURN_TO_SLOT_STATE.call(c, 0x0023_e6b0)?;
                    break 'body State03End::Return;
                }
                c.g[V0][0] = mcus_at(r32(c, S0), 0x43c).lw(c);
                c.cycles += 3;
                c.g[V0][0] &= 8;
                c.g[A1][0] = 0x31;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                    break 'body State03End::Report;
                }
                c.cycles += 1;
            }
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::SAVE).lw(c);
            c.cycles += 2;
            let saving = c.g[V0][0] != 0;
            mcus_load_card(c, r32(c, S0), A1);
            c.cycles += 7;
            if saving {
                let rets = [0x0023_e6ec, 0x0023_e70c, 0x0023_e72c, 0x0023_e74c];
                if mc_check_card_ready(c, S0, rets, Some(mcus::MSG))? {
                    c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                    c.cycles += 1;
                    c.g[A1][0] = 6;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 7;
                }
                break 'body State03End::Report;
            }
            let rets = [0x0023_e770, 0x0023_e790, 0x0023_e7b0, 0x0023_e7d0];
            if mc_check_card_ready(c, S0, rets, Some(mcus::MSG))? {
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                c.cycles += 1;
                c.g[A1][0] = 6;
                break 'body State03End::Report;
            }
            State03End::ReportNoCard
        };
        match end {
            State03End::Return => {}
            State03End::Report => {
                c.cycles += 6;
                mc_check_report(c, S0, 0x0023_e7ec)?;
            }
            State03End::ReportNoCard => {
                c.cycles += 6;
                c.g[A1][0] = 0x1d;
                mc_check_report(c, S0, 0x0023_e808)?;
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// How `MCManager_State06_Check` leaves its body (`s0` is reloaded on every
/// way out but `Return`, which reloaded it before).
#[derive(Clone, Copy)]
enum State06End {
    /// Straight to the epilogue.
    Return,
    /// Still counting down: one more cycle.
    Wait,
    /// Post `a1` to the sink in `v0`.
    Report,
    /// `MCManager_SetStateNoCard`.
    NoCard,
    /// Reread the slot's directory and post 0x1d.
    Fail,
}

/// `void MCManager_State06_Check(MCManager *self)` (0x23ec00): the card check
/// after a directory listing. Polls the card (`MCManager_PollCardStatus`,
/// kept in `s2`), counts the wait at `+0x108` down and returns while it was
/// above 0 or the backend (`+0x434`, slot 0x58) is busy. A changed card
/// (`+0xc8`): `quit` (save, delete pending or the load-pressed hook at
/// `+0x78`) with the word at `+0x128` set posts 0x38; a ready card (the
/// probe) posts 6. With a save pending (`+0xbc`) the flags at `+0x43c`
/// decide: flag 8 → `MCManager_SetStateNoCard` when the poll changed, else
/// the slot's directory is reread and 7 posted; flag 1 → 0x14; a ready card
/// that the backend says has no room (slot 0x68 zero, slot 0xa0 below slot
/// 0xb8 or slot 0xc8 zero) → 0x15; otherwise reread and post 7. Without a
/// save pending: flag 8 → `MCManager_SetStateNoCard` when quitting or the
/// word at `+0x124` is set; flag 1 → 0x24 or 5 by the load-pressed hook;
/// else reread the directory and post 0x1d.
pub fn mc_manager_state06_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::MC_MANAGER_POLL_CARD_STATUS.call(c, 0x0023_ec1c)?;
        c.g[S2][0] = c.g[V0][0];
        let end = 'body: {
            let wait = mcus_at(r32(c, S1), 0x108);
            c.g[V0][0] = wait.lw(c);
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let waiting = (c.g[V1][0] as i64) > 0;
            let left = c.g[V0][0];
            wait.sw(c, left);
            if waiting {
                break 'body State06End::Wait;
            }
            c.cycles += 8;
            mcus_load_card(c, r32(c, S1), A1);
            mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_ec4c)?;
            let busy = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x30);
            if busy {
                break 'body State06End::Return;
            }
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::CARD_CHANGED).lw(c);
            let changed = c.g[V0][0] != 0;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::SAVE).lw(c);
            if changed {
                c.cycles += 2;
                mc_check_quit(c, S1, S0, 0x0023_ec80)?;
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = 0x38;
                    if c.g[S0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                        break 'body State06End::Report;
                    }
                    c.cycles += 1;
                }
                mcus_load_card(c, r32(c, S1), A1);
                c.cycles += 7;
                let rets = [0x0023_ecc0, 0x0023_ece0, 0x0023_ed00, 0x0023_ed20];
                if mc_check_card_ready(c, S1, rets, Some(mcus::SAVE))? {
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    c.g[A1][0] = 6;
                    break 'body State06End::Report;
                }
            }
            // v0 = the save-pending word.
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                'reread: {
                    c.cycles += 4;
                    c.g[V1][0] = mcus_at(r32(c, S1), 0x43c).lw(c);
                    c.g[V0][0] = c.g[V1][0] & 8;
                    let no_card = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[V1][0] & 1;
                    if no_card {
                        c.cycles += 2;
                        if c.g[S2][0] != 0 {
                            break 'body State06End::NoCard;
                        }
                        break 'reread;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0x14;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                        break 'body State06End::Report;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, r32(c, S1), A1);
                    let rets = [0x0023_ed74, 0x0023_ed94, 0x0023_edb4, 0x0023_edd4];
                    if !mc_check_card_ready(c, S1, rets, None)? {
                        break 'reread;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, r32(c, S1), A1);
                    mcus_card_call(c, A1, V1, V0, 0x68, 0x0023_edf4)?;
                    if c.g[V0][0] != 0 {
                        break 'reread;
                    }
                    // No room: slot 0xa0 below slot 0xb8, or slot 0xc8 zero.
                    c.cycles += 16;
                    mcus_load_card(c, r32(c, S1), A1);
                    mcus_card_call(c, A1, V1, V0, 0xb8, 0x0023_ee14)?;
                    mcus_load_card(c, r32(c, S1), A1);
                    c.g[S0][0] = c.g[V0][0];
                    mcus_card_call(c, A1, V1, V0, 0xa0, 0x0023_ee30)?;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    } else {
                        c.cycles += 8;
                        mcus_load_card(c, r32(c, S1), A1);
                        mcus_card_call(c, A1, V1, V0, 0xc8, 0x0023_ee54)?;
                        if c.g[V0][0] != 0 {
                            break 'reread;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0x15;
                    break 'body State06End::Report;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                callee::MC_MANAGER_READ_CURRENT_DIR_INFO.call(c, 0x0023_ee78)?;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                c.g[A1][0] = 7;
                break 'body State06End::Report;
            }
            // No save pending: `quit` = delete pending or the hook.
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::DELETE).lw(c);
            c.cycles += 2;
            let mut pending = c.g[V0][0] != 0;
            c.g[S0][0] = 0;
            if !pending {
                c.cycles += 5;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_LOAD_PRESSED).lw(c);
                let hook = r32(c, V0);
                call_ptr(c, hook, 0x0023_ee98)?;
                pending = c.g[V0][0] != 0;
            }
            if pending {
                c.cycles += 2;
                c.g[S0][0] = 1;
            }
            c.g[V0][0] = mcus_at(r32(c, S1), 0x124).lw(c);
            c.cycles += 2;
            let mut no_card_check = c.g[V0][0] != 0;
            c.g[V0][0] = mcus_at(r32(c, S1), 0x43c).lw(c);
            if !no_card_check {
                c.cycles += 2;
                no_card_check = c.g[S0][0] != 0;
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] &= 8;
            let no_card = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[V1][0] & 1;
            if no_card {
                if no_card_check {
                    break 'body State06End::NoCard;
                }
                break 'body State06End::Fail;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break 'body State06End::Fail;
            }
            c.cycles += 5;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_LOAD_PRESSED).lw(c);
            let hook = r32(c, V0);
            call_ptr(c, hook, 0x0023_eefc)?;
            let released = c.g[V0][0] == 0;
            c.g[A1][0] = 0x24;
            if !released {
                c.cycles += 2;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                break 'body State06End::Report;
            }
            c.cycles += 2;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
            c.g[A1][0] = 5;
            State06End::Report
        };
        match end {
            State06End::Return => {}
            State06End::Wait => {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
            State06End::Report => {
                c.cycles += 6;
                mc_check_report(c, S1, 0x0023_ef24)?;
                frame.restore(c, S0, 0x30);
            }
            State06End::NoCard => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                callee::MC_MANAGER_SET_STATE_NO_CARD.call(c, 0x0023_eed0)?;
                frame.restore(c, S0, 0x30);
            }
            State06End::Fail => {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                callee::MC_MANAGER_READ_CURRENT_DIR_INFO.call(c, 0x0023_ef34)?;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                c.g[A1][0] = 0x1d;
                mc_check_report(c, S1, 0x0023_ef4c)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// How `MCManager_State07_Check` leaves its body.
#[derive(Clone, Copy)]
enum State07End {
    /// Straight to the epilogue (`s0` already reloaded).
    Return,
    /// Post `a1` to the sink in `v0`.
    Report,
    /// The delete-pressed tail (`v0` holds the hook at `+0x68`).
    DeleteCheck,
}

/// `void MCManager_State07_Check(MCManager *self)` (0x23f698): the save/load
/// screen's card check. Nothing while a periodic card poll (`+0x338`) runs.
/// The word at `+0x128` with a changed card (`+0xc8`) posts 0x38. Then
/// `MCManager_PollCardStatus` (kept in `s0`); with the save-pressed hook
/// (`+0x64`) quiet and a changed status the flags at `+0x43c` decide: 8 →
/// `MCManager_SetStateNoCard`, 0x10 → 0x27, 2 → 0x30, 1 → 5, 4 → 0x17. A
/// changed card → clear `+0xc8` and `MCManager_ReturnToSlotState`; the hook
/// at `+0x6c` → post 1. Save pressed: the floats at `+0x110`/`+0x114` are set
/// (1.0 and the constant at `gp-0x5124`); with saving allowed (`+0xcc`): a
/// card not ready (the probe) → 0xa; no room for a new file (backend slot
/// 0x218 for the cursor `+0xf8` zero, then slot 0xa0 below slot 0xb8 or slot
/// 0xc8 zero) → 0x15; a used slot among 0..5 other than the cursor whose name
/// (slot 0x70, into the 0x200-byte stack buffer) equals the one at `+0x276`
/// → 0x37; the cursor's slot used → its name into `+0x234`, post 9; else
/// 0xd. Without saving allowed: backend slot 0x208 → 0x16, slot 0x210 →
/// 0x17, slot 0x1a8 zero → 0x14. Last, the delete-pressed hook (`+0x68`)
/// with deleting allowed (`+0xd4`): the cursor's name into `+0x234`, post
/// 0x18.
pub fn mc_manager_state07_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x230);
        frame.save(c, S1, 0x210);
        frame.save(c, S0, 0x220);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x200);
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::CHECK_CARD).lw(c);
        let checking = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x220);
        if !checking {
            let end = mc_state07_body(c, frame)?;
            match end {
                State07End::Return => {}
                State07End::Report => {
                    c.cycles += 6;
                    mc_check_report(c, S1, 0x0023_fa34)?;
                    frame.restore(c, S0, 0x220);
                }
                State07End::DeleteCheck => {
                    c.cycles += 4;
                    let hook = r32(c, V0);
                    call_ptr(c, hook, 0x0023_fa80)?;
                    let pressed = c.g[V0][0] != 0;
                    frame.restore(c, S0, 0x220);
                    if pressed {
                        c.cycles += 3;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::DELETE_OK).lw(c);
                        let allowed = c.g[V0][0] != 0;
                        c.g[A2][0] = sx(r32(c, S1).wrapping_add(0x234));
                        if allowed {
                            c.cycles += 14;
                            mcus_load_card(c, r32(c, S1), A3);
                            c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                            mcus_card_call(c, A3, V0, V1, 0x70, 0x0023_fab0)?;
                            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                            c.g[A1][0] = 0x18;
                            mc_check_report(c, S1, 0x0023_fac8)?;
                            frame.restore(c, S0, 0x220);
                        }
                    }
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// [`mc_manager_state07_check`] after the card-poll test: returns how it ends.
fn mc_state07_body(c: &mut Cpu, frame: Frame) -> Result<State07End, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = mcus_at(r32(c, S1), 0x128).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::CARD_CHANGED).lw(c);
        let changed = c.g[V0][0] != 0;
        c.g[A1][0] = 0x38;
        if changed {
            c.cycles += 2;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
            return Ok(State07End::Report);
        }
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S1][0];
    callee::MC_MANAGER_POLL_CARD_STATUS.call(c, 0x0023_f6e0)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_SAVE_PRESSED).lw(c);
    let hook = r32(c, V0);
    call_ptr(c, hook, 0x0023_f6f0)?;
    'flags: {
        if c.g[V0][0] != 0 {
            break 'flags;
        }
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            break 'flags;
        }
        c.cycles += 4;
        c.g[V1][0] = mcus_at(r32(c, S1), 0x43c).lw(c);
        c.g[V0][0] = c.g[V1][0] & 8;
        let no_card = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0x10;
        if no_card {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            callee::MC_MANAGER_SET_STATE_NO_CARD.call(c, 0x0023_f718)?;
            frame.restore(c, S0, 0x220);
            return Ok(State07End::Return);
        }
        c.cycles += 2;
        c.g[A1][0] = 0x27;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
            return Ok(State07End::Report);
        }
        for (flag, code) in [(2, 0x30), (1, 5), (4, 0x17)] {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & flag;
            c.g[A1][0] = code;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                return Ok(State07End::Report);
            }
        }
        c.cycles += 1;
    }
    c.g[V0][0] = mcus_at(r32(c, S1), mcus::CARD_CHANGED).lw(c);
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        mcus_at(r32(c, S1), mcus::CARD_CHANGED).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::MC_MANAGER_RETURN_TO_SLOT_STATE.call(c, 0x0023_f784)?;
        frame.restore(c, S0, 0x220);
        return Ok(State07End::Return);
    }
    c.g[V0][0] = mcus_at(r32(c, S1), 0x6c).lw(c);
    c.cycles += 4;
    let hook = r32(c, V0);
    call_ptr(c, hook, 0x0023_f794)?;
    let quiet = c.g[V0][0] == 0;
    c.g[A1][0] = 1;
    if !quiet {
        c.cycles += 2;
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
        return Ok(State07End::Report);
    }
    c.cycles += 5;
    c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_SAVE_PRESSED).lw(c);
    let hook = r32(c, V0);
    call_ptr(c, hook, 0x0023_f7b0)?;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_DELETE_PRESSED).lw(c);
        return Ok(State07End::DeleteCheck);
    }
    'name_clash: {
        'post: {
            'not_allowed: {
                c.cycles += 7;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5124));
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::SAVE_OK).lw(c);
                c.swc1(0, r32(c, S1).wrapping_add(0x110));
                let allowed = c.g[V0][0] != 0;
                c.swc1(1, r32(c, S1).wrapping_add(0x114));
                if !allowed {
                    break 'not_allowed;
                }
                'not_ready: {
                    c.cycles += 8;
                    mcus_load_card(c, r32(c, S1), A1);
                    let rets = [0x0023_f7ec, 0x0023_f80c, 0x0023_f82c, 0x0023_f84c];
                    if !mc_check_card_ready(c, S1, rets, Some(mcus::MSG))? {
                        break 'not_ready;
                    }
                    // Room for a new file: the cursor's slot used, or slot
                    // 0xa0 not below slot 0xb8 and slot 0xc8 non-zero.
                    c.cycles += 9;
                    mcus_load_card(c, r32(c, S1), A2);
                    c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x218, 0x0023_f870)?;
                    let used = c.g[V0][0] != 0;
                    c.g[S0][0] = 0;
                    if !used {
                        c.cycles += 16;
                        mcus_load_card(c, r32(c, S1), A1);
                        mcus_card_call(c, A1, V1, V0, 0xb8, 0x0023_f890)?;
                        mcus_load_card(c, r32(c, S1), A1);
                        c.g[S0][0] = c.g[V0][0];
                        mcus_card_call(c, A1, V1, V0, 0xa0, 0x0023_f8ac)?;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                        let full = if c.g[V0][0] != 0 {
                            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                            true
                        } else {
                            c.cycles += 8;
                            mcus_load_card(c, r32(c, S1), A1);
                            mcus_card_call(c, A1, V1, V0, 0xc8, 0x0023_f8d0)?;
                            let room = c.g[V0][0] != 0;
                            c.g[S0][0] = 0;
                            if !room {
                                c.cycles += 1;
                                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                            }
                            !room
                        };
                        if full {
                            c.cycles += 2;
                            c.g[A1][0] = 0x15;
                            return Ok(State07End::Report);
                        }
                        c.cycles += 1;
                    }
                    c.cycles += 1;
                    mcus_load_card(c, r32(c, S1), A2);
                    // Slots 0..5 (s0): a used one other than the cursor with
                    // the same name clashes.
                    loop {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[S0][0];
                        mcus_card_call(c, A2, V1, V0, 0x218, 0x0023_f904)?;
                        let free = c.g[V0][0] == 0;
                        c.g[A1][0] = c.g[S0][0];
                        // A free slot, or the cursor's own name: one cycle more.
                        let mut next = free;
                        if !free {
                            c.cycles += 12;
                            mcus_load_card(c, r32(c, S1), A3);
                            c.g[A2][0] = c.g[SP][0];
                            mcus_card_call(c, A3, V0, V1, 0x70, 0x0023_f928)?;
                            c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x276));
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, callee::USTR_CMP, 0x0023_f934)?;
                            if c.g[V0][0] == 0 {
                                c.cycles += 3;
                                c.g[V0][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                                if c.g[S0][0] != c.g[V0][0] {
                                    c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                                    break 'name_clash;
                                }
                                next = true;
                            }
                        }
                        if next {
                            c.cycles += 1;
                        }
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < 6) as u64;
                        let more = c.g[V0][0] != 0;
                        mcus_load_card(c, r32(c, S1), A2);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_f8ec);
                        }
                    }
                    c.cycles += 8;
                    c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x218, 0x0023_f970)?;
                    let free = c.g[V0][0] == 0;
                    c.g[A2][0] = sx(r32(c, S1).wrapping_add(0x234));
                    if !free {
                        c.cycles += 10;
                        mcus_load_card(c, r32(c, S1), A3);
                        c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                        mcus_card_call(c, A3, V0, V1, 0x70, 0x0023_f994)?;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                        c.g[A1][0] = 9;
                        return Ok(State07End::Report);
                    }
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    c.g[A1][0] = 0xd;
                    break 'post;
                }
                c.cycles += 2;
                c.g[A1][0] = 0xa;
                return Ok(State07End::Report);
            }
            // Saving not allowed.
            c.cycles += 9;
            mcus_load_card(c, r32(c, S1), A2);
            c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
            mcus_card_call(c, A2, V1, V0, 0x208, 0x0023_f9d0)?;
            let quiet = c.g[V0][0] == 0;
            c.g[A1][0] = 0x16;
            if !quiet {
                c.cycles += 2;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                break 'post;
            }
            c.cycles += 9;
            mcus_load_card(c, r32(c, S1), A2);
            c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
            mcus_card_call(c, A2, V1, V0, 0x210, 0x0023_f9fc)?;
            let quiet = c.g[V0][0] == 0;
            c.g[A1][0] = 0x17;
            if quiet {
                c.cycles += 8;
                mcus_load_card(c, r32(c, S1), A1);
                mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_SAVE, 0x0023_fa54)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 7;
                    c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    c.g[A1][0] = 0x14;
                    mc_check_report(c, S1, 0x0023_fa74)?;
                }
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_DELETE_PRESSED).lw(c);
                return Ok(State07End::DeleteCheck);
            }
            c.cycles += 1;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
        }
        c.cycles += 6;
        mc_check_report(c, S1, 0x0023_fa18)?;
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_DELETE_PRESSED).lw(c);
        return Ok(State07End::DeleteCheck);
    }
    c.cycles += 1;
    c.g[A1][0] = 0x37;
    Ok(State07End::Report)
}

/// `void MCManager_State20_Load(MCManager *self)` (0x240130): one step of a
/// load. Counts the wait at `+0x108` down and stores the progress (backend
/// slot 0x90 over the word at `+0xf0`, as floats) at `+0x10c`. A finished
/// backend (slot 0x248) posts 0x24 and runs the hook at `+0x2c`. Otherwise,
/// when backend slot 0xf8 answers 1: while `+0xe8` is below `+0xf0` the hook
/// at `+0x1c` reads the rest (`+0xf0 - +0xe8`) and `MCManager_FinishLoad`
/// follows when it returns 0; else the hook at `+0x20` returning 0 → backend
/// slot 0x140 for the cursor `+0xf8`, then `MCManager_FinishLoad`; non-zero
/// → `MCManager_CardSetState1E` and post 0x21. Any other answer: when the
/// backend has finished meanwhile, run the hook at `+0x2c` and post 0x24.
pub fn mc_manager_state20_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let wait = mcus_at(r32(c, S0), 0x108);
        c.g[V0][0] = wait.lw(c);
        mcus_load_card(c, r32(c, S0), A1);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let left = c.g[V0][0];
        wait.sw(c, left);
        mcus_card_call(c, A1, V1, V0, 0x90, 0x0024_0164)?;
        // Progress = slot 0x90 / +0xf0.
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        mcus_load_card(c, r32(c, S0), A1);
        c.lwc1(1, r32(c, S0).wrapping_add(0xf0));
        c.cvt_s_w(1, 1);
        c.div_s(0, 0, 1);
        c.swc1(0, r32(c, S0).wrapping_add(0x10c));
        mcus_card_call(c, A1, V1, V0, 0x248, 0x0024_0194)?;
        let finished = c.g[V0][0] != 0;
        c.g[A1][0] = 0x24;
        if finished {
            c.cycles += 10;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
            mc_check_report(c, S0, 0x0024_01b0)?;
            c.g[V0][0] = mcus_at(r32(c, S0), 0x2c).lw(c);
            let hook = r32(c, V0);
            call_ptr(c, hook, 0x0024_01bc)?;
        } else {
            'other: {
                c.cycles += 9;
                mcus_load_card(c, r32(c, S0), A1);
                mcus_card_call(c, A1, V1, V0, 0xf8, 0x0024_01dc)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] != c.g[V1][0] {
                    mcus_load_card(c, r32(c, S0), A1);
                    break 'other;
                }
                'set_1e: {
                    'finish: {
                        c.cycles += 5;
                        c.g[V1][0] = mcus_at(r32(c, S0), 0xe8).lw(c);
                        c.g[A0][0] = mcus_at(r32(c, S0), 0xf0).lw(c);
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.cycles += 5;
                            c.g[V0][0] = mcus_at(r32(c, S0), 0x1c).lw(c);
                            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
                            let hook = r32(c, V0);
                            call_ptr(c, hook, 0x0024_0208)?;
                            if c.g[V0][0] != 0 {
                                frame.restore(c, S0, 0x10);
                                mc_state20_epilogue(c, frame);
                                return Ok(());
                            }
                            c.cycles += 2;
                            break 'finish;
                        }
                        c.g[V0][0] = mcus_at(r32(c, S0), 0x20).lw(c);
                        c.cycles += 4;
                        let hook = r32(c, V0);
                        call_ptr(c, hook, 0x0024_0220)?;
                        if c.g[V0][0] != 0 {
                            break 'set_1e;
                        }
                        c.cycles += 7;
                        mcus_load_card(c, r32(c, S0), A2);
                        c.g[A1][0] = mcus_at(r32(c, S0), mcus::FILE_CURSOR).lw(c);
                        mcus_card_call(c, A2, V0, V1, 0x140, 0x0024_0244)?;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_FINISH_LOAD.call(c, 0x0024_024c)?;
                    frame.restore(c, S0, 0x10);
                    mc_state20_epilogue(c, frame);
                    return Ok(());
                }
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_CARD_SET_STATE_1E.call(c, 0x0024_025c)?;
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                c.g[A1][0] = 0x21;
                mc_check_report(c, S0, 0x0024_0274)?;
                frame.restore(c, S0, 0x10);
                mc_state20_epilogue(c, frame);
                return Ok(());
            }
            // a1 = the backend: finished by now?
            c.cycles += 7;
            mcus_card_call(c, A1, V1, V0, 0x248, 0x0024_0290)?;
            if c.g[V0][0] != 0 {
                c.cycles += 10;
                c.g[V0][0] = mcus_at(r32(c, S0), 0x2c).lw(c);
                let hook = r32(c, V0);
                call_ptr(c, hook, 0x0024_02a4)?;
                c.g[V1][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                c.g[A1][0] = 0x24;
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(8)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0024_02bc)?;
            }
        }
        frame.restore(c, S0, 0x10);
        mc_state20_epilogue(c, frame);
        Ok(())
    })
}

/// [`mc_manager_state20_load`]'s epilogue: `ra`, pop the 0x20 frame, return.
fn mc_state20_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check_paths;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::{Rng, DATA};
    use crate::game::movie::movieplayer::tests::{mgr, ret_slot, w32, RET_ONE, RET_ZERO};

    /// The check states' manager at `DATA+0x400`: `mgr`, with every backend
    /// slot below 0x260 answering 0 (a real leaf), the message sink moved off
    /// the backend vtable (the `mgr` sink vtable sits on its slots 0x100..)
    /// to `DATA+0x1800` with slot 8 calling `sink`, and the manager hooks the
    /// handlers and their callees call answering 0.
    pub(crate) fn mc_check_mgr(c: &mut Cpu, seed: u32, salt: u64, sink: u32) -> Rng {
        let r = mgr(c, seed, salt, RET_ZERO);
        let this = DATA + 0x400;
        let vt = DATA + 0x1100;
        for off in (0..0x260).step_by(8) {
            ret_slot(c, vt, off, RET_ZERO);
        }
        let msg = DATA + 0x1800;
        w32(c, this + 0x748, msg);
        ret_slot(c, msg, 8, sink);
        for hook in [0x1c, 0x20, 0x2c, 0x64, 0x68, 0x6c, 0x78, 0xac, 0xb0] {
            w32(c, this + hook, RET_ZERO);
        }
        r
    }

    /// The card-ready probe answers for `rd`: fail at the first, second, third
    /// or fourth probe, or (4) ready. `one` is the non-zero answer.
    pub(crate) fn mc_check_ready_slots(c: &mut Cpu, rd: u32, one: u32) {
        let vt = DATA + 0x1100;
        let [a, b, x, y] = match rd {
            0 => [RET_ZERO, one, RET_ZERO, RET_ZERO],
            1 => [one, RET_ZERO, RET_ZERO, RET_ZERO],
            2 => [one, one, one, RET_ZERO],
            3 => [one, one, RET_ZERO, one],
            _ => [one, one, RET_ZERO, RET_ZERO],
        };
        ret_slot(c, vt, 0x1b8, a);
        ret_slot(c, vt, 0x1d0, b);
        ret_slot(c, vt, 0x198, x);
        ret_slot(c, vt, 0x1d8, y);
    }

    /// Cycle totals of the arms (deterministic seeds, all callees real leaves):
    /// busy; post 0x38 (quit by +0xc0, by the hook, by +0xbc); ReturnToSlotState
    /// after +0x128; SetStateNoCard (quit, +0x124); post 5 (quit, no quit);
    /// post 0x27; release with flags 0x10 and 4; post 0x31; release with no
    /// flags; OnCardScanned (flag 8 lost, all flags cleared); poll false after
    /// a change; ReturnToSlotState; the save-pending tail (four probe failures,
    /// ready); the no-save tail (four failures, ready).
    const STATE03_PATHS: &[u64] = &[
        21, 45, 53, 43, 120, 99, 103, 94, 101, 106, 130, 128, 108, 132, 214, 227, 186, 114, 92,
        102, 112, 122, 121, 95, 105, 115, 125, 126,
    ];

    #[test]
    fn mc_manager_state03_check_matches() {
        check_paths(
            ee::s06::f_0023e540__generated,
            mc_manager_state03_check,
            180,
            STATE03_PATHS,
            &|c, seed| {
                // Row (the arm), d (quit kind and probe answers), fakes (FAKE
                // for the non-zero answers and the sink: logged calls).
                let row = seed % 18;
                let d = seed / 18 % 5;
                let fakes = seed / 90 % 2 == 1;
                let one = if fakes { FAKE } else { RET_ONE };
                let _ = mc_check_mgr(c, seed, 0x23e540, if fakes { FAKE } else { RET_ZERO });
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                // Quit kinds: 0 none, 1 delete pending, 2 the load-pressed
                // hook, 3 save pending.
                let free = d % 4;
                let any = 1 + d % 3;
                // (quit, +0x128, +0xc8, +0x124, +0x120, flags, +0x440, computed
                // flags, poll hook answers 1)
                let (q, f128, c8, f124, f120, flags, f440, computed, hook_one) = match row {
                    0 => (free, 0, 0, 0, 0, 0, 0, false, false),
                    1 => (any, 1, 1, 0, 0, 0, 0, false, false),
                    2 => (0, 1, 1, 0, 0, 0, 1, false, false),
                    3 => (any, 0, 0, 0, 0, 8, 0, false, false),
                    4 => (0, 0, 0, 1, 0, 9, 0, false, false),
                    5 => (any, 0, 0, 0, 0, 1, 0, false, false),
                    6 => (0, 0, 0, 0, 0, 1, 0, false, false),
                    7 => (free, 0, 0, 0, 1, 0x10, 0, false, false),
                    8 => (free, 0, 0, 0, 0, 0x10, 0, false, false),
                    9 => (free, 0, 0, 0, 1, 4, 0, false, false),
                    10 => (0, 0, 0, 0, 0, 8, 0, false, false),
                    11 => (free, 0, 0, 0, 0, 0, 0, false, false),
                    12 => (free, 0, 0, 0, 0, 8, 0, true, false),
                    13 => (free, 0, 0, 0, 0, 1, 0, true, false),
                    14 => (free, 0, 0, 0, 0, 8, 0, true, true),
                    15 => (0, 0, 1, 0, 0, 0, 1, false, false),
                    16 => (3, 0, 0, 0, 0, 0, 1, false, false),
                    _ => (1, 1, 0, 0, 0, 0, 1, false, false),
                };
                if row == 0 {
                    ret_slot(c, vt, 0x58, one);
                }
                match q {
                    1 => w32(c, this + 0xc0, 1),
                    2 => w32(c, this + 0x78, one),
                    3 => w32(c, this + 0xbc, 1),
                    _ => {}
                }
                w32(c, this + 0x128, f128);
                w32(c, this + 0xc8, c8);
                w32(c, this + 0x124, f124);
                w32(c, this + 0x120, f120);
                w32(c, this + 0x43c, flags);
                w32(c, this + 0x440, f440);
                // ReadCardStatus keeps the old flags unless the current check
                // slot (+0x42c) is the selected one (+0x428).
                w32(c, this + 0x428, 0);
                w32(c, this + 0x42c, if computed { 0 } else { 1 });
                // Computed flags: bit 0 from slot 0x188, 3 from !0x1a8, 2 from
                // !0x68 (here 0); bits 1 and 4 follow the probe answers.
                ret_slot(c, vt, 0x188, RET_ZERO);
                ret_slot(c, vt, 0x1a8, RET_ONE);
                ret_slot(c, vt, 0x68, one);
                // Row 13 needs all computed flags clear: the "ready" answers.
                mc_check_ready_slots(c, if row == 13 { 4 } else { d }, one);
                if hook_one {
                    w32(c, this + 0xb0, RET_ONE);
                }
                c.pc = 0x0023_e540;
            },
        );
    }

    /// Cycle totals of the arms (deterministic seeds, all callees real leaves,
    /// the directory reread at slot 1): still waiting; busy; post 0x38 (quit by
    /// +0xc0, the hook, +0xbc); a changed card that is ready (post 6) or not
    /// (on to the no-save failure, the save path); save pending: SetStateNoCard,
    /// flag 8 without a change, post 0x14, the probe failing four ways or slot
    /// 0x68 set, no room by slot 0xa0, slot 0xc8 set, no room by slot 0xc8;
    /// no save pending: SetStateNoCard (delete pending, the hook, +0x124), flag
    /// 8 without quit, post 5, post 0x24, no flags.
    const STATE06_PATHS: &[u64] = &[
        58, 67, 90, 98, 88, 137, 300, 136, 286, 328, 95, 261, 88, 273, 283, 293, 303, 313, 158,
        343, 169, 101, 108, 104, 272, 103, 110, 274,
    ];

    #[test]
    fn mc_manager_state06_check_matches() {
        check_paths(
            ee::s06::f_0023ec00__generated,
            mc_manager_state06_check,
            190,
            STATE06_PATHS,
            &|c, seed| {
                let row = seed % 19;
                let d = seed / 19 % 5;
                let fakes = seed / 95 % 2 == 1;
                let one = if fakes { FAKE } else { RET_ONE };
                let _ = mc_check_mgr(c, seed, 0x23ec00, if fakes { FAKE } else { RET_ZERO });
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                let any = 1 + d % 3;
                // (quit kind as in State03, +0xc8, +0x128, +0x124, flags,
                // +0x440, ready probe answers)
                let (q, c8, f128, f124, flags, f440, rd) = match row {
                    0 | 1 => (0, 0, 0, 0, 0, 0, d),
                    2 => (any, 1, 1, 0, 0, 0, d),
                    3 => (0, 1, 1, 0, 0, 0, d),
                    4 => (any, 1, 0, 0, 0, 0, d),
                    5 => (3, 0, 0, 0, 8, 0, d),
                    6 => (3, 0, 0, 0, 8, 1, d),
                    7 => (3, 0, 0, 0, 1, 0, d),
                    8 => (3, 0, 0, 0, 0, 0, d),
                    9..=11 => (3, 0, 0, 0, 0, 0, 4),
                    12 => (1, 0, 0, 0, 8, 0, d),
                    13 => (2, 0, 0, 0, 8, 0, d),
                    14 => (0, 0, 0, 1, 8, 0, d),
                    15 => (0, 0, 0, 0, 8, 0, d),
                    16 => (d % 2, 0, 0, 0, 1, 0, d),
                    17 => (2, 0, 0, 0, 1, 0, d),
                    _ => (d % 2, 0, 0, 0, 0, 0, d),
                };
                // The wait above 0 for row 0; 0 or below elsewhere.
                w32(c, this + 0x108, if row == 0 { 1 } else { 0u32.wrapping_sub(d % 2) });
                if row == 1 {
                    ret_slot(c, vt, 0x58, one);
                }
                match q {
                    1 => w32(c, this + 0xc0, 1),
                    2 => w32(c, this + 0x78, one),
                    3 => w32(c, this + 0xbc, 1),
                    _ => {}
                }
                w32(c, this + 0xc8, c8);
                w32(c, this + 0x128, f128);
                w32(c, this + 0x124, f124);
                w32(c, this + 0x43c, flags);
                w32(c, this + 0x440, f440);
                // The poll keeps the flags; the directory reread is slot 1.
                w32(c, this + 0x428, 0);
                w32(c, this + 0x42c, 1);
                mc_check_ready_slots(c, rd, one);
                // Slot 0x68 set (row 8), or the room test: 0xa0 below 0xb8
                // (row 9), not below with slot 0xc8 set (10) or zero (11).
                ret_slot(c, vt, 0x68, if row == 8 { one } else { RET_ZERO });
                ret_slot(c, vt, 0xb8, if row == 9 { RET_ONE } else { RET_ZERO });
                ret_slot(c, vt, 0xc8, if row == 10 { one } else { RET_ZERO });
                c.pc = 0x0023_ec00;
            },
        );
    }

    /// The State07 fixture: `row` (the arm), `d` (probe answers, the delete
    /// tail: hook at +0x68 quiet, pressed, pressed with deleting allowed),
    /// `fakes` (FAKE for the non-zero answers and the sink).
    pub(crate) fn mc_state07_fixture(c: &mut Cpu, seed: u32, row: u32, d: u32, fakes: bool) -> Rng {
        let one = if fakes { FAKE } else { RET_ONE };
        let r = mc_check_mgr(c, seed, 0x23f698, if fakes { FAKE } else { RET_ZERO });
        let this = DATA + 0x400;
        let vt = DATA + 0x1100;
        let (mut f338, mut f128, mut c8, mut save, mut f440, mut flags) = (0, 0, 0, one, 0, 0);
        let (mut hook6c, mut allowed, mut rd, mut used, mut name, mut cursor) =
            (RET_ZERO, 1, d, RET_ZERO, 0u16, 0);
        let (mut b8, mut room, mut s208, mut s210, mut s1a8) =
            (RET_ZERO, one, RET_ZERO, RET_ZERO, RET_ZERO);
        match row {
            0 => f338 = 1,
            1 => (f128, c8) = (1, 1),
            2 => (f128, allowed, s208) = (1, 0, one),
            3 => (save, flags) = (RET_ZERO, 8),
            4 => (save, flags) = (RET_ZERO, 0x10),
            5 => (save, flags) = (RET_ZERO, 2),
            6 => (save, flags) = (RET_ZERO, 1),
            7 => (save, flags) = (RET_ZERO, 4),
            8 => save = RET_ZERO,
            9 => (save, f440, f128) = (RET_ZERO, 1, 1),
            10 => c8 = 1,
            11 => (save, hook6c) = (RET_ZERO, one),
            12 => {}
            13 => (rd, b8) = (4, RET_ONE),
            14 => (rd, room) = (4, RET_ZERO),
            15 => (rd, used) = (4, one),
            16 => (rd, used, name) = (4, one, 0x41),
            17 => (rd, used, cursor) = (4, one, 3),
            18 => (allowed, s210) = (0, one),
            19 => (allowed, s1a8) = (0, one),
            _ => allowed = 0,
        }
        w32(c, this + 0x338, f338);
        w32(c, this + 0x128, f128);
        w32(c, this + 0xc8, c8);
        w32(c, this + 0x64, save);
        w32(c, this + 0x440, f440);
        w32(c, this + 0x43c, flags);
        w32(c, this + 0x6c, hook6c);
        w32(c, this + 0xcc, allowed);
        w32(c, this + 0xf8, cursor);
        c.bus.write16(this + 0x276, name);
        // The poll keeps the flags.
        w32(c, this + 0x428, 0);
        w32(c, this + 0x42c, 1);
        mc_check_ready_slots(c, rd, one);
        ret_slot(c, vt, 0x218, used);
        ret_slot(c, vt, 0xb8, b8);
        ret_slot(c, vt, 0xc8, room);
        ret_slot(c, vt, 0x208, s208);
        ret_slot(c, vt, 0x210, s210);
        ret_slot(c, vt, 0x1a8, s1a8);
        let dc = d % 3;
        w32(c, this + 0x68, if dc == 0 { RET_ZERO } else { one });
        w32(c, this + 0xd4, (dc == 2) as u32);
        c.pc = 0x0023_f698;
        r
    }

    /// Cycle totals of the arms (deterministic seeds): checking the card;
    /// post 0x38; 0x16 (three delete tails); SetStateNoCard and posts 0x27,
    /// 0x30, 5, 0x17; no flag (two tails); poll unchanged; ReturnToSlotState;
    /// post 1; not ready (four probes); the name loop over free slots (0xd);
    /// no room (slot 0xa0, slot 0xc8); a clash after the cursor's own slot;
    /// six used slots, other names (post 9); a clash at slot 0; posts 0x17,
    /// nothing (slot 0x1a8 set) and 0x14, two tails each.
    const STATE07_PATHS: &[u64] = &[
        12, 28, 116, 119, 137, 87, 82, 85, 88, 91, 103, 124, 109, 98, 100, 106, 126, 136, 283,
        167, 178, 241, 480, 192, 123, 144, 127, 145, 133, 154,
    ];

    #[test]
    fn mc_manager_state07_check_matches() {
        check_paths(
            ee::s06::f_0023f698__generated,
            mc_manager_state07_check,
            210,
            STATE07_PATHS,
            &|c, seed| {
                let _ = mc_state07_fixture(c, seed, seed % 21, seed / 21 % 5, seed / 105 % 2 == 1);
            },
        );
    }

    driven!(gen_state07_check, ee::s06::f_0023f698__generated);
    driven!(hand_state07_check, mc_manager_state07_check);

    /// Events at the name loop's back edge (0x23f8ec): six free slots, or
    /// the cursor's slot 0 used (its empty name matches; the next slot
    /// clashes after one back edge). The names stay empty so `UStr_NCmp`'s
    /// own back edge never runs.
    #[test]
    fn mc_manager_state07_check_events() {
        check_events(gen_state07_check, hand_state07_check, 60, &|c, seed| {
            let row = if seed / 3 % 2 == 0 { 12 } else { 15 };
            let mut r = mc_state07_fixture(c, seed, row, 4, false);
            schedule(c, &mut r, seed, 0x0023_f698)
        });
    }

    /// Real leaves for a backend whose "finished" answer (slot 0x248) turns
    /// non-zero after slot 0xf8 runs: `[a0 + 0x2c] = a1`
    /// (`cFEStateBraggingRights_SetRowMask`) and `return [a0 + 0xc]`
    /// (`NetFEState_GetSelectedUser`).
    pub(crate) const STATE20_SETTER: u32 = 0x0019_2968;

    pub(crate) const STATE20_GETTER: u32 = 0x001d_d560;

    /// Cycle totals of the arms (deterministic seeds): finished at once;
    /// slot 0xf8 not 1 and not finished; finished meanwhile; the read hook
    /// busy; read done (FinishLoad); the +0x20 hook quiet (slot 0x140,
    /// FinishLoad); the +0x20 hook set (CardSetState1E, post 0x21).
    const STATE20_PATHS: &[u64] = &[48, 54, 68, 57, 92, 98, 81];

    #[test]
    fn mc_manager_state20_load_matches() {
        check_paths(
            ee::s06::f_00240130__generated,
            mc_manager_state20_load,
            64,
            STATE20_PATHS,
            &|c, seed| {
                let row = seed % 8;
                let d = seed / 8 % 4;
                let fakes = seed / 32 % 2 == 1;
                let one = if fakes { FAKE } else { RET_ONE };
                let mut r = mc_check_mgr(c, seed, 0x240130, if fakes { FAKE } else { RET_ZERO });
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                // (+0xe8, +0xf0): below for rows 3 and 4, not below elsewhere.
                let (e8, f0): (i32, i32) = if row == 3 || row == 4 {
                    [(0, 1), (-3, 2), (1, 5), (-1, 0)][d as usize]
                } else {
                    [(1, 1), (5, 2), (0, -4), (0, 0)][d as usize]
                };
                w32(c, this + 0xe8, e8 as u32);
                w32(c, this + 0xf0, f0 as u32);
                w32(c, this + 0x108, r.next_u32());
                w32(c, this + 0xf8, d);
                // The progress numerator, and the float there before.
                ret_slot(c, vt, 0x90, if d % 2 == 0 { RET_ZERO } else { one });
                w32(c, this + 0x10c, r.next_u32());
                ret_slot(c, vt, 0x248, if row == 0 { one } else { RET_ZERO });
                ret_slot(
                    c,
                    vt,
                    0xf8,
                    match row {
                        0..=2 => RET_ZERO,
                        7 => FAKE,
                        _ => RET_ONE,
                    },
                );
                if row == 2 {
                    // Slot 0xf8 stores the backend at backend+0x42c, which
                    // slot 0x248 reads.
                    c.bus.write16(vt + 0xf8, 0x400);
                    w32(c, vt + 0xfc, STATE20_SETTER);
                    c.bus.write16(vt + 0x248, 0x420);
                    w32(c, vt + 0x24c, STATE20_GETTER);
                }
                w32(c, this + 0x1c, if row == 3 { one } else { RET_ZERO });
                w32(c, this + 0x20, if row == 6 { one } else { RET_ZERO });
                ret_slot(c, vt, 0x140, if fakes { FAKE } else { RET_ZERO });
                c.pc = 0x0024_0130;
            },
        );
    }
}
