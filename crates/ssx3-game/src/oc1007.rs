//! Staging module oc1007 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1007`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0025_9b10, lobby_man_on_message, "LobbyMan_OnMessage")];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `cDirtysock_tag_TagFieldFind` (0x3eb6d8), via `call_indirect`.
    pub const TAG_FIELD_FIND: u32 = 0x003e_b6d8;
    /// `TagFieldGetString4` (0x3ed378), via `call_indirect`.
    pub const TAG_FIELD_GET_STRING4: u32 = 0x003e_d378;
    /// `sscanf` (0x4178b0), via `call_indirect`.
    pub const SSCANF_4178B0: u32 = 0x0041_78b0;
    /// `strcmp`-like (0x4165a8), via `call_indirect`.
    pub const STR_CMP_4165A8: u32 = 0x0041_65a8;
    /// String compare (0x41aa88, case-folding), via `call_indirect`.
    pub const STR_CMP_41AA88: u32 = 0x0041_aa88;
    /// Buddy-name lookup (0x2623b0), via `call_indirect`.
    pub const BUDDY_LOOKUP_2623B0: u32 = 0x0026_23b0;
    /// Challenge send (0x262af0), via `call_indirect`.
    pub const CHAL_SEND_262AF0: u32 = 0x0026_2af0;
    /// Buddy check (0x263128), via `call_indirect`.
    pub const BUDDY_CHECK_263128: u32 = 0x0026_3128;
    /// Buddy remove (0x263018), via `call_indirect`.
    pub const BUDDY_REMOVE_263018: u32 = 0x0026_3018;
    /// `cMemMan_alloc` / operator new (0x317d70), via `call_indirect`.
    pub const MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    /// `cBXString` ctor (0x3182d8), via `call_indirect`.
    pub const BXSTRING_CTOR_3182D8: u32 = 0x0031_82d8;
    /// `func_00318630` (BXString concat result, 0x318630), via `call_indirect`.
    pub const BXSTRING_OP_318630: u32 = 0x0031_8630;
    /// `cBXString` dtor (0x318260), via `call_indirect`.
    pub const BXSTRING_DTOR_318260: u32 = 0x0031_8260;

    pub const TAG_FIELD_GET_STRING_UNESCAPED: Callee = Callee {
        addr: 0x0025_9310,
        f: ee::s06::TagFieldGetStringUnescaped,
    };
    pub const COMM_SYSTEM_GET_VERSION_TAG: Callee = Callee {
        addr: 0x0025_5a10,
        f: ee::s06::cCommSystem_GetVersionTag,
    };
    pub const LOBBY_MAN_SEND_PRIVATE_MESSAGE: Callee = Callee {
        addr: 0x0025_fb80,
        f: ee::s06::LobbyMan_SendPrivateMessage,
    };
    pub const LOBBY_MAN_CANCEL_CHALLENGE: Callee = Callee {
        addr: 0x0025_f628,
        f: ee::s06::LobbyMan_CancelChallenge,
    };
    pub const LOBBY_MAN_CHAL_REQUEST_HOST: Callee = Callee {
        addr: 0x0025_fa48,
        f: ee::s06::LobbyMan_ChalRequestHost,
    };
    pub const LOBBY_MAN_CHAL_REQUEST_JOIN: Callee = Callee {
        addr: 0x0025_fad0,
        f: ee::s06::LobbyMan_ChalRequestJoin,
    };
    pub const LOBBY_MAN_RESET_CHAL_STATE: Callee = Callee {
        addr: 0x0025_f858,
        f: ee::s06::LobbyMan_ResetChalState,
    };
    pub const LOBBY_MAN_POST_CHAT_EVENT: Callee = Callee {
        addr: 0x0025_ab40,
        f: ee::s06::LobbyMan_PostChatEvent,
    };
    pub const LOBBY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::LobbyMan_Disconnect,
    };
    pub const BUDDY_MAN_CREATE: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::BuddyMan_Create,
    };
    pub const BUDDY_MAN_IS_CONNECTED: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::BuddyMan_IsConnected,
    };
    pub const BUDDY_MAN_RECONNECT: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::BuddyMan_Reconnect,
    };
}

/// The buddy-manager global (`gp+0x238`, `D_004A3328`).
pub(crate) const LOBBY_ON_MESSAGE_BUDDY_GP: i32 = 0x238;
/// The lobby-instance global (`gp-0xc8`, `D_004A3028`).
pub(crate) const LOBBY_ON_MESSAGE_LOBBY_GP: i32 = -0xc8;
/// The comm-system global (`gp-0x238`, `D_004A2EB8`).
pub(crate) const LOBBY_ON_MESSAGE_COMM_GP: i32 = -0x238;

/// The inlined buddy-manager getter (`Mgr_9B10` in the reference): create
/// the manager when the global is null, reconnect when it is not
/// connected, and disconnect the lobby when it is still null. Each call
/// site passes its own four return addresses, exactly as the generated
/// body inlines them. No loop and no event check: the three tests are
/// sequential, as in the generated blocks.
fn lobby_man_on_message_buddy_mgr(
    c: &mut Cpu,
    ret_create: u32,
    ret_is_connected: u32,
    ret_reconnect: u32,
    ret_disconnect: u32,
) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32)).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        callee::BUDDY_MAN_CREATE.call(c, ret_create)?;
    }
    c.cycles += 4;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32)).lw(c);
    callee::BUDDY_MAN_IS_CONNECTED.call(c, ret_is_connected)?;
    let connected = c.g[V0][0] != 0;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32)).lw(c);
    if !connected {
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32)).lw(c);
        callee::BUDDY_MAN_RECONNECT.call(c, ret_reconnect)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32)).lw(c);
    }
    c.cycles += 2;
    let mgr_present = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32)).lw(c);
    if !mgr_present {
        c.cycles += 3;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_ON_MESSAGE_LOBBY_GP as u32)).lw(c);
        callee::LOBBY_MAN_DISCONNECT.call(c, ret_disconnect)?;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32)).lw(c);
    }
    Ok(())
}

