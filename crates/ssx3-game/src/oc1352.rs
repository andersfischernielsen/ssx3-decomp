//! Staging module oc1352 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1352`.

#![allow(unused_imports)]

use crate::abi::{arg, r32, ret};
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
        0x0025_8cb0,
        c_game_comm_update_wait_popup,
        "cGameComm_UpdateWaitPopup",
    ),
    (
        0x0025_8f20,
        c_game_comm_send_sync_check,
        "cGameComm_SendSyncCheck",
    ),
    (
        0x0025_9098,
        lobby_man_setup_net_race,
        "LobbyMan_SetupNetRace",
    ),
    (0x0025_91e8, lobby_mem_alloc, "Lobby_MemAlloc"),
    (0x0025_9210, lobby_mem_free, "Lobby_MemFree"),
    (
        0x0025_9230,
        lobby_man_forward_event_to_ui,
        "LobbyMan_ForwardEventToUI",
    ),
    (
        0x0025_9390,
        lobby_man_null_lobby_callback,
        "LobbyMan_NullLobbyCallback",
    ),
    (
        0x0026_8890,
        lobby_man_init_string_fields,
        "LobbyMan_InitStringFields",
    ),
    (0x0026_89b8, buddy_man_dtor, "BuddyMan_dtor"),
    (
        0x0026_8e50,
        lobby_man_on_user_stats_reply_thunk,
        "LobbyMan_OnUserStatsReply_Thunk",
    ),
    (0x0026_9da0, net_event_fd_dtor, "NetEventFD_dtor"),
    (0x0026_9e20, net_event_fe_dtor, "NetEventFE_dtor"),
    (0x0026_9ea0, lobby_man_global_ctors, "LobbyMan_GlobalCtors"),
    (
        0x003e_74e8,
        proto_ping_rpc_reply_callback,
        "ProtoPing_RpcReplyCallback",
    ),
    (
        0x003e_eab8,
        lobby_hash_str_cmp_no_case,
        "Lobby_HashStrCmpNoCase",
    ),
    (0x003f_4410, dirty_rpc_intc_handler, "DirtyRpc_IntcHandler"),
    (
        0x003f_5a50,
        proto_aries_rpc_reply_callback,
        "ProtoAries_RpcReplyCallback",
    ),
    (0x003f_77f0, s_login_step_nop, "SLogin_StepNop"),
    (
        0x003f_87f8,
        s_login_mc_sync_result_cmd00,
        "SLoginMc_SyncResult_Cmd00",
    ),
    (
        0x003f_8800,
        s_login_mc_sync_result_cmd01,
        "SLoginMc_SyncResult_Cmd01",
    ),
];

/// Callees this module's ports call through (same targets as the generated
/// bodies: direct slot wrappers so the call logs match, `call_ptr` for the
/// virtual/method calls the originals make through pointers).
mod callee {
    use super::{ee, Callee};

    pub const STR_HASH: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::f_00317670,
    };
    pub const UI_STATE_MGR_FIND_STATE: Callee = Callee {
        addr: 0x0039_f9d8,
        f: ee::s13::f_0039f9d8,
    };
    pub const USTR_COPY: Callee = Callee {
        addr: 0x002c_2508,
        f: ee::s08::f_002c2508,
    };
    pub const C_GAME_COMM_SEND_PACKET: Callee = Callee {
        addr: 0x0025_7e98,
        f: ee::s06::f_00257e98,
    };
    pub const BE_GET_BE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    pub const BE_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    pub const BE_NEW_PLAYER_GET_CHAR: Callee = Callee {
        addr: 0x0014_7488,
        f: ee::s01::f_00147488,
    };
    pub const BE_SET_NET_STRING_A: Callee = Callee {
        addr: 0x0014_ebb8,
        f: ee::s01::f_0014ebb8,
    };
    pub const BE_SET_NET_STRING_B: Callee = Callee {
        addr: 0x0014_ebd8,
        f: ee::s01::f_0014ebd8,
    };
    pub const NET_START_GAME_SETUP: Callee = Callee {
        addr: 0x0026_39e0,
        f: ee::s07::f_002639e0,
    };
    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };
    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };
    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };
    pub const CBXSTRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::f_00318260,
    };
    pub const UI_LIST_FIRST_WRAPPER: Callee = Callee {
        addr: 0x0039_fae8,
        f: ee::s13::f_0039fae8,
    };
    pub const NET_EVENT_DTOR: Callee = Callee {
        addr: 0x0026_7e18,
        f: ee::s07::f_00267e18,
    };
    pub const LOBBY_MAN_STATIC_INIT: Callee = Callee {
        addr: 0x0026_7468,
        f: ee::s07::f_00267468,
    };
    pub const LOBBY_MAN_ON_USER_STATS_REPLY: Callee = Callee {
        addr: 0x0025_eb00,
        f: ee::s06::f_0025eb00,
    };
}

// ---------------------------------------------------------------------------
// `cGameComm_UpdateWaitPopup` (0x258cb0) constants.

/// Stack frame size of `cGameComm_UpdateWaitPopup`.
pub(crate) const WAIT_POPUP_FRAME: u32 = 0x150;
/// Frame slot holding the caller's `s3` (`self`).
pub(crate) const WAIT_POPUP_S3: u32 = 0x110;
/// Frame slot holding the caller's `s0` (UI manager + 0x18).
pub(crate) const WAIT_POPUP_S0: u32 = 0x140;
/// Frame slot holding the caller's `s1` (string object).
pub(crate) const WAIT_POPUP_S1: u32 = 0x130;
/// Frame slot holding the caller's `s2` (UI state).
pub(crate) const WAIT_POPUP_S2: u32 = 0x120;
/// Frame slot holding the return address.
pub(crate) const WAIT_POPUP_RA: u32 = 0x100;
/// `cGameComm` field: nonzero once the session has closed (clears +0x60).
pub(crate) const WAIT_POPUP_SHOW_WAIT: u32 = 0x84;
/// `cGameComm` field: cleared when the popup closes.
pub(crate) const WAIT_POPUP_RESULT: u32 = 0x60;
/// `cGameComm` field: local pause flag selecting the message.
pub(crate) const WAIT_POPUP_LOCAL_PAUSED: u32 = 0x78;
/// `cGameComm` field: popup mode (0 hidden, 1 waiting, 2 counting down).
pub(crate) const WAIT_POPUP_MODE: u32 = 0x98;
/// `cGameComm` field: peer pause flag.
pub(crate) const WAIT_POPUP_PEER_PAUSED: u32 = 0x74;
/// `cGameComm` field: resume grace left.
pub(crate) const WAIT_POPUP_RESUME_GRACE: u32 = 0x3fc;

