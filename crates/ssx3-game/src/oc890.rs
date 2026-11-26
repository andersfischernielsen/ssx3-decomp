//! Staging module oc890 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc890`.
//!
//! The `cFEStateCreateStep2_*` family of the account-creation screen (shard
//! s03). Skipped: `cFEStateCreateStep2_OnChildEvent` (0x1baa88) and
//! `cFEStateCreateStep2_OnCreatedPopupEvent` (0x1baed8) are resumable
//! dispatchers with `__entries` twins; the orchestrator converts those.

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
        0x001b_a8b8,
        cfe_state_create_step2_v25,
        "cFEStateCreateStep2_v25",
    ),
    (
        0x001b_ab28,
        cfe_state_create_step2_v31,
        "cFEStateCreateStep2_v31",
    ),
    (
        0x001b_ac90,
        cfe_state_create_step2_on_submit_result,
        "cFEStateCreateStep2_OnSubmitResult",
    ),
    (
        0x001b_ade8,
        cfe_state_create_step2_on_error_popup_event,
        "cFEStateCreateStep2_OnErrorPopupEvent",
    ),
    (
        0x001b_b098,
        cfe_state_create_step2_on_age_popup_event,
        "cFEStateCreateStep2_OnAgePopupEvent",
    ),
    (
        0x001b_b158,
        cfe_state_create_step2_on_save_account_answer,
        "cFEStateCreateStep2_OnSaveAccountAnswer",
    ),
];

/// The game's string constants live at `0x460000 + off` (lui 0x46).
const CS2_STR_BASE: u32 = 0x0046_0000;
/// `StrHash` (`GetHashValue32`).
const CS2_STR_HASH: u32 = 0x0031_7670;
/// `EA_MemAllocNamed`.
const CS2_ALLOC: u32 = 0x0031_7d70;
/// `cBXString_assign_cstr`.
const CS2_CSTR_ASSIGN: u32 = 0x0031_84e8;
/// `cFEStateNetHelpPopup_setNumHelpOptions` / `setDescription` / `setOption`.
const CS2_SET_NUM_HELP_OPTIONS: u32 = 0x001d_a528;
const CS2_SET_HELP_DESCRIPTION: u32 = 0x001d_a530;
const CS2_SET_HELP_OPTION: u32 = 0x001d_a510;
/// `cFEPopupConfirm_CfgSetOptionTextId`.
const CS2_CFG_SET_OPTION: u32 = 0x001c_57c0;
/// `NetFEState_GetPopup` / `ShowPopup` / `OnChildEvent` / `HandleNetEvent` /
/// `GotoNextState`.
const CS2_GET_POPUP: u32 = 0x001a_9c50;
const CS2_SHOW_POPUP: u32 = 0x001a_9bc0;
const CS2_ON_CHILD_EVENT: u32 = 0x001a_97b8;
const CS2_HANDLE_NET_EVENT: u32 = 0x001a_8e40;
const CS2_GOTO_NEXT_STATE: u32 = 0x001a_8a40;
/// `cFEStateCreateStep2_CheckAge` and `cFEStateCreateStep2_OnSubmitResult`.
const CS2_CHECK_AGE: u32 = 0x001b_bac8;
const CS2_ON_SUBMIT_RESULT: u32 = 0x001b_ac90;
/// `cFEStateCreateDuplicate_ctor`, `cFEStateSelectPersona_ctor`,
/// `cFEStateNetAccountMem_ctor`.
const CS2_DUP_CTOR: u32 = 0x001b_bd08;
const CS2_SELECT_PERSONA_CTOR: u32 = 0x001a_d130;
const CS2_ACCOUNT_MEM_CTOR: u32 = 0x001b_6ed0;
/// `LobbyMan_CreateAccount`, `ResetAccountInfo`, `Disconnect`, `Login`.
const CS2_LOBBY_CREATE_ACCOUNT: u32 = 0x0025_b9c8;
const CS2_LOBBY_RESET_ACCOUNT: u32 = 0x0026_0f10;
const CS2_LOBBY_DISCONNECT: u32 = 0x0025_b800;
const CS2_LOBBY_LOGIN: u32 = 0x0025_c610;
/// `BuddyMan_Create`, `IsConnected`, `Reconnect`, `Disconnect`.
const CS2_BUDDY_CREATE: u32 = 0x0026_0f80;
const CS2_BUDDY_IS_CONNECTED: u32 = 0x0026_1460;
const CS2_BUDDY_RECONNECT: u32 = 0x0026_13c8;
const CS2_BUDDY_DISCONNECT: u32 = 0x0026_1408;
/// The UI state manager's push-popup call.
const CS2_UI_PUSH_POPUP: u32 = 0x0039_f290;
/// The LobbyMan global (`gp - 0xc8`) and the BuddyMan global (`gp + 0x238`).
const CS2_GP_LOBBY: i32 = -0xc8;
const CS2_GP_BUDDY: i32 = 0x238;

