//! Staging module oc1006 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1006`.
//!
//! LobbyMan account / connection functions from `dirtysock/tagsunk.cpp`:
//! creating the lobby lists and API, posting a chat event, connecting,
//! handling the connect / create-account replies, creating an account,
//! requesting a lost account by e-mail or by name, and logging in.

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
    (0x0025_c610, lobby_man_login, "LobbyMan_Login"),
    (
        0x0025_c3f8,
        lobby_man_send_lost_by_name,
        "LobbyMan_SendLostByName",
    ),
    (
        0x0025_c1e0,
        lobby_man_send_lost_by_email,
        "LobbyMan_SendLostByEmail",
    ),
    (
        0x0025_bd30,
        lobby_man_on_create_account_reply,
        "LobbyMan_OnCreateAccountReply",
    ),
    (
        0x0025_b9c8,
        lobby_man_create_account,
        "LobbyMan_CreateAccount",
    ),
    (
        0x0025_b848,
        lobby_man_on_connect_reply,
        "LobbyMan_OnConnectReply",
    ),
    (0x0025_b6d8, lobby_man_connect, "LobbyMan_Connect"),
    (
        0x0025_ab40,
        lobby_man_post_chat_event,
        "LobbyMan_PostChatEvent",
    ),
    (
        0x0025_a948,
        lobby_man_create_lobby_api,
        "LobbyMan_CreateLobbyApi",
    ),
    (0x0025_a778, lobby_man_create_lists, "LobbyMan_CreateLists"),
];

/// Named callees used by the functions in this module (one name per address).
pub(crate) mod callee {
    use super::*;

    /// `cDirtysock_tag_TagFieldSetString` (0x3ec180).
    pub const TAG_FIELD_SET_STRING: Callee = Callee {
        addr: 0x003e_c180,
        f: ee::s14::f_003ec180,
    };
    /// `cDirtysock_tag_TagFieldSetNumber` (0x3ebe40).
    pub const TAG_FIELD_SET_NUMBER: Callee = Callee {
        addr: 0x003e_be40,
        f: ee::s14::f_003ebe40,
    };
    /// `cDirtysock_tag_TagFieldFind` (0x3eb6d8).
    pub const TAG_FIELD_FIND: Callee = Callee {
        addr: 0x003e_b6d8,
        f: ee::s14::f_003eb6d8,
    };
    /// `cDirtysock_tag_TagFieldGetNumber` (0x3ecf30).
    pub const TAG_FIELD_GET_NUMBER: Callee = Callee {
        addr: 0x003e_cf30,
        f: ee::s14::f_003ecf30,
    };
    /// `sprintf` (0x417828).
    pub const SPRINTF: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };
    /// `cMemMan_alloc` / EA_MemAllocNamed (0x317d70): the game's operator new.
    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };
    /// `cBXString` copy constructor (0x3182d8).
    pub const C_BX_STRING_CTOR_CSTR: Callee = Callee {
        addr: 0x0031_82d8,
        f: ee::s10::f_003182d8,
    };
    /// `cBXString` constructor from a C string (0x317fe8).
    pub const C_BX_STRING_CTOR_COPY: Callee = Callee {
        addr: 0x0031_7fe8,
        f: ee::s10::f_00317fe8,
    };
    /// `cBXString` destructor (0x318260).
    pub const C_BX_STRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::f_00318260,
    };
    /// `LobbyMan_GetLanguage` (0x259198).
    pub const LOBBY_MAN_GET_LANGUAGE: Callee = Callee {
        addr: 0x0025_9198,
        f: ee::s06::LobbyMan_GetLanguage,
    };
    /// `LobbyMan_SendRequest` (0x25b608).
    pub const LOBBY_MAN_SEND_REQUEST: Callee = Callee {
        addr: 0x0025_b608,
        f: ee::s06::LobbyMan_SendRequest,
    };
    /// `LobbyMan_CreateLobbyApi` (0x25a948).
    pub const LOBBY_MAN_CREATE_LOBBY_API: Callee = Callee {
        addr: 0x0025_a948,
        f: ee::s06::LobbyMan_CreateLobbyApi,
    };
    /// `LobbyMan_CreateLists` (0x25a778).
    pub const LOBBY_MAN_CREATE_LISTS: Callee = Callee {
        addr: 0x0025_a778,
        f: ee::s06::LobbyMan_CreateLists,
    };
    /// `LobbyMan_ParseNameSuggestions` (0x25bc48).
    pub const LOBBY_MAN_PARSE_NAME_SUGGESTIONS: Callee = Callee {
        addr: 0x0025_bc48,
        f: ee::s06::LobbyMan_ParseNameSuggestions,
    };
    /// Local-host / machine-name string getter (0x3f3f78).
    pub const NET_CONN_MAC: Callee = Callee {
        addr: 0x003f_3f78,
        f: ee::s14::f_003f3f78,
    };
    /// Connection-flags getter (0x266f38).
    pub const C_VOIP_MAN_GET_SHARED_STATUS: Callee = Callee {
        addr: 0x0026_6f38,
        f: ee::s07::f_00266f38,
    };
    /// Lobby-list allocator by index (0x3e85b0).
    pub const LOBBY_API_CREATE_LIST: Callee = Callee {
        addr: 0x003e_85b0,
        f: ee::s14::f_003e85b0,
    };
    /// Lobby-list field setter (0x3ee5a0).
    pub const LOBBY_LIST_SET_SORT: Callee = Callee {
        addr: 0x003e_e5a0,
        f: ee::s14::f_003ee5a0,
    };
    /// Lobby API creation helper (0x3e7ff0).
    pub const LOBBY_API_CREATE: Callee = Callee {
        addr: 0x003e_7ff0,
        f: ee::s14::f_003e7ff0,
    };
    /// Lobby API per-index slot setter (0x3e8248).
    pub const LOBBY_API_SET_CALLBACK: Callee = Callee {
        addr: 0x003e_8248,
        f: ee::s14::f_003e8248,
    };
    /// `LobbyApi_Connect` (0x3e8260).
    pub const LOBBY_API_CONNECT: Callee = Callee {
        addr: 0x003e_8260,
        f: ee::s14::f_003e8260,
    };
    /// Socket-lookup creation (0x3f3a98).
    pub const SOCKET_LOOKUP: Callee = Callee {
        addr: 0x003f_3a98,
        f: ee::s14::f_003f3a98,
    };
    /// Tag-record field clear / delete helper (0x3e8b10).
    pub const LOBBY_API_SEND: Callee = Callee {
        addr: 0x003e_8b10,
        f: ee::s14::f_003e8b10,
    };
}

