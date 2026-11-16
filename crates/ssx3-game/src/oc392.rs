//! Staging module oc392 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc392`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

use crate::call::Flow;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0023_d7d8,
    mc_manager_update_states,
    "MCManager_UpdateStates",
)];

/// Fields of the memory card overlay manager (`cMCOverlayManager`) that
/// `MCManager_UpdateStates` reads and writes.
pub(crate) mod mcus {
    /// `mState`: the state machine's current state.
    pub const STATE: u32 = 0x130;
    /// `mSaveOk`, `mLoadOk`, `mDeleteOk`, `mCancelOk`: what the screen offers.
    pub const SAVE_OK: u32 = 0xcc;
    pub const LOAD_OK: u32 = 0xd0;
    pub const DELETE_OK: u32 = 0xd4;
    pub const CANCEL_OK: u32 = 0xd8;
    /// `mCardChanged`.
    pub const CARD_CHANGED: u32 = 0xc8;
    /// `mFileCursor`: the highlighted save file.
    pub const FILE_CURSOR: u32 = 0xf8;
    /// `mSave`, `mDelete`: an operation is pending.
    pub const SAVE: u32 = 0xbc;
    pub const DELETE: u32 = 0xc0;
    /// Input callbacks (`bool (*)(void)`).
    pub const CB_SAVE_PRESSED: u32 = 0x64;
    pub const CB_DELETE_PRESSED: u32 = 0x68;
    pub const CB_DEVICE_SELECTED: u32 = 0x74;
    pub const CB_LOAD_PRESSED: u32 = 0x78;
    pub const CB_NEXT_DEVICE: u32 = 0x7c;
    pub const CB_PREV_DEVICE: u32 = 0x80;
    pub const CB_FORMAT_PRESSED: u32 = 0x88;
    /// `mCheckCard`, `mCheckDeviceTicks`: the periodic card poll.
    pub const CHECK_CARD: u32 = 0x338;
    pub const CHECK_TICKS: u32 = 0x33c;
    /// `mDevices[]`: 0xe0 bytes each; `bPluggedIn` at +0xc.
    pub const DEVICES: u32 = 0x344;
    pub const DEVICE_SIZE: u64 = 0xe0;
    pub const DEVICE_PLUGGED_IN: u32 = DEVICES + 0xc;
    /// `mSelectedDevice`, `mCurrCheckDevice`, `mLastCheckDevice`.
    pub const SELECTED: u32 = 0x428;
    pub const CURR_CHECK: u32 = 0x42c;
    pub const LAST_CHECK: u32 = 0x430;
    /// `mpMemCard`: the card driver object (its vtable holds
    /// `{short delta; fn}` slots).
    pub const CARD: u32 = 0x434;
    /// The message object: slot 8 of the vtable it points at posts a state.
    pub const MSG: u32 = 0x748;
    /// Card driver vtable slots.
    pub const VT_POLL: u32 = 0x18;
    pub const VT_BUSY: u32 = 0x58;
    pub const VT_SET_CHECK_DEVICE: u32 = 0x28;
    pub const VT_DEVICE_PRESENT: u32 = 0xd0;
    pub const VT_START_CHECK: u32 = 0xd8;
    pub const VT_CHANGED: u32 = 0x178;
    pub const VT_CAN_SAVE: u32 = 0x1a8;
    pub const VT_DEVICE_OK: u32 = 0x1a0;
    pub const VT_CAN_LOAD: u32 = 0x1b8;
    pub const VT_CARD_CHANGED: u32 = 0x1f8;
    pub const VT_FILE_EXISTS: u32 = 0x218;
    /// The switch's jump table (states 2..=56) in `.rodata`.
    pub const JUMP_TABLE: u32 = 0x0047_c280;
}

mod mcus_callee {
    use super::{ee, Callee};
    pub const READ_DIR_INFO: Callee = Callee {
        addr: 0x0024_20c8,
        f: ee::s06::f_002420c8,
    };
    pub const NEXT_SLOT: Callee = Callee {
        addr: 0x0024_1400,
        f: ee::s06::f_00241400,
    };
    pub const PREV_SLOT: Callee = Callee {
        addr: 0x0024_1540,
        f: ee::s06::f_00241540,
    };
    pub const ON_CARD_SCANNED: Callee = Callee {
        addr: 0x0024_1fd0,
        f: ee::s06::f_00241fd0,
    };
    pub const GET_NUM_SLOTS: Callee = Callee {
        addr: 0x0024_1ac0,
        f: ee::s06::f_00241ac0,
    };
    pub const REQUEST_CARD_INFO: Callee = Callee {
        addr: 0x0023_fb20,
        f: ee::s06::f_0023fb20,
    };
    pub const STATE38: Callee = Callee {
        addr: 0x0023_e428,
        f: ee::s06::f_0023e428,
    };
    /// `MEM_fill(dst, byte, n)` (reached through the function table).
    pub const MEM_FILL: u32 = 0x003e_6448;
}

/// First case block of the switch; the blocks are 0x10 apart, in this order
/// (the jump table maps a state to one of them). State 0x38's block
/// (0x23e240) and the no-handler target (0x23e248, state 8) are separate.
const MCUS_CASE_BASE: u32 = 0x0023_def0;
const MCUS_CASE_STATE38: u32 = 0x0023_e240;
const MCUS_CASE_NONE: u32 = 0x0023_e248;

