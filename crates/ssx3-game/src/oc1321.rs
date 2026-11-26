//! Staging module oc1321 (sceio-1 wave, SIF syscall stubs): hand-written
//! replacements, each with its differential test against the generated body.
//! Moved into the source-tree units by `tools/reorg_modules.py apply oc1321`.

use crate::call::{guest, Callee};
use crate::difftest::check_paths;
use crate::difftest::fakes::covered;
use crate::difftest::fakes::machine;
use crate::game::lib::sif::syscall_wrapper;
use crate::game::lib::sif::tests::syscall_test;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_4070, sce_sif_stop_dma, "sceSifStopDma"),
    (0x0042_4150, isce_sif_dma_stat, "isceSifDmaStat"),
    (0x0042_4170, isce_sif_set_dma, "isceSifSetDma"),
    (0x0042_4180, sce_sif_set_dchain, "sceSifSetDChain"),
    (0x0042_41a0, sce_sif_set_reg, "sceSifSetReg"),
    (0x0042_41b0, sce_sif_get_reg, "sceSifGetReg"),
    (0x0042_41d0, deci2_call, "Deci2Call"),
    (
        0x0042_4220,
        libkernl_reset_tty_init,
        "libkernl_ResetTtyInit",
    ),
    (0x0042_61f0, isce_sif_send_cmd, "isceSifSendCmd"),
    (0x0042_65a8, sce_sif_exit_rpc, "sceSifExitRpc"),
];

pub(crate) mod oc1321_callee {
    use super::ee;
    use super::Callee;

    /// `_sceSifSendCmd` (`0x00426078`).
    pub(crate) const OC1321_SCE_SIF_SEND_CMD: Callee = Callee {
        addr: 0x0042_6078,
        f: ee::s15::f_00426078,
    };

    /// `sceSifExitCmd` (`0x00425fb8`).
    pub(crate) const OC1321_SCE_SIF_EXIT_CMD: Callee = Callee {
        addr: 0x0042_5fb8,
        f: ee::s15::f_00425fb8,
    };
}

/// `int sceSifStopDma(unsigned int id)` (0x424070): syscall 0x6b.
pub fn sce_sif_stop_dma(c: &mut Cpu) {
    syscall_wrapper(c, 0x6b, 0x0042_4074, 0x0042_4078);
}

/// `int isceSifDmaStat(unsigned int id)` (0x424150): syscall -0x76 (0xffffff8a).
pub fn isce_sif_dma_stat(c: &mut Cpu) {
    syscall_wrapper(c, 0xffffff8a, 0x0042_4154, 0x0042_4158);
}

/// `int isceSifSetDma(SifDmaTransfer* t, int len)` (0x424170): syscall -0x77 (0xffffff89).
pub fn isce_sif_set_dma(c: &mut Cpu) {
    syscall_wrapper(c, 0xffffff89, 0x0042_4174, 0x0042_4178);
}

/// `int sceSifSetDChain(void)` (0x424180): syscall 0x78.
pub fn sce_sif_set_dchain(c: &mut Cpu) {
    syscall_wrapper(c, 0x78, 0x0042_4184, 0x0042_4188);
}

/// `int sceSifSetReg(int reg, unsigned int val)` (0x4241a0): syscall 0x79.
pub fn sce_sif_set_reg(c: &mut Cpu) {
    syscall_wrapper(c, 0x79, 0x0042_41a4, 0x0042_41a8);
}

/// `unsigned int sceSifGetReg(int reg)` (0x4241b0): syscall 0x7a.
pub fn sce_sif_get_reg(c: &mut Cpu) {
    syscall_wrapper(c, 0x7a, 0x0042_41b4, 0x0042_41b8);
}

/// `int Deci2Call(...)` (0x4241d0): syscall 0x7c.
pub fn deci2_call(c: &mut Cpu) {
    syscall_wrapper(c, 0x7c, 0x0042_41d4, 0x0042_41d8);
}

