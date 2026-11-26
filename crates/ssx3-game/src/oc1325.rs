//! Staging module oc1325 (sceio-1 wave, dispatcher conversions): `sceFsInit`
//! and `sceSifRpcLoop`, converted from their generated bodies with
//! `tools/port_dispatcher.py` (match-pc dispatchers the writers don't port),
//! each with its differential test against the generated body.
//! Moved into the source-tree units by `tools/reorg_modules.py apply oc1325`.

use crate::call::{guest, Callee};
use crate::events::back_edge_unwinds;
use crate::difftest::fakes::{covered, machine};
use crate::difftest::{check_paths, DATA};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0042_7a90, sce_fs_init, "sceFsInit"),
    (0x0042_73f8, sce_sif_rpc_loop, "sceSifRpcLoop"),
];

pub(crate) mod oc1325_callee {
    use super::Callee;
    use super::ee;

    pub(super) use crate::game::lib::cdvd::callee::ADD_CMD_HANDLER;
    pub(super) use crate::game::lib::gcc::SIGNAL_SEMA;
    pub(super) use crate::game::lib::kernel::DINTR;
    pub(super) use crate::game::lib::sif::callee::BIND_RPC;
    pub(super) use crate::game::lib::sif::callee::CALL_RPC;
    pub(super) use crate::game::lib::sif::callee::EINTR;
    pub(super) use crate::game::lib::sif::callee::INIT_RPC;
    pub(super) use crate::game::lib::gcc::WAIT_SEMA;

    /// `sceSifExecRequest` (`0x00427230`, dropped from this lane).
    pub(crate) const OC1325_EXEC_REQUEST: Callee = Callee {
        addr: 0x0042_7230,
        f: ee::s15::sceSifExecRequest,
    };

    /// `sceSifGetNextRequest` (`0x004271d8`, dropped from this lane).
    pub(crate) const OC1325_GET_NEXT_REQUEST: Callee = Callee {
        addr: 0x0042_71d8,
        f: ee::s15::sceSifGetNextRequest,
    };

    /// `SleepThread` (`0x00423cc0`).
    pub(crate) const OC1325_SLEEP_THREAD: Callee = Callee {
        addr: 0x0042_3cc0,
        f: ee::s15::f_00423cc0,
    };

    /// `sceFs_FdSemaInit` (`0x00427438`).
    pub(crate) const OC1325_FD_SEMA_INIT: Callee = Callee {
        addr: 0x0042_7438,
        f: ee::s15::f_00427438,
    };
}

