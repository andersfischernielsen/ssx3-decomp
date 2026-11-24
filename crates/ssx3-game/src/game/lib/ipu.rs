//! `src/lib/ipu.cpp`: hand-written IPU (`sceIpu*`, `libipu_*`) replacements.

use crate::abi::ret;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_9310, libipu_set_from_chcr, "libipu_SetFromChcr"),
    (0x0040_9388, libipu_set_to_chcr, "libipu_SetToChcr"),
    (0x0040_9400, sce_ipu_stop_dma, "sceIpuStopDMA"),
    (0x0040_94e8, sce_ipu_restart_dma, "sceIpuRestartDMA"),
    (0x0040_9638, sce_ipu_sync, "sceIpuSync"),
    (0x0040_96a0, libipu_init_to_chcr, "libipu_InitToChcr"),
    (0x0040_9708, sce_ipu_init, "sceIpuInit"),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `libipu_SetFromChcr` (`0x0040_9310`).
    pub(crate) const SET_FROM_CHCR: Callee = Callee {
        addr: 0x0040_9310,
        f: ee::s15::libipu_SetFromChcr,
    };

    /// `libipu_SetToChcr` (`0x0040_9388`).
    pub(crate) const SET_TO_CHCR: Callee = Callee {
        addr: 0x0040_9388,
        f: ee::s15::libipu_SetToChcr,
    };

    /// `libipu_InitToChcr` (`0x0040_96a0`).
    pub(crate) const INIT_TO_CHCR: Callee = Callee {
        addr: 0x0040_96a0,
        f: ee::s15::libipu_InitToChcr,
    };

    pub(super) use crate::game::lib::cdvd::callee::DINTR;

    pub(super) use crate::game::lib::sif::callee::EINTR;
}

/// `void libipu_InitToChcr(unsigned int chcr)` (0x4096a0): program the to-IPU
/// DMA channel control (`*(0x1000b400) = chcr`) with every DMAC channel
/// suspended (the `0x10000` bit set through `D_ENABLEW` `0x1000f590` and
/// cleared after), then call `EIntr` unconditionally.
pub fn libipu_init_to_chcr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_96b4);
        c.g[S0][0] = c.g[A0][0];
        callee::DINTR.call(c, 0x0040_96b4)?;
        c.g[A1][0] = 0x1000_0000;
        c.g[A3][0] = 0x1_0000;
        c.g[A1][0] |= 0xf520;
        c.g[A2][0] = 0x1000_0000;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A2][0] |= 0xf590;
        c.g[V1][0] = 0x1000_0000;
        c.g[A0][0] = 0xffff_ffff_fffe_0000;
        c.g[V0][0] |= c.g[A3][0];
        c.g[V1][0] |= 0xb400;
        Addr(c.g[A2][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] |= 0xffff;
        Addr(c.g[V1][0] as u32).sw(c, c.g[S0][0]);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        c.g[V0][0] &= c.g[A0][0];
        Addr(c.g[A2][0] as u32).sw(c, c.g[V0][0]);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        c.pc = callee::EINTR.addr;
        (callee::EINTR.f)(c);
        Ok(())
    })
}

/// `void libipu_SetFromChcr(unsigned int chcr)` (0x409310): program the
/// from-IPU DMA channel control (`*(0x1000b000) = chcr`) with every DMAC
/// channel suspended: sets the suspend bit (`0x10000`) through `D_ENABLEW`
/// (`0x1000f590`, from `D_ENABLER` `0x1000f520`) before the write and clears
/// it after. Interrupts are re-enabled only when `DIntr` found them on.
pub fn libipu_set_from_chcr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_9324);
        c.g[S0][0] = c.g[A0][0];
        callee::DINTR.call(c, 0x0040_9324)?;
        c.g[A2][0] = 0x1000_0000;
        c.g[A3][0] = 0x1000_0000;
        c.g[A2][0] |= 0xf520;
        c.g[T0][0] = 0x1_0000;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A3][0] |= 0xf590;
        c.g[A0][0] = 0x1000_0000;
        c.g[A1][0] = 0xffff_ffff_fffe_0000;
        c.g[V1][0] |= c.g[T0][0];
        c.g[A0][0] |= 0xb000;
        Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
        c.g[A1][0] |= 0xffff;
        Addr(c.g[A0][0] as u32).sw(c, c.g[S0][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V1][0] &= c.g[A1][0];
        Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
        let ints_were_off = c.g[V0][0] == 0;
        frame.restore_ra(c, 0x10);
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        if ints_were_off {
            frame.close(c);
        } else {
            c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
            c.pc = callee::EINTR.addr;
            (callee::EINTR.f)(c);
        }
        Ok(())
    })
}

