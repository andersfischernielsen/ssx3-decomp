//! Staging module oc1416 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1416`.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Callees of this module's ports, with main's names.
pub(crate) mod callee {}

/// `void VidEngine_DebugPrintfStub(...)` (0x253b18): spill the argument
/// registers to stack scratch and return (the debug print never happens).
pub fn vid_engine_debug_printf_stub(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x48), c.g[A1][0]);
        c.sd(frame.at(c, 0x50), c.g[A2][0]);
        c.sd(frame.at(c, 0x58), c.g[A3][0]);
        c.sd(frame.at(c, 0x60), c.g[T0][0]);
        c.sd(frame.at(c, 0x68), c.g[T1][0]);
        c.sd(frame.at(c, 0x70), c.g[T2][0]);
        c.sd(frame.at(c, 0x78), c.g[T3][0]);
        c.swc1(12, frame.at(c, 0x38));
        c.swc1(14, frame.at(c, 0x3c));
        c.swc1(16, frame.at(c, 0x40));
        c.swc1(18, frame.at(c, 0x44));
        frame.close(c);
        Ok(())
    })
}

/// `void VD_DebugPrintfStub(...)` (0x254e20): spill the argument registers
/// to stack scratch and return (the debug print never happens).
pub fn vd_debug_printf_stub(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x48), c.g[A1][0]);
        c.sd(frame.at(c, 0x50), c.g[A2][0]);
        c.sd(frame.at(c, 0x58), c.g[A3][0]);
        c.sd(frame.at(c, 0x60), c.g[T0][0]);
        c.sd(frame.at(c, 0x68), c.g[T1][0]);
        c.sd(frame.at(c, 0x70), c.g[T2][0]);
        c.sd(frame.at(c, 0x78), c.g[T3][0]);
        c.swc1(12, frame.at(c, 0x38));
        c.swc1(14, frame.at(c, 0x3c));
        c.swc1(16, frame.at(c, 0x40));
        c.swc1(18, frame.at(c, 0x44));
        frame.close(c);
        Ok(())
    })
}

/// `void *VD_Alloc(...)` (0x255670): allocate through the video alloc hook
/// (the function pointer at `0x509430`, reached as `*(0x510000 - 0x6bd0)`).
pub fn vd_alloc(c: &mut Cpu) {
    alloc_body(c, 0x0025_56a0);
}

/// `void *VD_Alloc_2(...)` (0x2556b0): allocate through the video alloc
/// hook, as `VD_Alloc` (a second call site).
pub fn vd_alloc_2(c: &mut Cpu) {
    alloc_body(c, 0x0025_56e0);
}