/// `sceFsInit` (0x00427a90): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sce_fs_init(c: &mut Cpu) {
    if c.pc != 0x00427a90 {
        return ee::s15::f_00427a90__generated(c);
    }
    guest(c, |c| {
        use oc1325_callee::*;

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
                INIT_RPC.call(c, 0x00427abc)?;
                c.sw((c.g[S0][0] as u32).wrapping_sub(0x1180), 0u64);
                c.g[RA][0] = sx(0x00427ac8);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                DINTR.call(c, 0x00427ac8)?;
                c.g[A1][0] = 0x420000;
                c.g[A2][0] = 0x530000;
                c.g[A0][0] = 0xffffffff80000000;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7590));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x11c0));
                c.g[RA][0] = sx(0x00427ae8);
                c.g[A0][0] = c.g[A0][0] | 0x11;
                ADD_CMD_HANDLER.call(c, 0x00427ae8)?;
                c.g[A1][0] = 0x420000;
                c.g[A0][0] = 0xffffffff80000000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7a60));
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00427b00);
                c.g[A0][0] = c.g[A0][0] | 0x13;
                ADD_CMD_HANDLER.call(c, 0x00427b00)?;
                let t = c.g[S0][0] == 0u64; 
                c.g[S3][0] = 0x530000;
                if t { pc = 0x00427b3c; continue; }
                pc = 0x00427b08; continue;
            }
            0x00427b08 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00427b10);
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1200));
                EINTR.call(c, 0x00427b10)?;
                let t = true; 
                
                if t { pc = 0x00427b40; continue; }
                pc = 0x00427b18; continue;
            }
            0x00427b18 => {
                c.cycles += 2;
                c.g[V1][0] = 0xffffffffffffffff;
                pc = 0x00427b20; continue;
            }
            0x00427b20 => {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                let t = c.g[V0][0] != c.g[V1][0]; 
                
                if t { if back_edge_unwinds(c, 0x00427b20) { return Ok(()); } pc = 0x00427b20; continue; }
                pc = 0x00427b3c; continue;
            }
            0x00427b3c => {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1200));
                pc = 0x00427b40; continue;
            }
            0x00427b40 => {
                c.cycles += 7;
                c.g[A1][0] = 0xffffffff80000000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[A1][0] | 0x1;
                c.g[RA][0] = sx(0x00427b54);
                c.g[A2][0] = 0u64;
                BIND_RPC.call(c, 0x00427b54)?;
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[V0][0] = 0xffffffffffffffff;
                if t { pc = 0x00427c74; continue; }
                pc = 0x00427b5c; continue;
            }
            0x00427b5c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x100000;
                if t { if back_edge_unwinds(c, 0x00427b18) { return Ok(()); } pc = 0x00427b18; continue; }
                pc = 0x00427b68; continue;
            }
            0x00427b68 => {
                c.cycles += 10;
                c.g[RA][0] = sx(0x00427b70);
                c.g[S4][0] = 0x450000;
                OC1325_FD_SEMA_INIT.call(c, 0x00427b70)?;
                c.g[RA][0] = sx(0x00427b78);
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                WAIT_SEMA.call(c, 0x00427b78)?;
                c.g[V1][0] = 0x530000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1400));
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x200));
                c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[S2][0] = 0x530000;
                if t { pc = 0x00427bbc; continue; }
                pc = 0x00427b90; continue;
            }
            0x00427b90 => {
                c.cycles += 2;
                c.g[S1][0] = 0x530000;
                c.g[S0][0] = 0x530000;
                pc = 0x00427b98; continue;
            }
            0x00427b98 => {
                c.cycles += 7;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
                let t = c.g[V0][0] != 0u64; 
                
                if t { if back_edge_unwinds(c, 0x00427b98) { return Ok(()); } pc = 0x00427b98; continue; }
                pc = 0x00427bb4; continue;
            }
            0x00427bb4 => {
                c.cycles += 2;
                let t = true; 
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                if t { pc = 0x00427bc8; continue; }
                pc = 0x00427bbc; continue;
            }
            0x00427bbc => {
                c.cycles += 3;
                c.g[S1][0] = 0x530000;
                c.g[S0][0] = 0x530000;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5218));
                pc = 0x00427bc8; continue;
            }
            0x00427bc8 => {
                c.cycles += 18;
                c.g[RA][0] = sx(0x00427bd0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1cc0));
                SIGNAL_SEMA.call(c, 0x00427bd0)?;
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
                CALL_RPC.call(c, 0x00427c08)?;
                let t = (c.g[V0][0] as i64) >= 0; 
                c.g[A0][0] = 0x20000000;
                if t { pc = 0x00427c1c; continue; }
                pc = 0x00427c10; continue;
            }
            0x00427c10 => {
                c.cycles += 3;
                c.g[V0][0] = 0xfffffffffffe0000;
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                if t { pc = 0x00427c74; continue; }
                pc = 0x00427c1c; continue;
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
                pc = 0x00427c74; continue;
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
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00427a90, 0x00427c94); return Ok(()); }
        }
    }
    });
}