/// `void libkernl_ResetTtyInit(void)` (0x424220): clear the TTY-init flag word.
pub fn libkernl_reset_tty_init(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x450000;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5168), 0u64);
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `isceSifSendCmd` (0x4261f0): repack the SIF send registers around
/// `_sceSifSendCmd` (moves `a1` into the command slot, `a2`/`a3` down one).
pub fn isce_sif_send_cmd(c: &mut Cpu) {
    use oc1321_callee::*;
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
        c.g[RA][0] = sx(0x00426220);
        c.g[A1][0] = 0x1;
        OC1321_SCE_SIF_SEND_CMD.call(c, 0x00426220)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceSifExitRpc(void)` (0x4265a8): run `sceSifExitCmd`, then clear the
/// SIF RPC client word.
pub fn sce_sif_exit_rpc(c: &mut Cpu) {
    use oc1321_callee::*;
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x004265b8);
        OC1321_SCE_SIF_EXIT_CMD.call(c, 0x004265b8)?;
        c.g[V0][0] = 0x450000;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x5180), 0u64);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::DATA;

    pub(crate) const SEEDS: u32 = 200;

    #[test]
    fn sce_sif_stop_dma_matches() {
        syscall_test(ee::s15::f_00424070, sce_sif_stop_dma, 1_321_001);
    }

    #[test]
    fn isce_sif_dma_stat_matches() {
        syscall_test(ee::s15::f_00424150, isce_sif_dma_stat, 1_321_002);
    }

    #[test]
    fn isce_sif_set_dma_matches() {
        syscall_test(ee::s15::f_00424170, isce_sif_set_dma, 1_321_003);
    }

    #[test]
    fn sce_sif_set_dchain_matches() {
        syscall_test(ee::s15::f_00424180, sce_sif_set_dchain, 1_321_004);
    }

    #[test]
    fn sce_sif_set_reg_matches() {
        syscall_test(ee::s15::f_004241a0, sce_sif_set_reg, 1_321_005);
    }

    #[test]
    fn sce_sif_get_reg_matches() {
        syscall_test(ee::s15::f_004241b0, sce_sif_get_reg, 1_321_006);
    }

    #[test]
    fn deci2_call_matches() {
        syscall_test(ee::s15::f_004241d0, deci2_call, 1_321_007);
    }

    /// `libkernl_ResetTtyInit`: the single store of 0 to the TTY-init flag.
    #[test]
    fn libkernl_reset_tty_init_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_321_008, 0);
            c.pc = 0x0042_4220;
            c.bus
                .write32(0x0045_5168, seed.wrapping_mul(0x9e37_79b9) | 1);
        };
        check_paths(
            ee::s15::f_00424220,
            libkernl_reset_tty_init,
            SEEDS,
            &[3],
            &setup,
        );
    }

    /// `isceSifSendCmd`: repack the registers, run `_sceSifSendCmd`, restore.
    /// The wrapper forces `a1 = 1`, so the callee sees `S1 = orig a2`
    /// (size), `S0 = orig a1` (packet pointer), `A1 = orig t2` (length)
    /// and `S3 = 1` (the write-back arm is only reachable through a direct
    /// `_sceSifSendCmd` call, covered in oc1320). Arms: size below/equal/
    /// above the 0x71 limit; length positive, zero (empty arm) and a huge
    /// length (packet arm — a zero-extended `0x00000000fffffffc` is a
    /// large positive length, not -1).
    #[test]
    fn isce_sif_send_cmd_matches() {
        covered(ee::s15::f_004261f0, isce_sif_send_cmd, SEEDS, &|c, seed| {
            machine(c, seed, 1_321_009, 2);
            c.pc = 0x0042_61f0;
            c.g[A0][0] = (DATA + 0x300) as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            // Size digit: 0x10 and 0x70 take the packet body, 0x71 and
            // 0x1000 take the early exit.
            c.g[A2][0] = [0x10, 0x70, 0x71, 0x1000][seed as usize % 4] as u64;
            c.g[A3][0] = (DATA + 0x300) as u64;
            c.g[T0][0] = (seed.wrapping_mul(0x9e37_79b9) | 0x0101_0100) as u64;
            c.g[T1][0] = (seed.wrapping_mul(0x5bd1_e995) | 0x0101_0100) as u64;
            // Length digit: 12 and 8 build a packet, 0 takes the empty
            // arm, 0x00000000fffffffc is a huge positive length (packet
            // arm, not -1: only a sign-extended -1 would be negative).
            c.g[T2][0] = [12, 8, 0, 0xffff_fffcu64][seed as usize / 4 % 4];
            for i in 0..32 {
                c.bus.write32(DATA + 0x100 + 4 * i, 0x0101_0100 + seed);
            }
        });
    }

    /// `sceSifExitRpc`: run `sceSifExitCmd`, clear the RPC client word.
    #[test]
    fn sce_sif_exit_rpc_matches() {
        covered(ee::s15::f_004265a8, sce_sif_exit_rpc, SEEDS, &|c, seed| {
            machine(c, seed, 1_321_010, 2);
            c.pc = 0x0042_65a8;
            c.bus
                .write32(0x0045_5180, seed.wrapping_mul(0x9e37_79b9) | 1);
        });
    }
}
