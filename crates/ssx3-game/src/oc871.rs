//! Staging module oc871 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc871`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001a_b478,
    net_fe_state_setup_buddy_action_popup,
    "NetFEState_SetupBuddyActionPopup",
)];

/// `cPopupInfo_clearPopupConfirmation` (0x1c5860): clears the option list.
const CLEAR_POPUP_CONFIRMATION: Callee = Callee {
    addr: 0x001c_5860,
    f: ee::s03::f_001c5860,
};

/// `cFEPopupConfirm_CfgSetOptionTextId` (0x1c57c0): adds a hashed-text option.
const SET_OPTION_TEXT_ID: Callee = Callee {
    addr: 0x001c_57c0,
    f: ee::s03::f_001c57c0,
};

/// `cFEPopupConfirm_CfgSetOptionConfirmWide` (0x1c5838): adds an option with
/// a formatted message.
const SET_OPTION_CONFIRM_WIDE: Callee = Callee {
    addr: 0x001c_5838,
    f: ee::s03::f_001c5838,
};

/// `NetFEState_ShowPopup` (0x1a9bc0): shows the filled popup.
const SHOW_POPUP: Callee = Callee {
    addr: 0x001a_9bc0,
    f: ee::s03::f_001a9bc0,
};

/// `void NetFEState_SetupBuddyActionPopup(cFEStateNet* self, cScreenPopup*
/// popup, const char* name, int fiveOptions, int selectedOption)` (0x1ab478).
///
/// Fills the popup with the buddy actions for `name`: the challenge (or
/// accept/decline) entry for five-option popups, the message entry, the
/// add/remove-messenger entry and the block/unblock entry, then sets the
/// default option, centres the list and shows the popup. The line texts are
/// formatted into `self`'s status-line buffers.
pub fn net_fe_state_setup_buddy_action_popup(c: &mut Cpu) {
    guest(c, net_fe_state_setup_buddy_action_popup_body);
}

