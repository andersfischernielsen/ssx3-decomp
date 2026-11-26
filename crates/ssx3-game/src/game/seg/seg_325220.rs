//! `src/seg/seg_325220.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::lib::kernel::do_syscall;
use crate::game::lib::kernel::event_if_due;
use crate::game::lib::kernel::DINTR;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0042_5cc0,
        sce_sif_cmd_set_sreg_handler,
        "sceSifCmd_SetSregHandler",
    ),
    (0x0042_5cf0, sce_sif_get_sreg, "sceSifGetSreg"),
    (0x0042_5ff0, sce_sif_set_cmd_buffer, "sceSifSetCmdBuffer"),
    (0x0042_6020, sce_sif_add_cmd_handler, "sceSifAddCmdHandler"),
    (0x0042_61b0, sce_sif_send_cmd, "sceSifSendCmd"),
    (
        0x0042_6358,
        sce_sif_write_back_d_cache,
        "sceSifWriteBackDCache",
    ),
    (0x0042_65d0, sce_sif_rpc_get_packet, "sceSifRpc_GetPacket"),
    (0x0042_6678, sce_sif_rpc_free_packet, "sceSifRpc_FreePacket"),
    (0x0042_6708, sce_sif_rpc_request_end, "sceSifRpc_RequestEnd"),
    (0x0042_6b48, sce_sif_bind_rpc, "sceSifBindRpc"),
    (0x0042_6d18, sce_sif_call_rpc, "sceSifCallRpc"),
    (0x0042_6f08, sce_sif_check_stat_rpc, "sceSifCheckStatRpc"),
    (0x0042_7438, sce_fs_fd_sema_init, "sceFs_FdSemaInit"),
    (0x0042_7938, sce_fs_sema_init, "sceFs_SemaInit"),
    (0x0042_7988, sce_fs_lock, "sceFs_Lock"),
    (0x0042_79b8, sce_fs_unlock, "sceFs_Unlock"),
    (0x0042_7c98, sce_fs_needs_rebind, "sceFs_NeedsRebind"),
    (0x0042_7d60, sce_open, "sceOpen"),
    (0x0042_ab18, sce_sif_init_iop_heap, "sceSifInitIopHeap"),
    (0x0042_aba0, sce_sif_alloc_iop_heap, "sceSifAllocIopHeap"),
    (0x0042_4950, disable_dmac_wrapper, "DisableDmac"),
    (0x0042_49b8, enable_dmac_wrapper, "EnableDmac"),
    (0x0042_4c50, i_wakeup_thread_signal, "iWakeupThread"),
    (0x0042_7a90, sce_fs_init, "sceFsInit"),
];

/// Direct-call targets of this module (address, generated function).
pub(crate) mod oc698_callee {
    use super::ee;
    use super::Callee;

    /// `_sceSifSendCmd` (`0x00426078`).
    pub(crate) const OC698_U_SCE_SIF_SEND_CMD: Callee = Callee {
        addr: 0x00426078,
        f: ee::s15::f_00426078,
    };

    /// `DIntr` (`0x0042c078`).
    pub(crate) const OC698_DINTR: Callee = Callee {
        addr: 0x0042c078,
        f: ee::s15::f_0042c078,
    };

    /// `EIntr` (`0x0042c0c0`).
    pub(crate) const OC698_EINTR: Callee = Callee {
        addr: 0x0042c0c0,
        f: ee::s15::f_0042c0c0,
    };

    /// `sceSifInitCmd` (`0x00425d38`).
    #[allow(dead_code)]
    pub(crate) const OC698_SCE_SIF_INIT_CMD: Callee = Callee {
        addr: 0x00425d38,
        f: ee::s15::f_00425d38,
    };

    /// `sceSifAddCmdHandler` (`0x00426020`).
    #[allow(dead_code)]
    pub(crate) const OC698_SCE_SIF_ADD_CMD_HANDLER: Callee = Callee {
        addr: 0x00426020,
        f: ee::s15::f_00426020,
    };

    /// `sceSifGetReg` (`0x004241b0`).
    #[allow(dead_code)]
    pub(crate) const OC698_SCE_SIF_GET_REG: Callee = Callee {
        addr: 0x004241b0,
        f: ee::s15::f_004241b0,
    };

    /// `sceSifSendCmd` (`0x004261b0`).
    pub(crate) const OC698_SCE_SIF_SEND_CMD: Callee = Callee {
        addr: 0x004261b0,
        f: ee::s15::f_004261b0,
    };

    /// `sceSifGetSreg` (`0x00425cf0`).
    #[allow(dead_code)]
    pub(crate) const OC698_SCE_SIF_GET_SREG: Callee = Callee {
        addr: 0x00425cf0,
        f: ee::s15::f_00425cf0,
    };

    /// `sceSifSetReg` (`0x004241a0`).
    #[allow(dead_code)]
    pub(crate) const OC698_SCE_SIF_SET_REG: Callee = Callee {
        addr: 0x004241a0,
        f: ee::s15::f_004241a0,
    };

    /// `iSignalSema` (`0x00423dd0`).
    pub(crate) const OC698_I_SIGNAL_SEMA: Callee = Callee {
        addr: 0x00423dd0,
        f: ee::s15::f_00423dd0,
    };

    /// `sceSifRpc_FreePacket` (`0x00426678`).
    pub(crate) const OC698_SCE_SIF_RPC_FREE_PACKET: Callee = Callee {
        addr: 0x00426678,
        f: ee::s15::f_00426678,
    };

    /// `sceSifRpc_GetPacket` (`0x004265d0`).
    pub(crate) const OC698_SCE_SIF_RPC_GET_PACKET: Callee = Callee {
        addr: 0x004265d0,
        f: ee::s15::f_004265d0,
    };

    /// `CreateSema` (`0x00423da0`).
    pub(crate) const OC698_CREATE_SEMA: Callee = Callee {
        addr: 0x00423da0,
        f: ee::s15::f_00423da0,
    };

    /// `DeleteSema` (`0x00423db0`).
    pub(crate) const OC698_DELETE_SEMA: Callee = Callee {
        addr: 0x00423db0,
        f: ee::s15::f_00423db0,
    };

    /// `WaitSema` (`0x00423de0`).
    pub(crate) const OC698_WAIT_SEMA: Callee = Callee {
        addr: 0x00423de0,
        f: ee::s15::f_00423de0,
    };

    /// `sceSifWriteBackDCache` (`0x00426358`).
    pub(crate) const OC698_SCE_SIF_WRITE_BACK_DCACHE: Callee = Callee {
        addr: 0x00426358,
        f: ee::s15::f_00426358,
    };
}

/// `void sceSifCmd_SetSregHandler(cmd, mgr)` (0x425cc0): the SIF "set SREG"
/// command's handler. `cmd` holds the register index (`+0x10`) and the value
/// (`+0x14`); the value goes to `mgr`'s SREG table (pointer at `+0x1c`).
pub fn sce_sif_cmd_set_sreg_handler(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    // The store sits in the return's delay slot.
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    ret(c);
}

