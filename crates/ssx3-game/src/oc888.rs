//! Staging module oc888 (wave wave252): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc888`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_89e8,
        cfe_state_create_step1_update,
        "cFEStateCreateStep1_Update",
    ),
    (
        0x001b_8aa0,
        cfe_state_create_step1_on_widget_created,
        "cFEStateCreateStep1_OnWidgetCreated",
    ),
    (
        0x001b_9088,
        cfe_state_create_step1_v25,
        "cFEStateCreateStep1_v25",
    ),
    (
        0x001b_91c8,
        cfe_state_create_step1_on_save_account_answer,
        "cFEStateCreateStep1_OnSaveAccountAnswer",
    ),
    (
        0x001b_9488,
        cfe_state_create_step1_v31,
        "cFEStateCreateStep1_v31",
    ),
];

/// `NetFEState_Update` (0x1a8770).
const STEP1_NET_UPDATE: Callee = Callee {
    addr: 0x001a_8770,
    f: ee::s03::f_001a8770,
};
/// `NetFEState_FillYesNoList` (0x1a9a18).
const STEP1_NET_FILL_YES_NO: Callee = Callee {
    addr: 0x001a_9a18,
    f: ee::s03::f_001a9a18,
};
/// `NetFEState_OnChildEvent` (0x1a97b8).
const STEP1_NET_ON_CHILD: Callee = Callee {
    addr: 0x001a_97b8,
    f: ee::s03::f_001a97b8,
};
/// `NetFEState_HandleNetEvent` (0x1a8e40).
const STEP1_NET_HANDLE_EVENT: Callee = Callee {
    addr: 0x001a_8e40,
    f: ee::s03::f_001a8e40,
};
/// `NetFEState_GetPopup` (0x1a9c50).
const STEP1_NET_GET_POPUP: Callee = Callee {
    addr: 0x001a_9c50,
    f: ee::s03::f_001a9c50,
};
/// `NetFEState_ShowPopup` (0x1a9bc0).
const STEP1_NET_SHOW_POPUP: Callee = Callee {
    addr: 0x001a_9bc0,
    f: ee::s03::f_001a9bc0,
};
/// `NetFEState_GotoNextState` (0x1a8a40).
const STEP1_NET_GOTO_NEXT_STATE: Callee = Callee {
    addr: 0x001a_8a40,
    f: ee::s03::f_001a8a40,
};
/// `cFEStateSelectPersona_ctor` (0x1ad130).
const STEP1_SELECT_PERSONA_CTOR: Callee = Callee {
    addr: 0x001a_d130,
    f: ee::s03::f_001ad130,
};
/// `cFEStateNetAccountMem_ctor` (0x1b6ed0).
const STEP1_ACCOUNT_MEM_CTOR: Callee = Callee {
    addr: 0x001b_6ed0,
    f: ee::s03::f_001b6ed0,
};
/// `cFEPopupConfirm_CfgSetOptionTextId` (0x1c57c0).
const STEP1_CONFIRM_SET_OPTION_TEXT: Callee = Callee {
    addr: 0x001c_57c0,
    f: ee::s03::f_001c57c0,
};

/// `StrHash(s)` (0x317670, `GetHashValue32`) of the string at `s`, returning
/// to `ret`.
fn step1_str_hash(c: &mut Cpu, s: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(s);
    call_ptr(c, 0x0031_7670, ret)
}

/// `cFEStateCreateStep1_Update` (0x1b89e8): forwards to `NetFEState_Update`.
pub fn cfe_state_create_step1_update(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(STEP1_NET_UPDATE), 0x001b_89f8);
}

/// `cFEStateCreateStep1_OnWidgetCreated(this, widget)` (0x1b8aa0): fills the
/// account form's help texts from the widget's state at +0x38, comparing the
/// hashed names of the current state against the widget's name hash.
pub fn cfe_state_create_step1_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        widget_created_body(c, frame)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The part of `OnWidgetCreated` between the prologue and the epilogue. Every
/// normal exit restores s0 first, as the generated `lq 16` does.
fn widget_created_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
    step1_str_hash(c, 0x0046_1520, 0x001b_8acc)?;
    if c.g[S0][0] != c.g[V0][0] {
        c.g[A0][0] = 0x0046_0000;
        return widget_created_second(c, frame);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6ec)).lw(c);
    let nonzero = c.g[V0][0] != 0;
    frame.restore(c, S0, 0x30);
    if nonzero {
        return Ok(());
    }
    c.cycles += 8;
    c.g[A0][0] = 0x0046_0000;
    step1_str_hash(c, 0x0046_4750, 0x001b_8aec)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x003a_1000, 0x001b_8af8)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// The second group of hash checks (the section after the first `if`).
