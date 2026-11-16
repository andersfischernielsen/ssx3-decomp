//! Staging module oc405 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc405`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Guest callees called by the ports in this module.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `cCommSystem` channel lookup (0x255d38).
    pub const ABORT_FIND_CHANNEL_255D38: Callee = Callee {
        addr: 0x0025_5d38,
        f: ee::s06::f_00255d38,
    };
    /// Lobby reset / flush entry (0x25b800).
    pub const LOBBY_FLUSH_25B800: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::f_0025b800,
    };
    /// Net manager create (0x260f80).
    pub const NET_CREATE_260F80: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::f_00260f80,
    };
    /// Net session disconnect (0x261408).
    pub const NET_DISCONNECT_261408: Callee = Callee {
        addr: 0x0026_1408,
        f: ee::s06::f_00261408,
    };
    /// Net reconnect (0x2613c8).
    pub const NET_RECONNECT_2613C8: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::f_002613c8,
    };
    /// Net is-connected query (0x261460).
    pub const NET_IS_CONNECTED_261460: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::f_00261460,
    };
    /// Module constructor helper (0x232720).
    pub const MODULE_CTOR_232720: Callee = Callee {
        addr: 0x0023_2720,
        f: ee::s06::f_00232720,
    };
    /// Back-end singleton accessor (0x14dc80).
    pub const BE_GET_BE_14DC80: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };
    /// Back-end interface accessor (0x14dd58).
    pub const BE_GET_INTERFACE_14DD58: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };
    /// Race interface: set human count (0x144aa8).
    pub const RACE_SET_HUMANS_144AA8: Callee = Callee {
        addr: 0x0014_4aa8,
        f: ee::s01::f_00144aa8,
    };
    /// Player interface: set rider slot (0x1472c8).
    pub const PLAYER_SET_RIDER_1472C8: Callee = Callee {
        addr: 0x0014_72c8,
        f: ee::s01::f_001472c8,
    };
    /// Player interface: set rider character (0x147338).
    pub const PLAYER_SET_CHAR_147338: Callee = Callee {
        addr: 0x0014_7338,
        f: ee::s01::f_00147338,
    };
    /// Interface method at 0x14ebf8.
    pub const IFACE_14EBF8: Callee = Callee {
        addr: 0x0014_ebf8,
        f: ee::s01::f_0014ebf8,
    };
    /// `Net_AbortGameSetup` (0x263ba8, generated body in this wave).
    pub const NET_ABORT_263BA8: Callee = Callee {
        addr: 0x0026_3ba8,
        f: ee::s07::f_00263ba8,
    };
    /// `cSetupMan_Create` (0x264030).
    pub const SETUP_CREATE_264030: Callee = Callee {
        addr: 0x0026_4030,
        f: ee::s07::f_00264030,
    };
    /// `cSetupMan_Destroy` (0x264098).
    pub const SETUP_DESTROY_264098: Callee = Callee {
        addr: 0x0026_4098,
        f: ee::s07::f_00264098,
    };
    /// `cSetupMan_SendHello` (0x264cb8).
    pub const SETUP_SEND_HELLO_264CB8: Callee = Callee {
        addr: 0x0026_4cb8,
        f: ee::s07::f_00264cb8,
    };
    /// VoIP activate (0x266e30).
    pub const VOIP_ACTIVATE_266E30: Callee = Callee {
        addr: 0x0026_6e30,
        f: ee::s07::f_00266e30,
    };
    /// VoIP deactivate (0x266e88).
    pub const VOIP_DEACTIVATE_266E88: Callee = Callee {
        addr: 0x0026_6e88,
        f: ee::s07::f_00266e88,
    };
    /// VoIP start (0x266d58).
    pub const VOIP_START_266D58: Callee = Callee {
        addr: 0x0026_6d58,
        f: ee::s07::f_00266d58,
    };
    /// VoIP stop (0x266df8).
    pub const VOIP_STOP_266DF8: Callee = Callee {
        addr: 0x0026_6df8,
        f: ee::s07::f_00266df8,
    };
    /// Memory allocator (0x317d70).
    pub const MEM_ALLOC_317D70: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };
    /// Next-module setter (0x316ef8).
    pub const SET_NEXT_MODULE_316EF8: Callee = Callee {
        addr: 0x0031_6ef8,
        f: ee::s10::f_00316ef8,
    };
    /// `BuddyMan_FindChallengerIndex` (0x262e20).
    pub const BUDDY_FIND_CHALLENGER_262E20: Callee = Callee {
        addr: 0x0026_2e20,
        f: ee::s07::BuddyMan_FindChallengerIndex,
    };
    /// `BuddyMan_IsTemp` (0x262438).
    pub const BUDDY_IS_TEMP_262438: Callee = Callee {
        addr: 0x0026_2438,
        f: ee::s07::BuddyMan_IsTemp,
    };
    /// `BuddyMan_HasMessageFrom` (0x262a38).
    pub const BUDDY_HAS_MESSAGE_262A38: Callee = Callee {
        addr: 0x0026_2a38,
        f: ee::s07::BuddyMan_HasMessageFrom,
    };
    /// `BuddyMan_FindRosterEntry` (0x262468).
    pub const BUDDY_FIND_ROSTER_262468: Callee = Callee {
        addr: 0x0026_2468,
        f: ee::s07::BuddyMan_FindRosterEntry,
    };
    /// `BuddyMan_GetRosterEntry` (0x2625c8).
    pub const BUDDY_GET_ROSTER_2625C8: Callee = Callee {
        addr: 0x0026_25c8,
        f: ee::s07::BuddyMan_GetRosterEntry,
    };
}