/// `void cGameComm_UpdateWaitPopup(cGameComm *self)` (0x258cb0): refresh the
/// net lobby's wait popup. When the session closed, clear the result and
/// hide the popup; otherwise look up the wait UI state and show, update or
/// hide the popup from the pause flags and the popup mode.
pub fn c_game_comm_update_wait_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, WAIT_POPUP_FRAME);
        frame.save(c, S3, WAIT_POPUP_S3);
        frame.save(c, S0, WAIT_POPUP_S0);
        frame.save(c, S1, WAIT_POPUP_S1);
        frame.save(c, S2, WAIT_POPUP_S2);
        frame.save_ra(c, WAIT_POPUP_RA);
        c.g[S3][0] = c.g[A0][0];
        let comm = Addr(r32(c, S3));
        c.g[V0][0] = comm.off(WAIT_POPUP_SHOW_WAIT).lw(c);
        if c.g[V0][0] != 0 {
            comm.off(WAIT_POPUP_RESULT).sw(c, 0);
        }
        'b_258f04: {
            'b_258f00: {
                c.cycles += 5;
                let app = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = app;
                c.g[V1][0] = Addr(app as u32).off(0x84).lw(c);
                c.g[S0][0] = Addr(c.g[V1][0] as u32).off(0x48).lw(c);
                let no_ui = c.g[S0][0] == 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                if no_ui {
                    break 'b_258f00;
                }
                c.cycles += 9;
                c.g[A0][0] = sx(0x004a_2ef0);
                callee::STR_HASH.call(c, 0x0025_8cf8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                callee::UI_STATE_MGR_FIND_STATE.call(c, 0x0025_8d04)?;
                c.g[S2][0] = c.g[V0][0];
                if c.g[S2][0] == 0u64 {
                    frame.restore(c, S0, WAIT_POPUP_S0);
                    break 'b_258f04;
                }
                'b_258d98: {
                    'b_258d5c: {
                        c.cycles += 3;
                        c.g[V0][0] = comm.off(WAIT_POPUP_LOCAL_PAUSED).lw(c);
                        let waiting = c.g[V0][0] == 0u64;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                        if waiting {
                            break 'b_258d5c;
                        }
                        c.cycles += 16;
                        c.g[A0][0] = sx(0x0048_05d8);
                        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
                        c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        callee::STR_HASH.call(c, 0x0025_8d38)?;
                        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                        c.g[A0][0] = c.g[S0][0];
                        let resolve = c.g[V1][0] as u32;
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, resolve, 0x0025_8d48)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = sx(sp(c));
                        callee::USTR_COPY.call(c, 0x0025_8d54)?;
                        c.g[V1][0] = comm.off(WAIT_POPUP_MODE).lw(c);
                        break 'b_258d98;
                    }
                    c.cycles += 15;
                    c.g[A0][0] = sx(0x0048_05f8);
                    c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    callee::STR_HASH.call(c, 0x0025_8d78)?;
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    let resolve = c.g[V1][0] as u32;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, resolve, 0x0025_8d88)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx(sp(c));
                    callee::USTR_COPY.call(c, 0x0025_8d94)?;
                    c.g[V1][0] = comm.off(WAIT_POPUP_MODE).lw(c);
                }
                'b_258ee4: {
                    'b_258e9c: {
                        c.cycles += 3;
                        c.g[S0][0] = 0x1;
                        let is_waiting = c.g[V1][0] == c.g[S0][0];
                        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                        if is_waiting {
                            break 'b_258e9c;
                        }
                        'b_258e80: {
                            'b_258e60: {
                                'b_258e48: {
                                    'b_258dcc: {
                                        'b_258dbc: {
                                            c.cycles += 2;
                                            let at_least_two = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x2;
                                            if at_least_two {
                                                break 'b_258dbc;
                                            }
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0 {
                                                c.g[V0][0] = comm.off(WAIT_POPUP_PEER_PAUSED).lw(c);
                                                break 'b_258dcc;
                                            }
                                            c.cycles += 2;
                                            c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                            break 'b_258ee4;
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.g[V0][0] = comm.off(WAIT_POPUP_PEER_PAUSED).lw(c);
                                            break 'b_258e48;
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                        break 'b_258ee4;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] != 0u64 {
                                        frame.restore(c, S0, WAIT_POPUP_S0);
                                        break 'b_258f04;
                                    }
                                    'b_258e18: {
                                        c.cycles += 3;
                                        c.g[V0][0] = comm.off(WAIT_POPUP_LOCAL_PAUSED).lw(c);
                                        let waiting = c.g[V0][0] == 0u64;
                                        c.g[V0][0] =
                                            Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                                        if waiting {
                                            break 'b_258e18;
                                        }
                                        c.cycles += 7;
                                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
                                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
                                        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                            break 'b_258e60;
                                        }
                                        c.cycles += 5;
                                        c.g[V1][0] =
                                            Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
                                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x550).lw(c);
                                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                                        if c.g[V0][0] == 0u64 {
                                            frame.restore(c, S0, WAIT_POPUP_S0);
                                            break 'b_258f04;
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                        break 'b_258e60;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = comm.off(WAIT_POPUP_SHOW_WAIT).lw(c);
                                    let hidden = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0xe;
                                    if hidden {
                                        break 'b_258f00;
                                    }
                                    c.cycles += 9;
                                    c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                    c.g[A2][0] = sx(sp(c));
                                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc0).lh(c);
                                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc4).lw(c);
                                    let show = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, show, 0x0025_8e3c)?;
                                    c.g[V1][0] = 0x2;
                                    comm.off(WAIT_POPUP_MODE).sw(c, c.g[V1][0]);
                                    break 'b_258f00;
                                }
                                c.cycles += 2;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                    break 'b_258ee4;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = comm.off(WAIT_POPUP_LOCAL_PAUSED).lw(c);
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = comm.off(WAIT_POPUP_SHOW_WAIT).lw(c);
                                    break 'b_258e80;
                                }
                                c.cycles += 1;
                                c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                                break 'b_258e60;
                            }
                            c.cycles += 8;
                            c.g[A1][0] = 0xe;
                            c.g[A2][0] = sx(sp(c));
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc0).lh(c);
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc4).lw(c);
                            let show = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, show, 0x0025_8e78)?;
                            comm.off(WAIT_POPUP_MODE).sw(c, c.g[S0][0]);
                            break 'b_258f00;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                            break 'b_258ee4;
                        }
                        // Resume grace, not the peer flag: an expired (signed <= 0)
                        // grace hides the popup through the tail below.
                        c.cycles += 3;
                        c.g[V0][0] = comm.off(WAIT_POPUP_RESUME_GRACE).lw(c);
                        let expired = (c.g[V0][0] as i64) <= 0;
                        frame.restore(c, S0, WAIT_POPUP_S0);
                        if expired {
                            break 'b_258f04;
                        }
                        c.cycles += 2;
                        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                        break 'b_258ee4;
                    }
                    'b_258ecc: {
                        c.cycles += 6;
                        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                        c.g[A1][0] = 0x2;
                        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x214).lw(c);
                        if c.g[V1][0] != c.g[A1][0] {
                            c.g[V0][0] = comm.off(WAIT_POPUP_LOCAL_PAUSED).lw(c);
                            break 'b_258ecc;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x84c)).lw(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x550).lw(c);
                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                            break 'b_258ee4;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = comm.off(WAIT_POPUP_LOCAL_PAUSED).lw(c);
                        break 'b_258ecc;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                        break 'b_258ee4;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = comm.off(WAIT_POPUP_PEER_PAUSED).lw(c);
                    let peer_quiet = c.g[V0][0] == 0u64;
                    frame.restore(c, S0, WAIT_POPUP_S0);
                    if peer_quiet {
                        break 'b_258f04;
                    }
                    c.cycles += 1;
                    c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
                    break 'b_258ee4;
                }
                c.cycles += 7;
                c.g[A1][0] = 0xf;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc4).lw(c);
                let hide = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, hide, 0x0025_8efc)?;
                comm.off(WAIT_POPUP_MODE).sw(c, 0u64);
                break 'b_258f00;
            }
            c.cycles += 1;
            frame.restore(c, S0, WAIT_POPUP_S0);
            break 'b_258f04;
        }
        c.cycles += 6;
        frame.restore(c, S1, WAIT_POPUP_S1);
        frame.restore(c, S2, WAIT_POPUP_S2);
        frame.restore(c, S3, WAIT_POPUP_S3);
        frame.restore_ra(c, WAIT_POPUP_RA);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cGameComm_SendSyncCheck` (0x258f20) constants.

/// Stack frame size of `cGameComm_SendSyncCheck`.
pub(crate) const SYNC_CHECK_FRAME: u32 = 0x40;
/// Frame slot holding the caller's `s0` (`self`).
pub(crate) const SYNC_CHECK_S0: u32 = 0x30;
/// Frame slot holding the caller's `s1` (ring base).
pub(crate) const SYNC_CHECK_S1: u32 = 0x20;
/// Frame slot holding the return address.
pub(crate) const SYNC_CHECK_RA: u32 = 0x10;
/// `cGameComm` field: nonzero once the session has closed.
pub(crate) const SYNC_CHECK_READY: u32 = 0x40;
/// `cGameComm` field: nonzero while this player has paused.
pub(crate) const SYNC_CHECK_LOCAL_PAUSED: u32 = 0x4;
/// `cGameComm` field: set when the rings disagree.
pub(crate) const SYNC_CHECK_DESYNCED: u32 = 0xc;
/// `cGameComm` field: sync packet already sent.
pub(crate) const SYNC_CHECK_SENT: u32 = 0x400;
/// `cGameComm` field: sync reply received.
pub(crate) const SYNC_CHECK_REPLIED: u32 = 0x404;
/// `cGameComm` field: last frame checked.
pub(crate) const SYNC_CHECK_FRAME_NO: u32 = 0x408;
/// `cGameComm` field: earliest frame either side has seen.
pub(crate) const SYNC_CHECK_MIN_FRAME: u32 = 0x40c;
/// `cGameComm` field: ring of recent check values, 60 entries.
pub(crate) const SYNC_CHECK_RING: u32 = 0x410;
/// `cGameComm` field: the peer's ring of check values, 60 entries.
pub(crate) const SYNC_CHECK_PEER_RING: u32 = 0x500;
/// Ring length: frames are reduced modulo 60 twice.
pub(crate) const SYNC_CHECK_RING_LEN: u32 = 0x3c;

/// `void cGameComm_SendSyncCheck(cGameComm *self, int frame, int value)`
/// (0x258f20): record this frame's check `value` in the sync ring and, the
/// first time, send it to the peer; then compare the earliest frame both
/// sides have reported and flag desync when the rings disagree. Only runs
/// while the lobby is in game (states 3 and 5) and the session is live.
pub fn c_game_comm_send_sync_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, SYNC_CHECK_FRAME);
        frame.save(c, S0, SYNC_CHECK_S0);
        frame.save(c, S1, SYNC_CHECK_S1);
        frame.save_ra(c, SYNC_CHECK_RA);
        c.g[A3][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        'b_259084: {
            'b_258f68: {
                let app = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                c.g[V0][0] = app;
                c.g[V1][0] = 0x5;
                c.g[A1][0] = Addr(app as u32).off(0x84).lw(c);
                let mgr = c.g[A1][0];
                c.g[V0][0] = Addr(mgr as u32).off(0xc).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = 0x3;
                if c.g[A0][0] == c.g[V1][0] {
                    break 'b_258f68;
                }
                c.cycles += 2;
                if c.g[A0][0] != c.g[V0][0] {
                    frame.restore(c, S0, SYNC_CHECK_S0);
                    break 'b_259084;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(mgr as u32).off(0x214).lw(c);
                if c.g[V0][0] != c.g[A0][0] {
                    frame.restore(c, S0, SYNC_CHECK_S0);
                    break 'b_259084;
                }
            }
            let comm = Addr(r32(c, S0));
            c.cycles += 3;
            c.g[V0][0] = comm.off(SYNC_CHECK_READY).lw(c);
            if c.g[V0][0] == 0u64 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            c.cycles += 3;
            c.g[V0][0] = comm.off(SYNC_CHECK_LOCAL_PAUSED).lw(c);
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            c.cycles += 3;
            c.g[V0][0] = comm.off(0x0).lw(c);
            if c.g[V0][0] == 0u64 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            c.cycles += 3;
            c.g[V0][0] = comm.off(SYNC_CHECK_DESYNCED).lw(c);
            if c.g[V0][0] != 0u64 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            // Ring slot ((frame / 60) % 60): two divides, quotient then
            // remainder, through LO/HI exactly as the original. The first
            // divide sits in its own +5 block; both generated arms break
            // out of it, so only the break is kept.
            'b_258fac: {
                c.cycles += 5;
                c.g[A0][0] = SYNC_CHECK_RING_LEN as u64;
                c.g[V1][0] = SYNC_CHECK_RING_LEN as u64;
                c.div(0, A3, A0);
                break 'b_258fac;
            }
            'b_259020: {
                'b_25901c: {
                    'b_258ff4: {
                        c.cycles += 12;
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(SYNC_CHECK_RING));
                        c.g[S1][0] = c.g[A1][0];
                        c.g[V1][0] = c.lo[0];
                        c.div(0, V1, A0);
                        c.g[V0][0] = c.hi[0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        let slot = Addr(c.g[A1][0] as u32);
                        slot.sw(c, c.g[A2][0]);
                        comm.off(SYNC_CHECK_FRAME_NO).sw(c, c.g[A3][0]);
                        c.g[V0][0] = comm.off(SYNC_CHECK_SENT).lw(c);
                        let need_send = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x4;
                        if need_send {
                            break 'b_258ff4;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = comm.off(SYNC_CHECK_REPLIED).lw(c);
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = comm.off(SYNC_CHECK_FRAME_NO).lw(c);
                            break 'b_259020;
                        }
                        c.cycles += 3;
                        comm.off(SYNC_CHECK_REPLIED).sw(c, 0u64);
                        comm.off(SYNC_CHECK_SENT).sw(c, 0u64);
                        break 'b_25901c;
                    }
                    c.cycles += 10;
                    Addr(sp(c).wrapping_add(0x4)).sw(c, c.g[A3][0]);
                    Addr(sp(c).wrapping_add(0x8)).sw(c, c.g[A2][0]);
                    c.g[A0][0] = c.g[S0][0];
                    Addr(sp(c)).sb(c, c.g[V0][0]);
                    c.g[A1][0] = 0x3;
                    c.g[A2][0] = sx(sp(c));
                    c.g[A3][0] = 0xc;
                    callee::C_GAME_COMM_SEND_PACKET.call(c, 0x0025_9014)?;
                    c.g[V1][0] = 0x1;
                    comm.off(SYNC_CHECK_SENT).sw(c, c.g[V1][0]);
                }
                c.cycles += 1;
                c.g[A0][0] = comm.off(SYNC_CHECK_FRAME_NO).lw(c);
            }
            c.cycles += 2;
            if (c.g[A0][0] as i64) < 0 {
                frame.restore(c, S0, SYNC_CHECK_S0);
                break 'b_259084;
            }
            'b_259080: {
                c.cycles += 3;
                c.g[V1][0] = comm.off(SYNC_CHECK_MIN_FRAME).lw(c);
                let min_pending = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if min_pending {
                    break 'b_259080;
                }
                'b_259044: {
                    c.cycles += 4;
                    c.g[A1][0] = SYNC_CHECK_RING_LEN as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[A0][0];
                    }
                    break 'b_259044;
                }
                c.cycles += 14;
                c.div(0, V1, A1);
                c.g[V0][0] = SYNC_CHECK_RING_LEN as u64;
                c.g[V1][0] = SYNC_CHECK_RING_LEN as u64;
                c.g[A0][0] = SYNC_CHECK_RING_LEN as u64;
                c.g[V1][0] = c.lo[0];
                c.div(0, V1, A1);
                c.g[V0][0] = c.hi[0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(SYNC_CHECK_PEER_RING).lw(c);
                let same = c.g[A1][0] == c.g[V1][0];
                c.g[V0][0] = 0x1;
                if same {
                    break 'b_259080;
                }
                c.cycles += 1;
                comm.off(SYNC_CHECK_DESYNCED).sw(c, c.g[V0][0]);
                break 'b_259080;
            }
            c.cycles += 1;
            frame.restore(c, S0, SYNC_CHECK_S0);
        }
        c.cycles += 4;
        frame.restore(c, S1, SYNC_CHECK_S1);
        frame.restore_ra(c, SYNC_CHECK_RA);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `LobbyMan_SetupNetRace` (0x259098) constants.

/// Stack frame size of `LobbyMan_SetupNetRace`.
pub(crate) const SETUP_NET_RACE_FRAME: u32 = 0x80;
/// Frame slots holding the caller's `s0`..`s6`.
pub(crate) const SETUP_NET_RACE_S0: u32 = 0x70;
pub(crate) const SETUP_NET_RACE_S1: u32 = 0x60;
pub(crate) const SETUP_NET_RACE_S2: u32 = 0x50;
pub(crate) const SETUP_NET_RACE_S3: u32 = 0x40;
pub(crate) const SETUP_NET_RACE_S4: u32 = 0x30;
pub(crate) const SETUP_NET_RACE_S5: u32 = 0x20;
pub(crate) const SETUP_NET_RACE_S6: u32 = 0x10;
/// Frame slot holding the return address.
pub(crate) const SETUP_NET_RACE_RA: u32 = 0x0;
/// `D_004A3028` field: nonzero while a net race is being set up.
pub(crate) const SETUP_NET_RACE_ACTIVE: u32 = 0xb4;
/// `D_004A3028` field: pending race id, used while active.
pub(crate) const SETUP_NET_RACE_PENDING: u32 = 0xac;
/// `D_004A3028` field: current race id, used once active.
pub(crate) const SETUP_NET_RACE_CURRENT: u32 = 0xa8;

/// `void LobbyMan_SetupNetRace(void *a0, int a1, int a2)` (0x259098): start
/// a network race. Resolves the player and race interfaces, records the
/// local player's character, publishes the pending race strings and hands
/// the lobby's arguments to the net game setup.
pub fn lobby_man_setup_net_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, SETUP_NET_RACE_FRAME);
        frame.save(c, S2, SETUP_NET_RACE_S2);
        frame.save(c, S4, SETUP_NET_RACE_S4);
        frame.save(c, S5, SETUP_NET_RACE_S5);
        frame.save(c, S6, SETUP_NET_RACE_S6);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, SETUP_NET_RACE_S0);
        frame.save(c, S1, SETUP_NET_RACE_S1);
        frame.save(c, S3, SETUP_NET_RACE_S3);
        frame.save_ra(c, SETUP_NET_RACE_RA);
        c.g[S5][0] = c.g[A1][0];
        let setup = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        c.g[S2][0] = setup;
        c.g[V0][0] = Addr(setup as u32).off(SETUP_NET_RACE_ACTIVE).lw(c);
        c.g[S6][0] = c.g[A2][0];
        let race = if c.g[V0][0] == 0u64 {
            c.cycles += 1;
            Addr(setup as u32).off(SETUP_NET_RACE_CURRENT).lw(c)
        } else {
            c.cycles += 2;
            Addr(setup as u32).off(SETUP_NET_RACE_PENDING).lw(c)
        };
        c.g[S3][0] = race;
        c.cycles += 45;
        callee::BE_GET_BE.call(c, 0x0025_90e8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        callee::BE_GET_INTERFACE.call(c, 0x0025_90f4)?;
        c.g[S1][0] = c.g[V0][0];
        let app = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = app;
        c.g[A1][0] = 0x7;
        c.g[A0][0] = Addr(app as u32).off(0x78).lw(c);
        callee::BE_GET_INTERFACE.call(c, 0x0025_9108)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0u64;
        callee::BE_NEW_PLAYER_GET_CHAR.call(c, 0x0025_9118)?;
        c.g[V1][0] = 0x530000;
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        Addr((c.g[V1][0] as u32).wrapping_add(0x4e58)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = Addr(c.g[A2][0] as u32).off(0x98).lw(c);
        callee::BE_SET_NET_STRING_A.call(c, 0x0025_9130)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x80).lw(c);
        callee::BE_SET_NET_STRING_B.call(c, 0x0025_9140)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let publish = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, publish, 0x0025_9154)?;
        c.g[A2][0] = Addr(c.g[S2][0] as u32).off(SETUP_NET_RACE_ACTIVE).lw(c);
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S5][0];
        c.g[T0][0] = c.g[S6][0];
        callee::NET_START_GAME_SETUP.call(c, 0x0025_916c)?;
        frame.restore(c, S0, SETUP_NET_RACE_S0);
        frame.restore(c, S1, SETUP_NET_RACE_S1);
        frame.restore(c, S2, SETUP_NET_RACE_S2);
        frame.restore(c, S3, SETUP_NET_RACE_S3);
        frame.restore(c, S4, SETUP_NET_RACE_S4);
        frame.restore(c, S5, SETUP_NET_RACE_S5);
        frame.restore(c, S6, SETUP_NET_RACE_S6);
        frame.restore_ra(c, SETUP_NET_RACE_RA);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small lobby allocators and event forwarding.

/// `void *Lobby_MemAlloc(unsigned int size)` (0x2591e8): allocate `size`
/// bytes from the lobby heap (`operator_new` with the lobby heap tag).
pub fn lobby_mem_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(0x004a_2f28);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        // A0 (size) passes through untouched.
        callee::OPERATOR_NEW_NAMED.call(c, 0x0025_9204)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Lobby_MemFree(void *ptr)` (0x259210): release a lobby-heap block,