fn widget_created_second(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
    step1_str_hash(c, 0x0046_4768, 0x001b_8b0c)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6ec)).lw(c);
    } else {
        c.cycles += 6;
        c.g[A0][0] = 0x0046_0000;
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
        step1_str_hash(c, 0x0046_2b88, 0x001b_8b24)?;
        if c.g[S0][0] != c.g[V0][0] {
            c.g[A0][0] = 0x0046_0000;
            return widget_created_third(c, frame);
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6ec)).lw(c);
    }
    c.cycles += 2;
    let nonzero = c.g[V0][0] != 0;
    frame.restore(c, S0, 0x30);
    if nonzero {
        return Ok(());
    }
    c.cycles += 14;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc8)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xcc)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001b_8b50)?;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
    c.g[A1][0] = 1;
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x40)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x44)).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001b_8b68)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// The third group: the `0x4780`/`0x4790` checks and the `FillYesNoList` call.
fn widget_created_third(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
    step1_str_hash(c, 0x0046_4780, 0x001b_8b7c)?;
    let equal_first = c.g[S0][0] == c.g[V0][0];
    c.g[A0][0] = 0x0046_0000;
    if !equal_first {
        c.cycles += 5;
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
        step1_str_hash(c, 0x0046_4790, 0x001b_8b90)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        if differs {
            return widget_created_fourth(c, frame);
        }
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = 1;
    STEP1_NET_FILL_YES_NO.call(c, 0x001b_8ba8)?;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// The fourth group: the `0x47a8` check, then the popup-config tail.
fn widget_created_fourth(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
    step1_str_hash(c, 0x0046_47a8, 0x001b_8bbc)?;
    let differs = c.g[S0][0] != c.g[V0][0];
    frame.restore(c, S0, 0x30);
    if differs {
        return Ok(());
    }
    widget_created_tail(c, frame)
}

/// The last section: a callback through the widget's vtable (or `0x3a1000`
/// with the stored slot), then the widget's state word at +0x6f0.
fn widget_created_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6ec)).lw(c);
    let zero = c.g[V0][0] == 0;
    c.g[A1][0] = 1;
    if !zero {
        c.cycles += 13;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc8)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xcc)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001b_8be4)?;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x40)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x44)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001b_8bfc)?;
        let state = c.g[S1][0];
        Addr(r32(c, S2).wrapping_add(0x6f0)).sw(c, state);
    } else {
        c.cycles += 7;
        c.g[A0][0] = 0x0046_0000;
        step1_str_hash(c, 0x0046_31d8, 0x001b_8c10)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003a_1000, 0x001b_8c1c)?;
        let state = c.g[S1][0];
        Addr(r32(c, S2).wrapping_add(0x6f0)).sw(c, state);
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x30);
    Ok(())
}

/// `cFEStateCreateStep1_v25` (0x1b9088): the help texts of the account form.
/// Sets the help-option count of `this`, stores one string hash at +0x54,
/// then adds the option entries (hashed names with their ids) at +0x64.
pub fn cfe_state_create_step1_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        // cFEStateNetHelpPopup_setNumHelpOptions(menu, 5)
        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001b_90a4)?;
        step1_str_hash(c, 0x0046_48a8, 0x001b_90b0)?;
        let config_at = Addr(r32(c, S0).wrapping_add(0x54));
        let v0 = c.g[V0][0];
        config_at.sw(c, v0);
        step1_str_hash(c, 0x0046_48c8, 0x001b_90c0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001b_90d0)?;
        step1_str_hash(c, 0x0046_48e0, 0x001b_90dc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x001d_a530, 0x001b_90ec)?;
        step1_str_hash(c, 0x0046_48f8, 0x001b_90f8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x001d_a530, 0x001b_9108)?;
        step1_str_hash(c, 0x0045_fe60, 0x001b_9114)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 3;
        call_ptr(c, 0x001d_a510, 0x001b_9128)?;
        step1_str_hash(c, 0x0045_fe40, 0x001b_9134)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A3][0] = 4;
        call_ptr(c, 0x001d_a510, 0x001b_9148)?;
        step1_str_hash(c, 0x0046_2c50, 0x001b_9154)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[A3][0] = 0;
        call_ptr(c, 0x001d_a510, 0x001b_9168)?;
        step1_str_hash(c, 0x0046_14e8, 0x001b_9174)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        c.g[A3][0] = 1;
        call_ptr(c, 0x001d_a510, 0x001b_9188)?;
        step1_str_hash(c, 0x0046_4910, 0x001b_9194)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        c.g[A3][0] = 2;
        call_ptr(c, 0x001d_a510, 0x001b_91a8)?;
        step1_str_hash(c, 0x0046_4930, 0x001b_91b4)?;
        let v0 = c.g[V0][0];
        Addr(r32(c, S0).wrapping_add(0x64)).sw(c, v0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The answer branch `cFEStateCreateStep1_OnSaveAccountAnswer` takes.
#[derive(Clone, Copy)]
enum Step1Answer {
    /// Answers 0x10 and 0x14: the 13-cycle popup/state path.
    Popup,
    /// Answer 0xf: the account-memory path.
    Account,
    /// Every other answer: forward the child event and return.
    Child,
}

/// `cFEStateCreateStep1_OnSaveAccountAnswer(this, answer)` (0x1b91c8): the
/// server's answer to the account save. Dispatches on the answer code in a2.
pub fn cfe_state_create_step1_on_save_account_answer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x10;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let answer = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        let branch = if answer == c.g[V0][0] {
            c.cycles += 1;
            c.g[A1][0] = 0x0046_0000;
            Step1Answer::Popup
        } else {
            c.cycles += 3;
            c.g[V0][0] = (answer < 0x11u64) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = 0x14;
                if answer == c.g[V0][0] {
                    c.g[A1][0] = 0x0046_0000;
                    Step1Answer::Popup
                } else {
                    c.cycles += 2;
                    Step1Answer::Child
                }
            } else {
                c.g[V0][0] = 0xf;
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if answer == c.g[V0][0] {
                    Step1Answer::Account
                } else {
                    c.cycles += 2;
                    Step1Answer::Child
                }
            }
        };
        match branch {
            Step1Answer::Popup => answer_popup(c)?,
            Step1Answer::Account => answer_account(c)?,
            Step1Answer::Child => answer_child(c)?,
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The popup path of `OnSaveAccountAnswer` (answers 0x10 and 0x14).
fn answer_popup(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[A0][0] = 0x0000_06d8;
    c.g[A1][0] = sx(0x0046_2d68);
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001b_9290)?;
    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    STEP1_SELECT_PERSONA_CTOR.call(c, 0x001b_929c)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    STEP1_NET_GOTO_NEXT_STATE.call(c, 0x001b_92a8)?;
    Ok(())
}