/// `void Net_UpdateGameSetup(NetGameSetup* self)` (0x263cf0): advance the
/// game-setup state machine in `self[0]`. State 1 waits for the comm channel:
/// on connect it creates the setup manager and says hello (moving to state
/// 2), on error/timeout it aborts. State 2 waits for the setup manager: when
/// it reports ready (0xd) the race is launched (presence, rider character,
/// game-comm construct and the next module), when it reports 0xb the setup
/// is aborted. Other states return at once.
pub fn net_update_game_setup(c: &mut Cpu) {
    // The frame is named before the blocks: the generated epilogue tails
    // (reached by `break` from the state blocks) reload from it.
    let frame = Frame { size: 0x60 };
    guest(c, |c| {
        'b_26401c: {
            'b_264018: {
                'b_264014: {
                    'b_263dfc: {
                        'b_263d34: {
                            c.cycles += 10;
                            c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x60));
                            frame.save(c, 18, 0x30);
                            frame.save(c, 16, 0x50);
                            c.g[S2][0] = c.g[A0][0];
                            frame.save(c, 17, 0x40);
                            frame.save_ra(c, 0x20);
                            c.g[V1][0] = c.lw(r32(c, S2));
                            let t = c.g[V1][0] == 1u64;
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_263d34;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                frame.restore(c, 16, 0x50);
                                if t {
                                    break 'b_264018;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                                    if t {
                                        break 'b_263dfc;
                                    } else {
                                        c.cycles += 2;
                                        frame.restore(c, 17, 0x40);
                                        break 'b_26401c;
                                    }
                                }
                            }
                        }
                        'b_263da4: {
                            c.cycles += 5;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                            c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
                            call_ptr(c, 0x0025_5d50, 0x0026_3d40)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                            if t {
                                break 'b_263da4;
                            } else {
                                'b_263d64: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((r32(c, S2)).wrapping_add(0xc));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_263d64;
                                    } else {
                                        c.cycles += 4;
                                        c.g[A1][0] = 0x1;
                                        callee::NET_ABORT_263BA8.call(c, 0x0026_3d5c)?;
                                        frame.restore(c, 16, 0x50);
                                        break 'b_264018;
                                    }
                                }
                                c.cycles += 16;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A1][0] = 0x7;
                                c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(0x78));
                                callee::BE_GET_INTERFACE_14DD58.call(c, 0x0026_3d74)?;
                                c.g[V1][0] = 0x530000;
                                c.g[A0][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
                                c.g[A2][0] = c.lw((r32(c, V1)).wrapping_add(0x4b34));
                                c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x8));
                                callee::SETUP_CREATE_264030.call(c, 0x0026_3d88)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                                c.g[A1][0] = c.lw((r32(c, V0)).wrapping_add(0x8c));
                                callee::SETUP_SEND_HELLO_264CB8.call(c, 0x0026_3d98)?;
                                c.g[V0][0] = 0x2;
                                c.sw(r32(c, S2), c.g[V0][0]);
                                break 'b_264014;
                            }
                        }
                        'b_263de8: {
                            'b_263dd8: {
                                c.cycles += 4;
                                c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
                                call_ptr(c, 0x0025_5e40, 0x0026_3dac)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_263dd8;
                                } else {
                                    'b_263dd0: {
                                        c.cycles += 6;
                                        c.lwc1(1, (r32(c, S2)).wrapping_add(0x10));
                                        c.mtc1(0, 0);
                                        c.c_lt_s(1, 0);
                                        let t = c.fcc();
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_263dd0;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] = 0u64;
                                            break 'b_263dd0;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_263de8;
                                    } else {
                                        break 'b_263dd8;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A1][0] = 0u64;
                            callee::NET_ABORT_263BA8.call(c, 0x0026_3de0)?;
                            frame.restore(c, 16, 0x50);
                            break 'b_264018;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.lwc1(0, (r32(c, V0)).wrapping_add(0x14));
                        c.sub_s(0, 1, 0);
                        c.swc1(0, (r32(c, S2)).wrapping_add(0x10));
                        break 'b_264014;
                    }
                    'b_263eb4: {
                        c.cycles += 4;
                        c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
                        call_ptr(c, 0x0025_5e40, 0x0026_3e04)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                        if t {
                            break 'b_263eb4;
                        } else {
                            'b_263ea0: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(r32(c, V1));
                                c.g[V0][0] = c.g[V0][0] ^ 0xb;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = 0x480000;
                                if t {
                                    break 'b_263ea0;
                                } else {
                                    'b_263e7c: {
                                        c.cycles += 24;
                                        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                        c.g[A0][0] = 0x8;
                                        c.g[A2][0] = 0x20000000;
                                        c.g[A3][0] = 0u64;
                                        callee::MEM_ALLOC_317D70.call(c, 0x0026_3e34)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[V1][0] = 0x113;
                                        c.g[V0][0] = 0x480000;
                                        c.sw(r32(c, S1), c.g[V1][0]);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                                        c.g[A1][0] = 0x480000;
                                        c.sw((r32(c, S1)).wrapping_add(0x4), c.g[V0][0]);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                        c.g[A0][0] = 0xc;
                                        c.g[A2][0] = 0x20000000;
                                        c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xf0));
                                        c.g[A3][0] = 0u64;
                                        c.sw((r32(c, SP)).wrapping_add(0x10), c.g[V0][0]);
                                        callee::MEM_ALLOC_317D70.call(c, 0x0026_3e6c)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                                        if c.g[V0][0] != 0u64 {
                                            c.sw((r32(c, A1)).wrapping_add(0x8), c.g[S1][0]);
                                        }
                                        break 'b_263e7c;
                                    }
                                    c.cycles += 9;
                                    c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
                                    c.sw(r32(c, A1), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0x4));
                                    c.sw((r32(c, A1)).wrapping_add(0x4), c.g[V1][0]);
                                    c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(0x4));
                                    c.sw(r32(c, A0), c.g[A1][0]);
                                    c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
                                    c.sw((r32(c, V0)).wrapping_add(0x4), c.g[A1][0]);
                                    c.sw(r32(c, SP), c.g[A1][0]);
                                    break 'b_263ea0;
                                }
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = 0x2;
                            callee::NET_ABORT_263BA8.call(c, 0x0026_3eac)?;
                            frame.restore(c, 16, 0x50);
                            break 'b_264018;
                        }
                    }
                    'b_263eec: {
                        'b_263edc: {
                            c.cycles += 8;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                            c.g[S0][0] = 0u64;
                            call_ptr(c, 0x0026_42b8, 0x0026_3ec0)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                            c.g[V1][0] = 0xd;
                            c.g[A0][0] = c.lw(r32(c, V0));
                            let t = c.g[A0][0] != c.g[V1][0];
                            c.g[V0][0] = 0xb;
                            if t {
                                break 'b_263edc;
                            } else {
                                c.cycles += 2;
                                c.g[S0][0] = 0x1;
                                break 'b_263eec;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_263eec;
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = 0x2;
                            callee::NET_ABORT_263BA8.call(c, 0x0026_3eec)?;
                            break 'b_263eec;
                        }
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == 0u64 {
                        frame.restore(c, 16, 0x50);
                        break 'b_264018;
                    } else {
                        'b_263f1c: {
                            c.cycles += 8;
                            callee::SETUP_DESTROY_264098.call(c, 0x0026_3efc)?;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                            callee::LOBBY_FLUSH_25B800.call(c, 0x0026_3f04)?;
                            c.g[V0][0] = 0x34;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            let t = c.g[V1][0] != 0u64;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x172c), c.g[V0][0]);
                            if t {
                                break 'b_263f1c;
                            } else {
                                c.cycles += 2;
                                callee::NET_CREATE_260F80.call(c, 0x0026_3f1c)?;
                                break 'b_263f1c;
                            }
                        }
                        'b_263f38: {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            callee::NET_IS_CONNECTED_261460.call(c, 0x0026_3f24)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            if t {
                                break 'b_263f38;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                callee::NET_RECONNECT_2613C8.call(c, 0x0026_3f34)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                break 'b_263f38;
                            }
                        }
                        'b_263f4c: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                            if t {
                                break 'b_263f4c;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                callee::LOBBY_FLUSH_25B800.call(c, 0x0026_3f48)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                break 'b_263f4c;
                            }
                        }
                        'b_263fdc: {
                            c.cycles += 33;
                            c.g[T0][0] = 0u64;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            c.g[T1][0] = 0u64;
                            c.g[A3][0] = c.lw((r32(c, V0)).wrapping_add(0x98));
                            c.g[A2][0] = 0x9;
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0026_2768, 0x0026_3f68)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0x7;
                            c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(0x78));
                            callee::BE_GET_INTERFACE_14DD58.call(c, 0x0026_3f78)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lw((r32(c, V1)).wrapping_add(0x78));
                            callee::BE_GET_INTERFACE_14DD58.call(c, 0x0026_3f8c)?;
                            c.g[V1][0] = 0x530000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A2][0] = c.lw((r32(c, V1)).wrapping_add(0x4e58));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            callee::PLAYER_SET_CHAR_147338.call(c, 0x0026_3fa4)?;
                            c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xc));
                            c.g[A0][0] = c.lh((r32(c, V0)).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0xc));
                            let t = r32(c, V1);
                            c.g[A0][0] = sx((r32(c, S0)).wrapping_add(r32(c, A0)));
                            call_ptr(c, t, 0x0026_3fb8)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = 0u64;
                            callee::IFACE_14EBF8.call(c, 0x0026_3fc4)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                            if c.g[V0][0] == 0u64 {
                                c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x8));
                                break 'b_263fdc;
                            } else {
                                c.cycles += 3;
                                call_ptr(c, 0x0025_6c50, 0x0026_3fd8)?;
                                c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x8));
                                break 'b_263fdc;
                            }
                        }
                        c.cycles += 14;
                        c.g[A0][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
                        call_ptr(c, 0x0025_6bf8, 0x0026_3fe4)?;
                        c.g[A1][0] = 0x480000;
                        c.g[A2][0] = 0x100;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xef0));
                        c.g[A3][0] = 0u64;
                        c.g[A0][0] = 0x8;
                        callee::MEM_ALLOC_317D70.call(c, 0x0026_3ffc)?;
                        c.g[A0][0] = c.g[V0][0];
                        callee::MODULE_CTOR_232720.call(c, 0x0026_4004)?;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = c.g[V0][0];
                        callee::SET_NEXT_MODULE_316EF8.call(c, 0x0026_4010)?;
                        c.sw(r32(c, S2), 0u64);
                        break 'b_264014;
                    }
                }
                c.cycles += 1;
                frame.restore(c, 16, 0x50);
            }
            c.cycles += 1;
            frame.restore(c, 17, 0x40);
        }
        c.cycles += 4;
        frame.restore(c, 18, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Post the abort notification node for `Net_AbortGameSetup`: fill the
/// message in `s1` (code already in `v1`), allocate the list node and link
/// it into the manager list at `s0 + 0xf0`. Shared by the arg-1 (0x10c) and
/// arg-0 (0x112) tails, which differ only in the code and first allocation.
fn abort_post_node(c: &mut Cpu, frame: Frame, ret: u32) -> crate::call::Flow {
    c.cycles += 16;
    c.g[V0][0] = 0x0048_0000;
    c.sw(r32(c, S1), c.g[V1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = 0x0048_0000;
    c.sw((r32(c, S1)).wrapping_add(4), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xf0));
    c.g[A3][0] = 0u64;
    c.sw((r32(c, SP)).wrapping_add(0x10), c.g[V0][0]);
    callee::MEM_ALLOC_317D70.call(c, ret)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
    if c.g[V0][0] != 0u64 {
        c.sw((r32(c, A1)).wrapping_add(8), c.g[S1][0]);
    }
    c.cycles += 10;
    c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
    c.sw(r32(c, A1), c.g[V0][0]);
    c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(4));
    c.sw((r32(c, A1)).wrapping_add(4), c.g[V1][0]);
    c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(4));
    c.sw(r32(c, A0), c.g[A1][0]);
    c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
    c.sw((r32(c, V0)).wrapping_add(4), c.g[A1][0]);
    c.sw(r32(c, SP), c.g[A1][0]);
    frame.restore(c, 16, 0x40);
    Ok(())
}

