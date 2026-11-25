//! Staging module oc698 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc698`.

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
    (0x00426020, sce_sif_add_cmd_handler, "sceSifAddCmdHandler"),
    (0x004261b0, sce_sif_send_cmd, "sceSifSendCmd"),
    (
        0x00426358,
        sce_sif_write_back_d_cache,
        "sceSifWriteBackDCache",
    ),
    (0x004265d0, sce_sif_rpc_get_packet, "sceSifRpc_GetPacket"),
    (0x00426678, sce_sif_rpc_free_packet, "sceSifRpc_FreePacket"),
    (0x00426708, sce_sif_rpc_request_end, "sceSifRpc_RequestEnd"),
    (0x00426b48, sce_sif_bind_rpc, "sceSifBindRpc"),
    (0x00426d18, sce_sif_call_rpc, "sceSifCallRpc"),
    (0x00426f08, sce_sif_check_stat_rpc, "sceSifCheckStatRpc"),
    (0x00427438, sce_fs_fd_sema_init, "sceFs_FdSemaInit"),
    (0x00427938, sce_fs_sema_init, "sceFs_SemaInit"),
];

/// Direct-call targets of this module (address, generated function).
mod oc698_callee {
    use super::{ee, Callee};

    /// `_sceSifSendCmd` (`0x00426078`).
    pub(super) const OC698_U_SCE_SIF_SEND_CMD: Callee = Callee {
        addr: 0x00426078,
        f: ee::s15::f_00426078,
    };

    /// `DIntr` (`0x0042c078`).
    pub(super) const OC698_DINTR: Callee = Callee {
        addr: 0x0042c078,
        f: ee::s15::f_0042c078,
    };

    /// `EIntr` (`0x0042c0c0`).
    pub(super) const OC698_EINTR: Callee = Callee {
        addr: 0x0042c0c0,
        f: ee::s15::f_0042c0c0,
    };

    /// `sceSifInitCmd` (`0x00425d38`).
    pub(super) const OC698_SCE_SIF_INIT_CMD: Callee = Callee {
        addr: 0x00425d38,
        f: ee::s15::f_00425d38,
    };

    /// `sceSifAddCmdHandler` (`0x00426020`).
    pub(super) const OC698_SCE_SIF_ADD_CMD_HANDLER: Callee = Callee {
        addr: 0x00426020,
        f: ee::s15::f_00426020,
    };

    /// `sceSifGetReg` (`0x004241b0`).
    pub(super) const OC698_SCE_SIF_GET_REG: Callee = Callee {
        addr: 0x004241b0,
        f: ee::s15::f_004241b0,
    };

    /// `sceSifSendCmd` (`0x004261b0`).
    pub(super) const OC698_SCE_SIF_SEND_CMD: Callee = Callee {
        addr: 0x004261b0,
        f: ee::s15::f_004261b0,
    };

    /// `sceSifGetSreg` (`0x00425cf0`).
    pub(super) const OC698_SCE_SIF_GET_SREG: Callee = Callee {
        addr: 0x00425cf0,
        f: ee::s15::f_00425cf0,
    };

    /// `sceSifSetReg` (`0x004241a0`).
    pub(super) const OC698_SCE_SIF_SET_REG: Callee = Callee {
        addr: 0x004241a0,
        f: ee::s15::f_004241a0,
    };

    /// `iSignalSema` (`0x00423dd0`).
    pub(super) const OC698_I_SIGNAL_SEMA: Callee = Callee {
        addr: 0x00423dd0,
        f: ee::s15::f_00423dd0,
    };

    /// `sceSifRpc_FreePacket` (`0x00426678`).
    pub(super) const OC698_SCE_SIF_RPC_FREE_PACKET: Callee = Callee {
        addr: 0x00426678,
        f: ee::s15::f_00426678,
    };

    /// `sceSifRpc_GetPacket` (`0x004265d0`).
    pub(super) const OC698_SCE_SIF_RPC_GET_PACKET: Callee = Callee {
        addr: 0x004265d0,
        f: ee::s15::f_004265d0,
    };

    /// `CreateSema` (`0x00423da0`).
    pub(super) const OC698_CREATE_SEMA: Callee = Callee {
        addr: 0x00423da0,
        f: ee::s15::f_00423da0,
    };

    /// `DeleteSema` (`0x00423db0`).
    pub(super) const OC698_DELETE_SEMA: Callee = Callee {
        addr: 0x00423db0,
        f: ee::s15::f_00423db0,
    };

    /// `WaitSema` (`0x00423de0`).
    pub(super) const OC698_WAIT_SEMA: Callee = Callee {
        addr: 0x00423de0,
        f: ee::s15::f_00423de0,
    };

    /// `sceSifWriteBackDCache` (`0x00426358`).
    pub(super) const OC698_SCE_SIF_WRITE_BACK_DCACHE: Callee = Callee {
        addr: 0x00426358,
        f: ee::s15::f_00426358,
    };
}

/// `void sceSifAddCmdHandler(int cid, void (*handler)(void), int arg)` (0x426020):
/// store `{handler, arg}` in the user (cid >= 0, table at `[0x52bce4]`) or system
/// (cid < 0, table at `[0x52bcec]`) command-handler table, indexed by `cid * 8`.
pub fn sce_sif_add_cmd_handler(c: &mut Cpu) {
    'b_42603c: {
        'b_426034: {
            c.cycles += 2;
            let t = (c.g[A0][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 3);
            if t {
                break 'b_426034;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x530000;
                let t = true;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x431c));
                if t {
                    break 'b_42603c;
                } else {
                    break 'b_426034;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0x530000;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x4314));
        break 'b_42603c;
    }
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
    let t = c.g[RA][0] as u32;
    c.sw((c.g[V1][0] as u32), c.g[A1][0]);
    c.pc = t;
    return;
}

