//! Staging module oc1486 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1486`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_e420, eafile_complete_request, "EAFile_CompleteRequest"),
    (0x002b_dd10, sky_box_is_equal, "tWPInGameData_SkyBox_isEqual"),
    (0x0011_5ab0, rider_get_gate_charge_angle, "Rider_GetGateChargeAngle"),
    (0x0010_8c28, rider_is_intersecting, "cRider_isIntersecting"),
    (0x0011_70a8, chudfx_show_number, "cHUDFX_ShowNumber"),
    (0x003a_01fc, lbl_003a01fc, "lbl_003A01FC"),
    (0x0031_a3c0, vu0_restore_context, "VU0_RestoreContext"),
    (0x0031_a308, vu0_save_context, "VU0_SaveContext"),
];

/// `EAFile_CompleteRequest` (0x003de420): completes one file request. Bumps
/// the outstanding-request counter (the word at 0x519ac0), takes the pending
/// request at 0x519ad4 when there is one, stores its status word (-1 while the
/// request is busy; -2, or 1 when `a0` is non-zero, when it is done), and calls
/// the request's completion handler at +0x28 when there is one. When the
/// counter falls back to zero it services the queue (`EAFile_ServiceQueue`,
/// 0x3ddfa8). Converted from the generated body: same registers, cycles, calls.
pub fn eafile_complete_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x520000;
        frame.save(c, 16, 0x10);
        c.g[A2][0] = c.g[V1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6550));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        let no_request = c.g[A1][0] == 0u64;
        frame.restore(c, 16, 0x10);
        'done: {
            if no_request {
                break 'done;
            }
            // Status word of the request.
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
            let busy = c.g[V0][0] != 0u64;
            c.g[V0][0] = u64::MAX;
            if !busy {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                c.g[V1][0] = 0x1;
                if c.g[A0][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
            }
            // Store it, then the completion handler.
            c.cycles += 6;
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6550));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28));
            if c.g[V0][0] != 0u64 {
                c.cycles += 5;
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32));
                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                let handler = c.g[V0][0] as u32;
                call_ptr(c, handler, 0x003d_e494)?;
            }
            // Count down; service the queue when it reaches zero.
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            let queue_left = c.g[V1][0] != 0u64;
            frame.restore(c, 16, 0x10);
            if !queue_left {
                c.cycles += 3;
                c.g[A0][0] = 0u64;
                Callee {
                    addr: 0x003d_dfa8,
                    f: ee::s14::f_003ddfa8,
                }
                .call(c, 0x003d_e4b4)?;
                frame.restore(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float Rider_GetGateChargeAngle(Rider* r)` (0x00115ab0): the start gate's
/// charge angle. Takes the BE interface (`cBE::getInterface(2)`), reads the
/// knee-offset value of the rider's +0x86c index through it, and returns
/// `75 + (-90) * offset` as a float.
pub fn rider_get_gate_charge_angle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0011_5ac4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0014_dd58, 0x0011_5ad0)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_efa8, 0x0011_5adc)?;
        c.g[AT][0] = 0xffff_ffff_c2b4_0000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x4296_0000;
        c.mtc1(1, 2);
        c.mul_s(0, 0, 1);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        c.add_s(0, 0, 2);
        frame.close(c);
        Ok(())
    })
}

/// `bool tWPInGameData_SkyBox_isEqual(a, b)` (0x002bdd10): true when the
/// sky-box rotation floats of the two records are equal (`a` at +0x8, `b`
/// at +0x4 in the generated body's order).
pub fn sky_box_is_equal(c: &mut Cpu) {
    c.cycles += 6;
    c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
    c.c_eq_s(1, 0);
    c.g[V0][0] = 0u64;
    if c.fcc() {
        c.cycles += 1;
        c.g[V0][0] = 0x1;
    }
    c.cycles += 2;
    ret(c);
}

/// `bool cRider_isIntersecting(Rider* a, Rider* b)` (0x00108c28): whether the
/// id at +0x78 of `b` is in `a`'s list at +0x5b8, an array of up to 64 ids
/// ended by -1. Returns 1 on a match, else 0. The scan is a structured loop:
/// at its back edge it delivers a due event and carries on.
pub fn rider_is_intersecting(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5b8));
    c.g[V0][0] = !(0u64 | c.g[V0][0]);
    let non_empty = c.g[V0][0] != 0u64;
    c.g[A2][0] = 0u64;
    if non_empty {
        c.cycles += 4;
        c.g[V1][0] = u64::MAX;
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x78));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b8));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
            let differs = c.g[V0][0] != c.g[A1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            if !differs {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 64i64) as u64;
            let more = c.g[V0][0] != 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
            if c.g[V0][0] == c.g[V1][0] {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0010_8c48);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `void cHUDFX::set(id, int displayInt, float decayTime, int data)`
/// (0x001170a8): formats `a2` into the number text buffer (0x4a5e20, through
/// the format at 0x4a1070), sets it on the HUD slot `a0` with
/// `cHUDFX_SetText`, and stores the slot's type, value, duration and data.
pub fn chudfx_show_number(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x50);
        frame.save(c, 20, 0x10);
        c.g[S0][0] = 0x4a0000;
        frame.save(c, 17, 0x40);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5e20));
        frame.save_fpr(c, 20, 0x60);
        c.g[S2][0] = c.g[A2][0];
        c.g[A1][0] = 0x4a0000;
        c.mov_s(20, 12);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1070));
        call_ptr(c, 0x0041_7828, 0x0011_70f4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0011_7008,
            f: ee::s00::f_00117008,
        }
        .call(c, 0x0011_7100)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[S2][0]);
        c.sw((c.g[S1][0] as u32), c.g[S4][0]);
        c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[S3][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `lbl_003A01FC` (0x003a01fc): a label of a larger function, entered on its
/// own. Sets bits 13-18 of the word at +0x14 of `a3` to the low six bits of
/// `(int) f1`, then tail-calls `0x3a0274` (which returns to the caller). The
/// generated branch on `true` always takes the tail call; its other arm
/// (0x3a0228) is dead code and is not reproduced.
pub fn lbl_003a01fc(c: &mut Cpu) {
    c.cycles += 11;
    c.cvt_w_s(0, 1);
    c.mfc1(2, 0);
    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
    c.g[V1][0] = 0xffff_ffff_fff8_0000;
    c.g[V1][0] = c.g[V1][0] | 0x1fff;
    c.g[V0][0] = c.g[V0][0] & 0x3f;
    c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 13);
    c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
    c.sw((c.g[A3][0] as u32).wrapping_add(0x14), c.g[A0][0]);
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

/// `VU0_RestoreContext(ctx)` (0x0031a3c0): reloads VU0 from a context record
/// (the mirror of [`vu0_save_context`]). vi22 and vi18 come from +0x208 and
/// +0x20c, vf1..vf6 from +0x1f0 (through ACC) and +0x00..+0x50, then
/// `VU0_SetSignedUnitVector(ctx->+0x200)` runs, then vf7..vf31 from +0x60 on,
/// vi16 from +0x204.
pub fn vu0_restore_context(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 51;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[T0][0] = c.g[S0][0];
        let ctx = c.g[S0][0] as u32;
        let slot = |vf: usize| ctx.wrapping_add(0x10 * (vf as u32 - 1));
        c.g[T1][0] = c.lw(ctx.wrapping_add(0x208));
        c.ctc2(9, 22);
        c.g[T1][0] = c.lw(ctx.wrapping_add(0x20c));
        c.ctc2(9, 18);
        c.lqc2(1, ctx.wrapping_add(0x1f0));
        c.vu0().adda(0b1111, 1, Src::Bc(0, X));
        for vf in 1..=6 {
            c.lqc2(vf, slot(vf));
        }
        c.g[A0][0] = c.lw(ctx.wrapping_add(0x200));
        Callee {
            addr: 0x0031_a268,
            f: ee::s10::f_0031a268,
        }
        .call(c, 0x0031_a40c)?;
        c.g[T0][0] = c.g[S0][0];
        for vf in 7..=12 {
            c.lqc2(vf, slot(vf));
        }
        c.g[T1][0] = c.lw(ctx.wrapping_add(0x204));
        for vf in 13..=14 {
            c.lqc2(vf, slot(vf));
        }
        c.ctc2(9, 16);
        for vf in 15..=31 {
            c.lqc2(vf, slot(vf));
        }
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `VU0_SaveContext(ctx)` (0x0031a308): stores VU0 into a context record: the
/// control words vi17 (+0x200), vi16 (+0x204), vi22 (+0x208) and vi18 (+0x20c),
/// and vf1..vf31 at +0x00..+0x1e0 (vf1 again at +0x1f0, after the final madd).
/// Leaves `t0 = a0`, and `t1` holds the last control word read, as the
/// generated body does.
pub fn vu0_save_context(c: &mut Cpu) {
    c.cycles += 45;
    let ctx = c.g[A0][0] as u32;
    c.g[T0][0] = c.g[A0][0];
    c.cfc2(9, 17);
    c.sqc2(1, ctx);
    c.sqc2(2, ctx.wrapping_add(0x10));
    c.sw(ctx.wrapping_add(0x200), c.g[T1][0]);
    c.sqc2(3, ctx.wrapping_add(0x20));
    c.sqc2(4, ctx.wrapping_add(0x30));
    c.cfc2(9, 16);
    c.sqc2(5, ctx.wrapping_add(0x40));
    c.sqc2(6, ctx.wrapping_add(0x50));
    c.sw(ctx.wrapping_add(0x204), c.g[T1][0]);
    c.vu0().waitq();
    c.cfc2(9, 22);
    c.sw(ctx.wrapping_add(0x208), c.g[T1][0]);
    c.cfc2(9, 18);
    c.sw(ctx.wrapping_add(0x20c), c.g[T1][0]);
    c.sqc2(7, ctx.wrapping_add(0x60));
    c.sqc2(8, ctx.wrapping_add(0x70));
    c.vu0().madd(0b1111, 1, 0, Src::Bc(0, X));
    for vf in 9..=31 {
        c.sqc2(vf, ctx.wrapping_add(0x10 * (vf as u32 - 1)));
    }
    c.sqc2(1, ctx.wrapping_add(0x1f0));
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule};
    use crate::difftest::fakes::{self, check_reaching};
    use crate::difftest::{check, fill, scramble, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real bodies).
    pub(crate) const OC1486_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const OC1486_MAGIC: u32 = 0x1486_f00d;

    macro_rules! oc1486_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1486_CTRL) != OC1486_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc1486_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc1486_fakes! {
        oc1486_fake_service_queue, 0x003d_dfa8, ee::s14::OVERRIDE_003DDFA8, ee::s14::f_003ddfa8__generated;
    }

    /// The completion state at 0x519ab0 (the counter at +0x10, the pending
    /// request at +0x24), one request, and the handler (a fake outside the
    /// function table). The seed's hash picks each branch.
    pub(crate) fn eafile_complete_request_setup(c: &mut Cpu, seed: u32) -> Rng {
        oc1486_hook();
        let mut r = fakes::machine(c, seed, 0x1486_0003, 0);
        c.bus.write32(OC1486_CTRL, OC1486_MAGIC);
        let h = fakes::mix(seed as u64 ^ 0x3de4_20);
        let bit = |n: u32| (h >> n) & 1 != 0;
        let state = 0x0051_9ab0u32;
        let req = DATA + 0x1000;
        // A count of 0 before this completion means the queue is serviced.
        c.bus
            .write32(state + 0x10, if bit(0) { 0 } else { 3 + (seed % 5) });
        c.bus.write32(state + 0x24, if bit(1) { 0 } else { req });
        c.bus.write32(req, r.next_u32());
        c.bus
            .write32(req + 0x4, if bit(2) { 0 } else { r.next_u32() });
        c.bus.write32(req + 0x8, r.next_u32());
        c.bus.write32(req + 0x14, r.next_u32());
        c.bus
            .write32(req + 0x28, if bit(3) { 0 } else { fakes::FAKE + 0x100 });
        c.g[A0][0] = if bit(4) { 0 } else { sx(r.next_u32()) };
        r
    }

    #[test]
    fn eafile_complete_request_matches() {
        check_reaching(
            ee::s14::f_003de420__generated,
            eafile_complete_request,
            240,
            &[0x003d_e494, 0x003d_e4b4],
            &|c, seed| {
                eafile_complete_request_setup(c, seed);
            },
        );
    }

    #[test]
    fn sky_box_is_equal_matches() {
        check(
            ee::s08::f_002bdd10__generated,
            sky_box_is_equal,
            200,
            &|c, seed| {
                let mut r = Rng(0x2bdd_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                let v = r.edge_f32();
                c.bus.write32(DATA + 0x8, v);
                let w = if seed % 2 == 0 { v } else { r.edge_f32() };
                c.bus.write32(DATA + 0x100 + 0x4, w);
            },
        );
    }

    /// A rider with its +0x86c index (`machine_rt` supplies the allocator and
    /// the BELibrary dummy the BE callees read).
    pub(crate) fn rider_gate_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine_rt(c, seed, 0x1486_1150, 0);
        c.g[A0][0] = sx(DATA);
        c.bus.write32(DATA + 0x86c, r.next_u32() % 4);
    }

    #[test]
    fn rider_get_gate_charge_angle_matches() {
        check(
            ee::s00::f_00115ab0__generated,
            rider_get_gate_charge_angle,
            200,
            &|c, seed| rider_gate_setup(c, seed),
        );
    }

    /// The rider's 64-slot id list at +0x5b8 (-1 ended), and the probe's id
    /// at +0x78. The seed picks the list length (0..69), whether the probe
    /// is in it, and where.
    pub(crate) fn rider_is_intersecting_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = Rng(0x108c_2800 ^ seed as u64);
        scramble(c, &mut r);
        let rider = DATA + 0x1000;
        let probe = DATA + 0x2000;
        c.g[A0][0] = sx(rider);
        c.g[A1][0] = sx(probe);
        let target = 0x0000_1000 + (seed % 7);
        c.bus.write32(probe + 0x78, target);
        let list = rider + 0x5b8;
        let n = seed % 70;
        let hit = if n == 0 { 0 } else { seed % n };
        for i in 0..n.min(64) {
            let v = if seed % 2 == 0 && i == hit {
                target
            } else {
                r.next_u32() & 0x7fff_ffff
            };
            c.bus.write32(list + 4 * i, v);
        }
        if n < 64 {
            c.bus.write32(list + 4 * n, 0xffff_ffff);
        }
        r
    }

    #[test]
    fn rider_is_intersecting_matches() {
        check(
            ee::s00::f_00108c28__generated,
            rider_is_intersecting,
            240,
            &|c, seed| {
                rider_is_intersecting_setup(c, seed);
            },
        );
    }

    /// Events due at the scan's back edges.
    #[test]
    fn rider_is_intersecting_events() {
        check_events(
            ee::s00::f_00108c28__generated,
            rider_is_intersecting,
            120,
            &|c, seed| {
                let mut r = rider_is_intersecting_setup(c, seed);
                schedule(c, &mut r, seed, 0x0010_8c28)
            },
        );
    }

    #[test]
    fn chudfx_show_number_matches() {
        check(
            ee::s00::f_001170a8__generated,
            chudfx_show_number,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1486_1170, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn lbl_003a01fc_matches() {
        check(
            ee::s13::f_003a01fc__generated,
            lbl_003a01fc,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1486_3a01, 0);
                c.g[A3][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1014, r.next_u32());
                // The tail callee returns at once when its bound is 0.
                c.g[A1][0] = 0;
                c.f[1] = r.edge_f32();
                c.g[RA][0] = sx(crate::difftest::RA);
            },
        );
    }

    #[test]
    fn vu0_save_context_matches() {
        check(
            ee::s10::f_0031a308__generated,
            vu0_save_context,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1486_3a08, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = sx(crate::difftest::RA);
                fill(c, &mut r, DATA + 0x1000, 0x20);
            },
        );
    }

    #[test]
    fn vu0_restore_context_matches() {
        check(
            ee::s10::f_0031a3c0__generated,
            vu0_restore_context,
            200,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1486_3a3c, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[RA][0] = sx(crate::difftest::RA);
                fill(c, &mut r, DATA + 0x1000, 0x90);
                // The unit-vector helper's argument: a small mask.
                c.bus.write32(DATA + 0x1200, r.next_u32() & 0xffff);
            },
        );
    }
}