/// The state handler each case block calls (`MCManager_StateNN`, NN = the state
/// in hex), by block.
const MCUS_CASES: [Callee; 53] = [
    Callee {
        addr: 0x0023_e268,
        f: ee::s06::f_0023e268,
    }, // 0x23def0 MCManager_State31
    Callee {
        addr: 0x0023_e2c0,
        f: ee::s06::f_0023e2c0,
    }, // 0x23df00 MCManager_State32
    Callee {
        addr: 0x0023_e2c8,
        f: ee::s06::f_0023e2c8,
    }, // 0x23df10 MCManager_State35
    Callee {
        addr: 0x0023_e320,
        f: ee::s06::f_0023e320,
    }, // 0x23df20 MCManager_State36
    Callee {
        addr: 0x0023_e498,
        f: ee::s06::f_0023e498,
    }, // 0x23df30 MCManager_State33
    Callee {
        addr: 0x0023_e4f0,
        f: ee::s06::f_0023e4f0,
    }, // 0x23df40 MCManager_State34
    Callee {
        addr: 0x0023_e4f8,
        f: ee::s06::f_0023e4f8,
    }, // 0x23df50 MCManager_State02
    Callee {
        addr: 0x0023_e540,
        f: ee::s06::f_0023e540,
    }, // 0x23df60 MCManager_State03_Check
    Callee {
        addr: 0x0023_e820,
        f: ee::s06::f_0023e820,
    }, // 0x23df70 MCManager_State04_NoCard
    Callee {
        addr: 0x0023_eb80,
        f: ee::s06::f_0023eb80,
    }, // 0x23df80 MCManager_State05
    Callee {
        addr: 0x0023_ec00,
        f: ee::s06::f_0023ec00,
    }, // 0x23df90 MCManager_State06_Check
    Callee {
        addr: 0x0023_ef68,
        f: ee::s06::f_0023ef68,
    }, // 0x23dfa0 MCManager_State0A_Format
    Callee {
        addr: 0x0023_f050,
        f: ee::s06::f_0023f050,
    }, // 0x23dfb0 MCManager_State0B_Format
    Callee {
        addr: 0x0023_f0f8,
        f: ee::s06::f_0023f0f8,
    }, // 0x23dfc0 MCManager_State0C
    Callee {
        addr: 0x0023_fd40,
        f: ee::s06::f_0023fd40,
    }, // 0x23dfd0 MCManager_State16
    Callee {
        addr: 0x0023_f2d8,
        f: ee::s06::f_0023f2d8,
    }, // 0x23dfe0 MCManager_State13
    Callee {
        addr: 0x0023_f150,
        f: ee::s06::f_0023f150,
    }, // 0x23dff0 MCManager_State14
    Callee {
        addr: 0x0023_f1d8,
        f: ee::s06::f_0023f1d8,
    }, // 0x23e000 MCManager_State15
    Callee {
        addr: 0x0023_f258,
        f: ee::s06::f_0023f258,
    }, // 0x23e010 MCManager_State17
    Callee {
        addr: 0x0023_f308,
        f: ee::s06::f_0023f308,
    }, // 0x23e020 MCManager_State0D
    Callee {
        addr: 0x0023_f3a0,
        f: ee::s06::f_0023f3a0,
    }, // 0x23e030 MCManager_State0E
    Callee {
        addr: 0x0023_f438,
        f: ee::s06::f_0023f438,
    }, // 0x23e040 MCManager_State0F_Save
    Callee {
        addr: 0x0023_f4f8,
        f: ee::s06::f_0023f4f8,
    }, // 0x23e050 MCManager_State10_Save
    Callee {
        addr: 0x0023_f578,
        f: ee::s06::f_0023f578,
    }, // 0x23e060 MCManager_State11_Save
    Callee {
        addr: 0x0023_f698,
        f: ee::s06::f_0023f698,
    }, // 0x23e070 MCManager_State07_Check
    Callee {
        addr: 0x0023_fbb8,
        f: ee::s06::f_0023fbb8,
    }, // 0x23e080 MCManager_State12_Save
    Callee {
        addr: 0x0023_ff00,
        f: ee::s06::f_0023ff00,
    }, // 0x23e090 MCManager_State22_Load
    Callee {
        addr: 0x0023_ffc0,
        f: ee::s06::f_0023ffc0,
    }, // 0x23e0a0 MCManager_State23
    Callee {
        addr: 0x0023_fc70,
        f: ee::s06::f_0023fc70,
    }, // 0x23e0b0 MCManager_State24
    Callee {
        addr: 0x0023_fe70,
        f: ee::s06::f_0023fe70,
    }, // 0x23e0c0 MCManager_State25
    Callee {
        addr: 0x0023_fff0,
        f: ee::s06::f_0023fff0,
    }, // 0x23e0d0 MCManager_State1E
    Callee {
        addr: 0x0024_00b0,
        f: ee::s06::f_002400b0,
    }, // 0x23e0e0 MCManager_State1F_Load
    Callee {
        addr: 0x0024_0130,
        f: ee::s06::f_00240130,
    }, // 0x23e0f0 MCManager_State20_Load
    Callee {
        addr: 0x0024_02d0,
        f: ee::s06::f_002402d0,
    }, // 0x23e100 MCManager_State1D_Check
    Callee {
        addr: 0x0024_05d0,
        f: ee::s06::f_002405d0,
    }, // 0x23e110 MCManager_State21_Load
    Callee {
        addr: 0x0023_fd08,
        f: ee::s06::f_0023fd08,
    }, // 0x23e120 MCManager_State26
    Callee {
        addr: 0x0024_0688,
        f: ee::s06::f_00240688,
    }, // 0x23e130 MCManager_State18
    Callee {
        addr: 0x0024_0768,
        f: ee::s06::f_00240768,
    }, // 0x23e140 MCManager_State19_Delete
    Callee {
        addr: 0x0024_0860,
        f: ee::s06::f_00240860,
    }, // 0x23e150 MCManager_State1A_Delete
    Callee {
        addr: 0x0024_0960,
        f: ee::s06::f_00240960,
    }, // 0x23e160 MCManager_State1B
    Callee {
        addr: 0x0024_0800,
        f: ee::s06::f_00240800,
    }, // 0x23e170 MCManager_State1C
    Callee {
        addr: 0x0024_09c8,
        f: ee::s06::f_002409c8,
    }, // 0x23e180 MCManager_State09
    Callee {
        addr: 0x0024_0ab0,
        f: ee::s06::f_00240ab0,
    }, // 0x23e190 MCManager_State27_Format
    Callee {
        addr: 0x0024_0b88,
        f: ee::s06::f_00240b88,
    }, // 0x23e1a0 MCManager_State28_Format
    Callee {
        addr: 0x0024_0c30,
        f: ee::s06::f_00240c30,
    }, // 0x23e1b0 MCManager_State29
    Callee {
        addr: 0x0024_0c90,
        f: ee::s06::f_00240c90,
    }, // 0x23e1c0 MCManager_State2A
    Callee {
        addr: 0x0024_0cc0,
        f: ee::s06::f_00240cc0,
    }, // 0x23e1d0 MCManager_State2B
    Callee {
        addr: 0x0024_0d90,
        f: ee::s06::f_00240d90,
    }, // 0x23e1e0 MCManager_State2C_Delete
    Callee {
        addr: 0x0024_0eb0,
        f: ee::s06::f_00240eb0,
    }, // 0x23e1f0 MCManager_State2D
    Callee {
        addr: 0x0024_0ee0,
        f: ee::s06::f_00240ee0,
    }, // 0x23e200 MCManager_State2E
    Callee {
        addr: 0x0024_0f10,
        f: ee::s06::f_00240f10,
    }, // 0x23e210 MCManager_State2F
    Callee {
        addr: 0x0024_0f30,
        f: ee::s06::f_00240f30,
    }, // 0x23e220 MCManager_State30
    Callee {
        addr: 0x0023_e3c0,
        f: ee::s06::f_0023e3c0,
    }, // 0x23e230 MCManager_State37
];

