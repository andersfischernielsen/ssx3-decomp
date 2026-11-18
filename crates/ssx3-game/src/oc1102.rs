//! Staging module oc1102 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1102`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_8688, lobby_api_destroy_list, "LobbyApi_DestroyList"),
    (0x003e_8778, lobby_api_add_list_entry, "LobbyApi_AddListEntry"),
    (0x003e_8968, lobby_api_remove_list_entry, "LobbyApi_RemoveListEntry"),
    (0x003e_8a28, lobby_api_has_list_entry, "LobbyApi_HasListEntry"),
    (0x003e_8a70, lobby_api_get_record, "LobbyApi_GetRecord"),
    (0x003e_8b10, lobby_api_send, "LobbyApi_Send"),
    (0x003e_8d70, lobby_api_cancel_request, "LobbyApi_CancelRequest"),
    (0x003e_8dc0, lobby_api_ping, "LobbyApi_Ping"),
    (0x003e_8f08, lobby_api_status, "LobbyApi_Status"),
    (0x003e_8fb8, lobby_api_get_event, "LobbyApi_GetEvent"),
    (0x003e_8fd0, lobby_api_debug_packet, "LobbyApi_DebugPacket"),
    (0x003e_9160, lobby_api_trim_rank_list, "LobbyApi_TrimRankList"),
    (0x003e_9378, lobby_api_pick_room, "LobbyApi_PickRoom"),
    (0x003e_94f0, lobby_api_server_ping_callback, "LobbyApi_ServerPingCallback"),
    (0x003e_9590, lobby_api_ping_server, "LobbyApi_PingServer"),
    (0x003e_ae68, tag_field_parse_decimal, "TagField_ParseDecimal"),
    (0x003e_aeb8, ds_secstotime, "ds_secstotime"),
];

/// Callee addresses this module calls.
pub(crate) mod callee {
    use super::*;
    // Reused names (one name per address; never redefine).
    pub(crate) use crate::game::dirtysock::tags::callee::DS_SECSTOTIME;
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_PARSE_DECIMAL;
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_SET_ADDRESS;
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_SET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOBBY_API_PING;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOBBY_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOBBY_LIST_VISIBLE_COUNT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEMSET_3 as MEMSET;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::STRLEN;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::render::ps2graphicsman::TEX_LOAD_STRNCPY as STRNCPY;
    pub(crate) use crate::game::world::worldcache::STRNCMP;
    // New names (no existing crate name for these addresses).
    pub const LOBBY_LIST_GET_HASH: Callee = Callee {
        addr: 0x003e_e770,
        f: ee::s14::f_003ee770,
    };
    pub const LOBBY_LIST_REMOVE: Callee = Callee {
        addr: 0x003e_e480,
        f: ee::s14::f_003ee480,
    };
    pub const LOBBY_HASH_POP_ANY: Callee = Callee {
        addr: 0x003e_e9a0,
        f: ee::s14::f_003ee9a0,
    };
    pub const LOBBY_HASH_DESTROY: Callee = Callee {
        addr: 0x003e_e820,
        f: ee::s14::f_003ee820,
    };
    pub const LOBBY_LIST_DESTROY: Callee = Callee {
        addr: 0x003e_e260,
        f: ee::s14::f_003ee260,
    };
    pub const LOBBY_HASH_FIND_STR: Callee = Callee {
        addr: 0x003e_ec88,
        f: ee::s14::f_003eec88,
    };
    pub const LOBBY_HASH_DEL_STR: Callee = Callee {
        addr: 0x003e_ed08,
        f: ee::s14::f_003eed08,
    };
    pub const LOBBY_LIST_ADD: Callee = Callee {
        addr: 0x003e_e320,
        f: ee::s14::f_003ee320,
    };
    pub const LOBBY_HASH_ADD_STR: Callee = Callee {
        addr: 0x003e_eb30,
        f: ee::s14::f_003eeb30,
    };
    pub const LOBBY_LIST_SORT: Callee = Callee {
        addr: 0x003e_e620,
        f: ee::s14::f_003ee620,
    };
    pub const LOBBY_LIST_MARK_DIRTY: Callee = Callee {
        addr: 0x003e_e600,
        f: ee::s14::f_003ee600,
    };
    pub const LOBBY_MEM_ALLOC: Callee = Callee {
        addr: 0x0025_91e8,
        f: ee::s06::f_002591e8,
    };
    pub const LOBBY_MEM_FREE: Callee = Callee {
        addr: 0x0025_9210,
        f: ee::s06::f_00259210,
    };
    pub const LOBBY_API_ADD_REQUEST: Callee = Callee {
        addr: 0x003e_7718,
        f: ee::s14::f_003e7718,
    };
    pub const LOBBY_API_DEBUG_PRINTF: Callee = Callee {
        addr: 0x003e_76a8,
        f: ee::s14::f_003e76a8,
    };
    pub const TAG_FIELD_STRNZCPY: Callee = Callee {
        addr: 0x003e_bb60,
        f: ee::s14::f_003ebb60,
    };
    pub const TAG_FIELD_MERGE: Callee = Callee {
        addr: 0x003e_cb00,
        f: ee::s14::f_003ecb00,
    };
    pub const TAG_FIELD_SET_ENCODED: Callee = Callee {
        addr: 0x003e_c6d0,
        f: ee::s14::f_003ec6d0,
    };
    pub const PROTO_PING_REQUEST: Callee = Callee {
        addr: 0x003e_7450,
        f: ee::s14::f_003e7450,
    };
    pub const PROTO_ARIES_SEND: Callee = Callee {
        addr: 0x003f_5910,
        f: ee::s14::f_003f5910,
    };
    pub const PROTO_ARIES_TICK: Callee = Callee {
        addr: 0x003f_5d88,
        f: ee::s14::f_003f5d88,
    };
    pub const NET_TICK_WRAPPER2: Callee = Callee {
        addr: 0x003f_4330,
        f: ee::s14::f_003f4330,
    };
    pub const LOBBY_ENCRYPT_STRING: Callee = Callee {
        addr: 0x003f_35d0,
        f: ee::s14::f_003f35d0,
    };
}

/// `int LobbyApi_DestroyList(LobbyApi* api /*a0*/, int kind /*a1*/,
/// void* list /*a2*/)` (0x3e8688): drain the rank-list hash for one list
/// kind (pop every hash entry, remove and free it), destroy the hash and the
/// list, then clear the list's slot words. Returns -1 when `kind` is out of
/// range, else 0.
pub fn lobby_api_destroy_list(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_8688 => {
                    c.cycles += 14;
                    let frame = Frame::open(c, 0x80);
                    frame.save(c, 22, 0x10);
                    frame.save(c, 16, 0x70);
                    c.g[S6][0] = c.g[A1][0];
                    frame.save(c, 18, 0x50);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, 17, 0x60);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save(c, 19, 0x40);
                    c.g[V0][0] = (c.g[S6][0] < 0x8u64) as u64;
                    frame.save(c, 20, 0x30);
                    frame.save(c, 21, 0x20);
                    let t = c.g[V0][0] == 0u64;
                    frame.save_ra(c, 0);
                    if t {
                        pc = 0x003e_874c;
                    } else {
                        pc = 0x003e_86c0;
                    }
                }
                0x003e_86c0 => {
                    c.cycles += 2;
                    if c.g[S2][0] == 0u64 {
                        frame_restore_destroy_list(c);
                        pc = 0x003e_8750;
                    } else {
                        pc = 0x003e_86c8;
                    }
                }
                0x003e_86c8 => {
                    c.cycles += 7;
                    // Registers at the call: list in a0, return below.
                    c.g[A0][0] = c.g[S2][0];
                    callee::LOBBY_LIST_GET_HASH.call(c, 0x003e_86d0)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x314));
                    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x318));
                    let t = true;
                    c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x31c));
                    if t {
                        pc = 0x003e_86f8;
                    } else {
                        pc = 0x003e_86e4;
                    }
                }
                0x003e_86e4 => {
                    c.cycles += 1;
                    pc = 0x003e_86e8;
                }
                0x003e_86e8 => {
                    c.cycles += 4;
                    // Registers at the call: list in a0, hash in a1.
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_86f0)?;
                    // Registers at the call: removed entry in a0.
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003e_86f8)?;
                    pc = 0x003e_86f8;
                }
                0x003e_86f8 => {
                    c.cycles += 4;
                    // Registers at the call: hash in a0.
                    c.g[A0][0] = c.g[S1][0];
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003e_8700)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x003e_86e8) {
                            return Ok(());
                        }
                        pc = 0x003e_86e8;
                    } else {
                        pc = 0x003e_8708;
                    }
                }
                0x003e_8708 => {
                    c.cycles += 10;
                    // Registers at the call: hash in a0.
                    c.g[A0][0] = c.g[S1][0];
                    callee::LOBBY_HASH_DESTROY.call(c, 0x003e_8710)?;
                    // Registers at the call: list in a0.
                    c.g[A0][0] = c.g[S2][0];
                    callee::LOBBY_LIST_DESTROY.call(c, 0x003e_8718)?;
                    c.g[V0][0] = 0x14;
                    c.mult(0, 3, 22, 2);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    let t = c.g[V0][0] != c.g[S2][0];
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        pc = 0x003e_8740;
                    } else {
                        pc = 0x003e_8730;
                    }
                }
                0x003e_8730 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x310), 0u64);
                    c.sw(c.g[A0][0] as u32, 0u64);
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    pc = 0x003e_8740;
                }
                0x003e_8740 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.sw(c.g[V1][0] as u32, 0u64);
                    pc = 0x003e_874c;
                }
                0x003e_874c => {
                    c.cycles += 1;
                    frame_restore_destroy_list(c);
                    pc = 0x003e_8750;
                }
                0x003e_8750 => {
                    c.cycles += 9;
                    let frame = Frame { size: 0x80 };
                    frame.restore(c, 17, 0x60);
                    frame.restore(c, 18, 0x50);
                    frame.restore(c, 19, 0x40);
                    frame.restore(c, 20, 0x30);
                    frame.restore(c, 21, 0x20);
                    frame.restore(c, 22, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x003e_8688, 0x003e_8774);
                    return Ok(());
                }
            }
        }
    })
}

/// Restore the callee-saved registers of `lobby_api_destroy_list` (the
/// 0x86c0 early-out and 0x874c join share this epilogue head).
fn frame_restore_destroy_list(c: &mut Cpu) {
    let frame = Frame { size: 0x80 };
    frame.restore(c, 16, 0x70);
}

/// `int LobbyApi_AddListEntry(LobbyApi* api /*a0*/, int kind /*a1*/,
/// void* entry /*a2*/)` (0x3e8778): add one entry to a lobby list kind:
/// kind 3 is the name list (dup-checked by string hash), kinds 4-7 are the
/// ranked lists (dup-checked by the entry's name field, entry copied into a
/// fresh block). Returns 1 added, 0 duplicate, -1 bad kind or empty list.
pub fn lobby_api_add_list_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 16, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, 20, 0x10);
        c.g[V0][0] = (c.g[S1][0] < 0x8u64) as u64;
        let t = c.g[V0][0] == 0u64;
        frame.save_ra(c, 0);
        if t {
            add_list_entry_fail(c, &frame);
            return Ok(());
        }
        c.cycles += 8;
        c.g[V1][0] = 0x14;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x310));
        c.mult(0, 3, 17, 3);
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0xffffffffffffffff;
        if t {
            frame_epilogue_add_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 0x3;
        let t = c.g[S1][0] != c.g[V0][0];
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
        if t {
            // Kinds 4-7: ranked entry.
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                frame_epilogue_add_list_entry(c, &frame);
                return Ok(());
            }
            c.cycles += 4;
            // Registers at the call: hash in a0, name+8 in a1.
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
            callee::LOBBY_HASH_FIND_STR.call(c, 0x003e_8840)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0u64;
            if t {
                frame_epilogue_add_list_entry(c, &frame);
                return Ok(());
            }
            c.cycles += 8;
            // Registers at the call: size in a0.
            c.g[A0][0] = 0xa8;
            callee::LOBBY_MEM_ALLOC.call(c, 0x003e_8850)?;
            c.g[A3][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[V0][0] = c.g[A1][0] | c.g[A3][0];
            c.g[V0][0] = c.g[V0][0] & 0x7;
            let t = c.g[V0][0] == 0u64;
            c.g[A2][0] = c.g[A3][0];
            if t {
                // Aligned: 0xa0-byte block copy with `ld`/`sd`.
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x314));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
                c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                'l_3e88d4: loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A1][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A2][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V1][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_88d4);
                        }
                        continue 'l_3e88d4;
                    } else {
                        c.cycles += 1;
                        c.g[S0][0] = 0x14;
                        break 'l_3e88d4;
                    }
                }
            } else {
                // Unaligned: 0xa0-byte block copy with `ldl`/`ldr`.
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x314));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
                c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                'l_3e8874: loop {
                    c.cycles += 19;
                    c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(4, c.g[A1][0] as u32);
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x1f));
                    c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[A2][0] as u32);
                    c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x17));
                    c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x10));
                    c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x1f));
                    c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V1][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_8874);
                        }
                        continue 'l_3e8874;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[S0][0] = 0x14;
                        if t {
                            break 'l_3e8874;
                        } else {
                            break 'l_3e8874;
                        }
                    }
                }
            }
            c.cycles += 12;
            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[A1][0] as u32);
            c.sdl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.mult(0, 16, 17, 16);
            c.sdr(3, c.g[A2][0] as u32);
            c.g[A1][0] = c.g[A3][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            // Registers at the call: list in a0, entry in a1.
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            callee::LOBBY_LIST_ADD.call(c, 0x003e_8928)?;
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            add_list_entry_finish_hash(c, &frame)?;
            return Ok(());
        }
        // Kind 3: bare name entry.
        c.cycles += 5;
        // Registers at the call: hash in a0, name in a1.
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34c));
        c.g[A1][0] = c.g[S3][0];
        callee::LOBBY_HASH_FIND_STR.call(c, 0x003e_87e0)?;
        if c.g[V0][0] != 0u64 {
            c.g[V0][0] = 0u64;
            frame_epilogue_add_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 17;
        // Registers at the call: size in a0.
        c.g[A0][0] = 0x138;
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_87f0)?;
        c.g[S0][0] = c.g[V0][0];
        // Registers at the call: block, zero, size (memset).
        c.g[A2][0] = 0x138;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0u64;
        callee::MEMSET.call(c, 0x003e_8804)?;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        // Registers at the call: dst, src (strcpy).
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S1][0];
        callee::STRCPY.call(c, 0x003e_8814)?;
        // Registers at the call: list in a0, entry in a1.
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x350));
        c.g[A1][0] = c.g[S0][0];
        callee::LOBBY_LIST_ADD.call(c, 0x003e_8820)?;
        // Registers at the call: hash in a0, name in a1.
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34c));
        let t = true;
        c.g[A1][0] = c.g[S1][0];
        if t {
            add_list_entry_finish_hash(c, &frame)?;
            return Ok(());
        }
        frame_epilogue_add_list_entry(c, &frame);
        Ok(())
    })
}