/// `void LobbyMan_OnMessage(LobbyMan* self, int* msg)` (0x259b10): handle
/// an incoming lobby message. The sender name and text are first pulled
/// from the message's tag list, then the message kind at msg+4 dispatches:
/// `"priv"` parses a private command line out of the text (subject to the
/// msg+8 flag word) and answers with private messages, challenge
/// host/join/cancel requests or small posted event objects, `"chat"`
/// forwards the text to the chat event poster unless the sender is a
/// known buddy, and `"cast"` does the same after a string transform.
/// Other kinds are ignored. The frame is 0x430 bytes: name at sp+0x40,
/// text at sp+0x140, the parsed command at sp+0x240 and its signature at
/// sp+0x340, with twelve parsed integers at sp+0x3A0..0x3D0.
pub fn lobby_man_on_message(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x430);
        'b_25a1b8: {
            'b_25a1b4: {
                'b_25a1b0: {
                    'b_25a160: {
                        'b_25a0f4: {
                            c.cycles += 36;
                            frame.save(c, S0, 0x420);
                            frame.save(c, S1, 0x410);
                            frame.save(c, S2, 0x400);
                            c.g[S1][0] = c.g[A1][0];
                            frame.save(c, S3, 0x3f0);
                            c.g[S2][0] = c.g[A0][0];
                            frame.save(c, S4, 0x3e0);
                            c.g[A1][0] = 0x4a0000;
                            frame.save_ra(c, 0x3d0);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f38));
                            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
                            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_9b44)?;
                            c.g[S0][0] = 0x4a0000;
                            c.g[A1][0] = sx(frame.at(c, 0x40));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2f40));
                            c.g[A2][0] = 0x100;
                            c.g[A3][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0025_9b60)?;
                            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
                            c.g[A1][0] = 0x4a0000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f48));
                            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_9b70)?;
                            c.g[S3][0] = sx(frame.at(c, 0x140));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A3][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S3][0];
                            c.g[A2][0] = 0x100;
                            c.g[S4][0] = c.g[S3][0];
                            callee::TAG_FIELD_GET_STRING_UNESCAPED.call(c, 0x0025_9b8c)?;
                            // Switch on the message kind at msg+4.
                            c.g[V0][0] = 0x6368_0000;
                            c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x4)).lw(c);
                            c.g[V0][0] = c.g[V0][0] | 0x6174;
                            let is_chat = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if is_chat {
                                break 'b_25a0f4;
                            }
                            'b_259bc0: {
                                c.cycles += 2;
                                let above_chat = c.g[V0][0] != 0;
                                c.g[V0][0] = 0x7072_0000;
                                if above_chat {
                                    break 'b_259bc0;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = 0x6361_0000;
                                c.g[V0][0] = c.g[V0][0] | 0x7374;
                                let is_cast = c.g[V1][0] == c.g[V0][0];
                                frame.restore(c, S0, 0x420);
                                if is_cast {
                                    break 'b_25a160;
                                }
                                c.cycles += 2;
                                frame.restore(c, S1, 0x410);
                                break 'b_25a1b8;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V0][0] | 0x6976;
                            let not_priv = c.g[V1][0] != c.g[V0][0];
                            frame.restore(c, S0, 0x420);
                            if not_priv {
                                break 'b_25a1b4;
                            }
                            // "priv": the flag word at msg+8 gates the parse.
                            c.cycles += 5;
                            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
                            c.g[V0][0] = 0x20_0000;
                            c.g[V0][0] = c.g[A1][0] & c.g[V0][0];
                            let flag_200000 = c.g[V0][0] != 0;
                            frame.restore(c, S1, 0x410);
                            if flag_200000 {
                                break 'b_25a1b8;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = 0x4000_0000;
                            c.g[V0][0] = c.g[A1][0] & c.g[V0][0];
                            let no_flag_40000000 = c.g[V0][0] == 0;
                            c.g[T0][0] = sx(frame.at(c, 0x3bc));
                            if no_flag_40000000 {
                                break 'b_25a1b8;
                            }
                            'b_259e64: {
                                'b_259e54: {
                                    'b_259de0: {
                                        c.cycles += 32;
                                        c.g[T1][0] = sx(frame.at(c, 0x3c0));
                                        c.g[T2][0] = sx(frame.at(c, 0x3c4));
                                        c.g[T3][0] = sx(frame.at(c, 0x3c8));
                                        c.g[S0][0] = sx(frame.at(c, 0x240));
                                        c.g[V0][0] = sx(frame.at(c, 0x3b4));
                                        c.g[V1][0] = sx(frame.at(c, 0x3b8));
                                        c.g[S1][0] = sx(frame.at(c, 0x340));
                                        c.g[T4][0] = sx(frame.at(c, 0x3cc));
                                        Addr(frame.at(c, 0x10)).sw(c, c.g[T0][0]);
                                        Addr(frame.at(c, 0x20)).sw(c, c.g[T1][0]);
                                        c.g[A1][0] = 0x480000;
                                        Addr(frame.at(c, 0x28)).sw(c, c.g[T2][0]);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x620));
                                        Addr(frame.at(c, 0x30)).sw(c, c.g[T3][0]);
                                        c.g[A0][0] = c.g[S3][0];
                                        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
                                        c.g[A2][0] = c.g[S0][0];
                                        Addr(frame.at(c, 0x8)).sw(c, c.g[V1][0]);
                                        c.g[A3][0] = sx(frame.at(c, 0x3a0));
                                        c.g[T0][0] = sx(frame.at(c, 0x3a4));
                                        c.g[T1][0] = sx(frame.at(c, 0x3a8));
                                        c.g[T2][0] = sx(frame.at(c, 0x3ac));
                                        c.g[T3][0] = sx(frame.at(c, 0x3b0));
                                        Addr(frame.at(c, 0x38)).sw(c, c.g[T4][0]);
                                        Addr(frame.at(c, 0x18)).sw(c, c.g[S1][0]);
                                        call_ptr(c, callee::SSCANF_4178B0, 0x0025_9c58)?;
                                        c.g[A1][0] = 0x480000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x610));
                                        call_ptr(c, callee::STR_CMP_4165A8, 0x0025_9c68)?;
                                        let cmd_differs = c.g[V0][0] != 0;
                                        c.g[A1][0] = 0x480000;
                                        if cmd_differs {
                                            break 'b_259de0;
                                        }
                                        'b_259c98: {
                                            c.cycles += 7;
                                            c.g[A0][0] = Addr(
                                                r32(c, GP)
                                                    .wrapping_add(LOBBY_ON_MESSAGE_COMM_GP as u32),
                                            )
                                            .lw(c);
                                            callee::COMM_SYSTEM_GET_VERSION_TAG
                                                .call(c, 0x0025_9c78)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, callee::STR_CMP_4165A8, 0x0025_9c84)?;
                                            let sig_equal = c.g[V0][0] == 0;
                                            c.g[A2][0] = 0x480000;
                                            if sig_equal {
                                                break 'b_259c98;
                                            }
                                            c.cycles += 3;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A2][0] =
                                                sx((c.g[A2][0] as u32).wrapping_add(0x7e8));
                                            break 'b_259e54;
                                        }
                                        lobby_man_on_message_buddy_mgr(
                                            c,
                                            0x0025_9cac,
                                            0x0025_9cb4,
                                            0x0025_9cc4,
                                            0x0025_9cd8,
                                        )?;
                                        'b_259cf8: {
                                            c.cycles += 4;
                                            c.g[A1][0] = sx(frame.at(c, 0x40));
                                            call_ptr(c, callee::BUDDY_LOOKUP_2623B0, 0x0025_9ce4)?;
                                            let buddy_known = c.g[V0][0] == 0;
                                            c.g[A2][0] = 0x4a0000;
                                            if buddy_known {
                                                break 'b_259cf8;
                                            }
                                            c.cycles += 3;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A2][0] =
                                                sx((c.g[A2][0] as u32).wrapping_add(0x2f50));
                                            break 'b_259e54;
                                        }
                                        'b_259d34: {
                                            c.cycles += 4;
                                            c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                            c.g[V0][0] = 0x16;
                                            let not_chal_state = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = Addr(
                                                r32(c, GP)
                                                    .wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32),
                                            )
                                            .lw(c);
                                            if not_chal_state {
                                                break 'b_259d34;
                                            }
                                            c.cycles += 5;
                                            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x98)).lw(c);
                                            c.g[A1][0] = sx(frame.at(c, 0x40));
                                            call_ptr(c, callee::STR_CMP_4165A8, 0x0025_9d14)?;
                                            let name_differs = c.g[V0][0] != 0;
                                            c.g[V0][0] = Addr(
                                                r32(c, GP)
                                                    .wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32),
                                            )
                                            .lw(c);
                                            if name_differs {
                                                break 'b_259d34;
                                            }
                                            c.cycles += 6;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = 0x1;
                                            c.g[A2][0] = 0x1;
                                            callee::LOBBY_MAN_CANCEL_CHALLENGE
                                                .call(c, 0x0025_9d2c)?;
                                            frame.restore(c, S0, 0x420);
                                            break 'b_25a1b4;
                                        }
                                        lobby_man_on_message_buddy_mgr(
                                            c,
                                            0x0025_9d44,
                                            0x0025_9d4c,
                                            0x0025_9d5c,
                                            0x0025_9d70,
                                        )?;
                                        c.cycles += 27;
                                        c.g[A1][0] = sx(frame.at(c, 0x40));
                                        c.g[T1][0] = Addr(frame.at(c, 0x3ac)).lw(c);
                                        c.g[T2][0] = Addr(frame.at(c, 0x3b0)).lw(c);
                                        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                                        c.g[T3][0] = Addr(frame.at(c, 0x3b4)).lw(c);
                                        c.g[T1][0] = (0u64 < c.g[T1][0]) as u64;
                                        c.g[V1][0] = Addr(frame.at(c, 0x3b8)).lw(c);
                                        c.g[T2][0] = (0u64 < c.g[T2][0]) as u64;
                                        c.g[T7][0] = Addr(frame.at(c, 0x3c0)).lw(c);
                                        c.g[T3][0] = (0u64 < c.g[T3][0]) as u64;
                                        c.g[T4][0] = Addr(frame.at(c, 0x3c4)).lw(c);
                                        c.g[T5][0] = Addr(frame.at(c, 0x3c8)).lw(c);
                                        c.g[T6][0] = Addr(frame.at(c, 0x3cc)).lw(c);
                                        c.g[A0][0] = Addr(
                                            r32(c, GP)
                                                .wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32),
                                        )
                                        .lw(c);
                                        c.g[A2][0] = Addr(frame.at(c, 0x3a0)).lw(c);
                                        c.g[A3][0] = Addr(frame.at(c, 0x3a4)).lw(c);
                                        c.g[T0][0] = Addr(frame.at(c, 0x3a8)).lw(c);
                                        Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
                                        Addr(frame.at(c, 0x8)).sw(c, c.g[V0][0]);
                                        Addr(frame.at(c, 0x10)).sw(c, c.g[S4][0]);
                                        Addr(frame.at(c, 0x18)).sw(c, c.g[T7][0]);
                                        Addr(frame.at(c, 0x20)).sw(c, c.g[T4][0]);
                                        Addr(frame.at(c, 0x28)).sw(c, c.g[T5][0]);
                                        Addr(frame.at(c, 0x30)).sw(c, c.g[T6][0]);
                                        call_ptr(c, callee::CHAL_SEND_262AF0, 0x0025_9dd8)?;
                                        frame.restore(c, S0, 0x420);
                                        break 'b_25a1b4;
                                    }
                                    // Second command word (0x480670).
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x670));
                                    call_ptr(c, callee::STR_CMP_4165A8, 0x0025_9dec)?;
                                    if c.g[V0][0] != 0 {
                                        c.g[A1][0] = 0x4a0000;
                                        break 'b_259e64;
                                    }
                                    'b_259e30: {
                                        c.cycles += 4;
                                        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                        c.g[V0][0] = 0x16;
                                        let not_state16 = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x17;
                                        if not_state16 {
                                            break 'b_259e30;
                                        }
                                        'b_259e2c: {
                                            c.cycles += 5;
                                            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x98)).lw(c);
                                            c.g[A1][0] = sx(frame.at(c, 0x40));
                                            call_ptr(c, callee::STR_CMP_4165A8, 0x0025_9e10)?;
                                            if c.g[V0][0] != 0 {
                                                c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                                break 'b_259e2c;
                                            }
                                            c.cycles += 5;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = sx(frame.at(c, 0x40));
                                            callee::LOBBY_MAN_CHAL_REQUEST_HOST
                                                .call(c, 0x0025_9e24)?;
                                            frame.restore(c, S0, 0x420);
                                            break 'b_25a1b4;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = 0x17;
                                    }
                                    'b_259e4c: {
                                        c.cycles += 2;
                                        let not_state17 = c.g[V1][0] != c.g[V0][0];
                                        c.g[A2][0] = 0x480000;
                                        if not_state17 {
                                            break 'b_259e4c;
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[A1][0] = sx(frame.at(c, 0x40));
                                        callee::LOBBY_MAN_CHAL_REQUEST_JOIN.call(c, 0x0025_9e44)?;
                                        frame.restore(c, S0, 0x420);
                                        break 'b_25a1b4;
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7f8));
                                    break 'b_259e54;
                                }
                                // Shared private-message tail (0x4807e8 /
                                // 0x4a2f50 / 0x4807f8 in a2).
                                c.cycles += 4;
                                c.g[A1][0] = sx(frame.at(c, 0x40));
                                callee::LOBBY_MAN_SEND_PRIVATE_MESSAGE.call(c, 0x0025_9e5c)?;
                                frame.restore(c, S0, 0x420);
                                break 'b_25a1b4;
                            }
                            // Third to sixth command words: each may post
                            // a small event object to the list at self+0xF0.
                            'b_25a094: {
                                'b_259eb8: {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ef8));
                                    call_ptr(c, callee::STR_CMP_4165A8, 0x0025_9e70)?;
                                    let differs = c.g[V0][0] != 0;
                                    c.g[A1][0] = 0x480000;
                                    if differs {
                                        break 'b_259eb8;
                                    }
                                    'b_259e90: {
                                        c.cycles += 4;
                                        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                        c.g[V0][0] = 0x16;
                                        let not_state16 = c.g[V1][0] != c.g[V0][0];
                                        c.g[A1][0] = 0x480000;
                                        if not_state16 {
                                            break 'b_259e90;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x1;
                                        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
                                    }
                                    c.cycles += 10;
                                    c.g[A2][0] = 0x2000_0000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                    c.g[A0][0] = 0x8;
                                    c.g[A3][0] = 0;
                                    call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_9ea4)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A2][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
                                    c.g[V0][0] = 0x480000;
                                    c.g[V1][0] = 0xed;
                                    break 'b_25a094;
                                }
                                'b_259ff0: {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x660));
                                    call_ptr(c, callee::STR_CMP_4165A8, 0x0025_9ec4)?;
                                    let differs = c.g[V0][0] != 0;
                                    c.g[A1][0] = 0x4a0000;
                                    if differs {
                                        break 'b_259ff0;
                                    }
                                    lobby_man_on_message_buddy_mgr(
                                        c,
                                        0x0025_9ee0,
                                        0x0025_9ee8,
                                        0x0025_9ef8,
                                        0x0025_9f0c,
                                    )?;
                                    'b_259f68: {
                                        c.cycles += 4;
                                        c.g[A1][0] = sx(frame.at(c, 0x40));
                                        call_ptr(c, callee::BUDDY_CHECK_263128, 0x0025_9f18)?;
                                        let check_zero = c.g[V0][0] == 0;
                                        c.g[V0][0] = Addr(
                                            r32(c, GP)
                                                .wrapping_add(LOBBY_ON_MESSAGE_BUDDY_GP as u32),
                                        )
                                        .lw(c);
                                        if check_zero {
                                            break 'b_259f68;
                                        }
                                        lobby_man_on_message_buddy_mgr(
                                            c,
                                            0x0025_9f30,
                                            0x0025_9f38,
                                            0x0025_9f48,
                                            0x0025_9f5c,
                                        )?;
                                        c.cycles += 2;
                                        c.g[A1][0] = sx(frame.at(c, 0x40));
                                        call_ptr(c, callee::BUDDY_REMOVE_263018, 0x0025_9f68)?;
                                    }
                                    'b_259fc0: {
                                        'b_259fa0: {
                                            c.cycles += 8;
                                            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x98)).lw(c);
                                            c.g[A1][0] = sx(frame.at(c, 0x40));
                                            call_ptr(c, callee::STR_CMP_41AA88, 0x0025_9f74)?;
                                            c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
                                            c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                            c.g[V0][0] = 0x18;
                                            let not_state18 = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = 0;
                                            if not_state18 {
                                                break 'b_259fa0;
                                            }
                                            c.cycles += 2;
                                            let gone_zero = c.g[A1][0] == 0;
                                            if gone_zero {
                                                break 'b_259fc0;
                                            }
                                            c.cycles += 4;
                                            c.g[A0][0] = c.g[S2][0];
                                            callee::LOBBY_MAN_RESET_CHAL_STATE
                                                .call(c, 0x0025_9f98)?;
                                            c.g[A0][0] = 0x1;
                                            break 'b_259fc0;
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x16));
                                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                        if c.g[V0][0] == 0 {
                                            c.g[A0][0] = 0x1;
                                            break 'b_259fc0;
                                        }
                                        c.cycles += 2;
                                        let gone_zero = c.g[A1][0] == 0;
                                        c.g[V0][0] = 0x1;
                                        if gone_zero {
                                            break 'b_259fc0;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = 0x1;
                                        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
                                    }
                                    c.cycles += 2;
                                    let no_send = c.g[A0][0] == 0;
                                    c.g[A2][0] = 0x2000_0000;
                                    if no_send {
                                        break 'b_25a1b0;
                                    }
                                    c.cycles += 10;
                                    c.g[A1][0] = 0x480000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                    c.g[A0][0] = 0x8;
                                    c.g[A3][0] = 0;
                                    call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_9fdc)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A2][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
                                    c.g[V0][0] = 0x480000;
                                    c.g[V1][0] = 0xee;
                                    break 'b_25a094;
                                }
                                'b_25a044: {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f00));
                                    call_ptr(c, callee::STR_CMP_4165A8, 0x0025_9ffc)?;
                                    let differs = c.g[V0][0] != 0;
                                    c.g[A1][0] = 0x480000;
                                    if differs {
                                        break 'b_25a044;
                                    }
                                    'b_25a01c: {
                                        c.cycles += 4;
                                        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                        c.g[V0][0] = 0x16;
                                        let not_state16 = c.g[V1][0] != c.g[V0][0];
                                        c.g[A1][0] = 0x480000;
                                        if not_state16 {
                                            break 'b_25a01c;
                                        }
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x1;
                                        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
                                    }
                                    c.cycles += 10;
                                    c.g[A2][0] = 0x2000_0000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                    c.g[A0][0] = 0x8;
                                    c.g[A3][0] = 0;
                                    call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_a030)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A2][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
                                    c.g[V0][0] = 0x480000;
                                    c.g[V1][0] = 0xf0;
                                    break 'b_25a094;
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x680));
                                call_ptr(c, callee::STR_CMP_4165A8, 0x0025_a050)?;
                                let differs = c.g[V0][0] != 0;
                                frame.restore(c, S0, 0x420);
                                if differs {
                                    break 'b_25a1b4;
                                }
                                'b_25a070: {
                                    c.cycles += 4;
                                    c.g[V1][0] = Addr(r32(c, S2)).lw(c);
                                    c.g[V0][0] = 0x16;
                                    let not_state16 = c.g[V1][0] != c.g[V0][0];
                                    c.g[A1][0] = 0x480000;
                                    if not_state16 {
                                        break 'b_25a070;
                                    }
                                    c.cycles += 2;
                                    c.g[V0][0] = 0x1;
                                    Addr(r32(c, S2)).sw(c, c.g[V0][0]);
                                }
                                c.cycles += 9;
                                c.g[A2][0] = 0x2000_0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                c.g[A0][0] = 0x8;
                                c.g[A3][0] = 0;
                                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_a084)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A2][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
                                c.g[V0][0] = 0x480000;
                                c.g[V1][0] = 0xf2;
                            }
                            // Post the event object in s0 (kind in v1,
                            // list head saved from self+0xF0 in a2).
                            {
                                c.cycles += 14;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                                Addr(r32(c, S0)).sw(c, c.g[V1][0]);
                                Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
                                c.g[A1][0] = 0x480000;
                                Addr(frame.at(c, 0x390)).sw(c, c.g[A2][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                c.g[A0][0] = 0xc;
                                c.g[A2][0] = 0x2000_0000;
                                c.g[A3][0] = 0;
                                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_a0bc)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                                if c.g[V0][0] != 0 {
                                    Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S0][0]);
                                }
                            }
                            c.cycles += 10;
                            c.g[V0][0] = Addr(frame.at(c, 0x390)).lw(c);
                            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                            Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
                            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                            Addr(r32(c, A0)).sw(c, c.g[A1][0]);
                            c.g[V0][0] = Addr(frame.at(c, 0x390)).lw(c);
                            Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
                            Addr(frame.at(c, 0x380)).sw(c, c.g[A1][0]);
                            break 'b_25a1b0;
                        }
                        // "chat": post unless the sender is a known buddy.
                        lobby_man_on_message_buddy_mgr(
                            c,
                            0x0025_a108,
                            0x0025_a110,
                            0x0025_a120,
                            0x0025_a134,
                        )?;
                        c.cycles += 4;
                        c.g[A1][0] = sx(frame.at(c, 0x40));
                        call_ptr(c, callee::BUDDY_LOOKUP_2623B0, 0x0025_a140)?;
                        let buddy_known = c.g[V0][0] != 0;
                        frame.restore(c, S0, 0x420);
                        if buddy_known {
                            break 'b_25a1b4;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A1][0] = sx(frame.at(c, 0x40));
                        callee::LOBBY_MAN_POST_CHAT_EVENT.call(c, 0x0025_a158)?;
                        frame.restore(c, S0, 0x420);
                        break 'b_25a1b4;
                    }
                    // "cast": transform the text, then post it as chat.
                    c.cycles += 20;
                    c.g[S0][0] = sx(frame.at(c, 0x250));
                    c.g[A1][0] = 0x480000;
                    c.g[S1][0] = sx(frame.at(c, 0x240));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x808));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::BXSTRING_CTOR_3182D8, 0x0025_a178)?;
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, callee::BXSTRING_OP_318630, 0x0025_a188)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, callee::BXSTRING_DTOR_318260, 0x0025_a194)?;
                    c.g[A2][0] = Addr(frame.at(c, 0x240)).lw(c);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx(frame.at(c, 0x40));
                    callee::LOBBY_MAN_POST_CHAT_EVENT.call(c, 0x0025_a1a4)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, callee::BXSTRING_DTOR_318260, 0x0025_a1b0)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x420);
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x410);
        }
        c.cycles += 6;
        frame.restore(c, S2, 0x400);
        frame.restore(c, S3, 0x3f0);
        frame.restore(c, S4, 0x3e0);
        frame.restore_ra(c, 0x3d0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fall_back, hook};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use crate::game::dirtysock::tags::tests::put_str;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word for this module's callee fakes, in the machine's RAM.
    const ONMSG_CTRL: u32 = DATA + 0xf_ff00;
    const ONMSG_MAGIC: u32 = 0x6f63_1007;
    /// Per-fake cells: count at +0x00, unwind index at +0x04, forced
    /// `v0` results at +0x08 + 4*idx (idx modulo 16).
    const R_STRCMP: u32 = DATA + 0xf_ff10;
    const R_BUDDY2623: u32 = DATA + 0xf_ff60;
    const R_CHECK263128: u32 = DATA + 0xf_ffb0;
    const R_GETSIG: u32 = DATA + 0xf_fe00;
    const R_SENDPRIV: u32 = DATA + 0xf_fe50;
    const R_HOST: u32 = DATA + 0xf_fea0;
    const R_JOIN: u32 = DATA + 0xf_fef0;
    const R_RESET: u32 = DATA + 0xf_fd00;
    const R_DISCONNECT: u32 = DATA + 0xf_fd50;
    const R_CREATE: u32 = DATA + 0xf_fda0;
    const R_ISCONN: u32 = DATA + 0xf_fdf0;
    const R_RECONN: u32 = DATA + 0xf_fc00;
    const R_ALLOC: u32 = DATA + 0xf_fc50;
    const R_BXCTOR: u32 = DATA + 0xf_fca0;
    const R_BXOP: u32 = DATA + 0xf_fcf0;
    const R_BXDTOR: u32 = DATA + 0xf_fb00;

    macro_rules! onmsg_gated {
        ($name:ident, $prev:ident, $cell:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(ONMSG_CTRL) == ONMSG_MAGIC {
                    let idx = c.bus.read32($cell);
                    c.bus.write32($cell, idx + 1);
                    if idx == c.bus.read32($cell + 4) {
                        c.pc = 0xdead_0000 | idx << 4;
                        return;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx(c.bus.read32($cell + 8 + 4 * (idx & 15)));
                    c.pc = r32(c, RA);
                    return;
                }
                fall_back(c, &$prev, $gen)
            }
        };
    }

    onmsg_gated!(
        onmsg_fake_strcmp,
        ONMSG_PREV_STRCMP,
        R_STRCMP,
        ee::s15::f_004165a8__generated
    );
    onmsg_gated!(
        onmsg_fake_buddy2623,
        ONMSG_PREV_BUDDY2623,
        R_BUDDY2623,
        ee::s07::f_002623b0__generated
    );
    onmsg_gated!(
        onmsg_fake_check263128,
        ONMSG_PREV_CHECK263128,
        R_CHECK263128,
        ee::s07::f_00263128__generated
    );
    onmsg_gated!(
        onmsg_fake_getsig,
        ONMSG_PREV_GETSIG,
        R_GETSIG,
        ee::s06::f_00255a10__generated
    );
    onmsg_gated!(
        onmsg_fake_sendpriv,
        ONMSG_PREV_SENDPRIV,
        R_SENDPRIV,
        ee::s06::f_0025fb80__generated
    );
    onmsg_gated!(
        onmsg_fake_host,
        ONMSG_PREV_HOST,
        R_HOST,
        ee::s06::f_0025fa48__generated
    );
    onmsg_gated!(
        onmsg_fake_join,
        ONMSG_PREV_JOIN,
        R_JOIN,
        ee::s06::f_0025fad0__generated
    );
    onmsg_gated!(
        onmsg_fake_reset,
        ONMSG_PREV_RESET,
        R_RESET,
        ee::s06::f_0025f858__generated
    );
    onmsg_gated!(
        onmsg_fake_disconnect,
        ONMSG_PREV_DISCONNECT,
        R_DISCONNECT,
        ee::s06::f_0025b800__generated
    );
    onmsg_gated!(
        onmsg_fake_create,
        ONMSG_PREV_CREATE,
        R_CREATE,
        ee::s06::f_00260f80__generated
    );
    onmsg_gated!(
        onmsg_fake_isconn,
        ONMSG_PREV_ISCONN,
        R_ISCONN,
        ee::s06::f_00261460__generated
    );
    onmsg_gated!(
        onmsg_fake_reconn,
        ONMSG_PREV_RECONN,
        R_RECONN,
        ee::s06::f_002613c8__generated
    );
    onmsg_gated!(
        onmsg_fake_alloc,
        ONMSG_PREV_ALLOC,
        R_ALLOC,
        ee::s10::f_00317d70__generated
    );
    onmsg_gated!(
        onmsg_fake_bxctor,
        ONMSG_PREV_BXCTOR,
        R_BXCTOR,
        ee::s10::f_003182d8__generated
    );
    onmsg_gated!(
        onmsg_fake_bxop,
        ONMSG_PREV_BXOP,
        R_BXOP,
        ee::s10::f_00318630__generated
    );
    onmsg_gated!(
        onmsg_fake_bxdtor,
        ONMSG_PREV_BXDTOR,
        R_BXDTOR,
        ee::s10::f_00318260__generated
    );

    fn onmsg_install() {
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            onmsg_fake_strcmp,
            &ONMSG_PREV_STRCMP,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_002623B0),
            onmsg_fake_buddy2623,
            &ONMSG_PREV_BUDDY2623,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00263128),
            onmsg_fake_check263128,
            &ONMSG_PREV_CHECK263128,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_00255A10),
            onmsg_fake_getsig,
            &ONMSG_PREV_GETSIG,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025FB80),
            onmsg_fake_sendpriv,
            &ONMSG_PREV_SENDPRIV,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025FA48),
            onmsg_fake_host,
            &ONMSG_PREV_HOST,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025FAD0),
            onmsg_fake_join,
            &ONMSG_PREV_JOIN,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025F858),
            onmsg_fake_reset,
            &ONMSG_PREV_RESET,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025B800),
            onmsg_fake_disconnect,
            &ONMSG_PREV_DISCONNECT,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_00260F80),
            onmsg_fake_create,
            &ONMSG_PREV_CREATE,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261460),
            onmsg_fake_isconn,
            &ONMSG_PREV_ISCONN,
        );
        hook(
            addr_of_mut!(ee::s06::OVERRIDE_002613C8),
            onmsg_fake_reconn,
            &ONMSG_PREV_RECONN,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            onmsg_fake_alloc,
            &ONMSG_PREV_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            onmsg_fake_bxctor,
            &ONMSG_PREV_BXCTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            onmsg_fake_bxop,
            &ONMSG_PREV_BXOP,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            onmsg_fake_bxdtor,
            &ONMSG_PREV_BXDTOR,
        );
    }

    const ONMSG_SELF: u32 = DATA + 0x1000;
    const ONMSG_MSG: u32 = DATA + 0x1800;
    const ONMSG_MGR: u32 = DATA + 0x2000;
    const ONMSG_HEAD: u32 = DATA + 0x3000;
    const ONMSG_TAGS: u32 = DATA + 0x4000;
    const ONMSG_PEER: u32 = DATA + 0x5000;

    fn onmsg_results(c: &mut Cpu, cell: u32, results: &[u32]) {
        c.bus.write32(cell + 4, u32::MAX);
        for (i, v) in results.iter().enumerate() {
            c.bus.write32(cell + 8 + 4 * i as u32, *v);
        }
    }

    /// Machine for one scenario: game gp, BELibrary dummy at gp-0x1f04,
    /// world at gp-0x848, lobby instance at gp-0xc8, buddy manager at
    /// gp+0x238, comm system at gp-0x238, a self-linked list head at
    /// self+0xF0 and a zeroed tag list behind msg+0xC. All branch
    /// steering is seed arithmetic on the scenario index below: the
    /// message kind word at msg+4, the flags at msg+8, the lobby state
    /// at self+0, the peer string behind self+0x98 and the forced-result
    /// tables the faked callees return from.
    fn onmsg_base(c: &mut Cpu, seed: u32) {
        onmsg_install();
        let mut r = Rng(0x259b_1000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = 0x0025_9b10;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.bus.write32(ONMSG_CTRL, ONMSG_MAGIC);
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9000);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x9800);
        c.bus.write32(
            GAME_GP.wrapping_add(LOBBY_ON_MESSAGE_LOBBY_GP as u32),
            ONMSG_SELF,
        );
        c.bus.write32(
            GAME_GP.wrapping_add(LOBBY_ON_MESSAGE_COMM_GP as u32),
            DATA + 0x2800,
        );
        c.bus.write32(ONMSG_SELF + 0xf0, ONMSG_HEAD);
        c.bus.write32(ONMSG_HEAD, ONMSG_HEAD);
        c.bus.write32(ONMSG_HEAD + 4, ONMSG_HEAD);
        c.bus.write32(ONMSG_SELF + 0x98, ONMSG_PEER);
        c.bus.write32(ONMSG_MSG + 0xc, ONMSG_TAGS);
        for i in 0..16u32 {
            c.bus.write32(ONMSG_TAGS + i * 4, 0);
        }
        for cell in [
            R_STRCMP,
            R_BUDDY2623,
            R_CHECK263128,
            R_GETSIG,
            R_SENDPRIV,
            R_HOST,
            R_JOIN,
            R_RESET,
            R_DISCONNECT,
            R_CREATE,
            R_ISCONN,
            R_RECONN,
            R_ALLOC,
            R_BXCTOR,
            R_BXOP,
            R_BXDTOR,
        ] {
            onmsg_results(c, cell, &[]);
        }
        onmsg_results(c, R_GETSIG, &[DATA + 0x6000]);
        onmsg_results(c, R_ISCONN, &[1]);
        onmsg_results(
            c,
            R_ALLOC,
            &[
                DATA + 0x7000,
                DATA + 0x7100,
                DATA + 0x7200,
                DATA + 0x7300,
                DATA + 0x7400,
                DATA + 0x7500,
                DATA + 0x7600,
                DATA + 0x7700,
            ],
        );
        c.g[A0][0] = ONMSG_SELF as u64;
        c.g[A1][0] = ONMSG_MSG as u64;
    }

    #[test]
    fn lobby_man_on_message_matches() {
        check(
            ee::s06::f_00259b10__generated,
            lobby_man_on_message,
            290,
            &|c, seed| {
                onmsg_base(c, seed);
                // Scenario = seed % 29, residues by division for sub-choices.
                let sc = seed % 29;
                let kind: u32 = match sc {
                    2 | 3 | 21 | 27 => 0x6368_6174, // "chat"
                    4 => 0x6361_7374,               // "cast"
                    0 => 0x1000,                    // below "chat", unknown
                    1 => 0x7072_6977,               // above "chat", not "priv"
                    _ => 0x7072_6976,               // "priv"
                };
                c.bus.write32(ONMSG_MSG + 4, kind);
                c.bus.write32(GAME_GP + 0x238, ONMSG_MGR);
                // Lobby state, peer string and strcmp result sequence.
                let (state, peer, cmps): (u32, &[u8], &[u32]) = match sc {
                    5 => (3, b"\0", &[]),                  // priv, flag 0x200000 exit
                    6 => (3, b"\0", &[]),                  // priv, no 0x40000000 exit
                    7 => (3, b"\0", &[0, 1]),              // cmd1, signature differs
                    8 => (3, b"\0", &[0, 0]),              // cmd1, buddy known
                    9 => (0x16, b"\0", &[0, 0, 0]),        // cmd1, self challenge: cancel
                    10 => (0x17, b"\0", &[1, 0]),          // cmd2, join
                    11 => (0x16, b"\0", &[1, 0, 0]),       // cmd2, host
                    12 => (5, b"\0", &[1, 0]),             // cmd2, private message
                    13 => (0x16, b"\0", &[1, 1, 0]),       // cmd3, post 0xED, state->1
                    14 => (3, b"\0", &[1, 1, 0]),          // cmd3, post 0xED
                    15 => (0x18, b"\0", &[1, 1, 1, 0]),    // cmd4, reset + post 0xEE
                    16 => (0x16, b"x\0", &[1, 1, 1, 0]),   // cmd4, peer present: no send
                    17 => (3, b"\0", &[1, 1, 1, 0]),       // cmd4, post 0xEE
                    18 => (0x16, b"\0", &[1, 1, 1, 1, 0]), // cmd5, post 0xF0
                    19 => (3, b"\0", &[1, 1, 1, 1, 1, 0]), // cmd6, post 0xF2
                    20 => (3, b"\0", &[1, 1, 1, 1, 1, 1]), // no command matches
                    22 => (0x17, b"\0", &[1, 1, 1, 0]),    // cmd4, state->1 + post 0xEE
                    23 => (0x16, b"\0", &[1, 0, 1]),       // cmd2, name differs: message
                    24 => (0x18, b"x\0", &[1, 1, 1, 0]),   // cmd4, peer present at 0x18
                    25 => (3, b"\0", &[0, 1]),             // cmd1, signature differs
                    26 => (5, b"\0", &[1, 0]),             // cmd2, private message
                    _ => (3, b"\0", &[]),
                };
                c.bus.write32(
                    ONMSG_MSG + 8,
                    match sc {
                        5 => 0x20_0000,
                        6 => 0,
                        _ => 0x4000_0000,
                    },
                );
                c.bus.write32(ONMSG_SELF, state);
                put_str(c, ONMSG_PEER, peer);
                onmsg_results(c, R_STRCMP, cmps);
                onmsg_results(
                    c,
                    R_BUDDY2623,
                    &[if matches!(sc, 2 | 8 | 27) { 1 } else { 0 }],
                );
                onmsg_results(c, R_CHECK263128, &[0]);
                if sc == 21 {
                    // No buddy manager yet: create, reconnect, disconnect.
                    c.bus.write32(GAME_GP + 0x238, 0);
                    onmsg_results(c, R_ISCONN, &[0]);
                    onmsg_results(c, R_BUDDY2623, &[1]);
                }
                if sc == 28 {
                    // A faked callee unwinds at the first strcmp.
                    onmsg_results(c, R_STRCMP, &[0, 0]);
                    c.bus.write32(R_STRCMP + 4, 0);
                }
            },
        );
    }
}