/// `int sceSifSendCmd(int cid, void *pkt, int pktsize, void *src, void *dest, int size)`
/// (0x4261b0): forwards to `_sceSifSendCmd(cid, mode = 0, pkt, pktsize, src, dest, size)`
/// (0x426078), shifting the arguments up.
pub fn sce_sif_send_cmd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = c.g[A2][0];
        c.g[V1][0] = c.g[A3][0];
        c.g[T3][0] = c.g[T0][0];
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[T2][0] = c.g[T1][0];
        c.g[A2][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = c.g[V1][0];
        c.g[T1][0] = c.g[T3][0];
        c.g[RA][0] = sx(0x004261e0);
        c.g[A1][0] = 0u64;
        oc698_callee::OC698_U_SCE_SIF_SEND_CMD.call(c, 0x004261e0)?;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    })
}

/// `void sceSifWriteBackDCache(void *addr, int size)` (0x426358): the cache
/// write-back by 64-byte lines (the `cache` instructions are not modelled; only
/// the line arithmetic and its cycle cost remain): the line count is
/// `((addr + size - 1) & ~63) - (addr & ~63)) / 64 + 1`, walked in 8-line steps
/// of 19 cycles plus a remainder of 7 cycles per line.
pub fn sce_sif_write_back_d_cache(c: &mut Cpu) {
    'b_4263fc: {
        c.cycles += 4;
        c.g[T9][0] = 0xffffffffffff0000;
        c.g[T9][0] = c.g[T9][0] | 0xffc0;
        let t = (c.g[A1][0] as i64) <= 0;
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        if t {
            break 'b_4263fc;
        } else {
            'b_4263a8: {
                c.cycles += 9;
                c.g[T0][0] = c.g[A0][0] & c.g[T9][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                c.g[T1][0] = c.g[T2][0] & c.g[T9][0];
                c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[T0][0] as u32)));
                c.g[T3][0] = sx((c.g[T2][0] as u32) >> 6);
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1));
                c.g[T1][0] = c.g[T3][0] & 0x7;
                let t = c.g[T1][0] == 0u64;
                c.g[T2][0] = sx((c.g[T3][0] as u32) >> 3);
                if t {
                    break 'b_4263a8;
                } else {
                    'l_42638c: loop {
                        c.cycles += 7;
                        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[T1][0] as i64) > 0;
                        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0042638c);
                            }
                            continue 'l_42638c;
                        } else {
                            break 'b_4263a8;
                        }
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[T2][0] == 0u64;
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
            if t {
                break 'b_4263fc;
            } else {
                'l_4263b0: loop {
                    c.cycles += 19;
                    let t = (c.g[T2][0] as i64) > 0;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x200));
                    if t {
                        c.cycles += 1;
                        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                        if event_due(c) {
                            deliver_event_here(c, 0x004263b0);
                        }
                        continue 'l_4263b0;
                    } else {
                        break 'b_4263fc;
                    }
                }
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
    return;
}

/// `packet *sceSifRpc_GetPacket(pool *p)` (0x4265d0): under `DIntr`, claim the
/// first free packet (flag bit 0 clear) of the pool `{total, packets, count}`
/// (stride 0x40): flags = `index << 16 | 5`, bump the pool total (skipping 0
/// on wrap), `[pkt+0x14] = pkt`, `[pkt+0x18] = total`. Returns the packet or 0.
pub fn sce_sif_rpc_get_packet(c: &mut Cpu) {
    guest(c, |c| {
        'b_426664: {
            'b_426658: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                c.sd((c.g[SP][0] as u32), c.g[S0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[RA][0] = sx(0x004265e8);
                c.g[S1][0] = c.g[A0][0];
                oc698_callee::OC698_DINTR.call(c, 0x004265e8)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0u64;
                let t = (c.g[A0][0] as i64) <= 0;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                if t {
                    break 'b_426658;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 0x1;
                    'l_426600: loop {
                        'b_42664c: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            if c.g[V0][0] != 0u64 {
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                break 'b_42664c;
                            } else {
                                'b_426638: {
                                    c.cycles += 7;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 16);
                                    c.g[V0][0] = c.g[V0][0] | 0x5;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    let t = c.g[V1][0] != c.g[A1][0];
                                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                                    if t {
                                        break 'b_426638;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                                        c.g[V1][0] = 0x1;
                                        c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                                        break 'b_426638;
                                    }
                                }
                                c.cycles += 5;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                                c.g[RA][0] = sx(0x00426644);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                oc698_callee::OC698_EINTR.call(c, 0x00426644)?;
                                let t = true;
                                c.g[V0][0] = c.g[S0][0];
                                if t {
                                    break 'b_426664;
                                } else {
                                    break 'b_42664c;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00426600);
                            }
                            continue 'l_426600;
                        } else {
                            break 'b_426658;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x00426660);
            oc698_callee::OC698_EINTR.call(c, 0x00426660)?;
            c.g[V0][0] = 0u64;
            break 'b_426664;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// `void sceSifRpc_FreePacket(packet *p)` (0x426678): clear `[p+0x18]` and the
/// flags' in-use bit (`[p+0x10] &= 0xffff_fffe`).
pub fn sce_sif_rpc_free_packet(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = 0xffffffffffff0000;
    c.g[V0][0] = c.g[V0][0] | 0xfffe;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x18), 0u64);
    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
    c.pc = t;
    return;
}

