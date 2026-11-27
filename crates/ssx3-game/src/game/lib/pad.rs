//! `src/lib/pad.cpp`: hand-written gamepad (`libpad_*`, `scePad*`) replacements.

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
    (0x003f_f5e8, libpad_init_rpc, "libpad_InitRpc"),
    (0x003f_f688, sce_pad_end, "scePadEnd"),
    (0x003f_f708, sce_pad_port_open, "scePadPortOpen"),
    (0x003f_f8f0, sce_pad_port_close, "scePadPortClose"),
    (0x003f_fc70, sce_pad_set_req_state, "scePadSetReqState"),
    (0x003f_fcd8, sce_pad_get_req_state, "scePadGetReqState"),
    (0x003f_fd60, sce_pad_info_act, "scePadInfoAct"),
    (0x003f_ffa0, sce_pad_info_mode, "scePadInfoMode"),
    (0x0040_00d8, sce_pad_set_main_mode, "scePadSetMainMode"),
    (0x0040_0250, sce_pad_set_act_align, "scePadSetActAlign"),
    (0x0040_0328, sce_pad_get_button_mask, "scePadGetButtonMask"),
    (0x0040_03e0, sce_pad_set_button_info, "scePadSetButtonInfo"),
    (0x0040_0490, sce_pad_info_press_mode, "scePadInfoPressMode"),
    (
        0x0040_04f0,
        sce_pad_enter_press_mode,
        "scePadEnterPressMode",
    ),
    (0x0040_05a0, sce_pad_set_vref_param, "scePadSetVrefParam"),
    (0x0040_0670, sce_pad_get_port_max, "scePadGetPortMax"),
    (0x0040_06d8, sce_pad_get_slot_max, "scePadGetSlotMax"),
    (0x0040_0740, sce_pad_get_mod_version, "scePadGetModVersion"),
    (
        0x0040_07a8,
        sce_pad_set_warning_level,
        "scePadSetWarningLevel",
    ),
];

// ---------------------------------------------------------------------------
// Shared direct-call targets (address, readable alias in the generated shard).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `libpad_GetDmaBuffer` (`0x003f_f9a8`).
    pub(crate) const GET_DMA_BUFFER: Callee = Callee {
        addr: 0x003f_f9a8,
        f: ee::s15::libpad_GetDmaBuffer,
    };

    /// `scePadSetReqState` (`0x003f_fc70`).
    pub(crate) const SET_REQ_STATE: Callee = Callee {
        addr: 0x003f_fc70,
        f: ee::s15::scePadSetReqState,
    };

    /// `scePadGetButtonMask` (`0x0040_0328`).
    pub(crate) const GET_BUTTON_MASK: Callee = Callee {
        addr: 0x0040_0328,
        f: ee::s15::scePadGetButtonMask,
    };

    /// `scePadSetButtonInfo` (`0x0040_03e0`).
    pub(crate) const SET_BUTTON_INFO: Callee = Callee {
        addr: 0x0040_03e0,
        f: ee::s15::scePadSetButtonInfo,
    };

    pub(super) use crate::game::lib::sif::callee::MEMSET;

    pub(super) use crate::game::lib::sif::callee::PRINTF;

    pub(super) use crate::game::lib::sif::callee::CALL_RPC;
}

/// Direct-call target: sceSifCallRpc (0x00426d18).

/// Direct-call target: memset (0x00416210).

/// DMA-buffer fields the pad wrappers read.
pub(crate) const DMA_CFG: u32 = 0x64;

pub(crate) const DMA_NACT: u32 = 0x6a;

pub(crate) const DMA_OK: u32 = 0x72;

pub(crate) const DMA_REQ: u32 = 0x71;

pub(crate) const SLOT_OPEN: u32 = 0x10;

/// libpad's port table: `PAD_PORTS + port * 0x70 + slot * 0x1c`.
/// +0 holds the DMA buffer pointer, +0x10 the open flag.
pub(crate) const PAD_PORTS: u32 = 0x0052_6250;

pub(crate) const PORT_STRIDE: u32 = 0x70;

/// The pad RPC request buffer (function word at +0, args after).
pub(crate) const PAD_RPC_BUF: u32 = 0x0052_6440;

pub(crate) const SLOT_STRIDE: u32 = 0x1c;

/// Direct-call target: scePrintf (0x00425c60).

/// `int libpad_InitRpc(int a)` (0x3ff5e8): clear the four pad DMA slot
/// headers and issue the bind RPC. Returns the bind reply word, or 0 when
/// the RPC fails.
pub fn libpad_init_rpc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x520000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6250));
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[A0][0] = 0x520000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = 0x3;
        'l_3ff608: loop {
            c.cycles += 9;
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x74), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x6c), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x70), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x4), 0u64);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
            c.sw(c.g[V0][0] as u32, 0u64);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003f_f608);
                }
                continue 'l_3ff608;
            } else {
                'b_3ff674: {
                    c.cycles += 17;
                    c.g[V0][0] = 0x10;
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6440));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6440), c.g[V0][0]);
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = 0x520000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6200));
                    c.sw(frame.at(c, 0), 0u64);
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = 0x80;
                    c.g[T1][0] = c.g[S0][0];
                    c.g[T2][0] = 0x80;
                    c.g[RA][0] = sx(0x003f_f668);
                    c.g[T3][0] = 0u64;
                    callee::CALL_RPC.call(c, 0x003f_f668)?;
                    if (c.g[V0][0] as i64) >= 0 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        break 'b_3ff674;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0u64;
                        break 'b_3ff674;
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = c.ld(frame.at(c, 0x20));
                c.g[S0][0] = c.ld(frame.at(c, 0x10));
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `int scePadEnd(void)` (0x3ff688): issue the pad-end RPC and, when its
/// reply word reads 1, clear the initialised flag at 0x4515c8. Returns the
/// reply word, or 0 when the RPC fails.
pub fn sce_pad_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x520000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V1][0] = 0xf;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6440));
        c.g[A0][0] = 0x520000;
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6200));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6440), c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.sw(frame.at(c, 0), 0u64);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x80;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[RA][0] = sx(0x003f_f6d0);
        c.g[T3][0] = 0u64;
        callee::CALL_RPC.call(c, 0x003f_f6d0)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let done = c.g[A3][0] != c.g[V0][0];
            c.g[V0][0] = c.g[A3][0];
            if !done {
                c.cycles += 3;
                c.g[V0][0] = 0x450000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x15c8), 0u64);
                c.g[V0][0] = c.g[A3][0];
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadEnterPressMode(port, slot)` (0x4004f0): on an open port,
/// `scePadSetButtonInfo(port, slot, 0xfff)`; otherwise 0.
pub fn sce_pad_enter_press_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, A2, V1);
        c.mult(0, A0, A1, A0);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(SLOT_OPEN)).lw(c);
        c.g[A0][0] = c.g[A2][0];
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            c.g[A2][0] = 0xfff;
            callee::SET_BUTTON_INFO.call(c, 0x0040_0538)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int scePadGetButtonMask(port, slot)` (0x400328): 0 unless the port is