/// Shared body of the two `VD_Alloc` wrappers.
fn alloc_body(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x510000;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = 0u64;
        c.g[A0][0] = 0x4a0000;
        c.g[A3][0] = 0u64;
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ea0));
        call_ptr(c, t, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PS2ExecMan_ForwardSlot06(PS2ExecMan *man)` (0x320a20): tail-call the
/// slot-6 handler through the object's slot table (-1 when `man` has no
/// table).
pub fn ps2_exec_man_forward_slot06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        if c.g[A1][0] != 0u64 {
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
            c.cycles += 4;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0032_0a50)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0025_3b18,
        vid_engine_debug_printf_stub,
        "VidEngine_DebugPrintfStub",
    ),
    (0x0025_4e20, vd_debug_printf_stub, "VD_DebugPrintfStub"),
    (0x0025_5670, vd_alloc, "VD_Alloc"),
    (0x0025_56b0, vd_alloc_2, "VD_Alloc_2"),
    (
        0x0032_0a20,
        ps2_exec_man_forward_slot06,
        "PS2ExecMan_ForwardSlot06",
    ),
];

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{require_paths, DATA};
    use ssx3_ee as ee;

    /// The `miss` hook with a fixed 4-cycle cost: the fake's hash-based cost
    /// would otherwise make single-path wrappers' totals seed-varying.
    fn norm_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        crate::difftest::fakes::fake_call(c, target);
        c.cycles = before + 4;
    }

    /// The two debug-print stubs: every spilled register is distinctive, so
    /// a missing or extra store changes RAM.
    #[test]
    fn debug_printf_stubs_match() {
        for (gen, hand, addr) in [
            (
                ee::s06::f_00253b18__generated as fn(&mut Cpu),
                vid_engine_debug_printf_stub as fn(&mut Cpu),
                0x0025_3b18,
            ),
            (
                ee::s06::f_00254e20__generated as fn(&mut Cpu),
                vd_debug_printf_stub as fn(&mut Cpu),
                0x0025_4e20,
            ),
        ] {
            crate::difftest::check_paths(gen, hand, 100, &[15], &|c, seed| {
                machine(c, seed, addr as u64, 0);
                c.g[A0][0] = (DATA + 0x123) as u64;
                c.g[A1][0] = 0x1111_0000 + seed as u64;
                c.g[A2][0] = 0x2222_0000 + seed as u64 * 3;
                c.g[A3][0] = 0x3333_0000 + seed as u64 * 7;
                c.g[T0][0] = 0x4444_0000 + seed as u64;
                c.g[T1][0] = 0x5555_0000 + seed as u64;
                c.g[T2][0] = 0x6666_0000 + seed as u64;
                c.g[T3][0] = 0x7777_0000 + seed as u64;
                c.f[12] = 0x3f80_0000 + seed;
                c.f[14] = 0x4000_0000 + seed * 3;
                c.f[16] = 0x4040_0000 + seed * 5;
                c.f[18] = 0x4080_0000 + seed * 7;
                c.pc = addr;
            });
        }
    }

    /// `VD_Alloc` and `VD_Alloc_2`: the hook word points at the fake, whose
    /// hash result varies with the object on both sides.
    #[test]
    fn vd_alloc_matches() {
        for (gen, hand, addr) in [
            (
                ee::s06::f_00255670__generated as fn(&mut Cpu),
                vd_alloc as fn(&mut Cpu),
                0x0025_5670,
            ),
            (
                ee::s06::f_002556b0__generated as fn(&mut Cpu),
                vd_alloc_2 as fn(&mut Cpu),
                0x0025_56b0,
            ),
        ] {
            let setup = |c: &mut Cpu, seed: u32| {
                let mut r = machine(c, seed, addr as u64, 0);
                c.hooks.miss = norm_miss;
                c.bus.write32(OPEN_ALLOC_HOOK, FAKE);
                c.g[A0][0] = (DATA + 0x123 + (r.next_u32() & 0xfff0)) as u64;
                c.pc = addr;
            };
            crate::difftest::check_paths(gen, hand, 300, &[19], &setup);
            assert_no_null_hook_calls(gen, 300, &setup);
        }

        /// The video hook words (`0x510000 - 0x6bd0`).
        pub(crate) const OPEN_ALLOC_HOOK: u32 = 0x0050_9430;

        /// The video hook words must actually be reached: run the setup
        /// through the generated body and require that no call fell through
        /// to address 0. (The pre-fix fixtures planted the hook at 0x4a9430 /
        /// 0x4a9434 instead of 0x509430 / 0x509434, so both sides called
        /// address 0 and agreed by accident.)
        fn assert_no_null_hook_calls(gen: fn(&mut Cpu), seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
            for seed in 0..crate::difftest::seed_count(seeds) {
                let mut c = crate::difftest::fresh();
                setup(&mut c, seed);
                crate::difftest::run_generated(&mut c, gen);
                let targets = crate::difftest::fakes::logged_targets(&mut c);
                assert!(
                    targets.iter().all(|&t| t != 0),
                    "seed {seed}: call to address 0 (hook word not planted): {targets:#x?}"
                );
            }
        }
    }

    /// `PS2ExecMan_ForwardSlot06`: no table reads -1; a table runs the fake
    /// handler with a steered offset.
    #[test]
    fn ps2_exec_man_forward_slot06_matches() {
        check(
            ee::s10::f_00320a20__generated,
            ps2_exec_man_forward_slot06,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_0a20, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                if seed % 2 == 0 {
                    c.bus.write32(OBJ + 8, 0);
                } else {
                    c.bus.write32(OBJ + 8, TAB1);
                    c.bus.write32(TAB1 + 8, TAB2);
                    for i in 0..0x20u32 {
                        c.bus.write32(TAB2 + 4 * i, 0x9e37_0000 + i);
                    }
                    let off = match seed % 3 {
                        0 => 0,
                        1 => (r.next_u32() % 0x4000) as u16,
                        _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                    };
                    c.bus.write16(TAB2 + 0x30, off);
                    c.bus.write32(TAB2 + 0x34, FAKE);
                }
                c.g[A0][0] = OBJ as u64;
                c.pc = 0x0032_0a20;
            },
        );
        require_paths(
            ee::s10::f_00320a20__generated,
            100,
            &[11, 16],
            &|c, seed| {
                let mut r = machine(c, seed, 0x0032_0a20, 0);
                c.hooks.miss = norm_miss;
                const OBJ: u32 = DATA + 0x110;
                const TAB1: u32 = DATA + 0x230;
                const TAB2: u32 = DATA + 0x350;
                if seed % 2 == 0 {
                    c.bus.write32(OBJ + 8, 0);
                } else {
                    c.bus.write32(OBJ + 8, TAB1);
                    c.bus.write32(TAB1 + 8, TAB2);
                    for i in 0..0x20u32 {
                        c.bus.write32(TAB2 + 4 * i, 0x9e37_0000 + i);
                    }
                    let off = match seed % 3 {
                        0 => 0,
                        1 => (r.next_u32() % 0x4000) as u16,
                        _ => (0x10000 - r.next_u32() % 0x4000) as u16,
                    };
                    c.bus.write16(TAB2 + 0x30, off);
                    c.bus.write32(TAB2 + 0x34, FAKE);
                }
                c.g[A0][0] = OBJ as u64;
                c.pc = 0x0032_0a20;
            },
        );
    }
}