/// `void sceSifRpc_RequestEnd(packet *reply)` (0x426708): the SIF command
/// handler for RPC replies. By command id (`[reply+0x20]`): 0x8000000a (call
/// done) runs the client's end callback `[cd+0x1c](cd[0x20])`; 0x80000009 (bind
/// done) copies the server words into the client; then the client's sema
/// (`[cd+8]`, when not negative) is signalled and the packet freed.
pub fn sce_sif_rpc_request_end(c: &mut Cpu) {
    guest(c, |c| {
        'b_42678c: {
            'b_426788: {
                'b_426770: {
                    'b_426750: {
                        c.cycles += 10;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                        c.g[V0][0] = 0xffffffff80000000;
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                        c.g[V0][0] = c.g[V0][0] | 0xa;
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                        c.g[S1][0] = c.g[A0][0];
                        c.sd((c.g[SP][0] as u32), c.g[S0][0]);
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                        if t {
                            break 'b_426750;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != 0u64 {
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                break 'b_426788;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = 0xffffffff80000000;
                                c.g[V0][0] = c.g[V0][0] | 0x9;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    break 'b_426770;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    if t {
                                        break 'b_426788;
                                    } else {
                                        break 'b_426750;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_42678c;
                    } else {
                        c.cycles += 4;
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00426768);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                        call_ptr(c, t, 0x00426768)?;
                        let t = true;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        if t {
                            break 'b_426788;
                        } else {
                            break 'b_426770;
                        }
                    }
                }
                c.cycles += 6;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_426788;
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            break 'b_42678c;
        }
        'b_42679c: {
            c.cycles += 2;
            let t = (c.g[A0][0] as i64) < 0;
            if t {
                break 'b_42679c;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0042679c);
                oc698_callee::OC698_I_SIGNAL_SEMA.call(c, 0x0042679c)?;
                break 'b_42679c;
            }
        }
        c.cycles += 8;
        c.g[RA][0] = sx(0x004267a4);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
        oc698_callee::OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x004267a4)?;
        c.sw((c.g[S0][0] as u32), 0u64);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// `int sceSifBindRpc(client *cd, int sid, int mode)` (0x426b48): bind a client to
/// a server. Takes a packet from the pool (0x52d680; -1 when none), fills it with
/// the bind request (cid 0x80000009), and sends it. Synchronous mode (bit 0
/// clear) makes a completion sema, waits for the reply and deletes the sema
/// (-3: CreateSema failed, packet freed; -2: send failed). Returns 0 on success.
pub fn sce_sif_bind_rpc(c: &mut Cpu) {
    guest(c, |c| {
        'b_426c6c: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
            c.g[A0][0] = 0x530000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.g[S3][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.g[S2][0] = c.g[A2][0];
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0u64);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2980));
            c.g[RA][0] = sx(0x00426b80);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x24), 0u64);
            oc698_callee::OC698_SCE_SIF_RPC_GET_PACKET.call(c, 0x00426b80)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_426c6c;
            } else {
                'b_426c30: {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.g[S2][0] & 0x1;
                    c.sw((c.g[S1][0] as u32), c.g[S0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S3][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                    let t = c.g[V1][0] != 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S1][0]);
                    if t {
                        break 'b_426c30;
                    } else {
                        'b_426bd8: {
                            c.cycles += 7;
                            c.g[V0][0] = 0x1;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.g[RA][0] = sx(0x00426bc0);
                            c.g[A0][0] = c.g[SP][0];
                            oc698_callee::OC698_CREATE_SEMA.call(c, 0x00426bc0)?;
                            let t = (c.g[V0][0] as i64) >= 0;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_426bd8;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00426bd0);
                                c.g[A0][0] = c.g[S0][0];
                                oc698_callee::OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x00426bd0)?;
                                let t = true;
                                c.g[V0][0] = 0xfffffffffffffffd;
                                if t {
                                    break 'b_426c6c;
                                } else {
                                    break 'b_426bd8;
                                }
                            }
                        }
                        'b_426c18: {
                            c.cycles += 10;
                            c.g[A0][0] = 0xffffffff80000000;
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[A0][0] | 0x9;
                            c.g[A2][0] = 0x40;
                            c.g[A3][0] = 0u64;
                            c.g[T0][0] = 0u64;
                            c.g[RA][0] = sx(0x00426bf8);
                            c.g[T1][0] = 0u64;
                            oc698_callee::OC698_SCE_SIF_SEND_CMD.call(c, 0x00426bf8)?;
                            let t = c.g[V0][0] != 0u64;
                            if t {
                                break 'b_426c18;
                            } else {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x00426c08);
                                c.g[A0][0] = c.g[S0][0];
                                oc698_callee::OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x00426c08)?;
                                c.g[RA][0] = sx(0x00426c10);
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                oc698_callee::OC698_DELETE_SEMA.call(c, 0x00426c10)?;
                                let t = true;
                                c.g[V0][0] = 0xfffffffffffffffe;
                                if t {
                                    break 'b_426c6c;
                                } else {
                                    break 'b_426c18;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x00426c20);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        oc698_callee::OC698_WAIT_SEMA.call(c, 0x00426c20)?;
                        c.g[RA][0] = sx(0x00426c28);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        oc698_callee::OC698_DELETE_SEMA.call(c, 0x00426c28)?;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_426c6c;
                        } else {
                            break 'b_426c30;
                        }
                    }
                }
                c.cycles += 12;
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A0][0] = 0xffffffff80000000;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[A0][0] | 0x9;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x40;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[RA][0] = sx(0x00426c58);
                c.g[T1][0] = 0u64;
                oc698_callee::OC698_SCE_SIF_SEND_CMD.call(c, 0x00426c58)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_426c6c;
                } else {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00426c68);
                    c.g[A0][0] = c.g[S0][0];
                    oc698_callee::OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x00426c68)?;
                    c.g[V0][0] = 0xfffffffffffffffe;
                    break 'b_426c6c;
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    })
}