/// `void Net_AbortGameSetup(int* self, int arg)` (0x263ba8): drop the
/// pending game-setup session: remove its comm channel and stop VoIP when
/// the channel exists, destroy the setup manager, reset the lobby state,
/// clear `self[0]`, and post the abort notice (0x10c for arg 1, 0x112 for
/// arg 0; no notice for other args).
pub fn net_abort_game_setup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw(r32(c, S0));
        let no_session = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if !no_session {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            c.g[A1][0] = c.lw((r32(c, S0)).wrapping_add(4));
            callee::ABORT_FIND_CHANNEL_255D38.call(c, 0x0026_3bd4)?;
            let missing = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            if !missing {
                c.cycles += 6;
                c.g[A1][0] = c.lw((r32(c, S0)).wrapping_add(4));
                call_ptr(c, 0x0025_5cc8, 0x0026_3be4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                callee::VOIP_DEACTIVATE_266E88.call(c, 0x0026_3bec)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                callee::VOIP_STOP_266DF8.call(c, 0x0026_3bf4)?;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
        let no_setup = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        if !no_setup {
            c.cycles += 3;
            callee::SETUP_DESTROY_264098.call(c, 0x0026_3c08)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        }
        c.cycles += 5;
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0025_f740, 0x0026_3c14)?;
        c.g[V0][0] = 0x1;
        let other = c.g[S1][0] != c.g[V0][0];
        c.sw(r32(c, S0), 0u64);
        if other {
            c.cycles += 2;
            let queued = c.g[S1][0] != 0u64;
            frame.restore(c, 16, 0x40);
            if !queued {
                c.cycles += 9;
                c.g[A1][0] = 0x0048_0000;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                c.g[A0][0] = 0x8;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                callee::MEM_ALLOC_317D70.call(c, 0x0026_3c6c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x112;
                abort_post_node(c, frame, 0x0026_3ca4)?;
            }
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x0048_0000;
            c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0x8;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0u64;
            callee::MEM_ALLOC_317D70.call(c, 0x0026_3c3c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = 0x10c;
            abort_post_node(c, frame, 0x0026_3ca4)?;
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Call interface slot 1 (`+0xc` vtable, entry at `+8`/`+0xc`) of `iface`
/// for `Net_StartGameSetup`, leaving the dance in v0/a0/v1 as generated.
fn start_iface_call(c: &mut Cpu, iface: u64, ret: u32) -> crate::call::Flow {
    c.g[V0][0] = c.lw((iface as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((r32(c, V0)).wrapping_add(0x8));
    c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0xc));
    let target = r32(c, V1);
    c.g[A0][0] = sx((iface as u32).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// `void Net_StartGameSetup(self, ip, chan, a3, a4)` (0x2639e0): open the
/// game-setup session: configure the race/player interfaces for two
/// humans, publish the DSock channel record (port 0xe4a), format the peer
/// IP and open the channel, start VoIP when the global flag allows, then
/// mark the session live (`self[0] = 1`, timeout at `self + 0x10`).
pub fn net_start_game_setup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 96;
        let frame = Frame::open(c, 0x180);
        frame.save(c, 21, 0x120);
        frame.save(c, 16, 0x170);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 17, 0x160);
        frame.save(c, 18, 0x150);
        frame.save(c, 19, 0x140);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x130);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, 22, 0x110);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        c.g[S6][0] = 0x1;
        c.sw((r32(c, S5)).wrapping_add(0xc), c.g[T0][0]);
        callee::BE_GET_BE_14DC80.call(c, 0x0026_3a20)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        callee::BE_GET_INTERFACE_14DD58.call(c, 0x0026_3a2c)?;
        c.g[S1][0] = c.g[V0][0];
        callee::BE_GET_BE_14DC80.call(c, 0x0026_3a34)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        callee::BE_GET_INTERFACE_14DD58.call(c, 0x0026_3a40)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0x2;
        c.g[A0][0] = c.g[S0][0];
        callee::RACE_SET_HUMANS_144AA8.call(c, 0x0026_3a50)?;
        start_iface_call(c, c.g[S0][0], 0x0026_3a64)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = 0u64;
        callee::PLAYER_SET_RIDER_1472C8.call(c, 0x0026_3a74)?;
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x1;
        callee::PLAYER_SET_RIDER_1472C8.call(c, 0x0026_3a84)?;
        start_iface_call(c, c.g[S1][0], 0x0026_3a98)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x7;
        c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(0x78));
        callee::BE_GET_INTERFACE_14DD58.call(c, 0x0026_3aa8)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4b74));
        c.g[V1][0] = 0x530000;
        c.sw((r32(c, V1)).wrapping_add(0x4b30), c.g[S6][0]);
        c.g[S1][0] = sx((r32(c, V1)).wrapping_add(0x4b30));
        c.g[A0][0] = 0xe4a;
        c.swc1(0, (r32(c, S1)).wrapping_add(0x10));
        c.sw((r32(c, S1)).wrapping_add(0xc), c.g[A0][0]);
        c.g[S0][0] = c.g[V0][0];
        c.sw((r32(c, S1)).wrapping_add(0x8), c.g[S2][0]);
        start_iface_call(c, c.g[S0][0], 0x0026_3ae0)?;
        start_iface_call(c, c.g[S0][0], 0x0026_3af4)?;
        c.sw((r32(c, S5)).wrapping_add(0x8), c.g[S2][0]);
        c.g[A1][0] = 0x480000;
        c.g[A3][0] = sx((r32(c, S4)) >> 16);
        c.g[T0][0] = sx((r32(c, S4)) >> 8);
        c.g[A0][0] = c.g[SP][0];
        c.g[T1][0] = c.g[S4][0] & 0xff;
        c.g[T0][0] = c.g[T0][0] & 0xff;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xee0));
        c.g[A3][0] = c.g[A3][0] & 0xff;
        c.g[A2][0] = sx((r32(c, S4)) >> 24);
        call_ptr(c, 0x0041_7828, 0x0026_3b20)?;
        c.g[A2][0] = c.lhu((r32(c, S1)).wrapping_add(0xc));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
        c.g[A3][0] = c.g[S2][0];
        call_ptr(c, 0x0025_5bf8, 0x0026_3b34)?;
        c.sw((r32(c, S5)).wrapping_add(0x4), c.g[V0][0]);
        callee::BE_GET_BE_14DC80.call(c, 0x0026_3b3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x4;
        callee::BE_GET_INTERFACE_14DD58.call(c, 0x0026_3b48)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = c.lw((r32(c, V1)).wrapping_add(0x5610));
        c.g[V0][0] = sx(((r32(c, V0)) as i32 >> 18) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        let voip_off = c.g[V0][0] == 0u64;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x1dc));
        if !voip_off {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
            c.g[A1][0] = c.g[S4][0];
            callee::VOIP_START_266D58.call(c, 0x0026_3b6c)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
            callee::VOIP_ACTIVATE_266E30.call(c, 0x0026_3b74)?;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x1dc));
        }
        c.cycles += 12;
        c.sw(r32(c, S5), c.g[S6][0]);
        c.swc1(0, (r32(c, S5)).wrapping_add(0x10));
        frame.restore(c, 16, 0x170);
        frame.restore(c, 17, 0x160);
        frame.restore(c, 18, 0x150);
        frame.restore(c, 19, 0x140);
        frame.restore(c, 20, 0x130);
        frame.restore(c, 21, 0x120);
        frame.restore(c, 22, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void Net_DisconnectAll(void* self, void* arg)` (0x2636f0): tear down
/// the network: make sure the net manager exists, stop any session, flush
/// the pending-message list, and when the comm system was busy post the
/// 0x116 disconnect notice carrying `arg` and clear `self[0]`.
pub fn net_disconnect_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 17, 0x50);
        let have = c.g[V0][0] != 0u64;
        frame.save_ra(c, 0x20);
        if !have {
            c.cycles += 2;
            callee::NET_CREATE_260F80.call(c, 0x0026_3720)?;
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::NET_IS_CONNECTED_261460.call(c, 0x0026_3728)?;
        let running = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if !running {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            callee::NET_RECONNECT_2613C8.call(c, 0x0026_3738)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        }
        c.cycles += 2;
        let pending = c.g[V0][0] != 0u64;
        if !pending {
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            callee::LOBBY_FLUSH_25B800.call(c, 0x0026_374c)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::NET_DISCONNECT_261408.call(c, 0x0026_3754)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        callee::LOBBY_FLUSH_25B800.call(c, 0x0026_375c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
        c.g[V0][0] = c.lw((r32(c, A0)).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let idle = c.g[V0][0] == 0u64;
        frame.restore(c, 16, 0x60);
        if !idle {
            c.cycles += 4;
            call_ptr(c, 0x0025_5fa0, 0x0026_3778)?;
            let no_arg = c.g[S2][0] == 0u64;
            c.g[A1][0] = 0x0048_0000;
            if !no_arg {
                c.cycles += 25;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xed0));
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                callee::MEM_ALLOC_317D70.call(c, 0x0026_3798)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x0048_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1250));
                c.g[V0][0] = 0x116;
                c.sw(r32(c, S1), c.g[V0][0]);
                c.g[A1][0] = 0x0048_0000;
                c.sw((r32(c, S1)).wrapping_add(4), c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.sw((r32(c, S1)).wrapping_add(8), c.g[S2][0]);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xf0));
                c.sw((r32(c, SP)).wrapping_add(0x10), c.g[V0][0]);
                callee::MEM_ALLOC_317D70.call(c, 0x0026_37d4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V0][0] != 0u64 {
                    c.sw((r32(c, A1)).wrapping_add(8), c.g[S1][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
                c.sw(r32(c, A1), c.g[V0][0]);
                c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(4));
                c.sw((r32(c, A1)).wrapping_add(4), c.g[V1][0]);
                c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(4));
                c.sw(r32(c, A0), c.g[A1][0]);
                c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
                c.sw((r32(c, V0)).wrapping_add(4), c.g[A1][0]);
                c.sw(r32(c, SP), c.g[A1][0]);
            }
            c.cycles += 2;
            c.sw(r32(c, S3), 0u64);
            frame.restore(c, 16, 0x60);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_DeleteMessage(BuddyMan* self, void* msg)` (0x263500):
/// scan the message vector backwards from its finish (`+0xb0`) to its
/// begin (`+0xac`); on finding `msg`, shift the tail down over it, drop
/// the finish by one slot and release the message through its vtable.
/// Dispatcher-shaped: the backward scan checks events at its head
/// (0x263528), where an unwind resumes.
pub fn buddy_man_delete_message(c: &mut Cpu) {
    const ENTRY: u32 = 0x0026_3500;
    const HEAD: u32 = 0x0026_3528;
    const ADVANCE: u32 = 0x0026_3590;
    const DONE: u32 = 0x0026_35a0;
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 9;
                    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
                    frame.save(c, 16, 0x20);
                    frame.save(c, 17, 0x10);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[A0][0] = c.lw((r32(c, S0)).wrapping_add(0xb0));
                    c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xac));
                    if c.g[A0][0] == c.g[V0][0] {
                        frame.restore(c, 16, 0x20);
                        pc = DONE;
                    } else {
                        c.cycles += 1;
                        c.g[S1][0] = c.lw(r32(c, A0));
                        pc = HEAD;
                    }
                }
                HEAD => {
                    c.cycles += 2;
                    if c.g[S1][0] != c.g[A1][0] {
                        c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xac));
                        pc = ADVANCE;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xb0));
                        c.g[V1][0] = sx((r32(c, A0)).wrapping_add(4));
                        let last = c.g[V1][0] == c.g[V0][0];
                        c.g[A1][0] = sx((r32(c, S0)).wrapping_add(0xa8));
                        if !last {
                            c.cycles += 7;
                            c.g[A2][0] = c.lw((r32(c, A1)).wrapping_add(8));
                            c.g[A1][0] = c.g[V1][0];
                            c.g[A2][0] = sx((r32(c, A2)).wrapping_sub(r32(c, V1)));
                            c.g[A2][0] = sx(((r32(c, A2) as i32) >> 2) as u32);
                            c.g[A2][0] = sx(r32(c, A2) << 2);
                            call_ptr(c, 0x0041_610c, 0x0026_3558)?;
                            c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xb0));
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
                        let empty = c.g[S1][0] == 0u64;
                        c.sw((r32(c, S0)).wrapping_add(0xb0), c.g[V0][0]);
                        if empty {
                            c.cycles += 1;
                            frame.restore(c, 16, 0x20);
                            pc = DONE;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((r32(c, S1)).wrapping_add(0xc));
                            c.g[A1][0] = 0x3;
                            c.g[A0][0] = c.lh((r32(c, V0)).wrapping_add(8));
                            c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0xc));
                            let target = r32(c, V1);
                            c.g[A0][0] = sx((r32(c, S1)).wrapping_add(r32(c, A0)));
                            call_ptr(c, target, 0x0026_3580)?;
                            frame.restore(c, 16, 0x20);
                            pc = DONE;
                        }
                    }
                }
                ADVANCE => {
                    c.cycles += 3;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(4));
                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[S1][0] = c.lw(r32(c, A0));
                        if event_due(c) { deliver_event_here(c, HEAD); }
                        pc = HEAD;
                    } else {
                        c.cycles += 1;
                        frame.restore(c, 16, 0x20);
                        pc = DONE;
                    }
                }
                DONE => {
                    c.cycles += 4;
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, 0x0026_35a8);
                    return Ok(());
                }
            }
        }
    })
}

