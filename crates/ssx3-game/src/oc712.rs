//! Staging module oc712 (wave wave240): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc712`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0039_b000, ui_menu_on_input, "cUIMenu_onInput")];

/// cUIThing `mAttr` (word).
const ONIN_ATTR: u32 = 0x14;
/// cUIMenu `mList` (the cList the items live in).
const ONIN_LIST: u32 = 0x74;
/// cUIMenu `mpScreen`.
const ONIN_SCREEN: u32 = 0x5c;
/// cUIMenu `mAttrMenu` (word): bit 3 is mDisableDirSound.
const ONIN_ATTR_MENU: u32 = 0x90;
/// cUIMenu `mInputMap[4]` (signed bytes at 0x91..0x94).
const ONIN_INPUT_MAP: u32 = 0x91;
/// cUIMenu `mScrollItemCurrent` (the selected row).
const ONIN_SCROLL_CUR: u32 = 0x95;
/// cUIMenu `mScrollItemOffset`.
const ONIN_SCROLL_OFFSET: u32 = 0x98;
/// cUIMenu `mpSelected` (the focused item).
const ONIN_SELECTED: u32 = 0xa0;

/// Keyframer-event lookup (`cUIThing_getKeyframerEvent`, 0x39ff78).
const ONIN_GET_EVENT_THREAD: Callee = Callee {
    addr: 0x0039_ff78,
    f: ee::s13::f_0039ff78,
};
/// `UIList_GetAt(list, index)` (0x397870).
const ONIN_LIST_GET_AT: Callee = Callee {
    addr: 0x0039_7870,
    f: ee::s13::f_00397870,
};
/// `UIMenu_FindNextSelectable` (0x39ab50).
const ONIN_FIND_NEXT: Callee = Callee {
    addr: 0x0039_ab50,
    f: ee::s13::f_0039ab50,
};
/// `UIMenu_FindPrevSelectable` (0x39ac48).
const ONIN_FIND_PREV: Callee = Callee {
    addr: 0x0039_ac48,
    f: ee::s13::f_0039ac48,
};
/// `UIMenu_SelectIndex(menu, index)` (0x39ae98).
const ONIN_SELECT_INDEX: Callee = Callee {
    addr: 0x0039_ae98,
    f: ee::s13::f_0039ae98,
};
/// `cUIScreen_playFrame(screen, event, 1)` (0x39c870).
const ONIN_PLAY_FRAME: Callee = Callee {
    addr: 0x0039_c870,
    f: ee::s13::f_0039c870,
};
/// `cUIThing_getState` (0x39ff50): the owner's state data.
const ONIN_GET_STATE_DATA: Callee = Callee {
    addr: 0x0039_ff50,
    f: ee::s13::f_0039ff50,
};
/// `UIObject_GetUserData` (0x3a0318).
const ONIN_GET_USER_DATA: Callee = Callee {
    addr: 0x003a_0318,
    f: ee::s13::f_003a0318,
};

