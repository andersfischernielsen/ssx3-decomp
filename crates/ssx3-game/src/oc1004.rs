//! Staging module oc1004 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1004`.

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
    (0x0025_d860, lobby_man_move_to_room, "LobbyMan_MoveToRoom"),
    (
        0x0025_d6c0,
        lobby_man_on_user_list_changed,
        "LobbyMan_OnUserListChanged",
    ),
    (
        0x0025_d538,
        lobby_man_on_delete_persona_reply,
        "LobbyMan_OnDeletePersonaReply",
    ),
    (
        0x0025_d428,
        lobby_man_delete_persona,
        "LobbyMan_DeletePersona",
    ),
    (
        0x0025_d2d8,
        lobby_man_on_create_persona_reply,
        "LobbyMan_OnCreatePersonaReply",
    ),
    (
        0x0025_d1b8,
        lobby_man_create_persona,
        "LobbyMan_CreatePersona",
    ),
    (
        0x0025_d048,
        lobby_man_on_own_stats_reply,
        "LobbyMan_OnOwnStatsReply",
    ),
    (
        0x0025_ceb0,
        lobby_man_on_select_persona_reply,
        "LobbyMan_OnSelectPersonaReply",
    ),
    (
        0x0025_cd50,
        lobby_man_select_persona,
        "LobbyMan_SelectPersona",
    ),
    (
        0x0025_c8a8,
        lobby_man_on_login_reply,
        "LobbyMan_OnLoginReply",
    ),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub const TAG_FIELD_SET_STRING: Callee = Callee {
        addr: 0x003e_c180,
        f: ee::s14::f_003ec180,
    };
    pub const TAG_FIELD_FIND: Callee = Callee {
        addr: 0x003e_b6d8,
        f: ee::s14::f_003eb6d8,
    };
    pub const TAG_FIELD_GET_STRING: Callee = Callee {
        addr: 0x003e_d378,
        f: ee::s14::f_003ed378,
    };
    pub const TAG_FIELD_GET_BINARY: Callee = Callee {
        addr: 0x003e_d678,
        f: ee::s14::f_003ed678,
    };
    pub const TAG_FIELD_GET_NUMBER: Callee = Callee {
        addr: 0x003e_cf30,
        f: ee::s14::f_003ecf30,
    };
    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };
    pub const CBXSTRING_ASSIGN: Callee = Callee {
        addr: 0x0031_8448,
        f: ee::s10::f_00318448,
    };
    pub const C_BX_STRING_ASSIGN_CSTR: Callee = Callee {
        addr: 0x0031_84e8,
        f: ee::s10::f_003184e8,
    };
    pub const C_BX_STRING_CTOR_CSTR: Callee = Callee {
        addr: 0x0031_82d8,
        f: ee::s10::f_003182d8,
    };
    pub const LOBBY_MAN_SEND_REQUEST: Callee = Callee {
        addr: 0x0025_b608,
        f: ee::s06::f_0025b608,
    };
    pub const LOBBY_MAN_PARSE_NAME_SUGGESTIONS: Callee = Callee {
        addr: 0x0025_bc48,
        f: ee::s06::f_0025bc48,
    };
    pub const LOBBY_MAN_QUERY_OWN_STATS: Callee = Callee {
        addr: 0x0025_cfe0,
        f: ee::s06::f_0025cfe0,
    };
    pub const LOBBY_MAN_REQUEST_NEWS: Callee = Callee {
        addr: 0x0025_e0c8,
        f: ee::s06::f_0025e0c8,
    };
    pub const NET_CONN_MAC: Callee = Callee {
        addr: 0x003f_3f78,
        f: ee::s14::f_003f3f78,
    };
    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::f_00416810,
    };
    pub const LOBBY_HASH_MEMORY_RANGES: Callee = Callee {
        addr: 0x003e_fd30,
        f: ee::s14::f_003efd30,
    };
}

/// `void LobbyMan_MoveToRoom(LobbyMan* self, const char* room, const char* name)`
/// (0x25d860): build the "move" dirtysock tag list (room name, plus the player
/// name when given) and send it as request 0xc ("move").
pub fn lobby_man_move_to_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        let no_net = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A2][0];
        if !no_net {
            c.cycles += 9;
            c.g[A2][0] = 0x4a0000;
            c.g[A3][0] = c.g[A1][0];
            c.g[A2][0] = sx(0x004a_2f58);
            Addr(sp(c)).sb(c, 0);
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_d89c)?;
            let has_name = c.g[S0][0] != 0;
            c.g[A2][0] = 0x4a0000;
            if has_name {
                c.cycles += 5;
                c.g[A3][0] = c.g[S0][0];
                c.g[A2][0] = sx(0x004a_3040);
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x100;
                callee::TAG_FIELD_SET_STRING.call(c, 0x0025_d8b8)?;
            }
            c.cycles += 10;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x6d6f0000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = sx(0x0026_8d38);
            c.g[A1][0] = 0xc;
            c.g[A2][0] = c.g[A2][0] | 0x7665;
            c.g[A3][0] = c.g[SP][0];
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_d8e0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnUserListChanged(LobbyMan* self)` (0x25d6c0): post the
/// 0xdf "user list changed" notice onto the lobby message list at self+0xf0.
pub fn lobby_man_on_user_list_changed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 0x8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d6ec)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0xdf;
        c.g[V0][0] = sx(0x0048_12b0);
        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
        Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x480000;
        Addr(frame.at(c, 0x10)).sw(c, c.g[A0][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d724)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x40);
        Addr(r32(c, A1)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        frame.restore_ra(c, 0x20);
        Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnDeletePersonaReply(LobbyMan* self, sMessage* msg)`