/// `void libipu_SetToChcr(unsigned int chcr)` (0x409388): program the to-IPU
/// DMA channel control (`*(0x1000b400) = chcr`) with every DMAC channel
/// suspended: sets the suspend bit (`0x10000`) through `D_ENABLEW`
/// (`0x1000f590`, from `D_ENABLER` `0x1000f520`) before the write and clears
/// it after. Interrupts are re-enabled only when `DIntr` found them on.
pub fn libipu_set_to_chcr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_939c);
        c.g[S0][0] = c.g[A0][0];
        callee::DINTR.call(c, 0x0040_939c)?;
        c.g[A2][0] = 0x1000_0000;
        c.g[A3][0] = 0x1000_0000;
        c.g[A2][0] |= 0xf520;
        c.g[T0][0] = 0x1_0000;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A3][0] |= 0xf590;
        c.g[A0][0] = 0x1000_0000;
        c.g[A1][0] = 0xffff_ffff_fffe_0000;
        c.g[V1][0] |= c.g[T0][0];
        c.g[A0][0] |= 0xb400;
        Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
        c.g[A1][0] |= 0xffff;
        Addr(c.g[A0][0] as u32).sw(c, c.g[S0][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[V1][0] &= c.g[A1][0];
        Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
        let ints_were_off = c.g[V0][0] == 0;
        frame.restore_ra(c, 0x10);
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        if ints_were_off {
            frame.close(c);
        } else {
            c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
            c.pc = callee::EINTR.addr;
            (callee::EINTR.f)(c);
        }
        Ok(())
    })
}