/// `m + off`.
#[inline]
fn mcus_at(m: u32, off: u32) -> Addr {
    Addr(m.wrapping_add(off))
}

/// A card driver method: `vt_r = *obj`, `a0 = obj + vt[off].delta`,
/// `fn_r = vt[off].fn`, call it returning to `ret`. `obj_r` holds the object.
fn mcus_card_call(c: &mut Cpu, obj_r: usize, vt_r: usize, fn_r: usize, off: u32, ret: u32) -> Flow {
    let obj = r32(c, obj_r);
    c.g[vt_r][0] = Addr(obj).lw(c);
    let vt = r32(c, vt_r);
    c.g[A0][0] = Addr(vt.wrapping_add(off)).lh(c);
    c.g[fn_r][0] = Addr(vt.wrapping_add(off + 4)).lw(c);
    c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
    let target = r32(c, fn_r);
    call_ptr(c, target, ret)
}

/// `a1 = m->mpMemCard` (the object register of most card calls).
#[inline]
fn mcus_load_card(c: &mut Cpu, m: u32, reg: usize) {
    c.g[reg][0] = mcus_at(m, mcus::CARD).lw(c);
}

/// The compiled `xori`/`beqz` chain that tests `st` against `list`: true at
/// the first of `list[..n-1]` that matches (the last one is tested by the
/// caller's tail). Leaves `v0` as the chain does (`st ^` the next constant,
/// loaded in each branch's delay slot).
fn mcus_state_chain(c: &mut Cpu, st: u64, list: &[u64]) -> bool {
    c.g[V0][0] = st ^ list[0];
    if st == list[0] {
        return true;
    }
    for i in 1..list.len() - 1 {
        c.cycles += if i == 1 { 3 } else { 2 };
        c.g[V0][0] = st ^ list[i + 1];
        if st == list[i] {
            return true;
        }
    }
    false
}