/// (0x25d538): on success (msg+8 == 0) drop the deleted persona, shifting the
/// remaining names down, and post 0xd8; otherwise post the 0x102 failure.
pub fn lobby_man_on_delete_persona_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        c.cycles += 14;
        c.g[V0][0] = 0x1;
        frame.save(c, S3, 0x40);
        frame.save(c, S1, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0x20);
        Addr(r32(c, S3)).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
        let failed = c.g[V0][0] != 0;
        c.g[A1][0] = 0x480000;
        if !failed {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x6c)).lw(c);
            c.g[S4][0] = sx(r32(c, S3).wrapping_add(0xec));
            c.g[S2][0] = Addr(r32(c, S3).wrapping_add(0x50)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let shift = c.g[V1][0] != 0;
            Addr(r32(c, S3).wrapping_add(0x6c)).sw(c, c.g[V0][0]);
            if shift {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, S2) << 2);
                c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x70));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x74));
                c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
                c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S1][0];
                    callee::CBXSTRING_ASSIGN.call(c, 0x0025_d5ac)?;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x6c)).lw(c);
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_d5a0);
                            // Reload loop state from registers after delivery.
                            c.g[S2][0] = c.g[S2][0];
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = 0x480000;
                    break;
                }
            }
            c.cycles += 19;
            c.g[A2][0] = 0x20000000;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A0][0] = 0x8;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d5dc)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = Addr(r32(c, S4).wrapping_add(4)).lw(c);
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = 0xd8;
            c.g[V0][0] = sx(0x0048_12b0);
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
        } else {
            c.cycles += 20;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x488));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d628)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            c.g[V0][0] = 0x480000;
            c.g[A2][0] = Addr(r32(c, S3).wrapping_add(0xf0)).lw(c);
            c.g[V0][0] = sx(0x0048_1290);
            c.g[V1][0] = 0x102;
            Addr(r32(c, S0).wrapping_add(8)).sw(c, c.g[A0][0]);
            c.g[A1][0] = 0x480000;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
            c.g[A3][0] = 0;
            c.g[A2][0] = 0x20000000;
        }
        // Post: allocate the list node and link the message in.
        c.cycles += 6;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d66c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S0][0]);
        }
        c.cycles += 17;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, A1)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_DeletePersona(LobbyMan* self, int idx)` (0x25d428): send
/// request 0xb ("dper") naming persona `idx`, remember `idx` at self+0x50,
/// and post the 0xd7 notice.
pub fn lobby_man_delete_persona(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x260);
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        frame.save(c, S0, 0x250);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x220);
        c.g[V0][0] = Addr(r32(c, S2)).lw(c);
        let no_net = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_net {
            c.cycles += 44;
            Addr(sp(c)).sb(c, 0);
            c.g[V0][0] = sx(r32(c, S1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S2)));
            c.g[A2][0] = 0x4a0000;
            c.g[A3][0] = Addr(r32(c, V0).wrapping_add(0x70)).lw(c);
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx(0x004a_3108);
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_d470)?;
            c.g[S0][0] = 0xd7;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x64700000;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx(0x0026_8d10);
            c.g[A1][0] = 0xb;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[A2][0] | 0x6572;
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_d49c)?;
            Addr(r32(c, S2).wrapping_add(0x50)).sw(c, c.g[S1][0]);
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A0][0] = 0x8;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d4b8)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
            c.g[V0][0] = 0x480000;
            Addr(r32(c, S1)).sw(c, c.g[S0][0]);
            c.g[V0][0] = sx(0x0048_12b0);
            c.g[A1][0] = 0x480000;
            Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            Addr(frame.at(c, 0x210)).sw(c, c.g[V1][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d4ec)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(frame.at(c, 0x210)).lw(c);
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[A1][0]);
            c.g[V0][0] = Addr(frame.at(c, 0x210)).lw(c);
            Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0x200)).sw(c, c.g[A1][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x250);
        frame.restore(c, S1, 0x240);
        frame.restore(c, S2, 0x230);
        frame.restore_ra(c, 0x220);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnCreatePersonaReply(LobbyMan* self, sMessage* msg)`