/// LobbyMan object: the lobby API / session handle.
pub(crate) const LOBBY_SESSION: u32 = 0x54;
/// LobbyMan object: first created list pointer.
pub(crate) const LOBBY_LIST_0: u32 = 0x5c;
/// LobbyMan object: second created list pointer.
pub(crate) const LOBBY_LIST_1: u32 = 0x60;
/// LobbyMan object: third created list pointer.
pub(crate) const LOBBY_LIST_2: u32 = 0x64;
/// LobbyMan object: message-list head pointer used when posting a message.
pub(crate) const LOBBY_MSG_HEAD: u32 = 0xf0;

/// Resume head of the clear loop in `LobbyMan_CreateLists`.
pub(crate) const CREATE_LISTS_CLEAR_HEAD: u32 = 0x0025_a820;
/// Resume head of the slot scan loop in `LobbyMan_Login`.
pub(crate) const LOGIN_SLOT_SCAN_HEAD: u32 = 0x0025_c740;

/// `void LobbyMan_CreateLists(LobbyMan* self)` (0x25a778): create the three
/// lobby lists (rooms, users and ranks) on the session at `self+0x54`, store
/// them at `self+0x5c..0x64`, then clear the 29 following list-head slots to -1.
pub fn lobby_man_create_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x270000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7570));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_SESSION));
        callee::LOBBY_API_CREATE_LIST.call(c, 0x0025_a7a0)?;
        c.g[A3][0] = 0x260000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_LIST_0), c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6c68));
        c.g[A1][0] = 0;
        callee::LOBBY_LIST_SET_SORT.call(c, 0x0025_a7bc)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_SESSION));
        c.g[A2][0] = 0x270000;
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7548));
        c.g[A1][0] = 0x1;
        callee::LOBBY_API_CREATE_LIST.call(c, 0x0025_a7d4)?;
        c.g[A3][0] = 0x260000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_LIST_1), c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6c48));
        c.g[A1][0] = 0;
        callee::LOBBY_LIST_SET_SORT.call(c, 0x0025_a7f0)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_SESSION));
        c.g[A2][0] = 0x270000;
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7520));
        c.g[A1][0] = 0x2;
        callee::LOBBY_API_CREATE_LIST.call(c, 0x0025_a808)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_LIST_2), c.g[V0][0]);
        c.g[V1][0] = 0x1c;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x178));
        c.g[V0][0] |= 0xffff;
        'clear: loop {
            c.cycles += 7;
            c.sw((c.g[S0][0] as u32).wrapping_sub(0x74), c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, CREATE_LISTS_CLEAR_HEAD);
                }
                continue 'clear;
            }
            break;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_CreateLobbyApi(LobbyMan* self)` (0x25a948): build the
/// "create session" tag record (name, language and host fields), create the
/// lobby API object, register its per-index slots, then create the lists.
pub fn lobby_man_create_lobby_api(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 84;
        let frame = Frame::open(c, 0x180);
        c.g[A2][0] = 0x4a0000;
        frame.save(c, S1, 0x160);
        c.g[A3][0] = 0x480000;
        frame.save(c, S2, 0x150);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        frame.save(c, S0, 0x170);
        frame.save_ra(c, 0x140);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2fc8));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x40), 0);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x13a0));
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x100;
        callee::TAG_FIELD_SET_STRING.call(c, 0x0025_a984)?;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0x4a0000;
        c.g[A3][0] = 0x480000;
        c.g[T0][0] = 0x480000;
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x828));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2fd0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x818));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x13b0));
        callee::SPRINTF.call(c, 0x0025_a9ac)?;
        c.g[A2][0] = 0x4a0000;
        c.g[A3][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2fd8));
        c.g[A1][0] = 0x100;
        callee::TAG_FIELD_SET_STRING.call(c, 0x0025_a9c4)?;
        c.g[S0][0] = 0x4a0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fe0));
        callee::LOBBY_MAN_GET_LANGUAGE.call(c, 0x0025_a9d0)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x100;
        callee::TAG_FIELD_SET_STRING.call(c, 0x0025_a9e4)?;
        c.g[A2][0] = 0x4a0000;
        c.g[A3][0] = 0x480000;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1390));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2fe8));
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x100;
        callee::TAG_FIELD_SET_STRING.call(c, 0x0025_aa00)?;
        c.g[A2][0] = 0x260000;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6c70));
        c.g[A1][0] = 0;
        callee::LOBBY_API_CREATE.call(c, 0x0025_aa14)?;
        c.g[A2][0] = 0x270000;
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(LOBBY_SESSION), c.g[V0][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x74f8));
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        callee::LOBBY_API_SET_CALLBACK.call(c, 0x0025_aa30)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_SESSION));
        c.g[A2][0] = 0x270000;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x74d0));
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = 0x1;
        callee::LOBBY_API_SET_CALLBACK.call(c, 0x0025_aa48)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_SESSION));
        c.g[A2][0] = 0x270000;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x74a8));
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = 0x3;
        callee::LOBBY_API_SET_CALLBACK.call(c, 0x0025_aa60)?;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_SESSION));
        c.g[A2][0] = 0x270000;
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7480));
        c.g[A1][0] = 0x4;
        callee::LOBBY_API_SET_CALLBACK.call(c, 0x0025_aa78)?;
        c.g[A0][0] = c.g[S1][0];
        callee::LOBBY_MAN_CREATE_LISTS.call(c, 0x0025_aa80)?;
        frame.restore(c, S0, 0x170);
        frame.restore(c, S1, 0x160);
        frame.restore(c, S2, 0x150);
        frame.restore_ra(c, 0x140);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_PostChatEvent(LobbyMan* self, const char* a1, const char* a2)`