/// `void MCManager_UpdateStates(cMCOverlayManager* m)` (0x23d7d8,
/// `cMCOverlayManager::ManagerLoadSaveUpdate`): the memory card overlay's
/// per-frame state machine step.
///
/// Unless the manager is closed (state 1): clears the offered actions and
/// polls the card driver; while the card is busy (or the screen is one of the
/// save/load/delete selection states 7/0x1d) works out which of save, load,
/// delete and cancel are offered and latches a card change; when the card is
/// idle re-reads the info of the last checked slot (or clears it when the card
/// is gone, moving to the next slot if nothing else is happening); handles the
/// next/previous slot inputs; every few updates (`mCheckDeviceTicks`) scans the
/// slots for the next present card and starts a check on it; then runs the
/// current state's handler through the switch's jump table (read from guest
/// memory, as the compiled `jr` does).
pub fn mc_manager_update_states(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 1;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let m = r32(c, S1);
        c.g[V0][0] = mcus_at(m, mcus::STATE).lw(c);
        if c.g[V0][0] == 1 {
            frame.restore(c, S0, 0x40);
            mcus_epilogue(c, frame);
            return Ok(());
        }

        // Clear the offered actions and poll the card.
        c.cycles += 18;
        for off in [
            mcus::SAVE_OK,
            mcus::LOAD_OK,
            mcus::DELETE_OK,
            mcus::CANCEL_OK,
        ] {
            mcus_at(m, off).sw(c, 0);
        }
        mcus_load_card(c, m, A1);
        mcus_card_call(c, A1, V0, V1, mcus::VT_POLL, 0x0023_d82c)?;
        mcus_load_card(c, m, A1);
        mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_d844)?;

        'card_idle: {
            // Busy, or a state other than 0x14/0x24/0x25/0x1c/0xc: the
            // action and card-change section.
            if c.g[V0][0] != 0 {
                c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
            } else {
                c.cycles += 5;
                let st = mcus_at(m, mcus::STATE).lw(c);
                c.g[V1][0] = st;
                c.g[A0][0] = 0;
                if !mcus_state_chain(c, st, &[0x14, 0x24, 0x25, 0x1c, 0xc]) {
                    c.cycles += 1;
                    if st != 0xc {
                        c.g[A0][0] = c.g[S0][0];
                    }
                }
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    mcus_load_card(c, m, A1);
                    break 'card_idle;
                }
                c.cycles += 1;
                c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
            }
            c.cycles += 3;
            let st = c.g[V1][0];
            c.g[V0][0] = 0x1d;
            'card_changed: {
                if st != 7 {
                    c.cycles += 2;
                    if st != 0x1d {
                        mcus_load_card(c, m, A1);
                        break 'card_changed;
                    }
                }
                // Selection screen (7 or 0x1d): cancel is always offered.
                c.cycles += 12;
                c.g[S3][0] = 1;
                mcus_load_card(c, m, A1);
                mcus_at(m, mcus::CANCEL_OK).sw(c, 1);
                c.g[S2][0] = sx(m.wrapping_add(mcus::SAVE_OK));
                c.g[S0][0] = 0;
                mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_SAVE, 0x0023_d8c8)?;
                'save_ok: {
                    if c.g[V0][0] == 0 {
                        break 'save_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x208, 0x0023_d8ec)?;
                    if c.g[V0][0] != 0 {
                        break 'save_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x210, 0x0023_d910)?;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[S3][0];
                    }
                }
                let save_ok = c.g[S0][0];
                Addr(r32(c, S2)).sw(c, save_ok);

                c.cycles += 10;
                c.g[S0][0] = sx(m.wrapping_add(mcus::LOAD_OK));
                c.g[S2][0] = 0;
                mcus_load_card(c, m, A1);
                mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_LOAD, 0x0023_d938)?;
                'load_ok: {
                    if c.g[V0][0] == 0 {
                        break 'load_ok;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, 0x1d0, 0x0023_d958)?;
                    if c.g[V0][0] == 0 {
                        break 'load_ok;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, 0x198, 0x0023_d978)?;
                    if c.g[V0][0] != 0 {
                        break 'load_ok;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, 0x1d8, 0x0023_d998)?;
                    if c.g[V0][0] != 0 {
                        break 'load_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, mcus::VT_FILE_EXISTS, 0x0023_d9bc)?;
                    if c.g[V0][0] == 0 {
                        break 'load_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x208, 0x0023_d9e0)?;
                    if c.g[V0][0] != 0 {
                        break 'load_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x210, 0x0023_da04)?;
                    c.g[S2][0] = (c.g[V0][0] < 1) as u64;
                }
                let load_ok = c.g[S2][0];
                Addr(r32(c, S0)).sw(c, load_ok);

                // Delete: never on the load screen, else if the file exists.
                c.cycles += 4;
                c.g[V1][0] = 0x1d;
                c.g[V0][0] = mcus_at(m, mcus::STATE).lw(c);
                if c.g[V0][0] != 0x1d {
                    mcus_load_card(c, m, A2);
                    c.cycles += 7;
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, mcus::VT_FILE_EXISTS, 0x0023_da3c)?;
                    let exists = c.g[V0][0];
                    mcus_at(m, mcus::DELETE_OK).sw(c, exists);
                } else {
                    c.cycles += 2;
                    mcus_at(m, mcus::DELETE_OK).sw(c, 0);
                }
                c.cycles += 1;
                mcus_load_card(c, m, A1);
            }
            // Latch a card change and tell the driver.
            c.cycles += 7;
            mcus_card_call(c, A1, V1, V0, mcus::VT_CARD_CHANGED, 0x0023_da58)?;
            let changed = c.g[V0][0] != 0;
            c.g[V1][0] = 1;
            if changed {
                c.cycles += 7;
                mcus_load_card(c, m, A1);
                mcus_at(m, mcus::CARD_CHANGED).sw(c, 1);
                mcus_card_call(c, A1, V0, V1, mcus::VT_CHANGED, 0x0023_da7c)?;
            }
            c.cycles += 1;
            mcus_load_card(c, m, A1);
        }

        // Card idle: refresh (or clear) the last checked slot.
        c.cycles += 7;
        mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_da94)?;
        'slot_done: {
            if c.g[V0][0] != 0 {
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            c.cycles += 5;
            let st = mcus_at(m, mcus::STATE).lw(c);
            c.g[V1][0] = st;
            c.g[A0][0] = 0;
            let list = [
                0x14, 0x24, 0x25, 0x1c, 0xc, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
            ];
            if !mcus_state_chain(c, st, &list) {
                c.cycles += 2;
                if st != 0x36 {
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 0) as u64;
                    c.g[A0][0] = c.g[V0][0] ^ 1;
                }
            }
            c.cycles += 2;
            c.g[S0][0] = mcus::DEVICE_SIZE;
            if c.g[A0][0] == 0 {
                c.cycles += 1;
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            c.cycles += 6;
            c.g[A1][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
            c.mult(0, V1, A1, S0);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(m));
            c.g[V1][0] = mcus_at(r32(c, V0), mcus::DEVICE_PLUGGED_IN).lw(c);
            let plugged_in = c.g[V1][0] != 0;
            mcus_load_card(c, m, A2);
            if plugged_in {
                c.cycles += 7;
                mcus_card_call(c, A2, V1, V0, mcus::VT_DEVICE_OK, 0x0023_dc68)?;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::READ_DIR_INFO.call(c, 0x0023_dc9c)?;
                    c.cycles += 1;
                    mcus_load_card(c, m, A1);
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                    mcus_clear_device(c, m, 0x0023_dc8c)?;
                    mcus_load_card(c, m, A1);
                }
                break 'slot_done;
            }
            c.cycles += 7;
            mcus_card_call(c, A2, V1, V0, mcus::VT_DEVICE_OK, 0x0023_db44)?;
            if c.g[V0][0] != 0 {
                // A card arrived in the slot.
                c.cycles += 4;
                c.g[A1][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                c.g[V0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                if c.g[A1][0] == c.g[V0][0] {
                    c.cycles += 8;
                    c.g[V0][0] = mcus_at(m, mcus::MSG).lw(c);
                    c.g[A1][0] = 0x31;
                    let vt = r32(c, V0);
                    c.g[A0][0] = Addr(vt.wrapping_add(8)).lh(c);
                    c.g[V1][0] = Addr(vt.wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = sx(m.wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0023_db74)?;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::READ_DIR_INFO.call(c, 0x0023_db84)?;
                }
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            // The card left the slot: clear it, and if it was the selected one
            // and nothing is going on, move to the next slot.
            c.g[A0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
            c.cycles += 10;
            mcus_clear_device(c, m, 0x0023_dba4)?;
            c.g[V1][0] = mcus_at(m, mcus::SELECTED).lw(c);
            c.g[V0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            for (cb, ret) in [
                (mcus::CB_DEVICE_SELECTED, 0x0023_dbc0),
                (mcus::CB_SAVE_PRESSED, 0x0023_dbd4),
                (mcus::CB_LOAD_PRESSED, 0x0023_dbe8),
                (mcus::CB_DELETE_PRESSED, 0x0023_dbfc),
                (mcus::CB_FORMAT_PRESSED, 0x0023_dc10),
            ] {
                c.cycles += 5;
                c.g[V0][0] = mcus_at(m, cb).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, ret)?;
                if c.g[V0][0] != 0 {
                    mcus_load_card(c, m, A1);
                    break 'slot_done;
                }
            }
            for flag in [mcus::SAVE, mcus::DELETE] {
                c.cycles += 3;
                c.g[V0][0] = mcus_at(m, flag).lw(c);
                if c.g[V0][0] != 0 {
                    mcus_load_card(c, m, A1);
                    break 'slot_done;
                }
            }
            c.cycles += 5;
            c.g[V0][0] = mcus_at(m, mcus::CB_LOAD_PRESSED).lw(c);
            let target = r32(c, V0);
            call_ptr(c, target, 0x0023_dc3c)?;
            if c.g[V0][0] != 0 {
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            mcus_callee::NEXT_SLOT.call(c, 0x0023_dc4c)?;
            mcus_load_card(c, m, A1);
        }

        // Card idle: the check countdown and the next/previous slot inputs.
        c.cycles += 7;
        mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_dcb4)?;
        'inputs: {
            'inputs_done: {
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
                    break 'inputs_done;
                }
                c.cycles += 3;
                c.g[V0][0] = mcus_at(m, mcus::CHECK_CARD).lw(c);
                let checking = c.g[V0][0] != 0;
                c.g[V0][0] = 5;
                if checking {
                    c.cycles += 2;
                    mcus_at(m, mcus::CHECK_CARD).sw(c, 0);
                    mcus_at(m, mcus::CHECK_TICKS).sw(c, 5);
                }
                c.cycles += 4;
                c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
                let st = c.g[V1][0];
                c.g[V0][0] = 0x1d;
                if st != 7 {
                    c.cycles += 2;
                    c.g[V0][0] = 7;
                    if st != 0x1d {
                        break 'inputs;
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = mcus_at(m, mcus::CB_NEXT_DEVICE).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_dcf4)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    c.g[S0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::NEXT_SLOT.call(c, 0x0023_dd08)?;
                    c.g[V0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                    if c.g[S0][0] != c.g[V0][0] {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S1][0];
                        mcus_callee::ON_CARD_SCANNED.call(c, 0x0023_dd1c)?;
                    }
                    c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
                    break 'inputs_done;
                }
                c.g[V0][0] = mcus_at(m, mcus::CB_PREV_DEVICE).lw(c);
                c.cycles += 4;
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_dd2c)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    c.g[S0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::PREV_SLOT.call(c, 0x0023_dd40)?;
                    c.g[V0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                    if c.g[S0][0] != c.g[V0][0] {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        mcus_callee::ON_CARD_SCANNED.call(c, 0x0023_dd54)?;
                    }
                }
                c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
            }
            c.cycles += 1;
            c.g[V0][0] = 7;
        }

        // The slot scan: in the screens that watch the cards, every few
        // updates find the next present card and start a check on it.
        c.cycles += 2;
        let st = c.g[V1][0];
        c.g[A0][0] = 0;
        if mcus_scan_state(c, st) {
            c.cycles += 5;
            c.g[V0][0] = mcus_at(m, mcus::CHECK_TICKS).lw(c);
            c.g[V1][0] = mcus_at(m, mcus::CHECK_CARD).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let ticks = c.g[V0][0];
            mcus_at(m, mcus::CHECK_TICKS).sw(c, ticks);
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                if (ticks as i64) <= 0 {
                    c.cycles += 7;
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_dddc)?;
                    c.g[A0][0] = (c.g[V0][0] < 1) as u64;
                }
            }
        }
        c.cycles += 2;
        let go = c.g[A0][0] != 0;
        c.g[S2][0] = 0;
        if go {
            c.cycles += 3;
            c.g[S0][0] = 0;
            c.g[S3][0] = 1;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                mcus_callee::GET_NUM_SLOTS.call(c, 0x0023_dea4)?;
                let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                mcus_load_card(c, m, A2);
                if event_due(c) {
                    deliver_event_here(c, 0x0023_ddf8);
                }
                c.cycles += 8;
                c.g[A1][0] = c.g[S0][0];
                mcus_card_call(c, A2, V1, V0, mcus::VT_DEVICE_PRESENT, 0x0023_de10)?;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    continue;
                }
                c.cycles += 4;
                c.g[V0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                let after_last = (c.g[V0][0] as i64) < (c.g[S0][0] as i64);
                c.g[V0][0] = after_last as u64;
                if !after_last {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::GET_NUM_SLOTS.call(c, 0x0023_de30)?;
                    c.g[V1][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        continue;
                    }
                    c.cycles += 1;
                }
                let slot = c.g[S0][0];
                mcus_at(m, mcus::CURR_CHECK).sw(c, slot);
                // Check this slot.
                c.cycles += 22;
                c.g[S2][0] = c.g[S3][0];
                mcus_at(m, mcus::LAST_CHECK).sw(c, slot);
                c.g[A1][0] = slot;
                mcus_load_card(c, m, A3);
                c.g[V0][0] = slot ^ mcus_at(m, mcus::SELECTED).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = 0;
                }
                mcus_card_call(c, A3, A2, V1, mcus::VT_SET_CHECK_DEVICE, 0x0023_de74)?;
                let one = c.g[S3][0];
                mcus_at(m, mcus::CHECK_CARD).sw(c, one);
                mcus_load_card(c, m, A2);
                c.g[A1][0] = mcus_at(m, mcus::CURR_CHECK).lw(c);
                mcus_card_call(c, A2, V0, V1, mcus::VT_START_CHECK, 0x0023_de94)?;
                break;
            }
            c.cycles += 2;
            if c.g[S2][0] != 0 {
                // The checked slot is not the selected one: refresh it.
                c.cycles += 3;
                c.g[A1][0] = mcus_at(m, mcus::SELECTED).lw(c);
                c.g[A0][0] = c.g[S1][0];
                mcus_callee::REQUEST_CARD_INFO.call(c, 0x0023_dec4)?;
                c.cycles += 1;
            }
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = mcus_at(m, mcus::STATE).lw(c);

        // switch (mState): states 2..=56 through the jump table.
        c.cycles += 4;
        c.g[A0][0] = sx(r32(c, V0).wrapping_sub(2));
        c.g[V1][0] = (c.g[A0][0] < 0x37) as u64;
        c.g[V0][0] = 0x48_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(mcus::JUMP_TABLE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(mcus::JUMP_TABLE));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let block = r32(c, A0);
            let index = block.wrapping_sub(MCUS_CASE_BASE);
            if block == MCUS_CASE_STATE38 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                mcus_callee::STATE38.call(c, block + 8)?;
            } else if block != MCUS_CASE_NONE {
                if index % 0x10 != 0 || (index / 0x10) as usize >= MCUS_CASES.len() {
                    c.jump_out(block, 0x0023_d7d8, 0x0023_e264);
                    return Ok(());
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                MCUS_CASES[(index / 0x10) as usize].call(c, block + 8)?;
                frame.restore(c, S0, 0x40);
                mcus_epilogue(c, frame);
                return Ok(());
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        mcus_epilogue(c, frame);
        Ok(())
    })
}

