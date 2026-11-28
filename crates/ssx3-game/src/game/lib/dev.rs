//! `src/lib/dev.cpp`: hand-written SCE device (`libdev_*`, `sceDev*`, `sceGs*`) replacements.

use crate::abi::ret;
use crate::call::guest;
use crate::call::sp;
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
    (0x003f_d910, sce_gs_reset_graph, "sceGsResetGraph"),
    (0x003f_dab0, sce_gs_reset_path, "sceGsResetPath"),
    (0x003f_db18, sce_gs_sync_path, "sceGsSyncPath"),
    (0x003f_e678, libdev_vu0_is_running, "libdev_Vu0IsRunning"),
    (0x003f_e690, sce_dev_vu0_reset, "sceDevVu0Reset"),
    (0x003f_e6a8, sce_dev_vu0_pause, "sceDevVu0Pause"),
    (0x003f_e708, sce_dev_vu0_continue, "sceDevVu0Continue"),
    (0x003f_e7d8, libdev_vu0_get_state, "libdev_Vu0GetState"),
    (0x003f_e818, sce_dev_vu0_exec, "sceDevVu0Exec"),
    (0x003f_e828, sce_dev_vu0_get_tpc, "sceDevVu0GetTpc"),
    (0x003f_e838, sce_dev_vu0_get_cnd, "sceDevVu0GetCnd"),
    (0x003f_e998, sce_dev_vu0_put_cnd, "sceDevVu0PutCnd"),
    (0x003f_eb08, libdev_vu1_is_running, "libdev_Vu1IsRunning"),
    (0x003f_eb38, libdev_vu1_put_reg, "libdev_Vu1PutReg"),
    (0x003f_eb78, libdev_vu1_get_reg, "libdev_Vu1GetReg"),
    (0x003f_ebc0, sce_dev_vu1_reset, "sceDevVu1Reset"),
    (0x003f_ebd8, sce_dev_vu1_pause, "sceDevVu1Pause"),
    (0x003f_ec38, sce_dev_vu1_continue, "sceDevVu1Continue"),
    (0x003f_ed08, libdev_vu1_get_state, "libdev_Vu1GetState"),
    (0x003f_ed50, sce_dev_vu1_exec, "sceDevVu1Exec"),
    (0x003f_ed60, sce_dev_vu1_get_tpc, "sceDevVu1GetTpc"),
    (0x003f_ed88, sce_dev_vu1_get_cnd, "sceDevVu1GetCnd"),
    (0x003f_ef18, sce_dev_vu1_put_cnd, "sceDevVu1PutCnd"),
    (0x003f_f060, sce_dev_vif0_reset, "sceDevVif0Reset"),
    (0x003f_f088, sce_dev_vif0_pause, "sceDevVif0Pause"),
    (0x003f_f0f0, sce_dev_vif0_continue, "sceDevVif0Continue"),
    (0x003f_f160, libdev_vif0_get_state, "libdev_Vif0GetState"),
    (0x003f_f1b0, sce_dev_vif0_get_cnd, "sceDevVif0GetCnd"),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `sceGsGetGParam` (`0x003f_daa0`).
    pub(crate) const GET_GPARAM: Callee = Callee {
        addr: 0x003f_daa0,
        f: ee::s15::sceGsGetGParam,
    };

    /// `libdev_Vu0IsRunning` (`0x003f_e678`).
    pub(crate) const VU0_IS_RUNNING: Callee = Callee {
        addr: 0x003f_e678,
        f: ee::s15::libdev_Vu0IsRunning,
    };

    /// `libdev_Vu0GetState` (`0x003f_e7d8`).
    pub(crate) const VU0_GET_STATE: Callee = Callee {
        addr: 0x003f_e7d8,
        f: ee::s15::libdev_Vu0GetState,
    };

    /// `sceDevVu0Exec` (`0x003f_e818`).
    pub(crate) const VU0_EXEC: Callee = Callee {
        addr: 0x003f_e818,
        f: ee::s15::sceDevVu0Exec,
    };

    /// `sceDevVu0GetTpc` (`0x003f_e828`).
    pub(crate) const VU0_GET_TPC: Callee = Callee {
        addr: 0x003f_e828,
        f: ee::s15::sceDevVu0GetTpc,
    };

    /// `libdev_Vu1IsRunning` (`0x003f_eb08`).
    pub(crate) const VU1_IS_RUNNING: Callee = Callee {
        addr: 0x003f_eb08,
        f: ee::s15::libdev_Vu1IsRunning,
    };

    /// `libdev_Vu1PutReg` (`0x003f_eb38`).
    pub(crate) const VU1_PUT_REG: Callee = Callee {
        addr: 0x003f_eb38,
        f: ee::s15::libdev_Vu1PutReg,
    };

    /// `libdev_Vu1GetReg` (`0x003f_eb78`).
    pub(crate) const VU1_GET_REG: Callee = Callee {
        addr: 0x003f_eb78,
        f: ee::s15::libdev_Vu1GetReg,
    };

    /// `libdev_Vu1GetState` (`0x003f_ed08`).
    pub(crate) const VU1_GET_STATE: Callee = Callee {
        addr: 0x003f_ed08,
        f: ee::s15::libdev_Vu1GetState,
    };

    /// `sceDevVu1Exec` (`0x003f_ed50`).
    pub(crate) const VU1_EXEC: Callee = Callee {
        addr: 0x003f_ed50,
        f: ee::s15::sceDevVu1Exec,
    };

    /// `sceDevVu1GetTpc` (`0x003f_ed60`).
    pub(crate) const VU1_GET_TPC: Callee = Callee {
        addr: 0x003f_ed60,
        f: ee::s15::sceDevVu1GetTpc,
    };

    /// `libdev_Vif0GetState` (`0x003f_f160`).
    pub(crate) const VIF0_GET_STATE: Callee = Callee {
        addr: 0x003f_f160,
        f: ee::s15::libdev_Vif0GetState,
    };

    /// `RemoveIntcHandler` (`0x0042_3aa0`).
    pub(crate) const REMOVE_INTC_HANDLER: Callee = Callee {
        addr: 0x0042_3aa0,
        f: ee::s15::RemoveIntcHandler,
    };

    /// `GsPutIMR` (`0x0042_40e0`).
    pub(crate) const PUT_IMR: Callee = Callee {
        addr: 0x0042_40e0,
        f: ee::s15::GsPutIMR,
    };

    /// `DisableIntc` (`0x0042_4880`).
    pub(crate) const DISABLE_INTC: Callee = Callee {
        addr: 0x0042_4880,
        f: ee::s15::DisableIntc,
    };

    pub(super) use crate::game::lib::sif::callee::PRINTF;
}

/// `int libdev_Vif0GetState(void)` (0x3ff160): decode VIF0_STAT
/// (`0x10003800`) into a run state: 0 when the packet status (bits 0-1) is
/// idle; otherwise 1 when a stall bit (`0x700`) is set, else 2 when a wait
/// bit (`0xc`) is set, else 3 when an error bit (`0x3000`) is set, else 4.
pub fn libdev_vif0_get_state(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x10000000;
    c.g[V0][0] = c.g[V0][0] | 0x3800;
    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = c.g[A0][0] & 0x3;
    let active = c.g[V1][0] != 0u64;
    c.g[V0][0] = c.g[A0][0] & 0x700;
    if active {
        c.cycles += 2;
        let no_code = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[A0][0] & 0xc;
        if no_code {
            c.cycles += 2;
            let no_mask = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[A0][0] & 0x3000;
            if no_mask {
                c.cycles += 4;
                c.g[V1][0] = 0x4;
                c.g[V0][0] = 0x3;
                let t = c.g[RA][0] as u32;
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.pc = t;
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = 0x2;
                c.pc = t;
            }
        } else {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = 0x1;
            c.pc = t;
        }
    } else {
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = 0u64;
        c.pc = t;
    }
}

/// `int libdev_Vu0GetState(void)` (0x3fe7d8): decode the VPU status word
/// (COP2 control 29) into a VU0 run state: bit 1 set is 1, else bit 2 set
/// is 2, else bit 3 set is 3, else bit 0 shifted up is 4, else 0. Unlike the
/// VU1 twin, the status word stays in V1 at exit.
pub fn libdev_vu0_get_state(c: &mut Cpu) {
    c.cycles += 4;
    c.cfc2(3, 29);
    c.g[V0][0] = c.g[V1][0] & 0x2;
    let bit1_clear = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.g[V1][0] & 0x4;
    if bit1_clear {
        c.cycles += 2;
        let unset = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[V1][0] & 0x8;
        if unset {
            c.cycles += 2;
            let clear = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[V1][0] & 0x1;
            if clear {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                ret(c);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x3;
                ret(c);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            ret(c);
        }
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        ret(c);
    }
}

/// `int libdev_Vu0IsRunning(void)` (0x3fe678): nonzero while VU0 is busy.
/// Reads the VPU status word (COP2 control 29): returns 1 when its low
/// nibble reads 1, otherwise 0.
pub fn libdev_vu0_is_running(c: &mut Cpu) {
    c.cycles += 5;
    c.cfc2(2, 29);
    c.g[V0][0] = c.g[V0][0] & 0xf;
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    ret(c);
}

/// `void libdev_Vu1GetReg(u128 *out, int reg)` (0x3feb78): read the 16-byte
/// VU1 register `reg` into `[out]` through VU0 macro mode (part of the
/// saved-VU1-context capture path).
pub fn libdev_vu1_get_reg(c: &mut Cpu) {
    c.cycles += 17;
    c.g[A1][0] = c.g[A1][0] & 0xffff;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.cfc2(6, 1);
    c.qmfc2(3, 1);
    c.ctc2(5, 1);
    c.vu0().lqi(0b1111, 1, 1);
    c.qmfc2(2, 1);
    c.ctc2(6, 1);
    c.qmtc2(3, 1);
    c.sq(2, c.g[SP][0] as u32);
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = c.ld(c.g[SP][0] as u32);
    c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x8));
    c.sd(c.g[A0][0] as u32, c.g[V1][0]);
    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
}

