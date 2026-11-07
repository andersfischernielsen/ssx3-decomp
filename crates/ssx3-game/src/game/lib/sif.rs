//! `src/lib/sif.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
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
    (0x0042_4140, sce_sif_dma_stat, "sceSifDmaStat"),
    (0x0042_4160, sce_sif_set_dma, "sceSifSetDma"),
    (0x0042_4190, isce_sif_set_dchain, "isceSifSetDChain"),
    (
        0x0042_6230,
        sce_sif_cmd_intr_handler,
        "sceSifCmdIntrHandler",
    ),
];

/// The `syscall` trap: `pc = ret`, invoke the hook at `at`; `false` when the
/// hook unwound (a thread switch mid-syscall).
#[inline]
pub(crate) fn do_syscall(c: &mut Cpu, at: u32, ret: u32) -> bool {
    c.pc = ret;
    c.syscall(at);
    c.pc == ret
}

/// The final due-event check of a syscall wrapper (result ignored, as in the
/// compiled code).
#[inline]
pub(crate) fn event_if_due(c: &mut Cpu) {
    if event_due(c) {
        deliver_event_here(c, c.pc);
    }
}

/// One `syscall` wrapper: 4 cycles, `v1 = code`, trap at `at`, return to
/// `ret`, then a due-event check and `jr ra`.
#[inline]
pub(crate) fn syscall_wrapper(c: &mut Cpu, code: u32, at: u32, ret: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx(code);
    if !do_syscall(c, at, ret) {
        return;
    }
    event_if_due(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Kernel syscall wrappers.

/// `int sceSifSetDma(SifDmaTransfer* t, int len)` (0x424160): syscall 0x77.
pub fn sce_sif_set_dma(c: &mut Cpu) {
    syscall_wrapper(c, 0x77, 0x0042_4164, 0x0042_4168);
}

/// `int isceSifSetDChain(void)` (0x424190): syscall -0x78 (0xffffff88).
pub fn isce_sif_set_dchain(c: &mut Cpu) {
    syscall_wrapper(c, (-120i32) as u32, 0x0042_4194, 0x0042_4198);
}

/// `int sceSifDmaStat(unsigned int id)` (0x424140): syscall 0x76.
pub fn sce_sif_dma_stat(c: &mut Cpu) {
    syscall_wrapper(c, 0x76, 0x0042_4144, 0x0042_4148);
}

pub(crate) mod sif_cmd {

    pub const DATA: u32 = 0x0052_bcd8;

    /// Pointer to the receive buffer (first byte: packet size).
    pub const RECV: u32 = 0x0;

    /// System command handlers (`{func, data}` pairs) and their count.
    pub const SYS_TABLE: u32 = 0xc;

    pub const SYS_COUNT: u32 = 0x10;

    /// User command handlers and their count.
    pub const USR_TABLE: u32 = 0x14;

    pub const USR_COUNT: u32 = 0x18;

    /// Packet header: the command id (bit 31: system command).
    pub const PKT_CID: u32 = 0x8;
}

/// `int sceSifCmdIntrHandler(void)` (0x426230, libkernel): the SIF DMA
/// interrupt handler. Re-enables interrupts, and if a packet arrived (its
/// size byte non-zero) copies it to the stack (rounded up to quadwords),
/// clears the size byte, re-arms the receive chain (`isceSifSetDChain`),
/// and dispatches the command id to the system (bit 31 set) or user handler
/// table (`func(packet, data)`) when it is in range and set; then `ei`.
/// Returns 0.
pub fn sce_sif_cmd_intr_handler(c: &mut Cpu) {
    use sif_cmd::*;
    const EINTR: Callee = Callee {
        addr: 0x0042_c0c0,
        f: ee::s15::f_0042c0c0,
    };
    const SET_DCHAIN: Callee = Callee {
        addr: 0x0042_4190,
        f: ee::s15::f_00424190,
    };
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x90);
        let at = frame.at(c, 0x70);
        c.sd(at, c.g[S0][0]);
        frame.save_ra(c, 0x80);
        EINTR.call(c, 0x0042_6244)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A3][0] = Addr(DATA + RECV).lw(c);
        c.g[S0][0] = sx(DATA);
        let recv = r32(c, A3);
        c.g[V0][0] = Addr(recv).lbu(c);
        c.g[A1][0] = c.g[V0][0] & 0xff;
        let size = c.g[A1][0];
        c.g[V0][0] = 0;
        if size != 0 {
            // Copy the packet (size rounded up to 16) to the stack.
            c.cycles += 10;
            c.g[V0][0] = sx((size as u32).wrapping_add(0xf));
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = sx((size as u32).wrapping_add(0x1e));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.g[A0][0] = c.g[V0][0];
            }
            c.g[A2][0] = c.g[A3][0];
            c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 4) as u32);
            Addr(recv).sb(c, 0);
            let quads = c.g[A1][0] as i64;
            c.g[A0][0] = c.g[A1][0];
            if quads > 0 {
                c.cycles += 2;
                c.g[V1][0] = c.g[SP][0];
                loop {
                    c.cycles += 7;
                    c.lq(V0, r32(c, A2));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.sq(V0, r32(c, V1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0042_6290);
                    }
                }
            }

            c.cycles += 5;
            SET_DCHAIN.call(c, 0x0042_62b4)?;
            let pkt = frame.at(c, 0);
            c.g[V1][0] = Addr(pkt + PKT_CID).lw(c);
            let system = (c.g[V1][0] as i64) < 0;
            // (table, count, return address) of the handler table used.
            let handler = if system {
                c.cycles += 8;
                c.g[V0][0] = Addr(pkt + PKT_CID).lw(c);
                c.g[V1][0] = 0x7fff_ffff;
                c.g[A0][0] = Addr(DATA + SYS_COUNT).lw(c);
                c.g[A1][0] = c.g[V0][0] & c.g[V1][0];
                (SYS_TABLE, 0x0042_6300u32, 5u64)
            } else {
                c.cycles += 5;
                c.g[A1][0] = Addr(pkt + PKT_CID).lw(c);
                c.g[V0][0] = Addr(DATA + USR_COUNT).lw(c);
                (USR_TABLE, 0x0042_633c, 3)
            };
            let (table, ret_to, call_cycles) = handler;
            // The count is in a0 (system) or v0 (user); the compare result
            // replaces it.
            let count_reg = if system { A0 } else { V0 };
            let in_range = (c.g[A1][0] as i64) < (c.g[count_reg][0] as i64);
            c.g[count_reg][0] = in_range as u64;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
            if in_range {
                c.cycles += 5;
                c.g[V1][0] = Addr(DATA + table).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                let entry = r32(c, V0);
                c.g[A2][0] = Addr(entry).lw(c);
                if c.g[A2][0] != 0 {
                    c.cycles += call_cycles;
                    c.g[A1][0] = Addr(entry + 4).lw(c);
                    c.g[A0][0] = c.g[SP][0];
                    let func = r32(c, A2);
                    call_ptr(c, func, ret_to)?;
                }
            }
            c.cycles += 3;
            c.ei();
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x80);
        let at = frame.at(c, 0x70);
        c.g[S0][0] = c.ld(at);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::put_words;

    use crate::difftest::DATA;

    pub(crate) const SEEDS: u32 = 200;

    pub(crate) fn syscall_test(gen: GuestFn, hand: GuestFn, salt: u64) {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, salt, 1);
            for i in 0..4 {
                c.g[[A0, A1, A2, A3][i]][0] = (DATA + (r.next_u32() & 0xff0)) as u64;
            }
        };
        covered(gen, hand, SEEDS, &setup);
    }

    // ------------------------------------------------------------ syscalls

    #[test]
    fn sce_sif_set_dma_matches() {
        syscall_test(ssx3_ee::s15::f_00424160__generated, sce_sif_set_dma, 34);
    }

    #[test]
    fn isce_sif_set_dchain_matches() {
        syscall_test(ssx3_ee::s15::f_00424190__generated, isce_sif_set_dchain, 35);
    }

    #[test]
    fn sce_sif_dma_stat_matches() {
        syscall_test(ssx3_ee::s15::f_00424140__generated, sce_sif_dma_stat, 38);
    }

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;

    /// sceSifCmdIntrHandler: no packet (size 0) or a packet of 1..0x70
    /// bytes; system or user command ids in range or not, handlers set or
    /// null. The SetDChain syscall and the handler are fakes (sometimes
    /// unwinding); EIntr runs for real.
    #[test]
    fn sce_sif_cmd_intr_handler_matches() {
        covered(
            ee::s15::f_00426230__generated,
            sce_sif_cmd_intr_handler,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x426230, 2);
                c.pc = 0x0042_6230;
                c.cop0[12] = r.next_u32();
                let recv = DATA;
                let (sys, usr) = (DATA + 0x200, DATA + 0x300);
                fill(c, &mut r, recv, 0x20);
                let size = if seed % 6 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x70
                };
                c.bus.write8(recv, size as u8);
                let n = r.next_u32() % 8;
                let cid = match r.next_u32() % 4 {
                    0 => 0x8000_0000 | n,
                    1 => n,
                    2 => 0x8000_0000 | (5 + r.next_u32() % 8),
                    _ => 5 + r.next_u32() % 8,
                };
                c.bus.write32(recv + sif_cmd::PKT_CID, cid);
                put_words(c, sif_cmd::DATA, &[recv]);
                c.bus.write32(sif_cmd::DATA + sif_cmd::SYS_TABLE, sys);
                c.bus.write32(sif_cmd::DATA + sif_cmd::SYS_COUNT, 6);
                c.bus.write32(sif_cmd::DATA + sif_cmd::USR_TABLE, usr);
                c.bus.write32(sif_cmd::DATA + sif_cmd::USR_COUNT, 6);
                for k in 0..8 {
                    let f = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        FAKE + 0x10 * k
                    };
                    c.bus.write32(sys + 8 * k, f);
                    c.bus.write32(sys + 8 * k + 4, r.next_u32());
                    let f = if r.next_u32() % 5 == 0 {
                        0
                    } else {
                        FAKE + 0x10 * k
                    };
                    c.bus.write32(usr + 8 * k, f);
                    c.bus.write32(usr + 8 * k + 4, r.next_u32());
                }
            },
        );
    }
}