/// (0x25d2d8): on success bump the persona count at self+0x6c and post 0xd6;
/// on a "dupl" failure first parse the name suggestions, then post 0x102.
pub fn lobby_man_on_create_persona_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        c.cycles += 12;
        c.g[V0][0] = 0x1;
        frame.save(c, S2, 0x30);
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
        let failed = c.g[V1][0] != 0;
        c.g[V0][0] = 0x64750000;
        if !failed {
            c.cycles += 23;
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6c)).lw(c);
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A2][0] = 0x20000000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[A0][0] = 0x8;
            c.g[A3][0] = 0;
            Addr(r32(c, S2).wrapping_add(0x6c)).sw(c, c.g[V0][0]);
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d32c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = 0xd6;
            c.g[V0][0] = sx(0x0048_12b0);
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] | 0x706c;
            let is_dupl = c.g[V1][0] == c.g[V0][0];
            c.g[A1][0] = 0x480000;
            if is_dupl {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                callee::LOBBY_MAN_PARSE_NAME_SUGGESTIONS.call(c, 0x0025_d37c)?;
                c.g[A1][0] = 0x480000;
            }
            c.cycles += 20;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x488));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d394)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
            c.g[V0][0] = 0x480000;
            c.g[A2][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
            c.g[V0][0] = sx(0x0048_1290);
            c.g[V1][0] = 0x102;
            Addr(r32(c, S0).wrapping_add(8)).sw(c, c.g[A0][0]);
            c.g[A1][0] = 0x480000;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
            c.g[A3][0] = 0;
            c.g[A2][0] = 0x20000000;
        }
        c.cycles += 6;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d3d8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S0][0]);
        }
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, A1)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_CreatePersona(LobbyMan* self, const char* name)` (0x25d1b8):
/// send request 0xa ("cper") with `name`, append `name` to the persona list,
/// and post the 0xd5 notice.
pub fn lobby_man_create_persona(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x260);
        frame.save(c, S1, 0x240);
        frame.save(c, S2, 0x230);
        frame.save(c, S0, 0x250);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x220);
        c.g[V0][0] = Addr(r32(c, S2)).lw(c);
        let no_net = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_net {
            c.cycles += 47;
            c.g[A2][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            Addr(sp(c)).sb(c, 0);
            c.g[A3][0] = c.g[S1][0];
            c.g[A2][0] = sx(0x004a_3108);
            c.g[A1][0] = 0x200;
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_d1f8)?;
            c.g[S0][0] = 0xd5;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x63700000;
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = sx(0x0026_8ce8);
            c.g[A2][0] = c.g[A2][0] | 0x6572;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0xa;
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_d224)?;
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x6c)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = sx(r32(c, A0) << 2);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x70));
            c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
            callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x0025_d23c)?;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d254)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
            c.g[V0][0] = 0x480000;
            Addr(r32(c, S1)).sw(c, c.g[S0][0]);
            c.g[V0][0] = sx(0x0048_12b0);
            c.g[A1][0] = 0x480000;
            Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            Addr(frame.at(c, 0x210)).sw(c, c.g[V1][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d288)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(frame.at(c, 0x210)).lw(c);
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[A1][0]);
            c.g[V0][0] = Addr(frame.at(c, 0x210)).lw(c);
            Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0x200)).sw(c, c.g[A1][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x250);
        frame.restore(c, S1, 0x240);
        frame.restore(c, S2, 0x230);
        frame.restore_ra(c, 0x220);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnOwnStatsReply(LobbyMan* self, sMessage* msg)` (0x25d048):