/// `addr = base + off` (the 32-bit address a guest pointer plus field offset).
#[inline(always)]
fn cs2_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `lw` of a global at `gp + off`.
fn cs2_gp_word(c: &mut Cpu, off: i32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_add(off as u32);
    c.lw(a)
}

/// `StrHash(string at addr)`: the call returns the hash in v0.
fn cs2_hash(c: &mut Cpu, addr: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(addr);
    call_ptr(c, CS2_STR_HASH, ret)
}

/// `setDescription(menu, index, text)`.
fn cs2_set_description(c: &mut Cpu, index: u64, text: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = text;
    c.g[A1][0] = index;
    call_ptr(c, CS2_SET_HELP_DESCRIPTION, ret)
}

/// `setOption(menu, index, text, value)`.
fn cs2_set_option(c: &mut Cpu, index: u64, text: u64, value: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = text;
    c.g[A1][0] = index;
    c.g[A3][0] = value;
    call_ptr(c, CS2_SET_HELP_OPTION, ret)
}

/// The restore-and-return tail shared by the paths that end at `lq 16` (the
/// callee-saved `s0` slot) before the common epilogue.
fn cs2_restore_s0(c: &mut Cpu, frame: Frame, off: u32) {
    frame.restore(c, S0, off);
}

/// `void cFEStateCreateStep2_v25(cFEStateCreateStep2* self, cMenu* menu)`
/// (0x1ba8b8): fills the help popup for the account form: the item-18
/// caption, three field descriptions and five option labels (the numbers
/// are the highlight targets), and item 22's note.
pub fn cfe_state_create_step2_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);

        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, CS2_SET_NUM_HELP_OPTIONS, 0x001b_a8d4)?;

        cs2_hash(c, CS2_STR_BASE + 0x48a8, 0x001b_a8e0)?;
        let v = c.g[V0][0];
        c.sw(cs2_at(c.g[S0][0], 0x54), v);

        cs2_hash(c, CS2_STR_BASE + 0x48c8, 0x001b_a8f0)?;
        let text = c.g[V0][0];
        cs2_set_description(c, 0, text, 0x001b_a900)?;

        cs2_hash(c, CS2_STR_BASE + 0x48e0, 0x001b_a90c)?;
        let text = c.g[V0][0];
        cs2_set_description(c, 1, text, 0x001b_a91c)?;

        cs2_hash(c, CS2_STR_BASE + 0x48f8, 0x001b_a928)?;
        let text = c.g[V0][0];
        cs2_set_description(c, 2, text, 0x001b_a938)?;

        cs2_hash(c, CS2_STR_BASE - 0x1a0, 0x001b_a944)?;
        let text = c.g[V0][0];
        cs2_set_option(c, 0, text, 3, 0x001b_a958)?;

        cs2_hash(c, CS2_STR_BASE - 0x1c0, 0x001b_a964)?;
        let text = c.g[V0][0];
        cs2_set_option(c, 1, text, 4, 0x001b_a978)?;

        cs2_hash(c, CS2_STR_BASE + 0x2c50, 0x001b_a984)?;
        let text = c.g[V0][0];
        cs2_set_option(c, 2, text, 0, 0x001b_a998)?;

        cs2_hash(c, CS2_STR_BASE + 0x14e8, 0x001b_a9a4)?;
        let text = c.g[V0][0];
        cs2_set_option(c, 3, text, 1, 0x001b_a9b8)?;

        cs2_hash(c, CS2_STR_BASE + 0x4910, 0x001b_a9c4)?;
        let text = c.g[V0][0];
        cs2_set_option(c, 4, text, 2, 0x001b_a9d8)?;

        cs2_hash(c, CS2_STR_BASE + 0x4930, 0x001b_a9e4)?;
        let v = c.g[V0][0];
        c.sw(cs2_at(c.g[S0][0], 0x64), v);

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateCreateStep2_v31` (0x1bab28): the switch on the popup id (a1)
/// with the 203 yes/no popup, 201 lobby create, 212 persona select, 258
/// submit result and the default net-event handler.
pub fn cfe_state_create_step2_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xcb;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);

        cs2_v31_dispatch(c, frame)?;

        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The switch of [`cfe_state_create_step2_v31`]; every path ends with its
/// `lq 16` (`s0`), and the `-1` results add the tail's extra cycle.
fn cs2_v31_dispatch(c: &mut Cpu, frame: Frame) -> Flow {
    if c.g[A1][0] == 0xcb {
        return cs2_v31_case_203(c, frame);
    }
    c.cycles += 3;
    let below = (c.g[A1][0] as i64) < 204;
    c.g[V0][0] = below as u64;
    c.g[V0][0] = 0xc9;
    if !below {
        c.cycles += 3;
        c.g[V0][0] = 0xd4;
        let is_212 = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = 0x102;
        if is_212 {
            return cs2_v31_case_212(c, frame);
        }
        c.cycles += 2;
        let is_258 = c.g[A1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        if is_258 {
            return cs2_v31_case_258(c, frame);
        }
        c.cycles += 2;
        return cs2_v31_default(c, frame);
    }
    c.cycles += 2;
    let is_201 = c.g[A1][0] == c.g[V0][0];
    c.g[A0][0] = cs2_gp_word(c, CS2_GP_LOBBY);
    if is_201 {
        return cs2_v31_case_201(c, frame);
    }
    c.cycles += 2;
    cs2_v31_default(c, frame)
}

/// Case 203: the "congratulations" yes/no popup; the result is -1.
fn cs2_v31_case_203(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 34;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CS2_GET_POPUP, 0x001b_ab98)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = 2;
    c.sw(cs2_at(c.g[S0][0], 0xbc), c.g[V0][0]);
    c.g[S1][0] = sx(cs2_at(c.g[S0][0], 0xbc));
    cs2_hash(c, CS2_STR_BASE + 0x4ad0, 0x001b_abb4)?;
    c.g[V1][0] = 3;
    c.sw(cs2_at(c.g[S0][0], 0xc0), c.g[V1][0]);
    c.sw(cs2_at(c.g[S0][0], 0xc4), c.g[V0][0]);
    cs2_hash(c, CS2_STR_BASE - 0x2338, 0x001b_abcc)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, CS2_CFG_SET_OPTION, 0x001b_abdc)?;
    cs2_hash(c, CS2_STR_BASE - 0x328, 0x001b_abe8)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 1;
    call_ptr(c, CS2_CFG_SET_OPTION, 0x001b_abf8)?;
    c.g[V0][0] = 0x81f;
    c.sw(cs2_at(c.g[S0][0], 0x164), 0);
    c.sw(cs2_at(c.g[S0][0], 0x150), c.g[V0][0]);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, CS2_SHOW_POPUP, 0x001b_ac10)?;
    c.g[V0][0] = u64::MAX;
    c.cycles += 1;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// Case 201: `LobbyMan_CreateAccount(lobby, lobby + 0x260)`; result -1.
fn cs2_v31_case_201(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    let lobby = c.g[A0][0] as u32;
    c.g[A1][0] = sx(lobby.wrapping_add(0x260));
    call_ptr(c, CS2_LOBBY_CREATE_ACCOUNT, 0x001b_ab88)?;
    c.g[V0][0] = u64::MAX;
    c.cycles += 1;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// Case 212: reset the lobby's account, allocate and build the persona
/// select state, and go to it; result -1.
fn cs2_v31_case_212(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 16;
    c.g[A0][0] = cs2_gp_word(c, CS2_GP_LOBBY);
    call_ptr(c, CS2_LOBBY_RESET_ACCOUNT, 0x001b_ac20)?;
    c.g[A1][0] = CS2_STR_BASE as u64;
    c.g[A3][0] = 0;
    c.g[A1][0] = sx(CS2_STR_BASE.wrapping_add(0x2d68));
    c.g[A0][0] = 0x6d8;
    c.g[A2][0] = 0;
    call_ptr(c, CS2_ALLOC, 0x001b_ac38)?;
    c.g[A1][0] = c.lw(cs2_at(c.g[S2][0], 0x10));
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, CS2_SELECT_PERSONA_CTOR, 0x001b_ac44)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, CS2_GOTO_NEXT_STATE, 0x001b_ac50)?;
    c.g[V0][0] = u64::MAX;
    c.cycles += 1;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// Case 258: forward the submit result to `OnSubmitResult`; result -1.
fn cs2_v31_case_258(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = c.g[A2][0];
    call_ptr(c, CS2_ON_SUBMIT_RESULT, 0x001b_ac60)?;
    c.g[V0][0] = u64::MAX;
    c.cycles += 1;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// The default: `NetFEState_HandleNetEvent(self, id, data)`; the result is
/// whatever the handler returned.
fn cs2_v31_default(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CS2_HANDLE_NET_EVENT, 0x001b_ac70)?;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// `void cFEStateCreateStep2_OnSubmitResult(self, online_ErrorEvent* msg)`
/// (0x1bac90): dispatch on the error code: a duplicate account, too young
/// (age), a parental e-mail that is already set (palm), else the default.
pub fn cfe_state_create_step2_on_submit_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[V0][0] = 0x706d_616c;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[V1][0] = c.lw(cs2_at(c.g[A2][0], 0x8));
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];

        if c.g[V1][0] == c.g[V0][0] {
            cs2_submit_palm(c, frame)?;
        } else {
            cs2_submit_other(c, frame)?;
        }

        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The error codes other than the parental-e-mail one.
fn cs2_submit_other(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    // `mError > 0x706d616c` (unsigned, as the generated `sltu`-free compare).
    let above = c.g[V0][0] < c.g[V1][0];
    c.g[V0][0] = 0x746f_0000;
    if above {
        c.cycles += 3;
        c.g[V0][0] |= 0x6f79;
        if c.g[V1][0] == c.g[V0][0] {
            // 0x746f6f79: too young.
            c.g[A1][0] = c.lw(cs2_at(c.g[A2][0], 0xc));
            return cs2_submit_check_age(c, frame, 0x001b_ad34);
        }
        c.cycles += 2;
        c.g[A1][0] = c.lw(cs2_at(c.g[A2][0], 0));
        return cs2_submit_default(c, frame);
    }
    c.cycles += 4;
    c.g[V0][0] = 0x6475_706c;
    c.g[A1][0] = CS2_STR_BASE as u64;
    if c.g[V1][0] == c.g[V0][0] {
        return cs2_submit_duplicate(c, frame);
    }
    c.cycles += 2;
    c.g[A1][0] = c.lw(cs2_at(c.g[A2][0], 0));
    cs2_submit_default(c, frame)
}

/// 0x706d616c (the parental e-mail error): when the e-mail is set, show the
/// invalid-parental-email popup; else the age tail with 17.
fn cs2_submit_palm(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = c.lw(cs2_at(c.g[S2][0], 0x6d0));
    c.g[V1][0] = c.lw(cs2_at(c.g[V0][0], 0x10));
    c.g[A0][0] = c.lw(cs2_at(c.g[V1][0], (-8i32) as u32));
    let empty = c.g[A0][0] == 0;
    c.g[A0][0] = c.g[S2][0];
    if empty {
        c.g[A1][0] = 0x11;
        return cs2_submit_check_age(c, frame, 0x001b_adc0);
    }
    c.cycles += 26;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CS2_GET_POPUP, 0x001b_ad58)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = 1;
    c.sw(cs2_at(c.g[S0][0], 0xbc), c.g[V0][0]);
    c.g[S1][0] = sx(cs2_at(c.g[S0][0], 0xbc));
    cs2_hash(c, CS2_STR_BASE + 0x2338, 0x001b_ad74)?;
    c.g[V1][0] = 3;
    c.sw(cs2_at(c.g[S0][0], 0xc0), c.g[V1][0]);
    c.sw(cs2_at(c.g[S0][0], 0xc4), c.g[V0][0]);
    cs2_hash(c, CS2_STR_BASE + 0x1618, 0x001b_ad8c)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, CS2_CFG_SET_OPTION, 0x001b_ad9c)?;
    c.g[V0][0] = 0x822;
    c.g[A0][0] = c.g[S2][0];
    c.sw(cs2_at(c.g[S0][0], 0x150), c.g[V0][0]);
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, CS2_SHOW_POPUP, 0x001b_adb0)?;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// Duplicate account: a new `cFEStateCreateDuplicate` state.
fn cs2_submit_duplicate(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 14;
    c.g[A0][0] = 0x6e0;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3090));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, CS2_ALLOC, 0x001b_ad08)?;
    c.g[A1][0] = c.lw(cs2_at(c.g[S2][0], 0x10));
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = 1;
    call_ptr(c, CS2_DUP_CTOR, 0x001b_ad18)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, CS2_GOTO_NEXT_STATE, 0x001b_ad24)?;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// `cFEStateCreateStep2_CheckAge(self, age)` then the common tail.
fn cs2_submit_check_age(c: &mut Cpu, frame: Frame, ret: u32) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CS2_CHECK_AGE, ret)?;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// Unknown error codes: `NetFEState_HandleNetEvent(self, ...)`.
fn cs2_submit_default(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CS2_HANDLE_NET_EVENT, 0x001b_add0)?;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// `void cFEStateCreateStep2_OnErrorPopupEvent(self, popup, id)` (0x1bade8):
/// the error popups 0xf and 0x14 refresh the buddy and lobby connections and
/// build the account object; other ids only forward to the child handler.
pub fn cfe_state_create_step2_on_error_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let is_f = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !is_f {
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            if c.g[A2][0] != c.g[V0][0] {
                cs2_error_other(c, frame)?;
                return Ok(());
            }
        }
        cs2_error_connections(c, frame)?;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Any other error id: `NetFEState_OnChildEvent(self, popup, id)`.
fn cs2_error_other(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, CS2_ON_CHILD_EVENT, 0x001b_aec8)?;
    cs2_restore_s0(c, frame, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// The connection refresh of the error popup, then the account object.
fn cs2_error_connections(c: &mut Cpu, frame: Frame) -> Flow {
    // Child event, then create the buddy list when it is missing.
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, CS2_ON_CHILD_EVENT, 0x001b_ae14)?;
    c.g[V0][0] = cs2_gp_word(c, CS2_GP_BUDDY);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        call_ptr(c, CS2_BUDDY_CREATE, 0x001b_ae28)?;
    }

    // Reconnect the buddy list when it is not connected.
    c.cycles += 4;
    c.g[A0][0] = cs2_gp_word(c, CS2_GP_BUDDY);
    call_ptr(c, CS2_BUDDY_IS_CONNECTED, 0x001b_ae30)?;
    let connected = c.g[V0][0] != 0;
    c.g[V0][0] = cs2_gp_word(c, CS2_GP_BUDDY);
    if !connected {
        c.cycles += 3;
        c.g[A0][0] = cs2_gp_word(c, CS2_GP_BUDDY);
        call_ptr(c, CS2_BUDDY_RECONNECT, 0x001b_ae40)?;
        c.g[V0][0] = cs2_gp_word(c, CS2_GP_BUDDY);
    }

    // Disconnect the lobby when the buddy list has no connection.
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A0][0] = cs2_gp_word(c, CS2_GP_LOBBY);
        call_ptr(c, CS2_LOBBY_DISCONNECT, 0x001b_ae54)?;
    }

    c.cycles += 27;
    c.g[A0][0] = cs2_gp_word(c, CS2_GP_BUDDY);
    call_ptr(c, CS2_BUDDY_DISCONNECT, 0x001b_ae5c)?;
    c.g[A0][0] = cs2_gp_word(c, CS2_GP_LOBBY);
    call_ptr(c, CS2_LOBBY_DISCONNECT, 0x001b_ae64)?;

    // The account object: allocate, construct, and show it in the popup.
    c.g[A1][0] = sx(CS2_STR_BASE.wrapping_add(0x2d10));
    c.g[A0][0] = 0xb10;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, CS2_ALLOC, 0x001b_ae7c)?;
    c.g[A1][0] = c.lw(cs2_at(c.g[S0][0], 0x10));
    c.g[A0][0] = c.g[V0][0];
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    c.g[A2][0] = c.g[S0][0];
    c.g[A3][0] = 0;
    call_ptr(c, CS2_ACCOUNT_MEM_CTOR, 0x001b_ae98)?;
    c.g[V1][0] = 0x832;
    c.g[A1][0] = c.g[V0][0];
    c.sw(cs2_at(c.g[V0][0], 0x18), c.g[V1][0]);
    c.g[V0][0] = c.lw(cs2_at(c.g[S0][0], 0x40));
    c.g[V1][0] = c.lw(cs2_at(c.g[V0][0], 0xd0));
    c.g[A0][0] = c.lw(cs2_at(c.g[V1][0], 0x10));
    c.g[A0][0] = sx(cs2_at(c.g[A0][0], 0x18));
    call_ptr(c, CS2_UI_PUSH_POPUP, 0x001b_aeb8)?;
    cs2_restore_s0(c, frame, 0x10);
    Ok(())
}

/// `void cFEStateCreateStep2_OnAgePopupEvent(self, popup, id)` (0x1bb098):
/// id 0x16 runs the child handler and then the too-young tail (17);
/// other ids only the child handler.
pub fn cfe_state_create_step2_on_age_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x16;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let not_16 = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if not_16 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, CS2_ON_CHILD_EVENT, 0x001b_b0d4)?;
            cs2_restore_s0(c, frame, 0x10);
        } else {
            c.cycles += 7;
            c.g[A2][0] = 0x16;
            call_ptr(c, CS2_ON_CHILD_EVENT, 0x001b_b0b8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x11;
            call_ptr(c, CS2_CHECK_AGE, 0x001b_b0c4)?;
            cs2_restore_s0(c, frame, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateCreateStep2_OnSaveAccountAnswer(self, popup, id)`
