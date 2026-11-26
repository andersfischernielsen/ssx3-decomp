//! Staging module oc1322 (sceio-1 wave, SIF/FS small wrappers): hand-written
//! replacements, each with its differential test against the generated body.
//! Moved into the source-tree units by `tools/reorg_modules.py apply oc1322`.

use crate::call::{call_ptr, guest, Callee};
use crate::difftest::fakes::FAKE;
use crate::difftest::fakes::{covered, machine};
use crate::difftest::{check_lohi, check_paths, require_paths, DATA};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0042_6050,
        sce_sif_remove_cmd_handler,
        "sceSifRemoveCmdHandler",
    ),
    (
        0x0042_5ce0,
        sce_sif_cmd_change_addr_handler,
        "sceSifCmd_ChangeAddrHandler",
    ),
    (0x0042_5fb8, sce_sif_exit_cmd, "sceSifExitCmd"),
    (0x0042_6698, sce_sif_rpc_get_fpacket, "sceSifRpc_GetFPacket"),
    (
        0x0042_66c8,
        sce_sif_rpc_get_fpacket2,
        "sceSifRpc_GetFPacket2",
    ),
    (
        0x0042_7a60,
        sce_fs_call_func_cmd_handler,
        "sceFs_CallFuncCmdHandler",
    ),
    (0x0042_7d28, sce_fs_reset, "sceFsReset"),
    (0x0042_a2e0, sce_umount, "sceUmount"),
];

/// `sceSifRpcLoop` (0x004273f8) is a resumable dispatcher: converted with
/// `tools/port_dispatcher.py` at install, not hand-ported here.
pub(crate) mod oc1322_callee {
    use super::ee;
    use super::Callee;

    pub(super) use crate::game::lib::sif::callee::MEMSET;

    /// `DisableDmac` (`0x00424950`).
    pub(crate) const OC1322_DISABLE_DMAC: Callee = Callee {
        addr: 0x0042_4950,
        f: ee::s15::f_00424950,
    };

    /// `RemoveDmacHandler` (`0x00423ad0`).
    pub(crate) const OC1322_REMOVE_DMAC_HANDLER: Callee = Callee {
        addr: 0x0042_3ad0,
        f: ee::s15::f_00423ad0,
    };

    /// `sceSifRpc_GetFPacket` (`0x00426698`, same-lane port below).
    pub(crate) const OC1322_GET_FPACKET: Callee = Callee {
        addr: 0x0042_6698,
        f: ee::s15::f_00426698,
    };

    /// `sceFs_PathCmd` (`0x00428e18`, dropped from this lane: never runs).
    pub(crate) const OC1322_PATH_CMD: Callee = Callee {
        addr: 0x0042_8e18,
        f: ee::s15::f_00428e18,
    };
}

/// Command-handler table pointers: the table used for `cid < 0` lives at
/// `0x0052_bce4`, the one for `cid >= 0` at `0x0052_bcec` (each entry
/// `{handler, arg}`, indexed by `cid * 8`; see `sceSifAddCmdHandler`).
/// (Note: the doc comment on `sceSifAddCmdHandler` names these the other
/// way round; the generated code reads `0x52bce4` on the negative arm.)
pub(crate) mod sif_cmd_table {
    /// Table pointer used when `cid < 0` (`0x0052_bce4`).
    pub(crate) const NEG_TABLE: u32 = 0x0052_bce4;
    /// Table pointer used when `cid >= 0` (`0x0052_bcec`).
    pub(crate) const POS_TABLE: u32 = 0x0052_bcec;
}