fn net_fe_state_setup_buddy_action_popup_body(c: &mut Cpu) -> Flow {
    c.cycles += 17;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S5][0] = c.g[A2][0];
    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S6][0] = c.g[A1][0];
    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S7][0] = c.g[A0][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.g[FP][0] = c.g[A3][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
    let t = c.g[S6][0] == 0u64;
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T0][0]);
    'b_1abb00: {
        if t {
            break 'b_1abb00;
        } else {
            'b_1ab4d0: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64;
                if t {
                    break 'b_1ab4d0;
                } else {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_b4d0)?;
                    break 'b_1ab4d0;
                }
            }
            'b_1ab4ec: {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x0026_1460, 0x001a_b4d8)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab4ec;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x0026_13c8, 0x001a_b4e8)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab4ec;
                }
            }
            'b_1ab500: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab500;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025_b800, 0x001a_b4fc)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab500;
                }
            }
            'b_1ab520: {
                c.cycles += 6;
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x0026_3490, 0x001a_b508)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64;
                if t {
                    break 'b_1ab520;
                } else {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_b520)?;
                    break 'b_1ab520;
                }
            }
            'b_1ab53c: {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x0026_1460, 0x001a_b528)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab53c;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x0026_13c8, 0x001a_b538)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab53c;
                }
            }
            'b_1ab550: {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab550;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025_b800, 0x001a_b54c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab550;
                }
            }
            'b_1ab77c: {
                'b_1ab658: {
                    c.cycles += 21;
                    c.g[A1][0] = c.g[S5][0];
                    c.g[S4][0] = 0u64;
                    c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_add(0xbc));
                    call_ptr(c, 0x0026_33c8, 0x001a_b560)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.g[A1][0] = c.g[S5][0];
                    c.g[S1][0] = c.g[V0][0];
                    call_ptr(c, 0x0031_82d8, 0x001a_b574)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x98));
                    call_ptr(c, 0x0031_8448, 0x001a_b580)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0031_8260, 0x001a_b58c)?;
                    c.g[V0][0] = 0x2;
                    c.g[V1][0] = 0x3;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x14c), c.g[V1][0]);
                    let t = c.g[FP][0] == 0u64;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0xcc), c.g[S5][0]);
                    if t {
                        break 'b_1ab658;
                    } else {
                        'b_1ab5d4: {
                            c.cycles += 10;
                            c.g[V0][0] = 0x5;
                            c.g[V1][0] = 0x80b;
                            c.sw((c.g[S6][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                            c.g[A0][0] = c.g[S3][0];
                            c.sw((c.g[S6][0] as u32).wrapping_add(0x150), c.g[V1][0]);
                            c.g[A1][0] = 0u64;
                            CLEAR_POPUP_CONFIRMATION.call(c, 0x001a_b5c0)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            let t = c.g[V0][0] != 0u64;
                            if t {
                                break 'b_1ab5d4;
                            } else {
                                c.cycles += 2;
                                call_ptr(c, 0x0026_0f80, 0x001a_b5d4)?;
                                break 'b_1ab5d4;
                            }
                        }
                        'b_1ab5f0: {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            call_ptr(c, 0x0026_1460, 0x001a_b5dc)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            if t {
                                break 'b_1ab5f0;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x0026_13c8, 0x001a_b5ec)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                break 'b_1ab5f0;
                            }
                        }
                        'b_1ab604: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            if t {
                                break 'b_1ab604;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                call_ptr(c, 0x0025_b800, 0x001a_b600)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                break 'b_1ab604;
                            }
                        }
                        'b_1ab634: {
                            c.cycles += 4;
                            c.g[A1][0] = c.g[S5][0];
                            call_ptr(c, 0x0026_3128, 0x001a_b60c)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = 0x460000;
                            if t {
                                break 'b_1ab634;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2938));
                                call_ptr(c, 0x0031_7670, 0x001a_b61c)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A1][0] = 0u64;
                                SET_OPTION_TEXT_ID.call(c, 0x001a_b62c)?;
                                let t = true;
                                c.g[S4][0] = 0x1;
                                if t {
                                    break 'b_1ab77c;
                                } else {
                                    break 'b_1ab634;
                                }
                            }
                        }
                        c.cycles += 9;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2958));
                        call_ptr(c, 0x0031_7670, 0x001a_b640)?;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = 0u64;
                        SET_OPTION_TEXT_ID.call(c, 0x001a_b650)?;
                        let t = true;
                        c.g[S4][0] = 0x1;
                        if t {
                            break 'b_1ab77c;
                        } else {
                            break 'b_1ab658;
                        }
                    }
                }
                'b_1ab770: {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        break 'b_1ab770;
                    } else {
                        'b_1ab76c: {
                            'b_1ab6bc: {
                                c.cycles += 2;
                                let t = c.g[S2][0] != 0u64;
                                c.g[V0][0] = 0x4;
                                if t {
                                    break 'b_1ab6bc;
                                } else {
                                    'b_1ab67c: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        let t = c.g[V0][0] != 0u64;
                                        if t {
                                            break 'b_1ab67c;
                                        } else {
                                            c.cycles += 2;
                                            call_ptr(c, 0x0026_0f80, 0x001a_b67c)?;
                                            break 'b_1ab67c;
                                        }
                                    }
                                    'b_1ab698: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x0026_1460, 0x001a_b684)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1ab698;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            call_ptr(c, 0x0026_13c8, 0x001a_b694)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1ab698;
                                        }
                                    }
                                    'b_1ab6ac: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1ab6ac;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                            call_ptr(c, 0x0025_b800, 0x001a_b6a8)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1ab6ac;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = c.g[S5][0];
                                    call_ptr(c, 0x0026_3128, 0x001a_b6b4)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x4;
                                    if t {
                                        break 'b_1ab76c;
                                    } else {
                                        break 'b_1ab6bc;
                                    }
                                }
                            }
                            'b_1ab6e0: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S3][0];
                                c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                                c.g[A1][0] = 0u64;
                                CLEAR_POPUP_CONFIRMATION.call(c, 0x001a_b6cc)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                let t = c.g[V0][0] != 0u64;
                                if t {
                                    break 'b_1ab6e0;
                                } else {
                                    c.cycles += 2;
                                    call_ptr(c, 0x0026_0f80, 0x001a_b6e0)?;
                                    break 'b_1ab6e0;
                                }
                            }
                            'b_1ab6fc: {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x0026_1460, 0x001a_b6e8)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if t {
                                    break 'b_1ab6fc;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    call_ptr(c, 0x0026_13c8, 0x001a_b6f8)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    break 'b_1ab6fc;
                                }
                            }
                            'b_1ab710: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if t {
                                    break 'b_1ab710;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                    call_ptr(c, 0x0025_b800, 0x001a_b70c)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    break 'b_1ab710;
                                }
                            }
                            'b_1ab760: {
                                'b_1ab740: {
                                    c.cycles += 4;
                                    c.g[A1][0] = c.g[S5][0];
                                    call_ptr(c, 0x0026_3128, 0x001a_b718)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1ab740;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2938));
                                        call_ptr(c, 0x0031_7670, 0x001a_b728)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A0][0] = c.g[S3][0];
                                        c.g[A1][0] = 0u64;
                                        SET_OPTION_TEXT_ID.call(c, 0x001a_b738)?;
                                        let t = true;
                                        c.g[V0][0] = 0x80b;
                                        if t {
                                            break 'b_1ab760;
                                        } else {
                                            break 'b_1ab740;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2958));
                                call_ptr(c, 0x0031_7670, 0x001a_b74c)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A1][0] = 0u64;
                                SET_OPTION_TEXT_ID.call(c, 0x001a_b75c)?;
                                c.g[V0][0] = 0x80b;
                                break 'b_1ab760;
                            }
                            c.cycles += 3;
                            c.g[S4][0] = 0x1;
                            let t = true;
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x94), c.g[V0][0]);
                            if t {
                                break 'b_1ab77c;
                            } else {
                                break 'b_1ab76c;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = 0x3;
                        break 'b_1ab770;
                    }
                }
                c.cycles += 3;
                c.g[V1][0] = 0x80c;
                c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x94), c.g[V1][0]);
                break 'b_1ab77c;
            }
            'b_1ab790: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64;
                if t {
                    break 'b_1ab790;
                } else {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_b790)?;
                    break 'b_1ab790;
                }
            }
            'b_1ab7ac: {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x0026_1460, 0x001a_b798)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab7ac;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x0026_13c8, 0x001a_b7a8)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab7ac;
                }
            }
            'b_1ab7c0: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab7c0;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025_b800, 0x001a_b7bc)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab7c0;
                }
            }
            'b_1ab834: {
                'b_1ab804: {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0026_2a38, 0x001a_b7c8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S4][0];
                    if t {
                        break 'b_1ab804;
                    } else {
                        c.cycles += 13;
                        c.g[A0][0] = c.g[S3][0];
                        CLEAR_POPUP_CONFIRMATION.call(c, 0x001a_b7d8)?;
                        c.g[S0][0] = c.g[S4][0];
                        c.g[A0][0] = 0x460000;
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2970));
                        call_ptr(c, 0x0031_7670, 0x001a_b7ec)?;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        SET_OPTION_TEXT_ID.call(c, 0x001a_b7fc)?;
                        let t = true;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        if t {
                            break 'b_1ab834;
                        } else {
                            break 'b_1ab804;
                        }
                    }
                }
                c.cycles += 12;
                c.g[A0][0] = c.g[S3][0];
                CLEAR_POPUP_CONFIRMATION.call(c, 0x001a_b80c)?;
                c.g[S0][0] = c.g[S4][0];
                c.g[A0][0] = 0x460000;
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2988));
                call_ptr(c, 0x0031_7670, 0x001a_b820)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                SET_OPTION_TEXT_ID.call(c, 0x001a_b830)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                break 'b_1ab834;
            }
            'b_1ab844: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                if t {
                    break 'b_1ab844;
                } else {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_b844)?;
                    break 'b_1ab844;
                }
            }
            'b_1ab860: {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x0026_1460, 0x001a_b84c)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab860;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x0026_13c8, 0x001a_b85c)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab860;
                }
            }
            'b_1ab874: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab874;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025_b800, 0x001a_b870)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab874;
                }
            }
            'b_1ab978: {
                'b_1ab900: {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0026_2360, 0x001a_b87c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_1ab900;
                    } else {
                        c.cycles += 31;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x29a0));
                        c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x49c));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x0031_7670, 0x001a_b8a4)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x001a_b8b4)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S5][0];
                        call_ptr(c, 0x002c_26d0, 0x001a_b8c4)?;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A3][0] = 0u64;
                        SET_OPTION_CONFIRM_WIDE.call(c, 0x001a_b8d8)?;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x29c8));
                        call_ptr(c, 0x0031_7670, 0x001a_b8e4)?;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                        SET_OPTION_TEXT_ID.call(c, 0x001a_b8f8)?;
                        let t = true;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        if t {
                            break 'b_1ab978;
                        } else {
                            break 'b_1ab900;
                        }
                    }
                }
                c.cycles += 30;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x29e8));
                c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x49c));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x001a_b920)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001a_b930)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S5][0];
                call_ptr(c, 0x002c_26d0, 0x001a_b940)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[A3][0] = 0u64;
                SET_OPTION_CONFIRM_WIDE.call(c, 0x001a_b954)?;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2a08));
                call_ptr(c, 0x0031_7670, 0x001a_b960)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                SET_OPTION_TEXT_ID.call(c, 0x001a_b974)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                break 'b_1ab978;
            }
            'b_1ab988: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                if t {
                    break 'b_1ab988;
                } else {
                    c.cycles += 2;
                    call_ptr(c, 0x0026_0f80, 0x001a_b988)?;
                    break 'b_1ab988;
                }
            }
            'b_1ab9a4: {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x0026_1460, 0x001a_b990)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab9a4;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x0026_13c8, 0x001a_b9a0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab9a4;
                }
            }
            'b_1ab9b8: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t {
                    break 'b_1ab9b8;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025_b800, 0x001a_b9b4)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    break 'b_1ab9b8;
                }
            }
            'b_1abab8: {
                'b_1aba44: {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0026_23b0, 0x001a_b9c0)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_1aba44;
                    } else {
                        c.cycles += 31;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2a20));
                        c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x59c));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x0031_7670, 0x001a_b9e8)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x001a_b9f8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S5][0];
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x002c_26d0, 0x001a_ba08)?;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A3][0] = 0u64;
                        SET_OPTION_CONFIRM_WIDE.call(c, 0x001a_ba1c)?;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2a40));
                        call_ptr(c, 0x0031_7670, 0x001a_ba28)?;
                        c.g[A1][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                        SET_OPTION_TEXT_ID.call(c, 0x001a_ba3c)?;
                        let t = true;
                        if t {
                            break 'b_1abab8;
                        } else {
                            break 'b_1aba44;
                        }
                    }
                }
                c.cycles += 29;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2a58));
                c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x59c));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x001a_ba64)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001a_ba74)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x002c_26d0, 0x001a_ba84)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[A3][0] = 0u64;
                SET_OPTION_CONFIRM_WIDE.call(c, 0x001a_ba98)?;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2a78));
                call_ptr(c, 0x0031_7670, 0x001a_baa4)?;
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                SET_OPTION_TEXT_ID.call(c, 0x001a_bab8)?;
                break 'b_1abab8;
            }
            'b_1abae4: {
                c.cycles += 2;
                let t = c.g[FP][0] == 0u64;
                c.g[A1][0] = c.g[S4][0];
                if t {
                    break 'b_1abae4;
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S3][0];
                    CLEAR_POPUP_CONFIRMATION.call(c, 0x001a_bac8)?;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2a90));
                    call_ptr(c, 0x0031_7670, 0x001a_bad4)?;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    SET_OPTION_TEXT_ID.call(c, 0x001a_bae4)?;
                    break 'b_1abae4;
                }
            }
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.g[S7][0];
            c.g[A1][0] = c.g[S6][0];
            c.sw((c.g[S3][0] as u32).wrapping_add(0xac), c.g[V0][0]);
            c.g[V0][0] = 0x1;
            c.sw((c.g[S3][0] as u32).wrapping_add(0xb8), c.g[V0][0]);
            SHOW_POPUP.call(c, 0x001a_bb00)?;
            break 'b_1abb00;
        }
    }
    c.cycles += 12;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
    c.pc = t;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word in the machine's RAM: the fakes answer only when it is set.
    const OC871_CTRL: u32 = DATA + 0x20_0000;
    const OC871_MAGIC: u32 = 0x4f43_3837;
    /// Bit n: the boolean answer of the n-th boolean callee.
    const OC871_BITS: u32 = OC871_CTRL + 4;

    /// A fake call to `target` that returns bit `bit` of [`OC871_BITS`] (a
    /// boolean answer) when it comes back.
    fn oc871_answer(c: &mut Cpu, target: u32, bit: u32) {
        let ret = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ret {
            c.g[V0][0] = ((c.bus.read32(OC871_BITS) >> bit) & 1) as u64;
        }
    }

    macro_rules! oc871_fake {
        ($fake:ident, $gen:path, $target:expr) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC871_CTRL) != OC871_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    macro_rules! oc871_bool_fake {
        ($fake:ident, $gen:path, $target:expr, $bit:expr) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC871_CTRL) != OC871_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                }
                oc871_answer(c, $target, $bit);
            }
        };
    }

    oc871_bool_fake!(
        oc871_fake_263490,
        ee::s07::f_00263490__generated,
        0x0026_3490,
        0
    );
    oc871_bool_fake!(
        oc871_fake_2633c8,
        ee::s07::f_002633c8__generated,
        0x0026_33c8,
        1
    );
    oc871_bool_fake!(
        oc871_fake_263128,
        ee::s07::f_00263128__generated,
        0x0026_3128,
        2
    );
    oc871_bool_fake!(
        oc871_fake_262a38,
        ee::s07::f_00262a38__generated,
        0x0026_2a38,
        3
    );
    oc871_bool_fake!(
        oc871_fake_262360,
        ee::s07::f_00262360__generated,
        0x0026_2360,
        4
    );
    oc871_bool_fake!(
        oc871_fake_2623b0,
        ee::s07::f_002623b0__generated,
        0x0026_23b0,
        5
    );
    oc871_bool_fake!(
        oc871_fake_261460,
        ee::s06::f_00261460__generated,
        0x0026_1460,
        6
    );
    oc871_fake!(
        oc871_fake_260f80,
        ee::s06::f_00260f80__generated,
        0x0026_0f80
    );
    oc871_fake!(
        oc871_fake_2613c8,
        ee::s06::f_002613c8__generated,
        0x0026_13c8
    );
    oc871_fake!(
        oc871_fake_25b800,
        ee::s06::f_0025b800__generated,
        0x0025_b800
    );
    oc871_fake!(
        oc871_fake_317670,
        ee::s10::f_00317670__generated,
        0x0031_7670
    );
    oc871_fake!(
        oc871_fake_3182d8,
        ee::s10::f_003182d8__generated,
        0x0031_82d8
    );
    oc871_fake!(
        oc871_fake_318448,
        ee::s10::f_00318448__generated,
        0x0031_8448
    );
    oc871_fake!(
        oc871_fake_318260,
        ee::s10::f_00318260__generated,
        0x0031_8260
    );
    oc871_fake!(
        oc871_fake_2c26d0,
        ee::s08::f_002c26d0__generated,
        0x002c_26d0
    );
    oc871_fake!(
        oc871_fake_1c5860,
        ee::s03::f_001c5860__generated,
        0x001c_5860
    );
    oc871_fake!(
        oc871_fake_1c57c0,
        ee::s03::f_001c57c0__generated,
        0x001c_57c0
    );
    oc871_fake!(
        oc871_fake_1c5838,
        ee::s03::f_001c5838__generated,
        0x001c_5838
    );
    oc871_fake!(
        oc871_fake_1a9bc0,
        ee::s03::f_001a9bc0__generated,
        0x001a_9bc0
    );

    /// Hook the callee slots once per process (each fake is gated on
    /// [`OC871_CTRL`], so other tests' machines run the real bodies).
    fn oc871_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00263490),
            oc871_fake_263490,
            fakes::prev_cell(oc871_fake_263490),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002633C8),
            oc871_fake_2633c8,
            fakes::prev_cell(oc871_fake_2633c8),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00263128),
            oc871_fake_263128,
            fakes::prev_cell(oc871_fake_263128),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262A38),
            oc871_fake_262a38,
            fakes::prev_cell(oc871_fake_262a38),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262360),
            oc871_fake_262360,
            fakes::prev_cell(oc871_fake_262360),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002623B0),
            oc871_fake_2623b0,
            fakes::prev_cell(oc871_fake_2623b0),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261460),
            oc871_fake_261460,
            fakes::prev_cell(oc871_fake_261460),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00260F80),
            oc871_fake_260f80,
            fakes::prev_cell(oc871_fake_260f80),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002613C8),
            oc871_fake_2613c8,
            fakes::prev_cell(oc871_fake_2613c8),
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025B800),
            oc871_fake_25b800,
            fakes::prev_cell(oc871_fake_25b800),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc871_fake_317670,
            fakes::prev_cell(oc871_fake_317670),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            oc871_fake_3182d8,
            fakes::prev_cell(oc871_fake_3182d8),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318448),
            oc871_fake_318448,
            fakes::prev_cell(oc871_fake_318448),
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            oc871_fake_318260,
            fakes::prev_cell(oc871_fake_318260),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            oc871_fake_2c26d0,
            fakes::prev_cell(oc871_fake_2c26d0),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C5860),
            oc871_fake_1c5860,
            fakes::prev_cell(oc871_fake_1c5860),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C57C0),
            oc871_fake_1c57c0,
            fakes::prev_cell(oc871_fake_1c57c0),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C5838),
            oc871_fake_1c5838,
            fakes::prev_cell(oc871_fake_1c5838),
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A9BC0),
            oc871_fake_1a9bc0,
            fakes::prev_cell(oc871_fake_1a9bc0),
        );
    }

    /// Fixture: the popup, the self object, the name, the world and the
    /// text table the body reads through `gp`, with the boolean answers and
    /// the NULL/session/five-option choices taken from `seed`.
    fn oc871_setup(c: &mut Cpu, seed: u32) {
        const POPUP: u32 = DATA + 0x1000;
        const SELF: u32 = DATA + 0x4000;
        const NAME: u32 = DATA + 0x8000;
        const SESSION: u32 = DATA + 0x20_0100;
        const LOBBY: u32 = DATA + 0x20_0200;
        const WORLD: u32 = DATA + 0x20_0400;
        const TEXT_OWNER: u32 = DATA + 0x20_0500;
        const TEXT_TABLE: u32 = DATA + 0x20_0600;

        let mut r = fakes::machine(c, seed, 0x871, 12);
        c.bus.write32(OC871_CTRL, OC871_MAGIC);
        c.bus.write32(OC871_BITS, r.next_u32() & 0x7f);
        c.bus
            .write32(GAME_GP + 0x238, if seed % 5 == 0 { 0 } else { SESSION });
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), LOBBY);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
        c.bus.write32(WORLD + 0x8c, TEXT_OWNER);
        c.bus.write32(TEXT_OWNER + 4, TEXT_TABLE);
        c.bus.write16(TEXT_TABLE + 0x20, 0x10);
        c.bus.write32(TEXT_TABLE + 0x24, fakes::FAKE);

        c.pc = 0x001a_b478;
        c.g[A0][0] = sx(SELF);
        c.g[A1][0] = if seed % 16 == 0 { 0 } else { sx(POPUP) };
        c.g[A2][0] = sx(NAME);
        c.g[A3][0] = (seed % 2) as u64;
        c.g[T0][0] = (seed % 4) as u64;
    }

    #[test]
    fn net_fe_state_setup_buddy_action_popup_matches() {
        oc871_install_fakes();
        fakes::covered(
            ee::s03::f_001ab478__generated,
            net_fe_state_setup_buddy_action_popup,
            300,
            &oc871_setup,
        );
    }
}