/// 0x3e893c: hash the copied name into the kind's string hash, return 1.
/// Falls into the shared 0x3e8948 epilogue (+8) like the generated `break`.
fn add_list_entry_finish_hash(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 4;
    // Registers at the call: hash in a0, name in a1, entry in a2.
    c.g[A2][0] = c.g[V0][0];
    callee::LOBBY_HASH_ADD_STR.call(c, 0x003e_893c)?;
    c.g[V0][0] = 0x1;
    c.cycles += 8;
    add_list_entry_restore(c, frame);
    Ok(())
}

/// 0x3e8944 exit of `lobby_api_add_list_entry`: return -1.
fn add_list_entry_fail(c: &mut Cpu, frame: &Frame) {
    c.cycles += 1;
    c.g[V0][0] = 0xffffffffffffffff;
    frame_epilogue_add_list_entry(c, frame);
}

/// 0x3e8948 epilogue of `lobby_api_add_list_entry` (V0 already set).
fn frame_epilogue_add_list_entry(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    add_list_entry_restore(c, frame);
}

/// Restore the frame of `lobby_api_add_list_entry` and return.
fn add_list_entry_restore(c: &mut Cpu, frame: &Frame) {
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore(c, 19, 0x20);
    frame.restore(c, 20, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `int LobbyApi_RemoveListEntry(LobbyApi* api /*a0*/, int kind /*a1*/,
/// char* name /*a2*/)` (0x3e8968): drop one entry from a lobby list kind:
/// find the name in the kind's string hash, unlink the hash entry, remove
/// the entry from the list and free it. Returns 1 removed, 0 missing, -1
/// bad kind or empty list.
pub fn lobby_api_remove_list_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = (c.g[A1][0] < 0x8u64) as u64;
        frame.save(c, 19, 0x20);
        frame.save(c, 20, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, 16, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        let t = c.g[V0][0] == 0u64;
        frame.save_ra(c, 0);
        if t {
            remove_list_entry_fail(c, &frame);
            return Ok(());
        }
        c.cycles += 7;
        c.g[V0][0] = 0x14;
        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x310));
        c.mult(0, 18, 5, 2);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0xffffffffffffffff;
        if t {
            frame_epilogue_remove_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3));
        c.g[V0][0] = (c.g[V0][0] < 0x5u64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0xffffffffffffffff;
        if t {
            frame_epilogue_remove_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 5;
        // Registers at the call: hash in a0, name in a1.
        c.g[A1][0] = c.g[S3][0];
        callee::LOBBY_HASH_FIND_STR.call(c, 0x003e_89c8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0u64 {
            c.cycles += 2;
            let t = true;
            c.g[V0][0] = 0u64;
            if t {
                frame_epilogue_remove_list_entry(c, &frame);
                return Ok(());
            }
        }
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        c.cycles += 10;
        // Registers at the call: hash in a0, name in a1.
        c.g[A1][0] = c.g[S3][0];
        callee::LOBBY_HASH_DEL_STR.call(c, 0x003e_89e4)?;
        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[A1][0] = c.g[S0][0];
        // Registers at the call: list+0x314 in a0, hash entry in a1.
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x314));
        callee::LOBBY_LIST_REMOVE.call(c, 0x003e_89f4)?;
        // Registers at the call: removed entry in a0.
        c.g[A0][0] = c.g[V0][0];
        callee::LOBBY_MEM_FREE.call(c, 0x003e_89fc)?;
        let t = true;
        c.g[V0][0] = 0x1;
        if t {
            frame_epilogue_remove_list_entry(c, &frame);
            return Ok(());
        }
        frame_epilogue_remove_list_entry(c, &frame);
        Ok(())
    })
}

/// 0x3e8a04 exit of `lobby_api_remove_list_entry`: return -1.
fn remove_list_entry_fail(c: &mut Cpu, frame: &Frame) {
    c.cycles += 1;
    c.g[V0][0] = 0xffffffffffffffff;
    frame_epilogue_remove_list_entry(c, &frame);
}

/// 0x3e8a08 epilogue of `lobby_api_remove_list_entry` (V0 already set).
fn frame_epilogue_remove_list_entry(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore(c, 19, 0x20);
    frame.restore(c, 20, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `int LobbyApi_HasListEntry(LobbyApi* api /*a0*/, int kind /*a1*/,
/// char* name /*a2*/)` (0x3e8a28): whether the kind's string hash contains
/// the name. Returns 1 present, 0 absent, -1 bad kind or empty list.
pub fn lobby_api_has_list_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[A1][0] < 0x8u64) as u64;
        let t = c.g[V0][0] == 0u64;
        frame.save_ra(c, 0);
        if t {
            has_list_entry_fail(c, &frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = 0x14;
        c.mult(0, 3, 5, 2);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x310));
        if c.g[A0][0] == 0u64 {
            c.g[V0][0] = 0xffffffffffffffff;
            frame_epilogue_has_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 4;
        // Registers at the call: hash in a0, name in a1.
        c.g[A1][0] = c.g[A2][0];
        callee::LOBBY_HASH_FIND_STR.call(c, 0x003e_8a58)?;
        let t = true;
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        if t {
            frame_epilogue_has_list_entry(c, &frame);
            return Ok(());
        }
        frame_epilogue_has_list_entry(c, &frame);
        Ok(())
    })
}

/// 0x3e8a60 exit of `lobby_api_has_list_entry`: return -1.
fn has_list_entry_fail(c: &mut Cpu, frame: &Frame) {
    c.cycles += 1;
    c.g[V0][0] = 0xffffffffffffffff;
    frame_epilogue_has_list_entry(c, frame);
}

/// 0x3e8a64 epilogue of `lobby_api_has_list_entry` (V0 already set).
fn frame_epilogue_has_list_entry(c: &mut Cpu, frame: &Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void* LobbyApi_GetRecord(LobbyApi* api /*a0*/, int tag /*a1*/)`
/// (0x3e8a70): the record block for a request tag (`"user"`, `"act"`,
/// `"pers"`, `"room"`, `"play"`, `"down"`): the per-tag lobby field, or the
/// first record pointer when the tag's flag word is clear, else null.
pub fn lobby_api_get_record(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x75730000;
    c.g[V0][0] = c.g[V0][0] | 0x6572;
    let t = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = 0x61630000;
    if t {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x134));
        c.pc = t;
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[V0][0] | 0x6374;
    let t = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x70650000;
    if t {
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] | 0x7273;
        let t = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = 0x726f0000;
        if t {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] | 0x6f6d;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 0x706c0000;
            if t {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] | 0x6179;
                if c.g[A1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x646f0000;
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x776e;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b8));
                    if t {
                        c.cycles += 2;
                        let t = c.g[RA][0] as u32;
                        c.g[V0][0] = 0u64;
                        c.pc = t;
                        return;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x3b8));
                    let t = c.g[RA][0] as u32;
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0u64;
                    }
                    c.pc = t;
                    return;
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x2cc));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x200));
                let t = c.g[RA][0] as u32;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.pc = t;
                return;
            }
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c0));
            c.pc = t;
            return;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x174));
        c.pc = t;
        return;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x134));
    c.pc = t;
}

/// `int LobbyApi_Send(LobbyApi* api /*a0*/, int tag /*a1*/, void* buf /*a2*/,
/// int cb /*a3*/, ...)` (0x3e8b10): queue a lobby request (`AddRequest`),
/// merge the connection fields into a tag buffer for auth/act/pass tags
/// (encrypting when the lobby holds a key), drop the snap rank list, bump
/// the sequence counter and hand the packet to `ProtoAriesSend`. Returns the
/// new request's first word, -2 when no request was made, -1 bad state tag.
pub fn lobby_api_send(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_8b10 => {
                    c.cycles += 17;
                    let frame = Frame::open(c, 0x290);
                    c.g[V0][0] = 0x69640000;
                    frame.save(c, 17, 0x270);
                    c.g[V0][0] = c.g[V0][0] | 0x6c65;
                    frame.save(c, 19, 0x250);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, 20, 0x240);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, 16, 0x280);
                    c.g[A2][0] = c.g[A3][0];
                    frame.save(c, 18, 0x260);
                    c.g[S4][0] = c.g[A1][0];
                    frame.save(c, 21, 0x230);
                    frame.save_ra(c, 0x220);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A3][0] = c.g[T0][0];
                    if t {
                        pc = 0x003e_8b8c;
                    } else {
                        pc = 0x003e_8b54;
                    }
                }
                0x003e_8b54 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x61750000;
                    c.g[V0][0] = c.g[V0][0] | 0x7468;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        pc = 0x003e_8b90;
                    } else {
                        pc = 0x003e_8b64;
                    }
                }
                0x003e_8b64 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x61630000;
                    c.g[V0][0] = c.g[V0][0] | 0x6374;
                    let t = c.g[V1][0] == c.g[V0][0];
                    if t {
                        pc = 0x003e_8b90;
                    } else {
                        pc = 0x003e_8b74;
                    }
                }
                0x003e_8b74 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x736b0000;
                    c.g[V0][0] = c.g[V0][0] | 0x6579;
                    let t = c.g[V1][0] == c.g[V0][0];
                    if t {
                        pc = 0x003e_8b90;
                    } else {
                        pc = 0x003e_8b84;
                    }
                }
                0x003e_8b84 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x003e_8d4c;
                    } else {
                        pc = 0x003e_8b8c;
                    }
                }
                0x003e_8b8c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x003e_8b90;
                }
                0x003e_8b90 => {
                    c.cycles += 5;
                    // Registers at the call: api, tag, buf?, cb?.
                    c.g[A1][0] = c.g[S4][0];
                    callee::LOBBY_API_ADD_REQUEST.call(c, 0x003e_8b98)?;
                    c.g[S5][0] = c.g[V0][0];
                    let t = c.g[S5][0] == 0u64;
                    c.g[V0][0] = 0xfffffffffffffffe;
                    if t {
                        pc = 0x003e_8d4c;
                    } else {
                        pc = 0x003e_8ba4;
                    }
                }
                0x003e_8ba4 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x61750000;
                    c.g[V0][0] = c.g[V0][0] | 0x7468;
                    let t = c.g[S4][0] == c.g[V0][0];
                    c.g[V0][0] = 0x61630000;
                    if t {
                        pc = 0x003e_8bcc;
                    } else {
                        pc = 0x003e_8bb4;
                    }
                }
                0x003e_8bb4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6374;
                    let t = c.g[S4][0] == c.g[V0][0];
                    c.g[V0][0] = 0x70610000;
                    if t {
                        pc = 0x003e_8bcc;
                    } else {
                        pc = 0x003e_8bc0;
                    }
                }
                0x003e_8bc0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7373;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[V0][0] = 0x736e0000;
                    if t {
                        pc = 0x003e_8ca0;
                    } else {
                        pc = 0x003e_8bcc;
                    }
                }
                0x003e_8bcc => {
                    c.cycles += 24;
                    // Registers at the call: dst, size, src (strnzcpy).
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_STRNZCPY.call(c, 0x003e_8bdc)?;
                    // Registers at the call: dst, size, src (merge).
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x34));
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_MERGE.call(c, 0x003e_8bec)?;
                    c.g[V1][0] = 0x4a0000;
                    // Registers at the call: buf, key (find).
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x48d8));
                    c.g[A1][0] = c.g[S2][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003e_8c00)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x201));
                    c.g[A3][0] = 0x4a0000;
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
                    // Registers at the call: field, out, size, def (get).
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x1f;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e_8c20)?;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x24));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x003e_8c64;
                    } else {
                        pc = 0x003e_8c2c;
                    }
                }
                0x003e_8c2c => {
                    c.cycles += 14;
                    // Registers at the call: buf, size, key, src, tmp.
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x200;
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4bc));
                    c.g[T0][0] = c.g[S0][0];
                    callee::TAG_FIELD_SET_ENCODED.call(c, 0x003e_8c44)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4b8));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x48e0));
                    // Registers at the call: buf, size, key, number.
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_8c5c)?;
                    let t = true;
                    c.g[S3][0] = c.g[SP][0];
                    if t {
                        pc = 0x003e_8c9c;
                    } else {
                        pc = 0x003e_8c64;
                    }
                }
                0x003e_8c64 => {
                    c.cycles += 14;
                    c.g[V0][0] = 0x7e;
                    c.g[A2][0] = c.g[A0][0];
                    c.g[A1][0] = 0x1f;
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    c.g[T0][0] = 0x10;
                    c.g[T1][0] = 0x10;
                    // Registers at the call: dst, len, src, key, klen, klen.
                    callee::LOBBY_ENCRYPT_STRING.call(c, 0x003e_8c84)?;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S3][0];
                    // Registers at the call: buf, size, key, value.
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_8c98)?;
                    c.g[S3][0] = c.g[SP][0];
                    pc = 0x003e_8c9c;
                }
                0x003e_8c9c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x736e0000;
                    pc = 0x003e_8ca0;
                }
                0x003e_8ca0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6170;
                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                        pc = 0x003e_8d34;
                    } else {
                        pc = 0x003e_8cac;
                    }
                }
                0x003e_8cac => {
                    c.cycles += 12;
                    c.g[A1][0] = 0x4a0000;
                    // Registers at the call: buf, key (find).
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48e8));
                    callee::TAG_FIELD_FIND.call(c, 0x003e_8cbc)?;
                    // Registers at the call: field, def (get number).
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e_8cc8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x14;
                    if t {
                        pc = 0x003e_8d20;
                    } else {
                        pc = 0x003e_8cdc;
                    }
                }
                0x003e_8cdc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x310));
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x314));
                    if t {
                        pc = 0x003e_8d20;
                    } else {
                        pc = 0x003e_8cf4;
                    }
                }
                0x003e_8cf4 => {
                    c.cycles += 3;
                    c.g[S2][0] = c.g[V1][0];
                    let t = true;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        pc = 0x003e_8d10;
                    } else {
                        pc = 0x003e_8d00;
                    }
                }
                0x003e_8d00 => {
                    c.cycles += 4;
                    // Registers at the call: list, entry.
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_8d08)?;
                    // Registers at the call: removed entry.
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003e_8d10)?;
                    pc = 0x003e_8d10;
                }
                0x003e_8d10 => {
                    c.cycles += 4;
                    // Registers at the call: hash.
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003e_8d18)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        if back_edge_unwinds(c, 0x003e_8d00) {
                            return Ok(());
                        }
                        pc = 0x003e_8d00;
                    } else {
                        pc = 0x003e_8d20;
                    }
                }
                0x003e_8d20 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50c));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x508), 0u64);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x50c), c.g[V0][0]);
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    pc = 0x003e_8d34;
                }
                0x003e_8d34 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A2][0] = 0u64;
                    c.g[T0][0] = 0xffffffffffffffff;
                    // Registers at the call: link, tag, flags, buf, type.
                    callee::PROTO_ARIES_SEND.call(c, 0x003e_8d48)?;
                    c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                    pc = 0x003e_8d4c;
                }
                0x003e_8d4c => {
                    c.cycles += 9;
                    let frame = Frame { size: 0x290 };
                    frame.restore(c, 16, 0x280);
                    frame.restore(c, 17, 0x270);
                    frame.restore(c, 18, 0x260);
                    frame.restore(c, 19, 0x250);
                    frame.restore(c, 20, 0x240);
                    frame.restore(c, 21, 0x230);
                    frame.restore_ra(c, 0x220);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x003e_8b10, 0x003e_8d70);
                    return Ok(());
                }
            }
        }
    })
}

