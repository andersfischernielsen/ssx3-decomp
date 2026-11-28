//! `src/lib/dma.cpp`: hand-written DMA (`libdma_*`, `sceDma*`) replacements.

use crate::abi::ret;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_de30, libdma_wait_chan_idle, "libdma_WaitChanIdle"),
    (0x003f_dea0, libdma_wait_madr, "libdma_WaitMadr"),
    (0x003f_df28, libdma_phys_addr, "libdma_PhysAddr"),
    (0x003f_df50, libdma_zero_bytes, "libdma_ZeroBytes"),
    (0x003f_df88, sce_dma_get_chan, "sceDmaGetChan"),
    (0x003f_dfb0, sce_dma_reset, "sceDmaReset"),
    (0x003f_e090, sce_dma_put_env, "sceDmaPutEnv"),
    (0x003f_e2b8, sce_dma_send, "sceDmaSend"),
    (0x003f_e320, sce_dma_send_n, "sceDmaSendN"),
    (0x003f_e398, sce_dma_send_i, "sceDmaSendI"),
    (0x003f_e410, sce_dma_recv, "sceDmaRecv"),
    (0x003f_e458, sce_dma_recv_n, "sceDmaRecvN"),
    (0x003f_e4d8, sce_dma_recv_i, "sceDmaRecvI"),
    (0x003f_e558, sce_dma_sync, "sceDmaSync"),
    (0x003f_e590, sce_dma_watch, "sceDmaWatch"),
    (0x003f_e5c8, sce_dma_pause, "sceDmaPause"),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `libdma_WaitChanIdle` (`0x003f_de30`).
    pub(crate) const WAIT_CHAN_IDLE: Callee = Callee {
        addr: 0x003f_de30,
        f: ee::s15::libdma_WaitChanIdle,
    };

    /// `libdma_WaitMadr` (`0x003f_dea0`).
    pub(crate) const WAIT_MADR: Callee = Callee {
        addr: 0x003f_dea0,
        f: ee::s15::libdma_WaitMadr,
    };

    /// `libdma_PhysAddr` (`0x003f_df28`).
    pub(crate) const PHYS_ADDR: Callee = Callee {
        addr: 0x003f_df28,
        f: ee::s15::libdma_PhysAddr,
    };

    /// `libdma_ZeroBytes` (`0x003f_df50`).
    pub(crate) const ZERO_BYTES: Callee = Callee {
        addr: 0x003f_df50,
        f: ee::s15::libdma_ZeroBytes,
    };

    /// `sceDmaPutEnv` (`0x003f_e090`).
    pub(crate) const PUT_ENV: Callee = Callee {
        addr: 0x003f_e090,
        f: ee::s15::sceDmaPutEnv,
    };

    /// `sceDmaPause` (`0x003f_e5c8`).
    pub(crate) const DMA_PAUSE: Callee = Callee {
        addr: 0x003f_e5c8,
        f: ee::s15::sceDmaPause,
    };

    pub(super) use crate::game::lib::sif::callee::PRINTF;

    pub(super) use crate::game::lib::cdvd::callee::DINTR;

    pub(super) use crate::game::lib::sif::callee::EINTR;
}

