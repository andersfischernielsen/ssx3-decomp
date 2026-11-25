//! Staging module oc699 (wave wave237): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc699`.

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
    (0x0042_7988, sce_fs_lock, "sceFs_Lock"),
    (0x0042_79b8, sce_fs_unlock, "sceFs_Unlock"),
    (0x0042_7c98, sce_fs_needs_rebind, "sceFs_NeedsRebind"),
    (0x0042_7d60, sce_open, "sceOpen"),
    (0x0042_ab18, sce_sif_init_iop_heap, "sceSifInitIopHeap"),
    (0x0042_aba0, sce_sif_alloc_iop_heap, "sceSifAllocIopHeap"),
];

/// `int sceFs_Lock(void)`: make sure the fs semaphore exists, wait on it and return 0.
pub fn sce_fs_lock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
            c.sd((c.g[SP][0] as u32), c.g[S0][0]);
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
        c.g[S0][0] = c.ld((c.g[SP][0] as u32));
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
                    c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
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
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                if t {
                                    break 'b_427e84;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
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
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
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
                c.sw((c.g[S1][0] as u32), c.g[S0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1200));
                c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x2900));
                c.g[A1][0] = 0u64;
                c.sw((c.g[SP][0] as u32), 0u64);
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
                c.g[S1][0] = c.lw((c.g[V0][0] as u32));
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
            c.sw((c.g[S3][0] as u32), c.g[V0][0]);
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
                    c.sd((c.g[SP][0] as u32), c.g[S0][0]);
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
                    c.g[S0][0] = c.ld((c.g[SP][0] as u32));
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
                    c.sw((c.g[SP][0] as u32), 0u64);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, machine};
    use crate::difftest::{check, put_words, DATA};
    use crate::game::lib::sif::tests::{put_cstr, sif_packet_setup};

    const SEEDS: u32 = 200;

    /// fs semaphore word (-1 until `sceFs_SemaInit` creates it).
    const FS_SEMA: u32 = 0x0045_5214;
    /// fd-table semaphore word (-1 until `sceFs_FdSemaInit` creates it).
    const FS_FD_SEMA: u32 = 0x0045_5218;
    /// Nonzero once `sceFsInit` ran.
    const FS_INITIALISED: u32 = 0x0045_520c;
    /// "SIF RPC library initialised" word of `sceSifInitRpc`.
    const SIF_RPC_INITIALISED: u32 = 0x0045_5180;
    /// The fd table: 32 entries of 16 bytes, in-use word at +4.
    const FS_FD_TABLE: u32 = 0x0052_ec00;
    /// The RPC stamp compared by `sceFs_NeedsRebind`, and the two known ones.
    const FS_STAMP: u32 = 0x0052_ee28;
    const FS_STAMP_A: u32 = 0x0045_5164;
    const FS_STAMP_B: u32 = 0x0045_5220;
    /// The fs RPC client (its server word is at +0x24).
    const FS_CLIENT: u32 = 0x0052_ee00;
    /// IOP heap error word and RPC client.
    const IOP_HEAP_STATE: u32 = 0x0045_5224;
    const IOP_HEAP_CLIENT: u32 = 0x0052_eec0;

    /// Shared fs state for the open/init fixtures: which semaphores exist,
    /// whether the fs is initialised, stamps (a rebind needed on seed % 7 == 3),
    /// a full fd table (seed % 5 == 4) and SIF packets (seed % 9 == 8).
    fn fs_setup(c: &mut Cpu, seed: u32) {
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
    fn iop_heap_setup(c: &mut Cpu, seed: u32) {
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
    fn fs_init_setup(c: &mut Cpu, seed: u32) {
        fs_setup(c, seed);
        // SIF RPC init already ran on most seeds; the others run it for real.
        c.bus.write32(SIF_RPC_INITIALISED, (seed % 5 != 0) as u32);
        c.bus
            .write32(FS_CLIENT + 0x24, if seed % 4 < 2 { 0 } else { 0x1_0000 });
    }

    /// The path `sceOpen` receives: empty, short, 1023 bytes, or without a
    /// terminator inside the 1024-byte limit.
    fn open_path(c: &mut Cpu, seed: u32) {
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
}