/// `int LobbyApi_CancelRequest(LobbyApi* api /*a0*/, int tag /*a1*/)`
/// (0x3e8d70): cancel one queued request: walk the request list at_3
/// `api+0x53c`, clear the match word of the entry whose tag matches (or all
/// entries for tag -1). Returns whether anything was cancelled.
pub fn lobby_api_cancel_request(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x53c));
    c.g[A2][0] = 0u64;
    let t = c.g[V0][0] == 0u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x53c));
    if t {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.g[A2][0];
        c.pc = t;
        return;
    }
    c.cycles += 2;
    c.g[A3][0] = 0xffffffffffffffff;
    'l_3e8d88: loop {
        c.cycles += 4;
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        if c.g[V0][0] == c.g[A1][0] {
            c.sw((c.g[V1][0] as u32).wrapping_add(0x38), 0u64);
            c.cycles += 2;
            c.g[A2][0] = 0x1;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        } else {
            c.cycles += 2;
            if c.g[A1][0] != c.g[A3][0] {
                // No match and not match-all: rejoin at the advance,
                // skipping the found tail.
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            } else {
                c.cycles += 1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x38), 0u64);
                c.cycles += 2;
                c.g[A2][0] = 0x1;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            }
        }
        c.cycles += 3;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
        let t = c.g[V1][0] != 0u64;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x003e_8d88);
            }
            continue 'l_3e8d88;
        } else {
            break 'l_3e8d88;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A2][0];
    c.pc = t;
}

/// `int LobbyApi_Ping(LobbyApi* api /*a0*/, void* addr /*a1*/,
/// int id /*a2*/, int extra /*a3*/, ...)` (0x3e8dc0): send one ping request:
/// queue a `"ping"` request holding the address, stamp it, and issue the
/// ping. Returns the new request's first word, -2/-3/-4 when the lobby is
/// down, full, or the queue failed, -1 bad state tag.
pub fn lobby_api_ping(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x160);
        c.g[V0][0] = 0x636f0000;
        frame.save(c, 16, 0x150);
        c.g[V0][0] = c.g[V0][0] | 0x6e6e;
        frame.save(c, 18, 0x130);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 19, 0x120);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x110);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 17, 0x140);
        c.g[S4][0] = c.g[A3][0];
        frame.save_ra(c, 0x100);
        c.g[A2][0] = c.g[T0][0];
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[A3][0] = c.g[T1][0];
        if t {
            ping_body(c, &frame)?;
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = 0x69640000;
        c.g[V0][0] = c.g[V0][0] | 0x6c65;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x61750000;
        if t {
            ping_body(c, &frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] | 0x7468;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x61630000;
        if t {
            ping_body(c, &frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] | 0x6374;
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0xffffffffffffffff;
        if t {
            frame_epilogue_ping(c, &frame);
            return Ok(());
        }
        ping_body(c, &frame)?;
        Ok(())
    })
}

/// 0x3e8e40 body of `lobby_api_ping` after the state-tag check.
fn ping_body(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    if c.g[V0][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x540));
    } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = 0xfffffffffffffffe;
        if t {
            frame_epilogue_ping(c, frame);
            return Ok(());
        }
        frame_epilogue_ping(c, frame);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 64i64) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = 0x70690000;
    if !t {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = 0xfffffffffffffffd;
        if t {
            frame_epilogue_ping(c, frame);
            return Ok(());
        }
        frame_epilogue_ping(c, frame);
        return Ok(());
    }
    c.cycles += 6;
    // Registers at the call: api, "ping", arg, arg.
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[A1][0] | 0x6e67;
    callee::LOBBY_API_ADD_REQUEST.call(c, 0x003e_8e60)?;
    c.g[S1][0] = c.g[V0][0];
    if c.g[S1][0] == 0u64 {
        c.g[V0][0] = 0xfffffffffffffffc;
        frame_epilogue_ping(c, frame);
        return Ok(());
    }
    c.cycles += 31;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
    // Registers at the call: link word.
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    callee::PROTO_ARIES_TICK.call(c, 0x003e_8e78)?;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[A2][0] = 0x1f;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
    c.g[A1][0] = c.g[S3][0];
    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
    // Registers at the call: dst, src, size (strncpy).
    call_ptr(c, callee::STRNCPY, 0x003e_8e90)?;
    c.g[A2][0] = 0x4a0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4880));
    c.sb(c.g[SP][0] as u32, 0u64);
    c.g[A3][0] = c.g[S3][0];
    // Registers at the call: buf, size, key, value (set string).
    c.g[A1][0] = 0x100;
    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_8eac)?;
    c.g[A2][0] = 0x4a0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4890));
    c.g[A1][0] = 0x100;
    c.g[A3][0] = c.g[S2][0];
    // Registers at the call: buf, size, key, addr (set address).
    callee::TAG_FIELD_SET_ADDRESS.call(c, 0x003e_8ec4)?;
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[SP][0];
    c.g[A3][0] = 0x100;
    // Registers at the call: link, addr, buf, size (ping request).
    callee::PROTO_PING_REQUEST.call(c, 0x003e_8ed8)?;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x540));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x540), c.g[V1][0]);
    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
    frame_epilogue_ping(c, frame);
    Ok(())
}

/// 0x3e8ee8 epilogue of `lobby_api_ping` (V0 already set).
fn frame_epilogue_ping(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    frame.restore(c, 16, 0x150);
    frame.restore(c, 17, 0x140);
    frame.restore(c, 18, 0x130);
    frame.restore(c, 19, 0x120);
    frame.restore(c, 20, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
}

/// `int LobbyApi_Status(LobbyApi* api /*a0*/, int tag /*a1*/)` (0x3e8f08):
/// lobby status words by tag: the state tag, roster counts, the error word,
/// the init flag, or the event/slot words for the room/login/more/slot
/// tags, else 0.
pub fn lobby_api_status(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x73740000;
    c.g[V0][0] = c.g[V0][0] | 0x6174;
    let t = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x72660000;
    if t {
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] | 0x6c67;
        let t = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = 0x65780000;
        if t {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] | 0x746e;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 0x6c6f0000;
            if t {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] | 0x636c;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x696e0000;
                if t {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6174;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] | 0x7265;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x736c0000;
                        if t {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V0][0] | 0x6f74;
                            let t = c.g[A1][0] == c.g[V0][0];
                            if t {
                                c.cycles += 2;
                                let t = c.g[RA][0] as u32;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x510));
                                c.pc = t;
                                return;
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[RA][0] as u32;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x504));
                            c.pc = t;
                            return;
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4fc));
                        let t = c.g[V1][0] == 0u64;
                        if t {
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x500));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.g[V1][0] ^ c.g[V0][0];
                            if t {
                            } else {
                                c.cycles += 2;
                                let t = c.g[RA][0] as u32;
                                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                                c.pc = t;
                                return;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = 0u64;
                    c.pc = t;
                    return;
                }
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x500));
                c.pc = t;
                return;
            }
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4fc));
            c.pc = t;
            return;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
        c.pc = t;
        return;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    c.pc = t;
}

/// `int LobbyApi_GetEvent(LobbyApi* api /*a0*/)` (0x3e8fb8): take one
/// pending lobby event bit out of the event word at_3 `api+0x8` (the lowest
/// set bit), leaving the rest queued. Returns the bit taken.
pub fn lobby_api_get_event(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.pc = t;
}