/// The account path of `OnSaveAccountAnswer` (answer 0xf): forwards the child
/// event, allocates the account memory and runs its constructor.
fn answer_account(c: &mut Cpu) -> Flow {
    c.cycles += 26;
    c.g[A2][0] = 0xf;
    STEP1_NET_ON_CHILD.call(c, 0x001b_9218)?;
    c.g[A1][0] = sx(0x0046_2d10);
    c.g[A0][0] = 0xb10;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001b_9230)?;
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x6d0)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x10)).lw(c);
    c.g[A2][0] = c.g[S0][0];
    c.g[T1][0] = Addr(r32(c, V1).wrapping_add(0x4)).lw(c);
    c.g[A3][0] = 1;
    c.g[T0][0] = Addr(r32(c, V1)).lw(c);
    STEP1_ACCOUNT_MEM_CTOR.call(c, 0x001b_9250)?;
    c.g[V1][0] = 0x82c;
    c.g[A1][0] = c.g[V0][0];
    let size = c.g[V1][0];
    Addr(r32(c, V0).wrapping_add(0x18)).sw(c, size);
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x40)).lw(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xd0)).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f290, 0x001b_9270)?;
    Ok(())
}

/// The child path of `OnSaveAccountAnswer` (every other answer).
fn answer_child(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[A0][0] = c.g[S0][0];
    STEP1_NET_ON_CHILD.call(c, 0x001b_92b8)?;
    Ok(())
}