/// ignoring null.
pub fn lobby_mem_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let live = c.g[A0][0] != 0u64;
        frame.save_ra(c, 0);
        if live {
            c.cycles += 2;
            callee::EA_MEM_FREE.call(c, 0x0025_9224)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_ForwardEventToUI(void *event)` (0x259230): hand a lobby
/// event to the lobby menu's UI object, through its virtual event handler.
pub fn lobby_man_forward_event_to_ui(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let app = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(app as u32).off(0x7c).lw(c);
        c.g[S0][0] = c.g[A0][0];
        if c.g[V0][0] != 0u64 {
            c.cycles += 10;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            callee::UI_LIST_FIRST_WRAPPER.call(c, 0x0025_9258)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0xc0).lh(c);
            c.g[A3][0] = Addr(c.g[V1][0] as u32).off(0xc4).lw(c);
            let on_event = c.g[A3][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, on_event, 0x0025_9274)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_NullLobbyCallback(void)` (0x259390): the lobby callback
/// installed when no handler is wanted; returns at once.
pub fn lobby_man_null_lobby_callback(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// `LobbyMan_InitStringFields` (0x268890) constants.

/// `D_004A...` table value copied into each string field.
pub(crate) const STRING_FIELDS_TABLE_OFF: u32 = 0xda0;
/// First string field offset in the object.
pub(crate) const STRING_FIELDS_FIRST: u32 = 0x14;

/// `void LobbyMan_InitStringFields(void *self)` (0x268890): point the
/// lobby's string fields at the shared string table: five fields, then four
/// more advancing one word at a time.
pub fn lobby_man_init_string_fields(c: &mut Cpu) {
    c.cycles += 10;
    let table = Addr((c.g[GP][0] as u32).wrapping_add(STRING_FIELDS_TABLE_OFF)).lw(c);
    c.g[V0][0] = table;
    let obj = Addr(r32(c, A0));
    obj.sw(c, table);
    obj.off(0x4).sw(c, table);
    obj.off(0x8).sw(c, table);
    obj.off(0xc).sw(c, table);
    obj.off(0x10).sw(c, table);
    c.g[V1][0] = sx(r32(c, A0).wrapping_add(STRING_FIELDS_FIRST));
    c.g[A1][0] = 0x3;
    c.g[A2][0] = table;
    c.g[A3][0] = 0xffffffffffffffff;
    'l_2688b8: loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != c.g[A3][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0026_88b8);
            }
            continue 'l_2688b8;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

// ---------------------------------------------------------------------------
// `BuddyMan_dtor` (0x2689b8) constants.

/// Frame slot holding the caller's `s0` (`self`).
pub(crate) const BUDDY_DTOR_S0: u32 = 0x20;
/// Frame slot holding the caller's `s1` (flags).
pub(crate) const BUDDY_DTOR_S1: u32 = 0x10;
/// Frame slot holding the return address.
pub(crate) const BUDDY_DTOR_RA: u32 = 0x0;
/// `BuddyMan` field: string block destroyed first.
pub(crate) const BUDDY_DTOR_STRINGS: u32 = 0xec;
/// `BuddyMan` field: start of the first pointer range.
pub(crate) const BUDDY_DTOR_RANGE_A_BEGIN: u32 = 0xbc;
/// `BuddyMan` field: end of the first pointer range.
pub(crate) const BUDDY_DTOR_RANGE_A_END: u32 = 0xc4;
/// `BuddyMan` field: fallback count for the first range.
pub(crate) const BUDDY_DTOR_COUNT_A: u32 = 0xac;
/// `BuddyMan` field: end of the second pointer range.
pub(crate) const BUDDY_DTOR_RANGE_B_END: u32 = 0xb4;

/// `void BuddyMan_dtor(BuddyMan *self, int flags)` (0x2689b8): destroy the
/// buddy manager's string block and its two pointer ranges (freeing the
/// range storage unless it holds exactly one entry, which is deleted
/// instead), then free `self` when `flags & 1`.
pub fn buddy_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, BUDDY_DTOR_S0);
        frame.save(c, S1, BUDDY_DTOR_S1);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(BUDDY_DTOR_STRINGS));
        frame.save_ra(c, BUDDY_DTOR_RA);
        c.g[A1][0] = 0x2;
        callee::CBXSTRING_DTOR.call(c, 0x0026_89dc)?;
        'b_268a1c: {
            'b_268a18: {
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(BUDDY_DTOR_RANGE_A_BEGIN).lw(c);
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(BUDDY_DTOR_RANGE_A_END).lw(c);
                c.g[V1][0] = c.g[A0][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                let empty = c.g[A0][0] == 0u64;
                c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                if empty {
                    break 'b_268a18;
                }
                'b_268a10: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[V1][0] != c.g[V0][0] {
                        break 'b_268a10;
                    }
                    c.cycles += 4;
                    callee::BUILTIN_DELETE.call(c, 0x0026_8a08)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(BUDDY_DTOR_COUNT_A).lw(c);
                    break 'b_268a1c;
                }
                c.cycles += 2;
                callee::EA_MEM_FREE.call(c, 0x0026_8a18)?;
            }
            c.cycles += 1;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(BUDDY_DTOR_COUNT_A).lw(c);
        }
        'b_268a58: {
            'b_268a54: {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(BUDDY_DTOR_RANGE_B_END).lw(c);
                c.g[A0][0] = c.g[V1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                let empty = c.g[A0][0] == 0u64;
                c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                if empty {
                    break 'b_268a54;
                }
                'b_268a4c: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    if c.g[V1][0] != c.g[V0][0] {
                        break 'b_268a4c;
                    }
                    c.cycles += 4;
                    callee::BUILTIN_DELETE.call(c, 0x0026_8a44)?;
                    c.g[V0][0] = c.g[S1][0] & 0x1;
                    break 'b_268a58;
                }
                c.cycles += 2;
                callee::EA_MEM_FREE.call(c, 0x0026_8a54)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0] & 0x1;
        }
        // Tail block charge runs on both arms; the `+3` only on the live one.
        c.cycles += 2;
        if c.g[V0][0] == 0u64 {
            frame.restore(c, S0, BUDDY_DTOR_S0);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::BUILTIN_DELETE.call(c, 0x0026_8a68)?;
            frame.restore(c, S0, BUDDY_DTOR_S0);
        }
        c.cycles += 4;
        frame.restore(c, S1, BUDDY_DTOR_S1);
        frame.restore_ra(c, BUDDY_DTOR_RA);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnUserStatsReply_Thunk(void *a0, void *a1, int shift)`
/// (0x268e50): forward a user-stats reply to the lobby handler, adding the
/// lobby's per-reply id offset to the callback argument.
pub fn lobby_man_on_user_stats_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x4a0000;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x31d0)).lh(c);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        callee::LOBBY_MAN_ON_USER_STATS_REPLY.call(c, 0x0026_8e68)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetEventFD_dtor(NetEventFD *self)` (0x269da0): destroy a friend-list
/// net event through the base `NetEvent` destructor.
pub fn net_event_fd_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        // A0 (`self`) passes through untouched.
        callee::NET_EVENT_DTOR.call(c, 0x0026_9db0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void NetEventFE_dtor(NetEventFE *self)` (0x269e20): destroy a
/// friend-event net event through the base `NetEvent` destructor.
pub fn net_event_fe_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        // A0 (`self`) passes through untouched.
        callee::NET_EVENT_DTOR.call(c, 0x0026_9e30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_GlobalCtors(void)` (0x269ea0): run the lobby manager's
/// static initializer.
pub fn lobby_man_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        callee::LOBBY_MAN_STATIC_INIT.call(c, 0x0026_9eb4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small RPC reply callbacks and login steps.

/// `void ProtoPing_RpcReplyCallback(void)` (0x3e74e8): stash the ping
/// reply's `t3` context into the reply slot `*(sp)`, clearing its status
/// word.
pub fn proto_ping_rpc_reply_callback(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = Addr(sp(c)).lw(c);
    c.g[V0][0] = 0u64;
    Addr(c.g[V1][0] as u32).sw(c, c.g[T3][0]);
    Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, 0u64);
    ret(c);
}

// ---------------------------------------------------------------------------
// `Lobby_HashStrCmpNoCase` (0x3eeab8) constants.

/// Case-fold table: `fold[c]` is 1 for skipped characters, otherwise the
/// comparison key (0 only for the NUL terminator).
#[cfg(test)]
pub(crate) const HASH_CMP_FOLD_TABLE: u32 = 0x0049_6a58;

/// `int Lobby_HashStrCmpNoCase(const char *a, const char *b)` (0x3eeab8):
/// compare two lobby strings case-insensitively through the fold table,
/// skipping characters that fold to 1; returns the first differing fold
/// difference, or 0 when both terminate together.
pub fn lobby_hash_str_cmp_no_case(c: &mut Cpu) {
    c.g[V0][0] = 0x490000;
    c.g[T0][0] = 0x1;
    c.g[T2][0] = c.g[V0][0];
    c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6a58));
    c.cycles += 4;
    'l_3eeac8: loop {
        c.cycles += 1;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
        'l_3eeacc: loop {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
            let end_a = c.g[V1][0] == c.g[T0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            if end_a {
                if event_due(c) {
                    deliver_event_here(c, 0x003e_eac8);
                }
                continue 'l_3eeac8;
            }
            c.cycles += 3;
            c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(0x6a58));
            c.g[A2][0] = 0x1;
            'l_3eeaf0: loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lbu(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lbu(c);
                let end_b = c.g[V0][0] == c.g[A2][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                if end_b {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_eaf0);
                    }
                    continue 'l_3eeaf0;
                }
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                if c.g[V0][0] != 0u64 {
                    c.cycles += 2;
                    ret(c);
                    return;
                }
                c.cycles += 2;
                if c.g[V1][0] != 0u64 {
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_eacc);
                    }
                    continue 'l_3eeacc;
                }
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                ret(c);
                return;
            }
        }
    }
}