/// `void LobbyApi_DebugPacket(LobbyApi* api /*a0*/, void* packet /*a1*/)`
/// (0x3e8fd0): log one lobby packet: print the type bytes, then scan the
/// payload for printable text (flagging control bytes) and print a length
/// line. All output goes through `LobbyApi_DebugPrintf`.
pub fn lobby_api_debug_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x2d2d0000;
        frame.save(c, 16, 0x30);
        c.g[V0][0] = c.g[V0][0] | 0x2d2d;
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[T2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        let t = c.g[T0][0] != c.g[V1][0];
        if c.g[T2][0] == 0 {
            c.g[T2][0] = c.g[V0][0];
        }
        if t {
            // t0 set: the packet carries its own kind word — straight to
            // the conn line, skipping the kind transform below.
            debug_packet_conn_line(c, &frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[T0][0] = 0x2a2a0000;
        let t = c.g[T2][0] != c.g[V1][0];
        c.g[T0][0] = c.g[T0][0] | 0x2a2a;
        if t {
            debug_packet_type_line(c, &frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[T2][0] = 0x636f0000;
        let t = true;
        c.g[T2][0] = c.g[T2][0] | 0x6e6e;
        if t {
            debug_packet_conn_line(c, &frame)?;
            return Ok(());
        }
        debug_packet_type_line(c, &frame)?;
        Ok(())
    })
}

/// 0x3e903c/0x3e901c type-word line of `lobby_api_debug_packet`.
fn debug_packet_type_line(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 4;
    c.g[V0][0] = 0xfffffffffefe0000;
    c.g[V0][0] = c.g[V0][0] | 0xfefe;
    if c.g[T2][0] != c.g[V0][0] {
        c.g[T2][0] = 0x2d2d0000;
        c.cycles += 1;
        c.g[T2][0] = c.g[T2][0] | 0x2d2d;
        debug_packet_conn_line(c, frame)?;
        return Ok(());
    }
    // Equal: 'disc' kind, straight to the conn line (break 'b_3e903c
    // skips the tail above).
    c.cycles += 3;
    c.g[T2][0] = 0x64690000;
    c.g[T2][0] = c.g[T2][0] | 0x7363;
    debug_packet_conn_line(c, frame)?;
    Ok(())
}

/// 0x3e90d8 body of `lobby_api_debug_packet`: print the type line, scan the
/// payload, print the length line.
fn debug_packet_conn_line(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 17;
    c.g[V0][0] = sx(((c.g[T2][0] as u32) as i32 >> 8) as u32);
    c.g[A2][0] = sx(((c.g[T0][0] as u32) as i32 >> 24) as u32);
    c.g[T1][0] = c.g[T0][0];
    c.g[T3][0] = sx(((c.g[T2][0] as u32) as i32 >> 16) as u32);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T2][0]);
    c.g[A3][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
    c.g[A1][0] = 0x490000;
    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
    c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 8) as u32);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f48));
    c.g[A0][0] = c.g[S0][0];
    c.g[T2][0] = sx(((c.g[T2][0] as u32) as i32 >> 24) as u32);
    // Registers at the call: api, format, bytes... (debug printf).
    callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9070)?;
    c.g[V0][0] = c.lb(c.g[S1][0] as u32);
    c.g[A2][0] = 0u64;
    let t = c.g[V0][0] == 0u64;
    c.g[T0][0] = 0u64;
    if t {
        debug_packet_length_line(c, frame)?;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
    let t = c.g[V0][0] == 0u64;
    if t {
        debug_packet_length_line(c, frame)?;
        return Ok(());
    }
    c.cycles += 3;
    c.g[A3][0] = 0xd;
    c.g[V1][0] = c.g[S1][0];
    c.g[A1][0] = c.lb(c.g[S1][0] as u32);
    'l_3e9098: loop {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 32i64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
        // Tab/newline skips the advance below (breaks the inner block).
        let mut tab = false;
        if t {
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x9));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                tab = true;
            } else {
                c.cycles += 2;
                if c.g[A1][0] != c.g[A3][0] {
                    c.g[T0][0] = 0x1;
                    debug_packet_length_line(c, frame)?;
                    return Ok(());
                } else {
                }
            }
        }
        if !tab {
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        }
        c.cycles += 3;
        c.g[V0][0] = c.lb(c.g[V1][0] as u32);
        let t = c.g[V0][0] == 0u64;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
        if t {
            debug_packet_length_line(c, frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[A1][0] = c.lb(c.g[V1][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x003e_9098);
            }
            continue 'l_3e9098;
        } else {
            debug_packet_length_line(c, frame)?;
            return Ok(());
        }
    }
}

/// 0x3e90f8/0x3e9118/0x3e9138/0x3e9148 length lines of
/// `lobby_api_debug_packet`, then the epilogue.
fn debug_packet_length_line(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 2;
    let t = c.g[A2][0] != 0u64;
    c.g[V0][0] = 0x1;
    if t {
    } else {
        c.cycles += 6;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = c.g[S0][0];
        // Registers at the call: api, format (debug printf).
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f68));
        callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_90f0)?;
        let t = true;
        frame.restore(c, 16, 0x30);
        if t {
            frame_epilogue_debug_packet(c, frame);
            return Ok(());
        }
        frame_epilogue_debug_packet(c, frame);
        return Ok(());
    }
    c.cycles += 2;
    let t = c.g[T0][0] != c.g[V0][0];
    c.g[V0][0] = ((c.g[A2][0] as i64) < 241i64) as u64;
    if t {
    } else {
        c.cycles += 6;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = c.g[S0][0];
        // Registers at the call: api, format (debug printf).
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f78));
        callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9110)?;
        let t = true;
        frame.restore(c, 16, 0x30);
        if t {
            frame_epilogue_debug_packet(c, frame);
            return Ok(());
        }
        frame_epilogue_debug_packet(c, frame);
        return Ok(());
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = 0x4a0000;
    if t {
    } else {
        c.cycles += 6;
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = c.g[S0][0];
        // Registers at the call: api, format (debug printf).
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f88));
        callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9130)?;
        let t = true;
        frame.restore(c, 16, 0x30);
        if t {
            frame_epilogue_debug_packet(c, frame);
            return Ok(());
        }
        frame_epilogue_debug_packet(c, frame);
        return Ok(());
    }
    c.cycles += 5;
    // Registers at the call: api, format, payload (debug printf).
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48f0));
    c.g[A2][0] = c.g[S1][0];
    callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9148)?;
    frame.restore(c, 16, 0x30);
    frame_epilogue_debug_packet(c, frame);
    Ok(())
}

/// 0x3e914c epilogue of `lobby_api_debug_packet`.
fn frame_epilogue_debug_packet(c: &mut Cpu, frame: &Frame) {
    c.cycles += 4;
    frame.restore(c, 17, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `int LobbyApi_TrimRankList(int a0 /*a0*/, int cap /*a1*/)` (0x3e9160):
/// trim one rank list to `cap` entries: histogram the entry ranks into a
/// scratch table, walk the histogram from the top to find the cutoff rank,
/// renumber the survivors, and delete (hash-unlink, list-remove, free) the
/// entries ranked 0. Returns the survivor count, -1 when the list is empty.
pub fn lobby_api_trim_rank_list(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_9160 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x80);
                    frame.save(c, 22, 0x10);
                    frame.save(c, 16, 0x70);
                    c.g[S6][0] = c.g[A1][0];
                    frame.save(c, 17, 0x60);
                    frame.save(c, 18, 0x50);
                    frame.save(c, 19, 0x40);
                    frame.save(c, 20, 0x30);
                    frame.save_ra(c, 0);
                    frame.save(c, 21, 0x20);
                    c.g[S5][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x338));
                    let t = c.g[S5][0] == 0u64;
                    c.g[S3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x33c));
                    if t {
                        pc = 0x003e_919c;
                    } else {
                        pc = 0x003e_9194;
                    }
                }
                0x003e_9194 => {
                    c.cycles += 2;
                    if c.g[S3][0] != 0u64 {
                        c.g[A0][0] = 0x4000;
                        pc = 0x003e_91a4;
                    } else {
                        pc = 0x003e_919c;
                    }
                }
                0x003e_919c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x003e_934c;
                    } else {
                        pc = 0x003e_91a4;
                    }
                }
                0x003e_91a4 => {
                    c.cycles += 10;
                    // Registers at the call: size in a0.
                    c.g[S2][0] = 0u64;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003e_91ac)?;
                    c.g[S1][0] = 0x1f4;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    // Registers at the call: block, zero, size (memset).
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = 0x4000;
                    callee::MEMSET.call(c, 0x003e_91c4)?;
                    let t = true;
                    c.g[S0][0] = 0x4a0000;
                    if t {
                        pc = 0x003e_9214;
                    } else {
                        pc = 0x003e_91cc;
                    }
                }
                0x003e_91cc => {
                    c.cycles += 1;
                    pc = 0x003e_91d0;
                }
                0x003e_91d0 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x1fffu64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        pc = 0x003e_91fc;
                    } else {
                        pc = 0x003e_91e0;
                    }
                }
                0x003e_91e0 => {
                    c.cycles += 7;
                    c.sw(c.g[A1][0] as u32, c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48f8));
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sh((c.g[A1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    pc = 0x003e_91fc;
                }
                0x003e_91fc => {
                    c.cycles += 6;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sh(c.g[V1][0] as u32, c.g[V0][0]);
                    pc = 0x003e_9214;
                }
                0x003e_9214 => {
                    c.cycles += 6;
                    // Registers at the call: list, index.
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_9220)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_91d0) {
                            return Ok(());
                        }
                        pc = 0x003e_91d0;
                    } else {
                        pc = 0x003e_922c;
                    }
                }
                0x003e_922c => {
                    c.cycles += 3;
                    c.g[S0][0] = 0x1;
                    c.g[S2][0] = 0x1fff;
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3ffe));
                    pc = 0x003e_9238;
                }
                0x003e_9238 => {
                    c.cycles += 6;
                    c.g[S1][0] = c.lh(c.g[V1][0] as u32);
                    c.sh(c.g[V1][0] as u32, c.g[S0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        pc = 0x003e_92a0;
                    } else {
                        pc = 0x003e_9250;
                    }
                }
                0x003e_9250 => {
                    c.cycles += 5;
                    // Registers at the call: list.
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_COUNT.call(c, 0x003e_9258)?;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x003e_92a8;
                    } else {
                        pc = 0x003e_9264;
                    }
                }
                0x003e_9264 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x003e_9284;
                    } else {
                        pc = 0x003e_926c;
                    }
                }
                0x003e_926c => {
                    c.cycles += 1;
                    pc = 0x003e_9270;
                }
                0x003e_9270 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.sw(c.g[A1][0] as u32, 0u64);
                        pc = 0x003e_927c;
                    } else {
                        pc = 0x003e_927c;
                    }
                }
                0x003e_927c => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x003e_9284;
                }
                0x003e_9284 => {
                    c.cycles += 5;
                    // Registers at the call: list, index.
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_928c)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_9270) {
                            return Ok(());
                        }
                        pc = 0x003e_9270;
                    } else {
                        pc = 0x003e_9298;
                    }
                }
                0x003e_9298 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x003e_92ac;
                    } else {
                        pc = 0x003e_92a0;
                    }
                }
                0x003e_92a0 => {
                    c.cycles += 2;
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                    if t {
                        if back_edge_unwinds(c, 0x003e_9238) {
                            return Ok(());
                        }
                        pc = 0x003e_9238;
                    } else {
                        pc = 0x003e_92a8;
                    }
                }
                0x003e_92a8 => {
                    c.cycles += 1;
                    c.g[S1][0] = 0u64;
                    pc = 0x003e_92ac;
                }
                0x003e_92ac => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x003e_92d4;
                    } else {
                        pc = 0x003e_92b4;
                    }
                }
                0x003e_92b4 => {
                    c.cycles += 1;
                    pc = 0x003e_92b8;
                }
                0x003e_92b8 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    if t {
                        pc = 0x003e_92d0;
                    } else {
                        pc = 0x003e_92c0;
                    }
                }
                0x003e_92c0 => {
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V1][0] = c.lh(c.g[V0][0] as u32);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    pc = 0x003e_92d0;
                }
                0x003e_92d0 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x003e_92d4;
                }
                0x003e_92d4 => {
                    c.cycles += 6;
                    // Registers at the call: list, index.
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_92e0)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_92b8) {
                            return Ok(());
                        }
                        pc = 0x003e_92b8;
                    } else {
                        pc = 0x003e_92ec;
                    }
                }
                0x003e_92ec => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x003e_9328;
                    } else {
                        pc = 0x003e_92f4;
                    }
                }
                0x003e_92f4 => {
                    c.cycles += 1;
                    pc = 0x003e_92f8;
                }
                0x003e_92f8 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x003e_9328;
                    } else {
                        pc = 0x003e_9300;
                    }
                }
                0x003e_9300 => {
                    c.cycles += 10;
                    // Registers at the call: hash, name.
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[S5][0];
                    callee::LOBBY_HASH_DEL_STR.call(c, 0x003e_930c)?;
                    // Registers at the call: list, entry.
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_9318)?;
                    // Registers at the call: removed entry.
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003e_9320)?;
                    let t = true;
                    c.g[A1][0] = c.g[S2][0];
                    if t {
                        pc = 0x003e_932c;
                    } else {
                        pc = 0x003e_9328;
                    }
                }
                0x003e_9328 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S2][0];
                    pc = 0x003e_932c;
                }
                0x003e_932c => {
                    c.cycles += 5;
                    // Registers at the call: list, index.
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_9334)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_92f8) {
                            return Ok(());
                        }
                        pc = 0x003e_92f8;
                    } else {
                        pc = 0x003e_9340;
                    }
                }
                0x003e_9340 => {
                    c.cycles += 3;
                    // Registers at the call: scratch table.
                    c.g[A0][0] = c.g[S4][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003e_9348)?;
                    c.g[V0][0] = c.g[S1][0];
                    pc = 0x003e_934c;
                }
                0x003e_934c => {
                    c.cycles += 10;
                    let frame = Frame { size: 0x80 };
                    frame.restore(c, 16, 0x70);
                    frame.restore(c, 17, 0x60);
                    frame.restore(c, 18, 0x50);
                    frame.restore(c, 19, 0x40);
                    frame.restore(c, 20, 0x30);
                    frame.restore(c, 21, 0x20);
                    frame.restore(c, 22, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x003e_9160, 0x003e_9374);
                    return Ok(());
                }
            }
        }
    })
}