/// `sceSifRpcLoop` (0x004273f8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sce_sif_rpc_loop(c: &mut Cpu) {
    if c.pc != 0x004273f8 {
        return ee::s15::f_004273f8__generated(c);
    }
    guest(c, |c| {
        use oc1325_callee::*;

    let mut pc: u32 = 0x004273f8;
    loop {
        match pc {
            0x004273f8 => {
                c.cycles += 5;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sd(c.g[SP][0] as u32, c.g[S0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                let t = true; 
                c.g[S0][0] = c.g[A0][0];
                if t { pc = 0x00427418; continue; }
                pc = 0x0042740c; continue;
            }
            0x0042740c => {
                c.cycles += 1;
                pc = 0x00427410; continue;
            }
            0x00427410 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00427418);
                OC1325_EXEC_REQUEST.call(c, 0x00427418)?;
                pc = 0x00427418; continue;
            }
            0x00427418 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00427420);
                c.g[A0][0] = c.g[S0][0];
                OC1325_GET_NEXT_REQUEST.call(c, 0x00427420)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.g[V0][0];
                if t { if back_edge_unwinds(c, 0x00427410) { return Ok(()); } pc = 0x00427410; continue; }
                pc = 0x00427428; continue;
            }
            0x00427428 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00427430);
                OC1325_SLEEP_THREAD.call(c, 0x00427430)?;
                let t = true; 
                
                if t { if back_edge_unwinds(c, 0x00427418) { return Ok(()); } pc = 0x00427418; continue; }
                c.pc = 0x00427438; ee::s15::sceFs_FdSemaInit(c); return Ok(());
            }
            _ => { c.jump_out(pc, 0x004273f8, 0x00427438); return Ok(()); }
        }
    }
    });
}


#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const SEEDS: u32 = 200;

    /// RPC-loop queue at `DATA + 0x100`: the indirect call is a real
    /// zero-returning leaf (`0x1406c0`, so no packet work happens), the
    /// send-loop word is clear, the GetFPacket selector has bit 2 clear.
    /// The next-request word alternates: 0 exits after one round, nonzero
    /// takes the re-request arm once. Every seed unwinds at a fake call
    /// (`LOG_UNWIND_AT` set explicitly — the loop itself never returns),
    /// so both sides are compared on the same prefix.
    pub(crate) fn rpcloop_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 1_325_001, 4);
        c.pc = 0x0042_73f8;
        c.g[A0][0] = (DATA + 0x100) as u64;
        c.bus.write32(DATA + 0x10c, if seed % 2 == 0 { 0 } else { DATA + 0x200 });
        c.bus.write32(DATA + 0x104, 0x0014_06c0);
        c.bus.write32(DATA + 0x108, seed);
        c.bus.write32(DATA + 0x124, seed);
        c.bus.write32(DATA + 0x118, seed);
        c.bus.write32(DATA + 0x114, seed | 1);
        c.bus.write32(DATA + 0x120, 3);
        c.bus.write32(DATA + 0x128, seed);
        c.bus.write32(DATA + 0x130, 0);
        c.bus.write32(DATA + 0x134, 0);
        c.bus.write32(DATA + 0x11c, seed | 1);
        c.bus.write32(DATA + 0x120 + 0x20, seed);
        c.bus.write32(DATA + 0x200 + 0x3c, 0);
        for i in 0..16 {
            c.bus.write32(DATA + 0x200 + 4 * i, 0x0101_0100 + seed);
        }
        c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, seed % 2);
    }

    /// `sceSifRpcLoop`: one `ExecRequest` + `GetNextRequest` round (empty
    /// queue exits; non-empty re-requests once), unwound at a fake call.
    #[test]
    fn sce_sif_rpc_loop_matches() {
        covered(
            ee::s15::f_004273f8__generated,
            sce_sif_rpc_loop,
            SEEDS,
            &|c, seed| {
                rpcloop_setup(c, seed);
            },
        );
    }

    /// `sceFsInit`: bring-up on a drained RPC pool (the bind and the call
    /// fail fast), the fd table zeroed.
    #[test]
    fn sce_fs_init_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            crate::game::lib::sif::tests::mrpc_setup(c, seed, 0x0042_7a90);
            c.pc = 0x0042_7a90;
            c.bus.write32(0x0045_5180, 0);
        };
        check_paths(
            ee::s15::f_00427a90__generated,
            sce_fs_init,
            SEEDS,
            &[1207],
            &setup,
        );
    }
}