/// `int libdev_Vu1GetState(void)` (0x3fed08): decode the VPU status word
/// (COP2 control 29) into a VU1 run state: bit 9 set is 1, else bit 10 set
/// is 2, else bit 11 set is 3, else bit 8 set is 4, else 0.
pub fn libdev_vu1_get_state(c: &mut Cpu) {
    c.cycles += 4;
    c.cfc2(3, 29);
    c.g[V0][0] = c.g[V1][0] & 0x200;
    let bit9_clear = c.g[V0][0] == 0u64;
    c.g[V0][0] = c.g[V1][0] & 0x400;
    if !bit9_clear {
        c.cycles += 2;
        c.g[V1][0] = 0x1;
    } else {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[V1][0] & 0x800;
        if !t {
            c.cycles += 2;
            c.g[V1][0] = 0x2;
        } else {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.g[V1][0] & 0x100;
            if !t {
                c.cycles += 2;
                c.g[V1][0] = 0x3;
            } else {
                c.cycles += 2;
                c.g[V1][0] = 0x4;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = 0u64;
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    ret(c);
}

/// `int libdev_Vu1IsRunning(void)` (0x3feb08): read the VPU status word (COP2
/// control 29) and return 1 when its low nibble is exactly 1 or its bits 8-11
/// are exactly `0x100`; otherwise 0.
pub fn libdev_vu1_is_running(c: &mut Cpu) {
    c.cycles += 5;
    c.cfc2(3, 29);
    c.g[A0][0] = 0x1;
    c.g[V0][0] = c.g[V1][0] & 0xf;
    let not_running = c.g[V0][0] != c.g[A0][0];
    c.g[V1][0] = c.g[V1][0] & 0xf00;
    if not_running {
        c.cycles += 4;
        c.g[V0][0] = c.g[A0][0];
        c.g[V1][0] = c.g[V1][0] ^ 0x100;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0u64;
        }
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
    }
    ret(c);
}

/// `void libdev_Vu1PutReg(int reg, const u128 *val)` (0x3feb38): write the
/// 16-byte VU1 register `reg` from `[val]` through VU0 macro mode (part of
/// the saved-VU1-context restore path).
pub fn libdev_vu1_put_reg(c: &mut Cpu) {
    c.cycles += 16;
    c.g[V0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[V1][0] = c.ld(c.g[A1][0] as u32);
    c.g[A0][0] = c.g[A0][0] & 0xffff;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    c.sd(c.g[SP][0] as u32, c.g[V1][0]);
    c.lq(2, c.g[SP][0] as u32);
    c.cfc2(5, 1);
    c.qmfc2(3, 1);
    c.qmtc2(2, 1);
    c.ctc2(4, 1);
    c.vu0().sqi(0b1111, 1, 1);
    c.ctc2(5, 1);
    c.qmtc2(3, 1);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
}

/// `int sceDevVif0Continue(void)` (0x3ff0f0): clear a VIF0 stall (writes 8
/// to FBRST, clearing VSS/VFS/VIS/INT) when the channel sits in state 1.
/// Returns 1, else 0.
pub fn sce_dev_vif0_continue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_f100);
        callee::VIF0_GET_STATE.call(c, 0x003f_f100)?;
        c.g[V1][0] = 0x1;
        let stalled = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0u64;
        if !stalled {
            c.cycles += 5;
            c.g[V1][0] = 0x10000000;
            c.g[A0][0] = 0x8;
            c.g[V1][0] = c.g[V1][0] | 0x3810;
            c.g[V0][0] = 0x1;
            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int sceDevVif0GetCnd(u32 *out)` (0x3ff1b0): capture the full VIF0
/// register context (rows, cols, mask, code, status, tops, mark, num, err,
/// cycle, mode) into the caller's 0x38-byte buffer. Returns 1, or 0 while
/// VIF0 is running (state 2 or more).
pub fn sce_dev_vif0_get_cnd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x10000000;
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[V0][0] = c.g[V0][0] | 0x3840;
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.g[S3][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x40), c.g[S4][0]);
        c.g[RA][0] = sx(0x003f_f1e0);
        c.g[S4][0] = c.lw(c.g[V0][0] as u32);
        callee::VIF0_GET_STATE.call(c, 0x003f_f1e0)?;
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        let settled = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0x10000000;
        if !settled {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 71;
            c.g[V1][0] = 0x10000000;
            c.g[V0][0] = c.g[V0][0] | 0x3900;
            c.g[V1][0] = c.g[V1][0] | 0x3910;
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A0][0] = 0x10000000;
            c.g[A0][0] = c.g[A0][0] | 0x3920;
            c.g[A2][0] = 0x10000000;
            c.sw(c.g[S3][0] as u32, c.g[A1][0]);
            c.g[A2][0] = c.g[A2][0] | 0x3930;
            c.g[A1][0] = 0x10000000;
            c.g[T7][0] = 0x10000000;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[A1][0] = c.g[A1][0] | 0x3940;
            c.g[T7][0] = c.g[T7][0] | 0x3950;
            c.g[S0][0] = 0x10000000;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[S0][0] = c.g[S0][0] | 0x3960;
            c.g[T6][0] = 0x10000000;
            c.g[S1][0] = 0x10000000;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[T6][0] = c.g[T6][0] | 0x3970;
            c.g[S1][0] = c.g[S1][0] | 0x3870;
            c.g[T4][0] = 0x10000000;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[T4][0] = c.g[T4][0] | 0x3880;
            c.g[T3][0] = 0x10000000;
            c.g[T5][0] = 0x10000000;
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            c.g[T3][0] = c.g[T3][0] | 0x3800;
            c.g[T5][0] = c.g[T5][0] | 0x38d0;
            c.g[A2][0] = 0x10000000;
            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[A2][0] = c.g[A2][0] | 0x3890;
            c.g[A3][0] = 0x10000000;
            c.g[T1][0] = 0x10000000;
            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
            c.g[A3][0] = c.g[A3][0] | 0x3830;
            c.g[T1][0] = c.g[T1][0] | 0x3860;
            c.g[T0][0] = 0x10000000;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[T0][0] = c.g[T0][0] | 0x3820;
            c.g[S2][0] = sx((c.g[S4][0] as u32) >> 8);
            c.g[T2][0] = 0x10000000;
            c.g[A0][0] = c.lw(c.g[T7][0] as u32);
            c.g[T2][0] = c.g[T2][0] | 0x3850;
            c.g[V0][0] = 0x1;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[A0][0]);
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[A0][0] = c.lw(c.g[T6][0] as u32);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
            c.g[V1][0] = c.lw(c.g[S1][0] as u32);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[V1][0]);
            c.g[A0][0] = c.lw(c.g[T4][0] as u32);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x24), c.g[A0][0]);
            c.g[V1][0] = c.lw(c.g[T3][0] as u32);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x28), c.g[V1][0]);
            c.g[A0][0] = c.lw(c.g[T5][0] as u32);
            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
            c.g[A1][0] = c.lw(c.g[A3][0] as u32);
            c.g[A2][0] = c.lw(c.g[T1][0] as u32);
            c.g[A3][0] = c.lw(c.g[T0][0] as u32);
            c.sh((c.g[S3][0] as u32).wrapping_add(0x2e), c.g[V1][0]);
            c.sh((c.g[S3][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
            c.sh((c.g[S3][0] as u32).wrapping_add(0x30), c.g[A1][0]);
            c.sh((c.g[S3][0] as u32).wrapping_add(0x32), c.g[A2][0]);
            c.sb((c.g[S3][0] as u32).wrapping_add(0x34), c.g[A3][0]);
            c.sb((c.g[S3][0] as u32).wrapping_add(0x35), c.g[S4][0]);
            c.sb((c.g[S3][0] as u32).wrapping_add(0x36), c.g[S2][0]);
            c.g[V1][0] = c.lw(c.g[T2][0] as u32);
            c.sb((c.g[S3][0] as u32).wrapping_add(0x37), c.g[V1][0]);
        }
        c.cycles += 8;
        c.g[RA][0] = c.ld(frame.at(c, 0x50));
        c.g[S4][0] = c.ld(frame.at(c, 0x40));
        c.g[S3][0] = c.ld(frame.at(c, 0x30));
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceDevVif0Pause(int mode)` (0x3ff088): stall VIF0 unless it is
/// already idle (state 0) or just started (state 1). Stalled with mode 0 it
/// writes 2 to FBRST, with mode 1 it writes 4; any other mode returns 1
/// without a write. Returns 0 when VIF0 was already idle.
pub fn sce_dev_vif0_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_f09c);
        c.g[S0][0] = c.g[A0][0];
        callee::VIF0_GET_STATE.call(c, 0x003f_f09c)?;
        let idle = c.g[V0][0] == 0u64;
        c.g[V1][0] = 0x1;
        // 0x3ff0d4: the fresh and stalled-plain paths rejoin for
        // the shared store; only stalled-and-other returns
        // through 0x3ff0e0 with the V0=1 set above.
        let mut store = false;
        if idle {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 2;
            let fresh = c.g[V0][0] != c.g[V1][0];
            if fresh {
                c.cycles += 2;
                c.g[V0][0] = 0x10000000;
                let stalled = c.g[S0][0] != 0u64;
                if stalled {
                    c.cycles += 2;
                    let other = c.g[S0][0] != c.g[V1][0];
                    c.g[V0][0] = 0x1;
                    if !other {
                        c.cycles += 2;
                        c.g[V0][0] = 0x10000000;
                        c.g[V1][0] = 0x4;
                        store = true;
                    }
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = 0x2;
                    store = true;
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0u64;
            }
        }
        if store {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] | 0x3810;
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[V0][0] = 0x1;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDevVif0Reset(void)` (0x3ff060): reset VIF0 (`VIF0_FBRST`
/// `0x10003810` = 1), then write 6 to `VIF0_ERR` (`0x10003820`), setting its
/// two mask bits so DMAtag-mismatch and VIFcode errors are ignored.
pub fn sce_dev_vif0_reset(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x10000000;
    c.g[A0][0] = 0x1;
    c.g[V0][0] = c.g[V0][0] | 0x3810;
    c.g[V1][0] = 0x10000000;
    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
    c.g[V1][0] = c.g[V1][0] | 0x3820;
    c.g[V0][0] = 0x6;
    let t = c.g[RA][0] as u32;
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.pc = t;
}

/// `int sceDevVu0Continue(void)` (0x3fe708): resume VU0 after a pause by
/// re-executing the microprogram from the saved TPC. Returns 1 when it
/// kicked VU0 (state 1-3), else 0.
pub fn sce_dev_vu0_continue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e71c);
        c.g[S0][0] = 0u64;
        callee::VU0_GET_STATE.call(c, 0x003f_e71c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
        let paused = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[S0][0];
        if !paused {
            c.cycles += 2;
            let idle = (c.g[V1][0] as i64) <= 0;
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
            if !idle {
                c.cycles += 5;
                c.g[RA][0] = sx(0x003f_e73c);
                c.g[S0][0] = 0x1;
                callee::VU0_GET_TPC.call(c, 0x003f_e73c)?;
                c.g[RA][0] = sx(0x003f_e744);
                c.g[A0][0] = c.g[V0][0];
                callee::VU0_EXEC.call(c, 0x003f_e744)?;
                c.g[V0][0] = c.g[S0][0];
                c.cycles += 1;
                c.g[RA][0] = c.ld(frame.at(c, 0x10));
            }
        } else {
            c.cycles += 1;
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
        }
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDevVu0Exec(int tpc)` (0x3fe818): start VU0 executing at `tpc`
/// (writes the masked address to CMSAR0, COP2 control 27, then runs the
/// microprogram to its E bit).
pub fn sce_dev_vu0_exec(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = c.g[A0][0] & 0xffff;
    c.ctc2(4, 27);
    c.vcallmsr();
    ret(c);
}

/// `int sceDevVu0GetCnd(u128 *out)` (0x3fe838): capture the VU0 context into
/// the caller's 0x238-byte buffer: COP2 control 16-18 and 20-22 (status, MAC,
/// clip, R, I, Q) as words at `+0x200..0x214` and VI00-VI15 as halfwords at
/// `+0x218..0x236`. The 32 `sqc2` stores of VF00-VF31 use `v0` as their base,
/// which is 0 here, so they land at guest `0x0..0x1ff` rather than in `out`.
/// Returns 1, or 0 while VU0 is running.
pub fn sce_dev_vu0_get_cnd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e84c);
        c.g[S0][0] = c.g[A0][0];
        callee::VU0_IS_RUNNING.call(c, 0x003f_e84c)?;
        let busy = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0u64;
        if !busy {
            c.cycles += 77;
            let out = c.g[S0][0] as u32;
            for i in 0..32usize {
                c.sqc2(i, (c.g[V0][0] as u32).wrapping_add(i as u32 * 0x10));
            }
            for (off, reg) in [
                (0x200u32, 16usize),
                (0x204, 17),
                (0x208, 18),
                (0x20c, 20),
                (0x210, 21),
                (0x214, 22),
            ] {
                c.cfc2(2, reg);
                c.sw(out.wrapping_add(off), c.g[V0][0]);
            }
            for (off, reg) in [
                (0x218u32, 0usize),
                (0x21a, 1),
                (0x21c, 2),
                (0x21e, 3),
                (0x220, 4),
                (0x222, 5),
                (0x224, 6),
                (0x226, 7),
                (0x228, 8),
                (0x22a, 9),
                (0x22c, 10),
                (0x22e, 11),
                (0x230, 12),
                (0x232, 13),
                (0x234, 14),
                (0x236, 15),
            ] {
                c.cfc2(2, reg);
                c.sh(out.wrapping_add(off), c.g[V0][0]);
            }
            c.g[V0][0] = 0x1;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceDevVu0GetTpc(void)` (0x3fe828): read VU0's current microprogram
/// counter (VU0 control register 26), masked to 16 bits.
pub fn sce_dev_vu0_get_tpc(c: &mut Cpu) {
    c.cycles += 3;
    c.cfc2(2, 26);
    c.g[V0][0] = c.g[V0][0] & 0xffff;
    ret(c);
}

/// `int sceDevVu0Pause(void)` (0x3fe6a8): pause VU0 by its run state: states
/// 0-4 dispatch through the handler table at 0x497850. The 0x3fe6e4 arm
/// forces a break (FBRST bit 0) and returns 1, the 0x3fe6f0 arm returns 1
/// without touching VU0, and the 0x3fe6f4 arm returns 0. Returns 0 without
/// touching hardware when the state is 5 or more.
pub fn sce_dev_vu0_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e6bc);
        c.g[S0][0] = 0u64;
        callee::VU0_GET_STATE.call(c, 0x003f_e6bc)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
        let unknown_state = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x490000;
        if !unknown_state {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7850));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            // Jump table (0x003f_e6a8): states 0-4 through 0x497850.
            let t = c.g[A0][0] as u32;
            match t {
                0x003f_e6e4 => {
                    c.cycles += 3;
                    c.cfc2(2, 28);
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.ctc2(2, 28);
                    c.cycles += 1;
                    c.g[S0][0] = 0x1;
                }
                0x003f_e6f0 => {
                    c.cycles += 1;
                    c.g[S0][0] = 0x1;
                }
                0x003f_e6f4 => {}
                _ => {
                    c.jump_out(t, 0x003f_e6a8, 0x003f_e708);
                    return Ok(());
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceDevVu0PutCnd(const u128 *in)` (0x3fe998): restore a saved VU0
/// context from the caller's 0x238-byte buffer: VF00-VF31 from
/// `+0x000..0x1ff`, COP2 control 16-18 and 20-22 (status, MAC, clip, R, I, Q)
/// from the words at `+0x200..0x214` and VI00-VI15 from the halfwords at
/// `+0x218..0x236`. Returns 1, or 0 while VU0 is running.
pub fn sce_dev_vu0_put_cnd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_e9b4);
        c.g[S0][0] = c.g[S1][0];
        callee::VU0_IS_RUNNING.call(c, 0x003f_e9b4)?;
        let busy = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0u64;
        if !busy {
            c.cycles += 77;
            let src = c.g[S0][0] as u32;
            for i in 0..32usize {
                c.lqc2(i, src.wrapping_add(i as u32 * 0x10));
            }
            for (off, reg) in [
                (0x200u32, 16usize),
                (0x204, 17),
                (0x208, 18),
                (0x20c, 20),
                (0x210, 21),
                (0x214, 22),
            ] {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(off));
                c.ctc2(2, reg);
            }
            for (off, reg) in [
                (0x218u32, 0usize),
                (0x21a, 1),
                (0x21c, 2),
                (0x21e, 3),
                (0x220, 4),
                (0x222, 5),
                (0x224, 6),
                (0x226, 7),
                (0x228, 8),
                (0x22a, 9),
                (0x22c, 10),
                (0x22e, 11),
                (0x230, 12),
                (0x232, 13),
                (0x234, 14),
                (0x236, 15),
            ] {
                c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(off));
                c.ctc2(2, reg);
            }
            c.g[V0][0] = 0x1;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDevVu0Reset(void)` (0x3fe690): reset VU0 (set FBRST bit 1).
pub fn sce_dev_vu0_reset(c: &mut Cpu) {
    c.cycles += 5;
    c.cfc2(2, 28);
    c.g[V0][0] = c.g[V0][0] | 0x2;
    c.ctc2(2, 28);
    ret(c);
}

/// `int sceDevVu1Continue(void)` (0x3fec38): resume VU1 after a pause by
/// re-executing the microprogram from the saved TPC. Returns 1 when it
/// kicked VU1 (state 1-3), else 0.
pub fn sce_dev_vu1_continue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_ec4c);
        c.g[S0][0] = 0u64;
        callee::VU1_GET_STATE.call(c, 0x003f_ec4c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
        let paused = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.g[S0][0];
        if !paused {
            c.cycles += 2;
            let idle = (c.g[V1][0] as i64) <= 0;
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
            if !idle {
                c.cycles += 5;
                c.g[RA][0] = sx(0x003f_ec6c);
                c.g[S0][0] = 0x1;
                callee::VU1_GET_TPC.call(c, 0x003f_ec6c)?;
                c.g[RA][0] = sx(0x003f_ec74);
                c.g[A0][0] = c.g[V0][0];
                callee::VU1_EXEC.call(c, 0x003f_ec74)?;
                c.g[V0][0] = c.g[S0][0];
                c.cycles += 1;
                c.g[RA][0] = c.ld(frame.at(c, 0x10));
            }
        } else {
            c.cycles += 1;
            c.g[RA][0] = c.ld(frame.at(c, 0x10));
        }
        c.cycles += 3;
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDevVu1Exec(int tpc)` (0x3fed50): start VU1 executing at `tpc`
/// (writes the masked address to CMSAR1, COP2 control 31).
pub fn sce_dev_vu1_exec(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = c.g[A0][0] & 0xffff;
    c.ctc2(4, 31);
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `int sceDevVu1GetCnd(u128 *out)` (0x3fed88): capture the VU1 register
/// context through `libdev_Vu1GetReg` into the caller's 0x23c-byte buffer:
/// VF00-VF31 at `+0x000..0x1ff`, registers 0x430-0x432 and 0x434-0x437 as
/// words at `+0x200..0x218`, and VI00-VI15 as halfwords at `+0x21c..0x23b`.
/// Returns 1, or 0 while VU1 is running.
pub fn sce_dev_vu1_get_cnd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x130);
        c.sd(frame.at(c, 0xb0), c.g[S2][0]);
        c.sd(frame.at(c, 0x120), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x110), c.g[FP][0]);
        c.sd(frame.at(c, 0x100), c.g[S7][0]);
        c.sd(frame.at(c, 0xf0), c.g[S6][0]);
        c.sd(frame.at(c, 0xe0), c.g[S5][0]);
        c.sd(frame.at(c, 0xd0), c.g[S4][0]);
        c.sd(frame.at(c, 0xc0), c.g[S3][0]);
        c.sd(frame.at(c, 0xa0), c.g[S1][0]);
        c.g[RA][0] = sx(0x003f_edbc);
        c.sd(frame.at(c, 0x90), c.g[S0][0]);
        callee::VU1_IS_RUNNING.call(c, 0x003f_edbc)?;
        let busy = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0u64;
        if !busy {
            c.cycles += 13;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x84), c.g[V0][0]);
            c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
            c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x21c));
            c.g[S0][0] = c.g[S2][0];
            loop {
                c.cycles += 10;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x400));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                c.g[RA][0] = sx(0x003f_ee0c);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                callee::VU1_GET_REG.call(c, 0x003f_ee0c)?;
                c.lq(2, c.g[SP][0] as u32);
                c.g[V1][0] = (c.g[S1][0] < 0x20u64) as u64;
                c.sq(2, c.g[S0][0] as u32);
                let more = c.g[V1][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_edf8);
                    }
                } else {
                    c.cycles += 38;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003f_ee2c);
                    c.g[A1][0] = 0x430;
                    callee::VU1_GET_REG.call(c, 0x003f_ee2c)?;
                    c.g[S0][0] = c.g[S3][0];
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0x431;
                    c.g[S1][0] = 0u64;
                    c.g[RA][0] = sx(0x003f_ee48);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    callee::VU1_GET_REG.call(c, 0x003f_ee48)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 0x432;
                    c.g[RA][0] = sx(0x003f_ee5c);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x204), c.g[V0][0]);
                    callee::VU1_GET_REG.call(c, 0x003f_ee5c)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = 0x434;
                    c.g[RA][0] = sx(0x003f_ee70);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x208), c.g[V0][0]);
                    callee::VU1_GET_REG.call(c, 0x003f_ee70)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = 0x435;
                    c.g[RA][0] = sx(0x003f_ee84);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x20c), c.g[V0][0]);
                    callee::VU1_GET_REG.call(c, 0x003f_ee84)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = 0x436;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x84));
                    c.g[RA][0] = sx(0x003f_ee98);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x210), c.g[V0][0]);
                    callee::VU1_GET_REG.call(c, 0x003f_ee98)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A1][0] = 0x437;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[RA][0] = sx(0x003f_eeac);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x214), c.g[V0][0]);
                    callee::VU1_GET_REG.call(c, 0x003f_eeac)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x218), c.g[V0][0]);
                    loop {
                        c.cycles += 10;
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x420));
                        c.g[A0][0] = c.g[FP][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        c.g[RA][0] = sx(0x003f_eecc);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        callee::VU1_GET_REG.call(c, 0x003f_eecc)?;
                        c.g[V0][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[V1][0] = (c.g[S1][0] < 0x10u64) as u64;
                        c.sh(c.g[S0][0] as u32, c.g[V0][0]);
                        let more = c.g[V1][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_eeb8);
                            }
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break;
                        }
                    }
                    break;
                }
            }
        }
        c.cycles += 12;
        c.g[RA][0] = c.ld(frame.at(c, 0x120));
        c.g[FP][0] = c.ld(frame.at(c, 0x110));
        c.g[S7][0] = c.ld(frame.at(c, 0x100));
        c.g[S6][0] = c.ld(frame.at(c, 0xf0));
        c.g[S5][0] = c.ld(frame.at(c, 0xe0));
        c.g[S4][0] = c.ld(frame.at(c, 0xd0));
        c.g[S3][0] = c.ld(frame.at(c, 0xc0));
        c.g[S2][0] = c.ld(frame.at(c, 0xb0));
        c.g[S1][0] = c.ld(frame.at(c, 0xa0));
        c.g[S0][0] = c.ld(frame.at(c, 0x90));
        frame.close(c);
        Ok(())
    })
}

