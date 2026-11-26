//! Staging module oc1320 (sceio-1 wave, SIF bring-up): hand-written
//! replacements, each with its differential test against the generated body.
//! Moved into the source-tree units by `tools/reorg_modules.py apply oc1320`.

use crate::call::{guest, Callee};
use crate::difftest::fakes::covered;
use crate::difftest::DATA;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_6078, _sce_sif_send_cmd, "_sceSifSendCmd"),
    (0x0042_6408, sce_sif_init_rpc, "sceSifInitRpc"),
];

/// `sceFsInit` (0x00427a90) is a resumable dispatcher: converted with
/// `tools/port_dispatcher.py` at install, not hand-ported here.
pub(crate) mod oc1320_callee {
    use super::ee;
    use super::Callee;

    pub(super) use crate::game::lib::cdvd::callee::ADD_CMD_HANDLER;
    pub(super) use crate::game::lib::kernel::DINTR;
    pub(super) use crate::game::lib::sif::callee::EINTR;
    pub(super) use crate::game::lib::sif::callee::GET_REG;
    pub(super) use crate::game::lib::sif::callee::GET_SREG;
    pub(super) use crate::game::lib::sif::callee::SEND_CMD;
    pub(super) use crate::game::lib::sif::callee::SET_DMA;
    pub(super) use crate::game::lib::sif::callee::WRITEBACK;

    /// `sceSifInitCmd` (`0x00425d38`).
    pub(crate) const OC1320_INIT_CMD: Callee = Callee {
        addr: 0x0042_5d38,
        f: ee::s15::f_00425d38,
    };

    /// `isceSifSetDma` (`0x00424170`).
    pub(crate) const OC1320_ISET_DMA: Callee = Callee {
        addr: 0x0042_4170,
        f: ee::s15::f_00424170,
    };
}