/// (0x1bb158): the save-account answers 0x10, 0x14, 0xf and 0x11..; each
/// path ends in the account popup, the login, or the child handler.
pub fn cfe_state_create_step2_on_save_account_answer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x10;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        let is_10 = c.g[A2][0] == c.g[V0][0];
        frame.save_ra(c, 0);

        cs2_save_answer(c, frame, is_10)?;

        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

fn cs2_save_answer(c: &mut Cpu, frame: Frame, is_10: bool) -> Flow {
    if is_10 {
        return cs2_save_answer_popup(c, frame);
    }
    c.cycles += 3;
    let below_11 = c.g[A2][0] < 0x11;
    c.g[V0][0] = below_11 as u64;
    c.g[V0][0] = 0xf;
    if below_11 {
        c.cycles += 2;
        let is_f = c.g[A2][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        if is_f {
            return cs2_save_account(c, frame);
        }
        c.cycles += 2;
        return cs2_save_child(c, frame);
    }
    c.cycles += 3;
    c.g[V0][0] = 0x14;
    if c.g[A2][0] == c.g[V0][0] {
        return cs2_save_answer_popup(c, frame);
    }
    c.cycles += 2;
    cs2_save_child(c, frame)
}

/// `NetFEState_OnChildEvent(self, ...)` then restore; the generic answer.
fn cs2_save_child(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CS2_ON_CHILD_EVENT, 0x001b_b2b8)?;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// Answer 0x0f: build the account object, store it, and push the popup.
fn cs2_save_account(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 26;
    c.g[A2][0] = 0xf;
    call_ptr(c, CS2_ON_CHILD_EVENT, 0x001b_b1b0)?;
    c.g[A1][0] = CS2_STR_BASE as u64;
    c.g[A0][0] = 0xb10;
    c.g[A1][0] = sx(CS2_STR_BASE.wrapping_add(0x2d10));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, CS2_ALLOC, 0x001b_b1c8)?;
    c.g[V1][0] = c.lw(cs2_at(c.g[S2][0], 0x6d0));
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.lw(cs2_at(c.g[S2][0], 0x10));
    c.g[A2][0] = c.g[S2][0];
    c.g[T1][0] = c.lw(cs2_at(c.g[V1][0], 4));
    c.g[A3][0] = 1;
    c.g[T0][0] = c.lw(cs2_at(c.g[V1][0], 0));
    call_ptr(c, CS2_ACCOUNT_MEM_CTOR, 0x001b_b1e8)?;
    c.g[V1][0] = 0x82c;
    c.g[A1][0] = c.g[V0][0];
    c.sw(cs2_at(c.g[V0][0], 0x18), c.g[V1][0]);
    c.g[V0][0] = c.lw(cs2_at(c.g[S2][0], 0x40));
    c.g[V1][0] = c.lw(cs2_at(c.g[V0][0], 0xd0));
    c.g[A0][0] = c.lw(cs2_at(c.g[V1][0], 0x10));
    c.g[A0][0] = sx(cs2_at(c.g[A0][0], 0x18));
    call_ptr(c, CS2_UI_PUSH_POPUP, 0x001b_b208)?;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

/// Answers 0x10 (and 0x14): the popup with the account text, the login.
fn cs2_save_answer_popup(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 40;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CS2_ON_CHILD_EVENT, 0x001b_b218)?;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, CS2_GET_POPUP, 0x001b_b220)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = 1;
    c.sw(cs2_at(c.g[S0][0], 0xbc), c.g[V0][0]);
    c.g[S1][0] = sx(cs2_at(c.g[S0][0], 0xbc));
    cs2_hash(c, CS2_STR_BASE + 0x2c70, 0x001b_b23c)?;
    c.g[V1][0] = 3;
    c.sw(cs2_at(c.g[S0][0], 0xc0), c.g[V1][0]);
    c.sw(cs2_at(c.g[S0][0], 0xc4), c.g[V0][0]);
    cs2_hash(c, CS2_STR_BASE + 0x2c88, 0x001b_b254)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0;
    call_ptr(c, CS2_CFG_SET_OPTION, 0x001b_b264)?;
    c.g[V0][0] = 0x80e;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S0][0];
    c.sw(cs2_at(c.g[S0][0], 0x150), c.g[V0][0]);
    call_ptr(c, CS2_SHOW_POPUP, 0x001b_b278)?;
    c.g[V0][0] = c.lw(cs2_at(c.g[S2][0], 0x6d0));
    c.g[A0][0] = cs2_gp_word(c, CS2_GP_LOBBY);
    c.g[A1][0] = c.lw(cs2_at(c.g[V0][0], 0));
    c.g[A0][0] = sx(cs2_at(c.g[A0][0], 0x48));
    call_ptr(c, CS2_CSTR_ASSIGN, 0x001b_b28c)?;
    c.g[V1][0] = c.lw(cs2_at(c.g[S2][0], 0x6d0));
    c.g[A0][0] = cs2_gp_word(c, CS2_GP_LOBBY);
    c.g[A1][0] = c.lw(cs2_at(c.g[V1][0], 4));
    c.g[A0][0] = sx(cs2_at(c.g[A0][0], 0x4c));
    call_ptr(c, CS2_CSTR_ASSIGN, 0x001b_b2a0)?;
    c.g[A0][0] = cs2_gp_word(c, CS2_GP_LOBBY);
    call_ptr(c, CS2_LOBBY_LOGIN, 0x001b_b2a8)?;
    cs2_restore_s0(c, frame, 0x30);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the fakes act only when the test machine holds it.
    pub(crate) const OC890_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC890_MAGIC: u32 = 0x890c_890c;
    /// The object graph the six functions read.
    pub(crate) const OC890_SELF: u32 = DATA + 0x1000;
    pub(crate) const OC890_MENU: u32 = DATA + 0x2000;
    pub(crate) const OC890_ACCT: u32 = DATA + 0x3000;
    pub(crate) const OC890_ACCT2: u32 = DATA + 0x4000;
    pub(crate) const OC890_STR: u32 = DATA + 0x6000;
    pub(crate) const OC890_STR2: u32 = DATA + 0x7000;
    pub(crate) const OC890_MSG: u32 = DATA + 0x8000;
    pub(crate) const OC890_LOBBY: u32 = DATA + 0x9000;
    pub(crate) const OC890_BUDDY: u32 = DATA + 0xa000;

    /// A fake that logs the call and returns to `ra`, for every callee that
    /// has an override slot. Each one is gated on [`OC890_CTRL`]; with the
    /// gate off it runs the body it replaced (`fall_back`), so other tests
    /// in the binary keep their callees.
    macro_rules! oc890_gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC890_CTRL) != OC890_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    oc890_gated_fake!(
        oc890_fake_strhash,
        OC890_PREV_STRHASH,
        0x0031_7670,
        ee::s10::f_00317670__generated
    );
    oc890_gated_fake!(
        oc890_fake_alloc,
        OC890_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    oc890_gated_fake!(
        oc890_fake_cstr_assign,
        OC890_PREV_CSTR,
        0x0031_84e8,
        ee::s10::f_003184e8__generated
    );
    oc890_gated_fake!(
        oc890_fake_set_desc,
        OC890_PREV_SET_DESC,
        0x001d_a530,
        ee::s04::f_001da530__generated
    );
    oc890_gated_fake!(
        oc890_fake_set_opt,
        OC890_PREV_SET_OPT,
        0x001d_a510,
        ee::s04::f_001da510__generated
    );
    oc890_gated_fake!(
        oc890_fake_cfg_opt,
        OC890_PREV_CFG_OPT,
        0x001c_57c0,
        ee::s03::f_001c57c0__generated
    );
    oc890_gated_fake!(
        oc890_fake_show_popup,
        OC890_PREV_SHOW,
        0x001a_9bc0,
        ee::s03::f_001a9bc0__generated
    );
    oc890_gated_fake!(
        oc890_fake_dup_ctor,
        OC890_PREV_DUP,
        0x001b_bd08,
        ee::s03::f_001bbd08__generated
    );
    oc890_gated_fake!(
        oc890_fake_persona_ctor,
        OC890_PREV_PERSONA,
        0x001a_d130,
        ee::s03::f_001ad130__generated
    );
    oc890_gated_fake!(
        oc890_fake_create_account,
        OC890_PREV_CREATE,
        0x0025_b9c8,
        ee::s06::f_0025b9c8__generated
    );
    oc890_gated_fake!(
        oc890_fake_reset_account,
        OC890_PREV_RESET,
        0x0026_0f10,
        ee::s06::f_00260f10__generated
    );
    oc890_gated_fake!(
        oc890_fake_lobby_disconnect,
        OC890_PREV_LDISC,
        0x0025_b800,
        ee::s06::f_0025b800__generated
    );
    oc890_gated_fake!(
        oc890_fake_lobby_login,
        OC890_PREV_LOGIN,
        0x0025_c610,
        ee::s06::f_0025c610__generated
    );
    oc890_gated_fake!(
        oc890_fake_buddy_create,
        OC890_PREV_BCREATE,
        0x0026_0f80,
        ee::s06::f_00260f80__generated
    );
    oc890_gated_fake!(
        oc890_fake_buddy_connected,
        OC890_PREV_BCONN,
        0x0026_1460,
        ee::s06::f_00261460__generated
    );
    oc890_gated_fake!(
        oc890_fake_buddy_reconnect,
        OC890_PREV_BRECON,
        0x0026_13c8,
        ee::s06::f_002613c8__generated
    );
    oc890_gated_fake!(
        oc890_fake_buddy_disconnect,
        OC890_PREV_BDISC,
        0x0026_1408,
        ee::s06::f_00261408__generated
    );

    /// Install every gated fake once per process.
    pub(crate) fn oc890_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc890_fake_strhash,
            &OC890_PREV_STRHASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc890_fake_alloc,
            &OC890_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003184E8),
            oc890_fake_cstr_assign,
            &OC890_PREV_CSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001DA530),
            oc890_fake_set_desc,
            &OC890_PREV_SET_DESC,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001DA510),
            oc890_fake_set_opt,
            &OC890_PREV_SET_OPT,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C57C0),
            oc890_fake_cfg_opt,
            &OC890_PREV_CFG_OPT,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A9BC0),
            oc890_fake_show_popup,
            &OC890_PREV_SHOW,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001BBD08),
            oc890_fake_dup_ctor,
            &OC890_PREV_DUP,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001AD130),
            oc890_fake_persona_ctor,
            &OC890_PREV_PERSONA,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025B9C8),
            oc890_fake_create_account,
            &OC890_PREV_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00260F10),
            oc890_fake_reset_account,
            &OC890_PREV_RESET,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025B800),
            oc890_fake_lobby_disconnect,
            &OC890_PREV_LDISC,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025C610),
            oc890_fake_lobby_login,
            &OC890_PREV_LOGIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00260F80),
            oc890_fake_buddy_create,
            &OC890_PREV_BCREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261460),
            oc890_fake_buddy_connected,
            &OC890_PREV_BCONN,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002613C8),
            oc890_fake_buddy_reconnect,
            &OC890_PREV_BRECON,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261408),
            oc890_fake_buddy_disconnect,
            &OC890_PREV_BDISC,
        );
    }

    /// The world the functions read: the account object at `self` (account
    /// +0x40, string table +0x6d0), the message at `msg` (type, error +8,
    /// age +0xc), the LobbyMan and BuddyMan globals. Zero lengths and empty
    /// buddy lists come from `seed`, so both sides of each branch run.
    pub(crate) fn oc890_world(c: &mut Cpu, seed: u32, msg_error: u32) {
        c.bus.write32(OC890_CTRL, OC890_MAGIC);
        c.bus.write32(OC890_SELF + 0x10, OC890_MSG);
        c.bus.write32(OC890_SELF + 0x40, OC890_ACCT);
        c.bus.write32(OC890_ACCT + 0xd0, OC890_ACCT2);
        c.bus.write32(OC890_ACCT2 + 0x10, OC890_STR);
        c.bus.write32(OC890_SELF + 0x6d0, OC890_STR);
        c.bus.write32(OC890_STR + 0x10, OC890_STR2);
        let len = if seed % 2 == 0 { 0 } else { 12 };
        c.bus.write32(OC890_STR2 - 8, len);
        c.bus.write32(OC890_STR, OC890_STR2);
        c.bus.write32(OC890_STR + 4, OC890_STR2);
        c.bus.write32(OC890_MSG, seed.wrapping_mul(0x9e37_79b9));
        c.bus.write32(OC890_MSG + 8, msg_error);
        c.bus.write32(OC890_MSG + 0xc, 17 + seed % 9);
        c.bus.write32(OC890_MENU, 0);
        let lobby_global = (GAME_GP).wrapping_sub(0xc8);
        c.bus.write32(lobby_global, OC890_LOBBY);
        let buddy_global = GAME_GP + 0x238;
        c.bus
            .write32(buddy_global, if seed % 3 == 0 { 0 } else { OC890_BUDDY });
    }

    #[test]
    fn cfe_state_create_step2_v25_matches() {
        oc890_hook();
        check(
            ee::s03::f_001ba8b8__generated,
            cfe_state_create_step2_v25,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0x25, 6);
                oc890_world(c, seed, 0);
                c.g[A0][0] = sx(OC890_SELF);
                c.g[A1][0] = sx(OC890_MENU);
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_v31_matches() {
        oc890_hook();
        const IDS: [i64; 8] = [203, 201, 212, 258, 0x10, 0x14, 300, -16];
        check(
            ee::s03::f_001bab28__generated,
            cfe_state_create_step2_v31,
            300,
            &|c, seed| {
                fakes::machine(c, seed, 0x31, 6);
                oc890_world(c, seed, 0x746f_6f79);
                c.g[A0][0] = sx(OC890_SELF);
                c.g[A1][0] = IDS[(seed % 8) as usize] as u64;
                c.g[A2][0] = sx(OC890_MSG);
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_on_submit_result_matches() {
        oc890_hook();
        const ERRORS: [u32; 6] = [
            0x706d_616c,
            0x6475_706c,
            0x746f_6f79,
            0x10,
            0x7000_0000,
            0xffff_fff0,
        ];
        check(
            ee::s03::f_001bac90__generated,
            cfe_state_create_step2_on_submit_result,
            300,
            &|c, seed| {
                fakes::machine(c, seed, 0x90, 6);
                oc890_world(c, seed, ERRORS[(seed % 6) as usize]);
                c.g[A0][0] = sx(OC890_SELF);
                c.g[A1][0] = sx(OC890_MSG);
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_on_error_popup_event_matches() {
        oc890_hook();
        const IDS: [u32; 4] = [0xf, 0x14, 0x10, 0x11];
        check(
            ee::s03::f_001bade8__generated,
            cfe_state_create_step2_on_error_popup_event,
            300,
            &|c, seed| {
                fakes::machine(c, seed, 0xade8, 6);
                oc890_world(c, seed, 0);
                c.g[A0][0] = sx(OC890_SELF);
                c.g[A1][0] = sx(OC890_MSG);
                c.g[A2][0] = IDS[(seed % 4) as usize] as u64;
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_on_age_popup_event_matches() {
        oc890_hook();
        const IDS: [u32; 2] = [0x16, 0x10];
        check(
            ee::s03::f_001bb098__generated,
            cfe_state_create_step2_on_age_popup_event,
            200,
            &|c, seed| {
                fakes::machine(c, seed, 0xb098, 4);
                oc890_world(c, seed, 0);
                c.g[A0][0] = sx(OC890_SELF);
                c.g[A1][0] = sx(OC890_MSG);
                c.g[A2][0] = IDS[(seed % 2) as usize] as u64;
            },
        );
    }

    #[test]
    fn cfe_state_create_step2_on_save_account_answer_matches() {
        oc890_hook();
        const IDS: [u32; 5] = [0x10, 0x14, 0xf, 0x11, 0x20];
        check(
            ee::s03::f_001bb158__generated,
            cfe_state_create_step2_on_save_account_answer,
            300,
            &|c, seed| {
                fakes::machine(c, seed, 0xb158, 6);
                oc890_world(c, seed, 0);
                c.g[A0][0] = sx(OC890_SELF);
                c.g[A1][0] = sx(OC890_MSG);
                c.g[A2][0] = IDS[(seed % 5) as usize] as u64;
            },
        );
    }
}