/// `Buddy* BuddyMan_GetChallengerByName(BuddyMan* self, char* name)`
/// (0x263270): linear scan of the challenger vector (`+0xbc`..`+0xc0`)
/// for the entry whose first word names `name` (case-insensitive);
/// returns the entry or 0. Dispatcher-shaped like the original: the
/// scan checks events at 0x2632ac (match) and 0x2632c8 (loop head).
pub fn buddy_man_get_challenger_by_name(c: &mut Cpu) {
    const ENTRY: u32 = 0x0026_3270;
    const FOUND: u32 = 0x0026_32ac;
    const HEAD: u32 = 0x0026_32c8;
    const DONE: u32 = 0x0026_32ec;
    let frame = Frame { size: 0x50 };
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 13;
                    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x50));
                    frame.save(c, 18, 0x20);
                    frame.save(c, 19, 0x10);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, 16, 0x40);
                    frame.save(c, 17, 0x30);
                    frame.save_ra(c, 0);
                    c.g[A0][0] = c.lw((r32(c, S2)).wrapping_add(0xc0));
                    c.g[V1][0] = c.lw((r32(c, S2)).wrapping_add(0xbc));
                    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
                    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
                    let empty = c.g[V0][0] == 0u64;
                    c.g[S3][0] = c.g[A1][0];
                    if empty {
                        c.cycles += 2;
                        c.g[V0][0] = 0u64;
                        pc = DONE;
                    } else {
                        pc = 0x0026_32b4;
                    }
                }
                0x0026_32b4 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[V1][0];
                    let at_end = c.g[S0][0] == c.g[A0][0];
                    c.g[V0][0] = 0u64;
                    if at_end {
                        pc = DONE;
                    } else {
                        pc = 0x0026_32c0;
                    }
                }
                0x0026_32c0 => {
                    c.cycles += 2;
                    c.g[S1][0] = c.lw(r32(c, S0));
                    pc = HEAD;
                }
                HEAD => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.lw(r32(c, S1));
                    call_ptr(c, 0x0041_65a8, 0x0026_32d4)?;
                    let same = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if same {
                        if back_edge_unwinds(c, FOUND) { return Ok(()); }
                        pc = FOUND;
                    } else {
                        pc = 0x0026_32dc;
                    }
                }
                FOUND => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S1][0];
                    pc = DONE;
                }
                0x0026_32dc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((r32(c, S2)).wrapping_add(0xc0));
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[S1][0] = c.lw(r32(c, S0));
                        if back_edge_unwinds(c, HEAD) { return Ok(()); }
                        pc = HEAD;
                    } else {
                        pc = 0x0026_32e8;
                    }
                }
                0x0026_32e8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = DONE;
                }
                DONE => {
                    c.cycles += 7;
                    frame.restore(c, 16, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 18, 0x20);
                    frame.restore(c, 19, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, 0x0026_3308);
                    return Ok(());
                }
            }
        }
    })
}