/// `int sceSifCallRpc(client *cd, int fno, int mode, void *send, int ssize,
/// void *recv, int rsize, void (*end)(void *), void *end_param)` (0x426d18): call
/// a bound RPC server function. Fills a packet (cid 0x8000000a), writes back the
/// data cache for the send/receive buffers unless mode bit 1 (no write-back), and
/// sends it. Synchronous mode (bit 0 clear) makes a completion sema, waits and
/// deletes it. Returns 0, -1 (no packet), -2 (send failed) or -3 (no sema).
pub fn sce_sif_call_rpc(c: &mut Cpu) {
    guest(c, |c| {
        'b_426ed4: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[FP][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S7][0]);
            c.g[A0][0] = 0x530000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S6][0]);
            c.g[FP][0] = c.g[A2][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S5][0]);
            c.g[S6][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
            c.g[S5][0] = c.g[A3][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
            c.g[S4][0] = c.g[T1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
            c.g[S3][0] = c.g[T2][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.g[S2][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
            c.g[S7][0] = c.g[T3][0];
            c.g[RA][0] = sx(0x00426d70);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2980));
            oc698_callee::OC698_SCE_SIF_RPC_GET_PACKET.call(c, 0x00426d70)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffffff;
            if t {
                break 'b_426ed4;
            } else {
                'b_426e00: {
                    'b_426dfc: {
                        c.cycles += 16;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.g[A0][0] = c.g[FP][0] & 0x2;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                        c.sw((c.g[S1][0] as u32), c.g[S0][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[S7][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S6][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[S2][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S4][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[S3][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S0][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[S1][0]);
                        let t = c.g[A0][0] != 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                        if t {
                            break 'b_426dfc;
                        } else {
                            'b_426ddc: {
                                c.cycles += 2;
                                let t = c.g[S5][0] != c.g[S4][0];
                                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                                if t {
                                    break 'b_426ddc;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[RA][0] = sx(0x00426dd4);
                                    if c.g[V0][0] == 0 {
                                        c.g[A1][0] = c.g[S2][0];
                                    }
                                    oc698_callee::OC698_SCE_SIF_WRITE_BACK_DCACHE
                                        .call(c, 0x00426dd4)?;
                                    let t = true;
                                    c.g[V0][0] = c.g[FP][0] & 0x1;
                                    if t {
                                        break 'b_426e00;
                                    } else {
                                        break 'b_426ddc;
                                    }
                                }
                            }
                            'b_426dec: {
                                c.cycles += 2;
                                let t = (c.g[S2][0] as i64) <= 0;
                                c.g[A0][0] = c.g[S5][0];
                                if t {
                                    break 'b_426dec;
                                } else {
                                    c.cycles += 2;
                                    c.g[RA][0] = sx(0x00426dec);
                                    c.g[A1][0] = c.g[S2][0];
                                    oc698_callee::OC698_SCE_SIF_WRITE_BACK_DCACHE
                                        .call(c, 0x00426dec)?;
                                    break 'b_426dec;
                                }
                            }
                            c.cycles += 2;
                            let t = (c.g[S3][0] as i64) <= 0;
                            c.g[A0][0] = c.g[S4][0];
                            if t {
                                break 'b_426dfc;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x00426dfc);
                                c.g[A1][0] = c.g[S3][0];
                                oc698_callee::OC698_SCE_SIF_WRITE_BACK_DCACHE
                                    .call(c, 0x00426dfc)?;
                                break 'b_426dfc;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[FP][0] & 0x1;
                    break 'b_426e00;
                }
                'b_426ec0: {
                    'b_426eb0: {
                        'b_426e54: {
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.g[S3][0] = 0x1;
                                break 'b_426e54;
                            } else {
                                'b_426e1c: {
                                    'b_426e18: {
                                        c.cycles += 2;
                                        let t = c.g[S7][0] != 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_426e18;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), 0u64);
                                            if t {
                                                break 'b_426e1c;
                                            } else {
                                                break 'b_426e18;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                                    break 'b_426e1c;
                                }
                                c.cycles += 12;
                                c.g[V0][0] = 0xffffffffffffffff;
                                c.g[A0][0] = 0xffffffff80000000;
                                c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                c.g[A3][0] = c.g[S5][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                c.g[T1][0] = c.g[S2][0];
                                c.g[A0][0] = c.g[A0][0] | 0xa;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00426e44);
                                c.g[A2][0] = 0x40;
                                oc698_callee::OC698_SCE_SIF_SEND_CMD.call(c, 0x00426e44)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_426ed4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    if t {
                                        break 'b_426eb0;
                                    } else {
                                        break 'b_426e54;
                                    }
                                }
                            }
                        }
                        'b_426e7c: {
                            c.cycles += 6;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                            c.g[RA][0] = sx(0x00426e64);
                            c.g[A0][0] = c.g[SP][0];
                            oc698_callee::OC698_CREATE_SEMA.call(c, 0x00426e64)?;
                            let t = (c.g[V0][0] as i64) >= 0;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            if t {
                                break 'b_426e7c;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00426e74);
                                c.g[A0][0] = c.g[S0][0];
                                oc698_callee::OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x00426e74)?;
                                let t = true;
                                c.g[V0][0] = 0xfffffffffffffffd;
                                if t {
                                    break 'b_426ed4;
                                } else {
                                    break 'b_426e7c;
                                }
                            }
                        }
                        c.cycles += 11;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[S3][0]);
                        c.g[A0][0] = 0xffffffff80000000;
                        c.g[A3][0] = c.g[S5][0];
                        c.g[T1][0] = c.g[S2][0];
                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        c.g[A0][0] = c.g[A0][0] | 0xa;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00426ea0);
                        c.g[A2][0] = 0x40;
                        oc698_callee::OC698_SCE_SIF_SEND_CMD.call(c, 0x00426ea0)?;
                        let t = c.g[V0][0] != 0u64;
                        if t {
                            break 'b_426ec0;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x00426eb0);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            oc698_callee::OC698_DELETE_SEMA.call(c, 0x00426eb0)?;
                            break 'b_426eb0;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00426eb8);
                    c.g[A0][0] = c.g[S0][0];
                    oc698_callee::OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x00426eb8)?;
                    let t = true;
                    c.g[V0][0] = 0xfffffffffffffffe;
                    if t {
                        break 'b_426ed4;
                    } else {
                        break 'b_426ec0;
                    }
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x00426ec8);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                oc698_callee::OC698_WAIT_SEMA.call(c, 0x00426ec8)?;
                c.g[RA][0] = sx(0x00426ed0);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                oc698_callee::OC698_DELETE_SEMA.call(c, 0x00426ed0)?;
                c.g[V0][0] = 0u64;
                break 'b_426ed4;
            }
        }
        c.cycles += 12;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    })
}

