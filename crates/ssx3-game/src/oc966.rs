//! Staging module oc966 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc966`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001e_0f60,
        c_fe_state_net_messenger_ctor_2,
        "cFEStateNetMessenger_ctor_2",
    ),
    (
        0x001e_0ff0,
        c_fe_state_read_message_popup_ctor_2,
        "cFEStateReadMessagePopup_ctor_2",
    ),
    (0x001e_12b0, front_end_global_ctors, "FrontEnd_GlobalCtors"),
    (0x001e_12d0, front_end_global_dtors, "FrontEnd_GlobalDtors"),
    (
        0x001e_1c10,
        message_center_post_foreshadow,
        "MessageCenter_PostForeshadow",
    ),
    (
        0x001e_20d0,
        message_center_post_peak_message_b,
        "MessageCenter_PostPeakMessageB",
    ),
    (
        0x001e_2220,
        message_center_post_peak_message_a,
        "MessageCenter_PostPeakMessageA",
    ),
    (
        0x001e_2648,
        message_center_post_bc_finish_message_a,
        "MessageCenter_PostBCFinishMessageA",
    ),
];

/// `cFEStateNetMessenger_ctor_2` (0x001e0f60): runs the base `NetFEState_ctor`
/// on `this`, then sets the messenger vtable (0x467f28) at +0x8, the shared
/// `D_004A3E90` value at +0x6d0, +0x6d4 and +0x6e0, and clears the three words
/// at +0x6e8..+0x6f0.
pub fn c_fe_state_net_messenger_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001e_0f7c)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xda0)).lw(c);
        c.g[V1][0] = sx(0x0046_7f28);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        let vtable = c.g[V1][0];
        let shared = c.g[A0][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        Addr(this.wrapping_add(0x6e0)).sw(c, shared);
        Addr(this.wrapping_add(0x6d0)).sw(c, shared);
        Addr(this.wrapping_add(0x6d4)).sw(c, shared);
        Addr(this.wrapping_add(0x6e8)).sw(c, 0);
        Addr(this.wrapping_add(0x6ec)).sw(c, 0);
        Addr(this.wrapping_add(0x6f0)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateReadMessagePopup_ctor_2` (0x001e0ff0): the base `NetFEState_ctor`
/// on `this`, then the popup vtable (0x467e18) at +0x8, the shared
/// `D_004A3E90` value at +0x6d0, and zeros at +0x6dc, +0x6e0 and +0x6e4.
pub fn c_fe_state_read_message_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001e_100c)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xda0)).lw(c);
        c.g[V1][0] = sx(0x0046_7e18);
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        let vtable = c.g[V1][0];
        let shared = c.g[A0][0];
        Addr(this.wrapping_add(0x8)).sw(c, vtable);
        Addr(this.wrapping_add(0x6d0)).sw(c, shared);
        Addr(this.wrapping_add(0x6dc)).sw(c, 0);
        Addr(this.wrapping_add(0x6e0)).sw(c, 0);
        Addr(this.wrapping_add(0x6e4)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `FrontEnd_GlobalCtors` (0x001e12b0): the static-init hook of the front end,
/// `FrontEnd_StaticInitDestroy(1, 0xffff)`.
pub fn front_end_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 1;
        c.g[A1][0] = 0xffff;
        call_ptr(c, 0x001d_bc98, 0x001e_12c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FrontEnd_GlobalDtors` (0x001e12d0): `FrontEnd_StaticInitDestroy(0, 0xffff)`.
pub fn front_end_global_dtors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0xffff;
        call_ptr(c, 0x001d_bc98, 0x001e_12e4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// The four message posts below are structural ports: the generated labelled
// blocks are kept as they are (every `lq` restore and cycle charge sits on the
// path it does in the generated body), with the calls through `Callee` and the
// unwind checks through `?`. Each one's game-level meaning is in its doc
// comment; the block nesting is the generated one.

/// `MessageCenter_PostForeshadow` (0x001e1c10, `func_001E1C10(group)`): unless the
/// replay is busy (replay counter 0 or at least 10), the foreshadow group
/// `which` (0..2) is skipped if its flags 0x1c/0x1d (or 0x1e/0x1f, 0x20/0x21)
/// are set. Otherwise it picks the foreshadow message for the human rider's
/// character from that group's table and posts it unless the pick is 0x100.
pub fn message_center_post_foreshadow(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e1c40: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e1c40;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e1c40;
            }
        }
        'b_1e1dc0: {
            c.cycles += 2;
            let t = c.g[A1][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_1e1dc0;
            } else {
                'b_1e1d5c: {
                    'b_1e1d34: {
                        'b_1e1cf4: {
                            c.cycles += 16;
                            c.g[RA][0] = sx(0x001e1c50);
                            call_ptr(c, 0x0014dc80, 0x001e1c50)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001e1c5c);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014dd58, 0x001e1c5c)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[RA][0] = sx(0x001e1c6c);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00146e98, 0x001e1c6c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001e1c78);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x00147398, 0x001e1c78)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V0][0] = 0x1;
                            let t = c.g[S1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_1e1cf4;
                            } else {
                                'b_1e1cb0: {
                                    'b_1e1ca0: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_1e1ca0;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == 0u64;

                                            if t {
                                                break 'b_1e1cb0;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_1e1dc0;
                                                } else {
                                                    break 'b_1e1ca0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[V0][0];

                                    if t {
                                        break 'b_1e1d5c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_1e1dc0;
                                        } else {
                                            break 'b_1e1cb0;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001e1cb8);
                                c.g[A0][0] = 0x1c;
                                Callee {
                                    addr: 0x001e3a30,
                                    f: ee::s04::MessageCenter_IsFlagSet,
                                }
                                .call(c, 0x001e1cb8)?;
                                if c.g[V0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    break 'b_1e1dc0;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001e1cc8);
                                    c.g[A0][0] = 0x1d;
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::MessageCenter_IsFlagSet,
                                    }
                                    .call(c, 0x001e1cc8)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        break 'b_1e1dc0;
                                    } else {
                                        'b_1e1ce8: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x3;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x440000;
                                            if t {
                                                break 'b_1e1ce8;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x230));
                                                if t {
                                                    break 'b_1e1d34;
                                                } else {
                                                    break 'b_1e1ce8;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x244));
                                        if t {
                                            break 'b_1e1d34;
                                        } else {
                                            break 'b_1e1cf4;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e1cfc);
                        c.g[A0][0] = 0x1e;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e1cfc)?;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_1e1dc0;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001e1d0c);
                            c.g[A0][0] = 0x1f;
                            Callee {
                                addr: 0x001e3a30,
                                f: ee::s04::MessageCenter_IsFlagSet,
                            }
                            .call(c, 0x001e1d0c)?;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_1e1dc0;
                            } else {
                                'b_1e1d2c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x7;
                                    let t = c.g[S0][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x440000;
                                    if t {
                                        break 'b_1e1d2c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x258));
                                        if t {
                                            break 'b_1e1d34;
                                        } else {
                                            break 'b_1e1d2c;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x26c));
                                break 'b_1e1d34;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x001e1d3c);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001e1ad0,
                        f: ee::s04::MessageCenter_PickForeshadowMessage,
                    }
                    .call(c, 0x001e1d3c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x100;
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1e1dc0;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e1d54);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001e2fe0,
                            f: ee::s04::MessageCenter_PostMessage,
                        }
                        .call(c, 0x001e1d54)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1e1dc0;
                        } else {
                            break 'b_1e1d5c;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001e1d64);
                c.g[A0][0] = 0x20;
                Callee {
                    addr: 0x001e3a30,
                    f: ee::s04::MessageCenter_IsFlagSet,
                }
                .call(c, 0x001e1d64)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_1e1dc0;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e1d74);
                    c.g[A0][0] = 0x21;
                    Callee {
                        addr: 0x001e3a30,
                        f: ee::s04::MessageCenter_IsFlagSet,
                    }
                    .call(c, 0x001e1d74)?;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_1e1dc0;
                    } else {
                        'b_1e1d9c: {
                            'b_1e1d94: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x8;
                                let t = c.g[S0][0] == c.g[V0][0];
                                c.g[V0][0] = 0x440000;
                                if t {
                                    break 'b_1e1d94;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x280));
                                    if t {
                                        break 'b_1e1d9c;
                                    } else {
                                        break 'b_1e1d94;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x294));
                            break 'b_1e1d9c;
                        }
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x001e1da4);
                        c.g[A1][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001e1ad0,
                            f: ee::s04::MessageCenter_PickForeshadowMessage,
                        }
                        .call(c, 0x001e1da4)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[V0][0] = 0x100;
                        let t = c.g[A0][0] == c.g[V0][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1e1dc0;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x001e1dbc);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e2fe0,
                                f: ee::s04::MessageCenter_PostMessage,
                            }
                            .call(c, 0x001e1dbc)?;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_1e1dc0;
                        }
                    }
                }
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `MessageCenter_PostPeakMessageB` (0x001e20d0, `func_001E20D0(which, flag)`):
/// unless the replay is busy, for peak group `which` (0..2, flags 0x23, 0x25,
/// 0x27) nothing happens if that flag is already set; otherwise the flag is set
/// first when `flag` is non-zero, and the group's message (table 0x441630, entry
/// `f4`) is posted.
pub fn message_center_post_peak_message_b(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e2104: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e2104;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e2104;
            }
        }
        'b_1e220c: {
            c.cycles += 2;
            if c.g[A1][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1e220c;
            } else {
                'b_1e21fc: {
                    'b_1e21e8: {
                        'b_1e21bc: {
                            'b_1e21ac: {
                                'b_1e2178: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
                                    if t {
                                        break 'b_1e2178;
                                    } else {
                                        'b_1e2140: {
                                            'b_1e2130: {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V0][0] = 0x2;
                                                    break 'b_1e2130;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[A2][0] == 0u64;

                                                    if t {
                                                        break 'b_1e2140;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_1e220c;
                                                        } else {
                                                            break 'b_1e2130;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == c.g[V0][0];

                                            if t {
                                                break 'b_1e21bc;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_1e220c;
                                                } else {
                                                    break 'b_1e2140;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001e2148);
                                        c.g[A0][0] = 0x23;
                                        Callee {
                                            addr: 0x001e3a30,
                                            f: ee::s04::MessageCenter_IsFlagSet,
                                        }
                                        .call(c, 0x001e2148)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            break 'b_1e220c;
                                        } else {
                                            'b_1e2170: {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x440000;
                                                let t = c.g[S0][0] == 0u64;
                                                c.g[S1][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                                if t {
                                                    break 'b_1e2170;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[A0][0] = 0x23;
                                                    c.g[RA][0] = sx(0x001e2168);
                                                    c.g[A1][0] = 0x1;
                                                    Callee {
                                                        addr: 0x001e3a78,
                                                        f: ee::s04::MessageCenter_SetFlag,
                                                    }
                                                    .call(c, 0x001e2168)?;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x2c0)
                                                        );
                                                    if t {
                                                        break 'b_1e21e8;
                                                    } else {
                                                        break 'b_1e2170;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x2c0));
                                            if t {
                                                break 'b_1e21ac;
                                            } else {
                                                break 'b_1e2178;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001e2180);
                                c.g[A0][0] = 0x25;
                                Callee {
                                    addr: 0x001e3a30,
                                    f: ee::s04::MessageCenter_IsFlagSet,
                                }
                                .call(c, 0x001e2180)?;
                                if c.g[V0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    break 'b_1e220c;
                                } else {
                                    'b_1e21a8: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x440000;
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                        if t {
                                            break 'b_1e21a8;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A0][0] = 0x25;
                                            c.g[RA][0] = sx(0x001e21a0);
                                            c.g[A1][0] = 0x1;
                                            Callee {
                                                addr: 0x001e3a78,
                                                f: ee::s04::MessageCenter_SetFlag,
                                            }
                                            .call(c, 0x001e21a0)?;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x2e8));
                                            if t {
                                                break 'b_1e21e8;
                                            } else {
                                                break 'b_1e21a8;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2e8));
                                    break 'b_1e21ac;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001e21b4);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e2fe0,
                                f: ee::s04::MessageCenter_PostMessage,
                            }
                            .call(c, 0x001e21b4)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1e220c;
                            } else {
                                break 'b_1e21bc;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e21c4);
                        c.g[A0][0] = 0x27;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e21c4)?;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_1e220c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x440000;
                            let t = c.g[S0][0] == 0u64;
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                            if t {
                                break 'b_1e21fc;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = 0x27;
                                c.g[RA][0] = sx(0x001e21e4);
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x001e3a78,
                                    f: ee::s04::MessageCenter_SetFlag,
                                }
                                .call(c, 0x001e21e4)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x310));
                                break 'b_1e21e8;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001e21f4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x001e2fe0,
                        f: ee::s04::MessageCenter_PostMessage,
                    }
                    .call(c, 0x001e21f4)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1e220c;
                    } else {
                        break 'b_1e21fc;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x310));
                c.g[RA][0] = sx(0x001e2208);
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x001e2fe0,
                    f: ee::s04::MessageCenter_PostMessage,
                }
                .call(c, 0x001e2208)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1e220c;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `MessageCenter_PostPeakMessageA` (0x001e2220, `func_001E2220(which, flag)`):
/// the same as `MessageCenter_PostPeakMessageB` for the groups 0x22, 0x24, 0x26
/// (table entries 34, 36, 38).
pub fn message_center_post_peak_message_a(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e2254: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e2254;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e2254;
            }
        }
        'b_1e235c: {
            c.cycles += 2;
            if c.g[A1][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1e235c;
            } else {
                'b_1e234c: {
                    'b_1e2338: {
                        'b_1e230c: {
                            'b_1e22fc: {
                                'b_1e22c8: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < 2i64) as u64;
                                    if t {
                                        break 'b_1e22c8;
                                    } else {
                                        'b_1e2290: {
                                            'b_1e2280: {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V0][0] = 0x2;
                                                    break 'b_1e2280;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[A2][0] == 0u64;

                                                    if t {
                                                        break 'b_1e2290;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_1e235c;
                                                        } else {
                                                            break 'b_1e2280;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == c.g[V0][0];

                                            if t {
                                                break 'b_1e230c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_1e235c;
                                                } else {
                                                    break 'b_1e2290;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001e2298);
                                        c.g[A0][0] = 0x22;
                                        Callee {
                                            addr: 0x001e3a30,
                                            f: ee::s04::MessageCenter_IsFlagSet,
                                        }
                                        .call(c, 0x001e2298)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            break 'b_1e235c;
                                        } else {
                                            'b_1e22c0: {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x440000;
                                                let t = c.g[S0][0] == 0u64;
                                                c.g[S1][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                                if t {
                                                    break 'b_1e22c0;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[A0][0] = 0x22;
                                                    c.g[RA][0] = sx(0x001e22b8);
                                                    c.g[A1][0] = 0x1;
                                                    Callee {
                                                        addr: 0x001e3a78,
                                                        f: ee::s04::MessageCenter_SetFlag,
                                                    }
                                                    .call(c, 0x001e22b8)?;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x2ac)
                                                        );
                                                    if t {
                                                        break 'b_1e2338;
                                                    } else {
                                                        break 'b_1e22c0;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x2ac));
                                            if t {
                                                break 'b_1e22fc;
                                            } else {
                                                break 'b_1e22c8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001e22d0);
                                c.g[A0][0] = 0x24;
                                Callee {
                                    addr: 0x001e3a30,
                                    f: ee::s04::MessageCenter_IsFlagSet,
                                }
                                .call(c, 0x001e22d0)?;
                                if c.g[V0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    break 'b_1e235c;
                                } else {
                                    'b_1e22f8: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x440000;
                                        let t = c.g[S0][0] == 0u64;
                                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                                        if t {
                                            break 'b_1e22f8;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A0][0] = 0x24;
                                            c.g[RA][0] = sx(0x001e22f0);
                                            c.g[A1][0] = 0x1;
                                            Callee {
                                                addr: 0x001e3a78,
                                                f: ee::s04::MessageCenter_SetFlag,
                                            }
                                            .call(c, 0x001e22f0)?;
                                            let t = true;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x2d4));
                                            if t {
                                                break 'b_1e2338;
                                            } else {
                                                break 'b_1e22f8;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2d4));
                                    break 'b_1e22fc;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001e2304);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001e2fe0,
                                f: ee::s04::MessageCenter_PostMessage,
                            }
                            .call(c, 0x001e2304)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1e235c;
                            } else {
                                break 'b_1e230c;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e2314);
                        c.g[A0][0] = 0x26;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e2314)?;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_1e235c;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x440000;
                            let t = c.g[S0][0] == 0u64;
                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1630));
                            if t {
                                break 'b_1e234c;
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = 0x26;
                                c.g[RA][0] = sx(0x001e2334);
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x001e3a78,
                                    f: ee::s04::MessageCenter_SetFlag,
                                }
                                .call(c, 0x001e2334)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2fc));
                                break 'b_1e2338;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001e2344);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    Callee {
                        addr: 0x001e2fe0,
                        f: ee::s04::MessageCenter_PostMessage,
                    }
                    .call(c, 0x001e2344)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1e235c;
                    } else {
                        break 'b_1e234c;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2fc));
                c.g[RA][0] = sx(0x001e2358);
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x001e2fe0,
                    f: ee::s04::MessageCenter_PostMessage,
                }
                .call(c, 0x001e2358)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1e235c;
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `MessageCenter_PostBCFinishMessageA` (0x001e2648, `func_001E2648(which, flag)`):
/// unless the replay is busy, `which` 0xe, 0xf or 0x10 (groups 22, 23, 24) does
/// nothing if the group's flag is set; otherwise it clears the group's messages,
/// sets the flag when `flag` is 0, and posts the group's message (which one
/// depends on the human rider's character).
pub fn message_center_post_bc_finish_message_a(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e2680: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A0][0] = c.lw((c.g[A0][0] as u32));
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1e2680;
            } else {
                c.cycles += 1;
                c.g[A1][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
                break 'b_1e2680;
            }
        }
        'b_1e2814: {
            c.cycles += 2;
            let t = c.g[A1][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_1e2814;
            } else {
                'b_1e2804: {
                    'b_1e27f4: {
                        'b_1e27a4: {
                            'b_1e274c: {
                                c.cycles += 16;
                                c.g[RA][0] = sx(0x001e2690);
                                call_ptr(c, 0x0014dc80, 0x001e2690)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001e269c);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, 0x0014dd58, 0x001e269c)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x001e26ac);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00146e98, 0x001e26ac)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001e26b8);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x00147398, 0x001e26b8)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[V0][0] = 0xf;
                                let t = c.g[S1][0] == c.g[V0][0];
                                c.g[V0][0] = ((c.g[S1][0] as i64) < 16i64) as u64;
                                if t {
                                    break 'b_1e274c;
                                } else {
                                    'b_1e26f4: {
                                        'b_1e26e0: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0xe;
                                            if t {
                                                break 'b_1e26e0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S1][0] == c.g[V0][0];

                                                if t {
                                                    break 'b_1e26f4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1e2814;
                                                    } else {
                                                        break 'b_1e26e0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x10;
                                        let t = c.g[S1][0] == c.g[V0][0];

                                        if t {
                                            break 'b_1e27a4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_1e2814;
                                            } else {
                                                break 'b_1e26f4;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001e26fc);
                                    c.g[A0][0] = 0x16;
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::MessageCenter_IsFlagSet,
                                    }
                                    .call(c, 0x001e26fc)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        break 'b_1e2814;
                                    } else {
                                        'b_1e273c: {
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x001e270c);
                                            c.g[A0][0] = 0x1;
                                            Callee {
                                                addr: 0x001e32c8,
                                                f: ee::s04::MessageCenter_RemoveMessagesOfGroup,
                                            }
                                            .call(c, 0x001e270c)?;
                                            c.g[RA][0] = sx(0x001e2714);
                                            c.g[A0][0] = 0x16;
                                            Callee {
                                                addr: 0x001e32c8,
                                                f: ee::s04::MessageCenter_RemoveMessagesOfGroup,
                                            }
                                            .call(c, 0x001e2714)?;
                                            let t = c.g[S2][0] != 0u64;
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1e273c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A0][0] = 0x16;
                                                c.g[RA][0] = sx(0x001e2728);
                                                c.g[A1][0] = 0x1;
                                                Callee {
                                                    addr: 0x001e3a78,
                                                    f: ee::s04::MessageCenter_SetFlag,
                                                }
                                                .call(c, 0x001e2728)?;
                                                c.g[V0][0] = 0x3;
                                                let t = c.g[S0][0] != c.g[V0][0];
                                                c.g[A0][0] = 0x74;
                                                if t {
                                                    break 'b_1e27f4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] = 0x76;
                                                    if t {
                                                        break 'b_1e27f4;
                                                    } else {
                                                        break 'b_1e273c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x73;
                                        if t {
                                            break 'b_1e27f4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = 0x75;
                                            if t {
                                                break 'b_1e27f4;
                                            } else {
                                                break 'b_1e274c;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001e2754);
                            c.g[A0][0] = 0x17;
                            Callee {
                                addr: 0x001e3a30,
                                f: ee::s04::MessageCenter_IsFlagSet,
                            }
                            .call(c, 0x001e2754)?;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_1e2814;
                            } else {
                                'b_1e2794: {
                                    c.cycles += 6;
                                    c.g[RA][0] = sx(0x001e2764);
                                    c.g[A0][0] = 0x2;
                                    Callee {
                                        addr: 0x001e32c8,
                                        f: ee::s04::MessageCenter_RemoveMessagesOfGroup,
                                    }
                                    .call(c, 0x001e2764)?;
                                    c.g[RA][0] = sx(0x001e276c);
                                    c.g[A0][0] = 0x17;
                                    Callee {
                                        addr: 0x001e32c8,
                                        f: ee::s04::MessageCenter_RemoveMessagesOfGroup,
                                    }
                                    .call(c, 0x001e276c)?;
                                    let t = c.g[S2][0] != 0u64;
                                    c.g[V0][0] = 0x7;
                                    if t {
                                        break 'b_1e2794;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A0][0] = 0x17;
                                        c.g[RA][0] = sx(0x001e2780);
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x001e3a78,
                                            f: ee::s04::MessageCenter_SetFlag,
                                        }
                                        .call(c, 0x001e2780)?;
                                        c.g[V0][0] = 0x7;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x78;
                                        if t {
                                            break 'b_1e27f4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = 0x7a;
                                            if t {
                                                break 'b_1e27f4;
                                            } else {
                                                break 'b_1e2794;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[A0][0] = 0x77;
                                if t {
                                    break 'b_1e27f4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = 0x79;
                                    if t {
                                        break 'b_1e27f4;
                                    } else {
                                        break 'b_1e27a4;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001e27ac);
                        c.g[A0][0] = 0x18;
                        Callee {
                            addr: 0x001e3a30,
                            f: ee::s04::MessageCenter_IsFlagSet,
                        }
                        .call(c, 0x001e27ac)?;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_1e2814;
                        } else {
                            'b_1e27ec: {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x001e27bc);
                                c.g[A0][0] = 0x3;
                                Callee {
                                    addr: 0x001e32c8,
                                    f: ee::s04::MessageCenter_RemoveMessagesOfGroup,
                                }
                                .call(c, 0x001e27bc)?;
                                c.g[RA][0] = sx(0x001e27c4);
                                c.g[A0][0] = 0x18;
                                Callee {
                                    addr: 0x001e32c8,
                                    f: ee::s04::MessageCenter_RemoveMessagesOfGroup,
                                }
                                .call(c, 0x001e27c4)?;
                                let t = c.g[S2][0] != 0u64;
                                c.g[V0][0] = 0x8;
                                if t {
                                    break 'b_1e27ec;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] = 0x18;
                                    c.g[RA][0] = sx(0x001e27d8);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001e3a78,
                                        f: ee::s04::MessageCenter_SetFlag,
                                    }
                                    .call(c, 0x001e27d8)?;
                                    c.g[V0][0] = 0x8;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[A0][0] = 0x7c;
                                    if t {
                                        break 'b_1e27f4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = 0x7e;
                                        if t {
                                            break 'b_1e27f4;
                                        } else {
                                            break 'b_1e27ec;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[S0][0] == c.g[V0][0];
                            c.g[A0][0] = 0x7b;
                            if t {
                                break 'b_1e2804;
                            } else {
                                break 'b_1e27f4;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e27fc);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x001e2fe0,
                        f: ee::s04::MessageCenter_PostMessage,
                    }
                    .call(c, 0x001e27fc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1e2814;
                    } else {
                        break 'b_1e2804;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = 0x7d;
                c.g[RA][0] = sx(0x001e2810);
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x001e2fe0,
                    f: ee::s04::MessageCenter_PostMessage,
                }
                .call(c, 0x001e2810)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1e2814;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{check, elf_image, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// The messenger object the constructors fill.
    const OC966_THIS: u32 = DATA + 0x100;

    /// The replay counter: the world (`gp-0x848`) at `DATA+0x1000`, its game
    /// object at `+0x84` (`DATA+0x2000`), whose `+0x28` points at the counter
    /// cell `DATA+0x5000`. Counter 0 or at least 10 is not busy.
    fn oc966_world(c: &mut Cpu, replay: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
        c.bus.write32(DATA + 0x1084, DATA + 0x2000);
        c.bus.write32(DATA + 0x2028, DATA + 0x5000);
        c.bus.write32(DATA + 0x5000, replay);
    }

    /// Busy and idle counters (busy is 1..9).
    fn oc966_replay(seed: u32) -> u32 {
        [0, 5, 12, 7][(seed % 4) as usize]
    }

    /// Gate word magic: a gated fake answers with its forced result only when
    /// its gate word (in the test machine's RAM) holds this; otherwise it runs
    /// the body it replaced. Each gate word has a forced result at +4.
    const OC966_MAGIC: u32 = 0x0966_1966;
    const OC966_GATE_BE_GET_BE: u32 = DATA + 0x21_0000;
    const OC966_GATE_GET_INTERFACE: u32 = DATA + 0x21_0100;
    const OC966_GATE_HUMAN_RIDER: u32 = DATA + 0x21_0200;
    const OC966_GATE_CHAR_ID: u32 = DATA + 0x21_0300;
    const OC966_GATE_IS_FLAG: u32 = DATA + 0x21_0400;
    const OC966_GATE_SET_FLAG: u32 = DATA + 0x21_0500;
    const OC966_GATE_POST: u32 = DATA + 0x21_0600;
    const OC966_GATE_PICK: u32 = DATA + 0x21_0700;
    const OC966_GATE_REMOVE: u32 = DATA + 0x21_0800;

    /// A gated fake: with its gate on, return the forced result in v0; with
    /// it off, run the body that was in the slot before (the chain of
    /// `fakes::hook`), so other tests see the real callee.
    macro_rules! oc966_fake {
        ($fake:ident, $gate:ident, $generated:path) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32($gate) != OC966_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                }
                let v = c.bus.read32($gate + 4);
                c.g[V0][0] = sx(v);
                ret(c);
            }
        };
    }

    oc966_fake!(
        oc966_fake_be_get_be,
        OC966_GATE_BE_GET_BE,
        ee::s01::f_0014dc80__generated
    );
    oc966_fake!(
        oc966_fake_get_interface,
        OC966_GATE_GET_INTERFACE,
        ee::s01::f_0014dd58__generated
    );
    oc966_fake!(
        oc966_fake_human_rider,
        OC966_GATE_HUMAN_RIDER,
        ee::s01::f_00146e98__generated
    );
    oc966_fake!(
        oc966_fake_char_id,
        OC966_GATE_CHAR_ID,
        ee::s01::f_00147398__generated
    );
    oc966_fake!(
        oc966_fake_is_flag,
        OC966_GATE_IS_FLAG,
        ee::s04::f_001e3a30__generated
    );
    oc966_fake!(
        oc966_fake_set_flag,
        OC966_GATE_SET_FLAG,
        ee::s04::f_001e3a78__generated
    );
    oc966_fake!(
        oc966_fake_post,
        OC966_GATE_POST,
        ee::s04::f_001e2fe0__generated
    );
    oc966_fake!(
        oc966_fake_pick,
        OC966_GATE_PICK,
        ee::s04::f_001e1ad0__generated
    );
    oc966_fake!(
        oc966_fake_remove,
        OC966_GATE_REMOVE,
        ee::s04::f_001e32c8__generated
    );

    /// Hooks one fake into its override slot (once per process).
    macro_rules! oc966_hook_one {
        ($slot:path, $fake:ident) => {
            fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake))
        };
    }

    /// Installs the fakes in the override slots.
    fn oc966_hook() {
        oc966_hook_one!(ee::s01::OVERRIDE_0014DC80, oc966_fake_be_get_be);
        oc966_hook_one!(ee::s01::OVERRIDE_0014DD58, oc966_fake_get_interface);
        oc966_hook_one!(ee::s01::OVERRIDE_00146E98, oc966_fake_human_rider);
        oc966_hook_one!(ee::s01::OVERRIDE_00147398, oc966_fake_char_id);
        oc966_hook_one!(ee::s04::OVERRIDE_001E3A30, oc966_fake_is_flag);
        oc966_hook_one!(ee::s04::OVERRIDE_001E3A78, oc966_fake_set_flag);
        oc966_hook_one!(ee::s04::OVERRIDE_001E2FE0, oc966_fake_post);
        oc966_hook_one!(ee::s04::OVERRIDE_001E1AD0, oc966_fake_pick);
        oc966_hook_one!(ee::s04::OVERRIDE_001E32C8, oc966_fake_remove);
    }

    /// Gate words for one seed: the BE getters return dummy objects, the rider
    /// character cycles through 0, 3, 7 and 8, flags set or not, the
    /// foreshadow pick is 0x100 (nothing to post) or 5.
    fn oc966_gates(c: &mut Cpu, seed: u32) {
        oc966_hook();
        let gate = |c: &mut Cpu, g: u32, v: u32| {
            c.bus.write32(g, OC966_MAGIC);
            c.bus.write32(g + 4, v);
        };
        gate(c, OC966_GATE_BE_GET_BE, DATA + 0x6000);
        gate(c, OC966_GATE_GET_INTERFACE, DATA + 0x6100);
        gate(c, OC966_GATE_HUMAN_RIDER, DATA + 0x6200);
        gate(c, OC966_GATE_CHAR_ID, [0, 3, 7, 8][(seed % 4) as usize]);
        gate(c, OC966_GATE_IS_FLAG, (seed / 2) % 2);
        gate(c, OC966_GATE_SET_FLAG, 0);
        gate(c, OC966_GATE_POST, 0);
        gate(c, OC966_GATE_PICK, if seed % 3 == 0 { 0x100 } else { 5 });
        gate(c, OC966_GATE_REMOVE, 0);
    }

    #[test]
    fn c_fe_state_net_messenger_ctor_2_matches() {
        check(
            ee::s04::f_001e0f60__generated,
            c_fe_state_net_messenger_ctor_2,
            96,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1e0f60, 0);
                c.bus.write32(
                    GAME_GP.wrapping_add(0xda0),
                    if seed % 2 == 0 { 0 } else { DATA + 0x7000 },
                );
                c.g[A0][0] = sx(OC966_THIS);
            },
        );
    }

    #[test]
    fn c_fe_state_read_message_popup_ctor_2_matches() {
        check(
            ee::s04::f_001e0ff0__generated,
            c_fe_state_read_message_popup_ctor_2,
            96,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1e0ff0, 0);
                c.bus.write32(
                    GAME_GP.wrapping_add(0xda0),
                    if seed % 2 == 0 { 0 } else { DATA + 0x7000 },
                );
                c.g[A0][0] = sx(OC966_THIS);
            },
        );
    }

    #[test]
    fn front_end_global_ctors_matches() {
        check(
            ee::s04::f_001e12b0__generated,
            front_end_global_ctors,
            16,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1e12b0, 0);
                elf_image(c);
            },
        );
    }

    #[test]
    fn front_end_global_dtors_matches() {
        check(
            ee::s04::f_001e12d0__generated,
            front_end_global_dtors,
            16,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1e12d0, 0);
                elf_image(c);
            },
        );
    }

    #[test]
    fn message_center_post_foreshadow_matches() {
        check(
            ee::s04::f_001e1c10__generated,
            message_center_post_foreshadow,
            96,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1e1c10, 0);
                elf_image(c);
                oc966_world(c, oc966_replay(seed));
                oc966_gates(c, seed);
                c.g[A0][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn message_center_post_peak_message_b_matches() {
        check(
            ee::s04::f_001e20d0__generated,
            message_center_post_peak_message_b,
            96,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1e20d0, 0);
                elf_image(c);
                oc966_world(c, oc966_replay(seed));
                oc966_gates(c, seed);
                c.g[A0][0] = (seed % 3) as u64;
                c.g[A1][0] = ((seed / 3) % 2) as u64;
            },
        );
    }

    #[test]
    fn message_center_post_peak_message_a_matches() {
        check(
            ee::s04::f_001e2220__generated,
            message_center_post_peak_message_a,
            96,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1e2220, 0);
                elf_image(c);
                oc966_world(c, oc966_replay(seed));
                oc966_gates(c, seed);
                c.g[A0][0] = (seed % 3) as u64;
                c.g[A1][0] = ((seed / 3) % 2) as u64;
            },
        );
    }

    #[test]
    fn message_center_post_bc_finish_message_a_matches() {
        check(
            ee::s04::f_001e2648__generated,
            message_center_post_bc_finish_message_a,
            96,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x1e2648, 0);
                elf_image(c);
                oc966_world(c, oc966_replay(seed));
                oc966_gates(c, seed);
                c.g[A0][0] = 0xe + (seed % 4) as u64;
                c.g[A1][0] = ((seed / 4) % 2) as u64;
            },
        );
    }
}