/// `void sceIpuInit(void)` (0x409708): reset the IPU and load its CSC and
/// quantisation tables from `0x453340` into the input FIFO, issuing the
/// SETIQ/SETVQ/SETTH commands between busy-waits on `IPU_CTRL`.
/// On a live machine each poll spins while the IPU is busy; the test
/// machine is idle so every poll exits at once.
pub fn sce_ipu_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_9718);
        c.g[A0][0] = 1;
        callee::INIT_TO_CHCR.call(c, 0x0040_9718)?;
        c.g[V0][0] = 0x1000_0000;
        c.g[V1][0] = 0x4000_0000;
        c.g[V0][0] |= 0x2010;
        c.g[A0][0] = 0x1000_0000;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] |= 0x2010;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_9730);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x1000_0000;
        c.g[V1][0] = 0x1000_0000;
        c.g[V0][0] |= 0x2000;
        c.g[V1][0] |= 0x2010;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_9760);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 27;
        c.g[A1][0] = 0x450000;
        c.g[A0][0] = 0x1000_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3340));
        c.g[A0][0] |= 0x7010;
        c.lq(2, c.g[A1][0] as u32);
        c.g[A2][0] = 0x1000_0000;
        c.g[A3][0] = 0x5000_0000;
        c.g[A2][0] |= 0x2000;
        c.sq(2, c.g[A0][0] as u32);
        c.g[T0][0] = 0x1000_0000;
        c.g[T0][0] |= 0x2010;
        c.lq(3, (c.g[A1][0] as u32).wrapping_add(0x10));
        c.sq(3, c.g[A0][0] as u32);
        c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x20));
        c.sq(2, c.g[A0][0] as u32);
        c.lq(3, (c.g[A1][0] as u32).wrapping_add(0x30));
        c.sq(3, c.g[A0][0] as u32);
        c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x40));
        c.sq(2, c.g[A0][0] as u32);
        c.lq(3, (c.g[A1][0] as u32).wrapping_add(0x40));
        c.sq(3, c.g[A0][0] as u32);
        c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x40));
        c.sq(2, c.g[A0][0] as u32);
        c.lq(3, (c.g[A1][0] as u32).wrapping_add(0x40));
        c.sq(3, c.g[A0][0] as u32);
        Addr(c.g[A2][0] as u32).sw(c, c.g[A3][0]);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_97e8);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x1000_0000;
        c.g[V1][0] = 0x5800_0000;
        c.g[V0][0] |= 0x2000;
        c.g[A0][0] = 0x1000_0000;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] |= 0x2010;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_9820);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 15;
        c.g[V1][0] = 0x450000;
        c.g[A0][0] = 0x1000_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3390));
        c.g[A0][0] |= 0x7010;
        c.lq(2, c.g[V1][0] as u32);
        c.g[A1][0] = 0x1000_0000;
        c.g[A2][0] = 0x6000_0000;
        c.g[A1][0] |= 0x2000;
        c.sq(2, c.g[A0][0] as u32);
        c.g[A3][0] = 0x1000_0000;
        c.g[A3][0] |= 0x2010;
        c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.sq(2, c.g[A0][0] as u32);
        Addr(c.g[A1][0] as u32).sw(c, c.g[A2][0]);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_9878);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x1000_0000;
        c.g[V1][0] = 0xffff_ffff_9000_0000;
        c.g[V0][0] |= 0x2000;
        c.g[A0][0] = 0x1000_0000;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] |= 0x2010;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_98b0);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x1000_0000;
        c.g[V1][0] = 0x4000_0000;
        c.g[V0][0] |= 0x2010;
        c.g[A0][0] = 0x1000_0000;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] |= 0x2010;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_98e8);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x1000_0000;
        c.g[V1][0] = 0x1000_0000;
        c.g[V0][0] |= 0x2000;
        c.g[V1][0] |= 0x2010;
        Addr(c.g[V0][0] as u32).sw(c, 0);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_9918);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void sceIpuRestartDMA(void *params)` (0x4094e8): restart a stalled
/// IPU DMA from the 8-word `params` block (base address, transfer
/// address, counts, channel controls and a flags word). Re-arms the
/// from-channel when words 4 and 5 are set, waits for the IPU to go
/// idle, then re-arms the to-channel when anything remains. Returns
/// early when there is nothing left to move.
pub fn sce_ipu_restart_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        c.g[V1][0] = sx((c.g[V0][0] as u32) >> 8);
        c.g[S3][0] = c.g[V0][0] & 0x7f;
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 16);
        c.g[V1][0] &= 0xf;
        c.g[V0][0] &= 3;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        let no_from = c.g[A2][0] == 0;
        c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        if !no_from {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lw(c);
            let no_from_len = c.g[V0][0] == 0;
            c.g[V0][0] = 0x1000_0000;
            if !no_from_len {
                c.cycles += 9;
                c.g[A0][0] = 0x1000_0000;
                c.g[V0][0] |= 0xb010;
                c.g[A0][0] |= 0xb020;
                Addr(c.g[V0][0] as u32).sw(c, c.g[A2][0]);
                c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lw(c);
                Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x18)).lw(c);
                c.g[RA][0] = sx(0x0040_956c);
                c.g[A0][0] |= 0x100;
                callee::SET_FROM_CHCR.call(c, 0x0040_956c)?;
            }
        }
        c.cycles += 3;
        c.g[V1][0] = 0x1000_0000;
        c.g[V1][0] |= 0x2010;
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_9578);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x1000_0000;
        c.g[V1][0] = 0x1000_0000;
        c.g[V0][0] |= 0x2000;
        c.g[V1][0] |= 0x2010;
        Addr(c.g[V0][0] as u32).sw(c, c.g[S3][0]);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let busy = (c.g[V0][0] as i64) < 0;
            if busy {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_95a8);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 2;
        let nothing_left = c.g[S1][0] == 0;
        frame.restore_ra(c, 0x40);
        if nothing_left {
            c.cycles += 1;
            c.g[S3][0] = c.ld(frame.at(c, 0x30));
        } else {
            c.cycles += 2;
            let nothing_to = c.g[S2][0] == 0;
            c.g[S3][0] = c.ld(frame.at(c, 0x30));
            if !nothing_to {
                c.cycles += 19;
                c.g[V0][0] = 0x1000_0000;
                c.g[A0][0] = 0x1000_0000;
                c.g[V0][0] |= 0xb410;
                c.g[A0][0] |= 0xb430;
                Addr(c.g[V0][0] as u32).sw(c, c.g[S1][0]);
                c.g[V1][0] = 0x1000_0000;
                c.g[V1][0] |= 0xb420;
                c.g[RA][0] = c.ld(frame.at(c, 0x40));
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                c.g[S3][0] = c.ld(frame.at(c, 0x30));
                Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                Addr(c.g[V1][0] as u32).sw(c, c.g[S2][0]);
                c.g[S2][0] = c.ld(frame.at(c, 0x20));
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[S1][0] = c.ld(frame.at(c, 0x10));
                c.g[S0][0] = c.ld(frame.at(c, 0));
                c.g[A0][0] |= 0x100;
                c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
                c.pc = callee::SET_TO_CHCR.addr;
                (callee::SET_TO_CHCR.f)(c);
                return Ok(());
            }
        }
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceIpuStopDMA(void *dump)` (0x409400): stop the IPU DMA,
/// stashing the channel registers into the 9-word `dump` buffer. Spins
/// until the in-flight transfer drains, then clears the from-channel.
pub fn sce_ipu_stop_dma(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x0040_9418);
        c.g[A0][0] = 1;
        callee::SET_TO_CHCR.call(c, 0x0040_9418)?;
        c.g[V0][0] = 0x1000_0000;
        c.g[A2][0] = 0x1000_0000;
        c.g[V0][0] |= 0xb410;
        c.g[A2][0] |= 0xb430;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = 0x1000_0000;
        c.g[A0][0] |= 0xb420;
        c.g[A1][0] = 0x1000_0000;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        c.g[A1][0] |= 0xb400;
        c.g[A3][0] = 0x1000_0000;
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A3][0] |= 0x2010;
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, c.g[V1][0]);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
            c.g[V0][0] &= 0xf0;
            let draining = c.g[V0][0] != 0;
            if draining {
                if event_due(c) {
                    deliver_event_here(c, 0x0040_9460);
                }
                continue;
            } else {
                break;
            }
        }
        c.cycles += 26;
        c.g[RA][0] = sx(0x0040_9484);
        c.g[A0][0] = 0;
        callee::SET_FROM_CHCR.call(c, 0x0040_9484)?;
        c.g[V0][0] = 0x1000_0000;
        c.g[A3][0] = 0x1000_0000;
        c.g[V0][0] |= 0xb010;
        c.g[A3][0] |= 0xb020;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A1][0] = 0x1000_0000;
        c.g[A1][0] |= 0xb000;
        c.g[A2][0] = 0x1000_0000;
        Addr((c.g[S0][0] as u32).wrapping_add(0x10)).sw(c, c.g[V1][0]);
        c.g[A2][0] |= 0x2020;
        c.g[A0][0] = 0x1000_0000;
        frame.restore_ra(c, 0x10);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] |= 0x2010;
        Addr((c.g[S0][0] as u32).wrapping_add(0x14)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        Addr((c.g[S0][0] as u32).wrapping_add(0x18)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        Addr((c.g[S0][0] as u32).wrapping_add(0x20)).sw(c, c.g[V0][0]);
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceIpuSync(int mode)` (0x409638): wait for the IPU. Mode 0 polls
/// `IPU_CTRL` until the busy bit clears and returns 0; mode 1 returns the
/// busy bit (shifted down); any other mode returns 0 at once.
pub fn sce_ipu_sync(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 2;
        let mode0 = c.g[A0][0] == 0;
        c.g[V1][0] = 0;
        if !mode0 {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let mode1 = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if !mode1 {
                c.cycles += 2;
                c.cycles += 2;
                ret(c);
                return Ok(());
            }
            c.cycles += 4;
            c.g[V0][0] = 0x1000_0000;
            c.g[V0][0] |= 0x2010;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 31);
        } else {
            c.cycles += 3;
            c.g[V1][0] = 0x1000_0000;
            c.g[V1][0] |= 0x2010;
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let busy = (c.g[V0][0] as i64) < 0;
                if busy {
                    if event_due(c) {
                        deliver_event_here(c, 0x0040_9660);
                    }
                    continue;
                } else {
                    break;
                }
            }
            c.cycles += 2;
            c.g[V1][0] = 0;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
        c.cycles += 2;
        ret(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fill;
    use crate::difftest::require_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee_rt::Cpu;

    /// InitToChcr: several chcr values, with DIntr off/on (the EIntr tail
    /// always runs). The DMAC registers (D_ENABLE's suspend bit set and
    /// cleared around the store) are compared after the run. Fully covered.
    #[test]
    fn libipu_init_to_chcr_matches() {
        let ipu_init_to_chcr_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            ipu_setup(c, seed, 0x0040_96a0);
            c.g[A0][0] = [0x100u64, 0x104, 0x110, 0x111][seed as usize % 4];
        };
        crate::game::lib::dma::tests::check_dmac(
            ee::s15::f_004096a0__generated,
            libipu_init_to_chcr,
            200,
            false,
            ipu_init_to_chcr_call,
        );
        require_paths(
            ee::s15::f_004096a0__generated,
            200,
            &[39, 46],
            ipu_init_to_chcr_call,
        );
    }

    /// SetFromChcr: several chcr values, with DIntr off (plain restore) or
    /// on (EIntr tail). Fully covered.
    #[test]
    fn libipu_set_from_chcr_matches() {
        let ipu_set_from_chcr_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            ipu_chcr_setup(c, seed, 0x0040_9310);
            c.g[A0][0] = [0x100u64, 0x104, 0x110, 0x1ff][seed as usize % 4];
        };
        check(
            ee::s15::f_00409310__generated,
            libipu_set_from_chcr,
            200,
            ipu_set_from_chcr_call,
        );
        require_paths(
            ee::s15::f_00409310__generated,
            200,
            &[34, 47],
            ipu_set_from_chcr_call,
        );
    }

    /// SetToChcr: several chcr values, with DIntr off (no EIntr) or on
    /// (EIntr tail). Fully covered.
    #[test]
    fn libipu_set_to_chcr_matches() {
        let ipu_set_to_chcr_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            ipu_setup(c, seed, 0x0040_9388);
            c.g[A0][0] = [0x100u64, 0x104, 0x110, 0x1ff][seed as usize % 4];
        };
        check(
            ee::s15::f_00409388__generated,
            libipu_set_to_chcr,
            200,
            ipu_set_to_chcr_call,
        );
        require_paths(
            ee::s15::f_00409388__generated,
            200,
            &[34, 47],
            ipu_set_to_chcr_call,
        );
    }

    /// IpuInit: straight-line table load into the input FIFO; every busy
    /// poll exits at once on the idle test machine. RAM word 0 reads busy
    /// (bit 31 set), so a poll or store that missed IPU_CTRL or IPU_CMD
    /// would show. The spinning legs need a busy IPU and are not covered.
    #[test]
    fn sce_ipu_init_matches() {
        let ipu_init_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            ipu_setup(c, seed, 0x0040_9708);
            c.bus.write32(0, 0x8000_0000);
        };
        check(
            ee::s15::f_00409708__generated,
            sce_ipu_init,
            40,
            ipu_init_call,
        );
        require_paths(
            ee::s15::f_00409708__generated,
            40,
            &[181, 188],
            ipu_init_call,
        );
    }

    /// RestartDMA: the from-channel skipped (word 4 or 5 clear) or
    /// re-armed, and the nothing-left / nothing-to / full-rearm tails.
    /// The busy polls exit at once on the idle test machine. The flags
    /// word is the BCLR bit pointer alone, with a count in bits 8-11 or
    /// 16-17 (one quadword left, so nothing is), with high bits the BCLR
    /// mask drops, or with IPU_CTRL's MP1 bit set (both polls read it).
    #[test]
    fn sce_ipu_restart_dma_matches() {
        let ipu_restart_dma_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            ipu_setup(c, seed, 0x0040_94e8);
            let p = DATA + 0x6000;
            let leg = seed / 12 % 5;
            c.bus.write32(p + 0x04, 0x100 + seed);
            c.bus
                .write32(p + 0x08, if seed % 4 == 1 { 0 } else { 0x20 });
            c.bus.write32(p + 0x0c, 0x110);
            c.bus.write32(p + 0x18, 0x104);
            let flags = match leg {
                1 => 0x100,
                2 => 0x1_0000,
                3 => 0x1000_0000 | seed % 4,
                _ => seed % 4,
            };
            c.bus.write32(p + 0x1c, flags);
            if leg == 4 {
                c.bus.write32(0x1000_2010, 0x0080_0000);
            }
            c.bus
                .write32(p + 0x10, if seed % 3 == 0 { 0 } else { DATA + 0x7100 });
            c.bus.write32(p + 0x14, if seed % 3 == 1 { 0 } else { 8 });
            // S1 = param0 - (V0 << 4), V0 the flags' count (0, or 1 on the
            // count legs, whose word 0 is 0x10): zero leaves early, 0x10
            // with no count skips only the to-channel.
            c.bus.write32(
                p,
                match (leg, seed % 4) {
                    (1 | 2, _) => 0x10,
                    (_, 0) => 0,
                    (_, 1) => 0x10,
                    _ => DATA + 0x7000,
                },
            );
            c.g[A0][0] = p as u64;
        };
        check(
            ee::s15::f_004094e8__generated,
            sce_ipu_restart_dma,
            120,
            ipu_restart_dma_call,
        );
        require_paths(
            ee::s15::f_004094e8__generated,
            120,
            &[51, 52, 54, 55, 98, 100, 103, 110, 113, 116, 146, 172],
            ipu_restart_dma_call,
        );
    }

    /// StopDMA: straight-line dump into the 9-word buffer; the drain poll
    /// exits at once on the idle test machine. The spinning leg is
    /// `sce_ipu_stop_dma_events`.
    #[test]
    fn sce_ipu_stop_dma_matches() {
        let ipu_stop_dma_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            ipu_setup(c, seed, 0x0040_9400);
            c.g[A0][0] = (DATA + 0x5000) as u64;
        };
        check(
            ee::s15::f_00409400__generated,
            sce_ipu_stop_dma,
            40,
            ipu_stop_dma_call,
        );
        require_paths(
            ee::s15::f_00409400__generated,
            40,
            &[125, 151],
            ipu_stop_dma_call,
        );
    }

    /// IpuSync: mode 0 (polls, exits at once when idle), mode 1 (the busy
    /// bit) and other modes (return 0 at once). Fully covered.
    #[test]
    fn sce_ipu_sync_matches() {
        let ipu_sync_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            ipu_setup(c, seed, 0x0040_9638);
            c.g[A0][0] = [0u64, 1, 2, 0xff][seed as usize % 4];
        };
        check(
            ee::s15::f_00409638__generated,
            sce_ipu_sync,
            200,
            ipu_sync_call,
        );
        require_paths(
            ee::s15::f_00409638__generated,
            200,
            &[9, 12, 17],
            ipu_sync_call,
        );
    }

    /// Fresh IPU state: scrambled regs, game gp, `pc` at `entry`, with
    /// interrupts forced off/on by seed (steers the DIntr/EIntr legs).
    pub(crate) fn ipu_setup(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(0x0040_9388 ^ (seed as u64) << 12 ^ entry as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        if seed % 2 == 0 {
            c.cop0[12] |= 0x1_0000;
        } else {
            c.cop0[12] &= !0x1_0000;
        }
        r
    }

    /// IPU DMA-channel fixture: scrambled regs, game gp, `pc` at `entry`,
    /// INTC bit in `cop0r12` flipped by seed so both `DIntr` legs run.
    pub(crate) fn ipu_chcr_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x0040_9310 ^ (seed as u64) << 12 ^ entry as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        if seed % 2 == 0 {
            c.cop0[12] &= !0x1_0000;
        } else {
            c.cop0[12] |= 0x1_0000;
        }
    }

    // Why a custom event hook: the IPU model (crates/ssx3-ps2/src/ipu) runs every
    // command synchronously when it is written and has no clock, and reading
    // IPU_CTRL changes nothing. A busy IPU therefore stays busy for ever on the
    // test machine, and an idle one makes every poll exit before its event check.
    // The fixtures leave the IPU busy only on event seeds, and the hook below
    // finishes its work when the event is taken (as the real IPU does while the
    // CPU waits), so each loop spins until the event lands on its head and then
    // exits on the next pass.

    const IPU_CMD: u32 = 0x1000_2000;
    const IPU_CTRL: u32 = 0x1000_2010;
    const IPU_IN_FIFO: u32 = 0x1000_7010;

    /// The test machine's event hook while the IPU works: by the time an event
    /// is taken the IPU has finished (a reset clears its command and any
    /// undelivered output), then the machine delivers the event as usual.
    /// Installed on both machines alike; it stands in for IPU progress
    /// because the IPU model has no clock and finishes nothing on its own.
    fn ipu_finishes_at_event(c: &mut Cpu) -> bool {
        assert!(
            c.cycles < crate::difftest::CYCLE_BUDGET,
            "cycle budget exceeded ({} cycles, pc {:#x}): runaway on test data",
            c.cycles,
            c.pc
        );
        c.bus.write32(IPU_CTRL, 0x4000_0000);
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    /// Leave the IPU busy on an FDEC waiting for input (the FIFO is empty and
    /// toIPU is stopped): IPU_CTRL bit 31 stays set.
    fn stall_ipu_command(c: &mut Cpu) {
        c.bus.write32(IPU_CMD, 0x4000_0000);
    }

    /// Leave one converted macroblock undelivered with fromIPU stopped: a CSC
    /// of one 384-byte block (24 quadwords of nonzero input) whose 64 output
    /// quadwords stay queued, so IPU_CTRL.OFC reads 8.
    fn leave_ipu_output(c: &mut Cpu, r: &mut Rng) {
        for _ in 0..24 {
            let q = (0..4).fold(0u128, |q, _| q << 32 | (r.next_u32() | 0x0101_0101) as u128);
            c.bus.write128(IPU_IN_FIFO, q);
        }
        c.bus.write32(IPU_CMD, 0x7000_0001);
    }

    driven!(sce_ipu_stop_dma_gen_driven, ee::s15::f_00409400__generated);
    driven!(sce_ipu_stop_dma_hand_driven, sce_ipu_stop_dma);

    /// StopDMA with real events: the dump waits for the IPU's queued output to
    /// drain, and the event lands in that wait.
    #[test]
    fn sce_ipu_stop_dma_events() {
        check_events(
            sce_ipu_stop_dma_gen_driven,
            sce_ipu_stop_dma_hand_driven,
            24,
            &|c, seed| {
                // seed % 3: event mode; seed / 3 % 2: interrupts on/off.
                let mut r = ipu_setup(c, seed / 3, 0x0040_9400);
                let dump = DATA + 0x5024;
                fill(c, &mut r, dump, 9);
                c.g[A0][0] = dump as u64;
                // Channel registers for the dump to copy (no STR: nothing starts).
                for (reg, v) in [
                    (0x1000_b410, DATA + 0x7040),
                    (0x1000_b420, 0x10),
                    (0x1000_b430, DATA + 0x7c44),
                    (0x1000_b010, DATA + 0x7840),
                    (0x1000_b020, 4),
                ] {
                    c.bus.write32(reg, v);
                }
                c.hooks.event = ipu_finishes_at_event;
                let mode = schedule(c, &mut r, seed, 0x0040_9400);
                if mode != Events::Off {
                    leave_ipu_output(c, &mut r);
                }
                mode
            },
        );
    }

    driven!(
        sce_ipu_restart_dma_gen_driven,
        ee::s15::f_004094e8__generated
    );
    driven!(sce_ipu_restart_dma_hand_driven, sce_ipu_restart_dma);

    /// RestartDMA with real events: the wait for the IPU to go idle, after the
    /// from-channel was skipped or re-armed, with each tail after it.
    #[test]
    fn sce_ipu_restart_dma_events() {
        check_events(
            sce_ipu_restart_dma_gen_driven,
            sce_ipu_restart_dma_hand_driven,
            72,
            &|c, seed| {
                // seed % 3: event mode; seed / 3 % 2: interrupts on/off;
                // seed / 6 % 4: tail; seed / 24 % 3: from-channel.
                let mut r = ipu_setup(c, seed / 3, 0x0040_94e8);
                let tail = seed / 6 % 4;
                let from = seed / 24 % 3;
                let p = DATA + 0x6034;
                fill(c, &mut r, p, 8);
                // toIPU source for the full re-arm tail.
                fill(c, &mut r, DATA + 0x7040, 0x80);
                // S1 = word 0 and S2 = word 2 here (flags < 4): 0 leaves
                // early, 0x10 with word 2 clear skips the to-channel, a buffer
                // re-arms it. The flags are the BCLR bit pointer.
                c.bus.write32(
                    p,
                    match tail {
                        0 => 0,
                        1 => 0x10,
                        _ => DATA + 0x7040,
                    },
                );
                c.bus.write32(p + 0x08, if tail == 1 { 0 } else { 0x20 });
                c.bus.write32(p + 0x0c, 0x110);
                c.bus
                    .write32(p + 0x10, if from == 0 { 0 } else { DATA + 0x7840 });
                c.bus.write32(p + 0x14, if from == 1 { 0 } else { 8 });
                c.bus.write32(p + 0x18, 0x104);
                c.bus.write32(p + 0x1c, tail);
                c.g[A0][0] = p as u64;
                c.hooks.event = ipu_finishes_at_event;
                let mode = schedule(c, &mut r, seed, 0x0040_94e8);
                if mode != Events::Off {
                    stall_ipu_command(c);
                }
                mode
            },
        );
    }

    driven!(sce_ipu_sync_gen_driven, ee::s15::f_00409638__generated);
    driven!(sce_ipu_sync_hand_driven, sce_ipu_sync);

    /// IpuSync with real events: mode 0 waits on a busy IPU and the event
    /// lands in that wait; modes 1 and 2 return at once.
    #[test]
    fn sce_ipu_sync_events() {
        check_events(
            sce_ipu_sync_gen_driven,
            sce_ipu_sync_hand_driven,
            48,
            &|c, seed| {
                // seed % 3: event mode; seed / 3 % 2: interrupts on/off;
                // seed / 6 % 4: mode 0 (three in four) or not; seed / 24 % 2:
                // mode 1 or 2.
                let mut r = ipu_setup(c, seed / 3, 0x0040_9638);
                c.g[A0][0] = match seed / 6 % 4 {
                    3 => 1 + (seed / 24 % 2) as u64,
                    _ => 0,
                };
                c.hooks.event = ipu_finishes_at_event;
                let mode = schedule(c, &mut r, seed, 0x0040_9638);
                if mode != Events::Off {
                    stall_ipu_command(c);
                }
                mode
            },
        );
    }

    // No `sce_ipu_init_events`: its event checks sit in the busy arms of its
    // polls, and each poll follows sceIpuInit's own IPU reset or command with
    // no call in between, so the IPU is never busy there and check_events
    // would fail its 1/4 delivery assertion with 0 delivered.
}