/// `int LobbyApi_PickRoom(LobbyApi* api /*a0*/, int want /*a1*/, uint mask
/// /*a2*/, uint bits /*a3*/, char* name /*t0*/, int best /*t1*/)` (0x3e9378):
/// pick the fullest matching visible room: sort the room list, walk it for
/// flag- and name-matching rooms, score each by `100*players/capacity`, keep
/// the best under `want` (falling back to the emptiest over it). Returns the
/// winning index, or the incoming best when nothing qualifies.
pub fn lobby_api_pick_room(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_9378 => {
                    c.cycles += 21;
                    let frame = Frame::open(c, 0xb0);
                    frame.save(c, 21, 0x50);
                    frame.save(c, 19, 0x70);
                    c.g[S5][0] = c.g[T1][0];
                    frame.save(c, 20, 0x60);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, 22, 0x40);
                    c.g[S4][0] = c.g[T0][0];
                    frame.save(c, 23, 0x30);
                    c.g[S6][0] = c.g[S5][0];
                    frame.save(c, 30, 0x20);
                    c.g[S7][0] = 0xffffffffffffffff;
                    frame.save(c, 16, 0xa0);
                    c.g[FP][0] = 0x3e7;
                    frame.save(c, 17, 0x90);
                    frame.save(c, 18, 0x80);
                    frame.save_ra(c, 0x10);
                    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    let t = c.g[S4][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    if t {
                        pc = 0x003e_93d4;
                    } else {
                        pc = 0x003e_93cc;
                    }
                }
                0x003e_93cc => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x4a0000;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4888));
                    pc = 0x003e_93d4;
                }
                0x003e_93d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S5][0];
                    if t {
                        pc = 0x003e_94c0;
                    } else {
                        pc = 0x003e_93e0;
                    }
                }
                0x003e_93e0 => {
                    c.cycles += 8;
                    // Registers at the call: list.
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    c.g[S2][0] = 0u64;
                    callee::LOBBY_LIST_SORT.call(c, 0x003e_93ec)?;
                    // Registers at the call: list, stamp.
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003e_93f8)?;
                    let t = true;
                    if t {
                        pc = 0x003e_94a0;
                    } else {
                        pc = 0x003e_9400;
                    }
                }
                0x003e_9400 => {
                    c.cycles += 9;
                    // Registers at the call: list, index.
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_9408)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x003e_94a0;
                    } else {
                        pc = 0x003e_9424;
                    }
                }
                0x003e_9424 => {
                    c.cycles += 9;
                    // Registers at the call: name (strlen).
                    c.g[A0][0] = c.g[S4][0];
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
                    callee::STRLEN.call(c, 0x003e_9430)?;
                    // Registers at the call: room name, want, len (strncmp).
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, callee::STRNCMP, 0x003e_9440)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x003e_94a0;
                    } else {
                        pc = 0x003e_9448;
                    }
                }
                0x003e_9448 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = 0x64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.mult(0, 4, 4, 3);
                    if c.g[V0][0] == 0u64 {
                        pc = 0x003e_9460;
                    } else {
                        pc = 0x003e_9460;
                    }
                }
                0x003e_9460 => {
                    c.cycles += 6;
                    c.div(0, 4, 2);
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = c.lo[0];
                    c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if t {
                        pc = 0x003e_9488;
                    } else {
                        pc = 0x003e_9478;
                    }
                }
                0x003e_9478 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[FP][0] as i64)) as u64;
                    if t {
                        pc = 0x003e_948c;
                    } else {
                        pc = 0x003e_9480;
                    }
                }
                0x003e_9480 => {
                    c.cycles += 2;
                    c.g[S5][0] = c.g[S2][0];
                    c.g[S7][0] = c.g[A0][0];
                    pc = 0x003e_9488;
                }
                0x003e_9488 => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[FP][0] as i64)) as u64;
                    pc = 0x003e_948c;
                }
                0x003e_948c => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x003e_94a0;
                    } else {
                        pc = 0x003e_9494;
                    }
                }
                0x003e_9494 => {
                    c.cycles += 3;
                    c.g[FP][0] = c.g[A0][0];
                    c.g[S6][0] = c.g[S2][0];
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x003e_94a0;
                }
                0x003e_94a0 => {
                    c.cycles += 5;
                    // Registers at the call: list.
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    callee::LOBBY_LIST_VISIBLE_COUNT.call(c, 0x003e_94a8)?;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                        if back_edge_unwinds(c, 0x003e_9400) {
                            return Ok(());
                        }
                        pc = 0x003e_9400;
                    } else {
                        pc = 0x003e_94b4;
                    }
                }
                0x003e_94b4 => {
                    c.cycles += 2;
                    if (c.g[S7][0] as i64) < 0 {
                        c.g[S5][0] = c.g[S6][0];
                        pc = 0x003e_94bc;
                    } else {
                        pc = 0x003e_94bc;
                    }
                }
                0x003e_94bc => {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S5][0];
                    pc = 0x003e_94c0;
                }
                0x003e_94c0 => {
                    c.cycles += 12;
                    let frame = Frame { size: 0xb0 };
                    frame.restore(c, 16, 0xa0);
                    frame.restore(c, 17, 0x90);
                    frame.restore(c, 18, 0x80);
                    frame.restore(c, 19, 0x70);
                    frame.restore(c, 20, 0x60);
                    frame.restore(c, 21, 0x50);
                    frame.restore(c, 22, 0x40);
                    frame.restore(c, 23, 0x30);
                    frame.restore(c, 30, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x003e_9378, 0x003e_94f0);
                    return Ok(());
                }
            }
        }
    })
}

/// `void LobbyApi_ServerPingCallback(void* api /*a0*/, void* packet /*a1*/)`
/// (0x3e94f0): handle a ping reply packet: ignore non-`"ping"` tags; else
/// look the sequence number up in the packet, ticking the net clock on a
/// live sequence or logging and counting down the server-ping retries on a
/// stale one (resetting the counter every third miss).
pub fn lobby_api_server_ping_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x70690000;
        frame.save(c, 16, 0x10);
        c.g[V0][0] = c.g[V0][0] | 0x6e67;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if t {
            frame_epilogue_server_ping_callback(c, &frame);
            return Ok(());
        }
        c.cycles += 9;
        // Registers at the call: field, key (find).
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4898));
        callee::TAG_FIELD_FIND.call(c, 0x003e_9520)?;
        // Registers at the call: field, def (get number).
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e_952c)?;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = 0x490000;
        if t {
            // Live reply: 0x9574 tick tail, then the shared epilogue.
            server_ping_callback_tick(c, &frame)?;
            frame_epilogue_server_ping_callback(c, &frame);
            return Ok(());
        }
        c.cycles += 8;
        // Registers at the call: api, format (debug printf).
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f98));
        callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9540)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = 0x3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        let t = c.g[V0][0] != c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        if t {
            // Below the ping-count trip: tick once (0x9564), then epilogue.
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_9564);
            callee::NET_TICK_WRAPPER2.call(c, 0x003e_9564)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
            frame_epilogue_server_ping_callback(c, &frame);
            return Ok(());
        }
        c.cycles += 2;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
        frame_epilogue_server_ping_callback(c, &frame);
        Ok(())
    })
}

/// 0x3e9574 tail of `lobby_api_server_ping_callback`: tick and store.
fn server_ping_callback_tick(c: &mut Cpu, _frame: &Frame) -> crate::call::Flow {
    c.cycles += 5;
    // Registers at the call: (none) tick the net clock.
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
    callee::NET_TICK_WRAPPER2.call(c, 0x003e_9574)?;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7530));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
    Ok(())
}