/// `bool BuddyMan_HasChallengerStr(BuddyMan* self, char** name)`
/// (0x2631e0): like `BuddyMan_GetChallengerByName`, but the name arrives
/// indirected (`*name`) and the result is 1/0. Dispatcher-shaped: the
/// scan checks events at 0x263218 (match) and 0x263230 (loop head).
pub fn buddy_man_has_challenger_str(c: &mut Cpu) {
    const ENTRY: u32 = 0x0026_31e0;
    const FOUND: u32 = 0x0026_3218;
    const HEAD: u32 = 0x0026_3230;
    const DONE: u32 = 0x0026_3254;
    let frame = Frame { size: 0x40 };
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 12;
                    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
                    frame.save(c, 17, 0x20);
                    frame.save(c, 18, 0x10);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, 16, 0x30);
                    frame.save_ra(c, 0);
                    c.g[A0][0] = c.lw((r32(c, S1)).wrapping_add(0xc0));
                    c.g[V1][0] = c.lw((r32(c, S1)).wrapping_add(0xbc));
                    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
                    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
                    let empty = c.g[V0][0] == 0u64;
                    c.g[S2][0] = c.g[A1][0];
                    if empty {
                        c.cycles += 2;
                        c.g[V0][0] = 0u64;
                        pc = DONE;
                    } else {
                        pc = 0x0026_3220;
                    }
                }
                0x0026_3220 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[V1][0];
                    let at_end = c.g[S0][0] == c.g[A0][0];
                    c.g[V0][0] = 0u64;
                    if at_end {
                        pc = DONE;
                    } else {
                        pc = 0x0026_322c;
                    }
                }
                0x0026_322c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S0));
                    pc = HEAD;
                }
                HEAD => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(r32(c, S2));
                    c.g[A0][0] = c.lw(r32(c, V0));
                    call_ptr(c, 0x0041_65a8, 0x0026_323c)?;
                    let same = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if same {
                        if back_edge_unwinds(c, FOUND) { return Ok(()); }
                        pc = FOUND;
                    } else {
                        pc = 0x0026_3244;
                    }
                }
                FOUND => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    pc = DONE;
                }
                0x0026_3244 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((r32(c, S1)).wrapping_add(0xc0));
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw(r32(c, S0));
                        if back_edge_unwinds(c, HEAD) { return Ok(()); }
                        pc = HEAD;
                    } else {
                        pc = 0x0026_3250;
                    }
                }
                0x0026_3250 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = DONE;
                }
                DONE => {
                    c.cycles += 6;
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore(c, 18, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, 0x0026_326c);
                    return Ok(());
                }
            }
        }
    })
}