/// `_sceSifSendCmd` (0x00426078): build the SIF send packet at the caller's
/// buffer and kick the DMA off. Sizes below 0x71 go inline; the length word
/// decides between the packet fill, the empty store and the early exit; the
/// s1 flag bit 0 picks the syscall DMA start.
pub fn _sce_sif_send_cmd(c: &mut Cpu) {
    use oc1320_callee::*;
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[S4][0]);
        c.g[S1][0] = c.g[A3][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[S3][0]);
        c.g[S4][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S0][0]);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x10));
        c.g[S3][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[S2][0]);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[T0][0];
        c.g[V0][0] = (c.g[V0][0] < 0x61u64) as u64;
        c.g[A1][0] = c.g[T2][0];
        // Small sizes go inline; anything bigger leaves through the early
        // exit with 0 (the delay-slot `A1 = T2` runs on both paths).
        if c.g[V0][0] == 0u64 {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            // Join `break 'b_42618c`: the shared restore below.
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
            c.cycles += 1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        } else {
            if (c.g[A1][0] as i64) <= 0 {
                // Empty packet: clear the header words.
                c.cycles += 2 + 3;
                c.g[S2][0] = 0u64;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                // Generated `+1` block.
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[S2][0] as u32) << 4);
            } else {
                // Fill the packet header.
                c.cycles += 2 + 12;
                c.g[V0][0] = c.lbu(c.g[S0][0] as u32);
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 8);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[T1][0]);
                c.g[S2][0] = 0x1;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.g[S3][0] & 0x4;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                // (the `A1 = S2 << 4` below runs unless the write-back arm
                // takes it first; the delay-slot store runs on both).
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                if c.g[V1][0] != 0u64 {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00426100);
                    WRITEBACK.call(c, 0x00426100)?;
                    // The write-back arm rejoins past the `+1` block.
                    c.g[A1][0] = sx((c.g[S2][0] as u32) << 4);
                } else {
                    // Generated `+1` block.
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[S2][0] as u32) << 4);
                }
            }
            // Commit the packet to the send area and kick the transfer.
            c.cycles += 23;
            c.g[V0][0] = 0x530000;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x4320));
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[S0][0]);
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[S1][0]);
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S4][0]);
            c.sb(c.g[S0][0] as u32, c.g[S1][0]);
            c.g[V0][0] = 0x44;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00426168);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            WRITEBACK.call(c, 0x00426168)?;
            c.g[V0][0] = c.g[S3][0] & 0x1;
            c.g[A1][0] = c.g[S2][0];
            if c.g[V0][0] == 0u64 {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0042618c);
                c.g[A0][0] = c.g[SP][0];
                SET_DMA.call(c, 0x0042618c)?;
                // Join `break 'b_42618c`: the shared restore below.
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
                c.cycles += 1;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0042617c);
                c.g[A0][0] = c.g[SP][0];
                OC1320_ISET_DMA.call(c, 0x0042617c)?;
                // Joins the epilogue directly (`break 'b_426190`).
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
            }
        }
        c.cycles += 7;
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceSifInitRpc(void)` (0x00426408): bring up SIF RPC. The first call
/// disables interrupts, runs `sceSifInitCmd`, registers the four command
/// handlers and re-enables interrupts; then it waits for the IOP side
/// (spins on SREG 0 until the peer answers) and records the SIF register.
/// Later calls return at once through the client-word early exit.
pub fn sce_sif_init_rpc(c: &mut Cpu) {
    use oc1320_callee::*;
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[RA][0] = sx(0x00426420);
        c.sd(c.g[SP][0] as u32, c.g[S0][0]);
        DINTR.call(c, 0x00426420)?;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5180));
        c.g[S1][0] = 0x1;
        // The client word is set after the first bring-up: later calls
        // take the early exit (the `S1 = 1` runs on both paths).
        if c.g[V0][0] == 0u64 {
            c.cycles += 60;
            c.g[RA][0] = sx(0x00426450);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x5180), c.g[S1][0]);
            EINTR.call(c, 0x00426450)?;
            c.g[RA][0] = sx(0x00426458);
            OC1320_INIT_CMD.call(c, 0x00426458)?;
            c.g[RA][0] = sx(0x00426460);
            DINTR.call(c, 0x00426460)?;
            c.g[V1][0] = 0x530000;
            c.g[T0][0] = 0x530000;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4180));
            c.g[A2][0] = 0x530000;
            c.g[A3][0] = 0x530000;
            c.g[S0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x2980));
            c.g[V1][0] = 0x20;
            c.g[V0][0] = 0x20000000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3980));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3180));
            c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
            c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
            c.g[V0][0] = c.g[S2][0] | c.g[V0][0];
            c.sw((c.g[T0][0] as u32).wrapping_sub(0x2980), c.g[S1][0]);
            c.g[A1][0] = 0x420000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A2][0]);
            c.g[A0][0] = 0xffffffff80000000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6708));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
            c.g[A0][0] = c.g[A0][0] | 0x8;
            c.g[A2][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[RA][0] = sx(0x004264d4);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
            ADD_CMD_HANDLER.call(c, 0x004264d4)?;
            c.g[A1][0] = 0x420000;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6a98));
            c.g[A0][0] = c.g[A0][0] | 0x9;
            c.g[RA][0] = sx(0x004264ec);
            c.g[A2][0] = c.g[S0][0];
            ADD_CMD_HANDLER.call(c, 0x004264ec)?;
            c.g[A1][0] = 0x420000;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c88));
            c.g[A0][0] = c.g[A0][0] | 0xa;
            c.g[RA][0] = sx(0x00426504);
            c.g[A2][0] = c.g[S0][0];
            ADD_CMD_HANDLER.call(c, 0x00426504)?;
            c.g[A1][0] = 0x420000;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6820));
            c.g[A2][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0042651c);
            c.g[A0][0] = c.g[A0][0] | 0xc;
            ADD_CMD_HANDLER.call(c, 0x0042651c)?;
            c.g[RA][0] = sx(0x00426524);
            EINTR.call(c, 0x00426524)?;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[RA][0] = sx(0x00426530);
            c.g[A0][0] = c.g[A0][0] | 0x2;
            GET_REG.call(c, 0x00426530)?;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            // A live SIF register means the peer answered: restore and
            // return (the delay-slot `RA` restore runs on both paths).
            if c.g[V0][0] != 0u64 {
                c.cycles += 5;
                c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t;
                return Ok(());
            }
            // Otherwise send the init packet and wait for the peer.
            c.cycles += 10;
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = 0xffffffff80000000;
            c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[S1][0]);
            c.g[A0][0] = c.g[A0][0] | 0x2;
            c.g[A2][0] = 0x10;
            c.g[A3][0] = 0u64;
            c.g[T0][0] = 0u64;
            c.g[RA][0] = sx(0x0042655c);
            c.g[T1][0] = 0u64;
            SEND_CMD.call(c, 0x0042655c)?;
            loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00426568);
                c.g[A0][0] = 0u64;
                GET_SREG.call(c, 0x00426568)?;
                let waiting = c.g[V0][0] == 0u64;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                if waiting {
                    if event_due(c) {
                        deliver_event_here(c, 0x00426560);
                    }
                    continue;
                }
                break;
            }
            c.cycles += 8;
            c.g[A0][0] = 0xffffffff80000000;
            c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = 0x1;
            c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.g[A0][0] | 0x2;
            c.g[S0][0] = c.ld(c.g[SP][0] as u32);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            // Tail call: SetReg returns straight to our caller.
            c.pc = 0x004241a0;
            (ee::s15::sceSifSetReg)(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        // Tail call: EIntr returns straight to our caller.
        c.pc = 0x0042c0c0;
        (ee::s15::EIntr)(c);
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;

    pub(crate) const SEEDS: u32 = 200;

    /// Client word (`0x0045_5180`, RAM: the early exit is testable) and the
    /// SREG polled words (`0x0052_be00`, RAM but zeroed by `sceSifInitCmd`
    /// during bring-up, so the poll loop always spins in-harness).
    pub(crate) const SIF_CLIENT_WORD: u32 = 0x0045_5180;
    pub(crate) const SIF_SREG0: u32 = 0x0052_be00;

    /// Common bring-up fixture: game-like machine, client word steered by
    /// `seed % 2` (nonzero: the early exit; zero: the full bring-up, which
    /// leaves with the peer live since the register read comes back
    /// nonzero). The SREG write sticks but `sceSifInitCmd` zeroes the word
    /// during bring-up, so only the live arm's test can complete (see
    /// branch-tables for the unreachable send path).
    pub(crate) fn init_rpc_setup(c: &mut Cpu, seed: u32) -> Rng {
        let r = machine(c, seed, 0x0042_6408, 4);
        c.pc = 0x0042_6408;
        c.bus
            .write32(SIF_CLIENT_WORD, if seed % 2 == 0 { 0x77 } else { 0 });
        c.bus.write32(SIF_SREG0, 1);
        r
    }

    /// `sceSifInitRpc`: the early exit and the live-peer bring-up.
    #[test]
    fn sce_sif_init_rpc_matches() {
        covered(ee::s15::f_00426408, sce_sif_init_rpc, SEEDS, &|c, seed| {
            init_rpc_setup(c, seed);
        });
    }

    /// `_sceSifSendCmd`: size classes around the 0x71 inline limit; length
    /// classes (empty via 0 and a real -1, plain, write-back via bit 2);
    /// the s1 flag (bit 0) picks the DMA start; every steering value from
    /// an independent digit. (A zero-extended `0x00000000fffffffc` would be
    /// a huge positive length and explode the write-back range; only a
    /// properly sign-extended -1 takes the empty arm.)
    #[test]
    fn _sce_sif_send_cmd_matches() {
        covered(ee::s15::f_00426078, _sce_sif_send_cmd, SEEDS, &|c, seed| {
            machine(c, seed, 1_320_001, 4);
            c.pc = 0x0042_6078;
            // Size digit: packet body (0x10, 0x70), early exit (0x71,
            // 0x1000). Length digit: empty (0 and sign-extended -1),
            // plain (8), with the cache write-back (12).
            c.g[A3][0] = [0x10, 0x70, 0x71, 0x1000][seed as usize % 4] as u64;
            c.g[A1][0] = [0, 0xffff_ffff_fffffffcu64, 8, 12][seed as usize / 4 % 4];
            // Packet pointer and data: real buffers, nonzero fill.
            c.g[A2][0] = (DATA + 0x100) as u64;
            c.g[A0][0] = (DATA + 0x300) as u64;
            c.g[T0][0] = (DATA + 0x300) as u64;
            c.g[T1][0] = (seed.wrapping_mul(0x9e37_79b9) | 0x0101_0100) as u64;
            c.g[T2][0] = c.g[A1][0];
            for i in 0..64 {
                c.bus.write32(DATA + 0x100 + 4 * i, 0x0101_0100 + seed);
            }
        });
    }

    /// `_sceSifSendCmd`: the s1 flag (bit 0) picks the DMA start on seeds
    /// where the packet path runs.
    #[test]
    fn _sce_sif_send_cmd_s1_flag_matches() {
        covered(ee::s15::f_00426078, _sce_sif_send_cmd, SEEDS, &|c, seed| {
            machine(c, seed, 1_320_002, 4);
            c.pc = 0x0042_6078;
            c.g[A3][0] = 0x20;
            c.g[A1][0] = 8;
            // s1 flag digit: bit 0 clear (syscall DMA start) vs set
            // (inline DMA start). Crossed with the odd/even seed.
            c.g[A1][0] |= seed as u64 & 1;
            c.g[A2][0] = (DATA + 0x100) as u64;
            c.g[A0][0] = (DATA + 0x300) as u64;
            c.g[T0][0] = (DATA + 0x300) as u64;
            c.g[T1][0] = (seed.wrapping_mul(0x9e37_79b9) | 0x0101_0100) as u64;
            c.g[T2][0] = c.g[A1][0];
            for i in 0..64 {
                c.bus.write32(DATA + 0x100 + 4 * i, 0x0101_0100 + seed);
            }
        });
    }
}