/// `MEM_fill(&m->mDevices[a0], 0, 0xe0)` (`a0` holds the slot index).
fn mcus_clear_device(c: &mut Cpu, m: u32, ret: u32) -> Flow {
    c.g[A1][0] = 0;
    c.g[A2][0] = mcus::DEVICE_SIZE;
    c.mult(0, A0, A0, S0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(mcus::DEVICES));
    c.g[A0][0] = sx(m.wrapping_add(r32(c, A0)));
    call_ptr(c, mcus_callee::MEM_FILL, ret)
}

/// The states that count `mCheckDeviceTicks` down (7, 0x16, 0x14, 0x1d, 4,
/// 0x17, 0x18, 9, 0xa), as the compiled compare chain tests them (`v0` = 7
/// on entry; leaves `v0` as the chain does).
fn mcus_scan_state(c: &mut Cpu, st: u64) -> bool {
    if st == 7 {
        return true;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x14;
    if st == 0x16 {
        return true;
    }
    for (cur, next) in [
        (0x14, 0x1d),
        (0x1d, 4),
        (4, 0x17),
        (0x17, 0x18),
        (0x18, 9),
        (9, 0xa),
    ] {
        c.cycles += 2;
        c.g[V0][0] = next;
        if st == cur {
            return true;
        }
    }
    c.cycles += 2;
    st == 0xa
}

/// Reload s1..s3 and `ra`, pop the 0x50 frame and return.
fn mcus_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{Rng, DATA};

    /// The manager, its card driver object, the driver's vtable and the
    /// message vtable.
    const MCUS_M: u32 = DATA + 0x1_0000;
    const MCUS_CARD: u32 = DATA + 0x2_0000;
    const MCUS_VT: u32 = DATA + 0x2_1000;
    const MCUS_MSG_VT: u32 = DATA + 0x2_2000;
    /// Real leaves: `return 0` and `return 1`.
    const MCUS_RET_ZERO: u32 = 0x001a_8828;
    const MCUS_RET_ONE: u32 = 0x001c_5dd0;
    /// Case blocks whose handlers stay short on this fixture (State13, 23,
    /// 2A, 2D, 2E, 37, 1B, 38), the no-handler target and an address outside
    /// the function (the `jump_out` path).
    const MCUS_TEST_BLOCKS: [u32; 10] = [
        0x0023_dfe0,
        0x0023_e0a0,
        0x0023_e1c0,
        0x0023_e1f0,
        0x0023_e200,
        0x0023_e230,
        0x0023_e160,
        0x0023_e240,
        0x0023_e248,
        0x0000_5000,
    ];
    /// States whose real jump-table entry is one of those blocks (8 has no
    /// handler), or outside the table.
    const MCUS_REAL_TABLE_STATES: [u32; 13] = [
        8,
        0x13,
        0x23,
        0x2a,
        0x2d,
        0x2e,
        0x37,
        0x1b,
        0x38,
        0,
        1,
        0x39,
        0xffff_ffff,
    ];
    /// States the earlier sections test for.
    const MCUS_BRANCH_STATES: [u32; 20] = [
        7, 0x1d, 0x14, 0x24, 0x25, 0x1c, 0xc, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x16, 4, 0x17,
        0x18, 9, 0xa, 2,
    ];

    /// The game's jump table for the switch (55 words from the ELF).
    fn mcus_real_table() -> &'static [u32] {
        static TABLE: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
        TABLE.get_or_init(|| {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../orig/SLUS_207.72");
            let d = std::fs::read(path).expect("orig/SLUS_207.72");
            let u32_at = |o: usize| u32::from_le_bytes(d[o..o + 4].try_into().unwrap());
            let ph = u32_at(0x1c) as usize;
            let (off, vaddr) = (u32_at(ph + 4) as usize, u32_at(ph + 8));
            let base = off + (mcus::JUMP_TABLE - vaddr) as usize;
            (0..55).map(|i| u32_at(base + 4 * i)).collect()
        })
    }

    /// A method result: 0, 1 or a (logged) fake.
    fn mcus_pick(r: &mut Rng) -> u32 {
        match r.next_u32() % 3 {
            0 => MCUS_RET_ZERO,
            1 => MCUS_RET_ONE,
            _ => FAKE,
        }
    }

    /// An input callback: mostly "not pressed".
    fn mcus_pick_input(r: &mut Rng) -> u32 {
        if r.next_u32() % 3 != 0 {
            MCUS_RET_ZERO
        } else {
            mcus_pick(r)
        }
    }

    fn mcus_slot(c: &mut Cpu, vt: u32, off: u32, delta: u32, target: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, target);
    }

    /// `MCManager_UpdateStates` against the generated body: every section's
    /// branches (busy/idle card, the state chains, the slot refresh, the
    /// next/previous inputs, the slot scan) and the switch through the real
    /// jump table (for states whose handlers stay short) or a test table of
    /// short handlers, the no-handler entry and an outside address.
    #[test]
    fn mc_manager_update_states_matches() {
        fakes::covered(
            ee::s06::f_0023d7d8__generated,
            mc_manager_update_states,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0023_d7d8, 24);
                c.pc = 0x0023_d7d8;
                let m = MCUS_M;
                c.g[A0][0] = sx(m);

                // The card driver: every slot a fake, then the ones this
                // function (and its short callees) branch on.
                c.bus.write32(m + mcus::CARD, MCUS_CARD);
                c.bus.write32(MCUS_CARD, MCUS_VT);
                for off in (0..0x240).step_by(8) {
                    let delta = r.next_u32() & 0x1c;
                    mcus_slot(c, MCUS_VT, off, delta, FAKE);
                }
                let max_devices = if seed % 3 == 0 {
                    (r.next_u32() % 3 + 1) as i32
                } else {
                    -1
                };
                for off in [
                    mcus::VT_CAN_SAVE,
                    0x208,
                    0x210,
                    mcus::VT_CAN_LOAD,
                    0x1d0,
                    0x198,
                    0x1d8,
                    mcus::VT_FILE_EXISTS,
                    mcus::VT_CARD_CHANGED,
                    mcus::VT_DEVICE_PRESENT,
                ] {
                    let t = mcus_pick(&mut r);
                    mcus_slot(c, MCUS_VT, off, 0, t);
                }
                let busy = if r.next_u32() % 2 == 0 {
                    MCUS_RET_ZERO
                } else {
                    mcus_pick(&mut r)
                };
                mcus_slot(c, MCUS_VT, mcus::VT_BUSY, 0, busy);
                // With a known slot count the slot stepping searches: keep it
                // finding a slot at once.
                let device_ok = if max_devices >= 0 {
                    if r.next_u32() % 2 == 0 {
                        MCUS_RET_ONE
                    } else {
                        FAKE
                    }
                } else {
                    mcus_pick(&mut r)
                };
                mcus_slot(c, MCUS_VT, mcus::VT_DEVICE_OK, 0, device_ok);
                c.bus.write32(m + mcus::MSG, MCUS_MSG_VT);
                mcus_slot(c, MCUS_MSG_VT, 8, r.next_u32() & 0x1c, FAKE);

                // Input callbacks and flags.
                for cb in [
                    mcus::CB_SAVE_PRESSED,
                    mcus::CB_DELETE_PRESSED,
                    mcus::CB_DEVICE_SELECTED,
                    mcus::CB_LOAD_PRESSED,
                    mcus::CB_NEXT_DEVICE,
                    mcus::CB_PREV_DEVICE,
                    mcus::CB_FORMAT_PRESSED,
                    0xac,
                    0x30,
                ] {
                    let t = mcus_pick_input(&mut r);
                    c.bus.write32(m + cb, t);
                }
                c.bus.write32(m + 0x6c, MCUS_RET_ZERO);
                for flag in [mcus::SAVE, mcus::DELETE] {
                    let v = if r.next_u32() % 3 == 0 { 1 } else { 0 };
                    c.bus.write32(m + flag, v);
                }
                c.bus.write32(m + mcus::FILE_CURSOR, r.next_u32() % 6);
                c.bus.write32(m + mcus::CHECK_CARD, r.next_u32() % 2);
                c.bus
                    .write32(m + mcus::CHECK_TICKS, (r.next_u32() % 5).wrapping_sub(1));
                c.bus.write32(m + 0x424, max_devices as u32);
                c.bus.write32(m + mcus::SELECTED, r.next_u32() % 3);
                c.bus
                    .write32(m + mcus::LAST_CHECK, (r.next_u32() % 4).wrapping_sub(1));
                for i in -1i32..4 {
                    let at = m
                        .wrapping_add(mcus::DEVICE_PLUGGED_IN)
                        .wrapping_add((i * 0xe0) as u32);
                    c.bus.write32(at, r.next_u32() % 2);
                }
                c.bus.write32(m + 0x100, r.next_u32() % 2);
                c.bus.write32(m + 0x444, r.next_u32() % 3);

                // The state and the switch's table.
                let real = seed % 4 == 0;
                let state = if real {
                    MCUS_REAL_TABLE_STATES[(r.next_u32() % 13) as usize]
                } else {
                    MCUS_BRANCH_STATES[(r.next_u32() % 20) as usize]
                };
                c.bus.write32(m + mcus::STATE, state);
                for i in 0..55u32 {
                    let entry = if real {
                        mcus_real_table()[i as usize]
                    } else {
                        MCUS_TEST_BLOCKS[(r.next_u32() % 10) as usize]
                    };
                    c.bus.write32(mcus::JUMP_TABLE + 4 * i, entry);
                }
            },
        );
    }
}