/// `int sceDevVu1GetTpc(void)` (0x3fed60): read VU1's current microprogram
/// counter (VU1 general register 0x43a).
pub fn sce_dev_vu1_get_tpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x43a;
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_ed74);
        c.g[A0][0] = c.g[SP][0];
        callee::VU1_GET_REG.call(c, 0x003f_ed74)?;
        c.g[V0][0] = c.lhu(c.g[SP][0] as u32);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int sceDevVu1Pause(void)` (0x3febd8): pause VU1 by its run state: states
/// 0-4 dispatch through the handler table at 0x497870. The 0x3fec14 arm
/// forces a break (FBRST bit 8) and returns 1, the 0x3fec20 arm returns 1
/// without touching VU1, and the 0x3fec24 arm returns 0. Returns 0 without
/// touching hardware when the state is 5 or more.
pub fn sce_dev_vu1_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[RA][0] = sx(0x003f_ebec);
        c.g[S0][0] = 0u64;
        callee::VU1_GET_STATE.call(c, 0x003f_ebec)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
        let unknown_state = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x490000;
        if !unknown_state {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7870));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            // Jump table (0x003f_ebd8): states 0-4 through 0x497870.
            let t = c.g[A0][0] as u32;
            match t {
                0x003f_ec14 => {
                    c.cycles += 3;
                    c.cfc2(2, 28);
                    c.g[V0][0] = c.g[V0][0] | 0x100;
                    c.ctc2(2, 28);
                    c.cycles += 1;
                    c.g[S0][0] = 0x1;
                }
                0x003f_ec20 => {
                    c.cycles += 1;
                    c.g[S0][0] = 0x1;
                }
                0x003f_ec24 => {}
                _ => {
                    c.jump_out(t, 0x003f_ebd8, 0x003f_ec38);
                    return Ok(());
                }
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int sceDevVu1PutCnd(const u128 *in)` (0x3fef18): restore a saved VU1
/// register context through `libdev_Vu1PutReg` from the caller's 0x23c-byte
/// buffer: VF00-VF31 from `+0x000..0x1ff`, registers 0x430-0x432 and
/// 0x434-0x437 from the words at `+0x200..0x218`, and VI00-VI15 from the
/// halfwords at `+0x21c..0x23b`. Returns 1, or 0 while VU1 is running.
pub fn sce_dev_vu1_put_cnd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x50), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[S3][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[RA][0] = sx(0x003f_ef38);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        callee::VU1_IS_RUNNING.call(c, 0x003f_ef38)?;
        let busy = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0u64;
        if !busy {
            c.cycles += 4;
            c.g[S1][0] = 0u64;
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x21c));
            c.g[S0][0] = c.g[S2][0];
            loop {
                c.cycles += 10;
                c.lq(2, c.g[S0][0] as u32);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x400));
                c.g[A0][0] = c.g[A0][0] & 0xffff;
                c.g[A1][0] = c.g[SP][0];
                c.sq(2, c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003f_ef6c);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                callee::VU1_PUT_REG.call(c, 0x003f_ef6c)?;
                c.g[V0][0] = (c.g[S1][0] < 0x20u64) as u64;
                let more = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_ef50);
                    }
                } else {
                    c.cycles += 40;
                    c.g[V0][0] = c.lwu((c.g[S2][0] as u32).wrapping_add(0x200));
                    c.por(3, 0, 0);
                    c.sq(3, c.g[SP][0] as u32);
                    c.g[A0][0] = 0x430;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003f_ef94);
                    c.sd(c.g[SP][0] as u32, c.g[V0][0]);
                    callee::VU1_PUT_REG.call(c, 0x003f_ef94)?;
                    c.g[S0][0] = c.g[S3][0];
                    c.g[V0][0] = c.lwu((c.g[S2][0] as u32).wrapping_add(0x204));
                    c.g[A0][0] = 0x431;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[S1][0] = 0u64;
                    c.g[RA][0] = sx(0x003f_efb0);
                    c.sd(c.g[SP][0] as u32, c.g[V0][0]);
                    callee::VU1_PUT_REG.call(c, 0x003f_efb0)?;
                    c.g[V0][0] = c.lwu((c.g[S2][0] as u32).wrapping_add(0x208));
                    c.g[A0][0] = 0x432;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003f_efc4);
                    c.sd(c.g[SP][0] as u32, c.g[V0][0]);
                    callee::VU1_PUT_REG.call(c, 0x003f_efc4)?;
                    c.g[V0][0] = c.lwu((c.g[S2][0] as u32).wrapping_add(0x20c));
                    c.g[A0][0] = 0x434;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003f_efd8);
                    c.sd(c.g[SP][0] as u32, c.g[V0][0]);
                    callee::VU1_PUT_REG.call(c, 0x003f_efd8)?;
                    c.g[V0][0] = c.lwu((c.g[S2][0] as u32).wrapping_add(0x210));
                    c.g[A0][0] = 0x435;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003f_efec);
                    c.sd(c.g[SP][0] as u32, c.g[V0][0]);
                    callee::VU1_PUT_REG.call(c, 0x003f_efec)?;
                    c.g[V0][0] = c.lwu((c.g[S2][0] as u32).wrapping_add(0x214));
                    c.g[A0][0] = 0x436;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003f_f000);
                    c.sd(c.g[SP][0] as u32, c.g[V0][0]);
                    callee::VU1_PUT_REG.call(c, 0x003f_f000)?;
                    c.g[V0][0] = c.lwu((c.g[S2][0] as u32).wrapping_add(0x218));
                    c.g[A0][0] = 0x437;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003f_f014);
                    c.sd(c.g[SP][0] as u32, c.g[V0][0]);
                    callee::VU1_PUT_REG.call(c, 0x003f_f014)?;
                    loop {
                        c.cycles += 10;
                        c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x420));
                        c.g[A0][0] = c.g[A0][0] & 0xffff;
                        c.g[A1][0] = c.g[SP][0];
                        c.sd(c.g[SP][0] as u32, c.g[V0][0]);
                        c.g[RA][0] = sx(0x003f_f034);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        callee::VU1_PUT_REG.call(c, 0x003f_f034)?;
                        c.g[V0][0] = (c.g[S1][0] < 0x10u64) as u64;
                        let more = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_f018);
                            }
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break;
                        }
                    }
                    break;
                }
            }
        }
        c.cycles += 7;
        c.g[RA][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `void sceDevVu1Reset(void)` (0x3febc0): reset VU1 (set FBRST bit 9).
pub fn sce_dev_vu1_reset(c: &mut Cpu) {
    c.cycles += 5;
    c.cfc2(2, 28);
    c.g[V0][0] = c.g[V0][0] | 0x200;
    c.ctc2(2, 28);
    let t = c.g[RA][0] as u32;
    c.pc = t;
}

/// `void sceGsResetGraph(short mode, short dx, short dy, short inter)`
/// (0x3fd910): mode 1 resets the GS CSR; mode 0 publishes the display
/// size/interlace into the graph param at `0x451550`, tears down the
/// vsync handler and reprograms the CRT (tail call to `SetGsCrt`);
/// mode 5 republishes without the teardown; other modes return.
pub fn sce_gs_reset_graph(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
        c.sd(frame.at(c, 0x30), c.g[S3][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 16);
        c.sd(frame.at(c, 0x20), c.g[S2][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 16);
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        let sign = (c.g[A0][0] as u32) as i32 >> 16;
        c.g[A0][0] = sx(sign as u32);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        let sign = (c.g[A1][0] as u32) as i32 >> 16;
        c.g[S1][0] = sx(sign as u32);
        let sign = (c.g[A2][0] as u32) as i32 >> 16;
        c.g[S2][0] = sx(sign as u32);
        c.g[V0][0] = 0x1;
        let mode_reset = c.g[A0][0] == c.g[V0][0];
        let sign = (c.g[A3][0] as u32) as i32 >> 16;
        c.g[S3][0] = sx(sign as u32);
        if mode_reset {
            c.cycles += 6;
            c.g[V0][0] = 0x12000000;
            c.g[V1][0] = 0x100;
            c.g[V0][0] = c.g[V0][0] | 0x1000;
            c.sd(c.g[V0][0] as u32, c.g[V1][0]);
            c.g[RA][0] = c.ld(frame.at(c, 0x40));
            c.cycles += 1;
            c.g[S3][0] = c.ld(frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = 0x5;
                c.cycles += 2;
                let mode_crt = c.g[A0][0] == c.g[V0][0];
                c.g[RA][0] = c.ld(frame.at(c, 0x40));
                if mode_crt {
                    c.cycles += 24;
                    c.g[RA][0] = sx(0x003f_da30);
                    callee::GET_GPARAM.call(c, 0x003f_da30)?;
                    c.g[V1][0] = 0x12000000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] | 0x1000;
                    c.g[A2][0] = (0u64 < c.g[S3][0]) as u64;
                    c.g[V0][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A0][0] = c.g[S1][0] & 0x1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.g[A1][0] = c.g[S2][0] & 0xff;
                    c.g[V0][0] = c.g[V0][0] >> 16;
                    c.sh(c.g[S0][0] as u32, c.g[S1][0]);
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x2), c.g[S2][0]);
                    c.g[V0][0] = c.g[V0][0] & 0xffff;
                    c.g[A2][0] = c.g[S3][0] & 0x1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                    c.g[RA][0] = c.ld(frame.at(c, 0x40));
                    c.g[S3][0] = c.ld(frame.at(c, 0x30));
                    c.g[S2][0] = c.ld(frame.at(c, 0x20));
                    c.g[S1][0] = c.ld(frame.at(c, 0x10));
                    c.g[S0][0] = c.ld(frame.at(c, 0));
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
                    c.pc = 0x0042_39a0;
                    ee::s15::SetGsCrt(c);
                    return Ok(());
                } else {
                    c.cycles += 2;
                    c.g[S3][0] = c.ld(frame.at(c, 0x30));
                }
            } else {
                c.cycles += 2;
                let mode_init = c.g[A0][0] == 0u64;
                c.g[RA][0] = c.ld(frame.at(c, 0x40));
                if !mode_init {
                    c.cycles += 2;
                    c.g[S3][0] = c.ld(frame.at(c, 0x30));
                } else {
                    c.cycles += 20;
                    c.g[RA][0] = sx(0x003f_d984);
                    callee::GET_GPARAM.call(c, 0x003f_d984)?;
                    c.g[V1][0] = 0x12000000;
                    c.g[A0][0] = 0x200;
                    c.g[V1][0] = c.g[V1][0] | 0x1000;
                    c.g[S0][0] = c.g[V0][0];
                    c.sd(c.g[V1][0] as u32, c.g[A0][0]);
                    c.sh(c.g[S0][0] as u32, c.g[S1][0]);
                    c.g[A0][0] = 0xff00;
                    c.g[V0][0] = c.ld(c.g[V1][0] as u32);
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x2), c.g[S2][0]);
                    c.g[V0][0] = c.g[V0][0] >> 16;
                    c.g[V0][0] = c.g[V0][0] & 0xff;
                    c.g[V0][0] = c.g[V0][0] & 0xffff;
                    c.g[RA][0] = sx(0x003f_d9bc);
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                    callee::PUT_IMR.call(c, 0x003f_d9bc)?;
                    c.g[V1][0] = (0u64 < c.g[S3][0]) as u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    let no_handler = c.g[V0][0] == 0u64;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    if !no_handler {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x003f_d9d4);
                        c.g[A0][0] = 0x2;
                        callee::DISABLE_INTC.call(c, 0x003f_d9d4)?;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[RA][0] = sx(0x003f_d9e0);
                        c.g[A0][0] = 0x2;
                        callee::REMOVE_INTC_HANDLER.call(c, 0x003f_d9e0)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                    }
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0] & 0x1;
                    c.g[A1][0] = c.g[S2][0] & 0xff;
                    c.g[A2][0] = c.g[S3][0] & 0x1;
                    c.g[RA][0] = c.ld(frame.at(c, 0x40));
                    c.g[S3][0] = c.ld(frame.at(c, 0x30));
                    c.g[S2][0] = c.ld(frame.at(c, 0x20));
                    c.g[S1][0] = c.ld(frame.at(c, 0x10));
                    c.g[S0][0] = c.ld(frame.at(c, 0));
                    c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
                    c.pc = 0x0042_39a0;
                    ee::s15::SetGsCrt(c);
                    return Ok(());
                }
            }
        }
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void sceGsResetPath(void)` (0x3fdab0): reset the GIF/VIF1 path: reset
/// VIF1 (`VIF1_FBRST` `0x10003c10` = 1) and write 2 to `VIF1_ERR`
/// (`0x10003c20`, masking DMAtag-mismatch errors), set the VU1 reset bit
/// (`0x200`) in FBRST (COP2 control 28), copy the 32-byte packet at
/// `0x451560` into the VIF1 FIFO (`0x10005000`) and reset the GIF (`GIF_CTRL`
/// `0x10003000` = 1). Leaf: no calls, no branches.
pub fn sce_gs_reset_path(c: &mut Cpu) {
    c.cycles += 26;
    c.g[V0][0] = 0x10000000;
    c.g[A3][0] = 0x1;
    c.g[V0][0] = c.g[V0][0] | 0x3c10;
    c.g[V1][0] = 0x10000000;
    c.sw(c.g[V0][0] as u32, c.g[A3][0]);
    c.g[V1][0] = c.g[V1][0] | 0x3c20;
    c.g[V0][0] = 0x2;
    c.g[A0][0] = 0u64;
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.cfc2(A0, 28);
    c.g[A0][0] = c.g[A0][0] | 0x200;
    c.ctc2(A0, 28);
    c.g[A1][0] = 0x450000;
    c.g[A2][0] = 0x10000000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1560));
    c.g[A2][0] = c.g[A2][0] | 0x5000;
    c.lq(4, c.g[A1][0] as u32);
    c.g[V1][0] = 0x10000000;
    c.g[V1][0] = c.g[V1][0] | 0x3000;
    c.sq(4, c.g[A2][0] as u32);
    c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.sq(2, c.g[A2][0] as u32);
    let t = c.g[RA][0] as u32;
    c.sw(c.g[V1][0] as u32, c.g[A3][0]);
    c.pc = t;
}

/// `int sceGsSyncPath(int mode)` (0x3fdb18): with nonzero `mode`,
/// sample the VIF1/GIF path-busy bits into a bitmask (VIF1 CHCR 1,
/// GIF CHCR 2, VIF1 STAT 4, VPU-STAT 8, GIF STAT 16). With `mode`
/// 0, wait for every stage to go idle (shared `0x1000000` spin
/// budget in `a1`, eleven `scePrintf` dumps and `-1` on timeout).
pub fn sce_gs_sync_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        'b_3fde20: {
            'b_3fddac: {
                c.g[A1][0] = 0u64;
                let status = c.g[A0][0] != 0u64;
                c.sd(frame.at(c, 0), c.g[RA][0]);
                if status {
                    break 'b_3fddac;
                } else {
                    'b_3fdcac: {
                        'b_3fdca4: {
                            'b_3fdc98: {
                                'b_3fdc8c: {
                                    'b_3fdc80: {
                                        'b_3fdc74: {
                                            'b_3fdb6c: {
                                                c.cycles += 6;
                                                c.g[V0][0] = 0x10000000;
                                                c.g[V0][0] = c.g[V0][0] | 0x9000;
                                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                c.g[V1][0] = c.g[V1][0] & 0x100;
                                                let idle = c.g[V1][0] == 0u64;
                                                c.g[V1][0] = 0x10000000;
                                                if idle {
                                                    break 'b_3fdb6c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[A0][0] = 0x1000000;
                                                    c.g[V1][0] = c.g[V1][0] | 0x9000;
                                                    c.g[V0][0] = c.g[A1][0];
                                                    'l_3fdb50: loop {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            (c.g[A0][0] < c.g[V0][0]) as u64;
                                                        let expired = c.g[V0][0] != 0u64;
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if expired {
                                                            break 'b_3fdc74;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                            c.g[V0][0] = c.g[V0][0] & 0x100;
                                                            let busy = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.g[A1][0];
                                                            if busy {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c,
                                                                        0x003f_db50,
                                                                    );
                                                                }
                                                                continue 'l_3fdb50;
                                                            } else {
                                                                break 'b_3fdb6c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            'b_3fdbac: {
                                                c.cycles += 6;
                                                c.g[V0][0] = 0x10000000;
                                                c.g[V0][0] = c.g[V0][0] | 0xa000;
                                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                c.g[V1][0] = c.g[V1][0] & 0x100;
                                                let idle = c.g[V1][0] == 0u64;
                                                c.g[V1][0] = 0x10000000;
                                                if idle {
                                                    break 'b_3fdbac;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = 0x1000000;
                                                    c.g[V1][0] = c.g[V1][0] | 0xa000;
                                                    c.g[V0][0] = c.g[A1][0];
                                                    'l_3fdb90: loop {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            (c.g[A0][0] < c.g[V0][0]) as u64;
                                                        let expired = c.g[V0][0] != 0u64;
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if expired {
                                                            break 'b_3fdc80;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                            c.g[V0][0] = c.g[V0][0] & 0x100;
                                                            let busy = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.g[A1][0];
                                                            if busy {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c,
                                                                        0x003f_db90,
                                                                    );
                                                                }
                                                                continue 'l_3fdb90;
                                                            } else {
                                                                break 'b_3fdbac;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            'b_3fdbfc: {
                                                c.cycles += 8;
                                                c.g[V0][0] = 0x10000000;
                                                c.g[A0][0] = 0x1f000000;
                                                c.g[V0][0] = c.g[V0][0] | 0x3c00;
                                                c.g[A0][0] = c.g[A0][0] | 0x3;
                                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                let idle = c.g[V1][0] == 0u64;
                                                c.g[V1][0] = 0x1f000000;
                                                if idle {
                                                    break 'b_3fdbfc;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[A0][0] = 0x10000000;
                                                    c.g[A2][0] = 0x1000000;
                                                    c.g[A0][0] = c.g[A0][0] | 0x3c00;
                                                    c.g[V1][0] = c.g[V1][0] | 0x3;
                                                    c.g[V0][0] = c.g[A1][0];
                                                    'l_3fdbe0: loop {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            (c.g[A2][0] < c.g[V0][0]) as u64;
                                                        let expired = c.g[V0][0] != 0u64;
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if expired {
                                                            break 'b_3fdc8c;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                                            let busy = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.g[A1][0];
                                                            if busy {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c,
                                                                        0x003f_dbe0,
                                                                    );
                                                                }
                                                                continue 'l_3fdbe0;
                                                            } else {
                                                                break 'b_3fdbfc;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            'b_3fdc2c: {
                                                c.cycles += 4;
                                                c.cfc2(A2, 29);
                                                c.g[V0][0] = c.g[A2][0] & 0x100;
                                                let idle = c.g[V0][0] == 0u64;
                                                c.g[V1][0] = 0x1000000;
                                                if idle {
                                                    break 'b_3fdc2c;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c.g[A1][0];
                                                    'l_3fdc10: loop {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            (c.g[V1][0] < c.g[V0][0]) as u64;
                                                        let expired = c.g[V0][0] != 0u64;
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if expired {
                                                            break 'b_3fdc98;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.cfc2(A2, 29);
                                                            c.g[V0][0] = c.g[A2][0] & 0x100;
                                                            let busy = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.g[A1][0];
                                                            if busy {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c,
                                                                        0x003f_dc10,
                                                                    );
                                                                }
                                                                continue 'l_3fdc10;
                                                            } else {
                                                                break 'b_3fdc2c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            'b_3fdc6c: {
                                                c.cycles += 6;
                                                c.g[V0][0] = 0x10000000;
                                                c.g[V0][0] = c.g[V0][0] | 0x3020;
                                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                c.g[V1][0] = c.g[V1][0] & 0xc00;
                                                let idle = c.g[V1][0] == 0u64;
                                                c.g[V1][0] = 0x10000000;
                                                if idle {
                                                    break 'b_3fdc6c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = 0x1000000;
                                                    c.g[V1][0] = c.g[V1][0] | 0x3020;
                                                    c.g[V0][0] = c.g[A1][0];
                                                    'l_3fdc50: loop {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            (c.g[A0][0] < c.g[V0][0]) as u64;
                                                        let expired = c.g[V0][0] != 0u64;
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if expired {
                                                            break 'b_3fdca4;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                            c.g[V0][0] = c.g[V0][0] & 0xc00;
                                                            let busy = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.g[A1][0];
                                                            if busy {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c,
                                                                        0x003f_dc50,
                                                                    );
                                                                }
                                                                continue 'l_3fdc50;
                                                            } else {
                                                                break 'b_3fdc6c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = 0u64;
                                            break 'b_3fde20;
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x490000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7648));
                                        break 'b_3fdcac;
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x490000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7728));
                                    break 'b_3fdcac;
                                }
                                c.cycles += 3;
                                c.g[A0][0] = 0x490000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7758));
                                break 'b_3fdcac;
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x490000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7788));
                            break 'b_3fdcac;
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0x490000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x77b0));
                        break 'b_3fdcac;
                    }
                    c.cycles += 64;
                    c.g[RA][0] = sx(0x003f_dcb4);
                    callee::PRINTF.call(c, 0x003f_dcb4)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0x9000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7678));
                    c.g[RA][0] = sx(0x003f_dccc);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dccc)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0x9030;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7688));
                    c.g[RA][0] = sx(0x003f_dce4);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dce4)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0x9010;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7698));
                    c.g[RA][0] = sx(0x003f_dcfc);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dcfc)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0x9020;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76a8));
                    c.g[RA][0] = sx(0x003f_dd14);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dd14)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0xa000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76b8));
                    c.g[RA][0] = sx(0x003f_dd2c);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dd2c)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0xa030;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76c8));
                    c.g[RA][0] = sx(0x003f_dd44);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dd44)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0xa010;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76d8));
                    c.g[RA][0] = sx(0x003f_dd5c);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dd5c)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0xa020;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76e8));
                    c.g[RA][0] = sx(0x003f_dd74);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dd74)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0x3c00;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x76f8));
                    c.g[RA][0] = sx(0x003f_dd8c);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dd8c)?;
                    c.g[V0][0] = 0x10000000;
                    c.g[A0][0] = 0x490000;
                    c.g[V0][0] = c.g[V0][0] | 0x3020;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7710));
                    c.g[RA][0] = sx(0x003f_dda4);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    callee::PRINTF.call(c, 0x003f_dda4)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break 'b_3fde20;
                }
            }
            c.cycles += 29;
            c.g[V1][0] = 0x10000000;
            c.g[A1][0] = 0x10000000;
            c.g[V1][0] = c.g[V1][0] | 0x9000;
            c.g[A1][0] = c.g[A1][0] | 0xa000;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[A0][0] = 0x10000000;
            c.g[A2][0] = c.lw(c.g[A1][0] as u32);
            c.g[A0][0] = c.g[A0][0] | 0x3c00;
            c.g[V0][0] = c.g[V0][0] & 0x100;
            c.g[A1][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[A2][0] = c.g[A2][0] & 0x100;
            c.g[A0][0] = c.g[V0][0] | 0x2;
            c.g[V1][0] = 0x1f000000;
            if c.g[A2][0] != 0 {
                c.g[V0][0] = c.g[A0][0];
            }
            c.g[V1][0] = c.g[V1][0] | 0x3;
            c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
            c.g[A0][0] = c.g[V0][0] | 0x4;
            if c.g[A1][0] != 0 {
                c.g[V0][0] = c.g[A0][0];
            }
            c.cfc2(A2, 29);
            c.g[V1][0] = 0x10000000;
            c.g[A2][0] = c.g[A2][0] & 0x100;
            c.g[V1][0] = c.g[V1][0] | 0x3020;
            c.g[A1][0] = c.g[V0][0] | 0x8;
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            if c.g[A2][0] != 0 {
                c.g[V0][0] = c.g[A1][0];
            }
            c.g[V1][0] = c.g[V0][0] | 0x10;
            c.g[A0][0] = c.g[A0][0] & 0xc00;
            if c.g[A0][0] != 0 {
                c.g[V0][0] = c.g[V1][0];
            }
            break 'b_3fde20;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(frame.at(c, 0));
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::machine;
    use crate::difftest::require_paths;
    use crate::difftest::DATA;
    use ssx3_ee_rt::Cpu;

    /// `int libdev_Vif0GetState(void)`: VIF0 idle (0), stalled (1),
    /// waiting (2), in error (3) or busy with no flag (4), with a non-zero
    /// word at RAM 0.
    #[test]
    fn libdev_vif0_get_state_matches() {
        let dev_vif0_get_state_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_000f, 0);
            c.pc = 0x003f_f160;
            ram0(c);
            vif0_in_state(c, seed % 5, seed / 5);
        };
        check(
            ee::s15::f_003ff160__generated,
            libdev_vif0_get_state,
            SEEDS,
            dev_vif0_get_state_call,
        );
        require_paths(
            ee::s15::f_003ff160__generated,
            SEEDS,
            &[8, 10, 12, 14],
            dev_vif0_get_state_call,
        );
    }

    /// `int libdev_Vu0GetState(void)`: idle (0) and VU0-running (4). Bits
    /// 1-3 are never set in the emulator's VPU status word, so states 1-3
    /// are unreachable.
    #[test]
    fn libdev_vu0_get_state_matches() {
        let dev_vu0_get_state_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1249_0005, 0);
            c.pc = 0x003f_e7d8;
            vu0_running(c, seed % 3 == 1, seed % 2 == 1);
        };
        check(
            ee::s15::f_003fe7d8__generated,
            libdev_vu0_get_state,
            SEEDS,
            dev_vu0_get_state_call,
        );
        require_paths(
            ee::s15::f_003fe7d8__generated,
            SEEDS,
            &[10],
            dev_vu0_get_state_call,
        );
    }

    /// `int libdev_Vu0IsRunning(void)`: 1 while VU0 runs (low nibble 1),
    /// else 0.
    #[test]
    fn libdev_vu0_is_running_matches() {
        let dev_vu0_is_running_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1249_0009, 0);
            c.pc = 0x003f_e678;
            let (vu0, vu1) = match seed % 4 {
                1 => (true, false),
                2 => (false, true),
                3 => (true, true),
                _ => (false, false),
            };
            vu0_running(c, vu0, vu1);
        };
        check(
            ee::s15::f_003fe678__generated,
            libdev_vu0_is_running,
            SEEDS,
            dev_vu0_is_running_call,
        );
        require_paths(
            ee::s15::f_003fe678__generated,
            SEEDS,
            &[5],
            dev_vu0_is_running_call,
        );
    }

    /// `void libdev_Vu1GetReg(out, reg)`: register indices across the VU1
    /// range into a scratch buffer, VU0 data memory filled with random
    /// words.
    #[test]
    fn libdev_vu1_get_reg_matches() {
        const REGS: [u32; 8] = [0, 0x400, 0x420, 0x430, 0x431, 0x437, 0x43a, 0xffff];
        let dev_vu1_get_reg_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1248_0003, 0);
            c.pc = 0x003f_eb78;
            c.g[A1][0] = REGS[seed as usize % REGS.len()] as u64;
            c.g[A0][0] = (DATA + 0x100) as u64;
            let m = ssx3_ps2::platform::machine_of(&mut c.bus.plat);
            for w in m.hw.vu0.data.chunks_mut(4) {
                w.copy_from_slice(&r.next_u32().to_le_bytes());
            }
        };
        check(
            ee::s15::f_003feb78__generated,
            libdev_vu1_get_reg,
            SEEDS,
            dev_vu1_get_reg_call,
        );
        require_paths(
            ee::s15::f_003feb78__generated,
            SEEDS,
            &[17],
            dev_vu1_get_reg_call,
        );
    }

    /// `int libdev_Vu1GetState(void)`: idle (0) and VU1-running (4). Bits
    /// 9-11 are never set in the emulator's VPU status word, so states
    /// 1-3 are unreachable.
    #[test]
    fn libdev_vu1_get_state_matches() {
        let dev_vu1_get_state_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_0007, 0);
            c.pc = 0x003f_ed08;
            vu_running(c, seed % 3 == 1, seed % 2 == 1);
        };
        check(
            ee::s15::f_003fed08__generated,
            libdev_vu1_get_state,
            SEEDS,
            dev_vu1_get_state_call,
        );
        require_paths(
            ee::s15::f_003fed08__generated,
            SEEDS,
            &[12],
            dev_vu1_get_state_call,
        );
    }

    /// `int libdev_Vu1IsRunning(void)`: VU0/VU1 running flags off (0), VU0
    /// on (1), VU1 on (1), both on (1).
    #[test]
    fn libdev_vu1_is_running_matches() {
        let dev_vu1_is_running_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_0001, 0);
            c.pc = 0x003f_eb08;
            let (vu0, vu1) = match seed % 4 {
                1 => (true, false),
                2 => (false, true),
                3 => (true, true),
                _ => (false, false),
            };
            vu_running(c, vu0, vu1);
        };
        check(
            ee::s15::f_003feb08__generated,
            libdev_vu1_is_running,
            SEEDS,
            dev_vu1_is_running_call,
        );
        require_paths(
            ee::s15::f_003feb08__generated,
            SEEDS,
            &[7, 9],
            dev_vu1_is_running_call,
        );
    }

    /// `void libdev_Vu1PutReg(reg, val)`: register indices across the VU1
    /// range with random 16-byte values; VU0 data memory must match after
    /// both runs.
    #[test]
    fn libdev_vu1_put_reg_matches() {
        const REGS: [u32; 8] = [0, 1, 0x42, 0x3ff, 0x400, 0x420, 0x430, 0x437];
        let dev_vu1_put_reg_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1248_0002, 0);
            c.pc = 0x003f_eb38;
            c.g[A0][0] = REGS[seed as usize % REGS.len()] as u64;
            let val = DATA + 0x100;
            c.g[A1][0] = val as u64;
            for i in 0..4 {
                c.bus.write32(val + 4 * i, r.next_u32());
            }
        };
        check(
            ee::s15::f_003feb38__generated,
            libdev_vu1_put_reg,
            SEEDS,
            dev_vu1_put_reg_call,
        );
        check_hw(
            ee::s15::f_003feb38__generated,
            libdev_vu1_put_reg,
            SEEDS,
            dev_vu1_put_reg_call,
            &|c| {
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .hw
                    .vu0
                    .data
                    .clone()
            },
        );
        require_paths(
            ee::s15::f_003feb38__generated,
            SEEDS,
            &[16],
            dev_vu1_put_reg_call,
        );
    }

    /// `int sceDevVif0Continue(void)`: VIF0 idle (returns 0) or stalled
    /// (state 1: writes 8 to FBRST, the FIFO drains, returns 1).
    #[test]
    fn sce_dev_vif0_continue_matches() {
        let dev_vif0_continue_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_000e, 0);
            c.pc = 0x003f_f0f0;
            vif0_in_state(c, seed % 2, seed / 2);
        };
        check(
            ee::s15::f_003ff0f0__generated,
            sce_dev_vif0_continue,
            SEEDS,
            dev_vif0_continue_call,
        );
        require_paths(
            ee::s15::f_003ff0f0__generated,
            SEEDS,
            &[18, 25],
            dev_vif0_continue_call,
        );
    }

    /// `int sceDevVif0GetCnd(out)`: VIF0 states 0 and 1 take the full
    /// 0x38-byte register capture returning 1, states 2-4 return 0 early.
    /// The VIF0 registers are zero or random, the buffer pre-filled, and
    /// RAM 0 holds a non-zero word.
    #[test]
    fn sce_dev_vif0_get_cnd_matches() {
        let dev_vif0_get_cnd_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1248_0010, 0);
            c.pc = 0x003f_f1b0;
            c.g[A0][0] = VIF_CND_BUF as u64;
            crate::difftest::fill(c, &mut r, VIF_CND_BUF, 14);
            ram0(c);
            vif0_in_state(c, seed % 5, seed / 5);
            if seed / 10 % 2 == 1 {
                let v = &mut ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vif0;
                for (i, w) in v.row.iter_mut().chain(v.col.iter_mut()).enumerate() {
                    *w = r.next_u32() ^ i as u32;
                }
                for w in [&mut v.mask, &mut v.code, &mut v.itops, &mut v.mark] {
                    *w = r.next_u32();
                }
                for w in [&mut v.num, &mut v.err, &mut v.mode, &mut v.itop] {
                    *w = r.next_u32();
                }
                v.cycle = r.next_u32() | 0x100;
            }
        };
        check(
            ee::s15::f_003ff1b0__generated,
            sce_dev_vif0_get_cnd,
            SEEDS,
            dev_vif0_get_cnd_call,
        );
        require_paths(
            ee::s15::f_003ff1b0__generated,
            SEEDS,
            &[37, 39, 102, 104],
            dev_vif0_get_cnd_call,
        );
    }

    /// `int sceDevVif0Pause(mode)`: VIF0 states 0-4 crossed with modes 0, 1,
    /// 2 and 5: idle (0) and just stalled (1) return 0; from state 2 on,
    /// mode 0 writes 2 and mode 1 writes 4 to FBRST, and every mode returns
    /// 1. RAM 0 holds a non-zero word.
    #[test]
    fn sce_dev_vif0_pause_matches() {
        let dev_vif0_pause_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_000d, 0);
            c.pc = 0x003f_f088;
            c.g[A0][0] = [0, 1, 2, 5][seed as usize % 4] as u64;
            ram0(c);
            vif0_in_state(c, seed / 4 % 5, seed / 20);
        };
        check(
            ee::s15::f_003ff088__generated,
            sce_dev_vif0_pause,
            SEEDS,
            dev_vif0_pause_call,
        );
        require_paths(
            ee::s15::f_003ff088__generated,
            SEEDS,
            &[21, 25, 29, 31, 32, 34, 36],
            dev_vif0_pause_call,
        );
    }

    /// `void sceDevVif0Reset(void)`: leaf MMIO sequence, no branches.
    #[test]
    fn sce_dev_vif0_reset_matches() {
        let dev_vif0_reset_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_000c, 0);
            c.pc = 0x003f_f060;
        };
        check(
            ee::s15::f_003ff060__generated,
            sce_dev_vif0_reset,
            SEEDS,
            dev_vif0_reset_call,
        );
        require_paths(
            ee::s15::f_003ff060__generated,
            SEEDS,
            &[9],
            dev_vif0_reset_call,
        );
    }

    /// `int sceDevVu0Continue(void)`: states 0 and 4 both return 0. The
    /// VPU status word never reports states 1-3, so on half the seeds a
    /// gated fake of `libdev_Vu0GetState` returns 1-3: the GetTpc/Exec leg
    /// restarts VU0 at its TPC and returns 1. CMSAR0 and the VU0 TPC must
    /// match after both runs.
    #[test]
    fn sce_dev_vu0_continue_matches() {
        let dev_vu0_continue_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1249_0006, 0);
            c.pc = 0x003f_e708;
            vu0_running(c, seed % 2 == 1, false);
            forced_vu_state(c, (seed / 2 % 2 == 1).then_some(1 + seed / 4 % 3));
        };
        check(
            ee::s15::f_003fe708__generated,
            sce_dev_vu0_continue,
            SEEDS,
            dev_vu0_continue_call,
        );
        check_hw(
            ee::s15::f_003fe708__generated,
            sce_dev_vu0_continue,
            SEEDS,
            dev_vu0_continue_call,
            &|c| {
                let vu0 = &ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0;
                (vu0.cmsar0, vu0.tpc)
            },
        );
        require_paths(
            ee::s15::f_003fe708__generated,
            SEEDS,
            &[23, 24, 31],
            dev_vu0_continue_call,
        );
    }

    /// `void sceDevVu0Exec(tpc)`: masked address write to CMSAR0 plus the
    /// microprogram run (stops at once: no translated microcode on the
    /// fixture machine), no branches. CMSAR0 and the VU0 TPC must match
    /// after both runs.
    #[test]
    fn sce_dev_vu0_exec_matches() {
        let dev_vu0_exec_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_0004, 0);
            c.pc = 0x003f_e818;
            c.g[A0][0] = if seed % 3 == 0 {
                0
            } else {
                r.next_u32() as u64
            };
        };
        check(
            ee::s15::f_003fe818__generated,
            sce_dev_vu0_exec,
            SEEDS,
            dev_vu0_exec_call,
        );
        check_hw(
            ee::s15::f_003fe818__generated,
            sce_dev_vu0_exec,
            SEEDS,
            dev_vu0_exec_call,
            &|c| {
                let vu0 = &ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0;
                (vu0.cmsar0, vu0.tpc)
            },
        );
        require_paths(
            ee::s15::f_003fe818__generated,
            SEEDS,
            &[4],
            dev_vu0_exec_call,
        );
    }

    /// `int sceDevVu0GetCnd(out)`: early 0 while VU0 runs, else the full
    /// 32+7 register capture returning 1.
    #[test]
    fn sce_dev_vu0_get_cnd_matches() {
        let dev_vu0_get_cnd_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1249_0002, 0);
            c.pc = 0x003f_e838;
            c.g[A0][0] = CND_BUF as u64;
            let (vu0, vu1) = match seed % 4 {
                1 => (true, false),
                2 => (false, true),
                3 => (true, true),
                _ => (false, false),
            };
            vu0_running(c, vu0, vu1);
        };
        check(
            ee::s15::f_003fe838__generated,
            sce_dev_vu0_get_cnd,
            SEEDS,
            dev_vu0_get_cnd_call,
        );
        require_paths(
            ee::s15::f_003fe838__generated,
            SEEDS,
            &[16, 93],
            dev_vu0_get_cnd_call,
        );
    }

    /// `int sceDevVu0GetTpc(void)`: leaf COP2 read of the TPC halfword,
    /// no branches.
    #[test]
    fn sce_dev_vu0_get_tpc_matches() {
        let dev_vu0_get_tpc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1249_0003, 0);
            c.pc = 0x003f_e828;
        };
        check(
            ee::s15::f_003fe828__generated,
            sce_dev_vu0_get_tpc,
            SEEDS,
            dev_vu0_get_tpc_call,
        );
        require_paths(
            ee::s15::f_003fe828__generated,
            SEEDS,
            &[3],
            dev_vu0_get_tpc_call,
        );
    }

    /// `int sceDevVu0Pause(void)`: state 0 takes the early return, state 4
    /// dispatches through the handler table (all three arms across seeds).
    /// The emulator's VPU status word only ever reports 0 (idle) and 4 (VU0
    /// running), so on some seeds a gated fake of `libdev_Vu0GetState`
    /// returns 1-3 (dispatched through the table) or 5-6 (past it). FBRST
    /// and the VU0 running flag must match after both runs.
    #[test]
    fn sce_dev_vu0_pause_matches() {
        const ARMS: [u32; 3] = [0x003f_e6e4, 0x003f_e6f0, 0x003f_e6f4];
        let dev_vu0_pause_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1249_0007, 0);
            c.pc = 0x003f_e6a8;
            vu0_running(c, seed / 5 % 2 == 1, false);
            forced_vu_state(
                c,
                (seed / 10 % 4 == 3).then_some([1, 2, 3, 5, 6][seed as usize / 40 % 5]),
            );
            for s in 0..6u32 {
                c.bus
                    .write32(VU0_STATE_TABLE + 4 * s, ARMS[(seed + s) as usize % 3]);
            }
        };
        check(
            ee::s15::f_003fe6a8__generated,
            sce_dev_vu0_pause,
            SEEDS,
            dev_vu0_pause_call,
        );
        check_hw(
            ee::s15::f_003fe6a8__generated,
            sce_dev_vu0_pause,
            SEEDS,
            dev_vu0_pause_call,
            &|c| {
                let vu0 = &ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0;
                (vu0.fbrst, vu0.running)
            },
        );
        require_paths(
            ee::s15::f_003fe6a8__generated,
            SEEDS,
            &[18, 24, 25, 28, 30, 31, 34],
            dev_vu0_pause_call,
        );
    }

    /// `int sceDevVu0PutCnd(in)`: early 0 while VU0 runs, else the full
    /// 32+7 register restore returning 1.
    #[test]
    fn sce_dev_vu0_put_cnd_matches() {
        let dev_vu0_put_cnd_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1249_0001, 0);
            c.pc = 0x003f_e998;
            c.g[A0][0] = CND_BUF as u64;
            for i in 0..0x8e {
                c.bus.write32(CND_BUF + 4 * i, r.next_u32());
            }
            let (vu0, vu1) = match seed % 4 {
                1 => (true, false),
                2 => (false, true),
                3 => (true, true),
                _ => (false, false),
            };
            vu0_running(c, vu0, vu1);
        };
        check(
            ee::s15::f_003fe998__generated,
            sce_dev_vu0_put_cnd,
            SEEDS,
            dev_vu0_put_cnd_call,
        );
        require_paths(
            ee::s15::f_003fe998__generated,
            SEEDS,
            &[19, 96],
            dev_vu0_put_cnd_call,
        );
    }

    /// `void sceDevVu0Reset(void)`: leaf COP2 sequence, no branches.
    #[test]
    fn sce_dev_vu0_reset_matches() {
        let dev_vu0_reset_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1249_0008, 0);
            c.pc = 0x003f_e690;
        };
        check(
            ee::s15::f_003fe690__generated,
            sce_dev_vu0_reset,
            SEEDS,
            dev_vu0_reset_call,
        );
        require_paths(
            ee::s15::f_003fe690__generated,
            SEEDS,
            &[5],
            dev_vu0_reset_call,
        );
    }

    /// `int sceDevVu1Continue(void)`: states 0 and 4 both return 0. The
    /// VPU status word never reports states 1-3, so on half the seeds a
    /// gated fake of `libdev_Vu1GetState` returns 1-3: the GetTpc/Exec leg
    /// reads the VU1 TPC, requests a VU1 start there and returns 1. The VU1
    /// start request must match after both runs.
    #[test]
    fn sce_dev_vu1_continue_matches() {
        let dev_vu1_continue_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_0006, 0);
            c.pc = 0x003f_ec38;
            vu_running(c, false, seed % 2 == 1);
            forced_vu_state(c, (seed / 2 % 2 == 1).then_some(1 + seed / 4 % 3));
        };
        check(
            ee::s15::f_003fec38__generated,
            sce_dev_vu1_continue,
            SEEDS,
            dev_vu1_continue_call,
        );
        check_hw(
            ee::s15::f_003fec38__generated,
            sce_dev_vu1_continue,
            SEEDS,
            dev_vu1_continue_call,
            &|c| {
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .hw
                    .vu0
                    .cmsar1_request
            },
        );
        require_paths(
            ee::s15::f_003fec38__generated,
            SEEDS,
            &[25, 26, 54],
            dev_vu1_continue_call,
        );
    }

    /// `void sceDevVu1Exec(tpc)`: masked address writes to CMSAR1, no
    /// branches. The VU1 start request must match after both runs.
    #[test]
    fn sce_dev_vu1_exec_matches() {
        let dev_vu1_exec_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1248_0008, 0);
            c.pc = 0x003f_ed50;
            c.g[A0][0] = if seed % 3 == 0 {
                0
            } else {
                r.next_u32() as u64
            };
        };
        check(
            ee::s15::f_003fed50__generated,
            sce_dev_vu1_exec,
            SEEDS,
            dev_vu1_exec_call,
        );
        check_hw(
            ee::s15::f_003fed50__generated,
            sce_dev_vu1_exec,
            SEEDS,
            dev_vu1_exec_call,
            &|c| {
                ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                    .hw
                    .vu0
                    .cmsar1_request
            },
        );
        require_paths(
            ee::s15::f_003fed50__generated,
            SEEDS,
            &[4],
            dev_vu1_exec_call,
        );
    }

    /// `int sceDevVu1GetCnd(out)`: the same capture loops with real events
    /// on a running thread (structured back edges: the unwind request is
    /// deferred, both sides loop on).
    #[test]
    fn sce_dev_vu1_get_cnd_events() {
        check_events(
            ee::s15::f_003fed88__generated,
            sce_dev_vu1_get_cnd,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1248_001a, 0);
                c.pc = 0x003f_ed88;
                c.g[A0][0] = CND_BUF as u64;
                vu_running(c, false, seed % 2 == 1);
                schedule(c, &mut r, seed, 0x003f_ed88)
            },
        );
    }

    /// `int sceDevVu1GetCnd(out)`: early 0 while VU1 runs, else the full
    /// 32+7 register capture returning 1.
    #[test]
    fn sce_dev_vu1_get_cnd_matches() {
        let dev_vu1_get_cnd_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_000a, 0);
            c.pc = 0x003f_ed88;
            c.g[A0][0] = CND_BUF as u64;
            vu_running(c, seed % 5 == 1, seed % 2 == 1);
        };
        check(
            ee::s15::f_003fed88__generated,
            sce_dev_vu1_get_cnd,
            SEEDS,
            dev_vu1_get_cnd_call,
        );
        require_paths(
            ee::s15::f_003fed88__generated,
            SEEDS,
            &[34, 36, 1503],
            dev_vu1_get_cnd_call,
        );
    }

    /// `int sceDevVu1GetTpc(void)`: the TPC halfword of VU1 register 0x43a
    /// (varies with the scrambled VU0 VF state the leaf reads).
    #[test]
    fn sce_dev_vu1_get_tpc_matches() {
        let dev_vu1_get_tpc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_0009, 0);
            c.pc = 0x003f_ed60;
        };
        check(
            ee::s15::f_003fed60__generated,
            sce_dev_vu1_get_tpc,
            SEEDS,
            dev_vu1_get_tpc_call,
        );
        require_paths(
            ee::s15::f_003fed60__generated,
            SEEDS,
            &[26],
            dev_vu1_get_tpc_call,
        );
    }

    /// `int sceDevVu1Pause(void)`: state 0 takes the early return, state 4
    /// dispatches through the handler table (all three arms across seeds).
    /// The emulator's VPU status word only ever reports 0 (idle) and 4 (VU1
    /// running), so on some seeds a gated fake of `libdev_Vu1GetState`
    /// returns 1-3 (dispatched through the table) or 5-6 (past it). FBRST
    /// must match after both runs.
    #[test]
    fn sce_dev_vu1_pause_matches() {
        const ARMS: [u32; 3] = [0x003f_ec14, 0x003f_ec20, 0x003f_ec24];
        let dev_vu1_pause_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_0005, 0);
            c.pc = 0x003f_ebd8;
            c.g[A0][0] = (seed % 3) as u64;
            vu_running(c, false, seed / 5 % 2 == 1);
            forced_vu_state(
                c,
                (seed / 10 % 4 == 3).then_some([1, 2, 3, 5, 6][seed as usize / 40 % 5]),
            );
            for s in 0..6u32 {
                c.bus
                    .write32(VU1_STATE_TABLE + 4 * s, ARMS[(seed + s) as usize % 3]);
            }
        };
        check(
            ee::s15::f_003febd8__generated,
            sce_dev_vu1_pause,
            SEEDS,
            dev_vu1_pause_call,
        );
        check_hw(
            ee::s15::f_003febd8__generated,
            sce_dev_vu1_pause,
            SEEDS,
            dev_vu1_pause_call,
            &|c| ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0.fbrst,
        );
        require_paths(
            ee::s15::f_003febd8__generated,
            SEEDS,
            &[18, 24, 25, 28, 32, 33, 36],
            dev_vu1_pause_call,
        );
    }

    /// `int sceDevVu1PutCnd(in)`: the same restore loops with real events
    /// on a running thread.
    #[test]
    fn sce_dev_vu1_put_cnd_events() {
        check_events(
            ee::s15::f_003fef18__generated,
            sce_dev_vu1_put_cnd,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1248_001b, 0);
                c.pc = 0x003f_ef18;
                c.g[A0][0] = CND_BUF as u64;
                for i in 0..0x88 {
                    c.bus.write32(CND_BUF + 4 * i, r.next_u32());
                }
                vu_running(c, false, false);
                schedule(c, &mut r, seed, 0x003f_ef18)
            },
        );
    }

    /// `int sceDevVu1PutCnd(in)`: early 0 while VU1 runs (VU0 running, or
    /// the VU1 peer flag set with a clear low nibble), else the full
    /// restore returning 1.
    #[test]
    fn sce_dev_vu1_put_cnd_matches() {
        let dev_vu1_put_cnd_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1248_000b, 0);
            c.pc = 0x003f_ef18;
            c.g[A0][0] = CND_BUF as u64;
            for i in 0..0x88 {
                c.bus.write32(CND_BUF + 4 * i, r.next_u32());
            }
            let (vu0, vu1) = match seed % 4 {
                1 => (true, false),
                2 => (false, true),
                3 => (true, true),
                _ => (false, false),
            };
            vu_running(c, vu0, vu1);
        };
        check(
            ee::s15::f_003fef18__generated,
            sce_dev_vu1_put_cnd,
            SEEDS,
            dev_vu1_put_cnd_call,
        );
        require_paths(
            ee::s15::f_003fef18__generated,
            SEEDS,
            &[24, 26, 1431],
            dev_vu1_put_cnd_call,
        );
    }

    /// `void sceDevVu1Reset(void)`: leaf COP2 sequence, no branches.
    #[test]
    fn sce_dev_vu1_reset_matches() {
        let dev_vu1_reset_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_0004, 0);
            c.pc = 0x003f_ebc0;
        };
        check(
            ee::s15::f_003febc0__generated,
            sce_dev_vu1_reset,
            SEEDS,
            dev_vu1_reset_call,
        );
        require_paths(
            ee::s15::f_003febc0__generated,
            SEEDS,
            &[5],
            dev_vu1_reset_call,
        );
    }

    /// `void sceGsResetGraph(mode, dx, dy, inter)`: modes 1 (CSR
    /// reset), 0 (publish + optional vsync teardown + `SetGsCrt`
    /// tail call), 5 (publish + `SetGsCrt`, no teardown) and the
    /// plain-return modes 2, -1 and 7. The teardown arm runs the
    /// real `DisableIntc` plus `RemoveIntcHandler`/`SetGsCrt`
    /// syscalls. The interlace argument's low halfword is zero on
    /// half the seeds.
    #[test]
    fn sce_gs_reset_graph_matches() {
        let gs_reset_graph_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1250_0008, 0);
            c.pc = 0x003f_d910;
            let mode = [1i32, 0, 5, 2, -1, 7][seed as usize % 6];
            c.g[A0][0] = (mode as u32) as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            let inter = r.next_u32();
            c.g[A3][0] = if seed / 6 % 2 == 0 {
                inter & 0xffff_0000
            } else {
                inter
            } as u64;
            for i in 0..8 {
                c.bus.write32(GRAPH_PARAM + 4 * i, 0);
            }
            if seed % 2 == 0 {
                c.bus.write32(GRAPH_PARAM + 0x8, 1);
                c.bus.write32(GRAPH_PARAM + 0xc, 0x1234);
            }
        };
        check(
            ee::s15::f_003fd910__generated,
            sce_gs_reset_graph,
            SEEDS,
            gs_reset_graph_call,
        );
        require_paths(
            ee::s15::f_003fd910__generated,
            SEEDS,
            &[
                28, 55, 56, 57, 58, 59, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 77, 78, 79, 80, 81,
            ],
            gs_reset_graph_call,
        );
    }

    /// `void sceGsResetPath()`: no arguments, no branches; the FIFO
    /// payload is non-zero VIF NOPs (op 0, no i-bit) so the emulated VIF1
    /// never runs a microprogram off the test words. FBRST must match
    /// after both runs.
    #[test]
    fn sce_gs_reset_path_matches() {
        let gs_reset_path_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1250_0007, 0);
            c.pc = 0x003f_dab0;
            for i in 0..8 {
                c.bus.write32(RESET_PACKET + 4 * i, 0x0000_1100 | (i + 1));
            }
        };
        check(
            ee::s15::f_003fdab0__generated,
            sce_gs_reset_path,
            SEEDS,
            gs_reset_path_call,
        );
        check_hw(
            ee::s15::f_003fdab0__generated,
            sce_gs_reset_path,
            SEEDS,
            gs_reset_path_call,
            &|c| ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vu0.fbrst,
        );
        require_paths(
            ee::s15::f_003fdab0__generated,
            SEEDS,
            &[26],
            gs_reset_path_call,
        );
    }

    /// `int sceGsSyncPath(mode)`: mode 0 with an idle path returns 0
    /// (no spin, no prints); nonzero modes fold the busy bits of
    /// VIF1/GIF CHCR, the VIF1 FIFO, VPU-STAT and GIF STAT into the
    /// mask. RAM 0 holds a non-zero word. The waits run in
    /// `sce_gs_sync_path_events`, the timeout dumps in
    /// `sce_gs_sync_path_timeout_matches`.
    #[test]
    fn sce_gs_sync_path_matches() {
        let gs_sync_path_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1250_0006, 0);
            c.pc = 0x003f_db18;
            ram0(c);
            if seed % 2 == 1 && seed / 2 % 7 == 0 {
                vif1_fifo_stalled(c, (r.next_u32() as u128) << 64 | 0x0123_4567);
            }
            let m = ssx3_ps2::platform::machine_of(&mut c.bus.plat);
            if seed % 2 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = 1 + (r.next_u32() % 3) as u64;
                if seed / 2 % 2 == 0 {
                    m.hw.ch[1].chcr |= 0x100;
                }
                if seed / 2 % 3 == 0 {
                    m.hw.ch[2].chcr |= 0x100;
                }
                if seed / 2 % 5 == 0 {
                    m.hw.vu0.peer_running = true;
                }
            }
        };
        check(
            ee::s15::f_003fdb18__generated,
            sce_gs_sync_path,
            SEEDS,
            gs_sync_path_call,
        );
        require_paths(
            ee::s15::f_003fdb18__generated,
            SEEDS,
            &[36, 39],
            gs_sync_path_call,
        );
    }

    /// Scratch context buffer for the VU1 get/put-cnd tests (0x220 bytes).
    pub(crate) const CND_BUF: u32 = DATA + 0x400;

    /// Graph param block `sceGsResetGraph` publishes.
    pub(crate) const GRAPH_PARAM: u32 = 0x0045_1550;

    /// Reset packet `sceGsResetPath` streams into the VIF1 FIFO.
    pub(crate) const RESET_PACKET: u32 = 0x0045_1560;

    /// Seeds per test (200-300).
    pub(crate) const SEEDS: u32 = 200;

    /// Scratch output buffer for the VIF0 get-cnd test (0x38 bytes).
    pub(crate) const VIF_CND_BUF: u32 = DATA + 0x700;

    /// The `sceDevVu0Pause` per-state handler table the port dispatches on.
    pub(crate) const VU0_STATE_TABLE: u32 = 0x0049_7850;

    /// The `sceDevVu1Pause` per-state handler table the port dispatches on.
    pub(crate) const VU1_STATE_TABLE: u32 = 0x0049_7870;

    /// VU running flags as the VPU status word (COP2 control 29) reports
    /// them: bit 0 is VU0, bit 8 (the peer flag) is VU1.
    pub(crate) fn vu0_running(c: &mut Cpu, vu0: bool, vu1: bool) {
        let m = ssx3_ps2::platform::machine_of(&mut c.bus.plat);
        m.hw.vu0.running = vu0;
        m.hw.vu0.peer_running = vu1;
    }

    /// VU running flags as the VPU status word (COP2 control 29) reports
    /// them: bit 0 is VU0, bit 8 (the peer flag) is VU1.
    pub(crate) fn vu_running(c: &mut Cpu, vu0: bool, vu1: bool) {
        let m = ssx3_ps2::platform::machine_of(&mut c.bus.plat);
        m.hw.vu0.running = vu0;
        m.hw.vu0.peer_running = vu1;
    }

    /// A word at RAM 0 with every byte non-zero and bit 10 set: what a
    /// register address built without its upper half reads instead.
    pub(crate) fn ram0(c: &mut Cpu) {
        c.bus.write32(0, 0x5a5a_a5a5);
    }

    /// VIF0 FBRST (bit 2 STP stalls the VIF) and the VIF0 FIFO.
    pub(crate) const VIF0_FBRST: u32 = 0x1000_3810;
    pub(crate) const VIF0_FIFO: u32 = 0x1000_4000;

    /// VIF0 in the run state `libdev_Vif0GetState` decodes as `state`:
    /// 0 leaves it idle; otherwise it is stalled with a NOP qword in its
    /// FIFO (packet status busy) and STAT holds a stall bit (1), a wait bit
    /// (2), an error bit (3) or none (4), the bit picked by `pick`.
    pub(crate) fn vif0_in_state(c: &mut Cpu, state: u32, pick: u32) {
        if state == 0 {
            return;
        }
        c.bus.write32(VIF0_FBRST, 4);
        c.bus.write128(VIF0_FIFO, 0);
        let stat = match state {
            1 => [0x100, 0x200, 0x400][pick as usize % 3],
            2 => [0x4, 0x8][pick as usize % 2],
            3 => [0x1000, 0x2000][pick as usize % 2],
            _ => 0,
        };
        ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.vif0.stat = stat;
    }

    /// Runs `generated` and `hand` after the same setup and requires the
    /// hardware state `hw` reads to match afterwards (state the register
    /// and RAM compare of [`check`] does not see).
    fn check_hw<T: PartialEq + std::fmt::Debug>(
        generated: fn(&mut Cpu),
        hand: fn(&mut Cpu),
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
        hw: &dyn Fn(&mut Cpu) -> T,
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let mut a = crate::difftest::fresh();
            let mut b = crate::difftest::fresh();
            setup(&mut a, seed);
            setup(&mut b, seed);
            generated(&mut a);
            hand(&mut b);
            assert_eq!(hw(&mut a), hw(&mut b), "seed {seed}: hardware state");
        }
    }

    /// Gate word for the VU state fakes (in the machine's RAM, so only the
    /// fixtures that set it see them) and the state they return.
    const STATE_GATE: u32 = DATA + 0x6000;
    const STATE_MAGIC: u32 = 0x5653_5441;
    const FORCED_STATE: u32 = DATA + 0x6004;

    /// `libdev_Vu0GetState` with its gate set: returns the forced state.
    fn fake_vu0_get_state(c: &mut Cpu) {
        if c.bus.read32(STATE_GATE) != STATE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_vu0_get_state),
                ee::s15::f_003fe7d8__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(FORCED_STATE) as u64;
        c.cycles += 4;
        ret(c);
    }

    /// `libdev_Vu1GetState` with its gate set: returns the forced state.
    fn fake_vu1_get_state(c: &mut Cpu) {
        if c.bus.read32(STATE_GATE) != STATE_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_vu1_get_state),
                ee::s15::f_003fed08__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(FORCED_STATE) as u64;
        c.cycles += 4;
        ret(c);
    }

    /// Hooks the VU state fakes into their override slots (once per
    /// process) and, with `state` set, opens their gate to return it.
    fn forced_vu_state(c: &mut Cpu, state: Option<u32>) {
        use crate::difftest::fakes::{hook, prev_cell};
        hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FE7D8),
            fake_vu0_get_state,
            prev_cell(fake_vu0_get_state),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FED08),
            fake_vu1_get_state,
            prev_cell(fake_vu1_get_state),
        );
        if let Some(state) = state {
            c.bus.write32(STATE_GATE, STATE_MAGIC);
            c.bus.write32(FORCED_STATE, state);
        }
    }

    driven!(sce_gs_sync_path_gen_driven, ee::s15::f_003fdb18__generated);

    driven!(sce_gs_sync_path_hand_driven, sce_gs_sync_path);

    /// `sceDmaPause` (0x3fe5c8): as an alarm callback it pauses the channel
    /// whose CHCR address is the alarm id (its first argument), clearing STR.
    pub(crate) const DMA_PAUSE: u32 = 0x003f_e5c8;

    /// `sceGsResetPath` (0x3fdab0): as an alarm callback it resets VIF1
    /// (FBRST.RST empties the FIFO) and streams `RESET_PACKET` back in.
    pub(crate) const GS_RESET_PATH: u32 = 0x003f_dab0;

    /// VIF1 and GIF DMA channel control registers: the alarm ids of the
    /// pauses (`sceDmaPause` reads its channel from the id).
    pub(crate) const VIF1_CHCR: u32 = 0x1000_9000;
    pub(crate) const GIF_CHCR: u32 = 0x1000_a000;

    /// VIF1 FBRST (bit 2 STP stalls the VIF) and the VIF1 FIFO.
    pub(crate) const VIF1_FBRST: u32 = 0x1000_3c10;
    pub(crate) const VIF1_FIFO: u32 = 0x1000_5000;

    /// Alarm id of the path reset (any id the pauses do not use).
    pub(crate) const RESET_ALARM: u32 = 0x0000_0101;

    /// Queue a kernel alarm `delay` cycles from now that runs `callback`
    /// with the alarm id `id` as its first argument.
    pub(crate) fn alarm_in(c: &mut Cpu, id: u32, callback: u32, delay: u64) {
        let due = c.cycles + delay;
        let m = ssx3_ps2::platform::machine_of(&mut c.bus.plat);
        m.kernel.alarms.insert(
            id as i32,
            ssx3_ps2::kernel::Alarm {
                callback,
                arg: 0,
                gp: crate::difftest::GAME_GP,
                due,
            },
        );
    }

    /// A transfer in flight on DMA channel `ch` (STR set; the model runs a
    /// transfer the moment CHCR is written, so only a pause clears it).
    pub(crate) fn channel_busy(c: &mut Cpu, ch: usize) {
        ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.ch[ch].chcr |= 0x100;
    }

    /// VIF1 stalled with one qword `q` waiting in its FIFO: VIF1 STAT
    /// reports the FIFO busy until the path is reset.
    pub(crate) fn vif1_fifo_stalled(c: &mut Cpu, q: u128) {
        c.bus.write32(VIF1_FBRST, 4);
        c.bus.write128(VIF1_FIFO, q);
    }

    /// `int sceGsSyncPath(0)`: the wait for the VIF1 and GIF channels and
    /// the VIF1 FIFO to drain, with real events on a running thread. Kernel
    /// alarms pause the busy channels and reset the path a few hundred
    /// cycles in, so each wait ends; the first event lands in the VIF1
    /// channel, GIF channel or VIF1 FIFO wait (or all three in turn). With
    /// events off no alarm can fire, so those seeds sample the same busy
    /// state instead (mode 1).
    #[test]
    fn sce_gs_sync_path_events() {
        check_events(
            sce_gs_sync_path_gen_driven,
            sce_gs_sync_path_hand_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1250_0016, 0);
                c.pc = 0x003f_db18;
                // VIF NOPs (op 0, no i-bit): the reset packet drains at once.
                for i in 0..8 {
                    c.bus.write32(RESET_PACKET + 4 * i, 0x0000_1100 | (i + 1));
                }
                let band = seed / 3 % 4;
                let delay = 60 + 40 * (seed / 12 % 8) as u64;
                let q = (r.next_u32() as u128) << 96 | 0x0123_4567_89ab_cdef;
                let mode = schedule(c, &mut r, seed, 0x003f_db18);
                if band == 0 || band == 3 {
                    channel_busy(c, 1);
                }
                if band == 1 || band == 3 {
                    channel_busy(c, 2);
                }
                if band == 2 || band == 3 {
                    vif1_fifo_stalled(c, q);
                }
                if mode == Events::Off {
                    c.g[A0][0] = 1;
                    return mode;
                }
                c.g[A0][0] = 0;
                match band {
                    0 => alarm_in(c, VIF1_CHCR, DMA_PAUSE, delay),
                    1 => alarm_in(c, GIF_CHCR, DMA_PAUSE, delay),
                    2 => alarm_in(c, RESET_ALARM, GS_RESET_PATH, delay),
                    _ => {
                        alarm_in(c, VIF1_CHCR, DMA_PAUSE, delay);
                        alarm_in(c, GIF_CHCR, DMA_PAUSE, delay + 80);
                        alarm_in(c, RESET_ALARM, GS_RESET_PATH, delay + 160);
                    }
                }
                mode
            },
        );
    }

    /// `int sceGsSyncPath(0)` with one stage busy for good (VIF1 channel,
    /// GIF channel, stalled VIF1 FIFO or VU1 running), events off and no
    /// cycle budget: the wait spins out its 0x1000000 passes, prints the
    /// timeout and the register dumps, and returns -1. RAM 0 holds a
    /// non-zero word. Two seeds per stage (each run is about 117M cycles).
    #[test]
    fn sce_gs_sync_path_timeout_matches() {
        let gs_sync_path_timeout_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1250_0026, 0);
            c.pc = 0x003f_db18;
            c.g[A0][0] = 0;
            c.next_event = u64::MAX;
            ram0(c);
            match seed % 4 {
                0 => channel_busy(c, 1),
                1 => channel_busy(c, 2),
                2 => vif1_fifo_stalled(c, (r.next_u32() as u128) << 96 | 0x0123_4567_89ab_cdef),
                _ => vu_running(c, false, true),
            }
        };
        check(
            ee::s15::f_003fdb18__generated,
            sce_gs_sync_path,
            8,
            gs_sync_path_timeout_call,
        );
        require_paths(
            ee::s15::f_003fdb18__generated,
            8,
            &[117_441_255, 117_441_260, 117_441_270],
            gs_sync_path_timeout_call,
        );
    }
}