/// (0x25ab40): allocate a chat event (type 0xde), copy the two chat strings
/// into it, and link it into the message list at `self+0xf0`.
pub fn lobby_man_post_chat_event(c: &mut Cpu) {
    guest(c, |c| {
        'post: {
            c.cycles += 52;
            let frame = Frame::open(c, 0x90);
            c.g[A3][0] = 0;
            frame.save(c, S0, 0x80);
            frame.save(c, S1, 0x70);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S2, 0x60);
            c.g[S1][0] = c.g[A2][0];
            c.g[S2][0] = c.g[A0][0];
            c.g[A1][0] = 0x480000;
            frame.save(c, S3, 0x50);
            c.g[A2][0] = 0x2000_0000;
            frame.save_ra(c, 0x40);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x498));
            c.g[A0][0] = 0x10;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_ab7c)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[S3][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            callee::C_BX_STRING_CTOR_CSTR.call(c, 0x0025_ab8c)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            callee::C_BX_STRING_CTOR_CSTR.call(c, 0x0025_ab9c)?;
            c.g[V1][0] = 0x480000;
            c.g[V0][0] = 0xde;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1230));
            c.g[A1][0] = c.g[SP][0];
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
            callee::C_BX_STRING_CTOR_COPY.call(c, 0x0025_abbc)?;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = c.g[S0][0];
            callee::C_BX_STRING_CTOR_COPY.call(c, 0x0025_abc8)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x2;
            callee::C_BX_STRING_DTOR.call(c, 0x0025_abd4)?;
            c.g[A1][0] = 0x2;
            c.g[A0][0] = c.g[S0][0];
            callee::C_BX_STRING_DTOR.call(c, 0x0025_abe0)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.g[A0][0] = 0xc;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_ac00)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
            }
            break 'post;
        }
        c.cycles += 16;
        let frame = Frame { size: 0x90 };
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
        frame.restore(c, S0, 0x80);
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        frame.restore(c, S1, 0x70);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        frame.restore(c, S2, 0x60);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        frame.restore(c, S3, 0x50);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        frame.restore_ra(c, 0x40);
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A1][0]);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_Connect(LobbyMan* self, void* persona)` (0x25b6d8): create
/// the lobby API if needed, connect to the lobby when a persona is given
/// (otherwise start the socket lookup), and post the "connecting" message.
pub fn lobby_man_connect(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        'done: {
            'have_api: {
                c.cycles += 9;
                frame.save(c, S0, 0x50);
                frame.save(c, S1, 0x40);
                frame.save(c, S2, 0x30);
                c.g[S1][0] = c.g[A0][0];
                frame.save_ra(c, 0x20);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_SESSION));
                let has_api = c.g[V0][0] != 0;
                c.g[S0][0] = c.g[A1][0];
                if has_api {
                    break 'have_api;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                callee::LOBBY_MAN_CREATE_LOBBY_API.call(c, 0x0025_b704)?;
                break 'have_api;
            }
            'connected: {
                'no_persona: {
                    c.cycles += 2;
                    let no_persona = c.g[S0][0] == 0;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                    if no_persona {
                        break 'no_persona;
                    }
                    c.cycles += 16;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xd8));
                    c.g[V0][0] = 0x3;
                    c.g[AT][0] = 0x41f0_0000;
                    c.mtc1(1, 0);
                    c.g[A3][0] = 0x270000;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_SESSION));
                    c.g[A1][0] = c.g[V1][0];
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7458));
                    c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xe0));
                    c.g[A2][0] = 0x2af8;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[T0][0] = c.g[S1][0];
                    callee::LOBBY_API_CONNECT.call(c, 0x0025_b744)?;
                    c.g[A1][0] = 0x480000;
                    break 'connected;
                }
                c.cycles += 7;
                c.g[V0][0] = 0x2;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe8));
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[A1][0] = 0;
                callee::SOCKET_LOOKUP.call(c, 0x0025_b760)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.g[A1][0] = 0x480000;
                break 'connected;
            }
            'posted: {
                c.cycles += 23;
                c.g[A0][0] = 0x8;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[S0][0] = 0xc8;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_b780)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
                c.g[V0][0] = 0x480000;
                c.sw(c.g[S2][0] as u32, c.g[S0][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                c.g[A1][0] = 0x480000;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_b7b4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] != 0 {
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                }
                break 'posted;
            }
            break 'done;
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.restore(c, S0, 0x50);
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        frame.restore(c, S2, 0x30);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        frame.restore_ra(c, 0x20);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_OnConnectReply(LobbyMan* self, sMessage* msg)` (0x25b848):
/// handle the reply to "connect": mark the lobby busy, clear the name field
/// and post the connected message on success, or post the failure message
/// (type 0x102 with the status) and mark the lobby connecting.
pub fn lobby_man_on_connect_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'done: {
            'have_status: {
                c.cycles += 13;
                frame.save(c, S2, 0x40);
                frame.save(c, S3, 0x30);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S1, 0x50);
                c.g[S3][0] = 0x1;
                frame.save(c, S0, 0x60);
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0x20);
                c.sw(c.g[S2][0] as u32, c.g[S3][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                let failed = c.g[V0][0] != 0;
                c.g[A1][0] = 0x480000;
                if failed {
                    break 'have_status;
                }
                'ok: {
                    c.cycles += 32;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(LOBBY_SESSION));
                    c.g[A2][0] = 0x480000;
                    c.g[A1][0] = 0x7365_0000;
                    c.g[T0][0] = 0;
                    c.g[A1][0] |= 0x6c65;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x848));
                    c.g[A3][0] = 0;
                    callee::LOBBY_API_SEND.call(c, 0x0025_b89c)?;
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                    c.g[A0][0] = 0x8;
                    c.g[A3][0] = 0;
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_b8b4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
                    c.g[V0][0] = 0x480000;
                    c.g[V1][0] = 0xc9;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A1][0] = 0x480000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_b8ec)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0 {
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                    }
                    break 'ok;
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                break 'done;
            }
            'failed: {
                c.cycles += 25;
                c.g[A0][0] = 0xc;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_b934)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x480000;
                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
                c.g[V1][0] = 0x102;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[A1][0] = 0x480000;
                c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = 0xc;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.g[A3][0] = 0;
                c.g[A2][0] = 0x2000_0000;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_b974)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] != 0 {
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                }
                break 'failed;
            }
            c.cycles += 9;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.sw(c.g[A1][0] as u32, c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.sw(c.g[A0][0] as u32, c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S3][0]);
            break 'done;
        }
        c.cycles += 8;
        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_CreateAccount(LobbyMan* self, AccountInfo* info)` (0x25b9c8):
/// format the account fields, build the "acct" create-account tag record and
/// send it, then post the create-account message (type 0xca).
pub fn lobby_man_create_account(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x2b0);
        'sent: {
            c.cycles += 67;
            c.g[V0][0] = 0x59;
            frame.save(c, S0, 0x2a0);
            c.g[A2][0] = 0x4e;
            frame.save(c, S1, 0x290);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S2, 0x280);
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
            frame.save(c, S3, 0x270);
            c.g[A1][0] = 0x4a0000;
            frame.save(c, S4, 0x260);
            c.g[A3][0] = 0x4e;
            frame.save_ra(c, 0x250);
            c.g[S4][0] = c.g[A0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3030));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
            if c.g[A0][0] != 0 {
                c.g[A2][0] = c.g[V0][0];
            }
            if c.g[V1][0] != 0 {
                c.g[A3][0] = c.g[V0][0];
            }
            c.g[A0][0] = c.g[S1][0];
            callee::SPRINTF.call(c, 0x0025_ba1c)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
            c.g[V0][0] = 0x4d;
            c.g[A1][0] = 0x4a0000;
            c.g[A2][0] = 0x46;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3038));
            if c.g[V1][0] == 0 {
                c.g[A2][0] = c.g[V0][0];
            }
            c.g[A0][0] = c.g[S3][0];
            callee::SPRINTF.call(c, 0x0025_ba40)?;
            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
            c.g[A1][0] = 0x480000;
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x870));
            c.g[A0][0] = c.g[S2][0];
            callee::SPRINTF.call(c, 0x0025_ba60)?;
            c.g[A3][0] = c.lw(c.g[S0][0] as u32);
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.sb(c.g[SP][0] as u32, 0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_ba7c)?;
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3040));
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_ba94)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3048));
            c.g[A3][0] = c.g[S1][0];
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_baac)?;
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A2][0] = 0x4a0000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3050));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_bac4)?;
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x8));
            let empty_extra = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[SP][0];
            if empty_extra {
                break 'sent;
            }
            c.cycles += 4;
            c.g[A2][0] = 0x4a0000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3058));
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_bae4)?;
            break 'sent;
        }
        'posted: {
            c.cycles += 72;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3060));
            c.g[A3][0] = c.g[S3][0];
            c.g[A1][0] = 0x200;
            c.g[S1][0] = 0xca;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_bb00)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3068));
            c.g[A3][0] = c.g[S2][0];
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_bb18)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3070));
            c.g[A1][0] = 0x200;
            c.g[A3][0] = 0;
            callee::TAG_FIELD_SET_NUMBER.call(c, 0x0025_bb30)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3078));
            c.g[A1][0] = 0x200;
            c.g[A3][0] = 0x4;
            callee::TAG_FIELD_SET_NUMBER.call(c, 0x0025_bb48)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3080));
            c.g[A1][0] = 0x200;
            c.g[A3][0] = 0xd;
            callee::TAG_FIELD_SET_NUMBER.call(c, 0x0025_bb60)?;
            c.g[S0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fe0));
            callee::LOBBY_MAN_GET_LANGUAGE.call(c, 0x0025_bb6c)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_bb80)?;
            c.g[AT][0] = 0x41a0_0000;
            c.mtc1(1, 12);
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x6163_0000;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7430));
            c.g[A1][0] = 0x4;
            c.g[A0][0] = c.g[S4][0];
            c.g[A2][0] |= 0x6374;
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_bba8)?;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_bbc0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
            c.g[V0][0] = 0x480000;
            c.sw(c.g[S0][0] as u32, c.g[S1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
            c.g[A1][0] = 0x480000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x240), c.g[V1][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_bbf4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
            }
            break 'posted;
        }
        c.cycles += 17;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
        frame.restore(c, S0, 0x2a0);
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        frame.restore(c, S1, 0x290);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        frame.restore(c, S2, 0x280);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        frame.restore(c, S3, 0x270);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        frame.restore(c, S4, 0x260);
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        frame.restore_ra(c, 0x250);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x230), c.g[A1][0]);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_OnCreateAccountReply(LobbyMan* self, sMessage* msg)`
/// (0x25bd30): handle the reply to "create account": mark the lobby busy and
/// post the created message on success, the name-suggestion message for a
/// duplicate name, or the generic failure message (type 0x102).
pub fn lobby_man_on_create_account_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'done: {
            'linked: {
                'node: {
                    'status: {
                        c.cycles += 13;
                        c.g[V0][0] = 0x1;
                        frame.save(c, S3, 0x30);
                        frame.save(c, S1, 0x50);
                        c.g[S3][0] = c.g[A0][0];
                        frame.save(c, S0, 0x60);
                        c.g[S1][0] = c.g[A1][0];
                        frame.save(c, S2, 0x40);
                        frame.save_ra(c, 0x20);
                        c.sw(c.g[S3][0] as u32, c.g[V0][0]);
                        c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        let failed = c.g[S2][0] != 0;
                        c.g[V0][0] = 0x746f_0000;
                        if failed {
                            break 'status;
                        }
                        c.cycles += 20;
                        c.g[A1][0] = 0x480000;
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                        c.g[A0][0] = 0x8;
                        c.g[A3][0] = 0;
                        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_bd7c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
                        c.g[V0][0] = 0x480000;
                        c.g[V1][0] = 0xcb;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[A1][0] = 0x480000;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                        c.g[A0][0] = 0xc;
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        break 'node;
                    }
                    'check_tooy: {
                        c.cycles += 3;
                        c.g[V0][0] |= 0x6f79;
                        let not_tooy = c.g[S2][0] != c.g[V0][0];
                        c.g[V0][0] = 0x6475_0000;
                        if not_tooy {
                            break 'check_tooy;
                        }
                        c.cycles += 34;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        c.g[A1][0] = 0x4a0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3098));
                        callee::TAG_FIELD_FIND.call(c, 0x0025_bdd0)?;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.g[V0][0];
                        callee::TAG_FIELD_GET_NUMBER.call(c, 0x0025_bddc)?;
                        c.g[A1][0] = 0x480000;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                        c.g[A0][0] = 0x10;
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_bdf8)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
                        c.g[V0][0] = 0x480000;
                        c.g[V1][0] = 0x102;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x11f0));
                        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                        c.g[A1][0] = 0x480000;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[S0][0]);
                        c.g[A0][0] = 0xc;
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_be38)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                        if c.g[V0][0] != 0 {
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                            break 'linked;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'done;
                    }
                    'check_dupl: {
                        c.cycles += 3;
                        c.g[V0][0] |= 0x706c;
                        let not_dupl = c.g[S2][0] != c.g[V0][0];
                        c.g[A1][0] = 0x480000;
                        if not_dupl {
                            break 'check_dupl;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S1][0];
                        callee::LOBBY_MAN_PARSE_NAME_SUGGESTIONS.call(c, 0x0025_be68)?;
                        c.g[A1][0] = 0x480000;
                        break 'check_dupl;
                    }
                    c.cycles += 20;
                    c.g[A0][0] = 0xc;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_be80)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x480000;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
                    c.g[V1][0] = 0x102;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[A1][0] = 0x480000;
                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = 0xc;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.g[A3][0] = 0;
                    c.g[A2][0] = 0x2000_0000;
                    break 'node;
                }
                c.cycles += 6;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_bec4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] != 0 {
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                }
                break 'linked;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
            break 'done;
        }
        c.cycles += 15;
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// Shared body of the two "lost account" requests: build a one-field tag
/// record from `a1`, send the "lost" request, and post the reply message.
/// The siblings differ only in the tag key, message type, request id,
/// callback and return addresses (all parameters here).
fn lobby_man_send_lost(
    c: &mut Cpu,
    key: u32,
    msg_type: u64,
    request_id: u64,
    callback_delta: u32,
    rets: [u32; 4],
) -> crate::call::Flow {
    let frame = Frame::open(c, 0x260);
    'posted: {
        c.cycles += 47;
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x250);
        c.g[A2][0] = 0x4a0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x220);
        c.g[A2][0] = sx(key);
        c.sb(c.g[SP][0] as u32, 0);
        c.g[A1][0] = 0x200;
        callee::TAG_FIELD_SET_STRING.call(c, rets[0])?;
        c.g[S1][0] = msg_type;
        c.g[AT][0] = 0x41a0_0000;
        c.mtc1(1, 12);
        c.g[T0][0] = 0x270000;
        c.g[A2][0] = 0x6c6f_0000;
        c.g[A3][0] = c.g[SP][0];
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(callback_delta));
        c.g[A1][0] = request_id;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] |= 0x7374;
        callee::LOBBY_MAN_SEND_REQUEST.call(c, rets[1])?;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x8;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, rets[2])?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_MSG_HEAD));
        c.g[V0][0] = 0x480000;
        c.sw(c.g[S2][0] as u32, c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x210), c.g[V1][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, rets[3])?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        }
        break 'posted;
    }
    c.cycles += 15;
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x210));
    frame.restore(c, S0, 0x250);
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    frame.restore(c, S1, 0x240);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    frame.restore(c, S2, 0x230);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    frame.restore_ra(c, 0x220);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x210));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x200), c.g[A1][0]);
    frame.close(c);
    Ok(())
}