/// `cFEStateCreateStep1_v31` (0x1b9488): the net event handler of the account
/// form. Event 0xce shows the confirm popup; other events go to
/// `NetFEState_HandleNetEvent` and return its result path.
pub fn cfe_state_create_step1_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xce;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        let is_popup_event = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if is_popup_event {
            v31_popup(c, frame)?;
        } else {
            c.cycles += 4;
            STEP1_NET_HANDLE_EVENT.call(c, 0x001b_94b0)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The popup path of `cFEStateCreateStep1_v31` (event 0xce).
fn v31_popup(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 35;
    c.g[A0][0] = c.g[S2][0];
    STEP1_NET_GET_POPUP.call(c, 0x001b_94c0)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = 2;
    let option_count_at = Addr(r32(c, S0).wrapping_add(0xbc));
    option_count_at.sw(c, 2);
    c.g[S1][0] = sx(r32(c, S0).wrapping_add(0xbc));
    step1_str_hash(c, 0x0046_2d80, 0x001b_94dc)?;
    c.g[V1][0] = 3;
    Addr(r32(c, S0).wrapping_add(0xc0)).sw(c, 3);
    let hash = c.g[V0][0];
    Addr(r32(c, S0).wrapping_add(0xc4)).sw(c, hash);
    step1_str_hash(c, 0x0045_dcc8, 0x001b_94f4)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[S1][0];
    STEP1_CONFIRM_SET_OPTION_TEXT.call(c, 0x001b_9504)?;
    step1_str_hash(c, 0x0045_fcd8, 0x001b_9510)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 1;
    STEP1_CONFIRM_SET_OPTION_TEXT.call(c, 0x001b_9520)?;
    c.g[V0][0] = 0x81e;
    Addr(r32(c, S0).wrapping_add(0x168)).sw(c, 0);
    Addr(r32(c, S0).wrapping_add(0x164)).sw(c, 0);
    c.g[A0][0] = c.g[S2][0];
    let text = c.g[V0][0];
    Addr(r32(c, S0).wrapping_add(0x150)).sw(c, text);
    c.g[A1][0] = c.g[S0][0];
    STEP1_NET_SHOW_POPUP.call(c, 0x001b_953c)?;
    c.g[V0][0] = u64::MAX;
    frame.restore(c, S0, 0x30);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, DATA};

    /// Writes `text` as a NUL-terminated string at `addr` and returns the hash
    /// the guest `StrHash` computes for it.
    fn oc888_name(c: &mut Cpu, addr: u32, text: &[u8]) -> u32 {
        for (i, &b) in text.iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + text.len() as u32, 0);
        crate::game::hashvalue::str_hash(text)
    }

    #[test]
    fn cfe_state_create_step1_update_matches() {
        check(
            ee::s03::f_001b89e8__generated,
            cfe_state_create_step1_update,
            40,
            &|c, seed| {
                machine(c, seed, 0x1b89_e8, 0);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_on_widget_created_matches() {
        check(
            ee::s03::f_001b8aa0__generated,
            cfe_state_create_step1_on_widget_created,
            64,
            &|c, seed| {
                machine(c, seed, 0x1b8a_a0, 0);
                const THIS: u32 = DATA + 0x2000;
                const WIDGET: u32 = DATA + 0x1000;
                const VT: u32 = DATA + 0x3000;
                let h = [
                    oc888_name(c, 0x0046_1520, b"alpha"),
                    oc888_name(c, 0x0046_4768, b"bravo"),
                    oc888_name(c, 0x0046_2b88, b"charlie"),
                    oc888_name(c, 0x0046_4780, b"delta"),
                    oc888_name(c, 0x0046_4790, b"echo"),
                    oc888_name(c, 0x0046_47a8, b"foxtrot"),
                ];
                // Which compare succeeds: the first name, each of the next
                // ones, none (fall through to the 0x3a1000 tail), the last.
                let state = match seed % 8 {
                    6 => 0xdead_beef,
                    7 => h[5],
                    k => h[k as usize],
                };
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(WIDGET);
                c.bus.write32(WIDGET + 0x38, state);
                c.bus.write32(THIS + 0x6ec, (seed / 8) % 2);
                c.bus.write32(WIDGET + 0x8, VT);
                c.bus.write32(VT + 0xcc, FAKE);
                c.bus.write32(VT + 0x44, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_v25_matches() {
        check(
            ee::s03::f_001b9088__generated,
            cfe_state_create_step1_v25,
            60,
            &|c, seed| {
                machine(c, seed, 0x1b90_88, 0);
                c.g[A1][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_on_save_account_answer_matches() {
        check(
            ee::s03::f_001b91c8__generated,
            cfe_state_create_step1_on_save_account_answer,
            60,
            &|c, seed| {
                machine(c, seed, 0x1b91_c8, 0);
                // Allocation and the account memory constructor need the pool
                // allocator; set it up before the argument registers.
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                // Answers 0x10 and 0x14 (popup), 0xf (account), 0x11 and 0x3
                // (child).
                let answers = [0x10u64, 0x14, 0xf, 0x11, 0x3, 0x12];
                c.g[A0][0] = sx(DATA + 0x8000);
                c.g[A2][0] = answers[seed as usize % answers.len()];
                c.bus.write32(DATA + 0x8000 + 0x6d0, DATA + 0x9000);
                c.bus.write32(DATA + 0x8000 + 0x40, DATA + 0xa000);
                c.bus.write32(DATA + 0xa000 + 0xd0, DATA + 0xb000);
                c.bus.write32(DATA + 0x8000 + 0x10, DATA + 0xc000);
            },
        );
    }

    #[test]
    fn cfe_state_create_step1_v31_matches() {
        check(
            ee::s03::f_001b9488__generated,
            cfe_state_create_step1_v31,
            60,
            &|c, seed| {
                machine(c, seed, 0x1b94_88, 0);
                c.g[A0][0] = sx(DATA);
                // Event 0xce takes the popup path, others the net handler.
                c.g[A1][0] = if seed % 3 == 0 { 0x3 } else { 0xce };
            },
        );
    }
}