/// 0x3e9580 epilogue of `lobby_api_server_ping_callback`.
fn frame_epilogue_server_ping_callback(c: &mut Cpu, frame: &Frame) {
    c.cycles += 4;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void LobbyApi_PingServer(LobbyApi* api /*a0*/)` (0x3e9590): ping the
/// lobby server: issue a `"ping"` request at_3 the server address with the
/// default id. Thin wrapper around `LobbyApi_Ping`.
pub fn lobby_api_ping_server(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x4a0000;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0x3f0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4900));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6b10));
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3b4));
        c.g[A3][0] = 0xbb8;
        c.g[T1][0] = 0u64;
        // Registers at the call: api, server key, addr, id, t0, t1 (ping).
        call_ptr(c, callee::LOBBY_API_PING, 0x003e_95b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char* TagField_ParseDecimal(char* p /*a0*/, uint* out /*a1*/)`
/// (0x3eae68): parse an unsigned decimal at_3 `p`, storing the running value
/// into `*out` after each digit. Returns the pointer just past the digits.
pub fn tag_field_parse_decimal(c: &mut Cpu) {
    c.cycles += 6;
    c.sw(c.g[A1][0] as u32, 0u64);
    c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[A2][0] = 0xa;
    if t {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.g[A0][0];
        c.pc = t;
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
    'l_3eae88: loop {
        c.cycles += 10;
        c.g[V1][0] = c.g[V1][0] & 0xf;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.mult(0, 2, 2, 6);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lbu(c.g[A0][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x003e_ae88);
            }
            continue 'l_3eae88;
        } else {
            break 'l_3eae88;
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A0][0];
    c.pc = t;
}

/// `void* ds_secstotime(void* out /*a0*/, int secs /*a1*/)` (0x3eaeb8):
/// split seconds into a date/time record: seconds, minutes, hours, the year
/// day count walked down through the leap-year table, then the month loop.
/// Returns `out`.
pub fn ds_secstotime(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_aeb8 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0x10000;
                    c.g[A1][0] = c.g[A1][0] << 32;
                    c.g[A1][0] = ((c.g[A1][0] as i64) >> 32) as u64;
                    c.g[V1][0] = c.g[V1][0] | 0x5180;
                    c.g[A2][0] = 0x3c;
                    c.divu(0, 5, 3);
                    c.g[V0][0] = 0x10000;
                    if c.g[V1][0] == 0u64 {
                        pc = 0x003e_aedc;
                    } else {
                        pc = 0x003e_aedc;
                    }
                }
                0x003e_aedc => {
                    c.cycles += 20;
                    c.g[V0][0] = c.g[V0][0] | 0x5180;
                    c.g[T6][0] = c.g[A0][0];
                    c.g[T0][0] = 0x3c;
                    c.g[T1][0] = 0x3c;
                    c.g[T2][0] = 0x3c;
                    c.g[T5][0] = 0x7b2;
                    c.g[A1][0] = c.lo[0];
                    c.g[A3][0] = c.hi[0];
                    c.g[T4][0] = c.g[A1][0];
                    c.div(0, 7, 6);
                    c.g[V1][0] = c.hi[0];
                    c.g[V0][0] = c.lo[0];
                    c.sw(c.g[T6][0] as u32, c.g[V1][0]);
                    c.div(0, 2, 6);
                    c.g[A0][0] = c.hi[0];
                    c.g[V0][0] = c.lo[0];
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    let t = true;
                    c.g[T7][0] = 0x490000;
                    if t {
                        pc = 0x003e_afdc;
                    } else {
                        pc = 0x003e_af2c;
                    }
                }
                0x003e_af2c => {
                    c.cycles += 1;
                    pc = 0x003e_af30;
                }
                0x003e_af30 => {
                    c.cycles += 10;
                    c.g[V1][0] = 0x16e;
                    c.g[A2][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x1));
                    c.div(0, 12, 3);
                    c.g[T0][0] = 0x64;
                    c.g[T1][0] = 0x190;
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[T5][0] as u32).wrapping_add(0x2));
                    if c.g[V1][0] == 0u64 {
                        pc = 0x003e_af58;
                    } else {
                        pc = 0x003e_af58;
                    }
                }
                0x003e_af58 => {
                    c.cycles += 33;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = c.g[A2][0];
                    }
                    c.g[V0][0] = 0x64;
                    c.g[V1][0] = 0x190;
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
                    c.g[T3][0] = 0x16d;
                    c.g[A0][0] = c.lo[0];
                    c.div(0, 6, 8);
                    c.g[A0][0] = sx((c.g[T5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.g[A0][0] ^ c.g[T5][0];
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = c.g[V0][0];
                    }
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                    c.g[T2][0] = ((c.g[T2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[T2][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    c.g[A3][0] = c.lo[0];
                    c.div(0, 6, 9);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[T5][0] as u32));
                    c.g[T5][0] = c.g[A0][0];
                    c.g[A2][0] = c.lo[0];
                    c.div(0, 3, 8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[T0][0] = c.lo[0];
                    c.div(0, 3, 9);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[T0][0] as u32));
                    c.g[V1][0] = c.lo[0];
                    c.mult(0, 7, 7, 11);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[A3][0] as u32));
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    pc = 0x003e_afdc;
                }
                0x003e_afdc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[T5][0] & 0x3;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = 0x16d;
                    if t {
                        pc = 0x003e_b024;
                    } else {
                        pc = 0x003e_afe8;
                    }
                }
                0x003e_afe8 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x64;
                    c.div(0, 13, 2);
                    if c.g[V0][0] == 0u64 {
                        pc = 0x003e_aff8;
                    } else {
                        pc = 0x003e_aff8;
                    }
                }
                0x003e_aff8 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.hi[0];
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = 0x16e;
                    if t {
                        pc = 0x003e_b024;
                    } else {
                        pc = 0x003e_b004;
                    }
                }
                0x003e_b004 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x190;
                    c.div(0, 13, 2);
                    if c.g[V0][0] == 0u64 {
                        pc = 0x003e_b014;
                    } else {
                        pc = 0x003e_b014;
                    }
                }
                0x003e_b014 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.hi[0];
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = 0x16d;
                    if t {
                        pc = 0x003e_b024;
                    } else {
                        pc = 0x003e_b020;
                    }
                }
                0x003e_b020 => {
                    c.cycles += 1;
                    c.g[A0][0] = 0x16e;
                    pc = 0x003e_b024;
                }
                0x003e_b024 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x30;
                    if t {
                        if back_edge_unwinds(c, 0x003e_af30) {
                            return Ok(());
                        }
                        pc = 0x003e_af30;
                    } else {
                        pc = 0x003e_b030;
                    }
                }
                0x003e_b030 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[A0][0] & 0x1;
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x76c));
                    c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_add(0x69e0));
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x1c), c.g[T4][0]);
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x10), 0u64);
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if t {
                        pc = 0x003e_b068;
                    } else {
                        pc = 0x003e_b054;
                    }
                }
                0x003e_b054 => {
                    c.cycles += 1;
                    pc = 0x003e_b058;
                }
                0x003e_b058 => {
                    c.cycles += 4;
                    c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    pc = 0x003e_b068;
                }
                0x003e_b068 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x10));
                        if back_edge_unwinds(c, 0x003e_b058) {
                            return Ok(());
                        }
                        pc = 0x003e_b058;
                    } else {
                        pc = 0x003e_b078;
                    }
                }
                0x003e_b078 => {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.g[T6][0];
                    c.sw((c.g[T6][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    let t = c.g[RA][0] as u32;
                    c.sw((c.g[T6][0] as u32).wrapping_add(0x20), 0u64);
                    c.pc = t;
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x003e_aeb8, 0x003e_b08c);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::callee;
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{fall_back, hook, prev_cell};
    use crate::difftest::{check, check_lohi, fill, scramble, Rng, DATA, GAME_GP, RA};
    use ssx3_ee_rt::cpu::reg::*;

    /// Fixture arena: the lobby, list/hash structs and tag text.
    const LOBBY: u32 = DATA + 0x100;
    /// Gated-fake control block for the allocator fake.
    const LOBBY_MEM_CTRL: u32 = DATA + 0xd_8000;
    /// Magic word enabling the allocator/free fakes.
    const LOBBY_MEM_MAGIC: u32 = 0x10_b10c;
    /// Bump pool the allocator fake hands out (setup pre-zeroes it).
    const LOBBY_MEM_POOL: u32 = DATA + 0xe_0000;
    /// Gated-fake control for the tick-syscall fake.
    const LOBBY_TICK_CTRL: u32 = DATA + 0xd_8100;
    /// Magic word enabling the tick-syscall fake.
    const LOBBY_TICK_MAGIC: u32 = 0x71_cc10;
    /// Game-data stubs: tag keys the ports look up.
    const KEY_AUTH_SRC: u32 = 0x004a_48d8;
    const KEY_SNAP: u32 = 0x004a_48e8;
    const KEY_AUTH_DEF: u32 = 0x004a_4888;
    const KEY_SEQ: u32 = 0x004a_4898;

    /// `EA_MemAlloc` (0x317e30, reached via `Lobby_MemAlloc`) for the lobby
    /// tests: with the gate off runs the previously hooked body; with
    /// `LOBBY_MEM_MAGIC` at `LOBBY_MEM_CTRL` returns the next 16-aligned
    /// bump block from `LOBBY_MEM_POOL` (`+4`: bump offset, `+8`: mode —
    /// 0 bump, 1 always NULL for alloc-fail arms), logging size and ra.
    /// Reusable shape for the later `LobbyApi_Update` lane: seed-steer the
    /// mode word per test; NULL forces every guarded alloc-fail path.
    pub(crate) fn lobby_fake_alloc(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(LOBBY_MEM_CTRL) != LOBBY_MEM_MAGIC {
            return fall_back(c, prev_cell(lobby_fake_alloc), ee::s10::f_00317e30__generated);
        }
        let mode = c.bus.read32(LOBBY_MEM_CTRL + 8);
        if mode == 1 {
            c.g[V0][0] = 0;
            c.cycles += 11;
            ret(c);
            return;
        }
        let off = c.bus.read32(LOBBY_MEM_CTRL + 4);
        let size = (c.g[A0][0] as u32 + 15) & !15;
        c.bus.write32(LOBBY_MEM_CTRL + 4, off + size);
        c.bus.write32(LOBBY_MEM_CTRL + 0x10, c.g[A0][0] as u32);
        c.bus.write32(
            LOBBY_MEM_CTRL + 0x14,
            c.g[ssx3_ee_rt::cpu::reg::RA][0] as u32,
        );
        c.g[V0][0] = (LOBBY_MEM_POOL + off) as u64;
        c.cycles += 11;
        ret(c);
    }

    /// `MEM_FreeInternal` path (0x317e98, reached via `Lobby_MemFree`) for
    /// the lobby tests: log-only no-op with the gate on (the freed lists
    /// and hashes are fixture memory), previous body with the gate off.
    pub(crate) fn lobby_fake_free(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(LOBBY_MEM_CTRL) != LOBBY_MEM_MAGIC {
            return fall_back(c, prev_cell(lobby_fake_free), ee::s10::f_00317e98__generated);
        }
        c.bus.write32(LOBBY_MEM_CTRL + 0x20, c.g[A0][0] as u32);
        c.g[V0][0] = 0;
        c.g[V1][0] = 0;
        c.cycles += 5;
        ret(c);
    }

    /// The syscall dispatcher (0x411538, reached via `NetTick` ←
    /// `ProtoAriesTick`/`NetTick_Wrapper2`) for the lobby tests: with the
    /// gate on return a canned tick (`LOBBY_TICK_CTRL+4`), so packet tests
    /// never touch the host clock; with the gate off run the previous body.
    pub(crate) fn lobby_fake_tick_syscall(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(LOBBY_TICK_CTRL) != LOBBY_TICK_MAGIC {
            return fall_back(
                c,
                prev_cell(lobby_fake_tick_syscall),
                ee::s15::f_00411538__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(LOBBY_TICK_CTRL + 4) as u64;
        c.cycles += 7;
        ret(c);
    }

    /// Hook the three lobby fakes (idempotent per process).
    fn hook_lobby_fakes() {
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lobby_fake_alloc,
            prev_cell(lobby_fake_alloc),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lobby_fake_free,
            prev_cell(lobby_fake_free),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00411538),
            lobby_fake_tick_syscall,
            prev_cell(lobby_fake_tick_syscall),
        );
    }

    /// Write a NUL-terminated string.
    fn put_str(c: &mut Cpu, a: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(a + i as u32, *b);
        }
        c.bus.write8(a + s.len() as u32, 0);
    }

    /// Game-data stubs the ports read: tag keys, printf formats, the
    /// TagField character tables, the month table and the room-name key.
    fn lobby_tables(c: &mut Cpu) {
        put_str(c, KEY_AUTH_SRC, b"conn");
        put_str(c, KEY_SNAP, b"snap");
        put_str(c, KEY_AUTH_DEF, b"auth");
        put_str(c, KEY_SEQ, b"seq");
        put_str(c, 0x004a_4888, b"room");
        put_str(c, 0x0049_5f48, b"%d %d %d %d %d %d");
        put_str(c, 0x0049_5f68, b"empty");
        put_str(c, 0x0049_5f78, b"binary");
        put_str(c, 0x0049_5f88, b"long");
        put_str(c, 0x004a_48f0, b"%s");
        for i in 0..256u32 {
            let cls = if (b'a'..=b'z').contains(&(i as u8))
                || (b'A'..=b'Z').contains(&(i as u8))
                || (b'0'..=b'9').contains(&(i as u8))
                || i == b'_' as u32
                || i == b'-' as u32
                || i == b'.' as u32
            {
                2
            } else if i == b'=' as u32 || i == b':' as u32 {
                1
            } else {
                0
            };
            c.bus.write8(0x0049_64c0 + i, cls);
            c.bus.write8(0x0049_63c0 + i, (i as u8).to_ascii_uppercase());
        }
        for (i, days) in [
            31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31,
            30, 31,
        ]
        .iter()
        .enumerate()
        {
            c.bus.write32(0x0049_69e0 + i as u32 * 4, *days);
        }
        // Rank-template word/halfword read by TrimRankList.
        c.bus.write32(0x004a_48f8, 500);
        c.bus.write16(0x004a_48fc, 0);
        // Name key and server key for PickRoom/PingServer.
        put_str(c, 0x004a_4900, b"room");
    }

    /// Zero the lobby struct and point A0 at it.
    fn lobby_base(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let _ = r;
        let _ = seed;
        for w in 0..0x180u32 {
            c.bus.write32(LOBBY + 4 * w, 0);
        }
        c.g[A0][0] = sx(LOBBY);
    }

    /// A `Lobby_List` at `a` with `n` entries: `+0x14` visible count,
    /// `+0x18` count, `+0x40` entry array, `+0x38` grow flag, `+0x24`
    /// callback guard (0 skips the sort callback).
    fn make_list(c: &mut Cpu, a: u32, n: u32, array: u32) {
        c.bus.write32(a + 0x8, 0);
        c.bus.write32(a + 0xc, 0);
        c.bus.write32(a + 0x10, 0);
        c.bus.write32(a + 0x14, n);
        c.bus.write32(a + 0x18, n);
        c.bus.write32(a + 0x24, 0);
        c.bus.write32(a + 0x38, 1);
        c.bus.write32(a + 0x3c, 0);
        c.bus.write32(a + 0x40, array);
    }

    /// A `Lobby_Hash` at `a` with one bucket holding `head`: `+0x10`
    /// bucket count 1, `+0x14` buckets, `+0x20` strcmp compare.
    fn make_hash(c: &mut Cpu, a: u32, buckets: u32, head: u32, count: u32, free: u32) {
        c.bus.write32(a, count);
        c.bus.write32(a + 4, 0);
        c.bus.write32(a + 0x10, 1);
        c.bus.write32(a + 0x14, buckets);
        c.bus.write32(a + 0x18, 0);
        c.bus.write32(a + 0x1c, free);
        c.bus.write32(a + 0x20, 0x0041_65a8);
        c.bus.write32(buckets, head);
    }

    driven!(destroy_list_gen, ee::s14::f_003e8688__generated);
    driven!(destroy_list_hand, lobby_api_destroy_list);

    /// `lobby_api_destroy_list`: kind (seed % 9, 8 = out of range), list
    /// (seed % 3 == 1: null), slot word (seed % 2: cleared or still the
    /// list). | kind>=8 | list null | slot mismatch | slot match |
    #[test]
    fn lobby_api_destroy_list_matches() {
        hook_lobby_fakes();
        check_events(destroy_list_gen, destroy_list_hand, 48, &|c, seed| {
            let mut r = Rng(0x8688 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            lobby_tables(c);
            lobby_base(c, &mut r, seed);
            c.bus.write32(LOBBY_MEM_CTRL, LOBBY_MEM_MAGIC);
            c.bus.write32(LOBBY_MEM_CTRL + 4, 0x1000);
            c.bus.write32(LOBBY_MEM_CTRL + 8, 0);
            for w in 0..0x100u32 {
                c.bus.write32(LOBBY_MEM_POOL + 4 * w, 0);
            }
            let kind = seed % 9;
            let list = DATA + 0x1000;
            let hash = DATA + 0x2000;
            let buckets = DATA + 0x2100;
            let array = DATA + 0x2200;
            let entry = DATA + 0x2300;
            let with_list = kind < 8 && seed % 3 != 1;
            if with_list {
                // List whose hash holds one entry (two pops: value, empty).
                make_list(c, list, 2, array);
                c.bus.write32(list + 0x34, hash);
                make_hash(c, hash, buckets, entry, 1, 0);
                // Entry: no chain, pop value 0x1111.
                c.bus.write32(entry, 0);
                c.bus.write32(entry + 4, 0x1111);
                put_str(c, entry + 8, b"kay");
                // Array: [hash, 0x1111] so both removes hit index 0.
                c.bus.write32(array, hash);
                c.bus.write32(array + 4, 0x1111);
                // Slot words the tail verifies/clears.
                let slot = LOBBY + 0x314 + kind * 0x14;
                c.bus.write32(slot, if seed % 2 == 0 { 0 } else { list });
            }
            let mode = schedule(c, &mut r, seed, 0x003e_8688);
            // Set argument registers after schedule (thread start clobbers them).
            c.g[A1][0] = sx(kind);
            c.g[A2][0] = if with_list {
                sx(list)
            } else if kind < 8 {
                0
            } else {
                sx(list)
            };
            c.pc = 0x003e_8688;
            mode
        });
    }

    /// `lobby_api_add_list_entry`: kind (seed % 10: 8,9 bad; 0-2 range or
    /// empty by seed; 3 name list; 4-7 ranked), dup-hit (seed / 10 % 2),
    /// entry alignment (seed / 20 % 2: aligned `ld/sd` vs unaligned
    /// `ldl/sdr` copy). | bad kind | empty slot | range fail | dup |
    /// added (name) | added (ranked, both copies) |
    #[test]
    fn lobby_api_add_list_entry_matches() {
        hook_lobby_fakes();
        check_lohi(
            ee::s14::f_003e8778__generated,
            lobby_api_add_list_entry,
            64,
            &|c, seed| {
                let mut r = Rng(0x8778 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                c.bus.write32(LOBBY_MEM_CTRL, LOBBY_MEM_MAGIC);
                c.bus.write32(LOBBY_MEM_CTRL + 4, 0x2000);
                c.bus.write32(LOBBY_MEM_CTRL + 8, 0);
                for w in 0..0x200u32 {
                    c.bus.write32(LOBBY_MEM_POOL + 4 * w, 0);
                }
                let kind = seed % 10;
                let hit = seed / 10 % 2 == 0;
                let off = (seed / 20 % 2) as u32;
                let slot_list = DATA + 0x1000;
                let hash = DATA + 0x1400;
                let buckets = DATA + 0x1500;
                let array = DATA + 0x1600;
                let entry = DATA + 0x1700;
                let free = DATA + 0x1800;
                let name = DATA + 0x9000 + off;
                let entr = DATA + 0x9100 + off;
                for w in 0..0x60u32 {
                    c.bus.write32(DATA + 0x9000 + 4 * w, 0);
                    c.bus.write32(DATA + 0x9100 + 4 * w, 0);
                }
                put_str(c, name, b"kay");
                put_str(c, entr + 8, b"kay");
                make_list(c, slot_list, 2, array);
                c.bus.write32(array, DATA + 0x4000);
                c.bus.write32(array + 4, DATA + 0x4100);
                make_hash(c, hash, buckets, if hit { entry } else { 0 }, 1, free);
                c.bus.write32(free, 0);
                c.bus.write32(free + 4, 0);
                put_str(c, entry + 8, b"kay");
                c.bus.write32(entry, 0);
                if kind < 3 {
                    // Range-fail (slot set) or empty-slot fail by seed.
                    let slot = LOBBY + 0x310 + kind * 0x14;
                    c.bus.write32(slot, if hit { slot_list } else { 0 });
                } else if kind == 3 {
                    c.bus.write32(LOBBY + 0x34c, hash);
                    c.bus.write32(LOBBY + 0x350, slot_list);
                } else if kind < 8 {
                    let slot = LOBBY + 0x310 + kind * 0x14;
                    c.bus.write32(slot, hash);
                    c.bus.write32(LOBBY + 0x314 + kind * 0x14, slot_list);
                }
                c.g[A1][0] = sx(kind);
                c.g[A2][0] = sx(if kind == 3 { name } else { entr });
                c.pc = 0x003e_8778;
            },
        );
    }

    /// `lobby_api_remove_list_entry`: kind (seed % 10: 8,9 bad; 0-2 range or
    /// empty; 3-7 hashed), dup-hit (seed / 10 % 2). | bad kind | empty |
    /// range fail | miss (0) | removed (1) |
    #[test]
    fn lobby_api_remove_list_entry_matches() {
        hook_lobby_fakes();
        check_lohi(
            ee::s14::f_003e8968__generated,
            lobby_api_remove_list_entry,
            64,
            &|c, seed| {
                let mut r = Rng(0x8968 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                c.bus.write32(LOBBY_MEM_CTRL, LOBBY_MEM_MAGIC);
                c.bus.write32(LOBBY_MEM_CTRL + 4, 0x3000);
                c.bus.write32(LOBBY_MEM_CTRL + 8, 0);
                let kind = seed % 10;
                let hit = seed / 10 % 2 == 0;
                let slot_list = DATA + 0x1000;
                let hash = DATA + 0x1400;
                let buckets = DATA + 0x1500;
                let array = DATA + 0x1600;
                let entry = DATA + 0x1700;
                let name = DATA + 0x9000;
                put_str(c, name, b"kay");
                // Removal target: value word is its own address so the
                // list scan hits index 0.
                make_list(c, slot_list, 1, array);
                c.bus.write32(array, entry);
                make_hash(c, hash, buckets, if hit { entry } else { 0 }, 1, 0);
                c.bus.write32(entry, 0);
                c.bus.write32(entry + 4, entry);
                put_str(c, entry + 8, b"kay");
                if kind < 3 {
                    let slot = LOBBY + 0x310 + kind * 0x14;
                    c.bus.write32(slot, if hit { slot_list } else { 0 });
                } else if kind < 8 {
                    let slot = LOBBY + 0x310 + kind * 0x14;
                    c.bus.write32(slot, hash);
                    c.bus.write32(LOBBY + 0x314 + kind * 0x14, slot_list);
                }
                c.g[A1][0] = sx(kind);
                c.g[A2][0] = sx(name);
                c.pc = 0x003e_8968;
            },
        );
    }

    /// `lobby_api_get_record`: tag word (seed % 8: user/act/pers/room/play/
    /// down/zero/bogus) with flag words (seed / 8 % 4) steering the
    /// conditional-null arms.
    #[test]
    fn lobby_api_get_record_matches() {
        check(
            ee::s14::f_003e8a70__generated,
            lobby_api_get_record,
            64,
            &|c, seed| {
                let mut r = Rng(0x8a70 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_base(c, &mut r, seed);
                let tag = match seed % 8 {
                    0 => 0x7573_6572u32,
                    1 => 0x6163_6374u32,
                    2 => 0x7065_7273u32,
                    3 => 0x726f_6f6du32,
                    4 => 0x706c_6179u32,
                    5 => 0x646f_776eu32,
                    6 => 0,
                    _ => r.next_u32(),
                };
                let flags = seed / 8 % 4;
                c.bus.write32(LOBBY + 0x2cc, flags & 1);
                c.bus.write8(LOBBY + 0x3b8, (flags & 2) as u8);
                c.g[A1][0] = sx(tag);
                c.pc = 0x003e_8a70;
            },
        );
    }

    /// `lobby_api_send`: state tag (seed % 5: idle/auth/act/skey/bogus).
    /// AddRequest always fails (alloc mode NULL) so only the no-request
    /// arms run: the DirtyRpc send wall below is untestable (no slots).
    /// | bad state (-1) | idle/auth/act/skey + alloc fail (-2) |
    #[test]
    fn lobby_api_send_matches() {
        hook_lobby_fakes();
        check(
            ee::s14::f_003e8b10__generated,
            lobby_api_send,
            48,
            &|c, seed| {
                let mut r = Rng(0x8b10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                fill(c, &mut r, crate::difftest::SP - 0x400, 0x100);
                c.bus.write32(LOBBY_MEM_CTRL, LOBBY_MEM_MAGIC);
                c.bus.write32(LOBBY_MEM_CTRL + 4, 0x4000);
                c.bus.write32(LOBBY_MEM_CTRL + 8, 1);
                let state = match seed % 5 {
                    0 => 0x6964_6c65u32,
                    1 => 0x6175_7468u32,
                    2 => 0x6163_6374u32,
                    3 => 0x736b_6579u32,
                    _ => 0xdead_beefu32,
                };
                c.bus.write32(LOBBY + 0xc, state);
                c.bus.write32(LOBBY + 0x53c, 0);
                c.bus.write32(LOBBY + 0x54c, seed);
                let tag = match seed % 4 {
                    0 => 0x6175_7468u32,
                    1 => 0x736e_6170u32,
                    _ => r.next_u32(),
                };
                c.g[A1][0] = sx(tag);
                c.g[A2][0] = sx(DATA + 0x5000);
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.pc = 0x003e_8b10;
            },
        );
    }

    /// `lobby_api_cancel_request`: request list (seed % 4: empty / one node /
    /// three nodes / three nodes), tag (seed / 4 % 3: match-all -1 / first
    /// tag / absent). | empty (0) | match (1) | no match (1, list walked) |
    #[test]
    fn lobby_api_cancel_request_matches() {
        check(
            ee::s14::f_003e8d70__generated,
            lobby_api_cancel_request,
            48,
            &|c, seed| {
                let mut r = Rng(0x8d70 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_base(c, &mut r, seed);
                let nodes = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    _ => 3,
                };
                let head = DATA + 0x3000;
                if nodes == 0 {
                    c.bus.write32(LOBBY + 0x53c, 0);
                } else {
                    c.bus.write32(LOBBY + 0x53c, head);
                    for i in 0..nodes {
                        let n = head + i * 0x80;
                        let tag = if i == 0 { 0x1111 } else { 0x2222 + i };
                        c.bus.write32(n, tag);
                        c.bus.write32(n + 0x38, r.next_u32() | 1);
                        c.bus.write32(
                            n + 0x40,
                            if i + 1 < nodes { n + 0x80 } else { 0 },
                        );
                    }
                }
                let want = match seed / 4 % 3 {
                    0 => 0xffff_ffffu32,
                    1 => 0x1111u32,
                    _ => 0x9999u32,
                };
                c.g[A1][0] = sx(want);
                c.pc = 0x003e_8d70;
            },
        );
    }

    /// `lobby_api_status`: tag (seed % 9: stat/rflg/extn/locl/init/more/
    /// slot/zero/bogus), flag words (seed / 9 % 4) steering the init arms.
    #[test]
    fn lobby_api_status_matches() {
        check(
            ee::s14::f_003e8f08__generated,
            lobby_api_status,
            64,
            &|c, seed| {
                let mut r = Rng(0x8f08 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_base(c, &mut r, seed);
                let tag = match seed % 9 {
                    0 => 0x7374_6174u32,
                    1 => 0x7266_6c67u32,
                    2 => 0x6578_746eu32,
                    3 => 0x6c6f_636cu32,
                    4 => 0x696e_6974u32,
                    5 => 0x6d6f_7265u32,
                    6 => 0x736c_6f74u32,
                    7 => 0,
                    _ => r.next_u32(),
                };
                let flags = seed / 9 % 4;
                c.bus.write32(LOBBY + 0x4fc, flags & 1);
                c.bus.write32(LOBBY + 0x500, (flags & 2) >> 1);
                c.g[A1][0] = sx(tag);
                c.pc = 0x003e_8f08;
            },
        );
    }

    /// `lobby_api_get_event`: event word (seed bits) — returns the lowest
    /// set bit, leaves the rest.
    #[test]
    fn lobby_api_get_event_matches() {
        check(
            ee::s14::f_003e8fb8__generated,
            lobby_api_get_event,
            32,
            &|c, seed| {
                let mut r = Rng(0x8fb8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_base(c, &mut r, seed);
                let word = match seed % 5 {
                    0 => 0,
                    1 => 1u32 << (seed % 31),
                    2 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(LOBBY + 0x8, word);
                c.pc = 0x003e_8fb8;
            },
        );
    }

    /// `lobby_api_debug_packet`: type bytes (seed % 5: mismatch / match /
    /// null-type / conn-disc / conn-other), payload (seed / 5 % 6: empty /
    /// printable / short-ctl / long-ctl / long-clean / tab-newline), keyed
    /// buffer (seed / 30 % 2), crypt flag (seed / 60 % 2) and printf
    /// callback (seed / 120 % 2: direct FAKE target via the miss hook).

    #[test]
    fn lobby_api_debug_packet_matches() {
        hook_lobby_fakes();
        check(
            ee::s14::f_003e8fd0__generated,
            lobby_api_debug_packet,
            192,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x8fd0, 0);
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                if seed / 120 % 2 == 0 {
                    c.bus.write32(LOBBY + 0x548, 0);
                } else {
                    c.bus.write32(LOBBY + 0x548, 1);
                    c.bus.write32(LOBBY + 0x544, crate::difftest::fakes::FAKE);
                }
                // Crypt flag + key at +0x24; merge source at +0x34.
                c.bus.write8(LOBBY + 0x24, (seed / 60 % 2) as u8);
                for i in 0..0x10u32 {
                    c.bus.write8(LOBBY + 0x25 + i, (0x41 + i) as u8);
                }
                put_str(c, LOBBY + 0x34, b"conn=abc");
                let (t0, t2) = match seed % 5 {
                    0 => (0x1111_1111u32, 0x2222_2222u32),
                    1 => (0xffff_ffffu32, 0x2222_2222u32),
                    2 => (0xffff_ffffu32, 0xffff_ffffu32),
                    3 => (0xffff_ffffu32, 0xfefe_fefeu32),
                    _ => (0xffff_ffffu32, 0),
                };
                let payload = DATA + 0x4000;
                for w in 0..0x80u32 {
                    c.bus.write32(payload + 4 * w, 0x4141_4141);
                }
                match seed / 5 % 6 {
                    0 => {}
                    1 => {
                        put_str(c, payload, b"hello world");
                    }
                    2 => {
                        put_str(c, payload, b"hi\x01there");
                    }
                    3 => {
                        for i in 0..250u32 {
                            c.bus.write8(payload + i, b'x');
                        }
                        c.bus.write8(payload + 250, 0);
                        c.bus.write8(payload + 10, 0x7f);
                    }
                    4 => {
                        for i in 0..250u32 {
                            c.bus.write8(payload + i, b'y');
                        }
                        c.bus.write8(payload + 250, 0);
                    }
                    _ => {
                        put_str(c, payload, b"a\tb\x0d");
                    }
                }
                // Strnzcpy source: keyed (Find hits) or bare (Find misses).
                let src = DATA + 0x5000;
                for w in 0..0x80u32 {
                    c.bus.write32(src + 4 * w, 0);
                }
                if seed / 30 % 2 == 0 {
                    put_str(c, src, b"conn=xyz");
                } else {
                    put_str(c, src, b"zzz=1");
                }
                let packet = DATA + 0x6000;
                c.bus.write32(packet + 4, t0);
                c.bus.write32(packet + 8, t2);
                c.bus.write32(packet + 0xc, payload);
                c.g[A1][0] = sx(packet);
                c.g[A2][0] = sx(src);
                c.pc = 0x003e_8fd0;
            },
        );
    }

    /// `lobby_api_trim_rank_list`: list (seed % 3: null / null / live),
    /// hash (seed % 4 == 0: null), mode (seed / 12 % 3: keep-all cap /
    /// one clean deletion / rank-0 defaults). | null (-1) | kept | trimmed |
    #[test]
    fn lobby_api_trim_rank_list_matches() {
        hook_lobby_fakes();
        check_lohi(
            ee::s14::f_003e9160__generated,
            lobby_api_trim_rank_list,
            48,
            &|c, seed| {
                let mut r = Rng(0x9160 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                c.bus.write32(LOBBY_MEM_CTRL, LOBBY_MEM_MAGIC);
                c.bus.write32(LOBBY_MEM_CTRL + 4, 0);
                c.bus.write32(LOBBY_MEM_CTRL + 8, 0);
                for w in 0..0x200u32 {
                    c.bus.write32(LOBBY_MEM_POOL + 4 * w, 0);
                }
                let live = seed % 3 == 2;
                let hash_on = seed % 4 != 0;
                if live && hash_on {
                    let list = DATA + 0x1000;
                    let hash = DATA + 0x2000;
                    let buckets = DATA + 0x2100;
                    let array = DATA + 0x2200;
                    let n = 2 + seed % 4;
                    let mode = seed / 12 % 3;
                    make_list(c, list, n, array);
                    // Bucket holds the first entry; every entry is
                    // self-valued with the same key. Mode 0 keeps all
                    // (cap over total); mode 1 deletes the rank-1 head
                    // cleanly; mode 2 exercises the rank-0 default path.
                    for i in 0..n {
                        let e = DATA + 0x3000 + i * 0x40;
                        let rank = match mode {
                            0 => 1 + (i % 5),
                            1 => {
                                if i == 0 {
                                    1
                                } else {
                                    5
                                }
                            }
                            _ => 0,
                        };
                        c.bus.write32(e, rank);
                        c.bus.write32(e + 4, e);
                        put_str(c, e + 8, b"kay");
                        c.bus.write32(array + 4 * i, e);
                    }
                    make_hash(c, hash, buckets, DATA + 0x3000, n, 0);
                    c.bus.write32(LOBBY + 0x338, hash);
                    c.bus.write32(LOBBY + 0x33c, list);
                    c.g[A1][0] = sx(match mode {
                        0 => n + 1,
                        1 => 1,
                        _ => 0,
                    });
                } else {
                    c.bus.write32(LOBBY + 0x338, if live { DATA + 0x2000 } else { 0 });
                    c.bus.write32(LOBBY + 0x33c, if live && !hash_on { 0 } else { DATA + 0x1000 });
                    c.g[A1][0] = sx(seed % 9);
                }
                c.pc = 0x003e_9160;
            },
        );
    }

    /// `lobby_api_ping_server`: state (seed % 5), link (seed % 2), count
    /// (seed % 70). Alloc mode NULL always: success would run the DirtyRpc
    /// wall (no slots). | bad state | no link | full | alloc fail |
    #[test]
    fn lobby_api_ping_server_matches() {
        hook_lobby_fakes();
        check(
            ee::s14::f_003e9590__generated,
            lobby_api_ping_server,
            48,
            &|c, seed| {
                let mut r = Rng(0x9590 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                c.bus.write32(LOBBY_MEM_CTRL, LOBBY_MEM_MAGIC);
                c.bus.write32(LOBBY_MEM_CTRL + 4, 0x6000);
                c.bus.write32(LOBBY_MEM_CTRL + 8, 1);
                let state = match seed % 5 {
                    0 => 0x636f_6e6eu32,
                    1 => 0x6964_6c65u32,
                    2 => 0x6175_7468u32,
                    3 => 0x6163_6374u32,
                    _ => 0xdead_beefu32,
                };
                c.bus.write32(LOBBY + 0xc, state);
                c.bus.write32(LOBBY + 0x4, if seed % 2 == 0 { 0 } else { DATA + 0x6000 });
                c.bus.write32(LOBBY + 0x540, seed % 70);
                c.bus.write32(LOBBY + 0x3b4, DATA + 0x6100);
                c.bus.write32(LOBBY + 0x53c, 0);
                c.bus.write32(LOBBY + 0x54c, seed);
                put_str(c, DATA + 0x6100, b"srv");
                c.pc = 0x003e_9590;
            },
        );
    }

    /// `tag_field_parse_decimal`: digit strings (seed % 8) with output
    /// pre-filled non-zero. | empty/non-digit | short | long | stopped |
    #[test]
    fn tag_field_parse_decimal_matches() {
        check_lohi(
            ee::s14::f_003eae68__generated,
            tag_field_parse_decimal,
            32,
            &|c, seed| {
                let mut r = Rng(0xae68 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let text = DATA + 0x5000;
                for w in 0..0x10u32 {
                    c.bus.write32(text + 4 * w, 0x5858_5858);
                }
                match seed % 8 {
                    0 => put_str(c, text, b""),
                    1 => put_str(c, text, b"abc"),
                    2 => put_str(c, text, b"0"),
                    3 => put_str(c, text, b"7"),
                    4 => put_str(c, text, b"42"),
                    5 => put_str(c, text, b"123456789"),
                    6 => put_str(c, text, b"12a34"),
                    _ => put_str(c, text, b"007"),
                }
                let out = DATA + 0x5100;
                c.bus.write32(out, 0xdead_beef);
                c.g[A0][0] = sx(text);
                c.g[A1][0] = sx(out);
                c.pc = 0x003e_ae68;
            },
        );
    }

    driven!(secstotime_gen, ee::s14::f_003eaeb8__generated);
    driven!(secstotime_hand, ds_secstotime);

    /// `ds_secstotime`: seconds (seed % 10: 0/59/3600/day/month/year/leap/
    /// decade/negative/max) with the month table stubbed, event delay
    /// (seed / 10 % 4) landing the first unwind on each loop head in turn.
    #[test]
    fn ds_secstotime_matches() {
        check_events(secstotime_gen, secstotime_hand, 48, &|c, seed| {
            let mut r = Rng(0xaeb8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            lobby_tables(c);
            let secs = match seed % 10 {
                0 => 0,
                1 => 59,
                2 => 3600,
                3 => 86400,
                4 => 30 * 86400,
                5 => 365 * 86400,
                6 => 366 * 86400,
                7 => 10 * 365 * 86400 + 12345,
                8 => 0xffff_ffffu32,
                _ => 0x7fff_ffffu32,
            };
            let out = DATA + 0x5000;
            for w in 0..9u32 {
                c.bus.write32(out + 4 * w, 0x5858_5858);
            }
            c.g[A0][0] = sx(out);
            c.g[A1][0] = sx(secs);
            let mode = schedule(c, &mut r, seed, 0x003e_aeb8);
            c.g[A0][0] = sx(out);
            c.g[A1][0] = sx(secs);
            // Steer the first event onto each head across seeds. Keep every
            // value inside even the shortest run (the 600/6000 values used
            // to overshoot short dates and starve the delivery count).
            c.next_event = c.cycles + [4u64, 30, 60, 120][(seed / 10 % 4) as usize];
            c.pc = 0x003e_aeb8;
            mode
        });
    }

    driven!(pick_room_gen, ee::s14::f_003e9378__generated);
    driven!(pick_room_hand, lobby_api_pick_room);

    /// `lobby_api_pick_room`: room list (seed % 4 == 0: null), name
    /// (seed % 3 == 0: default), flags (seed / 12 % 4), want (seed % 150).
    /// | no list | no match | best under want | fallback over want |
    #[test]
    fn lobby_api_pick_room_matches() {
        hook_lobby_fakes();
        check_events(pick_room_gen, pick_room_hand, 48, &|c, seed| {
            let mut r = Rng(0x9378 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            lobby_tables(c);
            lobby_base(c, &mut r, seed);
            let with_list = seed % 4 != 0;
            if with_list {
                let list = DATA + 0x1000;
                let array = DATA + 0x1200;
                let n = 1 + seed % 4;
                make_list(c, list, n, array);
                for i in 0..n {
                    let room = DATA + 0x2000 + i * 0x40;
                    c.bus.write32(room + 8, (seed / 12 + i) % 4);
                    c.bus.write32(room + 0xc, if seed % 7 == 0 { 0 } else { 10 });
                    c.bus.write32(room + 0x10, (seed + i * 3) % 13);
                    if seed % 5 == 0 {
                        put_str(c, room + 0x1c, b"lobby");
                    } else {
                        put_str(c, room + 0x1c, b"room");
                    }
                    c.bus.write32(array + 4 * i, room);
                }
                c.bus.write32(LOBBY + 0x310, list);
                c.bus.write32(LOBBY + 0x314, list);
            }
            let mode = schedule(c, &mut r, seed, 0x003e_9378);
            c.g[A0][0] = sx(LOBBY);
            c.g[A1][0] = sx(seed % 150);
            c.g[A2][0] = sx(0x3);
            c.g[A3][0] = sx(seed % 4);
            c.g[T0][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x4000) };
            c.g[T1][0] = sx(0x77);
            put_str(c, DATA + 0x4000, b"room");
            c.pc = 0x003e_9378;
            mode
        });
    }

    /// `lobby_api_server_ping_callback`: tag (seed % 3: ping/other),
    /// sequence (seed % 2: live/stale), counter (seed % 6).
    /// | non-ping | live (tick) | stale, count<3 | stale, count==3 (reset) |
    #[test]
    fn lobby_api_server_ping_callback_matches() {
        hook_lobby_fakes();
        check(
            ee::s14::f_003e94f0__generated,
            lobby_api_server_ping_callback,
            48,
            &|c, seed| {
                let mut r = Rng(0x94f0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                c.bus.write32(LOBBY_TICK_CTRL, LOBBY_TICK_MAGIC);
                c.bus.write32(LOBBY_TICK_CTRL + 4, 0x1234);
                let buf = DATA + 0x5000;
                for w in 0..0x40u32 {
                    c.bus.write32(buf + 4 * w, 0);
                }
                if seed % 2 == 0 {
                    put_str(c, buf, b"seq=5");
                } else {
                    put_str(c, buf, b"seq=0");
                }
                let packet = DATA + 0x6000;
                c.bus.write32(
                    packet + 4,
                    if seed % 3 == 0 {
                        0x1234_5678u32
                    } else {
                        0x7069_6e67u32
                    },
                );
                c.bus.write32(packet + 0xc, buf);
                c.bus.write32(LOBBY + 0x20, seed % 6);
                c.g[A1][0] = sx(packet);
                c.pc = 0x003e_94f0;
            },
        );
    }

    /// `lobby_api_ping`: state (seed % 5: conn/idle/auth/act/bogus), link
    /// (seed % 3 == 0: null), count (seed % 70: full at >= 64). Alloc mode
    /// NULL always: success would run the DirtyRpc wall (no slots).
    /// | bad state (-1) | no link (-2) | full (-3) | alloc fail (-4) |
    #[test]
    fn lobby_api_ping_matches() {
        hook_lobby_fakes();
        check(
            ee::s14::f_003e8dc0__generated,
            lobby_api_ping,
            64,
            &|c, seed| {
                let mut r = Rng(0x8dc0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                c.bus.write32(LOBBY_TICK_CTRL, LOBBY_TICK_MAGIC);
                c.bus.write32(LOBBY_TICK_CTRL + 4, 0x1234);
                c.bus.write32(LOBBY_MEM_CTRL, LOBBY_MEM_MAGIC);
                c.bus.write32(LOBBY_MEM_CTRL + 4, 0x5000);
                c.bus.write32(LOBBY_MEM_CTRL + 8, 1);
                let state = match seed % 5 {
                    0 => 0x636f_6e6eu32,
                    1 => 0x6964_6c65u32,
                    2 => 0x6175_7468u32,
                    3 => 0x6163_6374u32,
                    _ => 0xdead_beefu32,
                };
                c.bus.write32(LOBBY + 0xc, state);
                c.bus.write32(LOBBY + 0x4, if seed % 3 == 0 { 0 } else { DATA + 0x6000 });
                c.bus.write32(
                    LOBBY + 0x540,
                    match seed % 4 {
                        0 => seed % 64,
                        1 => 63,
                        2 => 64,
                        _ => 100,
                    },
                );
                c.bus.write32(LOBBY + 0x53c, 0);
                c.bus.write32(LOBBY + 0x54c, seed);
                c.g[A1][0] = sx(DATA + 0x7000);
                c.g[A2][0] = sx(DATA + 0x7100);
                c.g[A3][0] = (seed % 251) as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                put_str(c, DATA + 0x7000, b"addr");
                c.pc = 0x003e_8dc0;
            },
        );
    }

    /// `lobby_api_has_list_entry`: kind (seed % 10), dup-hit (seed % 2).
    /// | bad kind (-1) | empty (-1) | miss (0) | hit (1) |
    #[test]
    fn lobby_api_has_list_entry_matches() {
        check(
            ee::s14::f_003e8a28__generated,
            lobby_api_has_list_entry,
            48,
            &|c, seed| {
                let mut r = Rng(0x8a28 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                lobby_tables(c);
                lobby_base(c, &mut r, seed);
                let kind = seed % 10;
                let hit = seed % 2 == 0;
                let hash = DATA + 0x1400;
                let buckets = DATA + 0x1500;
                let entry = DATA + 0x1700;
                let name = DATA + 0x9000;
                put_str(c, name, b"kay");
                make_hash(c, hash, buckets, if hit { entry } else { 0 }, 1, 0);
                c.bus.write32(entry, 0);
                put_str(c, entry + 8, b"kay");
                if kind < 8 {
                    let slot = LOBBY + 0x310 + kind * 0x14;
                    c.bus.write32(slot, if hit || seed % 3 == 0 { hash } else { 0 });
                }
                c.g[A1][0] = sx(kind);
                c.g[A2][0] = sx(name);
                c.pc = 0x003e_8a28;
            },
        );
    }
}