/// `void LobbyMan_SendLostByEmail(LobbyMan* self, const char* email)`
/// (0x25c1e0): ask the lobby for the account that owns `email` ("lost"
/// request 6) and post the by-e-mail reply message (type 0xd1).
pub fn lobby_man_send_lost_by_email(c: &mut Cpu) {
    guest(c, |c| {
        lobby_man_send_lost(
            c,
            0x004a_3050,
            0xd1,
            0x6,
            0x73e0,
            [0x0025_c214, 0x0025_c240, 0x0025_c258, 0x0025_c28c],
        )
    });
}

/// `void LobbyMan_SendLostByName(LobbyMan* self, const char* name)`
/// (0x25c3f8): ask the lobby for the account called `name` ("lost" request
/// 7) and post the by-name reply message (type 0xd0).
pub fn lobby_man_send_lost_by_name(c: &mut Cpu) {
    guest(c, |c| {
        lobby_man_send_lost(
            c,
            0x004a_2f58,
            0xd0,
            0x7,
            0x73b8,
            [0x0025_c42c, 0x0025_c458, 0x0025_c470, 0x0025_c4a4],
        )
    });
}

/// `void LobbyMan_Login(LobbyMan* self)` (0x25c610): build the "auth" login
/// tag record from the account name/password, the host name and the
/// connection capability flags, send it, and post the login message
/// (type 0xd3).
pub fn lobby_man_login(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x2c0);
        'sent: {
            c.cycles += 53;
            c.g[A2][0] = 0x4a0000;
            frame.save(c, S0, 0x2b0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
            frame.save(c, S1, 0x2a0);
            c.g[A1][0] = 0x200;
            frame.save(c, S2, 0x290);
            c.g[S1][0] = 0;
            frame.save(c, S3, 0x280);
            c.g[S2][0] = 0;
            frame.save(c, S4, 0x270);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S5, 0x260);
            c.g[A0][0] = c.g[SP][0];
            frame.save(c, S6, 0x250);
            frame.save(c, S7, 0x240);
            frame.save(c, FP, 0x230);
            frame.save_ra(c, 0x220);
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
            c.sb(c.g[SP][0] as u32, 0);
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_c664)?;
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3040));
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_c67c)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30c0));
            c.g[A1][0] = 0x200;
            c.g[A3][0] = 0x1;
            callee::TAG_FIELD_SET_NUMBER.call(c, 0x0025_c694)?;
            c.g[S0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30c8));
            callee::NET_CONN_MAC.call(c, 0x0025_c6a0)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_c6b4)?;
            c.g[A2][0] = 0x4a0000;
            c.g[A3][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30d0));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x13a0));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_c6d0)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            c.g[V0][0] = 0x1;
            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
            let plain = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = c.g[V1][0] & 0x10;
            if plain {
                break 'sent;
            }
            'flags: {
                c.cycles += 4;
                c.g[S2][0] = 0x1_0000;
                c.g[S1][0] = c.g[V1][0] & 0x4;
                let no_flag10 = c.g[V0][0] == 0;
                c.g[S2][0] |= 0x24;
                if no_flag10 {
                    break 'flags;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x1_0000;
                c.g[S1][0] |= c.g[V0][0];
                break 'flags;
            }
            c.cycles += 3;
            c.g[V1][0] = c.g[V1][0] & 0x8;
            c.g[V0][0] = c.g[S1][0] | 0x20;
            if c.g[V1][0] != 0 {
                c.g[S1][0] = c.g[V0][0];
            }
            break 'sent;
        }
        'posted: {
            'scanned: {
                c.cycles += 13;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x300));
                c.g[S2][0] |= 0x800;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x88));
                c.g[A2][0] = c.g[V1][0];
                c.g[S4][0] = 0x4a0000;
                c.g[S5][0] = 0x4a0000;
                c.g[V0][0] &= 0x1;
                c.g[S6][0] = 0x270000;
                c.g[S7][0] = 0x480000;
                let first_flagged = c.g[V0][0] != 0;
                c.g[FP][0] = sx((c.g[S3][0] as u32).wrapping_add(0xec));
                if first_flagged {
                    break 'scanned;
                }
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                'scan: loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 8i64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x8c;
                    if out {
                        break 'posted;
                    }
                    c.cycles += 6;
                    c.mult(0, 3, 5, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    c.g[V1][0] &= 0x1;
                    let clear = c.g[V1][0] == 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    if clear {
                        if event_due(c) {
                            deliver_event_here(c, LOGIN_SLOT_SCAN_HEAD);
                        }
                        continue 'scan;
                    }
                    break 'scanned;
                }
            }
            c.cycles += 1;
            c.g[S1][0] |= 0x800;
        }
        'linked: {
            c.cycles += 46;
            c.g[S2][0] |= 0x100;
            callee::C_VOIP_MAN_GET_SHARED_STATUS.call(c, 0x0025_c770)?;
            c.g[V0][0] &= 0x10;
            c.g[A3][0] = c.g[S1][0] | 0x100;
            c.g[A0][0] = c.g[SP][0];
            if c.g[V0][0] == 0 {
                c.g[A3][0] = c.g[S1][0];
            }
            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x30d8));
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_NUMBER.call(c, 0x0025_c78c)?;
            c.g[S0][0] = 0xd3;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x30e0));
            c.g[A3][0] = c.g[S2][0];
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_NUMBER.call(c, 0x0025_c7a4)?;
            c.g[AT][0] = 0x41a0_0000;
            c.mtc1(1, 12);
            c.g[A2][0] = 0x6175_0000;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x7390));
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x8;
            c.g[A2][0] |= 0x7468;
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_c7c8)?;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x488));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_c7dc)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = 0x480000;
            c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x12b0));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A0][0] = 0xc;
            c.sw(c.g[S1][0] as u32, c.g[S0][0]);
            c.g[V0][0] = 0x480000;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x210), c.g[V1][0]);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_c810)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S1][0]);
            }
            break 'linked;
        }
        c.cycles += 21;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x210));
        frame.restore(c, S0, 0x2b0);
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        frame.restore(c, S1, 0x2a0);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        frame.restore(c, S2, 0x290);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        frame.restore(c, S3, 0x280);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        frame.restore(c, S4, 0x270);
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        frame.restore(c, S5, 0x260);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x210));
        frame.restore(c, S6, 0x250);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        frame.restore(c, S7, 0x240);
        frame.restore(c, FP, 0x230);
        frame.restore_ra(c, 0x220);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x200), c.g[A1][0]);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    driven!(gen_lobby_man_create_lists, ee::s06::f_0025a778__generated);
    driven!(hand_lobby_man_create_lists, lobby_man_create_lists);
    driven!(gen_lobby_man_login, ee::s06::f_0025c610__generated);
    driven!(hand_lobby_man_login, lobby_man_login);
    /// Lobby object at `DATA+0x1000`, session at `DATA+0x2000`, a self-linked
    /// message-list head at `DATA+0x3000`; BELibrary dummy, world, pool
    /// allocator and DLHeap arena set up like the tagsunk lobby tests.
    fn lobby_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        crate::game::fe::feasyncfile::tests::globals_13(c);
        crate::game::fe::feasyncfile::tests::pool_11(c);
        crate::game::dirtysock::tagsunk::tests::heap_init(c);
        let self_ = DATA + 0x1000;
        let head = DATA + 0x3000;
        c.bus.write32(self_ + LOBBY_SESSION, DATA + 0x2000);
        c.bus.write32(self_ + LOBBY_MSG_HEAD, head);
        c.bus.write32(head, head);
        c.bus.write32(head + 4, head);
        r
    }

    fn put_str_at(c: &mut Cpu, at: u32, seed: u32) {
        for i in 0..8u32 {
            c.bus.write8(at + i, b'a' + ((seed + i) % 26) as u8);
        }
        c.bus.write8(at + 8, 0);
    }

    #[test]
    fn lobby_man_create_lists_events() {
        check_events(
            gen_lobby_man_create_lists,
            hand_lobby_man_create_lists,
            36,
            &|c, seed| {
                let mut r = lobby_base(c, seed, 0x25a778);
                c.pc = 0x0025_a778;
                c.g[A0][0] = sx(DATA + 0x1000);
                let _m: Events = schedule(c, &mut r, seed, 0x0025_a778);
                _m
            },
        );
    }

    #[test]
    fn lobby_man_create_lobby_api_matches() {
        check(
            ee::s06::f_0025a948__generated,
            lobby_man_create_lobby_api,
            24,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x25a948);
                c.pc = 0x0025_a948;
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn lobby_man_post_chat_event_matches() {
        check(
            ee::s06::f_0025ab40__generated,
            lobby_man_post_chat_event,
            24,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x25ab40);
                c.pc = 0x0025_ab40;
                put_str_at(c, DATA + 0x6000, seed);
                put_str_at(c, DATA + 0x6100, seed.wrapping_add(3));
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x6000);
                c.g[A2][0] = sx(DATA + 0x6100);
            },
        );
    }

    #[test]
    fn lobby_man_connect_matches() {
        check(
            ee::s06::f_0025b6d8__generated,
            lobby_man_connect,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x25b6d8);
                c.pc = 0x0025_b6d8;
                let self_ = DATA + 0x1000;
                // Branch steering by seed arithmetic only: session present or
                // not, persona present or not.
                c.bus.write32(
                    self_ + LOBBY_SESSION,
                    if seed % 2 == 0 { DATA + 0x2000 } else { 0 },
                );
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x4000) };
            },
        );
    }

    #[test]
    fn lobby_man_on_connect_reply_matches() {
        check(
            ee::s06::f_0025b848__generated,
            lobby_man_on_connect_reply,
            24,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x25b848);
                c.pc = 0x0025_b848;
                let reply = DATA + 0x4000;
                c.bus
                    .write32(reply + 8, if seed % 2 == 0 { 0 } else { 0x2a });
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(reply);
            },
        );
    }

    #[test]
    fn lobby_man_create_account_matches() {
        check(
            ee::s06::f_0025b9c8__generated,
            lobby_man_create_account,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x25b9c8);
                c.pc = 0x0025_b9c8;
                let info = DATA + 0x4000;
                put_str_at(c, DATA + 0x6000, seed);
                c.bus.write32(info, DATA + 0x6000);
                c.bus.write32(info + 4, DATA + 0x6000);
                c.bus.write32(info + 0xc, DATA + 0x6000);
                c.bus.write32(info + 0x10, DATA + 0x5000);
                // Extra-field branch: word at *(info+0x10)-8 zero or not.
                c.bus
                    .write32(DATA + 0x5000 - 8, if seed % 2 == 0 { 0 } else { 0x11 });
                c.bus.write32(info + 0x28, seed.wrapping_mul(3));
                c.bus.write32(info + 0x2c, seed.wrapping_mul(5));
                c.bus.write32(info + 0x30, seed.wrapping_mul(7));
                c.bus
                    .write32(info + 0x34, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(info + 0x38, if seed % 4 == 0 { 0 } else { 1 });
                c.bus
                    .write32(info + 0x3c, if seed % 5 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(info);
            },
        );
    }

    #[test]
    fn lobby_man_on_create_account_reply_matches() {
        check(
            ee::s06::f_0025bd30__generated,
            lobby_man_on_create_account_reply,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x25bd30);
                c.pc = 0x0025_bd30;
                let reply = DATA + 0x4000;
                let status = match seed % 4 {
                    0 => 0,
                    1 => 0x746f_6f79,
                    2 => 0x6475_706c,
                    _ => 0x2a,
                };
                c.bus.write32(reply + 8, status);
                c.bus.write32(reply + 0xc, DATA + 0x5000);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(reply);
            },
        );
    }

    #[test]
    fn lobby_man_send_lost_by_email_matches() {
        check(
            ee::s06::f_0025c1e0__generated,
            lobby_man_send_lost_by_email,
            24,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x25c1e0);
                c.pc = 0x0025_c1e0;
                put_str_at(c, DATA + 0x6000, seed);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x6000);
            },
        );
    }

    #[test]
    fn lobby_man_send_lost_by_name_matches() {
        check(
            ee::s06::f_0025c3f8__generated,
            lobby_man_send_lost_by_name,
            24,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x25c3f8);
                c.pc = 0x0025_c3f8;
                put_str_at(c, DATA + 0x6000, seed);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x6000);
            },
        );
    }

    #[test]
    fn lobby_man_login_events() {
        check_events(gen_lobby_man_login, hand_lobby_man_login, 36, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x25c610);
            c.pc = 0x0025_c610;
            let self_ = DATA + 0x1000;
            put_str_at(c, DATA + 0x6000, seed);
            c.bus.write32(self_ + 0x48, DATA + 0x6000);
            c.bus.write32(self_ + 0x4c, DATA + 0x6000);
            // Connection flags word: plain (==1) or capability bits.
            let conn = DATA + 0x7000;
            c.bus.write32(GAME_GP.wrapping_sub(0x238), conn);
            let flags = match seed % 5 {
                0 => 1,
                1 => 0x1c,
                2 => 0x14,
                3 => 0x8,
                _ => 0,
            };
            c.bus.write32(conn + 0x30, flags);
            // Slot table: first slot flagged or clear (scan runs out).
            let slots = DATA + 0x8000;
            c.bus.write32(GAME_GP.wrapping_add(0x300), slots);
            c.bus.write32(GAME_GP.wrapping_add(0x304), DATA + 0x2000);
            c.bus
                .write32(slots + 0x88, if seed % 3 == 0 { 1 } else { 0 });
            c.g[A0][0] = sx(self_);
            let _m: Events = schedule(c, &mut r, seed, 0x0025_c610);
            _m
        });
    }
}