/// `int sceSifCheckStatRpc(client *cd)` (0x426f08): 1 when the client's packet is
/// still ours (sequence number `[cd+4] == [pkt+0x18]`) and in use (`[pkt+0x10] & 1`),
/// else 0.
pub fn sce_sif_check_stat_rpc(c: &mut Cpu) {
    'b_426f34: {
        c.cycles += 3;
        c.g[A1][0] = c.lw((c.g[A0][0] as u32));
        let t = c.g[A1][0] == 0u64;
        if t {
            break 'b_426f34;
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
            let t = c.g[V1][0] != c.g[V0][0];
            if t {
                break 'b_426f34;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] != 0u64;
                if t {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = 0x1;
                    c.pc = t;
                    return;
                } else {
                    break 'b_426f34;
                }
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `void sceFs_FdSemaInit(void)` (0x427438): while `[0x455218] == -1`, create the
/// two file-descriptor semaphores (initial count 1) into `[0x455218]`/`[0x45521c]`.
pub fn sce_fs_fd_sema_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_427488: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V1][0] = 0xffffffffffffffff;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.g[S0][0] = 0x450000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5218));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_427488;
            } else {
                c.cycles += 12;
                c.g[V0][0] = 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x00427470);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                oc698_callee::OC698_CREATE_SEMA.call(c, 0x00427470)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5218), c.g[V0][0]);
                c.g[RA][0] = sx(0x0042747c);
                c.g[A0][0] = c.g[SP][0];
                oc698_callee::OC698_CREATE_SEMA.call(c, 0x0042747c)?;
                c.g[V1][0] = 0x450000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x521c), c.g[V0][0]);
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_427488;
            }
        }
        c.cycles += 3;
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `void sceFs_SemaInit(void)` (0x427938): while `[0x455214] == -1`, create the
/// file-system semaphore (initial count 1) into `[0x455214]`.
pub fn sce_fs_sema_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_427978: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V1][0] = 0xffffffffffffffff;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
            c.g[S0][0] = 0x450000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5214));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_427978;
            } else {
                c.cycles += 8;
                c.g[V0][0] = 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x00427970);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                oc698_callee::OC698_CREATE_SEMA.call(c, 0x00427970)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5214), c.g[V0][0]);
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_427978;
            }
        }
        c.cycles += 3;
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::DATA;
    use crate::game::lib::sif::tests::{
        mrpc_setup, packet_slots, RPC_OWNER, RPC_PACKET, RPC_REQUEST, SIF_CLIENT, SIF_PACKET,
        SIF_RPC_POOL, SIF_RPC_POOL_COUNT,
    };

    /// Gate words of this module's fakes (in the machine's RAM).
    const OC698_WAIT_GATE: u32 = DATA + 0x7200;
    const OC698_DMA_GATE: u32 = DATA + 0x7204;
    const OC698_SEMA_FAIL_GATE: u32 = DATA + 0x7208;
    const OC698_SEMA_FAIL_SKIP: u32 = DATA + 0x720c;
    const OC698_GETREG_GATE: u32 = DATA + 0x7210;
    const OC698_GETREG_VALUE: u32 = DATA + 0x7214;
    const OC698_GATE_MAGIC: u32 = 0x4f43_3639;

    /// `WaitSema` with its gate set: the sema is already signalled (the IOP
    /// answered at once), so the wait returns its id without blocking.
    fn oc698_fake_wait_sema(c: &mut Cpu) {
        if c.bus.read32(OC698_WAIT_GATE) != OC698_GATE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(oc698_fake_wait_sema),
                ee::s15::f_00423de0__generated,
            );
        }
        c.g[V0][0] = sx(c.g[A0][0] as u32);
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// `sceSifSetDma` with its gate set: the DMA queue is full, so it returns
    /// 0 and nothing is sent.
    fn oc698_fake_set_dma(c: &mut Cpu) {
        if c.bus.read32(OC698_DMA_GATE) != OC698_GATE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(oc698_fake_set_dma),
                ee::s15::f_00424160__generated,
            );
        }
        c.g[V0][0] = 0;
        c.cycles += 4;
        crate::abi::ret(c);
    }

    /// Run on the main guest thread (a test machine has none), so the kernel
    /// calls that reschedule (`DeleteSema`) come back to the caller.
    fn oc698_on_thread(c: &mut Cpu) {
        let saved = (c.pc, c.g[A0], c.g[A1], c.g[RA]);
        ssx3_ps2::kernel::start_main_thread(c, saved.0, 0, 0);
        (c.pc, c.g[A0], c.g[A1], c.g[RA]) = saved;
    }

    /// Hook both fakes, open the gate at `gate`, and run on the main guest
    /// thread.
    fn oc698_answered_on_thread(c: &mut Cpu, gate: u32) {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00423DE0),
            oc698_fake_wait_sema,
            crate::difftest::fakes::prev_cell(oc698_fake_wait_sema),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00424160),
            oc698_fake_set_dma,
            crate::difftest::fakes::prev_cell(oc698_fake_set_dma),
        );
        c.bus.write32(gate, OC698_GATE_MAGIC);
        oc698_on_thread(c);
    }

    /// The syscall hook with its gates set: after `OC698_SEMA_FAIL_SKIP` more
    /// `CreateSema` calls (syscall 0x40), one fails with -1; `sceSifGetReg`
    /// (syscall 0x7a) returns the word at `OC698_GETREG_VALUE`; every other
    /// syscall goes to the machine's hook.
    fn oc698_syscall(c: &mut Cpu, at: u32) {
        if c.bus.read32(OC698_SEMA_FAIL_GATE) == OC698_GATE_MAGIC && c.g[V1][0] as u32 == 0x40 {
            let skip = c.bus.read32(OC698_SEMA_FAIL_SKIP);
            c.bus.write32(OC698_SEMA_FAIL_SKIP, skip.wrapping_sub(1));
            if skip == 0 {
                c.g[V0][0] = u64::MAX;
                return;
            }
        }
        if c.bus.read32(OC698_GETREG_GATE) == OC698_GATE_MAGIC && c.g[V1][0] as u32 == 0x7a {
            c.g[V0][0] = sx(c.bus.read32(OC698_GETREG_VALUE));
            return;
        }
        (ssx3_ps2::platform::HOOKS.syscall)(c, at)
    }

    /// Make the `CreateSema` after `skip` successful ones fail, on the main
    /// guest thread.
    fn oc698_create_sema_fails_after(c: &mut Cpu, skip: u32) {
        c.hooks.syscall = oc698_syscall;
        c.bus.write32(OC698_SEMA_FAIL_GATE, OC698_GATE_MAGIC);
        c.bus.write32(OC698_SEMA_FAIL_SKIP, skip);
        oc698_on_thread(c);
    }

    /// One free packet in the SIF RPC pool (`{total, packets, count}`).
    fn oc698_one_free_packet(c: &mut Cpu, total: u32) {
        c.bus.write32(SIF_RPC_POOL, total);
        c.bus.write32(SIF_RPC_POOL + 4, SIF_PACKET);
        c.bus.write32(SIF_RPC_POOL_COUNT, 1);
        packet_slots(c, SIF_PACKET, 1, &|_| false);
    }

    /// Zero the client struct at `SIF_CLIENT`.
    fn oc698_clear_client(c: &mut Cpu) {
        for i in 0..16 {
            c.bus.write32(SIF_CLIENT + i * 4, 0);
        }
    }

    /// Handler tables for user (cid >= 0) and system (cid < 0) commands; the
    /// slot is written at `cid * 8` (the system cid's high bits shift out).
    #[test]
    fn sce_sif_add_cmd_handler_matches() {
        check(
            ee::s15::f_00426020__generated,
            sce_sif_add_cmd_handler,
            200,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0042_6020);
                c.pc = 0x0042_6020;
                c.bus.write32(0x0052_bce4, DATA + 0x1000);
                c.bus.write32(0x0052_bcec, DATA + 0x1200);
                let cid = (r.next_u32() % 32) as u64;
                c.g[A0][0] = if seed % 2 == 0 {
                    cid
                } else {
                    0xffff_ffff_8000_0000 | cid
                };
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    /// Packet size inside (0x10..=0x70) and outside the range (returns 0
    /// without sending); with or without a receive buffer to write back; the
    /// DMA queue free (real `sceSifSetDma`) or full (fake, on the guest
    /// thread).
    #[test]
    fn sce_sif_send_cmd_matches() {
        check(
            ee::s15::f_004261b0__generated,
            sce_sif_send_cmd,
            216,
            &|c, seed| {
                mrpc_setup(c, seed, 0x0042_61b0);
                c.pc = 0x0042_61b0;
                c.bus.write32(0x0052_bce0, 0x1234);
                c.g[A0][0] = 0xffff_ffff_8000_0000 | (seed % 16) as u64;
                c.g[A1][0] = SIF_PACKET as u64;
                c.g[A2][0] = [0x40u64, 0x10, 0x70, 0x71, 0x0f, 0x200][(seed % 6) as usize];
                c.g[A3][0] = (DATA + 0x500) as u64;
                c.g[T0][0] = (DATA + 0x600) as u64;
                c.g[T1][0] = [0u64, 0x20, 0x80][(seed / 6 % 3) as usize];
                if seed / 18 % 2 == 1 {
                    oc698_answered_on_thread(c, OC698_DMA_GATE);
                }
            },
        );
    }

    /// Line counts from zero (size <= 0) over the remainder loop only, the
    /// 8-line block loop only, and both; aligned and unaligned starts.
    #[test]
    fn sce_sif_write_back_d_cache_matches() {
        check(
            ee::s15::f_00426358__generated,
            sce_sif_write_back_d_cache,
            300,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0042_6358);
                c.pc = 0x0042_6358;
                c.g[A0][0] = (DATA + r.next_u32() % 0x100) as u64;
                c.g[A1][0] = match seed % 12 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_fff0,
                    2 => 1,
                    3 => 0x40,
                    4 => 0x41,
                    5 => 0x1ff,
                    6 => 0x200,
                    7 => 0x201,
                    8 => 0x400,
                    9 => 0x1000,
                    10 => 0x1234,
                    _ => (r.next_u32() % 0x4000) as u64,
                };
            },
        );
    }

    driven!(oc698_write_back_gen_driven, ee::s15::f_00426358__generated);
    driven!(oc698_write_back_hand_driven, sce_sif_write_back_d_cache);

    /// Long write-backs with an event due in the 8-line block loop.
    #[test]
    fn sce_sif_write_back_d_cache_events() {
        check_events(
            oc698_write_back_gen_driven,
            oc698_write_back_hand_driven,
            144,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0042_638c);
                c.g[A0][0] = (DATA + r.next_u32() % 0x100) as u64;
                c.g[A1][0] = (0x2000 + r.next_u32() % 0x8000) as u64;
                schedule(c, &mut r, seed, 0x0042_6358)
            },
        );
    }

    /// Already initialised (init word set: only `EIntr`); the full bring-up
    /// with SIF register 2 already set (return), and with it clear (command
    /// sent, SREG 0 polled until the preset value, then `sceSifSetReg`).

    driven!(oc698_get_packet_gen_driven, ee::s15::f_004265d0__generated);
    driven!(oc698_get_packet_hand_driven, sce_sif_rpc_get_packet);

    /// Empty pool, all-used pool (both return 0), and free slots at several
    /// positions with both pool-total arms.
    #[test]
    fn sce_sif_rpc_get_packet_matches() {
        check(
            ee::s15::f_004265d0__generated,
            sce_sif_rpc_get_packet,
            200,
            &|c, seed| {
                mrpc_setup(c, seed, 0x0042_65d0);
                c.pc = 0x0042_65d0;
                let (total, base, count, used) = match seed % 6 {
                    0 => (0u32, 0u32, 0u32, 0u32),
                    1 => (0, SIF_PACKET, 1, 1),
                    2 => (5, SIF_PACKET, 2, 2),
                    3 => (0, SIF_PACKET, 3, 0),
                    4 => (5, SIF_PACKET, 3, 1),
                    _ => (0, SIF_PACKET, 3, 2),
                };
                c.bus.write32(DATA + 0x200, total);
                c.bus.write32(DATA + 0x204, base);
                c.bus.write32(DATA + 0x208, count);
                packet_slots(c, SIF_PACKET, 3, &|i| i < used);
                c.g[A0][0] = (DATA + 0x200) as u64;
            },
        );
    }

    /// Header `{total, base, count}` of the long packet pool.
    const OC698_LONG_POOL: u32 = DATA + 0x1204;

    /// Slots (stride 0x40) of the long packet pool.
    const OC698_LONG_POOL_SLOTS: u32 = DATA + 0x1440;

    /// Taking a packet from a pool whose leading slots are all busy: the
    /// scan steps over them with a live event, then takes the first free slot
    /// or, none free, returns 0.
    #[test]
    fn sce_sif_rpc_get_packet_events() {
        check_events(
            oc698_get_packet_gen_driven,
            oc698_get_packet_hand_driven,
            144,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0042_6600);
                let count = 8 + 4 * (seed / 3 % 4);
                let busy = count - seed / 3 / 4 % 3;
                let total = if seed / 3 / 12 % 2 == 0 {
                    0
                } else {
                    0x0011_0005
                };
                c.bus.write32(OC698_LONG_POOL, total);
                c.bus.write32(OC698_LONG_POOL + 4, OC698_LONG_POOL_SLOTS);
                c.bus.write32(OC698_LONG_POOL + 8, count);
                for i in 0..count {
                    let slot = OC698_LONG_POOL_SLOTS + i * 0x40;
                    for w in 0..16u32 {
                        c.bus.write32(slot + w * 4, r.next_u32() | 0x0101_0101);
                    }
                    let flags = c.bus.read32(slot + 0x10);
                    c.bus.write32(
                        slot + 0x10,
                        if i < busy { flags | 1 } else { flags & !1 | 2 },
                    );
                }
                c.g[A0][0] = OC698_LONG_POOL as u64;
                schedule(c, &mut r, seed, 0x0042_65d0)
            },
        );
    }

    /// Random flag/owner words: clears the owner word and masks the flags
    /// word down.
    #[test]
    fn sce_sif_rpc_free_packet_matches() {
        check(
            ee::s15::f_00426678__generated,
            sce_sif_rpc_free_packet,
            200,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0042_6678);
                c.pc = 0x0042_6678;
                c.g[A0][0] = RPC_PACKET as u64;
                c.bus.write32(RPC_PACKET + 0x10, r.next_u32());
                c.bus.write32(RPC_PACKET + 0x18, r.next_u32());
            },
        );
    }

    /// Every command id (call done with and without an end callback, bind
    /// done, a lower id, a higher one, a small positive one), with the
    /// owner's sema negative (skipped) and live-bogus (signalled, fails
    /// fast).
    #[test]
    fn sce_sif_rpc_request_end_matches() {
        check(
            ee::s15::f_00426708__generated,
            sce_sif_rpc_request_end,
            200,
            &|c, seed| {
                mrpc_setup(c, seed, 0x0042_6708);
                c.pc = 0x0042_6708;
                let cmd = [
                    0x8000_000au32,
                    0x8000_000a,
                    0x8000_0009,
                    0x8000_0008,
                    0x8000_000b,
                    0x5,
                    0x8000_0000,
                    0x8000_0009,
                ][seed as usize % 8];
                c.bus.write32(RPC_REQUEST + 0x20, cmd);
                c.bus.write32(RPC_REQUEST + 0x1c, RPC_OWNER);
                c.bus.write32(RPC_REQUEST + 0x24, 0xaaaa_0001);
                c.bus.write32(RPC_REQUEST + 0x28, 0xbbbb_0002);
                c.bus.write32(RPC_REQUEST + 0x2c, 0xcccc_0003);
                for w in [0x0u32, 0x8, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
                    c.bus.write32(RPC_OWNER + w, 0);
                }
                c.bus.write32(RPC_OWNER, RPC_PACKET);
                c.bus.write32(
                    RPC_OWNER + 8,
                    if seed % 3 == 2 { 0xffff_ffff } else { 0x1234 },
                );
                c.bus.write32(
                    RPC_OWNER + 0x1c,
                    if seed % 4 == 1 { 0x0014_06c0 } else { 0 },
                );
                c.bus.write32(RPC_OWNER + 0x20, (DATA + 0x600) as u32);
                for w in [0x10u32, 0x18] {
                    c.bus.write32(RPC_PACKET + w, 0);
                }
                c.g[A0][0] = RPC_REQUEST as u64;
            },
        );
    }

    /// Bind: no packet (-1); a free packet, in synchronous mode (sema made;
    /// the wait blocks with no IOP answering, or on the guest thread returns
    /// at once (the IOP answered: 0), or the send fails (-2)) and in
    /// asynchronous mode (mode bit 0: no sema, send only); the sema's
    /// `CreateSema` failing (-3, the packet freed).
    #[test]
    fn sce_sif_bind_rpc_matches() {
        check(
            ee::s15::f_00426b48__generated,
            sce_sif_bind_rpc,
            192,
            &|c, seed| {
                mrpc_setup(c, seed, 0x0042_6b48);
                c.pc = 0x0042_6b48;
                oc698_clear_client(c);
                c.g[A0][0] = SIF_CLIENT as u64;
                c.g[A1][0] = 0x2222;
                c.g[A2][0] = (seed / 4 % 2) as u64;
                if seed % 4 != 0 {
                    oc698_one_free_packet(c, seed % 2);
                    match seed / 8 % 3 {
                        1 => oc698_answered_on_thread(c, OC698_WAIT_GATE),
                        2 => oc698_answered_on_thread(c, OC698_DMA_GATE),
                        _ => {}
                    }
                } else {
                    oc698_on_thread(c);
                }
                if seed >= 120 {
                    oc698_create_sema_fails_after(c, 0);
                }
            },
        );
    }

    /// Call: no packet (-1); a free packet with write-back of the send and
    /// receive buffers (distinct or the same buffer, sizes zero or not) or
    /// none (mode bit 1); synchronous (sema, wait, delete) and asynchronous
    /// (mode bit 0, with or without an end callback) sends; the send failing
    /// (-2) and `CreateSema` failing (-3).
    #[test]
    fn sce_sif_call_rpc_matches() {
        check(
            ee::s15::f_00426d18__generated,
            sce_sif_call_rpc,
            384,
            &|c, seed| {
                mrpc_setup(c, seed, 0x0042_6d18);
                c.pc = 0x0042_6d18;
                oc698_clear_client(c);
                c.bus.write32(SIF_CLIENT + 0x14, (DATA + 0x80) as u32);
                c.g[A0][0] = SIF_CLIENT as u64;
                c.g[A1][0] = 0x77;
                c.g[A2][0] = (seed % 4) as u64;
                // Ninth argument (end_param) on the caller's stack.
                c.bus.write32(crate::difftest::SP, (DATA + 0x300) as u32);
                let (send, recv) = [
                    (DATA + 0x1400, DATA + 0x1500),
                    (DATA + 0x1440, DATA + 0x1440),
                    (DATA + 0x1400, DATA + 0x1500),
                    (DATA + 0x1480, DATA + 0x1480),
                ][(seed / 4 % 4) as usize];
                let (ssize, rsize) = [
                    (0x80u32, 0x40u32),
                    (0x40, 0x80),
                    (0, 0),
                    (0x80, 0),
                    (0, 0x40),
                ][(seed / 16 % 5) as usize];
                c.g[A3][0] = send as u64;
                c.g[T0][0] = ssize as u64;
                c.g[T1][0] = recv as u64;
                c.g[T2][0] = rsize as u64;
                c.g[T3][0] = (seed / 80 % 2 * 0x0041_2345) as u64;
                if seed / 2 % 3 != 0 {
                    oc698_one_free_packet(c, seed / 80 % 2);
                    match seed / 240 + seed / 8 % 3 {
                        1 => oc698_answered_on_thread(c, OC698_WAIT_GATE),
                        2 => oc698_answered_on_thread(c, OC698_DMA_GATE),
                        3 => oc698_create_sema_fails_after(c, 0),
                        _ => {}
                    }
                }
            },
        );
    }

    /// No packet, another call's packet (sequence mismatch), a matching
    /// packet not in use, and a matching packet in use.
    #[test]
    fn sce_sif_check_stat_rpc_matches() {
        check(
            ee::s15::f_00426f08__generated,
            sce_sif_check_stat_rpc,
            120,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0042_6f08);
                c.pc = 0x0042_6f08;
                oc698_clear_client(c);
                let seq = r.next_u32();
                c.bus
                    .write32(SIF_CLIENT, if seed % 4 == 0 { 0 } else { SIF_PACKET });
                c.bus
                    .write32(SIF_CLIENT + 4, if seed % 4 == 1 { seq ^ 1 } else { seq });
                c.bus.write32(SIF_PACKET + 0x18, seq);
                c.bus.write32(
                    SIF_PACKET + 0x10,
                    r.next_u32() & !1 | (seed % 4 == 3) as u32,
                );
                c.g[A0][0] = SIF_CLIENT as u64;
            },
        );
    }

    /// Descriptor semas not yet made (`[0x455218] == -1`) or already made;
    /// the second `CreateSema` also failing (-1 stored).
    #[test]
    fn sce_fs_fd_sema_init_matches() {
        check(
            ee::s15::f_00427438__generated,
            sce_fs_fd_sema_init,
            120,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0042_7438);
                c.pc = 0x0042_7438;
                c.bus.write32(
                    0x0045_5218,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32()
                    },
                );
                c.bus.write32(0x0045_521c, r.next_u32());
                if seed % 3 == 1 {
                    oc698_create_sema_fails_after(c, seed / 3 % 2);
                }
            },
        );
    }

    /// The file-system sema not yet made (`[0x455214] == -1`) or made; its
    /// `CreateSema` failing.
    #[test]
    fn sce_fs_sema_init_matches() {
        check(
            ee::s15::f_00427938__generated,
            sce_fs_sema_init,
            120,
            &|c, seed| {
                let mut r = mrpc_setup(c, seed, 0x0042_7938);
                c.pc = 0x0042_7938;
                c.bus.write32(
                    0x0045_5214,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32()
                    },
                );
                if seed % 3 == 1 {
                    oc698_create_sema_fails_after(c, 0);
                }
            },
        );
    }
}