/// `void DirtyRpc_IntcHandler(int arg0, void *state)` (0x3f4410):
/// account `arg0` against the RPC interrupt state: add the 32-bit
/// difference to the 64-bit total and latch the current count, leaving 0.
pub fn dirty_rpc_intc_handler(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x4df8).lw(c);
    c.mfc0(A0, 9);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x4e00).ld(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[V0][0] = c.g[V0][0] >> 32;
    c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[V0][0]);
    Addr(c.g[A1][0] as u32).off(0x4e00).sd(c, c.g[V1][0]);
    Addr(c.g[A1][0] as u32).off(0x4df8).sw(c, c.g[A0][0]);
    c.ei();
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `void ProtoAries_RpcReplyCallback(void)` (0x3f5a50): stash the Aries
/// reply's `t3` context into the reply slot `*(sp)`, clearing both status
/// words.
pub fn proto_aries_rpc_reply_callback(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = Addr(sp(c)).lw(c);
    c.g[V0][0] = 0u64;
    Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sw(c, c.g[T3][0]);
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, 0u64);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
    ret(c);
}

/// `int SLogin_StepNop(void)` (0x3f77f0): a login step that does nothing
/// and reports 0.
pub fn s_login_step_nop(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

// ---------------------------------------------------------------------------
// `SLoginMc_SyncResult_Cmd00/Cmd01` (0x3f87f8/0x3f8800) constants.

/// Global login state pointer (`gp + 0x1ae8`).
pub(crate) const SLOGIN_STATE_OFF: u32 = 0x1ae8;
/// Login state field: async step the sync result belongs to.
pub(crate) const SLOGIN_STEP_OFF: u32 = 0x4;

/// `int SLoginMc_SyncResult_Cmd00(int result)` (0x3f87f8): report a memory
/// card sync result for step 0: echo the result.
pub fn s_login_mc_sync_result_cmd00(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd01(int result)` (0x3f8800): report a memory
/// card sync result for step 1: 0x12 while the step is not running,
/// otherwise the normalized result (3 for 0 and 3, 0x11 for 0x11, 0x10
/// for anything else).
pub fn s_login_mc_sync_result_cmd01(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_STATE_OFF)).lw(c);
    c.g[V1][0] = 0x1;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(SLOGIN_STEP_OFF).lw(c);
    if c.g[V0][0] != c.g[V1][0] {
        c.g[V0][0] = 0x12;
    } else {
        'b_3f882c: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[V0][0] = 0x3;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x3;
                if c.g[A0][0] == c.g[V0][0] {
                    c.g[V1][0] = 0x11;
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = 0x11;
                    if c.g[A0][0] != c.g[V1][0] {
                        c.g[V0][0] = 0x10;
                        break 'b_3f882c;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0];
        }
    }
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word all of this module's fakes check in the machine's RAM.
    pub(crate) const OC1352_CTRL: u32 = DATA + 0x30_0000;
    /// Magic value arming the fakes for this module's machines.
    pub(crate) const OC1352_MAGIC: u32 = 0x1352_f4ce;
    /// Mailbox area: forced `v0` per faked callee address.
    pub(crate) const OC1352_MAIL: u32 = DATA + 0x31_0000;
    /// Mailbox cell forcing the result of the callee at `addr`.
    pub(crate) fn oc1352_mail(addr: u32) -> u32 {
        OC1352_MAIL + (addr & 0xfff)
    }

    macro_rules! oc1352_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1352_CTRL) != OC1352_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake_call(c, $addr);
                    if c.pc == ra {
                        c.g[V0][0] = c.bus.read32(oc1352_mail($addr)) as u64;
                    }
                }
            )*
            pub(crate) fn oc1352_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1352_fakes!(
        (
            oc1352_fk_39f9d8,
            OC1352_PREV_39F9D8,
            ee::s13::OVERRIDE_0039F9D8,
            ee::s13::f_0039f9d8__generated,
            0x0039_f9d8
        ),
        (
            oc1352_fk_14dc80,
            OC1352_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc1352_fk_147488,
            OC1352_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc1352_fk_14ebb8,
            OC1352_PREV_14EBB8,
            ee::s01::OVERRIDE_0014EBB8,
            ee::s01::f_0014ebb8__generated,
            0x0014_ebb8
        ),
        (
            oc1352_fk_14ebd8,
            OC1352_PREV_14EBD8,
            ee::s01::OVERRIDE_0014EBD8,
            ee::s01::f_0014ebd8__generated,
            0x0014_ebd8
        ),
        (
            oc1352_fk_2639e0,
            OC1352_PREV_2639E0,
            ee::s07::OVERRIDE_002639E0,
            ee::s07::f_002639e0__generated,
            0x0026_39e0
        ),
        (
            oc1352_fk_25eb00,
            OC1352_PREV_25EB00,
            ee::s06::OVERRIDE_0025EB00,
            ee::s06::f_0025eb00__generated,
            0x0025_eb00
        ),
        (
            oc1352_fk_317e30,
            OC1352_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc1352_fk_317e98,
            OC1352_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc1352_fk_317e50,
            OC1352_PREV_317E50,
            ee::s10::OVERRIDE_00317E50,
            ee::s10::f_00317e50__generated,
            0x0031_7e50
        ),
        (
            oc1352_fk_318260,
            OC1352_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260
        ),
        (
            oc1352_fk_39fae8,
            OC1352_PREV_39FAE8,
            ee::s13::OVERRIDE_0039FAE8,
            ee::s13::f_0039fae8__generated,
            0x0039_fae8
        ),
        (
            oc1352_fk_267e18,
            OC1352_PREV_267E18,
            ee::s07::OVERRIDE_00267E18,
            ee::s07::f_00267e18__generated,
            0x0026_7e18
        ),
        (
            oc1352_fk_267468,
            OC1352_PREV_267468,
            ee::s07::OVERRIDE_00267468,
            ee::s07::f_00267468__generated,
            0x0026_7468
        ),
    );

    /// `UStr_Copy` bounded like the real thing: the source is a miss-hook
    /// result pointing into the test data area, so copy at most 255 bytes
    /// up to the NUL.
    static OC1352_PREV_USTR_COPY: AtomicUsize = AtomicUsize::new(0);
    fn oc1352_fk_ustr_copy(c: &mut Cpu) {
        if c.bus.read32(OC1352_CTRL) != OC1352_MAGIC {
            return fakes::fall_back(c, &OC1352_PREV_USTR_COPY, ee::s08::f_002c2508__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, 0x002c_2508);
        if c.pc != ra {
            return;
        }
        let (mut dst, mut src) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        for _ in 0..256 {
            let b = c.lbu(src);
            c.sb(dst, b);
            if b == 0 {
                break;
            }
            dst = dst.wrapping_add(1);
            src = src.wrapping_add(1);
        }
        c.g[V0][0] = c.g[A0][0];
    }

    /// `BE_GetInterface`: interface 1 is the player interface, anything
    /// else the race interface.
    static OC1352_PREV_BE_GET_INTERFACE: AtomicUsize = AtomicUsize::new(0);
    fn oc1352_fk_be_get_interface(c: &mut Cpu) {
        if c.bus.read32(OC1352_CTRL) != OC1352_MAGIC {
            return fakes::fall_back(
                c,
                &OC1352_PREV_BE_GET_INTERFACE,
                ee::s01::f_0014dd58__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, 0x0014_dd58);
        if c.pc == ra {
            c.g[V0][0] = if c.g[A1][0] == 1 {
                OC1352_PI as u64
            } else {
                OC1352_RI as u64
            };
        }
    }

    /// Player interface block handed out by the `BE_GetInterface` fake.
    pub(crate) const OC1352_PI: u32 = DATA + 0x1_4000;
    /// Race interface block handed out by the `BE_GetInterface` fake.
    pub(crate) const OC1352_RI: u32 = DATA + 0x1_5000;

    /// Arm this module's fakes for the machine `c` (both diff sides get
    /// their own machine, so the gate is per-machine).
    pub(crate) fn oc1352_arm(c: &mut Cpu) {
        oc1352_install_fakes();
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2508),
            oc1352_fk_ustr_copy,
            &OC1352_PREV_USTR_COPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc1352_fk_be_get_interface,
            &OC1352_PREV_BE_GET_INTERFACE,
        );
        c.bus.write32(OC1352_CTRL, OC1352_MAGIC);
    }

    /// A scrambled machine with this module's fakes armed and no unwinds.
    pub(crate) fn machine_box(salt: u64) -> Box<Cpu> {
        let mut c = crate::difftest::fresh();
        machine(&mut c, 0, salt, 0);
        oc1352_arm(&mut c);
        c
    }

    // ------------------------------------------------------------------
    // `cGameComm_UpdateWaitPopup` (0x258cb0).

    pub(crate) const WAIT_POPUP_COMM: u32 = DATA + 0x1000;
    pub(crate) const WAIT_POPUP_APP: u32 = DATA + 0x2000;
    pub(crate) const WAIT_POPUP_MGR: u32 = DATA + 0x3000;
    pub(crate) const WAIT_POPUP_UIMGR: u32 = DATA + 0x4000;
    pub(crate) const WAIT_POPUP_STATE: u32 = DATA + 0x5000;
    pub(crate) const WAIT_POPUP_POPUP: u32 = DATA + 0x6000;
    pub(crate) const WAIT_POPUP_STROBJ: u32 = DATA + 0x7000;
    pub(crate) const WAIT_POPUP_STRS1: u32 = DATA + 0x7100;
    pub(crate) const WAIT_POPUP_Y: u32 = DATA + 0x8000;
    pub(crate) const WAIT_POPUP_W: u32 = DATA + 0x8100;

    fn wait_popup_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x258c_b0, 4);
        oc1352_arm(c);
        c.g[A0][0] = sx(WAIT_POPUP_COMM);
        c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_RESULT, 7);
        c.bus.write32(
            WAIT_POPUP_COMM + WAIT_POPUP_SHOW_WAIT,
            (seed % 3 == 0) as u32,
        );
        c.bus.write32(
            WAIT_POPUP_COMM + WAIT_POPUP_MODE,
            [0, 1, 2, 3, 7][seed as usize / 8 % 5],
        );
        c.bus
            .write32(WAIT_POPUP_COMM + WAIT_POPUP_LOCAL_PAUSED, seed / 40 % 2);
        c.bus
            .write32(WAIT_POPUP_COMM + WAIT_POPUP_PEER_PAUSED, seed / 80 % 2);
        c.bus.write32(
            WAIT_POPUP_COMM + WAIT_POPUP_RESUME_GRACE,
            [1, 0, 0xffff_ffffu32][seed as usize / 7 % 3],
        );
        c.bus.write32(GAME_GP - 0x848, WAIT_POPUP_APP);
        c.bus.write32(GAME_GP - 0x84c, WAIT_POPUP_Y);
        c.bus.write32(WAIT_POPUP_APP + 0x84, WAIT_POPUP_MGR);
        c.bus.write32(WAIT_POPUP_APP + 0x8c, WAIT_POPUP_STROBJ);
        c.bus.write32(
            WAIT_POPUP_MGR + 0x48,
            if seed % 8 == 0 { 0 } else { WAIT_POPUP_UIMGR },
        );
        c.bus
            .write32(WAIT_POPUP_MGR + 0x214, [1, 2, 3][seed as usize / 3 % 3]);
        c.bus.write32(WAIT_POPUP_MGR + 0xc, WAIT_POPUP_W);
        c.bus
            .write32(WAIT_POPUP_W, [0, 4, 5, 6][seed as usize / 11 % 4]);
        c.bus
            .write32(WAIT_POPUP_Y + 0x550, [0, 1, 2, 3][seed as usize / 5 % 4]);
        c.bus.write32(WAIT_POPUP_STROBJ + 0x4, WAIT_POPUP_STRS1);
        c.bus.write16(WAIT_POPUP_STRS1 + 0x20, 0);
        c.bus.write32(WAIT_POPUP_STRS1 + 0x24, fakes::FAKE + 0x10);
        c.bus.write32(
            oc1352_mail(0x0039_f9d8),
            if seed % 8 == 1 { 0 } else { WAIT_POPUP_STATE },
        );
        c.bus.write32(WAIT_POPUP_STATE + 0x8, WAIT_POPUP_POPUP);
        c.bus.write16(WAIT_POPUP_POPUP + 0xc0, 0);
        c.bus.write32(WAIT_POPUP_POPUP + 0xc4, fakes::FAKE + 0x20);
    }

    /// The wait popup refreshes, shows and hides the net lobby notice:
    /// early exits leave the mode alone, the show tails set it to 1 or 2
    /// and the hide tail clears it.
    #[test]
    fn c_game_comm_update_wait_popup_matches() {
        fakes::covered(
            ee::s06::f_00258cb0__generated,
            c_game_comm_update_wait_popup,
            300,
            &wait_popup_setup,
        );
    }

    /// Each tail writes the mode the popup ends in (hide 0, show-waiting 1,
    /// show-countdown 2, untouched 9).
    #[test]
    fn c_game_comm_update_wait_popup_arms() {
        use crate::difftest::run_generated;
        // (show, ui_null, found_null, mode, local, peer, w, want_mode)
        let rows: [(u32, u32, u32, u32, u32, u32, u32, u32); 4] = [
            // Mode 7 hides through the tail: 0.
            (1, 0, 0, 7, 0, 0, 5, 0),
            // Hidden, then shown counting down: 2.
            (1, 0, 0, 0, 0, 0, 5, 2),
            // Local pause picks the other message, still shows waiting: 1.
            (1, 0, 0, 0, 1, 0, 5, 1),
            // No UI manager: the mode is untouched.
            (1, 1, 0, 9, 0, 0, 5, 9),
        ];
        for (show, ui_null, found_null, mode, local, peer, w, want) in rows {
            for hand in [false, true] {
                let mut c = machine_box(0x258c_b0);
                c.g[A0][0] = sx(WAIT_POPUP_COMM);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_RESULT, 7);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_SHOW_WAIT, show);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_MODE, mode);
                c.bus
                    .write32(WAIT_POPUP_COMM + WAIT_POPUP_LOCAL_PAUSED, local);
                c.bus
                    .write32(WAIT_POPUP_COMM + WAIT_POPUP_PEER_PAUSED, peer);
                c.bus.write32(WAIT_POPUP_COMM + WAIT_POPUP_RESUME_GRACE, 1);
                c.bus.write32(GAME_GP - 0x848, WAIT_POPUP_APP);
                c.bus.write32(GAME_GP - 0x84c, WAIT_POPUP_Y);
                c.bus.write32(WAIT_POPUP_APP + 0x84, WAIT_POPUP_MGR);
                c.bus.write32(WAIT_POPUP_APP + 0x8c, WAIT_POPUP_STROBJ);
                c.bus.write32(
                    WAIT_POPUP_MGR + 0x48,
                    if ui_null != 0 { 0 } else { WAIT_POPUP_UIMGR },
                );
                c.bus.write32(WAIT_POPUP_MGR + 0x214, 3);
                c.bus.write32(WAIT_POPUP_MGR + 0xc, WAIT_POPUP_W);
                c.bus.write32(WAIT_POPUP_W, w);
                c.bus.write32(WAIT_POPUP_Y + 0x550, 0);
                c.bus.write32(WAIT_POPUP_STROBJ + 0x4, WAIT_POPUP_STRS1);
                c.bus.write16(WAIT_POPUP_STRS1 + 0x20, 0);
                c.bus.write32(WAIT_POPUP_STRS1 + 0x24, fakes::FAKE + 0x10);
                c.bus.write32(
                    oc1352_mail(0x0039_f9d8),
                    if found_null != 0 { 0 } else { WAIT_POPUP_STATE },
                );
                c.bus.write32(WAIT_POPUP_STATE + 0x8, WAIT_POPUP_POPUP);
                c.bus.write16(WAIT_POPUP_POPUP + 0xc0, 0);
                c.bus.write32(WAIT_POPUP_POPUP + 0xc4, fakes::FAKE + 0x20);
                if hand {
                    run_generated(&mut c, c_game_comm_update_wait_popup);
                } else {
                    run_generated(&mut c, ee::s06::f_00258cb0__generated);
                }
                assert_eq!(c.bus.read32(WAIT_POPUP_COMM + WAIT_POPUP_MODE), want);
            }
        }
    }

    // ------------------------------------------------------------------
    // `cGameComm_SendSyncCheck` (0x258f20).

    pub(crate) const SYNC_CHECK_COMM: u32 = DATA + 0x9000;
    pub(crate) const SYNC_CHECK_APP: u32 = DATA + 0xa000;
    pub(crate) const SYNC_CHECK_MGR: u32 = DATA + 0xb000;
    pub(crate) const SYNC_CHECK_X: u32 = DATA + 0xc000;

    pub(crate) fn sync_check_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x258f_20, 0);
        oc1352_arm(c);
        let frames: [u32; 10] = [0, 1, 59, 60, 61, 119, 120, 121, 1000, 0xffff_ffff];
        let frame = frames[seed as usize % 10];
        let value = 0x1000 + seed;
        c.g[A0][0] = sx(SYNC_CHECK_COMM);
        c.g[A1][0] = sx(frame);
        c.g[A2][0] = value as u64;
        c.bus.write32(GAME_GP - 0x848, SYNC_CHECK_APP);
        c.bus.write32(SYNC_CHECK_APP + 0x84, SYNC_CHECK_MGR);
        // Session-state gate: 5 runs at once, 3 needs the race flag,
        // anything else leaves early.
        let state = [5, 3, 3, 2, 7][seed as usize / 10 % 5];
        c.bus.write32(SYNC_CHECK_MGR + 0xc, SYNC_CHECK_X);
        c.bus.write32(SYNC_CHECK_X, state);
        c.bus.write32(
            SYNC_CHECK_MGR + 0x214,
            if seed / 50 % 2 == 0 { 3 } else { 5 },
        );
        c.bus.write32(
            SYNC_CHECK_COMM + SYNC_CHECK_READY,
            if seed / 3 % 5 == 0 { 0 } else { 1 },
        );
        c.bus.write32(
            SYNC_CHECK_COMM + SYNC_CHECK_LOCAL_PAUSED,
            if seed / 7 % 4 == 0 { 1 } else { 0 },
        );
        c.bus.write32(
            SYNC_CHECK_COMM + 0x0,
            if seed / 11 % 3 == 0 { 0 } else { 9 },
        );
        c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_DESYNCED, 0);
        c.bus.write32(
            SYNC_CHECK_COMM + SYNC_CHECK_SENT,
            if seed / 13 % 3 == 0 { 0 } else { 1 },
        );
        c.bus.write32(
            SYNC_CHECK_COMM + SYNC_CHECK_REPLIED,
            if seed / 17 % 2 == 0 { 0 } else { 1 },
        );
        c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_FRAME_NO, 0);
        let min = [-1i32, 0, 5, 100][seed as usize / 19 % 4];
        c.bus
            .write32(SYNC_CHECK_COMM + SYNC_CHECK_MIN_FRAME, min as u32);
        c.bus.write32(SYNC_CHECK_COMM + 0x364, 0);
        c.bus.write32(SYNC_CHECK_COMM + 0x368, 0);
        c.bus.write32(SYNC_CHECK_COMM + 0x6c, 0);
        // Nonzero ring fills so a wrong slot index shows as data. On
        // mismatch seeds the compared slot and the stored slot both
        // disagree, whichever the tail reaches.
        let frame_s = frame as i32;
        let m = if frame_s < 0 || min < 0 {
            0
        } else {
            (frame_s.min(min) / 60 % 60) as u32
        };
        let slot = ((frame_s as u32 / 60) % 60) % 60;
        for i in 0..60u32 {
            c.bus
                .write32(SYNC_CHECK_COMM + SYNC_CHECK_RING + 4 * i, 0xA100_0000 + i);
            c.bus.write32(
                SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * i,
                0xA100_0000 + i,
            );
        }
        if seed / 23 % 3 != 0 {
            c.bus.write32(
                SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * m,
                0xA100_0000 + m ^ 0xff,
            );
            c.bus.write32(
                SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * slot,
                0xA100_0000 + slot ^ 0xff,
            );
        }
    }

    /// The sync check gates on the lobby state and the session flags,
    /// records the frame in the ring (sending it first), then flags a
    /// mismatch between the two rings.
    #[test]
    fn c_game_comm_send_sync_check_matches() {
        check(
            ee::s06::f_00258f20__generated,
            c_game_comm_send_sync_check,
            300,
            &sync_check_setup,
        );
    }

    /// The send path stores the frame and value and raises SENT; a
    /// matching peer ring leaves DESYNCED clear, a mismatch sets it.
    #[test]
    fn c_game_comm_send_sync_check_arms() {
        use crate::difftest::run_generated;
        // (sent_in, replied_in, min, frame, value, want_sent, want_desync)
        // Note: the ring store runs unconditionally (even when SENT is
        // already set), so rows 1-2 still overwrite ring[slot] and the
        // compare that follows always sees the 0x42 mismatch: DESYNCED ends
        // up set on both the generated and the hand side.
        let rows: [(u32, u32, i32, u32, u32, u32, u32); 4] = [
            (0, 0, 200, 63, 0x42, 1, 1),
            (1, 0, 200, 63, 0x42, 1, 1),
            (1, 1, 200, 63, 0x42, 0, 1),
            (0, 0, -1, 63, 0x42, 1, 0),
        ];
        for (sent_in, replied_in, min, frame, value, want_sent, want_desync) in rows {
            for hand in [false, true] {
                let mut c = machine_box(0x258f_20);
                c.g[A0][0] = sx(SYNC_CHECK_COMM);
                c.g[A1][0] = sx(frame);
                c.g[A2][0] = value as u64;
                c.bus.write32(GAME_GP - 0x848, SYNC_CHECK_APP);
                c.bus.write32(SYNC_CHECK_APP + 0x84, SYNC_CHECK_MGR);
                c.bus.write32(SYNC_CHECK_MGR + 0xc, SYNC_CHECK_X);
                c.bus.write32(SYNC_CHECK_X, 5);
                c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_READY, 1);
                c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_LOCAL_PAUSED, 0);
                c.bus.write32(SYNC_CHECK_COMM + 0x0, 9);
                c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_DESYNCED, 0);
                c.bus.write32(SYNC_CHECK_COMM + SYNC_CHECK_SENT, sent_in);
                c.bus
                    .write32(SYNC_CHECK_COMM + SYNC_CHECK_REPLIED, replied_in);
                c.bus
                    .write32(SYNC_CHECK_COMM + SYNC_CHECK_MIN_FRAME, min as u32);
                c.bus.write32(SYNC_CHECK_COMM + 0x364, 0);
                c.bus.write32(SYNC_CHECK_COMM + 0x368, 0);
                c.bus.write32(SYNC_CHECK_COMM + 0x6c, 0);
                let slot = (frame / 60 % 60) % 60;
                let m = if min < 0 {
                    0
                } else {
                    ((frame.min(min as u32)) / 60 % 60) % 60
                };
                for i in 0..60u32 {
                    c.bus
                        .write32(SYNC_CHECK_COMM + SYNC_CHECK_RING + 4 * i, 0xA100_0000 + i);
                    c.bus.write32(
                        SYNC_CHECK_COMM + SYNC_CHECK_PEER_RING + 4 * i,
                        0xA100_0000 + i,
                    );
                }
                if hand {
                    run_generated(&mut c, c_game_comm_send_sync_check);
                } else {
                    run_generated(&mut c, ee::s06::f_00258f20__generated);
                }
                assert_eq!(c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_SENT), want_sent);
                assert_eq!(
                    c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_DESYNCED),
                    want_desync
                );
                if sent_in == 0 {
                    assert_eq!(
                        c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_RING + 4 * slot),
                        value
                    );
                    assert_eq!(c.bus.read32(SYNC_CHECK_COMM + SYNC_CHECK_FRAME_NO), frame);
                }
                let _ = m;
            }
        }
    }

    // ------------------------------------------------------------------
    // `LobbyMan_SetupNetRace` (0x259098).

    pub(crate) const SETUP_NET_RACE_G: u32 = DATA + 0xd000;
    pub(crate) const SETUP_NET_RACE_VT: u32 = DATA + 0xf100;

    fn setup_net_race_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x2590_98, 2);
        oc1352_arm(c);
        c.g[A0][0] = sx(DATA + 0x100 + seed % 0x10);
        c.g[A1][0] = sx(DATA + 0x200 + seed % 0x20);
        c.g[A2][0] = (seed % 7) as u64;
        c.bus.write32(GAME_GP - 0xc8, SETUP_NET_RACE_G);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x300);
        c.bus.write32(DATA + 0x300 + 0x78, DATA + 0x400);
        c.bus
            .write32(SETUP_NET_RACE_G + SETUP_NET_RACE_ACTIVE, seed % 2);
        c.bus.write32(
            SETUP_NET_RACE_G + SETUP_NET_RACE_PENDING,
            0xA0A0_0000 + seed % 5,
        );
        c.bus.write32(
            SETUP_NET_RACE_G + SETUP_NET_RACE_CURRENT,
            0xB0B0_0000 + seed % 5,
        );
        c.bus
            .write32(SETUP_NET_RACE_G + 0x98, 0xC0C0_0000 + seed % 3);
        c.bus
            .write32(SETUP_NET_RACE_G + 0x80, 0xD0D0_0000 + seed % 3);
        c.bus.write32(oc1352_mail(0x0014_dc80), DATA + 0x500);
        c.bus.write32(oc1352_mail(0x0014_7488), 3 + seed % 4);
        c.bus.write32(oc1352_mail(0x0014_ebb8), 0);
        c.bus.write32(oc1352_mail(0x0014_ebd8), 0);
        c.bus.write32(oc1352_mail(0x0026_39e0), 0);
        c.bus.write32(OC1352_RI + 0xc, SETUP_NET_RACE_VT);
        c.bus.write16(SETUP_NET_RACE_VT + 0x8, 0);
        c.bus.write32(SETUP_NET_RACE_VT + 0xc, fakes::FAKE + 0x10);
    }

    /// Setting up a net race resolves both interfaces, records the local
    /// character and publishes the pending strings before handing off.
    #[test]
    fn lobby_man_setup_net_race_matches() {
        fakes::covered(
            ee::s06::f_00259098__generated,
            lobby_man_setup_net_race,
            300,
            &setup_net_race_setup,
        );
    }

    // ------------------------------------------------------------------
    // `Lobby_MemAlloc` (0x2591e8), `Lobby_MemFree` (0x259210).

    /// Allocating goes through the named operator new; freeing a null
    /// pointer returns at once, freeing a block calls the heap free.
    #[test]
    fn lobby_mem_alloc_matches() {
        check(
            ee::s06::f_002591e8__generated,
            lobby_mem_alloc,
            200,
            &|c, seed| {
                machine(c, seed, 0x2591_e8, 0);
                oc1352_arm(c);
                c.g[A0][0] = [0, 1, 7, 16, 64, 0x1000, 0x8000_0000u32.cast_signed()]
                    [seed as usize % 7] as u64;
                c.bus.write32(
                    oc1352_mail(0x0031_7e30),
                    if seed % 3 == 0 { 0 } else { DATA + 0x6000 },
                );
            },
        );
    }

    #[test]
    fn lobby_mem_free_matches() {
        check(
            ee::s06::f_00259210__generated,
            lobby_mem_free,
            200,
            &|c, seed| {
                machine(c, seed, 0x2592_10, 0);
                oc1352_arm(c);
                c.g[A0][0] = if seed % 4 == 0 {
                    0
                } else {
                    (DATA + 0x6000 + seed % 0x40) as u64
                };
                c.bus.write32(oc1352_mail(0x0031_7e98), 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // `LobbyMan_ForwardEventToUI` (0x259230).

    pub(crate) const FORWARD_EVENT_APP: u32 = DATA + 0x8000;
    pub(crate) const FORWARD_EVENT_OBJ: u32 = DATA + 0x8100;
    pub(crate) const FORWARD_EVENT_ENV: u32 = DATA + 0x8200;
    pub(crate) const FORWARD_EVENT_VT: u32 = DATA + 0x8300;
    pub(crate) const FORWARD_EVENT_EV: u32 = DATA + 0x8400;

    fn forward_event_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x2592_30, 2);
        oc1352_arm(c);
        c.g[A0][0] = sx(FORWARD_EVENT_EV);
        c.bus.write32(FORWARD_EVENT_EV, 0xE000_0000 + seed % 16);
        c.bus.write32(GAME_GP - 0x848, FORWARD_EVENT_APP);
        c.bus.write32(
            FORWARD_EVENT_APP + 0x7c,
            if seed % 3 == 0 { 0 } else { FORWARD_EVENT_OBJ },
        );
        c.bus.write32(FORWARD_EVENT_OBJ + 0xc, FORWARD_EVENT_ENV);
        c.bus.write32(FORWARD_EVENT_ENV + 0x8, FORWARD_EVENT_VT);
        c.bus.write16(FORWARD_EVENT_VT + 0xc0, 0);
        c.bus.write32(FORWARD_EVENT_VT + 0xc4, fakes::FAKE + 0x10);
        c.bus.write32(oc1352_mail(0x0039_fae8), FORWARD_EVENT_ENV);
    }

    /// Lobby events with no menu object go nowhere; otherwise the menu's
    /// envelope resolves and its virtual handler runs.
    #[test]
    fn lobby_man_forward_event_to_ui_matches() {
        fakes::covered(
            ee::s06::f_00259230__generated,
            lobby_man_forward_event_to_ui,
            300,
            &forward_event_setup,
        );
    }

    /// The null lobby callback returns at once, leaving everything alone.
    #[test]
    fn lobby_man_null_lobby_callback_matches() {
        check(
            ee::s06::f_00259390__generated,
            lobby_man_null_lobby_callback,
            200,
            &|c, seed| {
                machine(c, seed, 0x2593_90, 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // `LobbyMan_InitStringFields` (0x268890).

    pub(crate) const STRING_FIELDS_OBJ: u32 = DATA + 0x1000;

    driven!(gen_init_string_fields, ee::s07::f_00268890__generated);
    driven!(hand_init_string_fields, lobby_man_init_string_fields);

    /// The string fields take the shared table value, five at once then
    /// four more through the event-checked loop.
    #[test]
    fn lobby_man_init_string_fields_matches() {
        check_events(
            gen_init_string_fields,
            hand_init_string_fields,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2688_90, 0);
                c.bus.write32(
                    GAME_GP + STRING_FIELDS_TABLE_OFF,
                    0x0A0B_0000 + seed % 0x1000,
                );
                let obj = STRING_FIELDS_OBJ + (seed % 4) * 0x40;
                for i in 0..9u32 {
                    c.bus.write32(obj + 4 * i, 0xA5A5_0000 + seed + i);
                }
                let mode = schedule(c, &mut r, seed, 0x0026_8890);
                c.g[A0][0] = sx(obj);
                if mode != Events::Off {
                    c.next_event = c.cycles + 4;
                }
                mode
            },
        );
    }

    // ------------------------------------------------------------------
    // `BuddyMan_dtor` (0x2689b8).

    pub(crate) const BUDDY_OBJ: u32 = DATA + 0x1000;
    pub(crate) const BUDDY_RANGE: u32 = DATA + 0x2000;

    fn buddy_dtor_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x2689_b8, 0);
        oc1352_arm(c);
        c.g[A0][0] = sx(BUDDY_OBJ);
        c.g[A1][0] = (seed % 4) as u64;
        c.bus
            .write32(BUDDY_OBJ + BUDDY_DTOR_STRINGS, BUDDY_OBJ + 0x40);
        // First range: null begin, or 0, 1, 2 entries.
        let (begin, end) = match seed / 5 % 4 {
            0 => (0, BUDDY_RANGE),
            1 => (BUDDY_RANGE, BUDDY_RANGE),
            2 => (BUDDY_RANGE, BUDDY_RANGE + 4),
            _ => (BUDDY_RANGE, BUDDY_RANGE + 8),
        };
        c.bus.write32(BUDDY_OBJ + BUDDY_DTOR_RANGE_A_BEGIN, begin);
        c.bus.write32(BUDDY_OBJ + BUDDY_DTOR_RANGE_A_END, end);
        c.bus
            .write32(BUDDY_OBJ + BUDDY_DTOR_COUNT_A, (seed / 11 % 3) as u32);
        // Second range starts where the first left off; its length varies
        // 0 to 3 entries so every arm runs.
        let begin_b = if begin == 0 {
            seed / 11 % 3
        } else {
            (end - begin) / 4
        };
        c.bus.write32(
            BUDDY_OBJ + BUDDY_DTOR_RANGE_B_END,
            begin_b + 4 * (seed / 13 % 4),
        );
        c.bus.write32(oc1352_mail(0x0031_8260), 0);
        c.bus.write32(oc1352_mail(0x0031_7e50), 0);
        c.bus.write32(oc1352_mail(0x0031_7e98), 0);
    }

    /// The buddy destructor clears both pointer ranges (freeing storage of
    /// any length but one, deleting a lone entry) and frees itself on the
    /// low flag bit.
    #[test]
    fn buddy_man_dtor_matches() {
        check(
            ee::s07::f_002689b8__generated,
            buddy_man_dtor,
            300,
            &buddy_dtor_setup,
        );
    }

    // ------------------------------------------------------------------
    // `LobbyMan_OnUserStatsReply_Thunk` (0x268e50).

    /// The stats-reply thunk forwards the callback argument plus the
    /// lobby's per-reply offset.
    #[test]
    fn lobby_man_on_user_stats_reply_thunk_matches() {
        check(
            ee::s07::f_00268e50__generated,
            lobby_man_on_user_stats_reply_thunk,
            200,
            &|c, seed| {
                machine(c, seed, 0x268e_50, 0);
                oc1352_arm(c);
                c.bus.write16(
                    GAME_GP + 0xe0,
                    [0, 1, 0x7fff, 0x8000, 0xffff][seed as usize % 5],
                );
                c.g[A2][0] =
                    [0, 5, 0x1_0000, 0xffff_ffffu32.cast_signed()][seed as usize / 5 % 4] as u64;
                c.bus.write32(oc1352_mail(0x0025_eb00), 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // `NetEventFD_dtor` (0x269da0), `NetEventFE_dtor` (0x269e20),
    // `LobbyMan_GlobalCtors` (0x269ea0).

    pub(crate) const NET_EVENT_OBJ: u32 = DATA + 0x1000;

    /// Both net event destructors run the base destructor and return.
    #[test]
    fn net_event_fd_dtor_matches() {
        check(
            ee::s07::f_00269da0__generated,
            net_event_fd_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 0x269d_a0, 0);
                oc1352_arm(c);
                c.g[A0][0] = sx(NET_EVENT_OBJ + (seed % 4) * 0x40);
                c.bus.write32(oc1352_mail(0x0026_7e18), 0);
            },
        );
    }

    #[test]
    fn net_event_fe_dtor_matches() {
        check(
            ee::s07::f_00269e20__generated,
            net_event_fe_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 0x269e_a0, 0);
                oc1352_arm(c);
                c.g[A0][0] = sx(NET_EVENT_OBJ + (seed % 4) * 0x40);
                c.bus.write32(oc1352_mail(0x0026_7e18), 0);
            },
        );
    }

    /// The global constructors run the lobby's static initializer.
    #[test]
    fn lobby_man_global_ctors_matches() {
        check(
            ee::s07::f_00269ea0__generated,
            lobby_man_global_ctors,
            100,
            &|c, seed| {
                machine(c, seed, 0x269e_a0, 0);
                oc1352_arm(c);
                c.bus.write32(oc1352_mail(0x0026_7468), 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // `ProtoPing_RpcReplyCallback` (0x3e74e8),
    // `ProtoAries_RpcReplyCallback` (0x3f5a50).

    /// The ping reply callback stashes `t3` in the reply slot and clears
    /// its status word.
    #[test]
    fn proto_ping_rpc_reply_callback_matches() {
        check(
            ee::s14::f_003e74e8__generated,
            proto_ping_rpc_reply_callback,
            200,
            &|c, seed| {
                machine(c, seed, 0x3e74_e8, 0);
                let slot = DATA + 0x2000 + (seed / 8 % 8) * 0x10;
                c.bus.write32(crate::difftest::SP, slot);
                c.g[T3][0] = (DATA + 0x1000 + (seed % 8) * 0x10) as u64;
            },
        );
    }

    /// The Aries reply callback stashes `t3` and clears both status words.
    #[test]
    fn proto_aries_rpc_reply_callback_matches() {
        check(
            ee::s14::f_003f5a50__generated,
            proto_aries_rpc_reply_callback,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f5a_50, 0);
                let slot = DATA + 0x2000 + (seed / 8 % 8) * 0x10;
                c.bus.write32(crate::difftest::SP, slot);
                c.g[T3][0] = (DATA + 0x1000 + (seed % 8) * 0x10) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // `Lobby_HashStrCmpNoCase` (0x3eeab8).

    pub(crate) const HASH_CMP_A: u32 = DATA + 0x1000;
    pub(crate) const HASH_CMP_B: u32 = DATA + 0x1100;

    driven!(gen_hash_str_cmp_no_case, ee::s14::f_003eeab8__generated);
    driven!(hand_hash_str_cmp_no_case, lobby_hash_str_cmp_no_case);

    fn hash_cmp_put(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// Case-insensitive lobby string compare: equal up to case and
    /// skippable blanks, ordered otherwise, empty against anything.
    #[test]
    fn lobby_hash_str_cmp_no_case_matches() {
        check_events(
            gen_hash_str_cmp_no_case,
            hand_hash_str_cmp_no_case,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3eea_b8, 0);
                // Fold table: blanks skip (1), upper case folds down,
                // everything else keys on itself, NUL terminates (0).
                for i in 0..256u32 {
                    let fold = if i == 0 {
                        0
                    } else if i == b' ' as u32 {
                        1
                    } else if (b'A' as u32..=b'Z' as u32).contains(&i) {
                        i + 32
                    } else {
                        i
                    };
                    c.bus.write8(HASH_CMP_FOLD_TABLE + i, fold as u8);
                }
                match seed % 6 {
                    0 => {
                        hash_cmp_put(c, HASH_CMP_A, b"a b-C07");
                        hash_cmp_put(c, HASH_CMP_B, b"A  B-c07");
                    }
                    1 => {
                        hash_cmp_put(c, HASH_CMP_A, b"abc");
                        hash_cmp_put(c, HASH_CMP_B, b"abd");
                    }
                    2 => {
                        hash_cmp_put(c, HASH_CMP_A, b"ab");
                        hash_cmp_put(c, HASH_CMP_B, b"abc");
                    }
                    3 => {
                        hash_cmp_put(c, HASH_CMP_A, b"");
                        hash_cmp_put(c, HASH_CMP_B, b"");
                    }
                    4 => {
                        hash_cmp_put(c, HASH_CMP_A, b"Quick Match 12");
                        hash_cmp_put(c, HASH_CMP_B, b"quick match 13");
                    }
                    _ => {
                        hash_cmp_put(c, HASH_CMP_A, b" - ");
                        hash_cmp_put(c, HASH_CMP_B, b"");
                    }
                }
                let mode = schedule(c, &mut r, seed, 0x003e_eab8);
                c.g[A0][0] = sx(HASH_CMP_A);
                c.g[A1][0] = sx(HASH_CMP_B);
                if mode != Events::Off {
                    c.next_event = c.cycles + 4;
                }
                mode
            },
        );
    }

    // ------------------------------------------------------------------
    // `DirtyRpc_IntcHandler` (0x3f4410).

    pub(crate) const INTC_STATE: u32 = DATA + 0x1000;

    /// The interrupt handler adds the count difference to the 64-bit
    /// total and latches the new count.
    #[test]
    fn dirty_rpc_intc_handler_matches() {
        check(
            ee::s14::f_003f4410__generated,
            dirty_rpc_intc_handler,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f44_10, 0);
                c.g[A0][0] = [
                    0,
                    1,
                    0x7fff_ffff,
                    0x8000_0000u32.cast_signed(),
                    0xffff_ffffu32.cast_signed(),
                ][seed as usize % 5] as u64;
                c.g[A1][0] = sx(INTC_STATE);
                c.bus.write32(
                    INTC_STATE + 0x4df8,
                    [0, 100, 0xffff_ffffu32][seed as usize / 5 % 3],
                );
                c.bus.write32(INTC_STATE + 0x4e00, 0x1111_1111 + seed);
                c.bus.write32(INTC_STATE + 0x4e04, 0x2222_2222 + seed);
            },
        );
    }

    // ------------------------------------------------------------------
    // `SLogin_StepNop` (0x3f77f0), `SLoginMc_SyncResult_Cmd00` (0x3f87f8),
    // `SLoginMc_SyncResult_Cmd01` (0x3f8800).

    /// The no-op login step reports 0.
    #[test]
    fn s_login_step_nop_matches() {
        check(
            ee::s14::f_003f77f0__generated,
            s_login_step_nop,
            100,
            &|c, seed| {
                machine(c, seed, 0x3f77_f0, 0);
            },
        );
    }

    pub(crate) const SLOGIN_STATE: u32 = DATA + 0x1000;

    /// Step 0 echoes the sync result.
    #[test]
    fn s_login_mc_sync_result_cmd00_matches() {
        check(
            ee::s14::f_003f87f8__generated,
            s_login_mc_sync_result_cmd00,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f87_f8, 0);
                c.g[A0][0] =
                    [0, 1, 3, 0x10, 0x11, 0xffff_ffffu32.cast_signed()][seed as usize % 6] as u64;
            },
        );
    }

    /// Step 1 reports 0x12 while the step is not running, otherwise the
    /// normalized result.
    #[test]
    fn s_login_mc_sync_result_cmd01_matches() {
        check(
            ee::s14::f_003f8800__generated,
            s_login_mc_sync_result_cmd01,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f88_00, 0);
                c.g[A0][0] =
                    [0, 3, 0x10, 0x11, 7, 0xffff_ffffu32.cast_signed()][seed as usize % 6] as u64;
                c.bus.write32(GAME_GP + SLOGIN_STATE_OFF, SLOGIN_STATE);
                c.bus.write32(SLOGIN_STATE + SLOGIN_STEP_OFF, seed / 6 % 3);
            },
        );
    }
}