/// on success unpack the stats structure into self+0xb8..0xc8, clear the
/// pending flag at self+0xc8, and request the news; otherwise post 0x102.
pub fn lobby_man_on_own_stats_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x180);
        c.cycles += 12;
        c.g[V0][0] = 0x1;
        frame.save(c, S0, 0x170);
        frame.save(c, S2, 0x150);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x160);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x140);
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
        let failed = c.g[V0][0] != 0;
        c.g[A1][0] = 0x480000;
        if !failed {
            c.cycles += 40;
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx(0x004a_3128);
            callee::TAG_FIELD_FIND.call(c, 0x0025_d088)?;
            c.g[A3][0] = 0x4a0000;
            c.g[A1][0] = c.g[SP][0];
            c.g[A3][0] = sx(0x004a_2f40);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = 0x100;
            callee::TAG_FIELD_GET_STRING.call(c, 0x0025_d0a0)?;
            c.g[A3][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx(sp(c).wrapping_add(0x100));
            c.g[A2][0] = 0x40;
            c.g[A3][0] = sx(0x004a_2f30);
            callee::TAG_FIELD_GET_BINARY.call(c, 0x0025_d0b8)?;
            c.g[A2][0] = Addr(sp(c).wrapping_add(0x108)).lw(c);
            c.g[A1][0] = 0x4a0000;
            c.g[V0][0] = Addr(sp(c).wrapping_add(0x10c)).lw(c);
            c.g[A1][0] = sx(0x004a_3130);
            c.g[V1][0] = Addr(sp(c).wrapping_add(0x110)).lw(c);
            c.g[A0][0] = Addr(sp(c).wrapping_add(0x118)).lw(c);
            c.g[V0][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            Addr(r32(c, S0).wrapping_add(0xbc)).sw(c, c.g[A2][0]);
            Addr(r32(c, S0).wrapping_add(0xc0)).sw(c, c.g[V0][0]);
            Addr(r32(c, S0).wrapping_add(0xc4)).sw(c, c.g[A0][0]);
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
            callee::TAG_FIELD_FIND.call(c, 0x0025_d0ec)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x0025_d0f8)?;
            c.g[V1][0] = u64::MAX;
            Addr(r32(c, S0).wrapping_add(0xb8)).sw(c, c.g[V0][0]);
            Addr(r32(c, S0).wrapping_add(0xc8)).sw(c, c.g[V1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::LOBBY_MAN_REQUEST_NEWS.call(c, 0x0025_d110)?;
            frame.restore(c, S0, 0x170);
        } else {
            c.cycles += 25;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x488));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d12c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
            c.g[V0][0] = 0x480000;
            c.g[A2][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
            c.g[V0][0] = sx(0x0048_1290);
            c.g[V1][0] = 0x102;
            Addr(r32(c, S1).wrapping_add(8)).sw(c, c.g[A0][0]);
            c.g[A1][0] = 0x480000;
            Addr(r32(c, S1)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
            c.g[A3][0] = 0;
            c.g[A2][0] = 0x20000000;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_d16c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
            }
            c.cycles += 10;
            c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[A1][0]);
            c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
            Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
            Addr(sp(c)).sw(c, c.g[A1][0]);
            frame.restore(c, S0, 0x170);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x160);
        frame.restore(c, S2, 0x150);
        frame.restore_ra(c, 0x140);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnSelectPersonaReply(LobbyMan* self, sMessage* msg)`
/// (0x25ceb0): on success re-query the player's own stats and post 0xda;
/// otherwise post the 0x102 failure.
pub fn lobby_man_on_select_persona_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        c.cycles += 12;
        c.g[V0][0] = 0x1;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
        let failed = c.g[V0][0] != 0;
        c.g[A1][0] = 0x480000;
        if !failed {
            c.cycles += 22;
            callee::LOBBY_MAN_QUERY_OWN_STATS.call(c, 0x0025_cee8)?;
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = 0x20000000;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A0][0] = 0x8;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_cf00)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = Addr(r32(c, S1).wrapping_add(0xf0)).lw(c);
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = 0xda;
            c.g[V0][0] = sx(0x0048_12b0);
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
        } else {
            c.cycles += 20;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x488));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_cf4c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
            c.g[V0][0] = 0x480000;
            c.g[A2][0] = Addr(r32(c, S1).wrapping_add(0xf0)).lw(c);
            c.g[V0][0] = sx(0x0048_1290);
            c.g[V1][0] = 0x102;
            Addr(r32(c, S0).wrapping_add(8)).sw(c, c.g[A0][0]);
            c.g[A1][0] = 0x480000;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
            c.g[A3][0] = 0;
            c.g[A2][0] = 0x20000000;
        }
        c.cycles += 6;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_cf90)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S0][0]);
        }
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, A1)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_SelectPersona(LobbyMan* self, const char* name)` (0x25cd50,
/// asm-only in the reference): copy the account name at self+0x80, tag the
/// persona name plus the saved name/flag fields, send request 9 ("pers"),
/// and post the 0xd9 notice.
pub fn lobby_man_select_persona(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x260);
        frame.save(c, S0, 0x250);
        frame.save(c, S2, 0x230);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x240);
        frame.save_ra(c, 0x220);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(0x80));
        callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x0025_cd74)?;
        c.g[A2][0] = 0x4a0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = c.g[S0][0];
        Addr(sp(c)).sb(c, 0);
        c.g[A2][0] = sx(0x004a_3108);
        c.g[A1][0] = 0x200;
        callee::TAG_FIELD_SET_STRING.call(c, 0x0025_cd90)?;
        c.g[S0][0] = 0x4a0000;
        c.g[S0][0] = sx(0x004a_30c8);
        callee::NET_CONN_MAC.call(c, 0x0025_cd9c)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        callee::TAG_FIELD_SET_STRING.call(c, 0x0025_cdb0)?;
        c.g[A2][0] = 0x4a0000;
        c.g[A3][0] = 0x480000;
        c.g[A2][0] = sx(0x004a_30d0);
        c.g[A3][0] = sx(0x0048_13a0);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        callee::TAG_FIELD_SET_STRING.call(c, 0x0025_cdcc)?;
        c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0x18)).lw(c);
        let has_flag = c.g[V1][0] != 0;
        c.g[A0][0] = c.g[SP][0];
        if has_flag {
            c.cycles += 5;
            c.g[A2][0] = 0x4a0000;
            c.g[A2][0] = sx(0x004a_3110);
            c.g[A1][0] = 0x200;
            c.g[A3][0] = sx(r32(c, S2).wrapping_add(0x1c));
            callee::TAG_FIELD_SET_STRING.call(c, 0x0025_cdec)?;
        }
        c.cycles += 34;
        c.g[AT][0] = 0x41a00000;
        c.mtc1(AT, 12);
        c.g[T0][0] = 0x270000;
        c.g[A2][0] = 0x70650000;
        c.g[A3][0] = c.g[SP][0];
        c.g[T0][0] = sx(0x0026_8c98);
        c.g[A1][0] = 0x9;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[A2][0] | 0x7273;
        callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_ce14)?;
        c.g[S0][0] = 0xd9;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x8;
        c.g[A1][0] = sx(0x0048_0488);
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_ce30)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
        c.g[V0][0] = 0x480000;
        Addr(r32(c, S1)).sw(c, c.g[S0][0]);
        c.g[V0][0] = sx(0x0048_12b0);
        c.g[A1][0] = 0x480000;
        Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        Addr(frame.at(c, 0x210)).sw(c, c.g[V1][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_ce64)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x210)).lw(c);
        frame.restore(c, S0, 0x250);
        Addr(r32(c, A1)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x240);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        frame.restore(c, S2, 0x230);
        Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
        frame.restore_ra(c, 0x220);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x210)).lw(c);
        Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0x200)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnLoginReply(LobbyMan* self, sMessage* msg)` (0x25c8a8,
/// asm-only in the reference, 1152 B): on success split the returned persona
/// list (comma-separated, at most four names) into self+0x70.., decode the
/// account flags into self+0x2e8/0x2ec, copy the persona strings, and post
/// 0xd4; on a "lock" error post the lock message, else the 0x102 failure.
pub fn lobby_man_on_login_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x3c0);
        c.cycles += 18;
        c.g[V0][0] = 0x1;
        frame.save(c, S2, 0x390);
        frame.save(c, S1, 0x3a0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x3b0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x380);
        frame.save(c, S4, 0x370);
        frame.save(c, S5, 0x360);
        frame.save(c, S6, 0x350);
        frame.save(c, S7, 0x340);
        frame.save(c, FP, 0x330);
        frame.save_ra(c, 0x320);
        Addr(r32(c, S2)).sw(c, c.g[V0][0]);
        c.g[S3][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
        let failed = c.g[S3][0] != 0;
        c.g[V0][0] = 0x6c6f0000;
        if !failed {
            // --- success: persona list -------------------------------------
            c.cycles += 27;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx(0x0048_0880);
            c.g[S0][0] = 0x4a0000;
            callee::TAG_FIELD_FIND.call(c, 0x0025_c904)?;
            c.g[S6][0] = sx(r32(c, S2).wrapping_add(0x2bc));
            c.g[A3][0] = sx(0x004a_2f40);
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = 0x100;
            callee::TAG_FIELD_GET_STRING.call(c, 0x0025_c91c)?;
            c.g[S7][0] = sx(r32(c, S2).wrapping_add(0x2b0));
            c.g[V0][0] = sx(sp(c).wrapping_add(0x200));
            c.g[V1][0] = sx(r32(c, S2).wrapping_add(0x2b4));
            Addr(frame.at(c, 0x31c)).sw(c, c.g[V0][0]);
            c.g[S4][0] = sx(sp(c).wrapping_add(0x100));
            c.g[V0][0] = Addr(sp(c)).lb(c);
            c.g[S0][0] = c.g[SP][0];
            Addr(r32(c, S2).wrapping_add(0x6c)).sw(c, 0);
            c.g[FP][0] = sx(r32(c, S2).wrapping_add(0x48));
            Addr(frame.at(c, 0x310)).sw(c, c.g[V1][0]);
            c.g[S5][0] = sx(r32(c, S2).wrapping_add(0x4c));
            c.g[V1][0] = sx(r32(c, S2).wrapping_add(0x2b8));
            Addr(frame.at(c, 0x314)).sw(c, c.g[V1][0]);
            c.g[V1][0] = sx(r32(c, S2).wrapping_add(0xec));
            let empty_list = c.g[V0][0] == 0;
            Addr(frame.at(c, 0x318)).sw(c, c.g[V1][0]);
            if !empty_list {
                c.cycles += 3;
                c.g[S3][0] = 0x2c;
                c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                // Outer loop head 0x25c968: one persona name per iteration.
                'persona: loop {
                    // Scan to the next ',' or NUL (inner head 0x25c980).
                    loop {
                        c.cycles += 2;
                        let at_end = c.g[V0][0] == 0;
                        c.g[A1][0] = c.g[S0][0];
                        if at_end {
                            break;
                        }
                        c.cycles += 2;
                        let at_comma = c.g[V0][0] == c.g[S3][0];
                        c.g[V1][0] = 0x2c;
                        if at_comma {
                            break;
                        }
                        c.cycles += 2;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[V1][0] {
                                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                                if event_due(c) {
                                    deliver_event_here(c, 0x0025_c980);
                                }
                                continue;
                            }
                            break;
                        }
                        // Found ',' (or fell out at NUL handled below).
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        c.cycles += 2;
                        Addr(r32(c, S0)).sb(c, 0);
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        break;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6c)).lw(c);
                    // Append this name and advance.
                    c.cycles += 9;
                    c.g[A0][0] = sx(r32(c, V0) << 2);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x70));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    Addr(r32(c, S2).wrapping_add(0x6c)).sw(c, c.g[V0][0]);
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
                    callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x0025_c9c4)?;
                    c.g[V1][0] = Addr(r32(c, S0)).lb(c);
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
                        break 'persona;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6c)).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 4) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x0025_c968);
                        }
                        continue 'persona;
                    }
                    break 'persona;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
            // --- account fields --------------------------------------------
            c.cycles += 45;
            c.g[V0][0] = 0x4a0000;
            c.g[A1][0] = 0x4a0000;
            c.g[S3][0] = sx(0x004a_2f40);
            c.g[A1][0] = sx(0x004a_3050);
            callee::TAG_FIELD_FIND.call(c, 0x0025_c9f8)?;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = c.g[S3][0];
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S4][0];
            callee::TAG_FIELD_GET_STRING.call(c, 0x0025_ca0c)?;
            c.g[A1][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S6][0];
            callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x0025_ca18)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx(0x004a_3048);
            callee::TAG_FIELD_FIND.call(c, 0x0025_ca28)?;
            c.g[A1][0] = Addr(frame.at(c, 0x31c)).lw(c);
            c.g[A2][0] = 0x3;
            c.g[A3][0] = c.g[S3][0];
            c.g[A0][0] = c.g[V0][0];
            callee::TAG_FIELD_GET_STRING.call(c, 0x0025_ca3c)?;
            c.g[V1][0] = Addr(sp(c).wrapping_add(0x200)).lb(c);
            c.g[A0][0] = c.g[S7][0];
            c.g[V0][0] = Addr(sp(c).wrapping_add(0x201)).lb(c);
            c.g[A1][0] = c.g[FP][0];
            c.g[V1][0] = c.g[V1][0] ^ 0x59;
            c.g[V0][0] = c.g[V0][0] ^ 0x59;
            c.g[V1][0] = (c.g[V1][0] < 1) as u64;
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            Addr(r32(c, S2).wrapping_add(0x2e8)).sw(c, c.g[V1][0]);
            Addr(r32(c, S2).wrapping_add(0x2ec)).sw(c, c.g[V0][0]);
            callee::CBXSTRING_ASSIGN.call(c, 0x0025_ca68)?;
            c.g[A0][0] = Addr(frame.at(c, 0x310)).lw(c);
            c.g[A1][0] = c.g[S5][0];
            callee::CBXSTRING_ASSIGN.call(c, 0x0025_ca74)?;
            c.g[A0][0] = Addr(frame.at(c, 0x314)).lw(c);
            c.g[A1][0] = c.g[S5][0];
            callee::CBXSTRING_ASSIGN.call(c, 0x0025_ca80)?;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx(0x004a_30f0);
            callee::TAG_FIELD_FIND.call(c, 0x0025_ca90)?;
            let no_persona = c.g[V0][0] == 0;
            c.g[S0][0] = sx(sp(c).wrapping_add(0x210));
            if no_persona {
                c.cycles += 2;
                Addr(r32(c, S2).wrapping_add(0x18)).sw(c, 0);
                c.g[V0][0] = 0x1;
            } else {
                c.cycles += 15;
                c.g[A3][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x100;
                c.g[A0][0] = c.g[V0][0];
                callee::TAG_FIELD_GET_STRING.call(c, 0x0025_caac)?;
                c.g[A0][0] = c.g[S0][0];
                callee::STRLEN.call(c, 0x0025_cab4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx(r32(c, S2).wrapping_add(0x1c));
                c.g[A3][0] = 0x21;
                callee::LOBBY_HASH_MEMORY_RANGES.call(c, 0x0025_cac8)?;
                c.g[V0][0] = 0x1;
                Addr(r32(c, S2).wrapping_add(0x18)).sw(c, c.g[V0][0]);
            }
            // --- post 0xd4 --------------------------------------------------
            c.cycles += 26;
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A2][0] = 0x20000000;
            c.g[A0][0] = 0x8;
            c.g[A3][0] = 0;
            Addr(r32(c, S2).wrapping_add(8)).sw(c, c.g[V0][0]);
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_caf8)?;
            c.g[V1][0] = Addr(frame.at(c, 0x318)).lw(c);
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = 0x480000;
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = Addr(r32(c, V1).wrapping_add(4)).lw(c);
            c.g[V0][0] = sx(0x0048_12b0);
            c.g[V1][0] = 0xd4;
            Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            Addr(frame.at(c, 0x220)).sw(c, c.g[A2][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_cb34)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S0][0]);
            }
            c.cycles += 10;
            c.g[V0][0] = Addr(frame.at(c, 0x220)).lw(c);
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[A1][0]);
            c.g[V0][0] = Addr(frame.at(c, 0x220)).lw(c);
            Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0x210)).sw(c, c.g[A1][0]);
        } else {
            // --- failure ----------------------------------------------------
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] | 0x636b;
            let is_lock = c.g[S3][0] == c.g[V0][0];
            c.g[A1][0] = 0x480000;
            if is_lock {
                c.cycles += 52;
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx(0x004a_30f8);
                callee::TAG_FIELD_FIND.call(c, 0x0025_cb88)?;
                c.g[S0][0] = 0x4a0000;
                c.g[A1][0] = c.g[SP][0];
                c.g[S0][0] = sx(0x004a_2f40);
                c.g[A2][0] = 0x100;
                c.g[A3][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                callee::TAG_FIELD_GET_STRING.call(c, 0x0025_cba4)?;
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx(0x004a_3100);
                callee::TAG_FIELD_FIND.call(c, 0x0025_cbb4)?;
                c.g[S1][0] = sx(sp(c).wrapping_add(0x100));
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = 0x100;
                callee::TAG_FIELD_GET_STRING.call(c, 0x0025_cbcc)?;
                c.g[A1][0] = 0x480000;
                c.g[A2][0] = 0x20000000;
                c.g[A3][0] = 0;
                c.g[A1][0] = sx(0x0048_0488);
                c.g[A0][0] = 0x14;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_cbe4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x480000;
                c.g[V1][0] = sx(0x0048_11d0);
                c.g[V0][0] = 0x102;
                c.g[A1][0] = c.g[SP][0];
                Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V1][0]);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0xc));
                Addr(r32(c, S0)).sw(c, c.g[V0][0]);
                Addr(r32(c, S0).wrapping_add(8)).sw(c, c.g[S3][0]);
                callee::C_BX_STRING_CTOR_CSTR.call(c, 0x0025_cc0c)?;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x10));
                callee::C_BX_STRING_CTOR_CSTR.call(c, 0x0025_cc18)?;
                c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx(0x0048_04a8);
                c.g[A0][0] = 0xc;
                Addr(frame.at(c, 0x210)).sw(c, c.g[V1][0]);
                c.g[A2][0] = 0x20000000;
                c.g[A3][0] = 0;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_cc38)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S0][0]);
                }
                c.cycles += 10;
                c.g[V0][0] = Addr(frame.at(c, 0x210)).lw(c);
                Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                Addr(r32(c, A0)).sw(c, c.g[A1][0]);
                c.g[V0][0] = Addr(frame.at(c, 0x210)).lw(c);
                Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
                Addr(frame.at(c, 0x200)).sw(c, c.g[A1][0]);
            } else {
                c.cycles += 25;
                c.g[A0][0] = 0xc;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x488));
                c.g[A2][0] = 0x20000000;
                c.g[A3][0] = 0;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_cc84)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(8)).lw(c);
                c.g[V0][0] = 0x480000;
                c.g[A2][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
                c.g[V0][0] = sx(0x0048_1290);
                c.g[V1][0] = 0x102;
                Addr(r32(c, S0).wrapping_add(8)).sw(c, c.g[A0][0]);
                c.g[A1][0] = 0x480000;
                Addr(r32(c, S0)).sw(c, c.g[V1][0]);
                c.g[A1][0] = sx(0x0048_04a8);
                Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
                c.g[A0][0] = 0xc;
                Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
                c.g[A3][0] = 0;
                c.g[A2][0] = 0x20000000;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0025_ccc4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S0][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
                Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                Addr(r32(c, A0)).sw(c, c.g[A1][0]);
                c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
                Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
                Addr(sp(c)).sw(c, c.g[A1][0]);
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x3b0);
        frame.restore(c, S1, 0x3a0);
        frame.restore(c, S2, 0x390);
        frame.restore(c, S3, 0x380);
        frame.restore(c, S4, 0x370);
        frame.restore(c, S5, 0x360);
        frame.restore(c, S6, 0x350);
        frame.restore(c, S7, 0x340);
        frame.restore(c, FP, 0x330);
        frame.restore_ra(c, 0x320);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::driven;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    driven!(
        gen_lobby_man_on_delete_persona_reply,
        ee::s06::f_0025d538__generated
    );
    driven!(
        hand_lobby_man_on_delete_persona_reply,
        lobby_man_on_delete_persona_reply
    );
    fn put_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    /// Engine heap arena so `cMemMan_alloc` works (same as tagsunk heap_init).
    fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x20_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32);
    }
    fn pool_init(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        put_w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, size)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let start = DATA + 0x40_0000 + 0x4_0000 * i;
            for n in 0..8u32 {
                let node = start + 0x40 * (n + 1);
                put_w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            put_w(c, start + size, start + 0x40);
            put_w(c, GAME_GP + slot, start);
        }
    }
    /// Lobby fixture: heap+pool, BELibrary dummy (gp-0x1f04), world (gp-0x848),
    /// self at DATA+0x1000, msg at DATA+0x2000, list head at DATA+0x2100.
    fn lobby_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        heap_init(c);
        pool_init(c);
        put_w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
        put_w(c, GAME_GP.wrapping_sub(0x848), DATA + 0x8_0000);
        let self_ = DATA + 0x1000;
        let head = DATA + 0x2100;
        put_w(c, self_ + 0xf0, head);
        put_w(c, head, head);
        put_w(c, head + 4, head);
        put_w(c, self_ + 0xf0 + 4, head);
        r
    }
    fn put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    #[test]
    fn lobby_man_move_to_room_matches() {
        check(
            ee::s06::f_0025d860__generated,
            lobby_man_move_to_room,
            60,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25d860);
                c.pc = 0x0025_d860;
                let self_ = DATA + 0x1000;
                c.g[A0][0] = sx(self_);
                put_str(c, DATA + 0x3000, b"RoomA");
                put_str(c, DATA + 0x3100, b"Player1");
                c.g[A1][0] = sx(DATA + 0x3000);
                // Branch (seed-arithmetic only): name present iff seed%2==1.
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x3100) };
                // Branch: net object present iff seed%3!=0.
                put_w(c, self_, if seed % 3 == 0 { 0 } else { 1 });
                put_w(c, self_ + 0x54, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn lobby_man_on_user_list_changed_matches() {
        check(
            ee::s06::f_0025d6c0__generated,
            lobby_man_on_user_list_changed,
            60,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25d6c0);
                c.pc = 0x0025_d6c0;
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn lobby_man_on_delete_persona_reply_matches() {
        check(
            ee::s06::f_0025d538__generated,
            lobby_man_on_delete_persona_reply,
            90,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25d538);
                c.pc = 0x0025_d538;
                let self_ = DATA + 0x1000;
                let msg = DATA + 0x2000;
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(msg);
                // Branch: success iff seed%2==0.
                put_w(c, msg + 8, if seed % 2 == 0 { 0 } else { 7 });
                // Shift loop: count/index by seed arithmetic.
                put_w(c, self_ + 0x6c, 1 + seed % 4);
                put_w(c, self_ + 0x50, seed % 2);
                put_w(c, self_ + 0xec, DATA + 0x2200);
                put_w(c, DATA + 0x2200 + 4, DATA + 0x2100);
            },
        );
    }

    #[test]
    fn lobby_man_on_delete_persona_reply_events() {
        use crate::difftest::events::{check_events, driven, schedule};
        check_events(
            gen_lobby_man_on_delete_persona_reply,
            hand_lobby_man_on_delete_persona_reply,
            60,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25d538);
                c.pc = 0x0025_d538;
                let self_ = DATA + 0x1000;
                let msg = DATA + 0x2000;
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(msg);
                put_w(c, msg + 8, 0);
                put_w(c, self_ + 0x6c, 4);
                put_w(c, self_ + 0x50, 0);
                put_w(c, self_ + 0xec, DATA + 0x2200);
                put_w(c, DATA + 0x2200 + 4, DATA + 0x2100);
                let mut r = Rng(0x25d538 ^ seed as u64);
                schedule(c, &mut r, seed, 0x0025_d538)
            },
        );
    }

    #[test]
    fn lobby_man_delete_persona_matches() {
        check(
            ee::s06::f_0025d428__generated,
            lobby_man_delete_persona,
            60,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25d428);
                c.pc = 0x0025_d428;
                let self_ = DATA + 0x1000;
                c.g[A0][0] = sx(self_);
                // Branch: idx by seed arithmetic; net present iff seed%3!=0.
                c.g[A1][0] = (seed % 3) as u64;
                put_w(c, self_, if seed % 3 == 0 { 0 } else { 1 });
                put_w(c, self_ + 0x54, DATA + 0x4000);
                for i in 0..4u32 {
                    put_w(c, self_ + 0x70 + 4 * i, DATA + 0x3000 + 0x100 * i);
                    put_str(c, DATA + 0x3000 + 0x100 * i, b"Persona");
                }
            },
        );
    }

    #[test]
    fn lobby_man_on_create_persona_reply_matches() {
        check(
            ee::s06::f_0025d2d8__generated,
            lobby_man_on_create_persona_reply,
            90,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25d2d8);
                c.pc = 0x0025_d2d8;
                let self_ = DATA + 0x1000;
                let msg = DATA + 0x2000;
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(msg);
                // Branches: 0=success, 1=dupl, 2=other failure (seed arithmetic).
                put_w(
                    c,
                    msg + 8,
                    match seed % 3 {
                        0 => 0,
                        1 => 0x6475_706c,
                        _ => 9,
                    },
                );
                put_w(c, self_ + 0x6c, seed % 4);
                put_w(c, msg + 0xc, DATA + 0x3000);
                put_str(c, DATA + 0x3000, b"x");
            },
        );
    }

    #[test]
    fn lobby_man_create_persona_matches() {
        check(
            ee::s06::f_0025d1b8__generated,
            lobby_man_create_persona,
            60,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25d1b8);
                c.pc = 0x0025_d1b8;
                let self_ = DATA + 0x1000;
                c.g[A0][0] = sx(self_);
                put_str(c, DATA + 0x3000, b"NewName");
                c.g[A1][0] = sx(DATA + 0x3000);
                put_w(c, self_, if seed % 3 == 0 { 0 } else { 1 });
                put_w(c, self_ + 0x54, DATA + 0x4000);
                put_w(c, self_ + 0x6c, seed % 3);
            },
        );
    }

    #[test]
    fn lobby_man_on_own_stats_reply_matches() {
        check(
            ee::s06::f_0025d048__generated,
            lobby_man_on_own_stats_reply,
            60,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25d048);
                c.pc = 0x0025_d048;
                let self_ = DATA + 0x1000;
                let msg = DATA + 0x2000;
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(msg);
                put_w(c, msg + 8, if seed % 2 == 0 { 0 } else { 5 });
                put_w(c, msg + 0xc, DATA + 0x3000);
                put_str(c, DATA + 0x3000, b"stats");
                put_w(c, self_ + 0x54, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn lobby_man_on_select_persona_reply_matches() {
        check(
            ee::s06::f_0025ceb0__generated,
            lobby_man_on_select_persona_reply,
            60,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25ceb0);
                c.pc = 0x0025_ceb0;
                let self_ = DATA + 0x1000;
                let msg = DATA + 0x2000;
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(msg);
                put_w(c, msg + 8, if seed % 2 == 0 { 0 } else { 5 });
                put_w(c, self_ + 0x80, DATA + 0x3000);
                put_str(c, DATA + 0x3000, b"nm");
            },
        );
    }

    #[test]
    fn lobby_man_select_persona_matches() {
        check(
            ee::s06::f_0025cd50__generated,
            lobby_man_select_persona,
            60,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25cd50);
                c.pc = 0x0025_cd50;
                let self_ = DATA + 0x1000;
                c.g[A0][0] = sx(self_);
                put_str(c, DATA + 0x3000, b"PersonaX");
                c.g[A1][0] = sx(DATA + 0x3000);
                // Branch: self+0x18 flag present iff seed%2==1.
                put_w(c, self_ + 0x18, seed % 2);
                put_str(c, self_ + 0x1c, b"saved");
                put_w(c, self_ + 0x54, DATA + 0x4000);
                put_w(c, self_ + 0x80, 0);
            },
        );
    }

    #[test]
    fn lobby_man_on_login_reply_matches() {
        check(
            ee::s06::f_0025c8a8__generated,
            lobby_man_on_login_reply,
            90,
            &|c, seed| {
                let _r = lobby_machine(c, seed, 0x25c8a8);
                c.pc = 0x0025_c8a8;
                let self_ = DATA + 0x1000;
                let msg = DATA + 0x2000;
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(msg);
                // Branches: 0=success, 1=lock, 2=other failure (seed arithmetic).
                put_w(
                    c,
                    msg + 8,
                    match seed % 3 {
                        0 => 0,
                        1 => 0x6c6f_636b,
                        _ => 9,
                    },
                );
                put_w(c, msg + 0xc, DATA + 0x3000);
                put_str(c, DATA + 0x3000, b"tagdata");
                put_w(c, self_ + 0x54, DATA + 0x4000);
            },
        );
    }
}