/// `int sceSifRemoveCmdHandler(int cid)` (0x00426050): clear the handler
/// slot for `cid` in the user (`cid >= 0`) or system table.
pub fn sce_sif_remove_cmd_handler(c: &mut Cpu) {
    use sif_cmd_table::*;
    'b_42606c: {
        'b_426064: {
            c.cycles += 2;
            let t = (c.g[A0][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 3);
            if t {
                break 'b_426064;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x530000;
                let t = true;
                c.g[A0][0] = c.lw(NEG_TABLE);
                if t {
                    break 'b_42606c;
                } else {
                    break 'b_426064;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0x530000;
        c.g[A0][0] = c.lw(POS_TABLE);
        break 'b_42606c;
    }
    c.cycles += 3;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let t = c.g[RA][0] as u32;
    c.sw(c.g[V1][0] as u32, 0u64);
    c.pc = t;
}

/// `void sceSifCmd_ChangeAddrHandler(...)` (0x00425ce0): copy the address
/// word at `a0 + 0x10` to `a1 + 8`.
pub fn sce_sif_cmd_change_addr_handler(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    c.pc = t;
}

/// `int sceSifExitCmd(void)` (0x00425fb8): stop DMA channel 5, drop its
/// handler, and clear the SIF command state word.
pub fn sce_sif_exit_cmd(c: &mut Cpu) {
    use oc1322_callee::*;
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x00425fc8);
        c.g[A0][0] = 0x5;
        OC1322_DISABLE_DMAC.call(c, 0x00425fc8)?;
        c.g[V1][0] = 0x530000;
        c.g[A0][0] = 0x5;
        c.g[RA][0] = sx(0x00425fd8);
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x432c));
        OC1322_REMOVE_DMAC_HANDLER.call(c, 0x00425fd8)?;
        c.g[V1][0] = 0x450000;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x517c), 0u64);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void *sceSifRpc_GetFPacket(...)` (0x00426698): packet slot for the next
/// free packet: the quotient of the pool words + 1, scaled to a slot.
pub fn sce_sif_rpc_get_fpacket(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
    c.div(0, 5, 3);
    // (both arms of the generated zero test fall through identically).
    c.cycles += 7;
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.g[V0][0] = c.hi[0];
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.pc = t;
}

/// `void *sceSifRpc_GetFPacket2(...)` (0x004266c8): the packet at slot
/// `a1`, or the next free packet when `a1` is negative or past the count.
pub fn sce_sif_rpc_get_fpacket2(c: &mut Cpu) {
    use oc1322_callee::*;
    guest(c, |c| {
        c.cycles += 3;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        let negative = (c.g[A1][0] as i64) < 0;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let direct = if negative {
            true
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                false
            } else {
                true
            }
        };
        if direct {
            c.cycles += 4;
            c.g[RA][0] = sx(0x004266ec);
            OC1322_GET_FPACKET.call(c, 0x004266ec)?;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        } else {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << 6);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceFs_CallFuncCmdHandler(...)` (0x00427a60): run the handler stored