/// `base + off` as a 32-bit guest address.
#[inline]
fn onin_at(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// A gcc virtual call: `A0 = lh off(vt)`, `func = lw off+4(vt)`, `A0 = obj +
/// A0`, `jalr func` returning to `ret`. `vt`, `obj` and `func` name registers.
fn onin_vcall(c: &mut Cpu, vt: usize, obj: usize, off: u32, func: usize, ret: u32) -> Flow {
    c.g[A0][0] = c.lh(onin_at(c.g[vt][0], off));
    c.g[func][0] = c.lw(onin_at(c.g[vt][0], off + 4));
    let target = c.g[func][0] as u32;
    c.g[A0][0] = sx(onin_at(c.g[obj][0], c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `int cUIMenu::onInput(cUIInput* in)` (0x39b000): the menu takes an input.
/// A hidden menu, one with no selected item, or one whose mAttr bit 4 is
/// clear ignores it (returns 0). The selected item gets the input first (its
/// `onInput` returning non-zero ends it with 1). Otherwise the direction
/// buttons (mInputMap; left/right for horizontal menus, mAttr bit 0) move the
/// selection, and the inputs 4-8 are reported to the item's widget event and
/// to the owner's focus handlers. Their results pick the item's sound; the
/// UI sound listener plays it. Returns 1 when the menu used the input.
pub fn ui_menu_on_input(c: &mut Cpu) {
    guest(c, ui_menu_on_input_body)
}

fn ui_menu_on_input_body(c: &mut Cpu) -> Flow {
    ssx3_ee_rt::prof::enter(c, 0x0039_b000);
    c.cycles += 12;
    let frame = Frame::open(c, 0x50);
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S2, 0x20);
    frame.save(c, S3, 0x10);
    frame.save_ra(c, 0);

    'b_39b680: {
        'b_39b64c: {
            'b_39b618: {
                'b_39b610: {
                    c.g[V1][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.g[V0][0] &= 1;
                    let hidden = c.g[V0][0] != 0;
                    c.g[S1][0] = c.g[A1][0];
                    if hidden {
                        break 'b_39b610;
                    }
                    c.cycles += 3;
                    c.g[A2][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                    let no_selection = c.g[A2][0] == 0;
                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 4) as u32);
                    if no_selection {
                        break 'b_39b610;
                    }
                    c.cycles += 3;
                    c.g[V0][0] &= 1;
                    let inactive = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if inactive {
                        break 'b_39b680;
                    }
                    // The selected item gets the input first.
                    'b_39b08c: {
                        'b_39b088: {
                            c.cycles += 5;
                            c.g[V1][0] = c.lw(onin_at(c.g[A2][0], ONIN_ATTR));
                            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 3) as u32);
                            c.g[V0][0] &= 1;
                            let not_enabled = c.g[V0][0] == 0;
                            c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                            if not_enabled {
                                break 'b_39b088;
                            }
                            c.cycles += 3;
                            c.g[V0][0] &= 1;
                            if c.g[V0][0] != 0 {
                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                break 'b_39b08c;
                            }
                            c.cycles += 8;
                            c.g[V1][0] = c.lw(onin_at(c.g[A2][0], 0x8));
                            c.g[A1][0] = c.g[S1][0];
                            onin_vcall(c, V1, A2, 0x78, V0, 0x0039_b080)?;
                            let used = c.g[V0][0] != 0;
                            c.g[V0][0] = 1;
                            if used {
                                break 'b_39b680;
                            }
                            break 'b_39b088;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                    }
                    // Input 3 / 1 / 2 / 0 are the direction buttons; the
                    // scroll index and the owner decide what they move.
                    'b_39b5f8: {
                        'b_39b5e8: {
                            'b_39b5e4: {
                                'b_39b18c: {
                                    'b_39b0f0: {
                                        'b_39b0c4: {
                                            c.cycles += 9;
                                            c.g[S3][0] = 0;
                                            c.g[A1][0] =
                                                c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 3));
                                            c.g[S2][0] =
                                                c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                            onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b0a8)?;
                                            let t = c.g[V0][0] == 0;
                                            c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                            if t {
                                                break 'b_39b0c4;
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                            c.g[V0][0] ^= 1;
                                            c.g[V0][0] &= 1;
                                            if c.g[V0][0] != 0 {
                                                break 'b_39b0f0;
                                            }
                                            break 'b_39b0c4;
                                        }
                                        c.cycles += 7;
                                        c.g[A1][0] = c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 1));
                                        onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b0d8)?;
                                        let t = c.g[V0][0] == 0;
                                        c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                        if t {
                                            break 'b_39b18c;
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                        c.g[V0][0] &= 1;
                                        if c.g[V0][0] == 0 {
                                            break 'b_39b18c;
                                        }
                                        break 'b_39b0f0;
                                    }
                                    'b_39b120: {
                                        c.cycles += 7;
                                        c.g[A1][0] = c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 3));
                                        onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b104)?;
                                        let t = c.g[V0][0] == 0;
                                        c.g[A1][0] = 0x7;
                                        if t {
                                            break 'b_39b120;
                                        }
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                        c.g[V0][0] ^= 1;
                                        c.g[V0][0] &= 1;
                                        if c.g[V0][0] != 0 {
                                            c.g[A1][0] = 0x9;
                                        }
                                        break 'b_39b120;
                                    }
                                    'b_39b14c: {
                                        c.cycles += 6;
                                        c.g[A0][0] = c.g[S0][0];
                                        ONIN_GET_EVENT_THREAD.call(c, 0x0039_b128)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[V0][0] = 0xffff;
                                        if c.g[A1][0] == c.g[V0][0] {
                                            break 'b_39b14c;
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                        if c.g[A0][0] == 0 {
                                            break 'b_39b14c;
                                        }
                                        c.cycles += 2;
                                        c.g[A2][0] = 0x1;
                                        ONIN_PLAY_FRAME.call(c, 0x0039_b14c)?;
                                        break 'b_39b14c;
                                    }
                                    // Button 3 (down, or right when horizontal) moves
                                    // the selection forward.
                                    c.cycles += 12;
                                    c.g[A0][0] = c.g[S0][0];
                                    ONIN_FIND_NEXT.call(c, 0x0039_b154)?;
                                    c.sb(onin_at(c.g[S0][0], ONIN_SCROLL_CUR), c.g[V0][0]);
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    ONIN_SELECT_INDEX.call(c, 0x0039_b164)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR_MENU));
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                                    c.g[V0][0] &= 1;
                                    if c.g[V0][0] != 0 {
                                        break 'b_39b64c;
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != 0;
                                    c.g[S3][0] = 1;
                                    if t {
                                        break 'b_39b64c;
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                    break 'b_39b5e4;
                                }
                                'b_39b5e0: {
                                    'b_39b284: {
                                        'b_39b1e8: {
                                            'b_39b1bc: {
                                                c.cycles += 7;
                                                c.g[A1][0] =
                                                    c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 2));
                                                onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b1a0)?;
                                                let t = c.g[V0][0] == 0;
                                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                                if t {
                                                    break 'b_39b1bc;
                                                }
                                                c.cycles += 5;
                                                c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                                c.g[V0][0] ^= 1;
                                                c.g[V0][0] &= 1;
                                                if c.g[V0][0] != 0 {
                                                    break 'b_39b1e8;
                                                }
                                                break 'b_39b1bc;
                                            }
                                            c.cycles += 7;
                                            c.g[A1][0] = c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP));
                                            onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b1d0)?;
                                            let t = c.g[V0][0] == 0;
                                            c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                            if t {
                                                break 'b_39b284;
                                            }
                                            c.cycles += 4;
                                            c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                            c.g[V0][0] &= 1;
                                            if c.g[V0][0] == 0 {
                                                break 'b_39b284;
                                            }
                                            break 'b_39b1e8;
                                        }
                                        'b_39b218: {
                                            c.cycles += 7;
                                            c.g[A1][0] =
                                                c.lb(onin_at(c.g[S0][0], ONIN_INPUT_MAP + 2));
                                            onin_vcall(c, V1, S1, 0x10, V0, 0x0039_b1fc)?;
                                            let t = c.g[V0][0] == 0;
                                            c.g[A1][0] = 0x6;
                                            if t {
                                                break 'b_39b218;
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR));
                                            c.g[V0][0] ^= 1;
                                            c.g[V0][0] &= 1;
                                            if c.g[V0][0] != 0 {
                                                c.g[A1][0] = 0x8;
                                            }
                                            break 'b_39b218;
                                        }
                                        'b_39b244: {
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S0][0];
                                            ONIN_GET_EVENT_THREAD.call(c, 0x0039_b220)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[V0][0] = 0xffff;
                                            if c.g[A1][0] == c.g[V0][0] {
                                                break 'b_39b244;
                                            }
                                            c.cycles += 3;
                                            c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                            if c.g[A0][0] == 0 {
                                                break 'b_39b244;
                                            }
                                            c.cycles += 2;
                                            c.g[A2][0] = 0x1;
                                            ONIN_PLAY_FRAME.call(c, 0x0039_b244)?;
                                            break 'b_39b244;
                                        }
                                        // Button 2 (up, or left when horizontal) moves
                                        // the selection back.
                                        c.cycles += 12;
                                        c.g[A0][0] = c.g[S0][0];
                                        ONIN_FIND_PREV.call(c, 0x0039_b24c)?;
                                        c.sb(onin_at(c.g[S0][0], ONIN_SCROLL_CUR), c.g[V0][0]);
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        ONIN_SELECT_INDEX.call(c, 0x0039_b25c)?;
                                        c.g[V1][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_ATTR_MENU));
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                                        c.g[V0][0] &= 1;
                                        if c.g[V0][0] != 0 {
                                            break 'b_39b64c;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0;
                                        c.g[S3][0] = 1;
                                        if t {
                                            break 'b_39b5e0;
                                        }
                                        c.cycles += 2;
                                        break 'b_39b64c;
                                    }
                                    // Inputs 5 and 6 (focus move) and 7, 8 and 4
                                    // (the item's focus events): the owner's focus
                                    // handlers decide the sound.
                                    'b_39b5d4: {
                                        'b_39b358: {
                                            c.cycles += 6;
                                            onin_vcall(c, V1, S1, 0x28, V0, 0x0039_b294)?;
                                            if c.g[V0][0] == 0 {
                                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                                break 'b_39b358;
                                            }
                                            'b_39b348: {
                                                'b_39b344: {
                                                    c.cycles += 31;
                                                    c.g[V0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
                                                    c.g[A1][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_sub(c.g[A1][0] as u32));
                                                    ONIN_LIST_GET_AT.call(c, 0x0039_b2b0)?;
                                                    c.g[V1][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[S2][0] = c.g[V0][0];
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A2][0] = 0x5;
                                                    c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
                                                    c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
                                                    onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b2d8)?;
                                                    c.g[V0][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A2][0] = 0x6;
                                                    c.g[T0][0] = c.lw(onin_at(c.g[V0][0], 0xd0));
                                                    c.g[A3][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[V1][0] = c.lw(onin_at(c.g[T0][0], 0x8));
                                                    c.g[A3][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_sub(c.g[A3][0] as u32));
                                                    onin_vcall(c, V1, T0, 0xa0, V0, 0x0039_b308)?;
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x100;
                                                    let t = c.g[V0][0] == 0;
                                                    c.g[A1][0] = 0x2;
                                                    if t {
                                                        break 'b_39b344;
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    ONIN_GET_EVENT_THREAD.call(c, 0x0039_b320)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[V0][0] = 0xffff;
                                                    let t = c.g[A1][0] == c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x1;
                                                    if t {
                                                        break 'b_39b348;
                                                    }
                                                    c.cycles += 3;
                                                    c.g[A0][0] =
                                                        c.lw(onin_at(c.g[S2][0], ONIN_SCREEN));
                                                    if c.g[A0][0] == 0 {
                                                        break 'b_39b348;
                                                    }
                                                    c.cycles += 2;
                                                    c.g[A2][0] = 0x1;
                                                    ONIN_PLAY_FRAME.call(c, 0x0039_b344)?;
                                                    break 'b_39b344;
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                break 'b_39b348;
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_39b5d4;
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                            break 'b_39b5e8;
                                        }
                                        'b_39b42c: {
                                            c.cycles += 6;
                                            onin_vcall(c, V1, S1, 0x30, V0, 0x0039_b368)?;
                                            if c.g[V0][0] == 0 {
                                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                                break 'b_39b42c;
                                            }
                                            'b_39b41c: {
                                                'b_39b418: {
                                                    c.cycles += 31;
                                                    c.g[V0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
                                                    c.g[A1][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_sub(c.g[A1][0] as u32));
                                                    ONIN_LIST_GET_AT.call(c, 0x0039_b384)?;
                                                    c.g[V1][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[S2][0] = c.g[V0][0];
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A2][0] = 0x6;
                                                    c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
                                                    c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
                                                    onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b3ac)?;
                                                    c.g[V0][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A2][0] = 0x7;
                                                    c.g[T0][0] = c.lw(onin_at(c.g[V0][0], 0xd0));
                                                    c.g[A3][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[V1][0] = c.lw(onin_at(c.g[T0][0], 0x8));
                                                    c.g[A3][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_sub(c.g[A3][0] as u32));
                                                    onin_vcall(c, V1, T0, 0xa0, V0, 0x0039_b3dc)?;
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x100;
                                                    let t = c.g[V0][0] == 0;
                                                    c.g[A1][0] = 0x3;
                                                    if t {
                                                        break 'b_39b418;
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    ONIN_GET_EVENT_THREAD.call(c, 0x0039_b3f4)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[V0][0] = 0xffff;
                                                    let t = c.g[A1][0] == c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x1;
                                                    if t {
                                                        break 'b_39b41c;
                                                    }
                                                    c.cycles += 3;
                                                    c.g[A0][0] =
                                                        c.lw(onin_at(c.g[S2][0], ONIN_SCREEN));
                                                    if c.g[A0][0] == 0 {
                                                        break 'b_39b41c;
                                                    }
                                                    c.cycles += 2;
                                                    c.g[A2][0] = 0x1;
                                                    ONIN_PLAY_FRAME.call(c, 0x0039_b418)?;
                                                    break 'b_39b418;
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                break 'b_39b41c;
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_39b5d4;
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                            break 'b_39b5e8;
                                        }
                                        'b_39b500: {
                                            c.cycles += 6;
                                            onin_vcall(c, V1, S1, 0x40, V0, 0x0039_b43c)?;
                                            if c.g[V0][0] == 0 {
                                                c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                                break 'b_39b500;
                                            }
                                            'b_39b4f0: {
                                                'b_39b4ec: {
                                                    c.cycles += 31;
                                                    c.g[V0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
                                                    c.g[A1][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_sub(c.g[A1][0] as u32));
                                                    ONIN_LIST_GET_AT.call(c, 0x0039_b458)?;
                                                    c.g[V1][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[S2][0] = c.g[V0][0];
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A2][0] = 0x8;
                                                    c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
                                                    c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
                                                    onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b480)?;
                                                    c.g[V0][0] =
                                                        c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A0][0] =
                                                        c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                    c.g[A2][0] = 0x9;
                                                    c.g[T0][0] = c.lw(onin_at(c.g[V0][0], 0xd0));
                                                    c.g[A3][0] = c.lbu(onin_at(
                                                        c.g[S0][0],
                                                        ONIN_SCROLL_OFFSET,
                                                    ));
                                                    c.g[V1][0] = c.lw(onin_at(c.g[T0][0], 0x8));
                                                    c.g[A3][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_sub(c.g[A3][0] as u32));
                                                    onin_vcall(c, V1, T0, 0xa0, V0, 0x0039_b4b0)?;
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x100;
                                                    let t = c.g[V0][0] == 0;
                                                    c.g[A1][0] = 0x5;
                                                    if t {
                                                        break 'b_39b4ec;
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    ONIN_GET_EVENT_THREAD.call(c, 0x0039_b4c8)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[V0][0] = 0xffff;
                                                    let t = c.g[A1][0] == c.g[V0][0];
                                                    c.g[V0][0] = c.g[S1][0] & 0x1;
                                                    if t {
                                                        break 'b_39b4f0;
                                                    }
                                                    c.cycles += 3;
                                                    c.g[A0][0] =
                                                        c.lw(onin_at(c.g[S2][0], ONIN_SCREEN));
                                                    if c.g[A0][0] == 0 {
                                                        break 'b_39b4f0;
                                                    }
                                                    c.cycles += 2;
                                                    c.g[A2][0] = 0x1;
                                                    ONIN_PLAY_FRAME.call(c, 0x0039_b4ec)?;
                                                    break 'b_39b4ec;
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                break 'b_39b4f0;
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_39b5d4;
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                            break 'b_39b5e8;
                                        }
                                        c.cycles += 6;
                                        onin_vcall(c, V1, S1, 0x38, V0, 0x0039_b510)?;
                                        if c.g[V0][0] == 0 {
                                            c.g[V1][0] = c.lw(onin_at(c.g[S1][0], 0x8));
                                            break 'b_39b5f8;
                                        }
                                        'b_39b5c4: {
                                            'b_39b5c0: {
                                                c.cycles += 31;
                                                c.g[V0][0] =
                                                    c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
                                                c.g[A1][0] =
                                                    c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_OFFSET));
                                                c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_sub(c.g[A1][0] as u32));
                                                ONIN_LIST_GET_AT.call(c, 0x0039_b52c)?;
                                                c.g[V1][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                c.g[S2][0] = c.g[V0][0];
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A2][0] = 0x7;
                                                c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
                                                c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
                                                onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b554)?;
                                                c.g[V0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
                                                c.g[A1][0] = c.g[S0][0];
                                                c.g[A0][0] =
                                                    c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                                                c.g[A2][0] = 0x8;
                                                c.g[T0][0] = c.lw(onin_at(c.g[V0][0], 0xd0));
                                                c.g[A3][0] =
                                                    c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_OFFSET));
                                                c.g[V1][0] = c.lw(onin_at(c.g[T0][0], 0x8));
                                                c.g[A3][0] = sx((c.g[A0][0] as u32)
                                                    .wrapping_sub(c.g[A3][0] as u32));
                                                onin_vcall(c, V1, T0, 0xa0, V0, 0x0039_b584)?;
                                                c.g[S1][0] = c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x100;
                                                let t = c.g[V0][0] == 0;
                                                c.g[A1][0] = 0x4;
                                                if t {
                                                    break 'b_39b5c0;
                                                }
                                                c.cycles += 6;
                                                c.g[A0][0] = c.g[S2][0];
                                                ONIN_GET_EVENT_THREAD.call(c, 0x0039_b59c)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] = 0xffff;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = c.g[S1][0] & 0x1;
                                                if t {
                                                    break 'b_39b5c4;
                                                }
                                                c.cycles += 3;
                                                c.g[A0][0] = c.lw(onin_at(c.g[S2][0], ONIN_SCREEN));
                                                if c.g[A0][0] == 0 {
                                                    break 'b_39b5c4;
                                                }
                                                c.cycles += 2;
                                                c.g[A2][0] = 0x1;
                                                ONIN_PLAY_FRAME.call(c, 0x0039_b5c0)?;
                                                break 'b_39b5c0;
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] = c.g[S1][0] & 0x1;
                                            break 'b_39b5c4;
                                        }
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0;
                                        c.g[A1][0] = 0x1;
                                        if t {
                                            break 'b_39b5d4;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                        break 'b_39b5e8;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.g[S1][0] & 0x10;
                                    let t = c.g[V0][0] == 0;
                                    if t {
                                        break 'b_39b64c;
                                    }
                                    break 'b_39b5e0;
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.lw(onin_at(c.g[S0][0], ONIN_SELECTED));
                                break 'b_39b5e4;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0;
                            break 'b_39b5e8;
                        }
                        // Input 9 (the item's user data, becomes the state's item).
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0039_b5f0);
                        ONIN_GET_USER_DATA.call(c, 0x0039_b5f0)?;
                        c.g[S3][0] = c.g[V0][0];
                        break 'b_39b64c;
                    }
                    c.cycles += 6;
                    onin_vcall(c, V1, S1, 0x20, V0, 0x0039_b608)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_CUR));
                        break 'b_39b618;
                    }
                    break 'b_39b610;
                }
                // Nothing used the input.
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'b_39b680;
            }
            // Input 4 (the item's focus event) reaches here with the scroll row in V0.
            c.cycles += 13;
            c.g[A0][0] = sx(onin_at(c.g[S0][0], ONIN_LIST));
            c.g[A1][0] = c.lbu(onin_at(c.g[S0][0], ONIN_SCROLL_OFFSET));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            ONIN_LIST_GET_AT.call(c, 0x0039_b628)?;
            c.g[V1][0] = c.lw(onin_at(c.g[S0][0], ONIN_SCREEN));
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 0x4;
            c.g[A3][0] = c.lw(onin_at(c.g[V1][0], 0xd0));
            c.g[V0][0] = c.lw(onin_at(c.g[A3][0], 0x8));
            onin_vcall(c, V0, A3, 0x98, V1, 0x0039_b64c)?;
            break 'b_39b64c;
        }
        // The UI sound listener (the owner's, else gUISoundNull) plays the result.
        'b_39b67c: {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            ONIN_GET_STATE_DATA.call(c, 0x0039_b654)?;
            c.g[A2][0] = c.lw(onin_at(c.g[V0][0], 0x14));
            c.g[V0][0] = sx(onin_at(c.g[GP][0], 0x2968));
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V0][0];
            }
            let no_listener = c.g[A2][0] == 0;
            c.g[A1][0] = c.g[S3][0];
            if no_listener {
                break 'b_39b67c;
            }
            c.cycles += 5;
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            onin_vcall(c, V0, A2, 0x10, V1, 0x0039_b67c)?;
            break 'b_39b67c;
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        break 'b_39b680;
    }

    c.cycles += 7;
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};
    use crate::game::ealib::seg_2dcae8::tests::OC445_GET_AT;
    use crate::game::ui::uitext::tests::oc445_gate;

    /// The menu object (its cUIThing mAttr at 0x14, mAttrMenu at 0x90).
    const SELF: u32 = DATA + 0x1000;
    /// The menu's screen; its owner at +0xd0 has the focus handlers.
    const SCREEN: u32 = DATA + 0x2000;
    const OWNER: u32 = DATA + 0x3000;
    const VT_OWNER: u32 = DATA + 0x3800;
    /// The input event (its vtable's slots 2 and 4-8 are the button tests).
    const INPUT: u32 = DATA + 0x4000;
    const VT_INPUT: u32 = DATA + 0x4800;
    /// The selected item (its vtable's slot at 0x78 is `onInput`).
    const CUR: u32 = DATA + 0x5000;
    const VT_CUR: u32 = DATA + 0x5800;
    /// The item `UIList_GetAt` returns.
    const ITEM: u32 = DATA + 0x6000;
    const VT_ITEM: u32 = DATA + 0x6800;
    /// The UI sound listener (`gUISoundNull`, a vtable pointer in GP+0x2968).
    const VT_SOUND: u32 = DATA + 0x7000;
    const SOUND_NULL: u32 = GAME_GP + 0x2968;
    /// A real leaf that returns 0 (a vtable slot that answers "not used").
    const ZERO_LEAF: u32 = 0x0014_06c0;

    /// One vtable slot at `off`: a random delta, and either the zero leaf or
    /// a fake method (the hashed result of the fake call).
    fn onin_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, zero: bool, n: u32) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + off, delta as u16);
        let f = if zero {
            ZERO_LEAF
        } else {
            fakes::FAKE + 0x10 * n
        };
        c.bus.write32(vt + off + 4, f);
    }

    fn onin_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x39_b000, 6);
        oc445_gate(c);
        // Forced results of the first fake calls (0: hashed).
        let forced = [0u32, 1, 0x101, 0x11, 0x10, 0x100][seed as usize % 6];
        for n in 0..8u32 {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n, forced);
        }
        // The menu: hidden on seed % 7 == 0, inactive (mAttr bit 4 clear) on
        // seed % 5 == 0; no selected item on seed % 6 == 0.
        let mut attr = r.next_u32() & !0x30;
        if seed % 7 == 0 {
            attr |= 1 << 5;
        }
        if seed % 5 != 0 {
            attr |= 1 << 4;
        }
        c.bus.write32(SELF + ONIN_ATTR, attr);
        let menu_word = if seed % 4 == 0 {
            r.next_u32() & !8
        } else {
            r.next_u32() | 8
        };
        c.bus.write32(SELF + ONIN_ATTR_MENU, menu_word);
        for k in 0..4 {
            c.bus.write8(SELF + ONIN_INPUT_MAP + k, r.next_u32() as u8);
        }
        c.bus.write8(SELF + ONIN_SCROLL_CUR, r.next_u32() as u8);
        c.bus.write8(SELF + 0x97, 1 + (r.next_u32() % 5) as u8);
        c.bus
            .write8(SELF + ONIN_SCROLL_OFFSET, (r.next_u32() % 6) as u8);
        c.bus
            .write32(SELF + ONIN_SCREEN, if seed % 3 == 0 { 0 } else { SCREEN });
        c.bus
            .write32(SELF + ONIN_SELECTED, if seed % 6 == 0 { 0 } else { CUR });
        // The screen's owner and its focus handlers (slots 19 and 20).
        c.bus.write32(SCREEN + 0xd0, OWNER);
        c.bus.write32(OWNER + 8, VT_OWNER);
        onin_slot(c, &mut r, VT_OWNER, 0x98, seed % 2 == 0, 1);
        onin_slot(c, &mut r, VT_OWNER, 0xa0, seed % 3 == 0, 2);
        // The input: slot 2 (button) and slots 4-8 (events), zero or fake
        // by bit i of the seed.
        c.bus.write32(INPUT + 8, VT_INPUT);
        for (i, off) in [0x10u32, 0x20, 0x28, 0x30, 0x38, 0x40]
            .into_iter()
            .enumerate()
        {
            let zero = (seed >> i) & 1 == 0;
            onin_slot(c, &mut r, VT_INPUT, off, zero, 3 + i as u32);
        }
        // The selected item: onInput returns 0 or 1 (fake).
        c.bus.write32(CUR + 8, VT_CUR);
        c.bus.write32(CUR + ONIN_ATTR, r.next_u32());
        onin_slot(c, &mut r, VT_CUR, 0x78, seed % 2 == 0, 9);
        // What UIList_GetAt returns (the oc445 fake), and its vtable.
        c.bus.write32(ITEM + 8, VT_ITEM);
        c.bus.write32(OC445_GET_AT, ITEM);
        // The sound listener (gUISoundNull's vtable: slot 2 plays).
        c.bus.write32(SOUND_NULL, VT_SOUND);
        onin_slot(c, &mut r, VT_SOUND, 0x10, seed % 4 == 0, 10);
        c.g[A0][0] = sx(SELF);
        c.g[A1][0] = sx(INPUT);
        c.pc = 0x0039_b000;
    }

    #[test]
    fn ui_menu_on_input_matches() {
        check(
            ee::s13::f_0039b000__generated,
            ui_menu_on_input,
            300,
            &onin_setup,
        );
    }
}
