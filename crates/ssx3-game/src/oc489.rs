//! Staging module oc489 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc489`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0016_73f8,
    chase_camera_algorithm_static_init,
    "ChaseCameraAlgorithm_StaticInit",
)];

/// Where a table store goes: an absolute address, or `gp + off`.
#[derive(Clone, Copy)]
pub(crate) enum CsiLoc {
    A(u32),
    G(i32),
}

/// What a table store writes: literal bits, or the `.sdata` word at `gp + off`.
#[derive(Clone, Copy)]
pub(crate) enum CsiSrc {
    B(u32),
    P(i32),
}

/// How a table store writes: `sw`, `swc1` (a bus word write), or `sb`.
#[derive(Clone, Copy)]
pub(crate) enum CsiKind {
    W,
    F,
    B,
}

// The store tables below copy the generated body's stores in the order it
// makes them, with the registers it leaves; `CsiLoc` and `CsiSrc` are the
// address and value forms.

fn csi_at(c: &Cpu, l: CsiLoc) -> Addr {
    match l {
        CsiLoc::A(a) => Addr(a),
        CsiLoc::G(off) => Addr((c.g[GP][0] as u32).wrapping_add(off as u32)),
    }
}

fn csi_src(c: &mut Cpu, s: CsiSrc) -> u32 {
    match s {
        CsiSrc::B(v) => v,
        CsiSrc::P(off) => Addr((c.g[GP][0] as u32).wrapping_add(off as u32)).read_u32(c),
    }
}

fn csi_run(c: &mut Cpu, table: &[(CsiKind, CsiLoc, CsiSrc)]) {
    for &(k, l, s) in table {
        let v = csi_src(c, s);
        let a = csi_at(c, l);
        match k {
            CsiKind::W => a.sw(c, v as u64),
            CsiKind::F => a.write_u32(c, v),
            CsiKind::B => a.sb(c, v as u64),
        }
    }
}

fn csi_regs(c: &mut Cpu, regs: &[(usize, u64)]) {
    for &(r, v) in regs {
        c.g[r][0] = v;
    }
}

fn csi_fprs(c: &mut Cpu, regs: &[(usize, CsiSrc)]) {
    for &(f, s) in regs {
        let v = csi_src(c, s);
        c.f[f] = v;
    }
}