/// The SIF SREG table (`sceSifGetSreg`): 32-bit registers at `0x52be00`.
pub(crate) const SIF_SREG_TABLE: u32 = 0x0052_be00;

/// `u32 sceSifGetSreg(int n)` (0x425cf0): SIF software register `n` (the
/// word the IOP set with the SREG command).
pub fn sce_sif_get_sreg(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x530000;
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx(SIF_SREG_TABLE);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
    ret(c);
}

/// The SIF command state: the receive buffer pointer at `+0x14` and its size
/// at `+0x18` (`sceSifSetCmdBuffer`).
pub(crate) const SIF_CMD_STATE: u32 = 0x0052_bcd8;

/// `void *sceSifSetCmdBuffer(void *buf, int size)` (0x425ff0): install the
/// SIF command receive buffer, returning the previous buffer pointer.
pub fn sce_sif_set_cmd_buffer(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = sx(SIF_CMD_STATE);
    c.g[V0][0] = c.lw(SIF_CMD_STATE + 0x14);
    c.sw(SIF_CMD_STATE + 0x18, c.g[A1][0]);
    // The buffer store sits in the return's delay slot.
    c.sw(SIF_CMD_STATE + 0x14, c.g[A0][0]);
    ret(c);
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
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
    let t = c.g[RA][0] as u32;
    c.sw(c.g[V1][0] as u32, c.g[A1][0]);
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
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = c.g[V1][0];
        c.g[T1][0] = c.g[T3][0];
        c.g[RA][0] = sx(0x004261e0);
        c.g[A1][0] = 0u64;
        oc698_callee::OC698_U_SCE_SIF_SEND_CMD.call(c, 0x004261e0)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
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
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        if t {
            break 'b_4263fc;
        } else {
            'b_4263a8: {
                c.cycles += 9;
                c.g[T0][0] = c.g[A0][0] & c.g[T9][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
                c.g[T1][0] = c.g[T2][0] & c.g[T9][0];
                c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[T0][0] as u32));
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
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
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
                                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                    let t = c.g[V1][0] != c.g[A1][0];
                                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                                    if t {
                                        break 'b_426638;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                                        c.g[V1][0] = 0x1;
                                        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
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
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
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
                        c.sd(c.g[SP][0] as u32, c.g[S0][0]);
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
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        oc698_callee::OC698_SCE_SIF_RPC_FREE_PACKET.call(c, 0x004267a4)?;
        c.sw(c.g[S0][0] as u32, 0u64);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
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
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
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
                        c.sw(c.g[S1][0] as u32, c.g[S0][0]);
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
        c.g[A1][0] = c.lw(c.g[A0][0] as u32);
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

/// `int sceFs_Lock(void)`: make sure the fs semaphore exists, wait on it and return 0.
pub fn sce_fs_lock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x00427998);
        Callee {
            addr: 0x00427938,
            f: ee::s15::sceFs_SemaInit,
        }
        .call(c, 0x00427998)?;
        c.g[V0][0] = 0x450000;
        c.g[RA][0] = sx(0x004279a4);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5214));
        Callee {
            addr: 0x00423de0,
            f: ee::s15::WaitSema,
        }
        .call(c, 0x004279a4)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = 0u64;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void sceFs_Unlock(void)`: signal the fs semaphore (tail call to SignalSema).
pub fn sce_fs_unlock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5214));
        c.pc = 0x00423dc0;
        (ee::s15::SignalSema)(c);
        return Ok(());
    });
}

/// `int sceFs_NeedsRebind(void)`: memcmp the RPC stamp against the two known stamps; 1 when it differs from both (after the IOP reset).
pub fn sce_fs_needs_rebind(c: &mut Cpu) {
    guest(c, |c| {
        'b_427d04: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = 0x450000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S3][0]);
            c.g[V1][0] = 0x530000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5164));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x11d8));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.g[S2][0] = 0u64;
            c.sd(c.g[SP][0] as u32, c.g[S0][0]);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x00427cd4);
            c.g[A2][0] = 0x4;
            Callee {
                addr: 0x00415fc8,
                f: ee::s15::memcmp,
            }
            .call(c, 0x00427cd4)?;
            let t = c.g[V0][0] == 0u64;
            c.g[S0][0] = 0x450000;
            if t {
                break 'b_427d04;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5220));
                c.g[RA][0] = sx(0x00427cec);
                c.g[A2][0] = 0x4;
                Callee {
                    addr: 0x00415fc8,
                    f: ee::s15::memcmp,
                }
                .call(c, 0x00427cec)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5220));
                if t {
                    break 'b_427d04;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00427d00);
                    c.g[A2][0] = 0x4;
                    Callee {
                        addr: 0x00415fc8,
                        f: ee::s15::memcmp,
                    }
                    .call(c, 0x00427d00)?;
                    c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_427d04;
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceOpen(char *path, int flags, ...)`: lock the fs, init on first use, allocate an fd, copy the path (max 1024), create a reply semaphore, call the open RPC and wait for the reply; returns the fd or a negative error.
pub fn sce_open(c: &mut Cpu) {
    guest(c, |c| {
        'b_427dd4: {
            c.cycles += 27;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S0][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[S7][0]);
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[FP][0]);
            c.g[S7][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S1][0]);
            c.g[A0][0] = 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
            c.g[FP][0] = 0x530000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[S6][0]);
            c.g[S1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x2900));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[S5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[S4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[S3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x120), c.g[A2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x128), c.g[A3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[T0][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x138), c.g[T1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x140), c.g[T2][0]);
            c.g[RA][0] = sx(0x00427dbc);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x148), c.g[T3][0]);
            Callee {
                addr: 0x00427988,
                f: ee::s15::sceFs_Lock,
            }
            .call(c, 0x00427dbc)?;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x520c));
            let t = c.g[V0][0] != 0u64;

            if t {
                break 'b_427dd4;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00427dd4);
                Callee {
                    addr: 0x00427a90,
                    f: ee::s15::sceFsInit,
                }
                .call(c, 0x00427dd4)?;
                break 'b_427dd4;
            }
        }
        'b_427fb4: {
            'b_427df8: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00427ddc);
                Callee {
                    addr: 0x00427c98,
                    f: ee::s15::sceFs_NeedsRebind,
                }
                .call(c, 0x00427ddc)?;
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_427df8;
                } else {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x00427dec);
                    Callee {
                        addr: 0x004279b8,
                        f: ee::s15::sceFs_Unlock,
                    }
                    .call(c, 0x00427dec)?;
                    c.g[V0][0] = 0xfffffffffffe0000;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] | 0xfffc;
                    if t {
                        break 'b_427fb4;
                    } else {
                        break 'b_427df8;
                    }
                }
            }
            'b_427e1c: {
                c.cycles += 5;
                c.g[RA][0] = sx(0x00427e00);
                Callee {
                    addr: 0x00427498,
                    f: ee::s15::sceFs_AllocFd,
                }
                .call(c, 0x00427e00)?;
                c.g[S3][0] = c.g[V0][0];
                if c.g[S3][0] != 0u64 {
                    c.g[V1][0] = c.lbu(c.g[S0][0] as u32);
                    break 'b_427e1c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00427e14);
                    Callee {
                        addr: 0x004279b8,
                        f: ee::s15::sceFs_Unlock,
                    }
                    .call(c, 0x00427e14)?;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffed;
                    if t {
                        break 'b_427fb4;
                    } else {
                        break 'b_427e1c;
                    }
                }
            }
            'b_427e88: {
                'b_427e84: {
                    'b_427e74: {
                        c.cycles += 5;
                        c.g[A1][0] = 0u64;
                        c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x120));
                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                        let t = c.g[V0][0] == 0u64;
                        c.sb((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                        if t {
                            break 'b_427e74;
                        } else {
                            c.cycles += 6;
                            c.g[A2][0] = 0x530000;
                            c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[S6][0] = 0x530000;
                            c.g[S4][0] = 0x530000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                            'l_427e48: loop {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A1][0] as i64) < 1024i64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                if t {
                                    break 'b_427e84;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.g[V1][0] = c.lbu(c.g[V0][0] as u32);
                                    c.sb((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
                                    if c.g[V1][0] != 0u64 {
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00427e48);
                                        }
                                        continue 'l_427e48;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0x400;
                                        if t {
                                            break 'b_427e88;
                                        } else {
                                            break 'b_427e74;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A2][0] = 0x530000;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S6][0] = 0x530000;
                    c.g[S4][0] = 0x530000;
                    break 'b_427e84;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x400;
                break 'b_427e88;
            }
            'b_427e90: {
                c.cycles += 2;
                if c.g[A1][0] == c.g[V0][0] {
                    c.sb((c.g[S1][0] as u32).wrapping_add(0x413), 0u64);
                    break 'b_427e90;
                } else {
                    break 'b_427e90;
                }
            }
            'b_427f2c: {
                c.cycles += 33;
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1400));
                c.g[V1][0] = 0x7fff0000;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[S5][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
                c.g[V1][0] = c.g[S7][0] & c.g[V1][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                c.g[V0][0] = 0x1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x414), c.g[S5][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
                c.g[RA][0] = sx(0x00427ed0);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                Callee {
                    addr: 0x00423da0,
                    f: ee::s15::CreateSema,
                }
                .call(c, 0x00427ed0)?;
                c.g[S0][0] = c.g[V0][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S2][0]);
                c.g[V0][0] = 0x4;
                c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1200));
                c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x2900));
                c.g[A1][0] = 0u64;
                c.sw(c.g[SP][0] as u32, 0u64);
                c.g[A2][0] = 0u64;
                c.g[T0][0] = 0x418;
                c.g[T1][0] = c.g[S4][0];
                c.g[T2][0] = 0x4;
                c.g[RA][0] = sx(0x00427f0c);
                c.g[T3][0] = 0u64;
                Callee {
                    addr: 0x00426d18,
                    f: ee::s15::sceSifCallRpc,
                }
                .call(c, 0x00427f0c)?;
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 0x20000000;
                if t {
                    break 'b_427f2c;
                } else {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x00427f1c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00423db0,
                        f: ee::s15::DeleteSema,
                    }
                    .call(c, 0x00427f1c)?;
                    c.g[RA][0] = sx(0x00427f24);
                    Callee {
                        addr: 0x004279b8,
                        f: ee::s15::sceFs_Unlock,
                    }
                    .call(c, 0x00427f24)?;
                    let t = true;
                    c.g[V0][0] = 0xfffffffffffffff5;
                    if t {
                        break 'b_427fb4;
                    } else {
                        break 'b_427f2c;
                    }
                }
            }
            'b_427f50: {
                c.cycles += 5;
                c.g[V0][0] = c.g[S4][0] | c.g[V0][0];
                c.g[RA][0] = sx(0x00427f38);
                c.g[S1][0] = c.lw(c.g[V0][0] as u32);
                Callee {
                    addr: 0x004279b8,
                    f: ee::s15::sceFs_Unlock,
                }
                .call(c, 0x00427f38)?;
                let t = c.g[S1][0] != 0u64;

                if t {
                    break 'b_427f50;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00427f48);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00423db0,
                        f: ee::s15::DeleteSema,
                    }
                    .call(c, 0x00427f48)?;
                    let t = true;
                    c.g[V0][0] = 0xfffffffffffffff5;
                    if t {
                        break 'b_427fb4;
                    } else {
                        break 'b_427f50;
                    }
                }
            }
            'b_427f88: {
                c.cycles += 7;
                c.g[RA][0] = sx(0x00427f58);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x00423de0,
                    f: ee::s15::WaitSema,
                }
                .call(c, 0x00427f58)?;
                c.g[RA][0] = sx(0x00427f60);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x00423db0,
                    f: ee::s15::DeleteSema,
                }
                .call(c, 0x00427f60)?;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                let t = (c.g[V1][0] as i64) >= 0;
                c.g[S0][0] = 0x450000;
                if t {
                    break 'b_427f88;
                } else {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00427f74);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5218));
                    Callee {
                        addr: 0x00423de0,
                        f: ee::s15::WaitSema,
                    }
                    .call(c, 0x00427f74)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[RA][0] = sx(0x00427f80);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5218));
                    Callee {
                        addr: 0x00423dc0,
                        f: ee::s15::SignalSema,
                    }
                    .call(c, 0x00427f80)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_427fb4;
                    } else {
                        break 'b_427f88;
                    }
                }
            }
            c.cycles += 11;
            c.g[S1][0] = c.g[S5][0];
            c.g[RA][0] = sx(0x00427f94);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5218));
            Callee {
                addr: 0x00423de0,
                f: ee::s15::WaitSema,
            }
            .call(c, 0x00427f94)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5218));
            c.g[V1][0] = c.g[V1][0] | c.g[S7][0];
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[RA][0] = sx(0x00427fb0);
            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
            Callee {
                addr: 0x00423dc0,
                f: ee::s15::SignalSema,
            }
            .call(c, 0x00427fb0)?;
            c.g[V0][0] = c.g[S1][0];
            break 'b_427fb4;
        }
        c.cycles += 12;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[FP][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[S7][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceSifInitIopHeap(void)`: bind the IOP heap RPC (spinning until the IOP answers) and clear the heap error word at 0x455224. Returns 0, or -1 when the bind fails.
pub fn sce_sif_init_iop_heap(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0042ab18 => {
                    c.cycles += 6;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.g[S1][0] = 0x530000;
                    let t = true;
                    c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                    if t {
                        pc = 0x0042ab54;
                        continue;
                    }
                    pc = 0x0042ab30;
                    continue;
                }
                0x0042ab30 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x100000;
                    c.g[V1][0] = 0xffffffffffffffff;
                    pc = 0x0042ab38;
                    continue;
                }
                0x0042ab38 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[V1][0];

                    if t {
                        if back_edge_unwinds(c, 0x0042ab38) {
                            return Ok(());
                        }
                        pc = 0x0042ab38;
                        continue;
                    }
                    pc = 0x0042ab54;
                    continue;
                }
                0x0042ab54 => {
                    c.cycles += 8;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1140));
                    c.g[A1][0] = 0xffffffff80000000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[A1][0] | 0x3;
                    c.g[RA][0] = sx(0x0042ab6c);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00426b48,
                        f: ee::s15::sceSifBindRpc,
                    }
                    .call(c, 0x0042ab6c)?;

                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                        pc = 0x0042ab7c;
                        continue;
                    }
                    pc = 0x0042ab74;
                    continue;
                }
                0x0042ab74 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x0042ab8c;
                        continue;
                    }
                    pc = 0x0042ab7c;
                    continue;
                }
                0x0042ab7c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x450000;
                    if t {
                        if back_edge_unwinds(c, 0x0042ab30) {
                            return Ok(());
                        }
                        pc = 0x0042ab30;
                        continue;
                    }
                    pc = 0x0042ab84;
                    continue;
                }
                0x0042ab84 => {
                    c.cycles += 2;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x5224), 0u64);
                    c.g[V0][0] = 0u64;
                    pc = 0x0042ab8c;
                    continue;
                }
                0x0042ab8c => {
                    c.cycles += 5;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0042ab18, 0x0042aba0);
                    return Ok(());
                }
            }
        }
    });
}

/// `int sceSifAllocIopHeap(int size)`: allocate IOP heap memory through the RPC (function 1); 0 when the heap is not initialised or the call fails.
pub fn sce_sif_alloc_iop_heap(c: &mut Cpu) {
    guest(c, |c| {
        'b_42ac00: {
            'b_42abfc: {
                c.cycles += 7;
                c.g[V1][0] = 0x450000;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5224));
                c.g[A1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                let t = (c.g[V0][0] as i64) < 0;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                if t {
                    break 'b_42abfc;
                } else {
                    c.cycles += 16;
                    c.g[A3][0] = 0x530000;
                    c.g[A0][0] = 0x530000;
                    c.sw((c.g[A3][0] as u32).wrapping_sub(0x10c0), c.g[A1][0]);
                    c.g[S0][0] = 0x530000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1140));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x10c0));
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0u64;
                    c.g[T0][0] = 0x4;
                    c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1100));
                    c.g[T2][0] = 0x4;
                    c.g[RA][0] = sx(0x0042abf4);
                    c.g[T3][0] = 0u64;
                    Callee {
                        addr: 0x00426d18,
                        f: ee::s15::sceSifCallRpc,
                    }
                    .call(c, 0x0042abf4)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_sub(0x1100));
                    if t {
                        break 'b_42ac00;
                    } else {
                        break 'b_42abfc;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_42ac00;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `_DisableDmac(int channel)` (0x423b10), the syscall wrapper.
pub(crate) const DISABLE_DMAC_CALL: Callee = Callee {
    addr: 0x0042_3b10,
    f: ee::s15::f_00423b10,
};

/// `_EnableDmac(int channel)` (0x423b00), the syscall wrapper.
pub(crate) const ENABLE_DMAC_CALL: Callee = Callee {
    addr: 0x0042_3b00,
    f: ee::s15::f_00423b00,
};

/// `EIntr()` (0x42c0c0): enable interrupts.
pub(crate) const EINTR_CALL: Callee = Callee {
    addr: 0x0042_c0c0,
    f: ee::s15::f_0042c0c0,
};

/// `_iWakeupThread(int thid)` (0x423ce0).
pub(crate) const I_WAKEUP_THREAD_CALL: Callee = Callee {
    addr: 0x0042_3ce0,
    f: ee::s15::f_00423ce0,
};

/// `iSignalSema(int sema)` (0x423dd0).
pub(crate) const I_SIGNAL_SEMA_CALL: Callee = Callee {
    addr: 0x0042_3dd0,
    f: ee::s15::f_00423dd0,
};

/// Shared shape of `DisableDmac` (0x424950) and `EnableDmac` (0x4249b8):
/// run the syscall wrapper `dmac` with interrupts off. When Status.EIE
/// (COP0 r12 bit 16) was set on entry, `DIntr` turns them off first and
/// `EIntr` turns them back on after the call. The wrapper's result is left
/// in v0. `rets` are the return addresses of the `DIntr`, `dmac` and
/// `EIntr` calls, in that order.
pub(crate) fn dmac_wrapper_with_intr(c: &mut Cpu, dmac: Callee, rets: (u32, u32, u32)) {
    let (intr_off_ret, dmac_ret, intr_on_ret) = rets;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        let at = frame.at(c, 0x10);
        c.sd(at, c.g[S1][0]);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        let at = frame.at(c, 0);
        c.sd(at, c.g[S0][0]);
        c.mfc0(S0, 12);
        c.g[V0][0] = 0x10000;
        c.g[S0][0] = c.g[S0][0] & c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            DINTR.call(c, intr_off_ret)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        dmac.call(c, dmac_ret)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = c.g[S1][0];
        // S0 is read after the call, as the compiled code does.
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            EINTR_CALL.call(c, intr_on_ret)?;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int DisableDmac(int channel)` (0x424950): `_DisableDmac` with the
/// interrupts off (see `dmac_wrapper_with_intr`).
pub fn disable_dmac_wrapper(c: &mut Cpu) {
    dmac_wrapper_with_intr(
        c,
        DISABLE_DMAC_CALL,
        (0x0042_4980, 0x0042_4988, 0x0042_49a0),
    );
}

/// `int EnableDmac(int channel)` (0x4249b8): `_EnableDmac` with the
/// interrupts off (see `dmac_wrapper_with_intr`).
pub fn enable_dmac_wrapper(c: &mut Cpu) {
    dmac_wrapper_with_intr(c, ENABLE_DMAC_CALL, (0x0042_49e8, 0x0042_49f0, 0x0042_4a08));
}

/// `iWakeupThread`-shaped wrapper (0x424c50, the generated name). It makes
/// the `-47` kernel call, which returns an id in v0 (kept in s0). If that id
/// equals the argument, the id is checked against 0x100 and the gate word at
/// 0x455170: when both pass, the id is queued in the 512-entry ring at
/// 0x52b3a8 (counter word at +4; each 2-byte entry at +8 is a zero flag byte
/// then the id), the semaphore whose id sits at 0x52b3a0 is signalled, and v0
/// is the id; otherwise v0 is -1. A different argument goes to
/// `_iWakeupThread` (0x423ce0) and its result is returned. Field meanings
/// are inferred from the generated code (the reference body at this address
/// is an unrelated hash function), so the names are provisional.
pub fn i_wakeup_thread_signal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        let at = frame.at(c, 0);
        c.sd(at, c.g[S0][0]);
        c.g[V1][0] = sx((-47i32) as u32);
        if !do_syscall(c, 0x0042_4c60, 0x0042_4c64) {
            return Err(Unwound);
        }
        event_if_due(c);
        c.g[S0][0] = c.g[V0][0];
        let same = c.g[S0][0] == c.g[A0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x100) as u64;
        if !same {
            c.cycles += 4;
            I_WAKEUP_THREAD_CALL.call(c, 0x0042_4c78)?;
            frame.restore_ra(c, 0x10);
            c.cycles += 3;
            c.g[S0][0] = c.ld(frame.at(c, 0));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let out_of_range = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        let mut queued = false;
        if !out_of_range {
            c.cycles += 3;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5170));
            let gate_set = c.g[V1][0] != 0;
            c.g[V1][0] = 0x530000;
            queued = gate_set;
        }
        if queued {
            c.cycles += 14;
            c.g[A1][0] = 0x530000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4c58));
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x4c60));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] &= 0x1ff;
            c.g[A2][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[V1][0] = c.g[A1][0];
            c.sb((c.g[A1][0] as u32).wrapping_add(0x8), 0u64);
            c.sb((c.g[V1][0] as u32).wrapping_add(0x9), c.g[S0][0]);
            I_SIGNAL_SEMA_CALL.call(c, 0x0042_4cd0)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0x10);
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `SceFsInit` (0x00427a90): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sce_fs_init(c: &mut Cpu) {
    if c.pc != 0x00427a90 {
        return ee::s15::f_00427a90__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00427a90;
        loop {
            match pc {
                0x00427a90 => {
                    c.cycles += 30;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.g[A0][0] = 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
                    c.g[S0][0] = 0x530000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1180));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
                    c.g[RA][0] = sx(0x00427abc);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
                    Callee {
                        addr: 0x00426408,
                        f: ee::s15::sceSifInitRpc,
                    }
                    .call(c, 0x00427abc)?;
                    c.sw((c.g[S0][0] as u32).wrapping_sub(0x1180), 0u64);
                    c.g[RA][0] = sx(0x00427ac8);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                    Callee {
                        addr: 0x0042c078,
                        f: ee::s15::DIntr,
                    }
                    .call(c, 0x00427ac8)?;
                    c.g[A1][0] = 0x420000;
                    c.g[A2][0] = 0x530000;
                    c.g[A0][0] = 0xffffffff80000000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7590));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x11c0));
                    c.g[RA][0] = sx(0x00427ae8);
                    c.g[A0][0] = c.g[A0][0] | 0x11;
                    Callee {
                        addr: 0x00426020,
                        f: ee::s15::sceSifAddCmdHandler,
                    }
                    .call(c, 0x00427ae8)?;
                    c.g[A1][0] = 0x420000;
                    c.g[A0][0] = 0xffffffff80000000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a60));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00427b00);
                    c.g[A0][0] = c.g[A0][0] | 0x13;
                    Callee {
                        addr: 0x00426020,
                        f: ee::s15::sceSifAddCmdHandler,
                    }
                    .call(c, 0x00427b00)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[S3][0] = 0x530000;
                    if t {
                        pc = 0x00427b3c;
                        continue;
                    }
                    pc = 0x00427b08;
                    continue;
                }
                0x00427b08 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00427b10);
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1200));
                    Callee {
                        addr: 0x0042c0c0,
                        f: ee::s15::EIntr,
                    }
                    .call(c, 0x00427b10)?;
                    let t = true;

                    if t {
                        pc = 0x00427b40;
                        continue;
                    }
                    pc = 0x00427b18;
                    continue;
                }
                0x00427b18 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0xffffffffffffffff;
                    pc = 0x00427b20;
                    continue;
                }
                0x00427b20 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[V1][0];

                    if t {
                        if back_edge_unwinds(c, 0x00427b20) {
                            return Ok(());
                        }
                        pc = 0x00427b20;
                        continue;
                    }
                    pc = 0x00427b3c;
                    continue;
                }
                0x00427b3c => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1200));
                    pc = 0x00427b40;
                    continue;
                }
                0x00427b40 => {
                    c.cycles += 7;
                    c.g[A1][0] = 0xffffffff80000000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[A1][0] | 0x1;
                    c.g[RA][0] = sx(0x00427b54);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00426b48,
                        f: ee::s15::sceSifBindRpc,
                    }
                    .call(c, 0x00427b54)?;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x00427c74;
                        continue;
                    }
                    pc = 0x00427b5c;
                    continue;
                }
                0x00427b5c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x100000;
                    if t {
                        if back_edge_unwinds(c, 0x00427b18) {
                            return Ok(());
                        }
                        pc = 0x00427b18;
                        continue;
                    }
                    pc = 0x00427b68;
                    continue;
                }
                0x00427b68 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00427b70);
                    c.g[S4][0] = 0x450000;
                    Callee {
                        addr: 0x00427438,
                        f: ee::s15::sceFs_FdSemaInit,
                    }
                    .call(c, 0x00427b70)?;
                    c.g[RA][0] = sx(0x00427b78);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                    Callee {
                        addr: 0x00423de0,
                        f: ee::s15::WaitSema,
                    }
                    .call(c, 0x00427b78)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1400));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x200));
                    c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0x530000;
                    if t {
                        pc = 0x00427bbc;
                        continue;
                    }
                    pc = 0x00427b90;
                    continue;
                }
                0x00427b90 => {
                    c.cycles += 2;
                    c.g[S1][0] = 0x530000;
                    c.g[S0][0] = 0x530000;
                    pc = 0x00427b98;
                    continue;
                }
                0x00427b98 => {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x00427b98) {
                            return Ok(());
                        }
                        pc = 0x00427b98;
                        continue;
                    }
                    pc = 0x00427bb4;
                    continue;
                }
                0x00427bb4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                    if t {
                        pc = 0x00427bc8;
                        continue;
                    }
                    pc = 0x00427bbc;
                    continue;
                }
                0x00427bbc => {
                    c.cycles += 3;
                    c.g[S1][0] = 0x530000;
                    c.g[S0][0] = 0x530000;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                    pc = 0x00427bc8;
                    continue;
                }
                0x00427bc8 => {
                    c.cycles += 18;
                    c.g[RA][0] = sx(0x00427bd0);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1cc0));
                    Callee {
                        addr: 0x00423dc0,
                        f: ee::s15::SignalSema,
                    }
                    .call(c, 0x00427bd0)?;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1c80));
                    c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x2940));
                    c.sw((c.g[S2][0] as u32).wrapping_sub(0x2940), c.g[V1][0]);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1200));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x440));
                    c.g[A1][0] = 0xff;
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[A2][0] = 0u64;
                    c.sw(c.g[SP][0] as u32, 0u64);
                    c.g[T0][0] = 0x8;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0x8;
                    c.g[RA][0] = sx(0x00427c08);
                    c.g[T3][0] = 0u64;
                    Callee {
                        addr: 0x00426d18,
                        f: ee::s15::sceSifCallRpc,
                    }
                    .call(c, 0x00427c08)?;
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[A0][0] = 0x20000000;
                    if t {
                        pc = 0x00427c1c;
                        continue;
                    }
                    pc = 0x00427c10;
                    continue;
                }
                0x00427c10 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xfffffffffffe0000;
                    let t = true;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    if t {
                        pc = 0x00427c74;
                        continue;
                    }
                    pc = 0x00427c1c;
                    continue;
                }
                0x00427c1c => {
                    c.cycles += 22;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.g[S0][0] | c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.g[A1][0] = 0x530000;
                    c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x11d8));
                    c.lwl(4, (c.g[V1][0] as u32).wrapping_add(0x3));
                    c.lwr(4, c.g[V1][0] as u32);
                    c.swl(4, (c.g[T0][0] as u32).wrapping_add(0x3));
                    c.swr(4, c.g[T0][0] as u32);
                    c.lwl(3, (c.g[V0][0] as u32).wrapping_add(0x3));
                    c.lwr(3, c.g[V0][0] as u32);
                    c.swl(3, (c.g[SP][0] as u32).wrapping_add(0x13));
                    c.swr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = 0x450000;
                    c.g[A1][0] = 0x450000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = 0x1;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x520c), c.g[A0][0]);
                    c.g[V0][0] = 0u64;
                    c.g[V1][0] = c.g[V1][0] ^ 0x2;
                    c.g[V1][0] = (c.g[V1][0] < 0x1u64) as u64;
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x5210), c.g[V1][0]);
                    pc = 0x00427c74;
                    continue;
                }
                0x00427c74 => {
                    c.cycles += 8;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00427a90, 0x00427c94);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::check;
    use crate::difftest::check_paths;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    use crate::difftest::put_words;
    use crate::game::lib::sif::tests::mrpc_setup;
    use crate::game::lib::sif::tests::packet_slots;
    use crate::game::lib::sif::tests::put_cstr;
    use crate::game::lib::sif::tests::sif_packet_setup;
    use crate::game::lib::sif::tests::RPC_OWNER;
    use crate::game::lib::sif::tests::RPC_PACKET;
    use crate::game::lib::sif::tests::RPC_REQUEST;
    use crate::game::lib::sif::tests::SIF_CLIENT;
    use crate::game::lib::sif::tests::SIF_PACKET;
    use crate::game::lib::sif::tests::SIF_RPC_POOL;
    use crate::game::lib::sif::tests::SIF_RPC_POOL_COUNT;

    /// `sceSifGetSreg`: random register numbers over a random table.
    #[test]
    fn sce_sif_get_sreg_matches() {
        check_paths(
            ee::s15::f_00425cf0__generated,
            sce_sif_get_sreg,
            100,
            &[6],
            &|c, seed| {
                let mut r = Rng(0x0042_5cf0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0042_5cf0;
                let n = r.next_u32() % 0x40;
                c.g[A0][0] = n as u64;
                for i in 0..0x40 {
                    c.bus.write32(SIF_SREG_TABLE + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `sceSifCmd_SetSregHandler`: a command (register index, value) against
    /// a manager with a table pointer.
    #[test]
    fn sce_sif_cmd_set_sreg_handler_matches() {
        check_paths(
            ee::s15::f_00425cc0__generated,
            sce_sif_cmd_set_sreg_handler,
            100,
            &[7],
            &|c, seed| {
                let mut r = Rng(0x0042_5cc0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0042_5cc0;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x110, r.next_u32() % 16);
                c.bus.write32(DATA + 0x114, r.next_u32());
                c.bus.write32(DATA + 0x21c, DATA + 0x300);
                for i in 0..16 {
                    c.bus.write32(DATA + 0x300 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `sceSifSetCmdBuffer`: random buffer and size over random state.
    #[test]
    fn sce_sif_set_cmd_buffer_matches() {
        check_paths(
            ee::s15::f_00425ff0__generated,
            sce_sif_set_cmd_buffer,
            100,
            &[6],
            &|c, seed| {
                let mut r = Rng(0x0042_5ff0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x0042_5ff0;
                c.g[A0][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(SIF_CMD_STATE + 0x14, r.next_u32());
                c.bus.write32(SIF_CMD_STATE + 0x18, r.next_u32());
            },
        );
    }

    /// Gate words of this module's fakes (in the machine's RAM).
    pub(crate) const OC698_WAIT_GATE: u32 = DATA + 0x7200;

    pub(crate) const OC698_DMA_GATE: u32 = DATA + 0x7204;

    pub(crate) const OC698_SEMA_FAIL_GATE: u32 = DATA + 0x7208;

    pub(crate) const OC698_SEMA_FAIL_SKIP: u32 = DATA + 0x720c;

    pub(crate) const OC698_GETREG_GATE: u32 = DATA + 0x7210;

    pub(crate) const OC698_GETREG_VALUE: u32 = DATA + 0x7214;

    pub(crate) const OC698_GATE_MAGIC: u32 = 0x4f43_3639;

    /// `WaitSema` with its gate set: the sema is already signalled (the IOP
    /// answered at once), so the wait returns its id without blocking.
    pub(crate) fn oc698_fake_wait_sema(c: &mut Cpu) {
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
    pub(crate) fn oc698_fake_set_dma(c: &mut Cpu) {
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
    pub(crate) fn oc698_on_thread(c: &mut Cpu) {
        let saved = (c.pc, c.g[A0], c.g[A1], c.g[RA]);
        ssx3_ps2::kernel::start_main_thread(c, saved.0, 0, 0);
        (c.pc, c.g[A0], c.g[A1], c.g[RA]) = saved;
    }

    /// Hook both fakes, open the gate at `gate`, and run on the main guest
    /// thread.
    pub(crate) fn oc698_answered_on_thread(c: &mut Cpu, gate: u32) {
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
    pub(crate) fn oc698_syscall(c: &mut Cpu, at: u32) {
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
    pub(crate) fn oc698_create_sema_fails_after(c: &mut Cpu, skip: u32) {
        c.hooks.syscall = oc698_syscall;
        c.bus.write32(OC698_SEMA_FAIL_GATE, OC698_GATE_MAGIC);
        c.bus.write32(OC698_SEMA_FAIL_SKIP, skip);
        oc698_on_thread(c);
    }

    /// One free packet in the SIF RPC pool (`{total, packets, count}`).
    pub(crate) fn oc698_one_free_packet(c: &mut Cpu, total: u32) {
        c.bus.write32(SIF_RPC_POOL, total);
        c.bus.write32(SIF_RPC_POOL + 4, SIF_PACKET);
        c.bus.write32(SIF_RPC_POOL_COUNT, 1);
        packet_slots(c, SIF_PACKET, 1, &|_| false);
    }

    /// Zero the client struct at `SIF_CLIENT`.
    pub(crate) fn oc698_clear_client(c: &mut Cpu) {
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
    pub(crate) const OC698_LONG_POOL: u32 = DATA + 0x1204;

    /// Slots (stride 0x40) of the long packet pool.
    pub(crate) const OC698_LONG_POOL_SLOTS: u32 = DATA + 0x1440;

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

    pub(crate) const SEEDS: u32 = 200;

    /// fs semaphore word (-1 until `sceFs_SemaInit` creates it).
    pub(crate) const FS_SEMA: u32 = 0x0045_5214;

    /// fd-table semaphore word (-1 until `sceFs_FdSemaInit` creates it).
    pub(crate) const FS_FD_SEMA: u32 = 0x0045_5218;

    /// Nonzero once `sceFsInit` ran.
    pub(crate) const FS_INITIALISED: u32 = 0x0045_520c;

    /// "SIF RPC library initialised" word of `sceSifInitRpc`.
    pub(crate) const SIF_RPC_INITIALISED: u32 = 0x0045_5180;

    /// The fd table: 32 entries of 16 bytes, in-use word at +4.
    pub(crate) const FS_FD_TABLE: u32 = 0x0052_ec00;

    /// The RPC stamp compared by `sceFs_NeedsRebind`, and the two known ones.
    pub(crate) const FS_STAMP: u32 = 0x0052_ee28;

    pub(crate) const FS_STAMP_A: u32 = 0x0045_5164;

    pub(crate) const FS_STAMP_B: u32 = 0x0045_5220;

    /// The fs RPC client (its server word is at +0x24).
    #[allow(dead_code)]
    pub(crate) const FS_CLIENT: u32 = 0x0052_ee00;

    /// IOP heap error word and RPC client.
    pub(crate) const IOP_HEAP_STATE: u32 = 0x0045_5224;

    pub(crate) const IOP_HEAP_CLIENT: u32 = 0x0052_eec0;

    /// Shared fs state for the open/init fixtures: which semaphores exist,
    /// whether the fs is initialised, stamps (a rebind needed on seed % 7 == 3),
    /// a full fd table (seed % 5 == 4) and SIF packets (seed % 9 == 8).
    pub(crate) fn fs_setup(c: &mut Cpu, seed: u32) {
        c.bus
            .write32(FS_SEMA, if seed % 2 == 0 { 7 } else { u32::MAX });
        c.bus
            .write32(FS_FD_SEMA, if seed / 2 % 2 == 0 { 9 } else { u32::MAX });
        c.bus
            .write32(FS_INITIALISED, if seed % 3 == 0 { 0 } else { 1 });
        c.bus.write32(SIF_RPC_INITIALISED, 1);
        if seed % 7 == 3 {
            put_words(c, FS_STAMP, &[0x0101_0101]);
            put_words(c, FS_STAMP_A, &[0x0202_0202]);
            put_words(c, DATA + 0x40, &[0x0303_0303]);
            put_words(c, FS_STAMP_B, &[DATA + 0x40]);
        }
        for i in 0..32 {
            c.bus
                .write32(FS_FD_TABLE + 16 * i + 4, (seed % 5 == 4) as u32);
        }
        sif_packet_setup(c, seed);
    }

    /// `sceFs_Lock`: creates the fs semaphore when missing, then waits on it.
    #[test]
    fn sce_fs_lock_matches() {
        covered(
            ee::s15::f_00427988__generated,
            sce_fs_lock,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 237_001, 2);
                c.pc = 0x0042_7988;
                c.bus
                    .write32(FS_SEMA, if seed % 2 == 0 { 7 } else { u32::MAX });
            },
        );
    }

    /// `sceFs_Unlock`: tail-calls SignalSema on the fs semaphore.
    #[test]
    fn sce_fs_unlock_matches() {
        covered(
            ee::s15::f_004279b8__generated,
            sce_fs_unlock,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 237_002, 1);
                c.pc = 0x0042_79b8;
                c.bus.write32(FS_SEMA, seed.wrapping_mul(0x9e37_79b9));
            },
        );
    }

    /// `sceFs_NeedsRebind`: the three stamp comparisons.
    #[test]
    fn sce_fs_needs_rebind_matches() {
        check(
            ee::s15::f_00427c98__generated,
            sce_fs_needs_rebind,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 237_003, 1);
                c.pc = 0x0042_7c98;
                match seed % 4 {
                    0 => {}
                    1 => put_words(c, FS_STAMP, &[0x0101_0101]),
                    2 => {
                        put_words(c, FS_STAMP, &[0x0101_0101]);
                        put_words(c, FS_STAMP_B, &[0x0202_0202]);
                        put_words(c, FS_STAMP_A, &[0xFFFF_FFFF]);
                    }
                    _ => {
                        put_words(c, FS_STAMP, &[0x0101_0101]);
                        put_words(c, FS_STAMP_B, &[0xFFFF_FFFF]);
                    }
                }
            },
        );
    }

    /// `sceSifInitIopHeap` fixture: bind succeeds on packet seeds; the server
    /// word is set on some of them (the bind may clear it again).
    pub(crate) fn iop_heap_setup(c: &mut Cpu, seed: u32) {
        sif_packet_setup(c, seed);
        c.bus
            .write32(IOP_HEAP_STATE, if seed % 2 == 0 { 0 } else { u32::MAX });
        c.bus.write32(
            IOP_HEAP_CLIENT + 0x24,
            if seed % 4 < 2 { 0 } else { 0x1_0000 },
        );
    }

    #[test]
    fn sce_sif_init_iop_heap_matches() {
        check(
            ee::s15::f_0042ab18__generated,
            sce_sif_init_iop_heap,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 237_004, 1);
                c.pc = 0x0042_ab18;
                iop_heap_setup(c, seed);
            },
        );
    }

    driven!(
        sce_sif_init_iop_heap_gen_driven,
        ee::s15::f_0042ab18__generated
    );

    driven!(sce_sif_init_iop_heap_hand_driven, sce_sif_init_iop_heap);

    /// The spin loops re-entered at their heads after an event.
    #[test]
    fn sce_sif_init_iop_heap_events() {
        check_events(
            sce_sif_init_iop_heap_gen_driven,
            sce_sif_init_iop_heap_hand_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 237_004, 0);
                iop_heap_setup(c, seed);
                schedule(c, &mut r, seed, 0x0042_ab18)
            },
        );
    }

    /// `sceSifAllocIopHeap`: 0 while the heap word is negative, else the RPC.
    #[test]
    fn sce_sif_alloc_iop_heap_matches() {
        check(
            ee::s15::f_0042aba0__generated,
            sce_sif_alloc_iop_heap,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 237_005, 1);
                c.pc = 0x0042_aba0;
                c.bus
                    .write32(IOP_HEAP_STATE, if seed % 2 == 0 { 0 } else { u32::MAX });
                c.bus.write32(0x0052_ef00, 0x7a00_7a00);
                sif_packet_setup(c, seed);
                c.g[A0][0] = (seed % 64) as u64;
            },
        );
    }

    /// `sceFsInit`: SIF init, handlers, bind spin, fd semaphore, init RPC.
    #[allow(dead_code)]
    pub(crate) fn fs_init_setup(c: &mut Cpu, seed: u32) {
        fs_setup(c, seed);
        // SIF RPC init already ran on most seeds; the others run it for real.
        c.bus.write32(SIF_RPC_INITIALISED, (seed % 5 != 0) as u32);
        c.bus
            .write32(FS_CLIENT + 0x24, if seed % 4 < 2 { 0 } else { 0x1_0000 });
    }

    /// The path `sceOpen` receives: empty, short, 1023 bytes, or without a
    /// terminator inside the 1024-byte limit.
    pub(crate) fn open_path(c: &mut Cpu, seed: u32) {
        let at = DATA + 0x1000;
        match seed % 4 {
            0 => put_cstr(c, at, b""),
            1 => put_cstr(c, at, b"cdrom0:\\SSX3\\DATA.BIN;1"),
            2 => {
                for i in 0..1023 {
                    c.bus.write8(at + i, b'a' + (i % 26) as u8);
                }
                c.bus.write8(at + 1023, 0);
            }
            _ => {
                for i in 0..1100 {
                    c.bus.write8(at + i, b'b');
                }
            }
        }
        c.g[A0][0] = at as u64;
        c.g[A1][0] = (seed % 8) as u64 | 0x8000_0000;
    }

    /// `sceOpen`: lock, init on first use, rebind check, fd allocation, path
    /// copy, reply semaphore and the open RPC.
    #[test]
    fn sce_open_matches() {
        check(
            ee::s15::f_00427d60__generated,
            sce_open,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 237_007, 6);
                c.pc = 0x0042_7d60;
                fs_setup(c, seed);
                open_path(c, seed);
            },
        );
    }

    #[test]
    fn sce_open_events() {
        check_events(
            ee::s15::f_00427d60__generated,
            sce_open,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 237_007, 0);
                fs_setup(c, seed);
                open_path(c, seed);
                schedule(c, &mut r, seed, 0x0042_7d60)
            },
        );
    }

    use crate::difftest::fakes::LOG_RESULTS;

    /// The DMAC wrappers: real `DIntr`/`EIntr`, fake syscalls (the wrapper's
    /// trap may unwind). Status.EIE is set on half the seeds.
    pub(crate) fn dmac_wrapper_syscall_test(gen: GuestFn, hand: GuestFn, entry: u32, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 1);
            c.pc = entry;
            c.g[A0][0] = (r.next_u32() % 8) as u64;
            let status = r.next_u32() & !0x1_0000;
            c.cop0[12] = if seed % 2 == 0 {
                status | 0x1_0000
            } else {
                status
            };
        };
        covered(gen, hand, 200, &setup);
    }

    #[test]
    fn disable_dmac_wrapper_matches() {
        dmac_wrapper_syscall_test(
            ee::s15::f_00424950__generated,
            disable_dmac_wrapper,
            0x0042_4950,
            0x4950,
        );
    }

    #[test]
    fn enable_dmac_wrapper_matches() {
        dmac_wrapper_syscall_test(
            ee::s15::f_004249b8__generated,
            enable_dmac_wrapper,
            0x0042_49b8,
            0x49b8,
        );
    }

    /// `iWakeupThread`: the `-47` call's id (forced through the fake's first
    /// result) equals the argument on two seeds in three, is in range on
    /// four in five, and the gate word is set on half the seeds; the other
    /// id paths call `_iWakeupThread`. Fake syscalls may unwind.
    #[test]
    fn i_wakeup_thread_signal_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x4c50, 3);
            c.pc = 0x0042_4c50;
            let id = if seed % 5 == 0 {
                0x100 + r.next_u32() % 0x1000
            } else {
                1 + r.next_u32() % 0xff
            };
            c.bus.write32(LOG_RESULTS, id);
            let same = seed % 3 != 0;
            c.g[A0][0] = if same { id as u64 } else { (id ^ 1) as u64 };
            let gate = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(0x0045_5170, gate);
        };
        covered(
            ee::s15::f_00424c50__generated,
            i_wakeup_thread_signal,
            200,
            &setup,
        );
    }

    use crate::difftest::fresh;
    use crate::difftest::run_generated;

    pub(crate) use crate::game::lib::eafile::tests::oc813_cd_init_setup;

    pub(crate) use crate::game::lib::eafile::tests::oc813_hook_all;

    pub(crate) use crate::game::lib::eafile::tests::OC813_BIND_LEFT;
    pub(crate) use crate::game::lib::eafile::tests::OC813_BIND_RES;
    pub(crate) use crate::game::lib::eafile::tests::OC813_CALL_RES;

    pub(crate) use crate::game::lib::eafile::tests::OC813_DINTR_VAL;

    pub(crate) use crate::game::lib::eafile::tests::OC813_GATE;

    pub(crate) use crate::game::lib::eafile::tests::OC813_MAGIC;

    pub(crate) use crate::game::lib::eafile::tests::OC813_SEEDS;

    /// The fixture reaches the early outs, the directory-list and root-scan
    /// branches and the full init: many distinct cycle totals.
    #[test]
    fn ea_file_cd_init_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..OC813_SEEDS {
            let mut c = fresh();
            oc813_cd_init_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s14::f_003e4040__generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= 12,
            "only {} distinct paths: {seen:?}",
            seen.len()
        );
    }

    /// `sceFsInit`: brings up SIF RPC, binds the fs RPC server (retrying
    /// while the server word is 0), clears the fd table, creates the fd
    /// semaphore and asks the IOP for its version (`sceSifCallRpc`).
    pub(crate) fn oc813_fs_init_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 813_002, 12);
        oc813_hook_all();
        c.bus.write32(OC813_GATE, OC813_MAGIC);
        c.pc = 0x0042_7a90;
        // sceSifInitRpc already done (its flag), so the real callee is short.
        c.bus.write32(0x0045_5180, 1);
        c.bus.write32(OC813_DINTR_VAL, seed & 1);
        let case = (seed >> 1) % 5;
        c.bus
            .write32(OC813_BIND_RES, if case == 2 { u32::MAX } else { 0 });
        c.bus.write32(OC813_BIND_LEFT, (case == 4) as u32);
        c.bus
            .write32(OC813_CALL_RES, if case == 3 { u32::MAX } else { 0 });
        // The IOP's reply (uncached alias of 0x52e340): version word at +4.
        put_words(
            c,
            0x0052_e340,
            &[
                seed.wrapping_mul(0x9e37_79b9),
                if case == 0 { 2 } else { 3 },
            ],
        );
        c.bus.write32(0x0045_5218, 5 + seed % 3);
    }

    #[test]
    fn sce_fs_init_matches() {
        check(
            ee::s15::f_00427a90__generated,
            sce_fs_init,
            40,
            &|c, seed| oc813_fs_init_setup(c, seed),
        );
    }

    #[test]
    fn sce_fs_init_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..40 {
            let mut c = fresh();
            oc813_fs_init_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s15::f_00427a90__generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= 6,
            "only {} distinct paths: {seen:?}",
            seen.len()
        );
    }
}
