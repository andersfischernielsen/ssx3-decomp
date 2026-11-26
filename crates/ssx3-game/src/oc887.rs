//! Staging module oc887 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc887`.

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
        0x001b_70a8,
        cfe_state_net_account_mem_clear_account_strings,
        "cFEStateNetAccountMem_ClearAccountStrings",
    ),
    (
        0x001b_7650,
        cfe_state_net_account_mem_show_popup,
        "cFEStateNetAccountMem_ShowPopup",
    ),
    (
        0x001b_77a8,
        cfe_state_net_account_mem_update_state01,
        "cFEStateNetAccountMem_UpdateState01",
    ),
    (
        0x001b_7a98,
        cfe_state_net_account_mem_update_state14,
        "cFEStateNetAccountMem_UpdateState14",
    ),
];

/// Address of a field of the object held in callee-saved `reg`, re-read from
/// the register each time (as the compiled code does, since callees run in
/// between).
fn account_mem_field(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `cFEStateNetAccountMem_SetState(self, state)` (0x1b70d0), called with
/// `a0` = self already in place and `a1` = the state set by the caller.
fn account_mem_set_state(c: &mut Cpu, ret_addr: u32) -> Flow {
    Callee {
        addr: 0x001b_70d0,
        f: ee::s03::f_001b70d0,
    }
    .call(c, ret_addr)
}

/// `cFEStateNetAccountMem_StoreAccountStrings(self)` (0x1b7778).
fn account_mem_store_strings(c: &mut Cpu, ret_addr: u32) -> Flow {
    Callee {
        addr: 0x001b_7778,
        f: ee::s03::f_001b7778,
    }
    .call(c, ret_addr)
}

/// `cFEPopupConfirm_CfgSetOptionTextId(sub, index, text)` (0x1c57c0), with
/// the arguments already in `a0..a2`.
fn account_mem_cfg_option(c: &mut Cpu, ret_addr: u32) -> Flow {
    Callee {
        addr: 0x001c_57c0,
        f: ee::s03::f_001c57c0,
    }
    .call(c, ret_addr)
}

/// `void cFEStateNetAccountMem_ClearAccountStrings(self)` (0x1b70a8): zero the
/// first byte of the account name (`+0x6e0`) and password (`+0x6f1`), then
/// copy both into the shared login manager.
pub fn cfe_state_net_account_mem_clear_account_strings(c: &mut Cpu) {
    guest(c, clear_account_strings_body)
}

fn clear_account_strings_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x001b_70a8);
    c.cycles += 9;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    let account = c.g[V0][0] as u32;
    c.sb(account.wrapping_add(0x6e0), 0);
    c.sb(account.wrapping_add(0x6f1), 0);
    account_mem_store_strings(c, 0x001b_70c0)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cFEStateNetAccountMem_UpdateState01(self)` (0x1b77a8): enter state 2
/// (`cFEStateNetAccountMem_SetState(self, 2)`).
pub fn cfe_state_net_account_mem_update_state01(c: &mut Cpu) {
    guest(c, update_state01_body)
}

fn update_state01_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x001b_77a8);
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = 0x2;
    account_mem_set_state(c, 0x001b_77b8)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cFEStateNetAccountMem_ShowPopup(self, popupId, defaultSel,
/// firstButton, secondButton, titleText, bodyText)` (0x1b7650): fetch the
/// account-memory popup (`NetFEState_GetPopup`), configure its one or two
/// option texts and its default option, set its title and body, and show it
/// (`NetFEState_ShowPopup`). `popupId` goes to the popup's misc data.
///
/// The popup's `info` block (`cPopupInfo`) sits at `+0xbc`; its fields are
/// addressed relative to that block (`sub`) and to the popup itself.
pub fn cfe_state_net_account_mem_show_popup(c: &mut Cpu) {
    guest(c, show_popup_body)
}

/// Popup-relative offsets (the info block's `meType` is at `POPUP_INFO`).
const POPUP_INFO: u32 = 0xbc;
const POPUP_RECEIVE_PREV: u32 = 0x164;
const POPUP_DEFAULT_OPTION: u32 = 0x168;