/// `u32 libdma_PhysAddr(void *p)` (0x3fdf28): convert an address for a DMA
/// address register: scratchpad addresses (top nibble 7,
/// `0x7000_0000..0x7fff_ffff`) come back as `(p & 0x0fff_ffff) | 0x8000_0000`
/// (sign-extended; bit 31 selects scratchpad memory), anything else passes
/// through unchanged.
pub fn libdma_phys_addr(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x7;
    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 28);
    let folded = c.g[V0][0] == c.g[V1][0];
    c.g[V0][0] = 0xfff0000;
    if folded {
        c.cycles += 4;
        c.g[V1][0] = 0xffff_ffff_8000_0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
        c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `void libdma_WaitChanIdle(void *chan)` (0x3fde30): spin until the start
/// bit (`0x100`) of the channel's CHCR (`chan+0`) clears. Once the
/// `0x1000000`-poll budget runs out it prints the timeout message
/// (`0x4977d8`) and calls `sceDmaPause(chan)`, which clears the bit and ends
/// the wait.
pub fn libdma_wait_chan_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S1][0] = 0x1000000;
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = c.g[V0][0] & 0x100;
        let idle = c.g[V0][0] == 0u64;
        c.g[S2][0] = 0x490000;
        if !idle {
            c.cycles += 1;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            loop {
                c.cycles += 2;
                let live = (c.g[S1][0] as i64) >= 0;
                if !live {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003f_de70);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x77d8));
                    callee::PRINTF.call(c, 0x003f_de70)?;
                    c.g[RA][0] = sx(0x003f_de78);
                    c.g[A0][0] = c.g[S0][0];
                    callee::DMA_PAUSE.call(c, 0x003f_de78)?;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = c.g[V0][0] & 0x100;
                let busy = c.g[V0][0] != 0u64;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                if busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_de60);
                    }
                } else {
                    break;
                }
            }
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void libdma_WaitMadr(void *chan, u32 pos, int count)` (0x3fdea0):
/// spin until the channel MADR (`chan+0x10`) reaches `pos`, printing
/// and pausing the channel once the `count` (default `0x1000000`)
/// budget runs out. The fast path returns with `v0` clear and the
/// frame restored; the timeout arm is unreachable without live DMA.
pub fn libdma_wait_madr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = c.g[A1][0];
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[S0][0] = 0x1000000;
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        if c.g[A2][0] != 0 {
            c.g[S0][0] = c.g[A2][0];
        }
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = (c.g[V0][0] < c.g[S2][0]) as u64;
        let done = c.g[V0][0] == 0u64;
        c.g[S3][0] = 0x490000;
        if !done {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            loop {
                c.cycles += 2;
                let live = (c.g[S0][0] as i64) >= 0;
                if !live {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003f_def0);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x77d8));
                    callee::PRINTF.call(c, 0x003f_def0)?;
                    c.g[RA][0] = sx(0x003f_def8);
                    c.g[A0][0] = c.g[S1][0];
                    callee::DMA_PAUSE.call(c, 0x003f_def8)?;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = (c.g[V0][0] < c.g[S2][0]) as u64;
                let waiting = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                if waiting {
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_dee0);
                    }
                } else {
                    break;
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void libdma_ZeroBytes(void *dst, int len)` (0x3fdf50): zero `len`
/// bytes at `dst`, one `sb` per byte (idle when `len` is 0).
pub fn libdma_zero_bytes(c: &mut Cpu) {
    c.cycles += 2;
    let empty = c.g[A1][0] == 0u64;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
    if !empty {
        c.cycles += 2;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        loop {
            c.cycles += 7;
            c.sb(c.g[A0][0] as u32, 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            let more = c.g[V0][0] != c.g[V1][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003f_df60);
                }
                continue;
            } else {
                break;
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `void *sceDmaGetChan(int id)` (0x3fdf88): register block of DMA
/// channel `id` (table word at `0x451590 + 4 * id`), or NULL when
/// `id >= 10`.
pub fn sce_dma_get_chan(c: &mut Cpu) {
    c.cycles += 3;
    let valid = c.g[A0][0] < 0xa;
    c.g[V0][0] = 0x450000;
    if valid {
        c.cycles += 5;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx(0x0045_1590);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0u64;
        ret(c);
    }
}

/// `int sceDmaPause(int chan)` (0x3fe5c8): stop a DMA channel: with
/// interrupts disabled, suspend every DMAC channel (set bit `0x10000` through
/// `D_ENABLEW` `0x1000f590` when `D_ENABLER` `0x1000f520` lacks it), clear
/// the start bit (`0x100`) of the channel's CHCR (`chan+0`), then write the
/// saved `D_ENABLER` value back to `D_ENABLEW` and re-enable interrupts when
/// they were enabled on entry. Returns the old CHCR.
pub fn sce_dma_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x003f_e5e0);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        callee::DINTR.call(c, 0x003f_e5e0)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[A0][0] = 0x10000;
        c.g[V0][0] = 0x10000000;
        c.g[V0][0] = c.g[V0][0] | 0xf520;
        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = c.g[A2][0] & c.g[A0][0];
        let stalled = c.g[V1][0] != 0u64;
        c.g[V0][0] = 0x10000000;
        if !stalled {
            c.cycles += 4;
            c.g[V1][0] = c.g[A2][0] | c.g[A0][0];
            c.g[V0][0] = c.g[V0][0] | 0xf590;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[V0][0] = 0x10000000;
        }
        c.cycles += 12;
        c.g[A0][0] = 0xffffffffffff0000;
        c.g[V0][0] = c.g[V0][0] | 0xe000;
        c.g[A0][0] = c.g[A0][0] | 0xfeff;
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = 0x10000000;
        c.g[S0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V1][0] = c.g[V1][0] | 0xf590;
        c.g[A0][0] = c.g[S0][0] & c.g[A0][0];
        c.sw(c.g[S1][0] as u32, c.g[A0][0]);
        c.sw(c.g[V1][0] as u32, c.g[A2][0]);
        let ints_off = c.g[A3][0] == 0u64;
        c.g[V0][0] = c.g[S0][0];
        if !ints_off {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003f_e648);
            callee::EINTR.call(c, 0x003f_e648)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceDmaPutEnv(const sceDmaEnv *env)` (0x3fe090): validate the four
/// channel-select bytes of a DMA environment (each < 10, the last < 7)
/// and publish it: the channel-control word folded from the DEV9/DMA
/// lookup tables, PCR, SQWC, RBOR, RBSR, plus a shadow copy at 0x5261d0.
/// Returns 0, or -1..-4 for the first bad byte.
pub fn sce_dma_put_env(c: &mut Cpu) {
    c.cycles += 20;
    c.g[V0][0] = 0x10000000;
    c.g[V1][0] = 0x10000000;
    c.g[V0][0] = c.g[V0][0] | 0xe000;
    c.g[V1][0] = c.g[V1][0] | 0xe020;
    c.g[T2][0] = c.lw(c.g[V0][0] as u32);
    c.g[A1][0] = 0x10000000;
    c.g[A2][0] = c.lw(c.g[V1][0] as u32);
    c.g[T1][0] = c.g[A0][0];
    c.g[A1][0] = c.g[A1][0] | 0xe030;
    c.g[V1][0] = 0x10000000;
    c.g[A1][0] = c.lw(c.g[A1][0] as u32);
    c.g[V1][0] = c.g[V1][0] | 0xe050;
    c.g[V0][0] = 0x10000000;
    c.g[A0][0] = c.lbu(c.g[T1][0] as u32);
    c.g[A2][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = c.g[V0][0] | 0xe040;
    c.g[T0][0] = c.lw(c.g[V0][0] as u32);
    c.g[A0][0] = (c.g[A0][0] < 0xau64) as u64;
    if c.g[A0][0] != 0u64 {
        c.g[V0][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[V0][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x2));
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[T3][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x3));
                c.cycles += 3;
                c.g[V0][0] = (c.g[T3][0] < 0x7u64) as u64;
                let valid = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x490000;
                if valid {
                    c.cycles += 30;
                    c.g[V1][0] = c.lbu(c.g[T1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7818));
                    c.g[A0][0] = 0x490000;
                    c.g[A1][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7828));
                    c.g[T0][0] = c.lbu(c.g[V1][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffcf;
                    c.g[A0][0] = 0x490000;
                    c.g[A3][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x2));
                    c.g[V0][0] = c.g[T2][0] & c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7838));
                    c.g[T0][0] = sx((c.g[T0][0] as u32) << 4);
                    c.g[A2][0] = c.lbu(c.g[A1][0] as u32);
                    c.g[V1][0] = 0xffffffffffff0000;
                    c.g[T2][0] = c.g[V0][0] | c.g[T0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.g[V1][0] | 0xff3f;
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 6);
                    c.g[V1][0] = c.g[T2][0] & c.g[V1][0];
                    c.g[A0][0] = c.lbu(c.g[A3][0] as u32);
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[T2][0] = c.g[V1][0] | c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] | 0xfff3;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = c.g[T2][0] & c.g[V0][0];
                    let plain = c.g[T3][0] == 0u64;
                    c.g[T2][0] = c.g[V0][0] | c.g[A0][0];
                    if plain {
                        c.cycles += 3;
                        c.g[V0][0] = 0xffffffffffff0000;
                        c.g[V0][0] = c.g[V0][0] | 0xfffd;
                        c.g[T2][0] = c.g[T2][0] & c.g[V0][0];
                    } else {
                        c.cycles += 9;
                        c.g[V1][0] = c.lbu((c.g[T1][0] as u32).wrapping_add(0x3));
                        c.g[V0][0] = 0xffffffffffff0000;
                        c.g[T2][0] = c.g[T2][0] | 0x2;
                        c.g[V0][0] = c.g[V0][0] | 0xfcff;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = c.g[T2][0] & c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                        c.g[T2][0] = c.g[V0][0] | c.g[V1][0];
                    }
                    c.cycles += 39;
                    c.g[V0][0] = c.lhu((c.g[T1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0x10000000;
                    c.g[A2][0] = c.lhu((c.g[T1][0] as u32).wrapping_add(0x6));
                    c.g[A0][0] = c.g[A0][0] | 0xe000;
                    c.g[A1][0] = c.lhu((c.g[T1][0] as u32).wrapping_add(0xa));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                    c.g[T0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = c.g[V0][0] | c.g[A2][0];
                    c.g[A3][0] = c.lhu((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x10000000;
                    c.sw(c.g[A0][0] as u32, c.g[T2][0]);
                    c.g[V1][0] = c.g[V1][0] | 0xe020;
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 16);
                    c.g[V0][0] = 0x10000000;
                    c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = c.g[A1][0] | c.g[A3][0];
                    c.g[V0][0] = c.g[V0][0] | 0xe030;
                    c.g[A2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xc));
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    c.g[V1][0] = 0x10000000;
                    c.g[V1][0] = c.g[V1][0] | 0xe050;
                    c.g[V0][0] = 0x10000000;
                    c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                    c.g[V0][0] = c.g[V0][0] | 0xe040;
                    c.sw(c.g[V0][0] as u32, c.g[T0][0]);
                    c.g[V1][0] = 0x520000;
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x61d0));
                    c.ldl(2, (c.g[T1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[T1][0] as u32);
                    c.ldl(4, (c.g[T1][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.sdl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[A2][0] as u32);
                    c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.sdr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = 0u64;
                    c.pc = t;
                    return;
                } else {
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = 0xfffffffffffffffc;
                    c.pc = t;
                    return;
                }
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0xfffffffffffffffd;
                c.pc = t;
                return;
            }
        } else {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = 0xfffffffffffffffe;
            c.pc = t;
            return;
        }
    } else {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0xffffffffffffffff;
        c.pc = t;
        return;
    }
}

/// `void sceDmaRecv(int chan)` (0x3fe410): kick a channel receive with no
/// new address: clear QWC (`chan+0x20`), CHCR (`chan+0`) to
/// `(old & ~0xd) | 0x104`.
pub fn sce_dma_recv(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e424);
        c.g[S0][0] = c.g[A0][0];
        callee::WAIT_CHAN_IDLE.call(c, 0x003f_e424)?;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V1][0] = 0xfffffffffffffff3;
        c.g[A0][0] = 0xfffffffffffffffe;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[V0][0] = c.g[V0][0] | 0x4;
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | 0x100;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDmaRecvI(int chan, void *addr, int size)` (0x3fe4d8): program
/// a channel for an interrupting receive: physical address into MADR
/// (`chan+0x10`), size into QWC (`chan+0x20`), CHCR (`chan+0`) to
/// `(old & ~0xd) | 0x108`.
pub fn sce_dma_recv_i(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e4fc);
        c.g[A0][0] = c.g[A1][0];
        callee::PHYS_ADDR.call(c, 0x003f_e4fc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003f_e508);
        c.g[A0][0] = c.g[S0][0];
        callee::WAIT_CHAN_IDLE.call(c, 0x003f_e508)?;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        if c.g[V1][0] != c.g[V0][0] {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        }
        c.cycles += 15;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V1][0] = 0xfffffffffffffff3;
        c.g[A0][0] = 0xfffffffffffffffe;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[V0][0] = c.g[V0][0] | 0x8;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[V0][0] = c.g[V0][0] | 0x100;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDmaRecvN(int chan, void *addr, int size)` (0x3fe458): program
/// a channel for a normal receive: physical address into MADR
/// (`chan+0x10`), size into QWC (`chan+0x20`), CHCR (`chan+0`) to
/// `(old & ~0xd) | 0x100`.
pub fn sce_dma_recv_n(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e47c);
        c.g[A0][0] = c.g[A1][0];
        callee::PHYS_ADDR.call(c, 0x003f_e47c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003f_e488);
        c.g[A0][0] = c.g[S0][0];
        callee::WAIT_CHAN_IDLE.call(c, 0x003f_e488)?;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        if c.g[V1][0] != c.g[V0][0] {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V1][0] = 0xfffffffffffffff3;
        c.g[A0][0] = 0xfffffffffffffffe;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[V0][0] = c.g[V0][0] | 0x100;
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceDmaReset(int flag)` (0x3fdfb0): clear the ten channel flag
/// words at 0x4977f0 (zeroing each live channel's registers through its
/// table pointer at 0x451590), zero 0x14 bytes at the stack env scratch,
/// publish it with `sceDmaPutEnv`, then enable the DMAC when `flag` is 1.
/// Returns the saved DMAC control bit.
pub fn sce_dma_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x10000000;
        c.sd(frame.at(c, 0x30), c.g[S1][0]);
        c.g[V0][0] = c.g[V0][0] | 0xe000;
        c.sd(frame.at(c, 0x20), c.g[S0][0]);
        c.g[A2][0] = 0x490000;
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[V1][0] = 0x450000;
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1590));
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x77f0));
        c.g[A0][0] = 0x9;
        c.g[S1][0] = c.g[A1][0] & 0x1;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0u64);
                c.sw(c.g[V0][0] as u32, 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x30), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x50), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0u64);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            }
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003f_dfe8);
                }
            } else {
                c.cycles += 16;
                c.g[V1][0] = 0x10000000;
                c.g[V0][0] = 0xff1f;
                c.g[V1][0] = c.g[V1][0] | 0xe010;
                c.g[A2][0] = 0xffffffffff1f0000;
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x14;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[RA][0] = sx(0x003f_e04c);
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                callee::ZERO_BYTES.call(c, 0x003f_e04c)?;
                c.g[RA][0] = sx(0x003f_e054);
                c.g[A0][0] = c.g[SP][0];
                callee::PUT_ENV.call(c, 0x003f_e054)?;
                c.g[V1][0] = 0x1;
                let not_enabled = c.g[S0][0] != c.g[V1][0];
                c.g[V0][0] = c.g[S1][0];
                if !not_enabled {
                    c.cycles += 6;
                    c.g[V1][0] = 0x10000000;
                    c.g[V1][0] = c.g[V1][0] | 0xe000;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.g[S1][0];
                }
                c.cycles += 5;
                c.g[RA][0] = c.ld(frame.at(c, 0x40));
                c.g[S1][0] = c.ld(frame.at(c, 0x30));
                c.g[S0][0] = c.ld(frame.at(c, 0x20));
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void sceDmaSend(int chan, void *addr)` (0x3fe2b8): program a channel
/// send with no new size: physical address into TADR (`chan+0x30`), clear
/// QWC (`chan+0x20`), CHCR (`chan+0`) to `(old & ~0xc) | 0x105`.
pub fn sce_dma_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e2d4);
        c.g[A0][0] = c.g[A1][0];
        callee::PHYS_ADDR.call(c, 0x003f_e2d4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003f_e2e0);
        c.g[A0][0] = c.g[S0][0];
        callee::WAIT_CHAN_IDLE.call(c, 0x003f_e2e0)?;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        if c.g[V1][0] != c.g[V0][0] {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V1][0] = 0xfffffffffffffff3;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[V0][0] = c.g[V0][0] | 0x105;
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDmaSendI(int chan, void *addr, int size)` (0x3fe398): program
/// a channel for an interrupting send: physical address into MADR
/// (`chan+0x10`), size into QWC (`chan+0x20`), CHCR (`chan+0`) to
/// `(old & ~0xc) | 0x109`.
pub fn sce_dma_send_i(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e3bc);
        c.g[A0][0] = c.g[A1][0];
        callee::PHYS_ADDR.call(c, 0x003f_e3bc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003f_e3c8);
        c.g[A0][0] = c.g[S0][0];
        callee::WAIT_CHAN_IDLE.call(c, 0x003f_e3c8)?;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        if c.g[V1][0] != c.g[V0][0] {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        }
        c.cycles += 12;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V1][0] = 0xfffffffffffffff3;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[V0][0] = c.g[V0][0] | 0x109;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDmaSendN(int chan, void *addr, int size)` (0x3fe320): program
/// a channel for a normal send: physical address into MADR (`chan+0x10`),
/// size into QWC (`chan+0x20`), CHCR (`chan+0`) to `(old & ~0xc) | 0x101`.
pub fn sce_dma_send_n(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[S2][0] = c.g[A2][0];
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e344);
        c.g[A0][0] = c.g[A1][0];
        callee::PHYS_ADDR.call(c, 0x003f_e344)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003f_e350);
        c.g[A0][0] = c.g[S0][0];
        callee::WAIT_CHAN_IDLE.call(c, 0x003f_e350)?;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        if c.g[V1][0] != c.g[V0][0] {
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        }
        c.cycles += 12;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V1][0] = 0xfffffffffffffff3;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[V0][0] = c.g[V0][0] | 0x101;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceDmaSync(int chan, int mode)` (0x3fe558): wait until the channel