/// `ChaseCameraAlgorithm_StaticInit(int initialize, int priority)` (0x1673f8),
/// the chase camera's one-time static init. Does its work only for
/// `priority == 0xffff` and a non-zero `initialize`: fills the camera colour
/// table at `D_004C5020`, the flag records at `D_004A4D40`, the axis-flip
/// matrix, calls `GameCamera_InitShakePresets` (0x15e2a8), then writes the byte
/// and word fields at `gp+0x1c50..` and `persistentPlayerChoiceType[0..2] = 61`.
/// 599 cycles before the call, 12 after (45 more in the callee).
pub fn chase_camera_algorithm_static_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0xffff;
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        // `priority` (a1) must be 0xffff and `initialize` (a0) non-zero.
        if c.g[A1][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[T5][0] = 0x4c0000;
            if c.g[A0][0] != 0 {
                c.cycles += 584;
                csi_body(c)?;
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body after the entry checks: the stores before the call, the registers
/// the original holds at the call, the call itself, then the stores and the
/// final registers after it. An unwinding callee returns straight away.
fn csi_body(c: &mut Cpu) -> Flow {
    csi_run(c, CSI_PRE);
    csi_regs(c, CSI_CALL_GPR);
    csi_fprs(c, CSI_CALL_FPR);
    call_ptr(c, 0x0015_e2a8, 0x0016_7d2c)?;
    csi_regs(c, CSI_POST_GPR);
    csi_fprs(c, CSI_POST_FPR);
    csi_run(c, CSI_POST);
    Ok(())
}

// Stores before the call (CSI_PRE) and after it (CSI_POST), in order.
pub(crate) const CSI_PRE: &[(CsiKind, CsiLoc, CsiSrc)] = &[
    (CsiKind::F, CsiLoc::A(0x004c5030), CsiSrc::P(-0x689c)),
    (CsiKind::F, CsiLoc::A(0x004c5040), CsiSrc::P(-0x6898)),
    (CsiKind::F, CsiLoc::A(0x004c5050), CsiSrc::P(-0x6894)),
    (CsiKind::F, CsiLoc::A(0x004c505c), CsiSrc::P(-0x6890)),
    (CsiKind::F, CsiLoc::A(0x004c5060), CsiSrc::P(-0x688c)),
    (CsiKind::F, CsiLoc::A(0x004c5070), CsiSrc::P(-0x6888)),
    (CsiKind::F, CsiLoc::A(0x004c5080), CsiSrc::P(-0x6884)),
    (CsiKind::F, CsiLoc::A(0x004c508c), CsiSrc::P(-0x6880)),
    (CsiKind::F, CsiLoc::A(0x004c5090), CsiSrc::P(-0x687c)),
    (CsiKind::F, CsiLoc::A(0x004c50a0), CsiSrc::P(-0x6878)),
    (CsiKind::F, CsiLoc::A(0x004c5020), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c502c), CsiSrc::B(0x3f000000)),
    (CsiKind::W, CsiLoc::A(0x004c504c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5054), CsiSrc::P(-0x6874)),
    (CsiKind::F, CsiLoc::A(0x004c5058), CsiSrc::P(-0x6870)),
    (CsiKind::F, CsiLoc::A(0x004c5064), CsiSrc::P(-0x686c)),
    (CsiKind::F, CsiLoc::A(0x004c5068), CsiSrc::P(-0x6868)),
    (CsiKind::W, CsiLoc::A(0x004c506c), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c507c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5084), CsiSrc::P(-0x6864)),
    (CsiKind::F, CsiLoc::A(0x004c5088), CsiSrc::P(-0x6860)),
    (CsiKind::F, CsiLoc::A(0x004c5094), CsiSrc::P(-0x685c)),
    (CsiKind::F, CsiLoc::A(0x004c509c), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5024), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5028), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5034), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5038), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c503c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5044), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5048), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5074), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5078), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5098), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c50a4), CsiSrc::P(-0x6858)),
    (CsiKind::F, CsiLoc::A(0x004c50b0), CsiSrc::P(-0x6854)),
    (CsiKind::F, CsiLoc::A(0x004c50bc), CsiSrc::P(-0x6850)),
    (CsiKind::F, CsiLoc::A(0x004c50ec), CsiSrc::P(-0x684c)),
    (CsiKind::F, CsiLoc::A(0x004c50a8), CsiSrc::P(-0x6848)),
    (CsiKind::W, CsiLoc::A(0x004c50ac), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c50b4), CsiSrc::P(-0x6844)),
    (CsiKind::F, CsiLoc::A(0x004c50b8), CsiSrc::P(-0x6840)),
    (CsiKind::F, CsiLoc::A(0x004c50c0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c50cc), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c50d0), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c50dc), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c50e0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c50e8), CsiSrc::P(-0x683c)),
    (CsiKind::F, CsiLoc::A(0x004c50f0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c50fc), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5100), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c510c), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5110), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c511c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5120), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c50c4), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c50c8), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c50d4), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c50d8), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c50e4), CsiSrc::P(-0x684c)),
    (CsiKind::F, CsiLoc::A(0x004c50f4), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c50f8), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5104), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5108), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c5114), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c5118), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c5124), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c5128), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c512c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c516c), CsiSrc::B(0x3f000000)),
    (CsiKind::F, CsiLoc::A(0x004c517c), CsiSrc::P(-0x6838)),
    (CsiKind::F, CsiLoc::A(0x004c5190), CsiSrc::P(-0x6834)),
    (CsiKind::F, CsiLoc::A(0x004c5130), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c513c), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5140), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c514c), CsiSrc::B(0x3f000000)),
    (CsiKind::F, CsiLoc::A(0x004c5150), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c515c), CsiSrc::B(0x3f000000)),
    (CsiKind::F, CsiLoc::A(0x004c5160), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5170), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5174), CsiSrc::P(-0x6830)),
    (CsiKind::F, CsiLoc::A(0x004c5178), CsiSrc::P(-0x682c)),
    (CsiKind::F, CsiLoc::A(0x004c5180), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5188), CsiSrc::P(-0x6828)),
    (CsiKind::W, CsiLoc::A(0x004c518c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5194), CsiSrc::P(-0x6824)),
    (CsiKind::F, CsiLoc::A(0x004c5198), CsiSrc::P(-0x6820)),
    (CsiKind::W, CsiLoc::A(0x004c519c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c51a0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51ac), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51b0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5134), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5138), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5144), CsiSrc::B(0x3f000000)),
    (CsiKind::F, CsiLoc::A(0x004c5148), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5154), CsiSrc::B(0x3f000000)),
    (CsiKind::F, CsiLoc::A(0x004c5158), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5164), CsiSrc::B(0x3f000000)),
    (CsiKind::F, CsiLoc::A(0x004c5168), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5184), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51a4), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51a8), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51b4), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51dc), CsiSrc::P(-0x681c)),
    (CsiKind::F, CsiLoc::A(0x004c51fc), CsiSrc::P(-0x6818)),
    (CsiKind::F, CsiLoc::A(0x004c51b8), CsiSrc::P(-0x6814)),
    (CsiKind::W, CsiLoc::A(0x004c51bc), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c51c0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51c8), CsiSrc::P(-0x680c)),
    (CsiKind::F, CsiLoc::A(0x004c51c4), CsiSrc::P(-0x6810)),
    (CsiKind::W, CsiLoc::A(0x004c51cc), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c51d0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51d4), CsiSrc::P(-0x6808)),
    (CsiKind::F, CsiLoc::A(0x004c51d8), CsiSrc::P(-0x6804)),
    (CsiKind::F, CsiLoc::A(0x004c51e0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51e8), CsiSrc::P(-0x6800)),
    (CsiKind::F, CsiLoc::A(0x004c51ec), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51f0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51f4), CsiSrc::P(-0x67fc)),
    (CsiKind::F, CsiLoc::A(0x004c5200), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c520c), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5210), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c521c), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5220), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5224), CsiSrc::P(-0x67f8)),
    (CsiKind::F, CsiLoc::A(0x004c5228), CsiSrc::P(-0x67f4)),
    (CsiKind::W, CsiLoc::A(0x004c522c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5230), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c51e4), CsiSrc::P(-0x67f0)),
    (CsiKind::F, CsiLoc::A(0x004c51f8), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5204), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5208), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5214), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5218), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5234), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5238), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c523c), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5270), CsiSrc::P(-0x67ec)),
    (CsiKind::F, CsiLoc::A(0x004c528c), CsiSrc::P(-0x67e8)),
    (CsiKind::F, CsiLoc::A(0x004c529c), CsiSrc::P(-0x67e4)),
    (CsiKind::F, CsiLoc::A(0x004c52bc), CsiSrc::P(-0x67e0)),
    (CsiKind::F, CsiLoc::A(0x004c5240), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5244), CsiSrc::P(-0x67dc)),
    (CsiKind::W, CsiLoc::A(0x004c524c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5250), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c525c), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5260), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c526c), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c527c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5280), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5284), CsiSrc::P(-0x67d8)),
    (CsiKind::F, CsiLoc::A(0x004c5288), CsiSrc::P(-0x67d4)),
    (CsiKind::F, CsiLoc::A(0x004c5290), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5294), CsiSrc::P(-0x67d0)),
    (CsiKind::F, CsiLoc::A(0x004c52a0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52a8), CsiSrc::P(-0x67cc)),
    (CsiKind::W, CsiLoc::A(0x004c52ac), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c52b0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52b4), CsiSrc::P(-0x67c8)),
    (CsiKind::F, CsiLoc::A(0x004c52c0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5248), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5254), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5258), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5264), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c5268), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c5274), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::A(0x004c5278), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5298), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52a4), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52b8), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52c4), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52dc), CsiSrc::P(-0x67f0)),
    (CsiKind::F, CsiLoc::A(0x004c531c), CsiSrc::P(-0x67c4)),
    (CsiKind::F, CsiLoc::A(0x004c532c), CsiSrc::P(-0x67c0)),
    (CsiKind::F, CsiLoc::A(0x004c52cc), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52d0), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52e0), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c52ec), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c52f0), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c52fc), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5300), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c530c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5310), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5314), CsiSrc::P(-0x67bc)),
    (CsiKind::F, CsiLoc::A(0x004c5320), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5328), CsiSrc::P(-0x67b8)),
    (CsiKind::F, CsiLoc::A(0x004c5330), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5338), CsiSrc::P(-0x67b4)),
    (CsiKind::W, CsiLoc::A(0x004c533c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5340), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5344), CsiSrc::P(-0x67b0)),
    (CsiKind::F, CsiLoc::A(0x004c5348), CsiSrc::P(-0x67ac)),
    (CsiKind::F, CsiLoc::A(0x004c52c8), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52d4), CsiSrc::P(-0x67f0)),
    (CsiKind::F, CsiLoc::A(0x004c52d8), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c52e4), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c52e8), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c52f4), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c52f8), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5304), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c5308), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5318), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5324), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5334), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c534c), CsiSrc::B(0x0)),
    (CsiKind::B, CsiLoc::G(0x1c68), CsiSrc::B(0x2)),
    (CsiKind::B, CsiLoc::G(0x1c58), CsiSrc::B(0x0)),
    (CsiKind::B, CsiLoc::G(0x1c60), CsiSrc::B(0x1)),
    (CsiKind::B, CsiLoc::G(0x1c70), CsiSrc::B(0x3)),
    (CsiKind::F, CsiLoc::A(0x004c536c), CsiSrc::P(-0x67a8)),
    (CsiKind::F, CsiLoc::A(0x004c537c), CsiSrc::P(-0x67a4)),
    (CsiKind::F, CsiLoc::A(0x004c5350), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5354), CsiSrc::P(-0x67a0)),
    (CsiKind::F, CsiLoc::A(0x004c5358), CsiSrc::P(-0x679c)),
    (CsiKind::W, CsiLoc::A(0x004c535c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5360), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5364), CsiSrc::P(-0x6798)),
    (CsiKind::F, CsiLoc::A(0x004c5368), CsiSrc::P(-0x6794)),
    (CsiKind::F, CsiLoc::A(0x004c5370), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5374), CsiSrc::P(-0x6790)),
    (CsiKind::F, CsiLoc::A(0x004c5378), CsiSrc::P(-0x678c)),
    (CsiKind::F, CsiLoc::A(0x004c5380), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c538c), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5390), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c539c), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::G(0x1c58), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::G(0x1c60), CsiSrc::B(0x101)),
    (CsiKind::W, CsiLoc::G(0x1c68), CsiSrc::B(0x202)),
    (CsiKind::W, CsiLoc::A(0x004c5384), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c5388), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c5394), CsiSrc::B(0x3f800000)),
    (CsiKind::W, CsiLoc::A(0x004c5398), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::G(0x1c54), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::G(0x1c50), CsiSrc::B(0x0)),
    (CsiKind::W, CsiLoc::G(0x1c70), CsiSrc::B(0x303)),
    (CsiKind::B, CsiLoc::G(0x1c78), CsiSrc::B(0x4)),
    (CsiKind::B, CsiLoc::G(0x1c80), CsiSrc::B(0x5)),
    (CsiKind::B, CsiLoc::G(0x1c88), CsiSrc::B(0x6)),
    (CsiKind::B, CsiLoc::G(0x1c98), CsiSrc::B(0x8)),
    (CsiKind::B, CsiLoc::G(0x1ca0), CsiSrc::B(0x9)),
    (CsiKind::B, CsiLoc::G(0x1ca8), CsiSrc::B(0xa)),
    (CsiKind::B, CsiLoc::G(0x1cb0), CsiSrc::B(0xb)),
    (CsiKind::B, CsiLoc::G(0x1cb8), CsiSrc::B(0xc)),
    (CsiKind::B, CsiLoc::G(0x1cc0), CsiSrc::B(0xd)),
    (CsiKind::B, CsiLoc::G(0x1c90), CsiSrc::B(0x7)),
    (CsiKind::W, CsiLoc::G(0x1cb0), CsiSrc::B(0xb0b)),
    (CsiKind::W, CsiLoc::G(0x1cb8), CsiSrc::B(0xc0c)),
    (CsiKind::W, CsiLoc::G(0x1cc0), CsiSrc::B(0xd0d)),
    (CsiKind::W, CsiLoc::G(0x1c78), CsiSrc::B(0x404)),
    (CsiKind::W, CsiLoc::G(0x1c80), CsiSrc::B(0x505)),
    (CsiKind::W, CsiLoc::G(0x1c88), CsiSrc::B(0x606)),
    (CsiKind::W, CsiLoc::G(0x1c90), CsiSrc::B(0x707)),
    (CsiKind::W, CsiLoc::G(0x1c98), CsiSrc::B(0x808)),
    (CsiKind::W, CsiLoc::G(0x1ca0), CsiSrc::B(0x909)),
    (CsiKind::W, CsiLoc::G(0x1ca8), CsiSrc::B(0xa0a)),
    (CsiKind::F, CsiLoc::A(0x004c53a0), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53b0), CsiSrc::B(0xbf800000)),
    (CsiKind::F, CsiLoc::A(0x004c53c0), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53dc), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c53cc), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53d0), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53a4), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53b4), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53c4), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c53d4), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53a8), CsiSrc::B(0x3f800000)),
    (CsiKind::F, CsiLoc::A(0x004c53b8), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53c8), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53d8), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53ac), CsiSrc::B(0x0)),
    (CsiKind::F, CsiLoc::A(0x004c53bc), CsiSrc::B(0x0)),
];
pub(crate) const CSI_POST: &[(CsiKind, CsiLoc, CsiSrc)] = &[
    (CsiKind::B, CsiLoc::G(0x1cd0), CsiSrc::B(0xff)),
    (CsiKind::W, CsiLoc::G(0x1ce4), CsiSrc::B(0x3d)),
    (CsiKind::W, CsiLoc::G(0x1ce0), CsiSrc::B(0x3d)),
    (CsiKind::W, CsiLoc::G(0x1cd0), CsiSrc::B(0xffffffff)),
];
pub(crate) const CSI_CALL_GPR: &[(usize, u64)] = &[
    (A0, 0xb0b),
    (A1, 0xa0a),
    (A2, 0x909),
    (A3, 0x808),
    (AT, 0xffffffffbf800000),
    (FP, 0x7),
    (S0, 0x4c5310),
    (S1, 0x4c5300),
    (S2, 0x6),
    (S3, 0x4c5180),
    (S4, 0x4c5130),
    (S5, 0x4c0000),
    (S6, 0x4c0000),
    (S7, 0x4c0000),
    (T0, 0x707),
    (T1, 0x404),
    (T2, 0x505),
    (T3, 0x606),
    (T4, 0x4c53a0),
    (T5, 0x4c5030),
    (T6, 0x4c5330),
    (T7, 0x4c0000),
    (T8, 0x4c0000),
    (T9, 0x4c0000),
    (V0, 0xd0d),
    (V1, 0xc0c),
];
pub(crate) const CSI_CALL_FPR: &[(usize, CsiSrc)] = &[
    (0, CsiSrc::B(0x0)),
    (1, CsiSrc::B(0x3f800000)),
    (2, CsiSrc::B(0xbf800000)),
    (3, CsiSrc::P(-0x67a0)),
    (4, CsiSrc::P(-0x679c)),
    (5, CsiSrc::P(-0x6798)),
    (6, CsiSrc::P(-0x6794)),
    (7, CsiSrc::P(-0x6790)),
    (8, CsiSrc::P(-0x678c)),
    (9, CsiSrc::P(-0x67cc)),
    (10, CsiSrc::P(-0x67c8)),
    (11, CsiSrc::P(-0x67f4)),
    (12, CsiSrc::P(-0x67f0)),
    (13, CsiSrc::P(-0x686c)),
    (14, CsiSrc::P(-0x6868)),
    (15, CsiSrc::P(-0x6864)),
    (16, CsiSrc::P(-0x6860)),
    (17, CsiSrc::P(-0x6874)),
    (18, CsiSrc::P(-0x6870)),
    (19, CsiSrc::P(-0x6858)),
];
pub(crate) const CSI_POST_GPR: &[(usize, u64)] = &[
    (A0, 0x3d),
    (V0, 0xffffffffffffffff),
    (V1, 0xffffffffffffff00),
];
pub(crate) const CSI_POST_FPR: &[(usize, CsiSrc)] = &[];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, GAME_GP};

    /// The fixture's stack and return address (the difftest values).
    const TEST_SP: u32 = crate::difftest::SP;
    const TEST_RA: u32 = crate::difftest::RA;

    #[test]
    fn chase_camera_algorithm_static_init_matches() {
        check(
            ee::s02::f_001673f8__generated,
            chase_camera_algorithm_static_init,
            12,
            &|c, seed| {
                let mut r = Rng(0x1673_f8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = sx(TEST_SP);
                c.g[RA][0] = sx(TEST_RA);
                c.pc = 0x0016_73f8;
                // Priority 0xffff with a non-zero initialize runs the body; the
                // other seeds take the two early exits.
                c.g[A1][0] = if seed % 4 == 3 { 0x1234 } else { 0xffff };
                c.g[A0][0] = if seed % 3 == 2 {
                    0
                } else {
                    1 + (r.next_u32() % 5) as u64
                };
            },
        );
    }
}