fn show_popup_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x001b_7650);
    c.cycles += 22;
    let frame = Frame::open(c, 0xa0);
    frame.save(c, 23, 0x20);
    frame.save(c, 18, 0x70);
    c.g[S7][0] = c.g[A0][0];
    frame.save(c, 19, 0x60);
    c.g[S2][0] = c.g[A3][0];
    frame.save(c, 20, 0x50);
    c.g[S3][0] = c.g[T0][0];
    frame.save(c, 21, 0x40);
    c.g[S4][0] = c.g[A2][0];
    frame.save(c, 22, 0x30);
    c.g[S5][0] = c.g[T1][0];
    frame.save(c, 30, 0x10);
    c.g[S6][0] = c.g[T2][0];
    frame.save(c, 16, 0x90);
    frame.save(c, 17, 0x80);
    frame.save_ra(c, 0);
    c.g[FP][0] = c.g[A1][0];

    // popup = NetFEState_GetPopup(self); sub = &popup->info.
    Callee {
        addr: 0x001a_9c50,
        f: ee::s03::f_001a9c50,
    }
    .call(c, 0x001b_769c)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[S0][0] = sx(account_mem_field(c, S1, POPUP_INFO));

    'b: {
        'cc: {
            if c.g[S2][0] == 0 {
                break 'cc;
            }
            // firstButton set: one or two option texts.
            'dd: {
                c.cycles += 2;
                let second = c.g[S3][0] != 0;
                c.g[V0][0] = 2;
                if !second {
                    break 'dd;
                }
                c.cycles += 11;
                c.g[A2][0] = c.g[S2][0];
                c.sw(account_mem_field(c, S1, POPUP_INFO), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                account_mem_cfg_option(c, 0x001b_76c4)?;
                c.g[A2][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                account_mem_cfg_option(c, 0x001b_76d4)?;
                c.sw(account_mem_field(c, S1, POPUP_DEFAULT_OPTION), c.g[S4][0]);
                break 'b;
            }
            // Only the first option text.
            c.cycles += 8;
            c.g[V0][0] = 1;
            c.g[A2][0] = c.g[S2][0];
            c.sw(account_mem_field(c, S1, POPUP_INFO), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            account_mem_cfg_option(c, 0x001b_76f4)?;
            c.sw(account_mem_field(c, S1, POPUP_DEFAULT_OPTION), 0);
            break 'b;
        }
        // No first button: a plain popup.
        c.cycles += 3;
        c.sw(account_mem_field(c, S1, POPUP_INFO), 0);
        c.sw(account_mem_field(c, S1, POPUP_RECEIVE_PREV), 0);
        c.sw(account_mem_field(c, S1, POPUP_DEFAULT_OPTION), 0);
    }

    // Title, body, misc data and flags on sub.
    c.cycles += 2;
    c.g[V0][0] = 1;
    if c.g[S5][0] != 0 {
        c.cycles += 2;
        c.sw(account_mem_field(c, S0, 0x10), c.g[S5][0]);
        c.sw(account_mem_field(c, S0, 0xc), c.g[V0][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    if c.g[S6][0] != 0 {
        c.cycles += 2;
        c.sw(account_mem_field(c, S0, 0x8), c.g[S6][0]);
        c.sw(account_mem_field(c, S0, 0x4), c.g[V0][0]);
    }
    c.cycles += 19;
    c.sw(account_mem_field(c, S0, 0x94), c.g[FP][0]);
    c.g[V0][0] = 1;
    c.sw(account_mem_field(c, S0, 0xa8), 0);
    c.g[A0][0] = c.g[S7][0];
    c.sw(account_mem_field(c, S0, 0x98), c.g[V0][0]);
    c.g[A1][0] = c.g[S1][0];
    Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::f_001a9bc0,
    }
    .call(c, 0x001b_7744)?;

    frame.restore(c, 16, 0x90);
    frame.restore(c, 17, 0x80);
    frame.restore(c, 18, 0x70);
    frame.restore(c, 19, 0x60);
    frame.restore(c, 20, 0x50);
    frame.restore(c, 21, 0x40);
    frame.restore(c, 22, 0x30);
    frame.restore(c, 23, 0x20);
    frame.restore(c, 30, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void cFEStateNetAccountMem_UpdateState14(self)` (0x1b7a98): the account
/// memory's step for state 0x14. The login manager's mode (`mgr+0`) picks
/// the path: mode 4 compares the account name and password with the stored
/// ones and enters state 0x17; mode 5 enters state 0x18 when the card slot
/// is unset; and the state the object reaches next is read from `+0x6d4`.
pub fn cfe_state_net_account_mem_update_state14(c: &mut Cpu) {
    guest(c, update_state14_body)
}

fn update_state14_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x001b_7a98);
    c.cycles += 9;
    let frame = Frame::open(c, 0x30);
    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
    frame.save(c, 16, 0x20);
    frame.save(c, 17, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = c.g[V1][0] ^ 4;
    c.g[S0][0] = c.g[A0][0];

    'o: {
        'p: {
            'q: {
                if c.g[V0][0] == 0 {
                    // Mode 4: is the stored account name and password the same?
                    'aec: {
                        c.cycles += 7;
                        c.g[V0][0] = 0x53_0000;
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6e0));
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6760));
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x0041_65a8, 0x001b_7ad0)?;
                        let name_differs = c.g[V0][0] != 0;
                        c.g[V1][0] = 1;
                        if name_differs {
                            break 'aec;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x11));
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6f1));
                        call_ptr(c, 0x0041_65a8, 0x001b_7ae4)?;
                        let password_same = c.g[V0][0] == 0;
                        c.g[V1][0] = 1;
                        if password_same {
                            break 'q;
                        }
                    }
                    c.cycles += 9;
                    c.g[V0][0] = 0x18;
                    c.sw(account_mem_field(c, S0, 0x6d4), c.g[V0][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(account_mem_field(c, S0, 0x6d8), c.g[V1][0]);
                    c.g[A1][0] = 0x17;
                    c.sw(account_mem_field(c, S0, 0xb08), c.g[V1][0]);
                    account_mem_set_state(c, 0x001b_7b08)?;
                    frame.restore(c, 16, 0x20);
                    break 'o;
                }

                // Mode other than 4: mode 5 is handled here, others leave.
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] ^ 5;
                if c.g[V0][0] != 0 {
                    frame.restore(c, 16, 0x20);
                    break 'o;
                }
                'b50: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(account_mem_field(c, S0, 0x6d4));
                    c.g[S1][0] = 1;
                    let other_than_one = c.g[V1][0] != c.g[S1][0];
                    c.g[V0][0] = 5;
                    if other_than_one {
                        break 'b50;
                    }
                    c.cycles += 9;
                    c.g[V0][0] = 0x18;
                    c.sw(account_mem_field(c, S0, 0x6d8), c.g[S1][0]);
                    c.sw(account_mem_field(c, S0, 0xb08), c.g[S1][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(account_mem_field(c, S0, 0x6d4), c.g[V0][0]);
                    c.g[A1][0] = 0x17;
                    account_mem_set_state(c, 0x001b_7b48)?;
                    frame.restore(c, 16, 0x20);
                    break 'o;
                }
                c.cycles += 2;
                let is_five = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 6;
                if is_five {
                    break 'q;
                }
                c.cycles += 2;
                let not_six = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 2;
                if not_six {
                    break 'p;
                }
                break 'q;
            }
            // Leaves the 0x14 step through state 0x15.
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x15;
            account_mem_set_state(c, 0x001b_7b6c)?;
            frame.restore(c, 16, 0x20);
            break 'o;
        }
        // Reached when `+0x6d4` is 2 after the mode-5 compares: the card-slot
        // unformatted check, then state 0x17 (or 0x18 when unformatted).
        'ba4: {
            c.cycles += 2;
            let not_two = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if not_two {
                break 'ba4;
            }
            'b94: {
                c.cycles += 4;
                c.g[A0][0] = c.g[A1][0];
                call_ptr(c, 0x0026_5cb0, 0x001b_7b84)?;
                let unformatted = c.g[V0][0] == 0;
                c.g[V0][0] = 0x18;
                if unformatted {
                    break 'b94;
                }
                c.cycles += 2;
                c.sw(account_mem_field(c, S0, 0xb08), c.g[S1][0]);
                c.sw(account_mem_field(c, S0, 0x6d4), c.g[V0][0]);
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x17;
            account_mem_set_state(c, 0x001b_7ba0)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 3;
        c.g[A1][0] = 0x17;
        account_mem_set_state(c, 0x001b_7bac)?;
        frame.restore(c, 16, 0x20);
    }

    c.cycles += 4;
    frame.restore(c, 17, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, LOG_RESULTS};
    use crate::difftest::{check, DATA, GAME_GP};

    /// The account world that `SetState` and the account memory read: the
    /// world (`gp-0x848`), its object at `+0x8c`, and the info block the
    /// object points at (vtable-less, so the method call hits a fake).
    fn account_world(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        let world = DATA + 0x6_8000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
        let object = DATA + 0x6_8100;
        c.bus.write32(world + 0x8c, object);
        let info = DATA + 0x6_8200;
        c.bus.write32(object + 4, info);
        c.bus.write16(info + 0x20, 0);
        c.bus.write32(info + 0x24, 0);
    }

    #[test]
    fn cfe_state_net_account_mem_clear_account_strings_matches() {
        check(
            ee::s03::f_001b70a8__generated,
            cfe_state_net_account_mem_clear_account_strings,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b70a8, 0);
                account_world(c);
                c.pc = 0x001b_70a8;
                c.g[A0][0] = sx(DATA);
                // Name and password bytes that the clear must overwrite.
                for i in 0..0x20u32 {
                    c.bus.write8(DATA + 0x6e0 + i, r.next_u32() as u8);
                }
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_show_popup_matches() {
        check(
            ee::s03::f_001b7650__generated,
            cfe_state_net_account_mem_show_popup,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b7650, 3);
                account_world(c);
                c.pc = 0x001b_7650;
                // NetFEState_GetPopup returns the popup stored at self+0x6a0.
                let popup = DATA + 0x1000;
                c.bus.write32(DATA + 0x6a0, popup);
                // NetFEState_ShowPopup's owner screen and list (its own test's
                // fixture): the popup is owned or pushed depending on the seed.
                let screen = DATA + 0x2000;
                c.bus.write32(DATA + 0x40, screen);
                c.bus.write32(screen + 0xd0, DATA + 0x2100);
                c.bus.write32(DATA + 0x2100 + 0x10, DATA + 0x2200);
                c.bus.write32(DATA + 0x2200 + 0x18, 0);
                c.bus.write32(
                    LOG_RESULTS,
                    if seed % 3 == 0 {
                        DATA + 0x2200
                    } else {
                        DATA + 0x2300
                    },
                );
                c.bus.write32(popup + 0x2b8, seed % 2);
                c.bus.write32(popup + 0x1c, 0x1234);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                // firstButton (a3), secondButton (t0): zero and non-zero.
                c.g[A3][0] = if seed % 3 == 0 {
                    0
                } else {
                    1 + (seed % 7) as u64
                };
                c.g[T0][0] = if seed % 4 == 0 {
                    0
                } else {
                    2 + (seed % 5) as u64
                };
                // titleText (t1) and bodyText (t2): zero and non-zero.
                c.g[T1][0] = if seed % 5 == 0 { 0 } else { sx(DATA + 0x3000) };
                c.g[T2][0] = if seed % 6 == 0 { 0 } else { sx(DATA + 0x3100) };
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_update_state01_matches() {
        check(
            ee::s03::f_001b77a8__generated,
            cfe_state_net_account_mem_update_state01,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1b77a8, 0);
                account_world(c);
                c.pc = 0x001b_77a8;
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_net_account_mem_update_state14_matches() {
        check(
            ee::s03::f_001b7a98__generated,
            cfe_state_net_account_mem_update_state14,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x1b7a98, 3);
                account_world(c);
                c.pc = 0x001b_7a98;
                // The login manager (gp+0x2e0) and its mode word: 4 compares
                // the account strings, 5 reads the state at +0x6d4, others leave.
                let mgr = DATA + 0x6_9000;
                c.bus.write32(GAME_GP.wrapping_add(0x2e0), mgr);
                let modes = [4u32, 4, 5, 5, 6, 0x10];
                c.bus.write32(mgr, modes[(seed % 6) as usize]);
                c.bus
                    .write32(DATA + 0x6d4, [1u32, 2, 5, 6, 3][(seed % 5) as usize]);
                // Account name and password: empty (equal to the stored
                // strings, which are zero) or not.
                c.bus
                    .write8(DATA + 0x6e0, if seed % 2 == 0 { 0 } else { b'x' });
                c.bus
                    .write8(DATA + 0x6f1, if seed % 3 == 0 { 0 } else { b'y' });
                c.g[A0][0] = sx(DATA);
            },
        );
    }
}