/// goes idle (via `libdma_WaitChanIdle`), unless `mode` is 1, in which
/// case return bit 8 of the channel status word at once. Returns 0 after
/// a wait, else the status bit.
pub fn sce_dma_sync(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0x1;
        let wait = c.g[A1][0] != c.g[V0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if !wait {
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
            c.g[V0][0] = c.g[V0][0] & 0x1;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003f_e580);
            callee::WAIT_CHAN_IDLE.call(c, 0x003f_e580)?;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `int sceDmaWatch(int chan, int pos, int mode, int size)` (0x3fe590):
/// when `mode` is 1, return at once whether the channel's MADR
/// (`chan+0x10`) is below `pos`; otherwise wait for it via
/// `libdma_WaitMadr` and return 0.
pub fn sce_dma_watch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0x1;
        let wait = c.g[A2][0] != c.g[V0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        if !wait {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003f_e5b4);
            c.g[A2][0] = c.g[A3][0];
            callee::WAIT_MADR.call(c, 0x003f_e5b4)?;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::require_paths;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    /// `u32 libdma_PhysAddr(p)`: the `0x7...` segment folds to
    /// `0x8000_0000 | (p & 0x0fff_ffff)`, everything else passes
    /// through.
    #[test]
    fn libdma_phys_addr_matches() {
        let dma_phys_addr_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1250_0003, 0);
            c.pc = 0x003f_df28;
            c.g[A0][0] = match seed % 4 {
                0 => 0x7000_1234,
                1 => 0x0000_1234,
                2 => r.next_u32() as u64 & 0x0fff_ffff,
                _ => r.next_u32() as u64 | 0x7000_0000,
            };
        };
        check(
            ee::s15::f_003fdf28__generated,
            libdma_phys_addr,
            SEEDS,
            dma_phys_addr_call,
        );
        require_paths(
            ee::s15::f_003fdf28__generated,
            SEEDS,
            &[6, 10],
            dma_phys_addr_call,
        );
    }

    /// `void libdma_WaitChanIdle(chan)`: a channel without bit 8 set
    /// returns at once. The busy spin is `libdma_wait_chan_idle_events`.
    #[test]
    fn libdma_wait_chan_idle_matches() {
        let dma_wait_chan_idle_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1250_0005, 0);
            c.pc = 0x003f_de30;
            c.g[A0][0] = DMA_CHAN as u64;
            c.bus.write32(DMA_CHAN, r.next_u32() & !0x100);
        };
        check(
            ee::s15::f_003fde30__generated,
            libdma_wait_chan_idle,
            SEEDS,
            dma_wait_chan_idle_call,
        );
        require_paths(
            ee::s15::f_003fde30__generated,
            SEEDS,
            &[17],
            dma_wait_chan_idle_call,
        );
    }

    /// `void libdma_WaitMadr(chan, pos, count)`: MADR already past
    /// `pos`, or exactly at it, returns at once, with both the default
    /// and an explicit spin budget. The spin itself is
    /// `libdma_wait_madr_events`.
    #[test]
    fn libdma_wait_madr_matches() {
        let dma_wait_madr_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1250_0004, 0);
            c.pc = 0x003f_dea0;
            c.g[A0][0] = DMA_CHAN as u64;
            let pos = 1 + seed % 5;
            c.g[A1][0] = pos as u64;
            c.g[A2][0] = if seed % 2 == 0 {
                0
            } else {
                r.next_u32() as u64
            };
            let madr = if seed / 2 % 2 == 0 { 0xffff_ffff } else { pos };
            c.bus.write32(DMA_CHAN + 0x10, madr);
        };
        check(
            ee::s15::f_003fdea0__generated,
            libdma_wait_madr,
            SEEDS,
            dma_wait_madr_call,
        );
        require_paths(
            ee::s15::f_003fdea0__generated,
            SEEDS,
            &[21],
            dma_wait_madr_call,
        );
    }

    /// `void libdma_WaitMadr(chan, pos, count)` on a stalled channel (toSPR's
    /// registers, MADR below `pos`) with an explicit budget of 1-4 polls: the
    /// budget runs out, the timeout message prints and `sceDmaPause` stops
    /// the channel. A gated pause fake runs the real pause and then leaves
    /// MADR at `pos` (the transfer's end), so the next poll ends the wait.
    #[test]
    fn libdma_wait_madr_timeout_matches() {
        let dma_wait_madr_timeout_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1250_0016, 0);
            c.pc = 0x003f_dea0;
            let pos = 0x0010_0000 + 0x10 * (seed % 7);
            c.g[A0][0] = TO_SPR as u64;
            c.g[A1][0] = pos as u64;
            c.g[A2][0] = (1 + seed % 4) as u64;
            c.bus.write32(TO_SPR, r.next_u32() & !0x100);
            c.bus
                .write32(TO_SPR + 0x10, pos - 0x10 * (1 + seed / 7 % 3));
            crate::difftest::fakes::hook(
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FE5C8),
                fake_dma_pause,
                crate::difftest::fakes::prev_cell(fake_dma_pause),
            );
            c.bus.write32(PAUSE_GATE, PAUSE_MAGIC);
            c.bus.write32(PAUSE_MADR, pos);
        };
        check(
            ee::s15::f_003fdea0__generated,
            libdma_wait_madr,
            SEEDS,
            dma_wait_madr_timeout_call,
        );
        require_paths(
            ee::s15::f_003fdea0__generated,
            SEEDS,
            &[141, 147, 153, 159],
            dma_wait_madr_timeout_call,
        );
    }

    /// The toSPR DMA channel's registers (CHCR, MADR at +0x10).
    const TO_SPR: u32 = 0x1000_d400;

    /// Gate words for [`fake_dma_pause`] (in the machine's RAM).
    const PAUSE_GATE: u32 = DATA + 0x3004;
    const PAUSE_MAGIC: u32 = 0x5041_5553;
    /// Where the paused channel's MADR is left.
    const PAUSE_MADR: u32 = DATA + 0x3008;

    /// `sceDmaPause` with its gate set: the real pause, then the channel's
    /// MADR reads `PAUSE_MADR` (the stalled transfer's end).
    fn fake_dma_pause(c: &mut Cpu) {
        if c.bus.read32(PAUSE_GATE) != PAUSE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_dma_pause),
                ee::s15::f_003fe5c8__generated,
            );
        }
        let chan = c.g[A0][0] as u32;
        ee::s15::f_003fe5c8__generated(c);
        let madr = c.bus.read32(PAUSE_MADR);
        c.bus.write32(chan + 0x10, madr);
    }

    /// `void libdma_ZeroBytes(dst, len)`: lengths 0-64 land exactly
    /// `len` zeroed bytes; the empty call writes nothing.
    #[test]
    fn libdma_zero_bytes_matches() {
        let dma_zero_bytes_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1250_0002, 0);
            c.pc = 0x003f_df50;
            c.g[A0][0] = ZERO_BUF as u64;
            c.g[A1][0] = (seed % 65) as u64;
            for i in 0..64 {
                c.bus.write8(ZERO_BUF + i, (i as u8).wrapping_add(1));
            }
        };
        check(
            ee::s15::f_003fdf50__generated,
            libdma_zero_bytes,
            SEEDS,
            dma_zero_bytes_call,
        );
        require_paths(
            ee::s15::f_003fdf50__generated,
            SEEDS,
            &[
                4, 13, 20, 27, 34, 41, 48, 55, 62, 69, 76, 83, 90, 97, 104, 111, 118, 125, 132,
                139, 146, 153, 160, 167, 174, 181, 188, 195, 202, 209, 216, 223, 230, 237, 244,
                251, 258, 265, 272, 279, 286, 293, 300, 307, 314, 321, 328, 335, 342, 349, 356,
                363, 370, 377, 384, 391, 398, 405, 412, 419, 426, 433, 440, 447, 454,
            ],
            dma_zero_bytes_call,
        );
    }

    /// `void *sceDmaGetChan(id)`: table word for ids 0-9, NULL for
    /// 10 and up (including large and sign-extended ids).
    #[test]
    fn sce_dma_get_chan_matches() {
        let dma_get_chan_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1250_0001, 0);
            c.pc = 0x003f_df88;
            for i in 0..10 {
                c.bus.write32(CHAN_TABLE + 4 * i, r.next_u32());
            }
            c.g[A0][0] = [0u64, 1, 5, 9, 10, 11, 0xffff_ffff, 0x1_0000_0000][seed as usize % 8];
        };
        check(
            ee::s15::f_003fdf88__generated,
            sce_dma_get_chan,
            SEEDS,
            dma_get_chan_call,
        );
        require_paths(
            ee::s15::f_003fdf88__generated,
            SEEDS,
            &[5, 8],
            dma_get_chan_call,
        );
    }

    /// `int sceDmaPause(chan)`: the channel-enable write is skipped when
    /// the stall bit already reads set, and EIntr runs only when
    /// interrupts were enabled on entry (all four combinations). D_CTRL
    /// is random, so the D_CTRL read EIntr gets in a1 is compared.
    #[test]
    fn sce_dma_pause_matches() {
        let dma_pause_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_000a, 0);
            c.pc = 0x003f_e5c8;
            c.g[A0][0] = DMA_CHAN as u64;
            c.bus.write32(DMA_CHAN, r.next_u32());
            if seed % 2 == 0 {
                c.cop0[12] &= !0x1_0000;
            } else {
                c.cop0[12] |= 0x1_0000;
            }
            let m = ssx3_ps2::platform::machine_of(&mut c.bus.plat);
            m.hw.d_ctrl = r.next_u32();
            if seed / 2 % 2 == 0 {
                m.hw.d_enable |= 0x1_0000;
            } else {
                m.hw.d_enable &= !0x1_0000;
            }
        };
        check(
            ee::s15::f_003fe5c8__generated,
            sce_dma_pause,
            SEEDS,
            dma_pause_call,
        );
        require_paths(
            ee::s15::f_003fe5c8__generated,
            SEEDS,
            &[39, 43, 55, 59],
            dma_pause_call,
        );
    }

    /// `int sceDmaPutEnv(env)`: each of the four select bytes rejects its
    /// out-of-range value (-1..-4); in-range bytes take the full publish
    /// path returning 0, with byte 3 zero and 1-6. The game's channel
    /// tables are loaded, the DMAC registers start random (D_CTRL bits 1
    /// and 10 both ways), and the DMAC and scratch registers are compared
    /// after the run.
    #[test]
    fn sce_dma_put_env_matches() {
        let dma_put_env_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            elf_image(c);
            let mut r = machine(c, seed, 0x1249_0013, 0);
            random_dmac(c, &mut r);
            let m = ssx3_ps2::platform::machine_of(&mut c.bus.plat);
            if seed / 50 % 2 == 0 {
                m.hw.d_ctrl |= 0x402;
            } else {
                m.hw.d_ctrl &= !0x402;
            }
            c.pc = 0x003f_e090;
            c.g[A0][0] = ENV_BUF as u64;
            let bad = (seed % 5) as u8;
            let mut b = [0u8; 4];
            for (i, v) in b.iter_mut().enumerate() {
                *v = if i as u8 == bad && bad < 4 {
                    [10u8, 10, 10, 7][i] + (seed % 3) as u8
                } else if i < 3 {
                    (r.next_u32() % 10) as u8
                } else {
                    (r.next_u32() % 7) as u8
                };
            }
            if bad == 4 {
                b[0] = (r.next_u32() % 10) as u8;
                b[1] = (r.next_u32() % 10) as u8;
                b[2] = (r.next_u32() % 10) as u8;
                b[3] = if seed % 2 == 0 {
                    0
                } else {
                    1 + (seed / 10 % 6) as u8
                };
            }
            for (i, v) in b.iter().enumerate() {
                c.bus.write8(ENV_BUF + i as u32, *v);
            }
            for i in 0..4 {
                c.bus.write16(ENV_BUF + 4 + 2 * i, r.next_u32() as u16);
            }
            c.bus.write32(ENV_BUF + 0xc, r.next_u32());
            c.bus.write32(ENV_BUF + 0x10, r.next_u32());
        };
        check_dmac(
            ee::s15::f_003fe090__generated,
            sce_dma_put_env,
            SEEDS,
            true,
            dma_put_env_call,
        );
        require_paths(
            ee::s15::f_003fe090__generated,
            SEEDS,
            &[22, 25, 28, 31, 101, 107],
            dma_put_env_call,
        );
    }

    /// `void sceDmaRecvI(chan, addr, size)`: MADR (`chan+0x10`, holding the
    /// address, a random word, all ones or 0) takes the physical address
    /// unless it reads all ones; CHCR lands on `(old & ~0xd) | 0x108`, on an
    /// idle channel or, with WaitChanIdle returning at once, one whose
    /// start bit is still set.
    #[test]
    fn sce_dma_recv_i_matches() {
        let dma_recv_i_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_000d, 0);
            c.pc = 0x003f_e4d8;
            c.g[A0][0] = DMA_CHAN as u64;
            c.g[A1][0] = DATA as u64;
            c.g[A2][0] = r.next_u32() as u64;
            chan_word(c, &mut r, seed / 4 % 2 == 1);
            let madr = [DATA, r.next_u32(), 0xffff_ffff, 0][seed as usize % 4];
            c.bus.write32(DMA_CHAN + 0x10, madr);
        };
        check(
            ee::s15::f_003fe4d8__generated,
            sce_dma_recv_i,
            SEEDS,
            dma_recv_i_call,
        );
        require_paths(
            ee::s15::f_003fe4d8__generated,
            SEEDS,
            &[55],
            dma_recv_i_call,
        );
    }

    /// `void sceDmaRecv(chan)`: clears QWC and kicks the receive, on an
    /// idle channel (the WaitChanIdle short path) or, with WaitChanIdle
    /// returning at once, one whose start bit is still set.
    #[test]
    fn sce_dma_recv_matches() {
        let dma_recv_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_000f, 0);
            c.pc = 0x003f_e410;
            c.g[A0][0] = DMA_CHAN as u64;
            chan_word(c, &mut r, seed % 2 == 1);
        };
        check(
            ee::s15::f_003fe410__generated,
            sce_dma_recv,
            SEEDS,
            dma_recv_call,
        );
        require_paths(ee::s15::f_003fe410__generated, SEEDS, &[35], dma_recv_call);
    }

    /// `void sceDmaRecvN(chan, addr, size)`: MADR (`chan+0x10`, holding the
    /// address, a random word, all ones or 0) takes the physical address
    /// unless it reads all ones; CHCR lands on `(old & ~0xd) | 0x100`, on an
    /// idle channel or, with WaitChanIdle returning at once, one whose
    /// start bit is still set.
    #[test]
    fn sce_dma_recv_n_matches() {
        let dma_recv_n_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_000e, 0);
            c.pc = 0x003f_e458;
            c.g[A0][0] = DMA_CHAN as u64;
            c.g[A1][0] = DATA as u64;
            c.g[A2][0] = r.next_u32() as u64;
            chan_word(c, &mut r, seed / 4 % 2 == 1);
            let madr = [DATA, r.next_u32(), 0xffff_ffff, 0][seed as usize % 4];
            c.bus.write32(DMA_CHAN + 0x10, madr);
        };
        check(
            ee::s15::f_003fe458__generated,
            sce_dma_recv_n,
            SEEDS,
            dma_recv_n_call,
        );
        require_paths(
            ee::s15::f_003fe458__generated,
            SEEDS,
            &[54],
            dma_recv_n_call,
        );
    }

    /// `int sceDmaReset(flag)`: the ten-entry scan loop with real events
    /// on a running thread (structured back edge: the unwind request is
    /// deferred, both sides loop on).
    #[test]
    fn sce_dma_reset_events() {
        check_events(
            ee::s15::f_003fdfb0__generated,
            sce_dma_reset,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1249_0094, 0);
                c.pc = 0x003f_dfb0;
                c.g[A0][0] = (seed % 2) as u64;
                if seed % 3 == 0 {
                    let k = (seed / 3 % 10) as u32;
                    c.bus.write32(RESET_FLAGS + 4 * k, 1);
                    c.bus.write32(RESET_ENVS + 4 * k, RESET_SCRATCH);
                }
                schedule(c, &mut r, seed, 0x003f_dfb0)
            },
        );
    }

    /// `int sceDmaReset(flag)`: `flag` 1 takes the DMAC-enable write,
    /// other flags skip it; one live channel zeroes its registers every
    /// third seed, the rest scan clean. The DMAC registers start random
    /// (D_CTRL bit 0 both ways) and are compared after the run.
    #[test]
    fn sce_dma_reset_matches() {
        let dma_reset_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_0014, 0);
            random_dmac(c, &mut r);
            let m = ssx3_ps2::platform::machine_of(&mut c.bus.plat);
            if seed / 30 % 2 == 0 {
                m.hw.d_ctrl |= 1;
            } else {
                m.hw.d_ctrl &= !1;
            }
            c.pc = 0x003f_dfb0;
            c.g[A0][0] = [0u32, 1, 2][seed as usize % 3] as u64;
            if seed % 3 == 0 {
                let k = (seed / 3 % 10) as u32;
                c.bus.write32(RESET_FLAGS + 4 * k, 1);
                c.bus.write32(RESET_ENVS + 4 * k, RESET_SCRATCH);
            }
        };
        check_dmac(
            ee::s15::f_003fdfb0__generated,
            sce_dma_reset,
            SEEDS,
            false,
            dma_reset_call,
        );
        require_paths(
            ee::s15::f_003fdfb0__generated,
            SEEDS,
            &[342, 348, 350],
            dma_reset_call,
        );
    }

    /// `void sceDmaSendI(chan, addr, size)`: MADR (`chan+0x10`, holding the
    /// address, a random word, all ones or 0) takes the physical address
    /// unless it reads all ones; CHCR lands on `(old & ~0xc) | 0x109`.
    #[test]
    fn sce_dma_send_i_matches() {
        let dma_send_i_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_0010, 0);
            c.pc = 0x003f_e398;
            c.g[A0][0] = DMA_CHAN as u64;
            c.g[A1][0] = DATA as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.bus.write32(DMA_CHAN, r.next_u32() & !0x100);
            let madr = [DATA, r.next_u32(), 0xffff_ffff, 0][seed as usize % 4];
            c.bus.write32(DMA_CHAN + 0x10, madr);
        };
        check(
            ee::s15::f_003fe398__generated,
            sce_dma_send_i,
            SEEDS,
            dma_send_i_call,
        );
        require_paths(
            ee::s15::f_003fe398__generated,
            SEEDS,
            &[52],
            dma_send_i_call,
        );
    }

    /// `void sceDmaSend(chan, addr)`: TADR (`chan+0x30`, holding the
    /// address, a random word, all ones or 0) takes the physical address
    /// unless it reads all ones; CHCR lands on `(old & ~0xc) | 0x105`.
    #[test]
    fn sce_dma_send_matches() {
        let dma_send_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_0012, 0);
            c.pc = 0x003f_e2b8;
            c.g[A0][0] = DMA_CHAN as u64;
            c.g[A1][0] = DATA as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.bus.write32(DMA_CHAN, r.next_u32() & !0x100);
            let tadr = [DATA, r.next_u32(), 0xffff_ffff, 0][seed as usize % 4];
            c.bus.write32(DMA_CHAN + 0x30, tadr);
        };
        check(
            ee::s15::f_003fe2b8__generated,
            sce_dma_send,
            SEEDS,
            dma_send_call,
        );
        require_paths(ee::s15::f_003fe2b8__generated, SEEDS, &[49], dma_send_call);
    }

    /// `void sceDmaSendN(chan, addr, size)`: MADR (`chan+0x10`, holding the
    /// address, a random word, all ones or 0) takes the physical address
    /// unless it reads all ones; CHCR lands on `(old & ~0xc) | 0x101`.
    #[test]
    fn sce_dma_send_n_matches() {
        let dma_send_n_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_0011, 0);
            c.pc = 0x003f_e320;
            c.g[A0][0] = DMA_CHAN as u64;
            c.g[A1][0] = DATA as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.bus.write32(DMA_CHAN, r.next_u32() & !0x100);
            let madr = [DATA, r.next_u32(), 0xffff_ffff, 0][seed as usize % 4];
            c.bus.write32(DMA_CHAN + 0x10, madr);
        };
        check(
            ee::s15::f_003fe320__generated,
            sce_dma_send_n,
            SEEDS,
            dma_send_n_call,
        );
        require_paths(
            ee::s15::f_003fe320__generated,
            SEEDS,
            &[52],
            dma_send_n_call,
        );
    }

    /// `int sceDmaSync(chan, mode)`: mode 1 returns status bit 8 at once
    /// (set and clear); other modes take the WaitChanIdle short path and
    /// return 0.
    #[test]
    fn sce_dma_sync_matches() {
        let dma_sync_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_000c, 0);
            c.pc = 0x003f_e558;
            c.g[A0][0] = DMA_CHAN as u64;
            let mode = [1u32, 0, 1, 2][seed as usize % 4];
            c.g[A1][0] = mode as u64;
            let bit = if mode == 1 { seed / 4 % 2 } else { 0 };
            c.bus.write32(DMA_CHAN, r.next_u32() & !0x100 | (bit << 8));
        };
        check(
            ee::s15::f_003fe558__generated,
            sce_dma_sync,
            SEEDS,
            dma_sync_call,
        );
        require_paths(
            ee::s15::f_003fe558__generated,
            SEEDS,
            &[11, 27],
            dma_sync_call,
        );
    }

    /// `int sceDmaWatch(chan, pos, mode, size)`: mode 1 returns the
    /// MADR-below-pos test at once (MADR below, at and past `pos`); other
    /// modes take the WaitMadr short path (fixture MADR already past
    /// `pos`). The busy-wait loop is `libdma_wait_madr_events`.
    #[test]
    fn sce_dma_watch_matches() {
        let dma_watch_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_000b, 0);
            c.pc = 0x003f_e590;
            c.g[A0][0] = DMA_CHAN as u64;
            let pos = 1 + seed % 5;
            c.g[A1][0] = pos as u64;
            c.g[A2][0] = [1u32, 0, 1, 2][seed as usize % 4] as u64;
            c.g[A3][0] = r.next_u32() as u64;
            let madr = match seed % 4 {
                0 => 0,
                2 if seed / 4 % 2 == 0 => pos,
                _ => 0xffff_ffff,
            };
            c.bus.write32(DMA_CHAN + 0x10, madr);
        };
        check(
            ee::s15::f_003fe590__generated,
            sce_dma_watch,
            SEEDS,
            dma_watch_call,
        );
        require_paths(
            ee::s15::f_003fe590__generated,
            SEEDS,
            &[10, 31],
            dma_watch_call,
        );
    }

    /// Channel table `sceDmaGetChan` indexes.
    pub(crate) const CHAN_TABLE: u32 = 0x0045_1590;

    /// Scratch DMA channel struct for the send/recv/sync/watch/pause
    /// tests (CHCR/QWC/TADR words at +0/+0x10/+0x20/+0x30).
    pub(crate) const DMA_CHAN: u32 = DATA + 0x800;

    /// Scratch DMA environment for the put-env test (0x14 bytes).
    pub(crate) const ENV_BUF: u32 = DATA + 0x1000;

    pub(crate) const RESET_ENVS: u32 = 0x0045_1590;

    /// Channel flag words `sceDmaReset` scans, and the matching channel
    /// pointers, in EE RAM.
    pub(crate) const RESET_FLAGS: u32 = 0x0049_77f0;

    /// Scratch channel-register block the reset test's live-channel arm
    /// zeroes (six words at +0..+0x80).
    pub(crate) const RESET_SCRATCH: u32 = DATA + 0x2000;

    /// Seeds per test (200-300).
    pub(crate) const SEEDS: u32 = 200;

    /// Scratch zeroing buffer for the ZeroBytes test.
    pub(crate) const ZERO_BUF: u32 = DATA + 0x400;

    /// The DMAC registers, D_CTRL to D_ENABLE (`check` compares neither
    /// hardware state).
    fn dmac(c: &mut Cpu) -> [u32; 7] {
        let h = &ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw;
        [
            h.d_ctrl, h.d_stat, h.d_pcr, h.d_sqwc, h.d_rbsr, h.d_rbor, h.d_enable,
        ]
    }

    /// Random D_CTRL, D_STAT, D_PCR, D_SQWC, D_RBSR and D_RBOR (a fresh
    /// machine has them all 0).
    fn random_dmac(c: &mut Cpu, r: &mut Rng) {
        let h = &mut ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw;
        for reg in [
            &mut h.d_ctrl,
            &mut h.d_stat,
            &mut h.d_pcr,
            &mut h.d_sqwc,
            &mut h.d_rbsr,
            &mut h.d_rbor,
        ] {
            *reg = r.next_u32();
        }
    }

    /// [`check`], and the DMAC registers must match after the run, and with
    /// `scratch` the scratch registers too.
    pub(crate) fn check_dmac(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        scratch: bool,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        check(generated, hand, seeds, setup);
        for seed in 0..crate::difftest::seed_count(seeds) {
            let mut a = crate::difftest::fresh();
            let mut b = crate::difftest::fresh();
            setup(&mut a, seed);
            setup(&mut b, seed);
            generated(&mut a);
            hand(&mut b);
            assert_eq!(dmac(&mut a), dmac(&mut b), "seed {seed}: DMAC registers");
            if scratch {
                let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
                assert!(r.soft.is_empty(), "seed {seed} (scratch): {:#?}", r.soft);
            }
        }
    }

    /// Gate word for [`fake_wait_chan_idle`] (in the machine's RAM).
    const IDLE_GATE: u32 = DATA + 0x3000;
    const IDLE_MAGIC: u32 = 0x4944_4c45;

    /// `libdma_WaitChanIdle` with its gate set: returns at once, leaving the
    /// channel's start bit as it was (the real wait spins until it clears).
    fn fake_wait_chan_idle(c: &mut Cpu) {
        if c.bus.read32(IDLE_GATE) != IDLE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_wait_chan_idle),
                ee::s15::f_003fde30__generated,
            );
        }
        c.cycles += 4;
        ret(c);
    }

    /// A random CHCR at [`DMA_CHAN`]: with the start bit set and
    /// WaitChanIdle returning at once when `busy`, else idle.
    fn chan_word(c: &mut Cpu, r: &mut Rng, busy: bool) {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FDE30),
            fake_wait_chan_idle,
            crate::difftest::fakes::prev_cell(fake_wait_chan_idle),
        );
        if busy {
            c.bus.write32(IDLE_GATE, IDLE_MAGIC);
            c.bus.write32(DMA_CHAN, r.next_u32() | 0x100);
        } else {
            c.bus.write32(DMA_CHAN, r.next_u32() & !0x100);
        }
    }

    driven!(
        libdma_wait_chan_idle_gen_driven,
        ee::s15::f_003fde30__generated
    );

    driven!(libdma_wait_chan_idle_hand_driven, libdma_wait_chan_idle);

    driven!(libdma_wait_madr_gen_driven, ee::s15::f_003fdea0__generated);

    driven!(libdma_wait_madr_hand_driven, libdma_wait_madr);

    driven!(libdma_zero_bytes_gen_driven, ee::s15::f_003fdf50__generated);

    driven!(libdma_zero_bytes_hand_driven, libdma_zero_bytes);

    /// EE timer 1 counter (T1_COUNT), read live by `lw`: a status word that
    /// changes on its own while a wait polls it.
    pub(crate) const TIMER1_COUNT: u32 = 0x1000_0800;

    /// Start EE timer 1 counting up from `count` now: bus clock (one tick
    /// per two EE cycles), no compare or overflow interrupts.
    pub(crate) fn timer1_counting_from(c: &mut Cpu, count: u32) {
        let now = c.cycles;
        let t = &mut ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.timers[1];
        t.mode = 0x80;
        t.comp = 0;
        t.count_at_base = count;
        t.base = now;
        t.irq_at = None;
    }

    /// `void libdma_WaitChanIdle(chan)`: the busy spin with real events.
    /// The status word is timer 1's counter started between 0x100 and
    /// 0x1a8: bit 8 stays set for about 28-85 passes, then the wait ends.
    #[test]
    fn libdma_wait_chan_idle_events() {
        check_events(
            libdma_wait_chan_idle_gen_driven,
            libdma_wait_chan_idle_hand_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1250_0015, 0);
                c.pc = 0x003f_de30;
                c.g[A0][0] = TIMER1_COUNT as u64;
                let mode = schedule(c, &mut r, seed, 0x003f_de30);
                timer1_counting_from(c, 0x100 + 0x18 * (seed / 3 % 8));
                mode
            },
        );
    }

    /// `void libdma_WaitMadr(chan, pos, count)`: the MADR spin with real
    /// events, default and explicit spin budgets. MADR (`chan+0x10`) is
    /// timer 1's counter; `pos` is 40-320 ticks ahead of it, so the wait
    /// runs about 11-105 passes and ends before either budget runs out.
    #[test]
    fn libdma_wait_madr_events() {
        check_events(
            libdma_wait_madr_gen_driven,
            libdma_wait_madr_hand_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1250_0014, 0);
                c.pc = 0x003f_dea0;
                c.g[A0][0] = (TIMER1_COUNT - 0x10) as u64;
                c.g[A2][0] = if seed / 3 % 2 == 0 { 0 } else { 0x1357 };
                let start = 0x0101 | (r.next_u32() & 0x0ffe);
                let mode = schedule(c, &mut r, seed, 0x003f_dea0);
                timer1_counting_from(c, start);
                c.g[A1][0] = (start + 40 + 40 * (seed / 6 % 8)) as u64;
                mode
            },
        );
    }

    /// `void libdma_ZeroBytes(dst, len)`: 8-64 bytes zeroed at an odd
    /// address with real events in the byte loop.
    #[test]
    fn libdma_zero_bytes_events() {
        check_events(
            libdma_zero_bytes_gen_driven,
            libdma_zero_bytes_hand_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1250_0012, 0);
                c.pc = 0x003f_df50;
                for i in 0..0x50 {
                    c.bus.write8(ZERO_BUF + i, (i as u8).wrapping_add(1));
                }
                c.g[A0][0] = (ZERO_BUF + 1 + 2 * (seed / 24 % 4)) as u64;
                c.g[A1][0] = (8 + 8 * (seed / 3 % 8)) as u64;
                schedule(c, &mut r, seed, 0x003f_df50)
            },
        );
    }
}