/// at `[a1]` (or return 0 when the slot is empty), with interrupts enabled.
pub fn sce_fs_call_func_cmd_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        // A set slot is an indirect call; an empty slot skips it (the
        // delay-slot `RA` save runs on both paths).
        if c.g[V0][0] != 0u64 {
            c.cycles += 2;
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00427a78);
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
            c.pc = t;
            call_ptr(c, t, 0x00427a78)?;
        }
        c.cycles += 5;
        c.ei();
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceFsReset(void)` (0x00427d28): clear the fs client word and wipe
/// the fd table, then return 0.
pub fn sce_fs_reset(c: &mut Cpu) {
    use oc1322_callee::*;
    guest(c, |c| {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0x450000;
        c.g[A0][0] = 0x530000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x520c), 0u64);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11d8));
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x00427d4c);
        c.g[A2][0] = 0x4;
        MEMSET.call(c, 0x00427d4c)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = 0u64;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `int sceUmount(const char *fs)` (0x0042a2e0): run the path command 0x15
/// (`umount`) on `a0`.
pub fn sce_umount(c: &mut Cpu) {
    use oc1322_callee::*;
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x0042a2f0);
        c.g[A1][0] = 0x15;
        OC1322_PATH_CMD.call(c, 0x0042a2f0)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::sif_cmd_table::{NEG_TABLE, POS_TABLE};
    use super::*;

    pub(crate) const SEEDS: u32 = 200;

    /// `sceSifRemoveCmdHandler`: negative (system table) and non-negative
    /// (user table) cids, crossing the table words.
    #[test]
    fn sce_sif_remove_cmd_handler_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_001, 0);
            c.pc = 0x0042_6050;
            // cid digit: negative, zero, small, large (index words differ).
            c.g[A0][0] = [-4, 0, 3, 0x1_0000i64][seed as usize % 4] as u64;
            c.bus.write32(NEG_TABLE, 0x0101_0100 + seed);
            c.bus.write32(POS_TABLE, 0x0202_0200 + seed);
            c.bus.write32(NEG_TABLE + 0x18, 0x0303_0300 + seed);
            c.bus.write32(POS_TABLE + 0x18, 0x0404_0400 + seed);
        };
        check_paths(
            ee::s15::f_00426050,
            sce_sif_remove_cmd_handler,
            SEEDS,
            &[7, 8],
            &setup,
        );
    }

    /// `sceSifCmd_ChangeAddrHandler`: copy `a0 + 0x10` to `a1 + 8`.
    #[test]
    fn sce_sif_cmd_change_addr_handler_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_002, 0);
            c.pc = 0x0042_5ce0;
            c.g[A0][0] = (DATA + 0x100) as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus
                .write32(DATA + 0x110, seed.wrapping_mul(0x9e37_79b9) | 1);
            c.bus
                .write32(DATA + 0x208, seed.wrapping_mul(0x5bd1_e995) | 1);
        };
        check_paths(
            ee::s15::f_00425ce0,
            sce_sif_cmd_change_addr_handler,
            SEEDS,
            &[3],
            &setup,
        );
    }

    /// `sceSifExitCmd`: stop DMA channel 5, drop its handler, clear state.
    /// Single path (the totals vary with the syscall fake's charge in
    /// `RemoveDmacHandler` and the scrambled cop0 state in `DisableDmac`).
    #[test]
    fn sce_sif_exit_cmd_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_003, 4);
            c.pc = 0x0042_5fb8;
            c.bus
                .write32(0x0052_bcd4, seed.wrapping_mul(0x9e37_79b9) | 1);
            c.bus
                .write32(0x0045_517c, seed.wrapping_mul(0x5bd1_e995) | 1);
        };
        check_paths(
            ee::s15::f_00425fb8,
            sce_sif_exit_cmd,
            SEEDS,
            &[
                36, 38, 39, 40, 41, 42, 43, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                62,
            ],
            &setup,
        );
    }

    /// `sceSifRpc_GetFPacket`: divisor/dividend classes, including 0, with
    /// LO/HI compared (the `div` writes both).
    #[test]
    fn sce_sif_rpc_get_fpacket_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_004, 0);
            c.pc = 0x0042_6698;
            c.g[A0][0] = (DATA + 0x100) as u64;
            // Divisor digit: 0, 1, 7. Dividend digit: 0, 0x40, -5.
            c.bus.write32(DATA + 0x118, [0, 1, 7][seed as usize % 3]);
            c.bus
                .write32(DATA + 0x124, [0, 0x40, 0xffff_fffb][seed as usize / 3 % 3]);
            c.bus
                .write32(DATA + 0x114, seed.wrapping_mul(0x9e37_79b9) | 1);
        };
        check_lohi(ee::s15::f_00426698, sce_sif_rpc_get_fpacket, SEEDS, &setup);
        require_paths(ee::s15::f_00426698, SEEDS, &[12], &setup);
    }

    /// `sceSifRpc_GetFPacket2`: negative, in-range and past-the-count
    /// indexes; the struct words vary independently.
    #[test]
    fn sce_sif_rpc_get_fpacket2_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_005, 0);
            c.pc = 0x0042_66c8;
            c.g[A0][0] = (DATA + 0x100) as u64;
            // Index digit: negative (direct), 0 and 2 in range of the
            // count word, 9 past it (direct).
            c.g[A1][0] = [-1, 0, 2, 9i64][seed as usize % 4] as u64;
            c.bus.write32(DATA + 0x120, 3);
            c.bus
                .write32(DATA + 0x11c, seed.wrapping_mul(0x9e37_79b9) | 1);
            c.bus
                .write32(DATA + 0x114, seed.wrapping_mul(0x5bd1_e995) | 1);
            c.bus.write32(DATA + 0x118, (seed & 0x7f) + 1);
            c.bus.write32(DATA + 0x124, seed | 1);
        };
        check_paths(
            ee::s15::f_004266c8,
            sce_sif_rpc_get_fpacket2,
            SEEDS,
            &[12, 21, 25],
            &setup,
        );
    }

    /// `sceFs_CallFuncCmdHandler`: empty slot (returns 0) and set slots
    /// (the indirect call runs the test fake).
    #[test]
    fn sce_fs_call_func_cmd_handler_matches() {
        covered(
            ee::s15::f_00427a60,
            sce_fs_call_func_cmd_handler,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 1_322_006, 2);
                c.pc = 0x0042_7a60;
                c.g[A1][0] = (DATA + 0x100) as u64;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0);
                } else {
                    c.bus.write32(DATA + 0x100, FAKE);
                    c.bus.write32(DATA + 0x104, DATA + 0x200);
                }
                for i in 0..16 {
                    c.bus.write32(DATA + 0x200 + 4 * i, 0x0101_0100 + seed);
                }
            },
        );
    }

    /// `sceFsReset`: wipe the fd table (the `memset` runs for real).
    #[test]
    fn sce_fs_reset_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_007, 0);
            c.pc = 0x0042_7d28;
            c.bus
                .write32(0x0045_520c, seed.wrapping_mul(0x9e37_79b9) | 1);
            for i in 0..16 {
                c.bus.write32(0x0052_c000 + 4 * i, 0x0101_0100 + seed);
            }
        };
        check_paths(ee::s15::f_00427d28, sce_fs_reset, SEEDS, &[53], &setup);
    }

    /// `sceUmount`: the path command runs for real on the fixture string.
    #[test]
    fn sce_umount_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            machine(c, seed, 1_322_008, 0);
            c.pc = 0x0042_a2e0;
            c.g[A0][0] = (DATA + 0x100) as u64;
            let name = b"host0:/\0";
            for (i, b) in name.iter().enumerate() {
                c.bus.write8(DATA + 0x100 + i as u32, *b);
            }
        };
        check_paths(ee::s15::f_0042a2e0, sce_umount, SEEDS, &[21176], &setup);
    }
}