/// open, the DMA buffer is valid (`+0x72 == 1`) and in a button mode
/// (`+0x64 >= 2`, `+0x66 >= 2`); then the sign-extended 32-bit mask
/// assembled from bytes `+0x79..+0x7c`.
pub fn sce_pad_get_button_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, A2, V1);
        c.mult(0, A0, A1, A0);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(SLOT_OPEN)).lw(c);
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.g[A2][0];
            callee::GET_DMA_BUFFER.call(c, 0x0040_0368)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[V1][0] = 1;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(DMA_OK)).lbu(c);
            if c.g[V0][0] != c.g[V1][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
            } else {
                c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(DMA_CFG)).lbu(c);
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x66)).lbu(c);
                    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = 0;
                    } else {
                        c.cycles += 12;
                        c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x7a)).lbu(c);
                        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x7c)).lbu(c);
                        c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x7b)).lbu(c);
                        c.g[A1][0] <<= 8;
                        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x79)).lbu(c);
                        c.g[A0][0] <<= 24;
                        c.g[V1][0] <<= 16;
                        c.g[V0][0] = c.g[V0][0].wrapping_add(c.g[A0][0]);
                        c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[A1][0]);
                        c.g[V0][0] = c.g[V0][0].wrapping_add(c.g[V1][0]);
                        c.g[V0][0] <<= 32;
                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    }
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int scePadGetModVersion()` (0x400740): RPC function 0x12.
/// Negative RPC results give 0, otherwise the reply word at `+0xc`.
pub fn sce_pad_get_mod_version(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V1][0] = 0x12;
        c.g[S0][0] = sx(PAD_RPC_BUF);
        c.g[A0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[A0][0] = sx(0x52_6200);
        Addr(PAD_RPC_BUF).sw(c, c.g[V1][0]);
        c.g[A1][0] = 1;
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x80;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_0788)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadGetPortMax()` (0x400670): RPC function 0xc.
/// Negative RPC results give 0, otherwise the reply word at `+0xc`.
pub fn sce_pad_get_port_max(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V1][0] = 0xc;
        c.g[S0][0] = sx(PAD_RPC_BUF);
        c.g[A0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[A0][0] = sx(0x52_6200);
        Addr(PAD_RPC_BUF).sw(c, c.g[V1][0]);
        c.g[A1][0] = 1;
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x80;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_06b8)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadGetReqState(port, slot)` (0x3ffcd8): 0 on a closed port,
/// else the DMA buffer's request-state byte (`+0x71`).
pub fn sce_pad_get_req_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, A2, V1);
        c.mult(0, A0, A1, A0);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(SLOT_OPEN)).lw(c);
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[A2][0];
            callee::GET_DMA_BUFFER.call(c, 0x003f_fd18)?;
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(DMA_REQ)).lbu(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int scePadGetSlotMax(port)` (0x4006d8): RPC function 0xd.
/// Negative RPC results give 0, otherwise the reply word at `+0xc`.
/// Resume entries go to the generated body (`__generated`), which handles
/// every entry point.
pub fn sce_pad_get_slot_max(c: &mut Cpu) {
    if c.pc != 0x0040_06d8 {
        return ee::s15::f_004006d8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[A2][0] = 0xd;
        c.g[S0][0] = sx(PAD_RPC_BUF);
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, c.g[A0][0]);
        c.g[V1][0] = 0x52_0000;
        Addr(PAD_RPC_BUF).sw(c, c.g[A2][0]);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx(0x52_6200);
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x80;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_0724)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadInfoAct(port, slot, id, idx)` (0x3ffd60): actuator queries.
/// 0 unless the port is open, the buffer is valid (`+0x72 == 1`) and in a
/// multi-actuator mode (`+0x64 >= 2`), and for an `id` at or past the
/// actuator count (`+0x6a`, signed compare); an id of -1 reports the count;
/// otherwise `idx` 1..4 gives byte `0x30..0x33` of actuator `id`'s word
/// (`buf[id * 4 + 0x2f + idx]`) and any other `idx` gives 0.
pub fn sce_pad_info_act(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[T0][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, T0, V1);
        c.mult(0, A0, A1, A0);
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(SLOT_OPEN)).lw(c);
        c.g[S0][0] = c.g[A3][0];
        'b_3ffe70: {
            'b_3ffe6c: {
                'b_3ffe68: {
                    if c.g[V1][0] == 0 {
                        break 'b_3ffe68;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.g[T0][0];
                    callee::GET_DMA_BUFFER.call(c, 0x003f_fdac)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(DMA_OK)).lbu(c);
                    c.g[V0][0] = 1;
                    let invalid = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0;
                    if invalid {
                        break 'b_3ffe6c;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(DMA_CFG)).lbu(c);
                    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                    let single = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if single {
                        break 'b_3ffe6c;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(DMA_NACT)).lbu(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let past_count = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if past_count {
                        break 'b_3ffe68;
                    }
                    'b_3ffdf0: {
                        c.cycles += 2;
                        let actuator = c.g[S1][0] != c.g[V0][0];
                        c.g[V0][0] = 2;
                        if actuator {
                            break 'b_3ffdf0;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = c.g[V1][0];
                        break 'b_3ffe6c;
                    }
                    // By `idx` (v0 = 2, a1 = 1 here): 1..4 pick a byte of the
                    // actuator's word, anything else returns 0.
                    'b_3ffe58: {
                        'b_3ffe48: {
                            'b_3ffe38: {
                                c.cycles += 2;
                                let two = c.g[S0][0] == c.g[V0][0];
                                c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
                                if two {
                                    break 'b_3ffe38;
                                }
                                'b_3ffe28: {
                                    'b_3ffe10: {
                                        c.cycles += 2;
                                        let at_least_three = c.g[V0][0] == 0;
                                        c.g[V0][0] = 3;
                                        if at_least_three {
                                            break 'b_3ffe10;
                                        }
                                        c.cycles += 2;
                                        let one = c.g[S0][0] == c.g[A1][0];
                                        c.g[V0][0] = 0;
                                        if one {
                                            break 'b_3ffe28;
                                        }
                                        c.cycles += 2;
                                        frame.restore_ra(c, 0x20);
                                        break 'b_3ffe70;
                                    }
                                    c.cycles += 2;
                                    let three = c.g[S0][0] == c.g[V0][0];
                                    c.g[V0][0] = 4;
                                    if three {
                                        break 'b_3ffe48;
                                    }
                                    c.cycles += 2;
                                    let four = c.g[S0][0] == c.g[V0][0];
                                    c.g[V0][0] = 0;
                                    if four {
                                        break 'b_3ffe58;
                                    }
                                    c.cycles += 2;
                                    frame.restore_ra(c, 0x20);
                                    break 'b_3ffe70;
                                }
                                actuator_byte(c, 0x30);
                                break 'b_3ffe6c;
                            }
                            actuator_byte(c, 0x31);
                            break 'b_3ffe6c;
                        }
                        actuator_byte(c, 0x32);
                        break 'b_3ffe6c;
                    }
                    actuator_byte(c, 0x33);
                    break 'b_3ffe6c;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'b_3ffe6c;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0x20);
            break 'b_3ffe70;
        }
        c.cycles += 4;
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `scePadInfoAct`'s answer for `idx` 1..4: byte `off` of actuator `id`'s
/// word in the DMA buffer (`a0`), `id` in s1.
fn actuator_byte(c: &mut Cpu, off: u32) {
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(off)).lbu(c);
}

/// `int scePadInfoMode(port, slot, term, offs)` (0x3fffa0): mode-list
/// queries. 0 unless the port is open and the buffer is valid
/// (`+0x72 == 1`, `+0x71 != 2`); then by `term`: 1 gives the mode id
/// (`+0x65 >> 4`, 0 when `0xf3`); 2 gives the mode-table entry
/// (`lhu(buf + (buf[0x69] << 1) + 0x50)` unless the mode changed);
/// 3 gives the mode offset (`+0x69`); 4 gives the table entry at
/// `(offs << 1)` (`+0x68` when `offs == -1`); other terms give 0.
/// Resume entries go to the generated body (`__generated`), which handles
/// every entry point.
pub fn sce_pad_info_mode(c: &mut Cpu) {
    if c.pc != 0x003f_ffa0 {
        return ee::s15::f_003fffa0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[T0][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, T0, V1);
        c.mult(0, A0, A1, A0);
        c.sd(frame.at(c, 0x10), c.g[S1][0]);
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx(PAD_PORTS.wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(SLOT_OPEN)).lw(c);
        c.g[S1][0] = c.g[A3][0];
        'b_4000c8: {
            'b_4000c4: {
                'b_4000c0: {
                    if c.g[V1][0] == 0 {
                        break 'b_4000c0;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.g[T0][0];
                    callee::GET_DMA_BUFFER.call(c, 0x003f_ffec)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(DMA_OK)).lbu(c);
                    c.g[V0][0] = 1;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0;
                        break 'b_4000c4;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(DMA_REQ)).lbu(c);
                    c.g[A1][0] = 2;
                    if c.g[V0][0] == c.g[A1][0] {
                        c.g[V0][0] = 0;
                        break 'b_4000c4;
                    }
                    'b_4000b4: {
                        'b_40008c: {
                            'b_400078: {
                                'b_400060: {
                                    c.cycles += 2;
                                    if c.g[S0][0] == c.g[A1][0] {
                                        break 'b_400060;
                                    }
                                    'b_400048: {
                                        'b_400030: {
                                            c.cycles += 2;
                                            c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
                                            if c.g[V0][0] == 0 {
                                                c.g[V0][0] = 3;
                                                break 'b_400030;
                                            }
                                            c.cycles += 2;
                                            let t = c.g[S0][0] == c.g[V1][0];
                                            c.g[V0][0] = 0;
                                            if t {
                                                break 'b_400048;
                                            }
                                            c.cycles += 2;
                                            c.g[RA][0] = c.ld(frame.at(c, 0x20));
                                            break 'b_4000c8;
                                        }
                                        c.cycles += 2;
                                        // V0 is 3 here (term >= 3 arm above): test
                                        // term == 3, then V0 becomes 4 either way.
                                        let t = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] = 4;
                                        if t {
                                            break 'b_400078;
                                        }
                                        c.cycles += 2;
                                        // V0 is 4 here: test term == 4, then V0
                                        // becomes 0 either way.
                                        let t = c.g[S0][0] == c.g[V0][0];
                                        c.g[V0][0] = 0;
                                        if t {
                                            break 'b_40008c;
                                        }
                                        c.cycles += 2;
                                        c.g[RA][0] = c.ld(frame.at(c, 0x20));
                                        break 'b_4000c8;
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] =
                                        Addr((c.g[A0][0] as u32).wrapping_add(0x65)).lbu(c);
                                    c.g[V0][0] = 0xf3;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 4);
                                    if c.g[V1][0] == 0xf3 {
                                        break 'b_4000c0;
                                    }
                                    c.cycles += 2;
                                    c.g[RA][0] = c.ld(frame.at(c, 0x20));
                                    break 'b_4000c8;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(DMA_CFG)).lbu(c);
                                if c.g[V0][0] == c.g[V1][0] {
                                    c.g[V0][0] = 0;
                                    break 'b_4000c4;
                                }
                                c.cycles += 3;
                                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x69)).lbu(c);
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                                break 'b_4000b4;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(DMA_CFG)).lbu(c);
                            if c.g[V0][0] == c.g[V1][0] {
                                c.g[V0][0] = 0;
                                break 'b_4000c4;
                            }
                            c.cycles += 2;
                            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x69)).lbu(c);
                            break 'b_4000c4;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(DMA_CFG)).lbu(c);
                        let t = c.g[V0][0] == c.g[V1][0];
                        // Delay-slot write: V0 becomes -1 on both arms (the
                        // taken arm still returns 0 through the shared tail,
                        // the fall-through compares offs against -1).
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        if t {
                            break 'b_4000c0;
                        }
                        'b_4000a8: {
                            c.cycles += 2;
                            let t = c.g[S1][0] != c.g[V0][0];
                            // Delay-slot load: runs on both arms, so the
                            // offs == -1 early return leaves [0x68] in V0.
                            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x68)).lbu(c);
                            if t {
                                break 'b_4000a8;
                            }
                            c.cycles += 2;
                            c.g[RA][0] = c.ld(frame.at(c, 0x20));
                            break 'b_4000c8;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] == 0;
                        // Delay-slot store: runs whether the bounds check
                        // passes or not (generated body writes V1 before
                        // the branch).
                        c.g[V1][0] = sx((c.g[S1][0] as u32) << 1);
                        if t {
                            break 'b_4000c0;
                        }
                        break 'b_4000b4;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x50)).lhu(c);
                    break 'b_4000c4;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'b_4000c4;
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld(frame.at(c, 0x20));
            break 'b_4000c8;
        }
        c.cycles += 4;
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadInfoPressMode(port, slot)` (0x400490): 0 on a closed port,
/// else 1 when the button mask is exactly the pressure/buttons word
/// (`mask ^ 0x3ffff == 0`), else 0.
pub fn sce_pad_info_press_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, A2, V1);
        c.mult(0, A0, A1, A0);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0), c.g[RA][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(SLOT_OPEN)).lw(c);
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[A2][0];
            callee::GET_BUTTON_MASK.call(c, 0x0040_04d0)?;
            c.g[V1][0] = 0x3_0000;
            c.g[V1][0] |= 0xffff;
            c.g[V0][0] ^= c.g[V1][0];
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int scePadPortClose(port, slot)` (0x3ff8f0): 0 on a closed port;
/// otherwise the RPC (function 0xe); a negative RPC result gives 0, and
/// on success the port's open word is cleared and the reply word at
/// `+0xc` returned.
pub fn sce_pad_port_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        c.g[A3][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, A3, V1);
        c.mult(0, A0, A1, A0);
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x52_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6250));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x30), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 21;
            c.g[V0][0] = 0x52_0000;
            c.g[V1][0] = 0xe;
            c.g[S0][0] = sx(PAD_RPC_BUF);
            Addr(PAD_RPC_BUF).sw(c, c.g[V1][0]);
            c.g[A2][0] = 1;
            Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, c.g[A3][0]);
            Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, c.g[A1][0]);
            c.g[A0][0] = 0x52_0000;
            Addr((c.g[S0][0] as u32).wrapping_add(0x10)).sw(c, c.g[A2][0]);
            c.g[A0][0] = sx(0x52_6200);
            c.g[A1][0] = 1;
            c.g[A2][0] = 0;
            Addr(frame.at(c, 0)).sw(c, 0);
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 0x80;
            c.g[T1][0] = c.g[S0][0];
            c.g[T2][0] = 0x80;
            c.g[T3][0] = 0;
            callee::CALL_RPC.call(c, 0x003f_f980)?;
            if (c.g[V0][0] as i64) >= 0 {
                Addr(c.g[S1][0] as u32).sw(c, 0);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 5;
                c.g[RA][0] = c.ld(frame.at(c, 0x30));
                c.g[S1][0] = c.ld(frame.at(c, 0x20));
                c.g[S0][0] = c.ld(frame.at(c, 0x10));
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadPortOpen(int port, int slot, void *buf)` (0x3ff708): open pad
/// `port`/`slot` with the DMA buffer `buf`. Returns 0 when `buf` is not
/// 64-byte aligned or the slot's table word
/// (`0x526250 + port * 0x70 + slot * 0x1c`, `+0x10`) already reads 1,
/// printing first when the `0x4515cc` debug word is set. Otherwise
/// initialises both 0x80-byte halves of `buf`, sends the open request (RPC
/// function 1, waiting, block at `0x526440`) and, when the RPC succeeds,
/// marks the slot open, records `buf` and the reply geometry in the table and
/// returns reply word 3 (`+0xc`); 0 when the RPC fails.
pub fn sce_pad_port_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        'b_3ff8bc: {
            'b_3ff85c: {
                'b_3ff854: {
                    'b_3ff76c: {
                        c.sd(frame.at(c, 0x50), c.g[S4][0]);
                        c.sd(frame.at(c, 0x40), c.g[S3][0]);
                        c.g[S4][0] = c.g[A2][0];
                        c.sd(frame.at(c, 0x30), c.g[S2][0]);
                        c.g[S3][0] = c.g[A0][0];
                        c.sd(frame.at(c, 0xa0), c.g[RA][0]);
                        c.g[S2][0] = c.g[A1][0];
                        c.sd(frame.at(c, 0x90), c.g[FP][0]);
                        c.g[V0][0] = c.g[S4][0] & 0x3f;
                        c.sd(frame.at(c, 0x80), c.g[S7][0]);
                        c.sd(frame.at(c, 0x70), c.g[S6][0]);
                        c.sd(frame.at(c, 0x60), c.g[S5][0]);
                        c.sd(frame.at(c, 0x20), c.g[S1][0]);
                        let aligned = c.g[V0][0] == 0u64;
                        c.sd(frame.at(c, 0x10), c.g[S0][0]);
                        if aligned {
                            break 'b_3ff76c;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x450000;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15cc));
                            let unaligned_debug = c.g[V1][0] == 0u64;
                            c.g[A0][0] = 0x490000;
                            if unaligned_debug {
                                break 'b_3ff854;
                            } else {
                                c.cycles += 5;
                                c.g[A1][0] = c.g[S4][0];
                                c.g[RA][0] = sx(0x003f_f764);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7920));
                                callee::PRINTF.call(c, 0x003f_f764)?;
                                c.g[V0][0] = 0u64;
                                break 'b_3ff8bc;
                            }
                        }
                    }
                    'b_3ff7c4: {
                        c.cycles += 12;
                        c.g[A0][0] = 0x1c;
                        c.g[V1][0] = 0x70;
                        c.mult(1, 3, 19, 3);
                        c.mult(0, 4, 18, 4);
                        c.g[FP][0] = 0x520000;
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6250));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                        let not_open = c.g[V1][0] != c.g[A1][0];
                        c.g[S0][0] = c.g[S4][0];
                        if not_open {
                            break 'b_3ff7c4;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x450000;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15cc));
                            let hit_debug = c.g[V1][0] == 0u64;
                            c.g[A0][0] = 0x490000;
                            if hit_debug {
                                break 'b_3ff854;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = c.g[S3][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7950));
                                c.g[RA][0] = sx(0x003f_f7bc);
                                c.g[A2][0] = c.g[S2][0];
                                callee::PRINTF.call(c, 0x003f_f7bc)?;
                                c.g[V0][0] = 0u64;
                                break 'b_3ff8bc;
                            }
                        }
                    }
                    // Reached when the slot word is not 1 (the `not_open`
                    // break above): the buffer init + open request, with the
                    // loop's structured event check.
                    c.cycles += 5;
                    c.g[S7][0] = 0x520000;
                    c.g[S6][0] = 0x5;
                    c.g[S5][0] = 0x2;
                    c.g[S1][0] = 0x1;
                    'l_3ff7d8: loop {
                        c.cycles += 12;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
                        c.g[A0][0] = c.g[S0][0];
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x70), c.g[S6][0]);
                        c.g[A1][0] = 0xff;
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x71), c.g[S5][0]);
                        c.g[A2][0] = 0x20;
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x67), 0u64);
                        c.g[RA][0] = sx(0x003f_f7fc);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                        callee::MEMSET.call(c, 0x003f_f7fc)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), 0u64);
                        let more = (c.g[S1][0] as i64) >= 0;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x80));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_f7d8);
                            }
                            continue 'l_3ff7d8;
                        } else {
                            c.cycles += 19;
                            c.g[S1][0] = 0x1;
                            c.g[S0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x6440));
                            c.sw((c.g[S7][0] as u32).wrapping_add(0x6440), c.g[S1][0]);
                            c.g[A0][0] = 0x520000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6200));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                            c.g[A1][0] = 0x1;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S4][0]);
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = c.g[S0][0];
                            c.sw(c.g[SP][0] as u32, 0u64);
                            c.g[T0][0] = 0x80;
                            c.g[T1][0] = c.g[S0][0];
                            c.g[T2][0] = 0x80;
                            c.g[RA][0] = sx(0x003f_f84c);
                            c.g[T3][0] = 0u64;
                            callee::CALL_RPC.call(c, 0x003f_f84c)?;
                            let rpc_ok = (c.g[V0][0] as i64) >= 0;
                            c.g[V1][0] = 0x1c;
                            if rpc_ok {
                                break 'b_3ff85c;
                            } else {
                                break 'b_3ff854;
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = 0u64;
                break 'b_3ff8bc;
            }
            c.cycles += 24;
            c.g[A3][0] = 0x70;
            c.mult(1, 7, 19, 7);
            c.mult(0, 3, 18, 3);
            c.g[V0][0] = 0x520000;
            c.g[A1][0] = sx((c.g[S2][0] as u32) << 5);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6340));
            c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x6250));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = sx((c.g[S3][0] as u32) << 7);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
            c.g[A1][0] = c.g[V0][0];
            c.g[A3][0] = c.g[V0][0];
            c.sw(c.g[A2][0] as u32, 0u64);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
            c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[T0][0]);
            c.g[A0][0] = c.g[A1][0];
            c.sw(c.g[V0][0] as u32, c.g[S4][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            break 'b_3ff8bc;
        }
        c.cycles += 12;
        c.g[RA][0] = c.ld(frame.at(c, 0xa0));
        c.g[FP][0] = c.ld(frame.at(c, 0x90));
        c.g[S7][0] = c.ld(frame.at(c, 0x80));
        c.g[S6][0] = c.ld(frame.at(c, 0x70));
        c.g[S5][0] = c.ld(frame.at(c, 0x60));
        c.g[S4][0] = c.ld(frame.at(c, 0x50));
        c.g[S3][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadSetActAlign(port, slot, align)` (0x400250): copies the
/// 6-byte `align` vector into the RPC request (function 8), then the RPC.
/// Negative RPC results give 0; a non-1 reply is returned; a 1 reply goes
/// through `scePadSetReqState(port, slot, 2)` first.
pub fn sce_pad_set_act_align(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x52_0000;
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[V0][0] = 8;
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = sx(PAD_RPC_BUF);
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        Addr(PAD_RPC_BUF).sw(c, c.g[V0][0]);
        c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, c.g[S2][0]);
        c.g[A1][0] = 0;
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, c.g[S1][0]);
        'l_400290: loop {
            c.cycles += 8;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A1][0] as i64) < 6) as u64;
            Addr(c.g[A0][0] as u32).sb(c, c.g[V1][0]);
            if c.g[V0][0] == 0 {
                break 'l_400290;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0040_0290);
            }
        }
        c.cycles += 13;
        c.g[A0][0] = 0x52_0000;
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A0][0] = sx(0x52_6200);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x80;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_02dc)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 6;
            c.g[RA][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 2;
            callee::SET_REQ_STATE.call(c, 0x0040_0308)?;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lw(c);
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadSetButtonInfo(port, slot, info)` (0x4003e0): RPC function
/// 0xa. Negative RPC results give 0; a non-1 reply is returned; a 1 reply
/// goes through `scePadSetReqState(port, slot, 2)` first.
pub fn sce_pad_set_button_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V1][0] = 0xa;
        c.g[S0][0] = sx(PAD_RPC_BUF);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
        c.g[A0][0] = 0x52_0000;
        Addr(PAD_RPC_BUF).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(0x52_6200);
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, c.g[S2][0]);
        c.g[A1][0] = 1;
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, c.g[S1][0]);
        c.g[A2][0] = 0;
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x80;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_0444)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 6;
            c.g[RA][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 2;
            callee::SET_REQ_STATE.call(c, 0x0040_0470)?;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadSetMainMode(port, slot, mode, lock)` (0x4000d8):
/// RPC function 6. Negative RPC results give 0; a non-1 reply is returned;
/// a 1 reply goes through `scePadSetReqState(port, slot, 2)` first.
pub fn sce_pad_set_main_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x52_0000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[V0][0] = 6;
        c.g[S0][0] = sx(PAD_RPC_BUF);
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
        c.g[A0][0] = 0x52_0000;
        Addr((c.g[S0][0] as u32).wrapping_add(0x10)).sw(c, c.g[A3][0]);
        c.g[A0][0] = sx(0x52_6200);
        Addr(PAD_RPC_BUF).sw(c, c.g[V0][0]);
        c.g[A1][0] = 1;
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, c.g[S2][0]);
        c.g[A2][0] = 0;
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, c.g[S1][0]);
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x80;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_0140)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 6;
            c.g[RA][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 2;
            callee::SET_REQ_STATE.call(c, 0x0040_016c)?;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lw(c);
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadSetReqState(port, slot, state)` (0x3ffc70): 0 on a closed
/// port, else stores `state` in the DMA buffer's request-state byte
/// (`+0x71`) and returns 1.
pub fn sce_pad_set_req_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[A3][0] = c.g[A0][0];
        c.g[V1][0] = PORT_STRIDE as u64;
        c.g[A0][0] = SLOT_STRIDE as u64;
        c.mult(1, V1, A3, V1);
        c.mult(0, A0, A1, A0);
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0), c.g[S0][0]);
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        c.g[V0][0] = sx(PAD_PORTS);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(SLOT_OPEN)).lw(c);
        c.g[S0][0] = c.g[A2][0];
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[A3][0];
            callee::GET_DMA_BUFFER.call(c, 0x003f_fcbc)?;
            Addr((c.g[V0][0] as u32).wrapping_add(DMA_REQ)).sb(c, c.g[S0][0]);
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadSetVrefParam(int port, int slot, const unsigned char *param)`
/// (0x4005a0): send request 0xb (RPC function 1) with `port`, `slot` and the
/// 12 bytes of `param` (copied with unaligned loads/stores to `+0xc`).
/// Negative RPC results give 0, otherwise the reply word at `+0x1c`; when
/// that word is 1 it first calls `scePadSetReqState(port, slot, 2)` and
/// returns the reply word re-read.
pub fn sce_pad_set_vref_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x30), c.g[S2][0]);
        c.g[V1][0] = 0xb;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[S2][0] = c.g[A0][0];
        c.g[S0][0] = sx(PAD_RPC_BUF);
        c.sd(frame.at(c, 0x20), c.g[S1][0]);
        c.sd(frame.at(c, 0x40), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, c.g[S2][0]);
        c.g[A0][0] = 0x52_0000;
        Addr(PAD_RPC_BUF).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(0x52_6200);
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, c.g[S1][0]);
        c.g[T0][0] = 0x80;
        // Unaligned 12-byte copy: param[a2..a2+12] to s0+0xc.
        c.ldl(V0, (c.g[A2][0] as u32).wrapping_add(7));
        c.ldr(V0, c.g[A2][0] as u32);
        c.lwl(V1, (c.g[A2][0] as u32).wrapping_add(0xb));
        c.lwr(V1, (c.g[A2][0] as u32).wrapping_add(8));
        c.sdl(V0, (c.g[S0][0] as u32).wrapping_add(0x13));
        c.sdr(V0, (c.g[S0][0] as u32).wrapping_add(0xc));
        c.swl(V1, (c.g[S0][0] as u32).wrapping_add(0x17));
        c.swr(V1, (c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[T1][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_0620)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 6;
            c.g[RA][0] = c.ld(frame.at(c, 0x40));
            c.g[S2][0] = c.ld(frame.at(c, 0x30));
            c.g[S1][0] = c.ld(frame.at(c, 0x20));
            c.g[S0][0] = c.ld(frame.at(c, 0x10));
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 2;
            callee::SET_REQ_STATE.call(c, 0x0040_064c)?;
            c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[V0][0] = c.g[V1][0];
        } else {
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(frame.at(c, 0x40));
        c.g[S2][0] = c.ld(frame.at(c, 0x30));
        c.g[S1][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

/// `int scePadSetWarningLevel(level)` (0x4007a8): RPC function 0x14.
/// Negative RPC results give 0, otherwise the reply word at `+0x8`.
pub fn sce_pad_set_warning_level(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x52_0000;
        c.sd(frame.at(c, 0x10), c.g[S0][0]);
        c.g[A2][0] = 0x14;
        c.g[S0][0] = sx(PAD_RPC_BUF);
        c.sd(frame.at(c, 0x20), c.g[RA][0]);
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, c.g[A0][0]);
        c.g[V1][0] = 0x52_0000;
        Addr(PAD_RPC_BUF).sw(c, c.g[A2][0]);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx(0x52_6200);
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = 0x80;
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = 0x80;
        c.g[T3][0] = 0;
        callee::CALL_RPC.call(c, 0x0040_07f4)?;
        if (c.g[V0][0] as i64) >= 0 {
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(frame.at(c, 0x20));
        c.g[S0][0] = c.ld(frame.at(c, 0x10));
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ee;
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::require_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::lib::cdvd::tests::rpc_answered;
    use crate::game::lib::sif::tests::SIF_RPC_POOL;
    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    /// `int libpad_InitRpc(a)`: the header-clear loop with real events on
    /// a running thread (structured back edge).
    #[test]
    fn libpad_init_rpc_events() {
        check_events(
            ee::s15::f_003ff5e8__generated,
            libpad_init_rpc,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1248_001c, 0);
                c.pc = 0x003f_f5e8;
                c.g[A0][0] = r.next_u32() as u64;
                schedule(c, &mut r, seed, 0x003f_f5e8)
            },
        );
    }

    /// `int libpad_InitRpc(a)`: the header-clear loop plus the bind RPC,
    /// which fails on fixture data (empty SIF packet pool) and returns 0.
    /// The reply-word path is not taken here.
    #[test]
    fn libpad_init_rpc_matches() {
        let pad_init_rpc_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x1248_0011, 0);
            c.pc = 0x003f_f5e8;
            c.g[A0][0] = r.next_u32() as u64;
        };
        check(
            ee::s15::f_003ff5e8__generated,
            libpad_init_rpc,
            SEEDS,
            pad_init_rpc_call,
        );
        require_paths(
            ee::s15::f_003ff5e8__generated,
            SEEDS,
            &[135],
            pad_init_rpc_call,
        );
    }

    /// `int scePadEnd(void)`: the end RPC fails on fixture data (empty SIF
    /// packet pool) and returns 0. The reply-word paths run in
    /// `sce_pad_end_answered_matches`.
    #[test]
    fn sce_pad_end_matches() {
        let pad_end_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_0012, 0);
            c.pc = 0x003f_f688;
        };
        check(
            ee::s15::f_003ff688__generated,
            sce_pad_end,
            SEEDS,
            pad_end_call,
        );
        require_paths(ee::s15::f_003ff688__generated, SEEDS, &[95], pad_end_call);
    }

    /// `int scePadEnd(void)` with the end RPC answered: a reply word of 1
    /// clears the initialised flag, 0 and 2 are returned as they are.
    #[test]
    fn sce_pad_end_answered_matches() {
        let pad_end_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            pad_answered(c, seed, 0x003f_f688);
            c.bus.write32(0x0045_15c8, 1);
            c.bus
                .write32(PAD_RPC_BUF + 0xc, [1, 0, 2][(seed % 3) as usize]);
        };
        check(
            ee::s15::f_003ff688__generated,
            sce_pad_end,
            12,
            pad_end_call,
        );
        require_paths(
            ee::s15::f_003ff688__generated,
            12,
            &[316, 319],
            pad_end_call,
        );
    }

    /// `scePadEnterPressMode`: closed and open ports (the open path runs the
    /// real `scePadSetButtonInfo`, whose RPC fails).
    #[test]
    fn sce_pad_enter_press_mode_matches() {
        let pad_enter_press_mode_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_04f0, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x0040_04f0;
            let (port, slot) = (r.next_u32() % 2, r.next_u32() % 2);
            pad_dma_buffer(c, &mut r, port, slot, (seed % 3 != 0) as u32);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
        };
        check_lohi(
            ee::s15::f_004004f0__generated,
            sce_pad_enter_press_mode,
            300,
            pad_enter_press_mode_call,
        );
        require_paths(
            ee::s15::f_004004f0__generated,
            300,
            &[19, 123],
            pad_enter_press_mode_call,
        );
    }

    /// `scePadGetButtonMask`: closed ports, invalid buffers (ok/config
    /// bytes), and valid buffers with random masks.
    #[test]
    fn sce_pad_get_button_mask_matches() {
        let pad_get_button_mask_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_0328, 0);
            elf_image(c);
            c.pc = 0x0040_0328;
            let (port, slot) = (r.next_u32() % 2, r.next_u32() % 2);
            let buf = pad_dma_buffer(c, &mut r, port, slot, (seed % 5 != 0) as u32);
            for half in [buf, buf + 0x80] {
                c.bus
                    .write8(half + DMA_OK, if seed % 5 == 1 { 0 } else { 1 });
                c.bus.write8(
                    half + DMA_CFG,
                    if seed % 5 == 2 {
                        0
                    } else {
                        2 + (r.next_u32() % 4) as u8
                    },
                );
                c.bus.write8(
                    half + 0x66,
                    if seed % 5 == 3 {
                        1
                    } else {
                        2 + (r.next_u32() % 4) as u8
                    },
                );
            }
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
        };
        check_lohi(
            ee::s15::f_00400328__generated,
            sce_pad_get_button_mask,
            300,
            pad_get_button_mask_call,
        );
        require_paths(
            ee::s15::f_00400328__generated,
            300,
            &[17, 1680, 1681, 1685, 1697],
            pad_get_button_mask_call,
        );
    }

    /// `scePadGetModVersion`: the RPC always fails.
    #[test]
    fn sce_pad_get_mod_version_matches() {
        let pad_get_mod_version_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x0040_0740, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x0040_0740;
        };
        check_lohi(
            ee::s15::f_00400740__generated,
            sce_pad_get_mod_version,
            200,
            pad_get_mod_version_call,
        );
        require_paths(
            ee::s15::f_00400740__generated,
            200,
            &[94],
            pad_get_mod_version_call,
        );
    }

    /// `scePadGetPortMax`: the RPC always fails.
    #[test]
    fn sce_pad_get_port_max_matches() {
        let pad_get_port_max_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x0040_0670, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x0040_0670;
        };
        check_lohi(
            ee::s15::f_00400670__generated,
            sce_pad_get_port_max,
            200,
            pad_get_port_max_call,
        );
        require_paths(
            ee::s15::f_00400670__generated,
            200,
            &[94],
            pad_get_port_max_call,
        );
    }

    /// `scePadGetReqState`: closed and open ports, every request state.
    #[test]
    fn sce_pad_get_req_state_matches() {
        let pad_get_req_state_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x003f_fcd8, 0);
            elf_image(c);
            c.pc = 0x003f_fcd8;
            let (port, slot) = (r.next_u32() % 2, r.next_u32() % 2);
            let buf = pad_dma_buffer(c, &mut r, port, slot, (seed % 4 != 0) as u32);
            let req = (r.next_u32() % 4) as u8;
            for half in [buf, buf + 0x80] {
                c.bus.write8(half + DMA_REQ, req);
            }
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
        };
        check_lohi(
            ee::s15::f_003ffcd8__generated,
            sce_pad_get_req_state,
            300,
            pad_get_req_state_call,
        );
        require_paths(
            ee::s15::f_003ffcd8__generated,
            300,
            &[17, 1674],
            pad_get_req_state_call,
        );
    }

    /// `scePadGetSlotMax`: ports 0..3; the RPC always fails. Also entered
    /// with the frame open at the failed-RPC arm (0x40072c) and at the
    /// epilogue (0x400730), which only the generated body handles.
    #[test]
    fn sce_pad_get_slot_max_matches() {
        let pad_get_slot_max_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_06d8, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = [0x0040_06d8, 0x0040_072c, 0x0040_0730][(seed % 3) as usize];
            if c.pc != 0x0040_06d8 {
                open_frame(c, 0x30, 0x20);
            }
            c.g[A0][0] = (r.next_u32() % 4) as u64;
        };
        check_lohi(
            ee::s15::f_004006d8__generated,
            sce_pad_get_slot_max,
            200,
            pad_get_slot_max_call,
        );
        require_paths(
            ee::s15::f_004006d8__generated,
            200,
            &[4, 5, 95],
            pad_get_slot_max_call,
        );
    }

    /// `scePadInfoAct`: each port and slot; closed, an invalid buffer, a
    /// single-actuator mode or a valid buffer; ids of -1 (the count), inside,
    /// at and past the actuator count and 0xffffffff; every `idx` 0..5 plus
    /// junk.
    #[test]
    fn sce_pad_info_act_matches() {
        let info_act: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x003f_fd60, 0);
            elf_image(c);
            c.pc = 0x003f_fd60;
            let (port, slot) = (seed % 2, seed / 2 % 2);
            let state = seed / 4 % 4;
            let buf = pad_dma_buffer(c, &mut r, port, slot, (state != 0) as u32);
            let count = 1 + r.next_u32() % 3;
            for half in [buf, buf + 0x80] {
                c.bus.write8(half + DMA_OK, (state != 1) as u8);
                let cfg = if state == 2 {
                    r.next_u32() % 2
                } else {
                    2 + r.next_u32() % 3
                };
                c.bus.write8(half + DMA_CFG, cfg as u8);
                c.bus.write8(half + DMA_NACT, count as u8);
            }
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
            c.g[A2][0] = match seed / 16 % 5 {
                0 => 0xffff_ffff_ffff_ffff,
                1 => (r.next_u32() % count) as u64,
                2 => count as u64,
                3 => (count + 1) as u64,
                _ => 0xffff_ffff,
            };
            c.g[A3][0] = match seed / 80 % 7 {
                k @ 0..=5 => k as u64,
                _ => 99,
            };
        };
        check_lohi(
            ee::s15::f_003ffd60__generated,
            sce_pad_info_act,
            560,
            info_act,
        );
        require_paths(
            ee::s15::f_003ffd60__generated,
            560,
            &[23, 1683, 1687, 1692, 1695, 1699, 1700, 1702, 1703, 1705],
            info_act,
        );
    }

    /// `scePadInfoMode`: closed ports, invalid buffers, every `term`
    /// (0..4 plus junk), -1/small/large `offs`, matching and mismatching
    /// config bytes, and a term-4 `offs` equal to the mode count. Also
    /// entered with the frame open after the port check (0x3fffe4) and at
    /// the epilogue (0x4000c8), which only the generated body handles.
    #[test]
    fn sce_pad_info_mode_matches() {
        let pad_info_mode_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x003f_ffa0, 0);
            elf_image(c);
            c.pc = 0x003f_ffa0;
            let (port, slot) = (r.next_u32() % 2, r.next_u32() % 2);
            let at_count = seed % 11 == 9;
            let open = seed % 6 != 0 || at_count;
            let buf = pad_dma_buffer(c, &mut r, port, slot, open as u32);
            let ok = if seed % 6 == 1 { 0 } else { 1 };
            let cfg = if seed % 6 == 2 {
                0
            } else {
                1 + (r.next_u32() % 3) as u8
            };
            for half in [buf, buf + 0x80] {
                c.bus.write8(half + DMA_OK, ok);
                c.bus
                    .write8(half + DMA_REQ, if seed % 7 == 0 { 2 } else { 0 });
                c.bus.write8(half + DMA_CFG, cfg);
                c.bus.write8(
                    half + 0x65,
                    if seed % 5 == 0 {
                        0xf3
                    } else {
                        r.next_u32() as u8
                    },
                );
                c.bus.write8(half + 0x68, (r.next_u32() % 4) as u8);
                c.bus.write8(half + 0x69, (r.next_u32() % 8) as u8);
            }
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
            c.g[A2][0] = match seed % 8 {
                0..=4 => seed as u64 % 8,
                5 => 5,
                6 => 0xffff_ffff,
                _ => r.next_u32() as u64,
            };
            c.g[A3][0] = match seed % 5 {
                0 => 0xffff_ffff_ffff_ffff,
                1 => (r.next_u32() % 4) as u64,
                2 => 99,
                _ => r.next_u32() as u64,
            };
            if at_count {
                // Term 4 on a valid buffer, `offs` == the count at +0x68.
                let count = (r.next_u32() % 4) as u8;
                for half in [buf, buf + 0x80] {
                    c.bus.write8(half + DMA_OK, 1);
                    c.bus.write8(half + DMA_REQ, 0);
                    c.bus.write8(half + DMA_CFG, 2);
                    c.bus.write8(half + 0x68, count);
                }
                c.g[A2][0] = 4;
                c.g[A3][0] = count as u64;
            }
            if seed % 11 == 10 {
                c.pc = [0x003f_ffe4, 0x0040_00c8][(seed / 11 % 2) as usize];
                open_frame(c, 0x30, 0x20);
                (c.g[T0][0], c.g[S0][0], c.g[S1][0]) = (port as u64, c.g[A2][0], c.g[A3][0]);
            }
        };
        check_lohi(
            ee::s15::f_003fffa0__generated,
            sce_pad_info_mode,
            300,
            pad_info_mode_call,
        );
        require_paths(
            ee::s15::f_003fffa0__generated,
            300,
            &[
                4, 23, 1683, 1687, 1692, 1694, 1696, 1698, 1699, 1701, 1704, 1706,
            ],
            pad_info_mode_call,
        );
    }

    /// `scePadInfoPressMode`: closed ports, and open ports whose button
    /// mask is exactly `0x3ffff`, one bit short of it (`0x3fffe`) or random.
    #[test]
    fn sce_pad_info_press_mode_matches() {
        let pad_info_press_mode_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_0490, 0);
            elf_image(c);
            c.pc = 0x0040_0490;
            let (port, slot) = (r.next_u32() % 2, r.next_u32() % 2);
            let buf = pad_dma_buffer(c, &mut r, port, slot, (seed % 4 != 0) as u32);
            let mask = seed % 3;
            for half in [buf, buf + 0x80] {
                c.bus.write8(half + DMA_OK, 1);
                c.bus.write8(half + DMA_CFG, 2 + (r.next_u32() % 3) as u8);
                c.bus.write8(half + 0x66, 2 + (r.next_u32() % 3) as u8);
                if mask < 2 {
                    c.bus.write8(half + 0x79, 0xff - mask as u8);
                    c.bus.write8(half + 0x7a, 0xff);
                    c.bus.write8(half + 0x7b, 0x03);
                    c.bus.write8(half + 0x7c, 0x00);
                }
            }
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
        };
        check_lohi(
            ee::s15::f_00400490__generated,
            sce_pad_info_press_mode,
            300,
            pad_info_press_mode_call,
        );
        require_paths(
            ee::s15::f_00400490__generated,
            300,
            &[17, 1720],
            pad_info_press_mode_call,
        );
    }

    /// `scePadPortClose`: closed and open ports; the RPC always fails.
    #[test]
    fn sce_pad_port_close_matches() {
        let pad_port_close_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x003f_f8f0, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x003f_f8f0;
            let (port, slot) = (r.next_u32() % 2, r.next_u32() % 2);
            pad_dma_buffer(c, &mut r, port, slot, (seed % 3 != 0) as u32);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
        };
        check_lohi(
            ee::s15::f_003ff8f0__generated,
            sce_pad_port_close,
            200,
            pad_port_close_call,
        );
        require_paths(
            ee::s15::f_003ff8f0__generated,
            200,
            &[22, 114],
            pad_port_close_call,
        );
    }

    /// `int scePadPortOpen(port, slot, buf)`: misaligned buffers and
    /// slots already open (table word 1) print (debug flag set) or not,
    /// and return 0; other slots run the two-half buffer init and the open
    /// request, which fails here (no IOP) and returns 0.
    #[test]
    fn sce_pad_port_open_matches() {
        let pad_port_open_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            machine(c, seed, 0x1248_0013, 0);
            c.pc = 0x003f_f708;
            let port = (seed % 2) as u32;
            let slot = ((seed / 2) % 4) as u32;
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
            let aligned = seed / 8 % 2 == 0;
            let buf = if aligned {
                PORT_OPEN_BUF
            } else {
                PORT_OPEN_BUF + 1
            };
            c.g[A2][0] = buf as u64;
            c.bus.write32(PAD_DEBUG_WORD, seed / 16 % 2);
            c.bus.write32(
                PAD_SLOT_TABLE + port * 0x70 + slot * 0x1c,
                if seed % 3 == 0 { 0 } else { 1 },
            );
        };
        check_lohi(
            ee::s15::f_003ff708__generated,
            sce_pad_port_open,
            SEEDS,
            pad_port_open_call,
        );
        require_paths(
            ee::s15::f_003ff708__generated,
            SEEDS,
            &[34, 46, 96, 109, 231],
            pad_port_open_call,
        );
    }

    /// `int scePadPortOpen(port, slot, buf)` with the open request
    /// answered, for every port and slot: the slot is marked open, `buf`
    /// and the reply's geometry word (`+0x14`) go into the slot table, and
    /// reply word 3 (`+0xc`) is returned.
    #[test]
    fn sce_pad_port_open_answered_matches() {
        let pad_port_open_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = pad_answered(c, seed, 0x003f_f708);
            let (port, slot) = (seed % 2, seed / 2 % 4);
            // Any word but 1: the slot is not open yet.
            c.bus.write32(
                PAD_SLOT_TABLE + port * 0x70 + slot * 0x1c,
                [0u32, 2][(seed / 8 % 2) as usize],
            );
            fill(c, &mut r, PORT_OPEN_BUF, 0x40);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
            c.g[A2][0] = PORT_OPEN_BUF as u64;
        };
        check_lohi(
            ee::s15::f_003ff708__generated,
            sce_pad_port_open,
            16,
            pad_port_open_call,
        );
        require_paths(
            ee::s15::f_003ff708__generated,
            16,
            &[473],
            pad_port_open_call,
        );
    }

    /// `scePadSetActAlign`: aligned and misaligned 6-byte vectors; the RPC
    /// always fails.
    #[test]
    fn sce_pad_set_act_align_matches() {
        let pad_set_act_align_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_0250, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x0040_0250;
            let align = DATA + 0x3000 + if seed % 3 == 0 { 1 } else { 0 };
            for i in 0..6u32 {
                c.bus.write8(align + i, r.next_u32() as u8);
            }
            c.g[A0][0] = (r.next_u32() % 2) as u64;
            c.g[A1][0] = (r.next_u32() % 2) as u64;
            c.g[A2][0] = sx(align);
        };
        check_lohi(
            ee::s15::f_00400250__generated,
            sce_pad_set_act_align,
            200,
            pad_set_act_align_call,
        );
        require_paths(
            ee::s15::f_00400250__generated,
            200,
            &[154],
            pad_set_act_align_call,
        );
    }

    /// `scePadSetButtonInfo`: ports and infos; the RPC always fails.
    #[test]
    fn sce_pad_set_button_info_matches() {
        let pad_set_button_info_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_03e0, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x0040_03e0;
            c.g[A0][0] = (r.next_u32() % 2) as u64;
            c.g[A1][0] = (r.next_u32() % 2) as u64;
            c.g[A2][0] = if seed % 3 == 0 {
                0xfff
            } else {
                r.next_u32() as u64
            };
        };
        check_lohi(
            ee::s15::f_004003e0__generated,
            sce_pad_set_button_info,
            200,
            pad_set_button_info_call,
        );
        require_paths(
            ee::s15::f_004003e0__generated,
            200,
            &[104],
            pad_set_button_info_call,
        );
    }

    /// `scePadSetMainMode`: modes and locks; the RPC always fails.
    #[test]
    fn sce_pad_set_main_mode_matches() {
        let pad_set_main_mode_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_00d8, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x0040_00d8;
            c.g[A0][0] = (r.next_u32() % 2) as u64;
            c.g[A1][0] = (r.next_u32() % 2) as u64;
            c.g[A2][0] = if seed % 3 == 0 {
                2
            } else {
                r.next_u32() as u64
            };
            c.g[A3][0] = (seed % 2) as u64;
        };
        check_lohi(
            ee::s15::f_004000d8__generated,
            sce_pad_set_main_mode,
            200,
            pad_set_main_mode_call,
        );
        require_paths(
            ee::s15::f_004000d8__generated,
            200,
            &[105],
            pad_set_main_mode_call,
        );
    }

    /// `scePadSetReqState`: closed and open ports, states 0..3 and junk.
    #[test]
    fn sce_pad_set_req_state_matches() {
        let pad_set_req_state_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x003f_fc70, 0);
            elf_image(c);
            c.pc = 0x003f_fc70;
            let (port, slot) = (r.next_u32() % 2, r.next_u32() % 2);
            pad_dma_buffer(c, &mut r, port, slot, (seed % 4 != 0) as u32);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
            c.g[A2][0] = match seed % 5 {
                0..=3 => seed as u64 % 5,
                _ => r.next_u32() as u64,
            };
        };
        check_lohi(
            ee::s15::f_003ffc70__generated,
            sce_pad_set_req_state,
            300,
            pad_set_req_state_call,
        );
        require_paths(
            ee::s15::f_003ffc70__generated,
            300,
            &[21, 1677],
            pad_set_req_state_call,
        );
    }

    /// `scePadSetVrefParam`: aligned and misaligned params; the RPC fails.
    #[test]
    fn sce_pad_set_vref_param_matches() {
        let pad_set_vref_param_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_05a0, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x0040_05a0;
            let param = DATA + 0x3000 + if seed % 4 == 0 { 3 } else { 0 };
            fill(c, &mut r, param, 4);
            c.g[A0][0] = (r.next_u32() % 2) as u64;
            c.g[A1][0] = (r.next_u32() % 2) as u64;
            c.g[A2][0] = sx(param);
        };
        check_lohi(
            ee::s15::f_004005a0__generated,
            sce_pad_set_vref_param,
            200,
            pad_set_vref_param_call,
        );
        require_paths(
            ee::s15::f_004005a0__generated,
            200,
            &[111],
            pad_set_vref_param_call,
        );
    }

    /// `scePadSetWarningLevel`: levels 0..2 and junk; the RPC always fails.
    #[test]
    fn sce_pad_set_warning_level_matches() {
        let pad_set_warning_level_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x0040_07a8, 0);
            elf_image(c);
            pad_no_iop(c);
            c.pc = 0x0040_07a8;
            c.g[A0][0] = match seed % 4 {
                0 => 0,
                1 => 1,
                2 => 2,
                _ => r.next_u32() as u64,
            };
        };
        check_lohi(
            ee::s15::f_004007a8__generated,
            sce_pad_set_warning_level,
            200,
            pad_set_warning_level_call,
        );
        require_paths(
            ee::s15::f_004007a8__generated,
            200,
            &[95],
            pad_set_warning_level_call,
        );
    }

    /// `int libpad_InitRpc(a)` with the init RPC answered: returns reply
    /// word 3 of the block at 0x526440.
    #[test]
    fn libpad_init_rpc_answered_matches() {
        let call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = pad_answered(c, seed, 0x003f_f5e8);
            c.g[A0][0] = r.next_u32() as u64;
        };
        check(ee::s15::f_003ff5e8__generated, libpad_init_rpc, 12, call);
        require_paths(ee::s15::f_003ff5e8__generated, 12, &[354], call);
    }

    /// `scePadGetModVersion` with the RPC answered: returns reply word 3.
    #[test]
    fn sce_pad_get_mod_version_answered_matches() {
        let call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            pad_answered(c, seed, 0x0040_0740);
        };
        check_lohi(
            ee::s15::f_00400740__generated,
            sce_pad_get_mod_version,
            12,
            call,
        );
        require_paths(ee::s15::f_00400740__generated, 12, &[313], call);
    }

    /// `scePadGetPortMax` with the RPC answered: returns reply word 3.
    #[test]
    fn sce_pad_get_port_max_answered_matches() {
        let call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            pad_answered(c, seed, 0x0040_0670);
        };
        check_lohi(
            ee::s15::f_00400670__generated,
            sce_pad_get_port_max,
            12,
            call,
        );
        require_paths(ee::s15::f_00400670__generated, 12, &[313], call);
    }

    /// `scePadGetSlotMax` with the RPC answered, ports 0..3: returns reply
    /// word 3.
    #[test]
    fn sce_pad_get_slot_max_answered_matches() {
        let call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            pad_answered(c, seed, 0x0040_06d8);
            c.g[A0][0] = (seed % 4) as u64;
        };
        check_lohi(
            ee::s15::f_004006d8__generated,
            sce_pad_get_slot_max,
            12,
            call,
        );
        require_paths(ee::s15::f_004006d8__generated, 12, &[314], call);
    }

    /// `scePadPortClose` with the RPC answered, closed and open ports: an open
    /// port's open word is cleared and reply word 3 returned.
    #[test]
    fn sce_pad_port_close_answered_matches() {
        let call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = pad_answered(c, seed, 0x003f_f8f0);
            let (port, slot) = (seed % 2, seed / 2 % 2);
            pad_dma_buffer(c, &mut r, port, slot, (seed / 4 % 3 != 0) as u32);
            c.g[A0][0] = port as u64;
            c.g[A1][0] = slot as u64;
        };
        check_lohi(ee::s15::f_003ff8f0__generated, sce_pad_port_close, 12, call);
        require_paths(ee::s15::f_003ff8f0__generated, 12, &[22, 333], call);
    }

    /// `scePadSetWarningLevel` with the RPC answered, levels 0..2: returns
    /// reply word 2.
    #[test]
    fn sce_pad_set_warning_level_answered_matches() {
        let call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            pad_answered(c, seed, 0x0040_07a8);
            c.g[A0][0] = (seed % 3) as u64;
        };
        check_lohi(
            ee::s15::f_004007a8__generated,
            sce_pad_set_warning_level,
            12,
            call,
        );
        require_paths(ee::s15::f_004007a8__generated, 12, &[314], call);
    }

    /// `scePadSetActAlign` with the RPC answered: reply word 1 (through
    /// `scePadSetReqState`, open and closed ports), 0 or 2.
    #[test]
    fn sce_pad_set_act_align_answered_matches() {
        let pad_set_act_align_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = pad_reply_call(c, seed, 0x0040_0250, 0x14);
            fill(c, &mut r, DATA + 0x3000, 2);
            c.g[A2][0] = (DATA + 0x3000) as u64;
        };
        check_lohi(
            ee::s15::f_00400250__generated,
            sce_pad_set_act_align,
            24,
            pad_set_act_align_call,
        );
        require_paths(
            ee::s15::f_00400250__generated,
            24,
            &[375, 402, 2081],
            pad_set_act_align_call,
        );
    }

    /// `scePadSetButtonInfo` with the RPC answered: reply word 1 (through
    /// `scePadSetReqState`, open and closed ports), 0 or 2.
    #[test]
    fn sce_pad_set_button_info_answered_matches() {
        let pad_set_button_info_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = pad_reply_call(c, seed, 0x0040_03e0, 0x10);
            c.g[A2][0] = r.next_u32() as u64;
        };
        check_lohi(
            ee::s15::f_004003e0__generated,
            sce_pad_set_button_info,
            24,
            pad_set_button_info_call,
        );
        require_paths(
            ee::s15::f_004003e0__generated,
            24,
            &[325, 352, 2031],
            pad_set_button_info_call,
        );
    }

    /// `scePadSetMainMode` with the RPC answered: reply word 1 (through
    /// `scePadSetReqState`, open and closed ports), 0 or 2.
    #[test]
    fn sce_pad_set_main_mode_answered_matches() {
        let pad_set_main_mode_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = pad_reply_call(c, seed, 0x0040_00d8, 0x14);
            c.g[A2][0] = (r.next_u32() % 2) as u64;
            c.g[A3][0] = (r.next_u32() % 4) as u64;
        };
        check_lohi(
            ee::s15::f_004000d8__generated,
            sce_pad_set_main_mode,
            24,
            pad_set_main_mode_call,
        );
        require_paths(
            ee::s15::f_004000d8__generated,
            24,
            &[326, 353, 2032],
            pad_set_main_mode_call,
        );
    }

    /// `scePadSetVrefParam` with the RPC answered: reply word 1 (through
    /// `scePadSetReqState`, open and closed ports), 0 or 2.
    #[test]
    fn sce_pad_set_vref_param_answered_matches() {
        let pad_set_vref_param_call: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = pad_reply_call(c, seed, 0x0040_05a0, 0x1c);
            fill(c, &mut r, DATA + 0x3000, 3);
            c.g[A2][0] = (DATA + 0x3000) as u64;
        };
        check_lohi(
            ee::s15::f_004005a0__generated,
            sce_pad_set_vref_param,
            24,
            pad_set_vref_param_call,
        );
        require_paths(
            ee::s15::f_004005a0__generated,
            24,
            &[332, 359, 2038],
            pad_set_vref_param_call,
        );
    }

    /// The pad debug-print flag at 0x4515cc both pad setup paths consult.
    pub(crate) const PAD_DEBUG_WORD: u32 = 0x0045_15cc;

    /// The pad slot-use table `scePadPortOpen` tests (port * 0x70 + slot
    /// * 0x1c words from this base).
    pub(crate) const PAD_SLOT_TABLE: u32 = 0x0052_6260;

    /// Scratch `buf` argument for `scePadPortOpen` (0x100 bytes).
    pub(crate) const PORT_OPEN_BUF: u32 = DATA + 0x200;

    /// Seeds per test (200-300).
    pub(crate) const SEEDS: u32 = 200;

    /// Point the libpad entry `port * 0x70 + slot * 0x1c` at a scratch DMA
    /// buffer, mark it `open`, randomize both halves, and pick the newer
    /// half with the frame counters. Returns the buffer.
    pub(crate) fn pad_dma_buffer(c: &mut Cpu, r: &mut Rng, port: u32, slot: u32, open: u32) -> u32 {
        let entry = PAD_PORTS + port * PORT_STRIDE + slot * SLOT_STRIDE;
        let buf = DATA + 0x1000 + 0x100 * (r.next_u32() % 8);
        c.bus.write32(entry, buf);
        c.bus.write32(entry + SLOT_OPEN, open);
        fill(c, r, buf, 0x40);
        if r.next_u32() % 2 == 0 {
            c.bus.write32(buf + 0x58, 1);
            c.bus.write32(buf + 0xd8, 0);
        } else {
            c.bus.write32(buf + 0x58, 0);
            c.bus.write32(buf + 0xd8, 1);
        }
        buf
    }

    /// Zero the SIF RPC packet pool so the pad RPC wrappers fail at once.
    pub(crate) fn pad_no_iop(c: &mut Cpu) {
        put_words(c, SIF_RPC_POOL, &[0, 0, 0, 0]);
    }

    /// The libpad SIF RPC client.
    const PAD_CLIENT: u32 = 0x0052_6200;

    /// Pad RPCs the server answers, called at `entry` on a running guest
    /// thread. No reply packet comes back, so the request/reply block at
    /// 0x526440 keeps the random words written here.
    fn pad_answered(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(entry as u64 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
        elf_image(c);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        rpc_answered(c, PAD_CLIENT, 0);
        ssx3_ps2::kernel::start_main_thread(c, entry, 0, 0);
        c.g[RA][0] = crate::difftest::RA as u64;
        fill(c, &mut r, PAD_RPC_BUF, 0x20);
        r
    }

    /// An answered pad RPC wrapper call: port and slot from the seed, the
    /// port open or closed (what the reply-1 arm's `scePadSetReqState`
    /// sees), and the reply word at `PAD_RPC_BUF + reply` 1, 0 or 2.
    fn pad_reply_call(c: &mut Cpu, seed: u32, entry: u32, reply: u32) -> Rng {
        let mut r = pad_answered(c, seed, entry);
        let (port, slot) = (seed / 3 % 2, seed / 6 % 2);
        pad_dma_buffer(c, &mut r, port, slot, seed / 12 % 2);
        c.bus
            .write32(PAD_RPC_BUF + reply, [1, 0, 2][(seed % 3) as usize]);
        c.g[A0][0] = port as u64;
        c.g[A1][0] = slot as u64;
        r
    }

    /// Enter mid-body with a `frame`-byte frame already open: the saved `ra`
    /// at `ra_slot` returns to the harness; the other slots keep the
    /// random bytes `machine` put around the stack.
    fn open_frame(c: &mut Cpu, frame: u32, ra_slot: u32) {
        let sp = crate::difftest::SP - frame;
        c.g[SP][0] = sp as u64;
        c.bus.write64(sp + ra_slot, crate::difftest::RA as u64);
    }

    driven!(sce_pad_port_open_gen_driven, ee::s15::f_003ff708__generated);
    driven!(sce_pad_port_open_hand_driven, sce_pad_port_open);

    /// Opening a pad port on an aligned buffer whose slot is not marked
    /// open: the two-half buffer init (a memset per half) with a live
    /// event between the halves; the open request then fails (no IOP) and
    /// returns 0.
    #[test]
    fn sce_pad_port_open_events() {
        check_events(
            sce_pad_port_open_gen_driven,
            sce_pad_port_open_hand_driven,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003f_f7d8, 0);
                elf_image(c);
                pad_no_iop(c);
                c.pc = 0x003f_f708;
                let port = seed / 3 % 2;
                let slot = seed / 3 / 2 % 4;
                // 64-byte aligned (the init path), nonzero low byte.
                let buf = DATA + 0x2c0;
                for i in 0..0x40u32 {
                    c.bus.write32(buf + i * 4, r.next_u32() | 0x0101_0101);
                }
                // Anything but 1 reaches the init loop.
                c.bus.write32(
                    PAD_SLOT_TABLE + port * 0x70 + slot * 0x1c,
                    [0u32, 2, 0xffff_ffff][(seed / 3 / 8 % 3) as usize],
                );
                c.g[A0][0] = port as u64;
                c.g[A1][0] = slot as u64;
                c.g[A2][0] = buf as u64;
                schedule(c, &mut r, seed, 0x003f_f708)
            },
        );
    }

    driven!(
        sce_pad_set_act_align_gen_driven,
        ee::s15::f_00400250__generated
    );
    driven!(sce_pad_set_act_align_hand_driven, sce_pad_set_act_align);

    /// Setting the actuator alignment: the 6-byte vector is copied into
    /// the request a byte at a time with a live event during the copy; the
    /// request then fails (no IOP) and returns 0.
    #[test]
    fn sce_pad_set_act_align_events() {
        check_events(
            sce_pad_set_act_align_gen_driven,
            sce_pad_set_act_align_hand_driven,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0040_0290, 0);
                elf_image(c);
                pad_no_iop(c);
                c.pc = 0x0040_0250;
                // Aligned and misaligned vectors.
                let align = DATA + 0x3010 + seed / 3 % 4;
                for i in 0..6u32 {
                    c.bus.write8(align + i, r.next_u32() as u8 | 0x80);
                }
                c.g[A0][0] = (seed / 3 / 4 % 2) as u64;
                c.g[A1][0] = (seed / 3 / 4 / 2 % 2) as u64;
                c.g[A2][0] = sx(align);
                schedule(c, &mut r, seed, 0x0040_0250)
            },
        );
    }
}