/// `void BuddyMan_RemoveChallenger(BuddyMan* self, char* name)`
/// (0x263018): remove the named challenger: notify the lobby, erase the
/// entry from the challenger vector (`+0xbc`..`+0xc0`) and release it;
/// then, for a temporary buddy with no pending message, schedule the
/// roster re-check (600 ticks) through the timer at `self + 0xc8`.
pub fn buddy_man_remove_challenger(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    guest(c, |c| {
        'b_263110: {
            'b_26310c: {
                c.cycles += 11;
                c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
                frame.save(c, 16, 0x30);
                frame.save(c, 18, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, 17, 0x20);
                frame.save_ra(c, 0);
                c.g[S2][0] = c.g[A1][0];
                callee::BUDDY_FIND_CHALLENGER_262E20.call(c, 0x0026_3038)?;
                c.g[S1][0] = c.g[V0][0];
                let t = (c.g[S1][0] as i64) < 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                if t {
                    break 'b_26310c;
                } else {
                    'b_263088: {
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0025_fc28, 0x0026_304c)?;
                        c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xbc));
                        c.g[V1][0] = sx(r32(c, S1) << 2);
                        c.g[A0][0] = c.lw((r32(c, S0)).wrapping_add(0xc0));
                        c.g[A1][0] = sx((r32(c, S0)).wrapping_add(0xb8));
                        c.g[V0][0] = sx((r32(c, V0)).wrapping_add(r32(c, V1)));
                        c.g[V1][0] = sx((r32(c, V0)).wrapping_add(4));
                        let t = c.g[V1][0] == c.g[A0][0];
                        c.g[S1][0] = c.lw(r32(c, V0));
                        if t {
                            break 'b_263088;
                        } else {
                            c.cycles += 7;
                            c.g[A2][0] = c.lw((r32(c, A1)).wrapping_add(8));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[V1][0];
                            c.g[A2][0] = sx((r32(c, A2)).wrapping_sub(r32(c, V1)));
                            c.g[A2][0] = sx(((r32(c, A2) as i32) >> 2) as u32);
                            c.g[A2][0] = sx(r32(c, A2) << 2);
                            call_ptr(c, 0x0041_610c, 0x0026_3088)?;
                            break 'b_263088;
                        }
                    }
                    'b_2630b0: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xc0));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
                        let t = c.g[S1][0] == 0u64;
                        c.sw((r32(c, S0)).wrapping_add(0xc0), c.g[V0][0]);
                        if t {
                            break 'b_2630b0;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((r32(c, S1)).wrapping_add(0x3c));
                            c.g[A1][0] = 0x3;
                            c.g[A0][0] = c.lh((r32(c, V0)).wrapping_add(8));
                            c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0xc));
                            let target = r32(c, V1);
                            c.g[A0][0] = sx((r32(c, S1)).wrapping_add(r32(c, A0)));
                            call_ptr(c, target, 0x0026_30b0)?;
                            break 'b_2630b0;
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    callee::BUDDY_IS_TEMP_262438.call(c, 0x0026_30bc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_26310c;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S2][0];
                        callee::BUDDY_HAS_MESSAGE_262A38.call(c, 0x0026_30cc)?;
                        if c.g[V0][0] != 0u64 {
                            frame.restore(c, 16, 0x30);
                            break 'b_263110;
                        } else {
                            c.cycles += 8;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A0][0] = c.g[S0][0];
                            callee::BUDDY_FIND_ROSTER_262468.call(c, 0x0026_30e0)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            callee::BUDDY_GET_ROSTER_2625C8.call(c, 0x0026_30ec)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A2][0] = 0x270000;
                            if t {
                                break 'b_26310c;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((r32(c, S0)).wrapping_add(0xc8));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A2][0] = sx((r32(c, A2)).wrapping_sub(0x7070));
                                c.g[A3][0] = c.g[S0][0];
                                c.g[T0][0] = 0x258;
                                call_ptr(c, 0x003f_11f8, 0x0026_310c)?;
                                break 'b_26310c;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_3cf0, net_update_game_setup, "Net_UpdateGameSetup"),
    (0x0026_3ba8, net_abort_game_setup, "Net_AbortGameSetup"),
    (0x0026_39e0, net_start_game_setup, "Net_StartGameSetup"),
    (0x0026_36f0, net_disconnect_all, "Net_DisconnectAll"),
    (
        0x0026_3500,
        buddy_man_delete_message,
        "BuddyMan_DeleteMessage",
    ),
    (
        0x0026_3270,
        buddy_man_get_challenger_by_name,
        "BuddyMan_GetChallengerByName",
    ),
    (
        0x0026_31e0,
        buddy_man_has_challenger_str,
        "BuddyMan_HasChallengerStr",
    ),
    (
        0x0026_3018,
        buddy_man_remove_challenger,
        "BuddyMan_RemoveChallenger",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{self, machine, FAKE, LOG_UNWIND_AT};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate for this module's callee fakes: a word in the machine's RAM.
    /// With it off, a hooked slot runs the body it held before.
    const OC405_CTRL: u32 = DATA + 0xf_ff00;
    const OC405_MAGIC: u32 = 0x6f63_3405;
    const OC405_COUNT: u32 = DATA + 0xf_ff04;
    const OC405_UNWIND: u32 = DATA + 0xf_ff08;
    /// Per-callee forced `v0` result cells.
    const OC405_R_BE: u32 = DATA + 0xf_ff10;
    const OC405_R_IFACE: u32 = DATA + 0xf_ff14;
    const OC405_R_ALLOC: u32 = DATA + 0xf_ff18;
    const OC405_R_ISCONN: u32 = DATA + 0xf_ff1c;
    const OC405_R_FIND: u32 = DATA + 0xf_ff20;
    const OC405_R_IDX: u32 = DATA + 0xf_ff24;
    const OC405_R_ISTEMP: u32 = DATA + 0xf_ff28;
    const OC405_R_HASMSG: u32 = DATA + 0xf_ff2c;
    const OC405_R_GROSTER: u32 = DATA + 0xf_ff30;

    macro_rules! oc405_gated {
        ($name:ident, $prev:ident, $cell:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC405_CTRL) == OC405_MAGIC {
                    let idx = c.bus.read32(OC405_COUNT);
                    c.bus.write32(OC405_COUNT, idx + 1);
                    if idx == c.bus.read32(OC405_UNWIND) {
                        c.pc = 0xdead_0000 | idx << 4;
                        return;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx(c.bus.read32($cell));
                    c.pc = r32(c, RA);
                    return;
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    oc405_gated!(
        oc405_fake_get_be,
        OC405_PREV_GET_BE,
        OC405_R_BE,
        ee::s01::f_0014dc80__generated
    );
    oc405_gated!(
        oc405_fake_get_iface,
        OC405_PREV_GET_IFACE,
        OC405_R_IFACE,
        ee::s01::f_0014dd58__generated
    );
    oc405_gated!(
        oc405_fake_set_humans,
        OC405_PREV_SET_HUMANS,
        OC405_R_BE,
        ee::s01::f_00144aa8__generated
    );
    oc405_gated!(
        oc405_fake_set_rider,
        OC405_PREV_SET_RIDER,
        OC405_R_BE,
        ee::s01::f_001472c8__generated
    );
    oc405_gated!(
        oc405_fake_set_char,
        OC405_PREV_SET_CHAR,
        OC405_R_BE,
        ee::s01::f_00147338__generated
    );
    oc405_gated!(
        oc405_fake_iface_ebf8,
        OC405_PREV_IFACE_EBF8,
        OC405_R_BE,
        ee::s01::f_0014ebf8__generated
    );
    oc405_gated!(
        oc405_fake_alloc,
        OC405_PREV_ALLOC,
        OC405_R_ALLOC,
        ee::s10::f_00317d70__generated
    );
    oc405_gated!(
        oc405_fake_setup_create,
        OC405_PREV_SETUP_CREATE,
        OC405_R_ALLOC,
        ee::s07::f_00264030__generated
    );
    oc405_gated!(
        oc405_fake_setup_hello,
        OC405_PREV_SETUP_HELLO,
        OC405_R_BE,
        ee::s07::f_00264cb8__generated
    );
    oc405_gated!(
        oc405_fake_setup_destroy,
        OC405_PREV_SETUP_DESTROY,
        OC405_R_BE,
        ee::s07::f_00264098__generated
    );
    oc405_gated!(
        oc405_fake_flush,
        OC405_PREV_FLUSH,
        OC405_R_BE,
        ee::s06::f_0025b800__generated
    );
    oc405_gated!(
        oc405_fake_net_create,
        OC405_PREV_NET_CREATE,
        OC405_R_BE,
        ee::s06::f_00260f80__generated
    );
    oc405_gated!(
        oc405_fake_is_conn,
        OC405_PREV_IS_CONN,
        OC405_R_ISCONN,
        ee::s06::f_00261460__generated
    );
    oc405_gated!(
        oc405_fake_reconnect,
        OC405_PREV_RECONNECT,
        OC405_R_BE,
        ee::s06::f_002613c8__generated
    );
    oc405_gated!(
        oc405_fake_net_disc,
        OC405_PREV_NET_DISC,
        OC405_R_BE,
        ee::s06::f_00261408__generated
    );
    oc405_gated!(
        oc405_fake_find_chan,
        OC405_PREV_FIND_CHAN,
        OC405_R_FIND,
        ee::s06::f_00255d38__generated
    );
    oc405_gated!(
        oc405_fake_voip_start,
        OC405_PREV_VOIP_START,
        OC405_R_BE,
        ee::s07::f_00266d58__generated
    );
    oc405_gated!(
        oc405_fake_voip_activate,
        OC405_PREV_VOIP_ACTIVATE,
        OC405_R_BE,
        ee::s07::f_00266e30__generated
    );
    oc405_gated!(
        oc405_fake_voip_deactivate,
        OC405_PREV_VOIP_DEACTIVATE,
        OC405_R_BE,
        ee::s07::f_00266e88__generated
    );
    oc405_gated!(
        oc405_fake_voip_stop,
        OC405_PREV_VOIP_STOP,
        OC405_R_BE,
        ee::s07::f_00266df8__generated
    );
    oc405_gated!(
        oc405_fake_module_ctor,
        OC405_PREV_MODULE_CTOR,
        OC405_R_ALLOC,
        ee::s06::f_00232720__generated
    );
    oc405_gated!(
        oc405_fake_set_next,
        OC405_PREV_SET_NEXT,
        OC405_R_BE,
        ee::s10::f_00316ef8__generated
    );
    oc405_gated!(
        oc405_fake_find_chal,
        OC405_PREV_FIND_CHAL,
        OC405_R_IDX,
        ee::s07::f_00262e20__generated
    );
    oc405_gated!(
        oc405_fake_is_temp,
        OC405_PREV_IS_TEMP,
        OC405_R_ISTEMP,
        ee::s07::f_00262438__generated
    );
    oc405_gated!(
        oc405_fake_has_msg,
        OC405_PREV_HAS_MSG,
        OC405_R_HASMSG,
        ee::s07::f_00262a38__generated
    );
    oc405_gated!(
        oc405_fake_find_roster,
        OC405_PREV_FIND_ROSTER,
        OC405_R_GROSTER,
        ee::s07::f_00262468__generated
    );
    oc405_gated!(
        oc405_fake_get_roster,
        OC405_PREV_GET_ROSTER,
        OC405_R_GROSTER,
        ee::s07::f_002625c8__generated
    );

    /// Install every gated fake used below (once per process each).
    fn oc405_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc405_fake_get_be,
            &OC405_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc405_fake_get_iface,
            &OC405_PREV_GET_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144AA8),
            oc405_fake_set_humans,
            &OC405_PREV_SET_HUMANS,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001472C8),
            oc405_fake_set_rider,
            &OC405_PREV_SET_RIDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147338),
            oc405_fake_set_char,
            &OC405_PREV_SET_CHAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014EBF8),
            oc405_fake_iface_ebf8,
            &OC405_PREV_IFACE_EBF8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc405_fake_alloc,
            &OC405_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264030),
            oc405_fake_setup_create,
            &OC405_PREV_SETUP_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264CB8),
            oc405_fake_setup_hello,
            &OC405_PREV_SETUP_HELLO,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264098),
            oc405_fake_setup_destroy,
            &OC405_PREV_SETUP_DESTROY,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025B800),
            oc405_fake_flush,
            &OC405_PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00260F80),
            oc405_fake_net_create,
            &OC405_PREV_NET_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261460),
            oc405_fake_is_conn,
            &OC405_PREV_IS_CONN,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002613C8),
            oc405_fake_reconnect,
            &OC405_PREV_RECONNECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261408),
            oc405_fake_net_disc,
            &OC405_PREV_NET_DISC,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00255D38),
            oc405_fake_find_chan,
            &OC405_PREV_FIND_CHAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00266D58),
            oc405_fake_voip_start,
            &OC405_PREV_VOIP_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00266E30),
            oc405_fake_voip_activate,
            &OC405_PREV_VOIP_ACTIVATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00266E88),
            oc405_fake_voip_deactivate,
            &OC405_PREV_VOIP_DEACTIVATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00266DF8),
            oc405_fake_voip_stop,
            &OC405_PREV_VOIP_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00232720),
            oc405_fake_module_ctor,
            &OC405_PREV_MODULE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00316EF8),
            oc405_fake_set_next,
            &OC405_PREV_SET_NEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262E20),
            oc405_fake_find_chal,
            &OC405_PREV_FIND_CHAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262438),
            oc405_fake_is_temp,
            &OC405_PREV_IS_TEMP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262A38),
            oc405_fake_has_msg,
            &OC405_PREV_HAS_MSG,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262468),
            oc405_fake_find_roster,
            &OC405_PREV_FIND_ROSTER,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002625C8),
            oc405_fake_get_roster,
            &OC405_PREV_GET_ROSTER,
        );
    }

    fn oc405_gate(c: &mut Cpu) {
        oc405_install();
        c.bus.write32(OC405_CTRL, OC405_MAGIC);
        c.bus.write32(OC405_COUNT, 0);
        c.bus.write32(OC405_UNWIND, u32::MAX);
        c.bus.write32(LOG_UNWIND_AT, u32::MAX);
    }

    fn oc405_str(c: &mut Cpu, at: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + text.len() as u32, 0);
    }

    /// A BuddyMan at DATA with three challengers named alice/bob/carol.
    /// Returns the wanted name text for `class` (see the tests).
    fn oc405_challengers(c: &mut Cpu, class: u32) -> &'static [u8] {
        let begin = DATA + 0x200;
        c.bus.write32(DATA + 0xbc, begin);
        let n: u32 = if class == 0 { 0 } else { 3 };
        c.bus.write32(DATA + 0xc0, begin + 4 * n);
        let names: [&[u8]; 3] = [b"alice", b"bob", b"carol"];
        for i in 0..3u32 {
            let entry = DATA + 0x400 + 0x40 * i;
            c.bus.write32(begin + 4 * i, entry);
            c.bus.write32(entry, DATA + 0x600 + 0x20 * i);
            oc405_str(c, DATA + 0x600 + 0x20 * i, names[i as usize]);
        }
        match class {
            1 => b"alice",
            2 => b"carol",
            3 => b"dave",
            _ => b"bob",
        }
    }

    #[test]
    fn buddy_man_has_challenger_str_matches() {
        check(
            ee::s07::f_002631e0__generated,
            buddy_man_has_challenger_str,
            100,
            &|c, seed| {
                let mut r = Rng(0x0026_31e0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let want = oc405_challengers(c, seed % 5);
                oc405_str(c, DATA + 0x800, want);
                c.bus.write32(DATA + 0x880, DATA + 0x800);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x880) as u64;
                c.pc = 0x0026_31e0;
            },
        );
    }

    #[test]
    fn buddy_man_has_challenger_str_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x0026_31e0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let want = oc405_challengers(c, seed % 5);
            oc405_str(c, DATA + 0x800, want);
            c.bus.write32(DATA + 0x880, DATA + 0x800);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x880) as u64;
            schedule(c, &mut r, seed, 0x0026_31e0)
        };
        check_events(
            ee::s07::f_002631e0__generated,
            buddy_man_has_challenger_str,
            60,
            &setup,
        );
    }

    #[test]
    fn buddy_man_get_challenger_by_name_matches() {
        check(
            ee::s07::f_00263270__generated,
            buddy_man_get_challenger_by_name,
            100,
            &|c, seed| {
                let mut r = Rng(0x0026_3270 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let want = oc405_challengers(c, seed % 5);
                oc405_str(c, DATA + 0x800, want);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.pc = 0x0026_3270;
            },
        );
    }

    #[test]
    fn buddy_man_get_challenger_by_name_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x0026_3270 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let want = oc405_challengers(c, seed % 5);
            oc405_str(c, DATA + 0x800, want);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            schedule(c, &mut r, seed, 0x0026_3270)
        };
        check_events(
            ee::s07::f_00263270__generated,
            buddy_man_get_challenger_by_name,
            60,
            &setup,
        );
    }

    /// Message vector fixture for `BuddyMan_DeleteMessage`: entries in the
    /// slots at DATA+0x200.., finish pointing at the last slot.
    fn oc405_messages(c: &mut Cpu, seed: u32) -> u32 {
        let buf = DATA + 0x200;
        for i in 0..3u32 {
            let obj = DATA + 0x400 + 0x40 * i;
            let vt = DATA + 0x800 + 0x20 * i;
            c.bus.write32(buf + 4 * i, obj);
            c.bus.write32(obj + 0xc, vt);
            c.bus.write16(vt + 8, 0);
            c.bus.write32(vt + 0xc, FAKE);
        }
        // The scan starts at the finish slot `*(self+0xb0)` and walks down
        // to `*(self+0xac)`; the finish slot holds a dummy that never
        // matches, the three messages sit below it. A match one slot below
        // the finish needs no tail shift; lower matches shift the slots
        // above them down by one (memmove size from the finish value).
        c.bus.write32(buf + 12, DATA + 0x6000);
        let class = seed % 5;
        if class == 0 {
            // Empty: begin == finish.
            c.bus.write32(DATA + 0xac, buf + 12);
            c.bus.write32(DATA + 0xb0, buf + 12);
        } else {
            c.bus.write32(DATA + 0xac, buf - 4);
            c.bus.write32(DATA + 0xb0, buf + 12);
        }
        match class {
            // Target one slot below the finish: no tail shift.
            1 => DATA + 0x400 + 0x80,
            // Target two slots below: tail shifts by one.
            2 => DATA + 0x400 + 0x40,
            // Null message two slots below: drop only.
            4 => {
                c.bus.write32(buf + 4, 0);
                0
            }
            // Absent (also the empty-vector class).
            _ => DATA + 0x5000,
        }
    }

    #[test]
    fn buddy_man_delete_message_matches() {
        check(
            ee::s07::f_00263500__generated,
            buddy_man_delete_message,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x263500, 4);
                c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                let target = oc405_messages(c, seed);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = target as u64;
                c.pc = 0x0026_3500;
            },
        );
    }

    #[test]
    fn buddy_man_delete_message_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x263500, 4);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            let target = oc405_messages(c, seed);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = target as u64;
            schedule(c, &mut r, seed, 0x0026_3500)
        };
        check_events(ee::s07::f_00263500__generated, buddy_man_delete_message, 60, &setup);
    }

    #[test]
    fn buddy_man_remove_challenger_matches() {
        check(
            ee::s07::f_00263018__generated,
            buddy_man_remove_challenger,
            160,
            &|c, seed| {
                let _r = machine(c, seed, 0x263018, 6);
                oc405_gate(c);
                // Challenger vector (finish at the last slot) and vtables.
                let buf = DATA + 0x200;
                for i in 0..3u32 {
                    let obj = DATA + 0x400 + 0x40 * i;
                    let vt = DATA + 0x800 + 0x20 * i;
                    c.bus.write32(buf + 4 * i, obj);
                    c.bus.write32(obj + 0x3c, vt);
                    c.bus.write16(vt + 8, 0);
                    c.bus.write32(vt + 0xc, FAKE);
                }
                c.bus.write32(DATA + 0xbc, buf);
                c.bus.write32(DATA + 0xc0, buf + 12);
                c.bus.write32(DATA + 0xc8, DATA + 0x2000);
                // Lobby manager for the notify callee: no API, one list node.
                let mgr = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0xc8, mgr);
                c.bus.write32(mgr + 0xf0, DATA + 0x1800);
                c.bus.write32(DATA + 0x1800 + 4, DATA + 0x1900);
                // Branch steering from the seed residue only.
                let class = seed % 8;
                let (idx, temp, msg, roster) = match class {
                    0 => (u32::MAX, 0, 0, 0),
                    1 => (2, 0, 0, 0),
                    2 => (1, 0, 0, 0),
                    3 => (0, 1, 1, 0),
                    4 => (1, 1, 0, 0),
                    5 => (1, 1, 0, DATA + 0x2100),
                    6 => (1, 0, 0, 0),
                    _ => (2, 1, 0, DATA + 0x2100),
                };
                if class == 6 {
                    c.bus.write32(buf + 4, 0);
                }
                c.bus.write32(OC405_R_IDX, idx);
                c.bus.write32(OC405_R_ISTEMP, temp);
                c.bus.write32(OC405_R_HASMSG, msg);
                c.bus.write32(OC405_R_GROSTER, roster);
                if class == 7 {
                    c.bus.write32(OC405_UNWIND, (seed / 8) % 3);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                c.pc = 0x0026_3018;
            },
        );
    }

    /// Globals shared by the Net fixtures: comm system, lobby manager,
    /// setup manager, world and the BE-library dummy.
    fn oc405_net_globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9000);
        let world = DATA + 0x4000;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x78, DATA + 0x2100);
        c.bus.write32(world + 0x14, 0x3f00_0000);
        let mgr = DATA + 0x1000;
        c.bus.write32(GAME_GP - 0xc8, mgr);
        c.bus.write32(mgr + 0xf0, DATA + 0x1800);
        c.bus.write32(DATA + 0x1800 + 4, DATA + 0x1900);
        c.bus.write32(mgr + 0x8c, 7);
        c.bus.write32(mgr + 0x98, 9);
        let comm = DATA + 0x3000;
        c.bus.write32(GAME_GP - 0x238, comm);
        c.bus.write32(comm, DATA + 0x3100);
        let chan = DATA + 0x3100;
        let vt = DATA + 0x3200;
        c.bus.write32(chan + 8, vt);
        c.bus.write16(vt + 8, 0);
        c.bus.write32(vt + 0xc, FAKE);
        c.bus.write16(vt + 0x10, 0);
        c.bus.write32(vt + 0x14, FAKE);
        c.bus.write16(vt + 0x20, 0);
        c.bus.write32(vt + 0x24, FAKE);
        let iface = DATA + 0x2100;
        c.bus.write32(iface + 0xc, DATA + 0x2200);
        c.bus.write16(DATA + 0x2200 + 8, 0);
        c.bus.write32(DATA + 0x2200 + 0xc, FAKE);
    }

    #[test]
    fn net_disconnect_all_matches() {
        check(ee::s07::f_002636f0__generated, net_disconnect_all, 160, &|c, seed| {
            let _r = machine(c, seed, 0x2636f0, 6);
            oc405_gate(c);
            oc405_net_globals(c);
            c.bus.write32(OC405_R_ALLOC, DATA + 0x6000);
            c.bus.write32(OC405_R_ISCONN, (seed / 2) & 1);
            // Net manager pointer and the comm busy flag, from seed bits.
            if seed & 1 == 0 {
                c.bus.write32(GAME_GP + 0x238, DATA + 0x3800);
            }
            let comm = DATA + 0x3000;
            c.bus.write32(comm + 0x1c, u32::MAX);
            c.bus.write32(comm + 0x20, (seed / 4) & 1);
            if seed % 16 == 15 {
                c.bus.write32(OC405_UNWIND, (seed / 16) % 4);
            }
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed & 8 == 0 {
                0
            } else {
                (DATA + 0x500) as u64
            };
            c.pc = 0x0026_36f0;
        });
    }

    #[test]
    fn net_abort_game_setup_matches() {
        check(
            ee::s07::f_00263ba8__generated,
            net_abort_game_setup,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x263ba8, 6);
                oc405_gate(c);
                oc405_net_globals(c);
                c.bus.write32(OC405_R_ALLOC, DATA + 0x6000);
                c.bus.write32(OC405_R_FIND, (seed / 2) & 1);
                if (seed / 4) & 1 == 1 {
                    c.bus.write32(GAME_GP + 0x2dc, DATA + 0x2000);
                }
                c.bus.write32(GAME_GP + 0x304, DATA + 0x5000);
                c.bus.write32(DATA, seed & 1);
                c.bus.write32(DATA + 4, 0);
                if seed % 24 == 23 {
                    c.bus.write32(OC405_UNWIND, (seed / 24) % 3);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.pc = 0x0026_3ba8;
            },
        );
    }

    #[test]
    fn net_start_game_setup_matches() {
        check(ee::s07::f_002639e0__generated, net_start_game_setup, 60, &|c, seed| {
            let _r = machine(c, seed, 0x2639e0, 8);
            oc405_gate(c);
            oc405_net_globals(c);
            c.bus.write32(OC405_R_BE, DATA + 0x2000);
            c.bus.write32(OC405_R_IFACE, DATA + 0x2100);
            c.bus.write32(OC405_R_ALLOC, DATA + 0x6000);
            c.bus.write32(GAME_GP - 0x4b74, 0x3c88_8889);
            c.bus.write32(GAME_GP - 0x1dc, 0x3f80_0000);
            c.bus.write32(GAME_GP + 0x304, DATA + 0x5000);
            c.bus
                .write32(0x0053_5610, if seed & 1 == 1 { 1 << 18 } else { 0 });
            if seed % 12 == 11 {
                c.bus.write32(OC405_UNWIND, (seed / 12) % 4);
            }
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (0xc0a8_0101 + (seed % 4)) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = (seed % 3) as u64;
            c.g[T0][0] = (seed % 5) as u64;
            c.pc = 0x0026_39e0;
        });
    }

    #[test]
    fn net_update_game_setup_matches() {
        check(
            ee::s07::f_00263cf0__generated,
            net_update_game_setup,
            220,
            &|c, seed| {
                let _r = machine(c, seed, 0x263cf0, 10);
                oc405_gate(c);
                oc405_net_globals(c);
                c.bus.write32(OC405_R_IFACE, DATA + 0x2100);
                c.bus.write32(OC405_R_ALLOC, DATA + 0x6000);
                c.bus.write32(OC405_R_ISCONN, 1);
                c.bus.write32(OC405_R_FIND, 1);
                c.bus.write32(GAME_GP + 0x238, DATA + 0x3800);
                c.bus.write32(GAME_GP + 0x304, DATA + 0x5000);
                let setup = DATA + 0x2000;
                c.bus.write32(GAME_GP + 0x2dc, setup);
                let comm = DATA + 0x3000;
                let chan = DATA + 0x3100;
                // Branch steering from the seed residue only (11 classes).
                let class = seed % 11;
                let (state, setup_state, comm28, chan_fn, timer) = match class {
                    0 => (0, 0, 0, FAKE, 0),
                    1 => (3, 0, 0, FAKE, 0),
                    2 => (1, 0, 0, FAKE, 0),
                    3 => (1, 0, 0, FAKE, 0),
                    4 => (1, 0, 1, 0x0014_06c0, 0),
                    5 => (1, 0, 0, 0x0014_06c0, 0x3fc0_0000),
                    6 => (1, 0, 0, 0x0014_06c0, 0xbfc0_0000),
                    7 => (2, 5, 1, FAKE, 0),
                    8 => (2, 0xd, 0, FAKE, 0),
                    9 => (2, 0xb, 0, FAKE, 0),
                    _ => (2, 5, 0, FAKE, 0),
                };
                c.bus.write32(DATA, state);
                c.bus.write32(DATA + 4, 0);
                c.bus.write32(DATA + 8, DATA + 0x1111);
                c.bus.write32(DATA + 0xc, if class == 2 { 1 } else { 0 });
                c.bus.write32(DATA + 0x10, timer);
                c.bus.write32(setup, setup_state);
                c.bus.write32(comm + 0x28, comm28);
                c.bus.write32(chan + 4, 0);
                c.bus.write32(chan + 0x20, 0);
                c.bus.write32(DATA + 0x3200 + 0x24, chan_fn);
                if class == 9 {
                    c.bus.write32(OC405_UNWIND, 1);
                }
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0026_3cf0;
            },
        );
    }
}
